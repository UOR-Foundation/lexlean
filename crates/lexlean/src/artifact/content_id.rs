//! Content identity (SPEC.md §21.1–§21.5, §22.9, §19.8): the κ-label, the
//! frame function, the tree κ, and every labeled recipe.
//!
//! Every identity LexLean computes is a UOR κ-label on the BLAKE3 σ-axis,
//! `blake3:` and the 64 lowercase hexadecimal digits of the BLAKE3 digest of
//! its carrier, which is exactly the label uor-addr and Hologram's store give
//! the same bytes. The digest is the standard library's certified
//! `LexLeanStd.Blake3.blake3Label`, compiled from the committed copy of its
//! production export (`crates/lexlean/blake3_package.rs`, held byte for byte
//! to the export by `SL-06`); no other implementation computes an identity.

use crate::blake3_package;

/// The κ-label of `message` as the certified package writes it.
fn blake3_label(message: &[u8]) -> [u8; KAPPA_BYTES] {
    // The package declares the effect `overflow`, which its arithmetic on
    // byte offsets can raise only for a message whose length does not fit 64
    // bits; no host allocates one, so this is an impossibility of the host and
    // not a failure of any input (R5).
    let label = blake3_package::run(blake3_package::Bytes::lit(message))
        .expect("a message whose length fits 64 bits is inside the domain of BLAKE3");
    label
        .octets()
        .try_into()
        .expect("the package's label is `blake3:` and 64 hexadecimal digits")
}

/// The width of a κ-label on the BLAKE3 axis: `blake3:` and 64 hex digits.
pub const KAPPA_BYTES: usize = 71;

/// The σ-axis prefix of every κ-label LexLean computes.
pub const KAPPA_PREFIX: &str = "blake3:";

/// A UOR κ-label on the BLAKE3 σ-axis.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Kappa([u8; KAPPA_BYTES]);

impl Kappa {
    /// The κ-label of raw bytes: the content address of the bytes themselves.
    #[must_use]
    pub fn of(bytes: &[u8]) -> Self {
        Self(blake3_label(bytes))
    }

    /// The label of no content, `blake3:` and 64 zeros: the value a field
    /// holds while the recipe that omits it is computed, and after an error
    /// that ends the read of the document holding it. BLAKE3 gives it to no
    /// input anyone can exhibit.
    pub const ZERO: Self = {
        let mut out = [b'0'; KAPPA_BYTES];
        let prefix = KAPPA_PREFIX.as_bytes();
        let mut index = 0;
        while index < prefix.len() {
            out[index] = prefix[index];
            index += 1;
        }
        Self(out)
    };

    /// The label, `blake3:<64 hex>`.
    #[must_use]
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0).expect("a κ-label is ASCII")
    }

    /// The label as an owned string.
    #[must_use]
    pub fn label(&self) -> String {
        self.as_str().to_owned()
    }

    /// The label's 71 bytes, which is how a κ is framed into another (§21.1).
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; KAPPA_BYTES] {
        &self.0
    }

    /// The 64 hexadecimal digits of the digest, without the axis.
    #[must_use]
    pub fn hex(&self) -> &str {
        &self.as_str()[KAPPA_PREFIX.len()..]
    }

    /// Parse a κ-label: `blake3:` and exactly 64 lowercase hex digits.
    ///
    /// # Errors
    ///
    /// The first way `text` is not such a label.
    pub fn parse(text: &str) -> Result<Self, String> {
        let Some(digits) = text.strip_prefix(KAPPA_PREFIX) else {
            return Err(format!("a κ-label begins with `{KAPPA_PREFIX}`"));
        };
        if digits.len() != 64 {
            return Err(format!("expected 64 hex digits, found {}", digits.len()));
        }
        if !digits
            .bytes()
            .all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'))
        {
            return Err("lowercase hexadecimal is mandatory".to_owned());
        }
        let mut out = [0u8; KAPPA_BYTES];
        out.copy_from_slice(text.as_bytes());
        Ok(Self(out))
    }
}

impl std::fmt::Debug for Kappa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Display for Kappa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl serde::Serialize for Kappa {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for Kappa {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Self::parse(&s).map_err(serde::de::Error::custom)
    }
}

/// An incremental framed carrier of one labeled recipe (§21.1).
///
/// The package digests a whole message, so the frames are held until `finish`.
pub struct FramedHasher {
    message: Vec<u8>,
}

impl FramedHasher {
    /// Start a recipe with its domain-separation prefix, e.g.
    /// `"lexlean-source-v2\0"`.
    #[must_use]
    pub fn new(domain: &str) -> Self {
        let mut message = domain.as_bytes().to_vec();
        message.push(0u8);
        Self { message }
    }

    /// Append one `frame(label, bytes)` (§21.1).
    pub fn frame(&mut self, label: &str, bytes: &[u8]) {
        let label_bytes = label.as_bytes();
        self.message.extend_from_slice(
            &u32::try_from(label_bytes.len())
                .expect("label fits u32")
                .to_be_bytes(),
        );
        self.message.extend_from_slice(label_bytes);
        self.message
            .extend_from_slice(&(bytes.len() as u64).to_be_bytes());
        self.message.extend_from_slice(bytes);
    }

    /// Append a κ as its 71 label bytes, so that the outer κ covers its axis.
    pub fn frame_kappa(&mut self, label: &str, kappa: Kappa) {
        self.frame(label, kappa.as_bytes());
    }

    /// The carrier so far, for test vectors that publish what they hash.
    #[must_use]
    pub fn carrier(&self) -> &[u8] {
        &self.message
    }

    /// Finish the recipe: the κ-label of the carrier.
    #[must_use]
    pub fn finish(self) -> Kappa {
        Kappa::of(&self.message)
    }
}

/// The §11.5 tree κ over `(relative path, bytes)` rows. The caller
/// supplies rows already restricted to participating files; this function
/// enforces bytewise-sorted path order and unique paths.
#[must_use]
pub fn tree_kappa(files: &[(&str, &[u8])]) -> Kappa {
    debug_assert!(
        files
            .windows(2)
            .all(|pair| pair[0].0.as_bytes() < pair[1].0.as_bytes()),
        "tree κ input must be strictly sorted by path bytes"
    );
    let mut message = b"lexlean-tree-v2\0".to_vec();
    for (path, bytes) in files {
        let path_bytes = path.as_bytes();
        message.extend_from_slice(
            &u32::try_from(path_bytes.len())
                .expect("path fits u32")
                .to_be_bytes(),
        );
        message.extend_from_slice(path_bytes);
        message.extend_from_slice(&(bytes.len() as u64).to_be_bytes());
        message.extend_from_slice(bytes);
    }
    Kappa::of(&message)
}

/// The source ID (§21.3). `sources` are `(project-relative path, normalized
/// bytes)` in sorted path order.
#[must_use]
pub fn source_id(
    canonical_project_toml: &[u8],
    canonical_lock: &[u8],
    sources: &[(String, Vec<u8>)],
) -> Kappa {
    let mut hasher = FramedHasher::new("lexlean-source-v2");
    hasher.frame("project", canonical_project_toml);
    hasher.frame("lock", canonical_lock);
    for (path, bytes) in sources {
        hasher.frame("path", path.as_bytes());
        hasher.frame("source", bytes);
    }
    hasher.finish()
}

/// The semantic ID (§21.4).
#[must_use]
pub fn semantic_id_for(
    compiler_semantics: Kappa,
    language: &str,
    linked_ir_json: &str,
    lexicon_closure_json: &str,
) -> Kappa {
    let mut hasher = FramedHasher::new("lexlean-semantic-v2");
    hasher.frame_kappa("compiler-semantics", compiler_semantics);
    hasher.frame("language", language.as_bytes());
    hasher.frame("toolchain", crate::LEAN_TOOLCHAIN.as_bytes());
    hasher.frame("linked-ir", linked_ir_json.as_bytes());
    hasher.frame("lexicon-closure", lexicon_closure_json.as_bytes());
    hasher.finish()
}

/// The language-1.0 semantic ID recipe retained for API compatibility.
#[must_use]
pub fn semantic_id(
    compiler_semantics: Kappa,
    linked_ir_json: &str,
    lexicon_closure_json: &str,
) -> Kappa {
    semantic_id_for(
        compiler_semantics,
        crate::LANGUAGE_VERSION,
        linked_ir_json,
        lexicon_closure_json,
    )
}

/// The build ID (§21.5).
#[must_use]
pub fn build_id(source: Kappa, semantic: Kappa) -> Kappa {
    let mut hasher = FramedHasher::new("lexlean-build-v2");
    hasher.frame_kappa("source-id", source);
    hasher.frame_kappa("semantic-id", semantic);
    hasher.finish()
}

/// The attestation ID over the canonical body without its `attestation_id`
/// field (§22.9).
#[must_use]
pub fn attestation_id(body_without_id_json: &str) -> Kappa {
    let mut hasher = FramedHasher::new("lexlean-attestation-v2");
    hasher.frame("attestation-body", body_without_id_json.as_bytes());
    hasher.finish()
}

/// The PDF recipe ID (§19.8).
#[must_use]
pub fn pdf_recipe_id(
    tex: Kappa,
    program: Kappa,
    version_output: Kappa,
    compile_argv_json: &str,
    resources_json: &str,
) -> Kappa {
    let mut hasher = FramedHasher::new("lexlean-pdf-recipe-v2");
    hasher.frame_kappa("tex", tex);
    hasher.frame_kappa("program", program);
    hasher.frame_kappa("version-output", version_output);
    hasher.frame("argv", compile_argv_json.as_bytes());
    hasher.frame("resources", resources_json.as_bytes());
    hasher.finish()
}
