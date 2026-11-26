//! Where the expensive conformance cases spend their time (#603 DOD-7).
//! Evidence only: nothing reads these records, and no case's outcome depends
//! on them.
//!
//! A `BS` case's log lines measure from the case's own start, but on a host
//! the cases run one at a time behind `bootstrap::run`'s memory mutex, and a
//! test's captured output is printed only when it fails. So the log shows
//! neither how long a case waited for the mutex nor what its time was spent
//! on. When `LEXLEAN_CONFORMANCE_PHASES` names a directory, each process
//! appends one JSON object per phase to `conformance-phases-<pid>.jsonl`
//! there: what ran, for which case, when it started relative to the process,
//! how long it took, and, on Linux, the process's peak resident set so far.
//! The first record describes the host's effective CPU and memory limits.
//! Unset, every call is a timer read and nothing else.

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// The directory the records go to; unset or empty records nothing.
pub const VARIABLE: &str = "LEXLEAN_CONFORMANCE_PHASES";

struct Sink {
    origin: Instant,
    file: Mutex<File>,
}

fn sink() -> Option<&'static Sink> {
    static SINK: OnceLock<Option<Sink>> = OnceLock::new();
    SINK.get_or_init(|| {
        let directory = std::env::var_os(VARIABLE).filter(|value| !value.is_empty())?;
        let path = PathBuf::from(directory)
            .join(format!("conformance-phases-{}.jsonl", std::process::id()));
        // Evidence must not decide a case: a directory that cannot be written
        // is reported once and the records are dropped.
        let file = std::fs::create_dir_all(path.parent().unwrap_or(&path))
            .and_then(|()| {
                std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&path)
            })
            .map_err(|error| eprintln!("{VARIABLE}: {}: {error}; no phase records", path.display()))
            .ok()?;
        let sink = Sink {
            origin: Instant::now(),
            file: Mutex::new(file),
        };
        sink.write(&host_record());
        Some(sink)
    })
    .as_ref()
}

impl Sink {
    fn write(&self, record: &serde_json::Value) {
        let mut file = self
            .file
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Err(error) = writeln!(file, "{record}") {
            eprintln!("{VARIABLE}: a phase record was not written: {error}");
        }
    }
}

/// The case the running phases belong to: `bootstrap::run` sets it while it
/// holds the memory mutex, so at most one is current.
static CURRENT: Mutex<Option<String>> = Mutex::new(None);

/// Name the case the following phases belong to, until the guard drops.
#[must_use]
pub fn case(id: &str) -> CaseGuard {
    *CURRENT
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(id.to_owned());
    CaseGuard
}

/// Clears the current case when dropped.
pub struct CaseGuard;

impl Drop for CaseGuard {
    fn drop(&mut self) {
        *CURRENT
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = None;
    }
}

fn current() -> Option<String> {
    CURRENT
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .clone()
}

/// Record that `phase` of `case` (the current case when `None`) ran from
/// `started` for `took`, with `detail` beside it.
pub fn record(
    case: Option<&str>,
    phase: &str,
    started: Instant,
    took: Duration,
    detail: serde_json::Value,
) {
    let Some(sink) = sink() else { return };
    let case = case.map(str::to_owned).or_else(current);
    sink.write(&serde_json::json!({
        "case": case,
        "phase": phase,
        "start_ms": started.saturating_duration_since(sink.origin).as_secs_f64() * 1e3,
        "ms": took.as_secs_f64() * 1e3,
        "thread": format!("{:?}", std::thread::current().id()),
        "peak_rss_kib": peak_rss_kib(),
        "detail": detail,
    }));
}

/// Run `body` as `phase` of the current case.
pub fn span<T>(phase: &str, body: impl FnOnce() -> T) -> T {
    let started = Instant::now();
    let value = body();
    record(
        None,
        phase,
        started,
        started.elapsed(),
        serde_json::Value::Null,
    );
    value
}

/// Whether records are being written: a caller may skip work done only to
/// describe a phase.
#[must_use]
pub fn enabled() -> bool {
    sink().is_some()
}

fn read(path: &str) -> Option<String> {
    std::fs::read_to_string(path)
        .ok()
        .map(|text| text.trim().to_owned())
}

/// The peak resident set of this process so far (Linux `VmHWM`).
fn peak_rss_kib() -> Option<u64> {
    read("/proc/self/status")?
        .lines()
        .find_map(|line| line.strip_prefix("VmHWM:"))
        .and_then(|value| value.trim().trim_end_matches("kB").trim().parse().ok())
}

/// What bounds this host: the parallelism the process sees, the test-thread
/// setting, and where the platform exposes them, the cgroup CPU and memory
/// limits and the physical memory.
fn host_record() -> serde_json::Value {
    let epoch_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0.0, |d| d.as_secs_f64() * 1e3);
    serde_json::json!({
        "phase": "host",
        "epoch_ms": epoch_ms,
        "os": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
        "available_parallelism": std::thread::available_parallelism().map(std::num::NonZeroUsize::get).ok(),
        "rust_test_threads": std::env::var("RUST_TEST_THREADS").ok(),
        "cgroup_cpu_max": read("/sys/fs/cgroup/cpu.max"),
        "cgroup_memory_max": read("/sys/fs/cgroup/memory.max"),
        "mem_total_kib": read("/proc/meminfo").and_then(|text| text.lines().find_map(|line| line.strip_prefix("MemTotal:"))
            .and_then(|value| value.trim().trim_end_matches("kB").trim().parse::<u64>().ok())),
    })
}

#[cfg(test)]
mod tests {
    use super::{host_record, peak_rss_kib};

    #[test]
    fn the_host_record_names_its_bounds() {
        let host = host_record();
        assert_eq!(host["phase"], "host");
        assert!(host["available_parallelism"]
            .as_u64()
            .is_some_and(|n| n > 0));
        if cfg!(target_os = "linux") {
            assert!(host["mem_total_kib"].as_u64().is_some_and(|n| n > 0));
            assert!(peak_rss_kib().is_some_and(|n| n > 0));
        }
    }
}
