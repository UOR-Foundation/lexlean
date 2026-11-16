//! Discovery-only parser for the pinned Lake 4.32.1 `lake env` overlay.
//!
//! Lake.Config.Env.baseVars and Workspace.augmentedEnvVars emit these ordered
//! keys; CLI.Main prints every row, including absent values, with a final LF.
//! The overlay is not a lossless environment representation. Only LEAN_PATH,
//! which Lake always emits as Some, is returned. Other values can be secrets:
//! never log the input, and never reconstruct an environment from this output.
//! This parser establishes no launcher import custody or execution authority.

const KEYS: [&str; 23] = [
    "ELAN",
    "ELAN_HOME",
    "ELAN_TOOLCHAIN",
    "LAKE",
    "LAKE_HOME",
    "LAKE_CONFIG",
    "LAKE_PKG_URL_MAP",
    "LAKE_NO_CACHE",
    "LAKE_CACHE_KEY",
    "LAKE_CACHE_ARTIFACT_ENDPOINT",
    "LAKE_CACHE_REVISION_ENDPOINT",
    "LAKE_CACHE_SERVICE",
    "LEAN",
    "LEAN_SYSROOT",
    "LEAN_AR",
    "LEAN_CC",
    "LAKE_CACHE_DIR",
    "LAKE_ARTIFACT_CACHE",
    "LAKE_RESTORE_ARTIFACTS",
    "LEAN_PATH",
    "LEAN_SRC_PATH",
    "LEAN_GITHASH",
    "PATH",
];

/// The platform of the pinned Lake executable, established by its caller.
#[derive(Clone, Copy)]
pub(crate) enum Platform {
    Windows,
    MacOs,
    Other,
}

impl Platform {
    fn suffix(self) -> Option<&'static str> {
        match self {
            Self::Windows => None,
            Self::MacOs => Some("DYLD_LIBRARY_PATH"),
            Self::Other => Some("LD_LIBRARY_PATH"),
        }
    }
}

/// Parse only a successful raw capture, never a normalized ChildRecord. The
/// byte limit is checked before decoding or allocating an output string. All
/// failures are fixed text with no input values. Empty and whitespace-bearing
/// LEAN_PATH values are preserved exactly; platform splitting belongs to the
/// caller. Strict framing rejects embedded LF even when it resembles a key.
pub(crate) fn lean_path(
    bytes: &[u8],
    byte_limit: usize,
    platform: Platform,
) -> Result<String, &'static str> {
    if bytes.is_empty() || bytes.len() > byte_limit {
        return Err("Lake overlay exceeds the admitted byte bound or is empty");
    }
    if bytes.contains(&b'\r') || bytes.contains(&0) {
        return Err("Lake overlay contains a forbidden framing byte");
    }
    let text = std::str::from_utf8(bytes).map_err(|_| "Lake overlay is not UTF-8")?;
    let body = text
        .strip_suffix('\n')
        .ok_or("Lake overlay lacks its final newline")?;
    // Do not use lines(), trim(), or the process-record normalizer here.
    let mut lines = body.split('\n');
    let mut captured = None;
    for expected in KEYS.into_iter().chain(platform.suffix()) {
        let line = lines.next().ok_or("Lake overlay is missing a row")?;
        let (key, value) = line
            .split_once('=')
            .ok_or("Lake overlay row lacks a separator")?;
        if key != expected {
            return Err("Lake overlay keys do not match the pinned ordered contract");
        }
        if key == "LEAN_PATH" {
            captured = Some(value);
        }
    }
    if lines.next().is_some() {
        return Err("Lake overlay contains extra rows");
    }
    captured
        .map(str::to_owned)
        .ok_or("Lake overlay lacks LEAN_PATH")
}

#[cfg(test)]
mod tests {
    use super::{lean_path, Platform, KEYS};

    fn overlay(platform: Platform, path: &str) -> String {
        KEYS.into_iter()
            .chain(platform.suffix())
            .map(|key| {
                let value = match key {
                    "LEAN_PATH" => path,
                    "LAKE_CACHE_KEY" => "private-cache-secret",
                    _ => "",
                };
                format!("{key}={value}\n")
            })
            .collect()
    }

    #[test]
    fn preserves_only_exact_path_including_equals_and_whitespace() {
        for path in ["", "a=b:c", " relative path :tail \t", "a\u{2028}b"] {
            let raw = overlay(Platform::Other, path);
            assert_eq!(
                lean_path(raw.as_bytes(), raw.len(), Platform::Other),
                Ok(path.to_owned())
            );
        }
    }

    #[test]
    fn platform_suffix_is_exact_and_mandatory() {
        for platform in [Platform::Windows, Platform::MacOs, Platform::Other] {
            let raw = overlay(platform, "root");
            assert_eq!(
                lean_path(raw.as_bytes(), raw.len(), platform),
                Ok("root".to_owned())
            );
        }
        let mac = overlay(Platform::MacOs, "root");
        assert!(lean_path(mac.as_bytes(), mac.len(), Platform::Other).is_err());
        assert!(lean_path(mac.as_bytes(), mac.len(), Platform::Windows).is_err());
        let windows = overlay(Platform::Windows, "root");
        assert!(lean_path(windows.as_bytes(), windows.len(), Platform::Other).is_err());
    }

    #[test]
    fn injected_unknown_duplicate_and_reordered_keys_refuse() {
        let raw = overlay(Platform::Other, "root");
        for bad in [
            raw.replace("LEAN_PATH=root", "LEAN_PATH=root\nLEAN_PATH=evil"),
            raw.replace("ELAN_HOME=", "UNKNOWN="),
            raw.replace("ELAN_HOME=", "ELAN="),
            raw.replacen("ELAN=\nELAN_HOME=\n", "ELAN_HOME=\nELAN=\n", 1),
            raw.replace(
                "LAKE_CACHE_KEY=private-cache-secret",
                "LAKE_CACHE_KEY=private-cache-secret\nPATH=evil",
            ),
            raw.replace("LEAN_PATH=root", "LEAN_PATH=one\ntwo"),
        ] {
            let error =
                lean_path(bad.as_bytes(), bad.len(), Platform::Other).expect_err("invalid overlay");
            assert!(!error.contains("private-cache-secret"));
            assert!(!error.contains("evil"));
        }
    }

    #[test]
    fn raw_framing_refuses_normalizer_hidden_changes() {
        let raw = overlay(Platform::Other, "root");
        for bad in [
            raw.replace('\n', "\r\n"),
            raw.replace("LEAN_PATH=", "LEAN_PATH\0="),
            raw.trim_end_matches('\n').to_owned(),
            format!("{raw}\n"),
            format!("{raw} \t\n"),
            raw.replace("LEAN_PATH=", "LEAN_PATH ="),
            raw.replace("LEAN_PATH=", "\u{1b}[31mLEAN_PATH="),
        ] {
            assert!(lean_path(bad.as_bytes(), bad.len(), Platform::Other).is_err());
        }
    }

    #[test]
    fn byte_bound_and_utf8_refuse_before_returning_a_value() {
        let raw = overlay(Platform::Windows, "root");
        assert!(lean_path(raw.as_bytes(), raw.len() - 1, Platform::Windows).is_err());
        assert!(lean_path(&[], 100, Platform::Windows).is_err());
        let mut invalid = raw.into_bytes();
        invalid[0] = 255;
        assert!(lean_path(&invalid, invalid.len(), Platform::Windows).is_err());
    }
}
