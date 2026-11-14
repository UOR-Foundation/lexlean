//! Target machine contracts (SPEC.md §17.15, §17.22, `GN-09`, #119): the
//! `LexLeanStd.Target` module against the instructions it models.
//!
//! Each of the nine instruction roots is lowered and run by the calculus
//! evaluator on a seeded corpus whose lanes sit at the saturation and wrap
//! edges, and the same corpus is run by the instruction itself, compiled by
//! the pinned rustc under `#[target_feature]`. Where the host can execute the
//! target the two must agree on every case; where it cannot, the harness is
//! built for the target and the case says so (C9: rustc and the hardware are
//! checked by differentials, never assumed).

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;
use std::process::Command;

use lexlean::calculus::{interp, rust, Value};
use lexlean::production::lower::Lowered;
use serde_json::Value as Json;

use super::stdlib::lower_in;
use crate::support::{self, P};

/// The module.
const MODULE: &str = "LexLeanStd.Target";

/// The effects every instruction root declares and its closure realizes: the
/// result list is allocated, the lane arithmetic is checked, and the lanes
/// are walked by recursion.
const EFFECTS: [&str; 3] = ["allocation", "overflow", "recursion"];

/// The evaluator's fuel.
const FUEL: u64 = 5_000_000;
/// Seeded cases per instruction, beside the edges.
const SEEDED: usize = 300;

/// An inclusive range of lane values.
type Range = (i32, i32);
const U8: Range = (0, 255);
const I8: Range = (-128, 127);
const I16: Range = (-32768, 32767);

/// One modeled instruction: its root, the operand shapes it reads, and the
/// target it belongs to. The index in [`INSTRUCTIONS`] is the code the
/// generated harness dispatches on.
struct Instruction {
    root: &'static str,
    target: usize,
    first: (usize, Range),
    /// Lanes and range of the second operand; none for a unary instruction.
    second: Option<(usize, Range)>,
}

const INSTRUCTIONS: [Instruction; 9] = [
    Instruction {
        root: "avx2MaddubsEpi16",
        target: 0,
        first: (32, U8),
        second: Some((32, I8)),
    },
    Instruction {
        root: "avx2MaddEpi16",
        target: 0,
        first: (16, I16),
        second: Some((16, I16)),
    },
    Instruction {
        root: "avx2AddsEpi16",
        target: 0,
        first: (16, I16),
        second: Some((16, I16)),
    },
    Instruction {
        root: "neonMullS8",
        target: 1,
        first: (8, I8),
        second: Some((8, I8)),
    },
    Instruction {
        root: "neonPaddlqS16",
        target: 1,
        first: (8, I16),
        second: None,
    },
    Instruction {
        root: "neonQaddqS16",
        target: 1,
        first: (8, I16),
        second: Some((8, I16)),
    },
    Instruction {
        root: "wasmExtmulLowI8x16",
        target: 2,
        first: (16, I8),
        second: Some((16, I8)),
    },
    Instruction {
        root: "wasmDotI16x8",
        target: 2,
        first: (8, I16),
        second: Some((8, I16)),
    },
    Instruction {
        root: "wasmAddSatI16x8",
        target: 2,
        first: (8, I16),
        second: Some((8, I16)),
    },
];

/// A D4 target.
struct Target {
    triple: &'static str,
    feature: &'static str,
}

const TARGETS: [Target; 3] = [
    Target {
        triple: "x86_64-unknown-linux-gnu",
        feature: "avx2",
    },
    Target {
        triple: "aarch64-unknown-linux-gnu",
        feature: "neon",
    },
    Target {
        triple: "wasm32-unknown-unknown",
        feature: "simd128",
    },
];

/// A deterministic generator (xorshift64*), so the corpus is the same on
/// every host.
struct Seeded(u64);

impl Seeded {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    fn below(&mut self, bound: u64) -> u64 {
        self.next() % bound
    }
    /// A lane: half the time an edge of the range or zero, else uniform.
    fn lane(&mut self, (low, high): Range) -> i32 {
        if self.below(2) == 0 {
            let edges = [
                low,
                high,
                0_i32.max(low),
                (-1_i32).max(low),
                1_i32.min(high),
                low + 1,
                high - 1,
            ];
            edges[usize::try_from(self.below(7)).expect("an index")]
        } else {
            let span = u64::try_from(i64::from(high) - i64::from(low) + 1).expect("a span");
            low + i32::try_from(self.below(span)).expect("a lane")
        }
    }
}

/// One case: the instruction and its operands.
type Case = (usize, Vec<i32>, Vec<i32>);

/// The corpus of an instruction: every lane at one edge and at the other, the
/// edges alternating, then seeded cases.
fn corpus(code: usize) -> Vec<Case> {
    let instruction = &INSTRUCTIONS[code];
    let mut seeded = Seeded(0x9e37_79b9_7f4a_7c15 ^ u64::try_from(code).expect("a code") << 32);
    let (first_lanes, first_range) = instruction.first;
    let second = instruction.second;
    let mut cases = Vec::new();
    let patterns: [fn(Range, usize) -> i32; 5] = [
        |(low, _), _| low,
        |(_, high), _| high,
        |(low, high), index| if index % 2 == 0 { low } else { high },
        |(low, high), index| if index % 2 == 0 { high } else { low },
        |_, _| 0,
    ];
    for first_pattern in patterns {
        for second_pattern in patterns {
            cases.push((
                code,
                (0..first_lanes)
                    .map(|index| first_pattern(first_range, index))
                    .collect(),
                second.map_or_else(Vec::new, |(lanes, range)| {
                    (0..lanes)
                        .map(|index| second_pattern(range, index))
                        .collect()
                }),
            ));
        }
    }
    for _ in 0..SEEDED {
        cases.push((
            code,
            (0..first_lanes).map(|_| seeded.lane(first_range)).collect(),
            second.map_or_else(Vec::new, |(lanes, range)| {
                (0..lanes).map(|_| seeded.lane(range)).collect()
            }),
        ));
    }
    cases
}

fn list(lanes: &[i32]) -> Value {
    Value::List {
        items: lanes
            .iter()
            .map(|lane| Value::Int {
                value: lane.to_string(),
            })
            .collect(),
    }
}

/// The lanes of an evaluator result. They are wider than a lane of the
/// instruction, so that a model that forgets to wrap or saturate disagrees
/// with the instruction instead of failing to parse.
fn lanes_of(observed: &Json) -> Vec<i64> {
    assert_eq!(observed["kind"], "list", "a list of lanes: {observed}");
    observed["items"]
        .as_array()
        .expect("items")
        .iter()
        .map(|item| {
            item["value"]
                .as_str()
                .expect("an integer")
                .parse()
                .expect("a lane")
        })
        .collect()
}

/// What the evaluator makes of every case of a root.
fn evaluate(lowered: &Lowered, cases: &[Case]) -> Vec<Vec<i64>> {
    cases
        .iter()
        .map(|(_, first, second)| {
            let mut arguments = vec![list(first)];
            if !second.is_empty() {
                arguments.push(list(second));
            }
            let outcome = interp::run(&lowered.program, FUEL, lowered.entry(), &arguments);
            let observed = rust::observable(&outcome)
                .unwrap_or_else(|| panic!("{first:?} {second:?}: {outcome:?}"));
            lanes_of(&observed)
        })
        .collect()
}

/// The Rust source of the harness: the cases, the instruction of every
/// modeled target of the architecture it is compiled for, and an entry that
/// prints the lanes of each case (a native program) or exports them (a
/// WebAssembly module).
fn harness(cases: &[Case]) -> String {
    let mut source = String::from(HARNESS_HEAD);
    source.push_str("static CASES: &[(u32, &[i32], &[i32])] = &[\n");
    for (code, first, second) in cases {
        let _ = writeln!(source, "    ({code}, &{first:?}, &{second:?}),");
    }
    source.push_str("];\n");
    source.push_str(HARNESS_TAIL);
    source
}

const HARNESS_HEAD: &str = r#"#![allow(dead_code, unused_unsafe, unexpected_cfgs)]
use core::mem::transmute;

fn i8s<const N: usize>(v: &[i32]) -> [i8; N] {
    let mut o = [0i8; N];
    for (d, s) in o.iter_mut().zip(v) {
        *d = *s as i8;
    }
    o
}
fn u8s<const N: usize>(v: &[i32]) -> [u8; N] {
    let mut o = [0u8; N];
    for (d, s) in o.iter_mut().zip(v) {
        *d = *s as u8;
    }
    o
}
fn i16s<const N: usize>(v: &[i32]) -> [i16; N] {
    let mut o = [0i16; N];
    for (d, s) in o.iter_mut().zip(v) {
        *d = *s as i16;
    }
    o
}
fn widen<T: Copy + Into<i32>>(r: &[T]) -> Vec<i32> {
    r.iter().map(|x| (*x).into()).collect()
}

#[cfg(target_arch = "x86_64")]
mod arch {
    use super::*;
    use core::arch::x86_64::*;

    #[target_feature(enable = "avx2")]
    fn maddubs(a: &[i32], b: &[i32]) -> Vec<i32> {
        let r: [i16; 16] =
            unsafe { transmute(_mm256_maddubs_epi16(transmute(u8s::<32>(a)), transmute(i8s::<32>(b)))) };
        widen(&r)
    }
    #[target_feature(enable = "avx2")]
    fn madd(a: &[i32], b: &[i32]) -> Vec<i32> {
        let r: [i32; 8] =
            unsafe { transmute(_mm256_madd_epi16(transmute(i16s::<16>(a)), transmute(i16s::<16>(b)))) };
        widen(&r)
    }
    #[target_feature(enable = "avx2")]
    fn adds(a: &[i32], b: &[i32]) -> Vec<i32> {
        let r: [i16; 16] =
            unsafe { transmute(_mm256_adds_epi16(transmute(i16s::<16>(a)), transmute(i16s::<16>(b)))) };
        widen(&r)
    }
    pub fn compute(op: u32, a: &[i32], b: &[i32]) -> Vec<i32> {
        assert!(is_x86_feature_detected!("avx2"), "the host has no AVX2");
        unsafe {
            match op {
                0 => maddubs(a, b),
                1 => madd(a, b),
                2 => adds(a, b),
                _ => panic!("no such instruction"),
            }
        }
    }
}

#[cfg(target_arch = "aarch64")]
mod arch {
    use super::*;
    use core::arch::aarch64::*;

    #[target_feature(enable = "neon")]
    fn mull(a: &[i32], b: &[i32]) -> Vec<i32> {
        let r: [i16; 8] = unsafe { transmute(vmull_s8(transmute(i8s::<8>(a)), transmute(i8s::<8>(b)))) };
        widen(&r)
    }
    #[target_feature(enable = "neon")]
    fn paddl(a: &[i32], _b: &[i32]) -> Vec<i32> {
        let r: [i32; 4] = unsafe { transmute(vpaddlq_s16(transmute(i16s::<8>(a)))) };
        widen(&r)
    }
    #[target_feature(enable = "neon")]
    fn qadd(a: &[i32], b: &[i32]) -> Vec<i32> {
        let r: [i16; 8] = unsafe { transmute(vqaddq_s16(transmute(i16s::<8>(a)), transmute(i16s::<8>(b)))) };
        widen(&r)
    }
    pub fn compute(op: u32, a: &[i32], b: &[i32]) -> Vec<i32> {
        unsafe {
            match op {
                3 => mull(a, b),
                4 => paddl(a, b),
                5 => qadd(a, b),
                _ => panic!("no such instruction"),
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod arch {
    use super::*;
    use core::arch::wasm32::*;

    #[target_feature(enable = "simd128")]
    fn extmul(a: &[i32], b: &[i32]) -> Vec<i32> {
        let r: [i16; 8] =
            unsafe { transmute(i16x8_extmul_low_i8x16(transmute(i8s::<16>(a)), transmute(i8s::<16>(b)))) };
        widen(&r)
    }
    #[target_feature(enable = "simd128")]
    fn dot(a: &[i32], b: &[i32]) -> Vec<i32> {
        let r: [i32; 4] = unsafe { transmute(i32x4_dot_i16x8(transmute(i16s::<8>(a)), transmute(i16s::<8>(b)))) };
        widen(&r)
    }
    #[target_feature(enable = "simd128")]
    fn add_sat(a: &[i32], b: &[i32]) -> Vec<i32> {
        let r: [i16; 8] = unsafe { transmute(i16x8_add_sat(transmute(i16s::<8>(a)), transmute(i16s::<8>(b)))) };
        widen(&r)
    }
    pub fn compute(op: u32, a: &[i32], b: &[i32]) -> Vec<i32> {
        match op {
            6 => extmul(a, b),
            7 => dot(a, b),
            8 => add_sat(a, b),
            _ => panic!("no such instruction"),
        }
    }
}

"#;

const HARNESS_TAIL: &str = r#"
#[cfg(not(any(target_arch = "wasm32", lib_build)))]
fn main() {
    for (index, (op, a, b)) in CASES.iter().enumerate() {
        let lanes: Vec<String> = arch::compute(*op, a, b).iter().map(|v| v.to_string()).collect();
        println!("{index} {}", lanes.join(","));
    }
}

#[cfg(target_arch = "wasm32")]
mod exports {
    static mut OUT: [i32; 64] = [0; 64];
    #[no_mangle]
    pub extern "C" fn count() -> u32 {
        super::CASES.len() as u32
    }
    #[no_mangle]
    pub extern "C" fn out_ptr() -> *const i32 {
        core::ptr::addr_of!(OUT) as *const i32
    }
    #[no_mangle]
    pub extern "C" fn run(index: u32) -> u32 {
        let (op, a, b) = super::CASES[index as usize];
        let lanes = super::arch::compute(op, a, b);
        unsafe {
            let out = &mut *core::ptr::addr_of_mut!(OUT);
            for (d, s) in out.iter_mut().zip(&lanes) {
                *d = *s;
            }
        }
        lanes.len() as u32
    }
}
"#;

/// The driver `node` runs over the module: the lanes of every case, in the
/// native harness's format.
const NODE_DRIVER: &str = r"
const fs = require('fs');
const wasm = new WebAssembly.Instance(new WebAssembly.Module(fs.readFileSync(process.argv[2])), {}).exports;
const lines = [];
for (let index = 0; index < wasm.count(); index++) {
  const length = wasm.run(index);
  lines.push(index + ' ' + Array.from(new Int32Array(wasm.memory.buffer, wasm.out_ptr(), length)).join(','));
}
console.log(lines.join('\n'));
";

/// How a target's instructions were checked.
enum Ran {
    /// The instruction ran and its lanes are these, one entry per case.
    Executed(Vec<Vec<i32>>),
    /// The harness was built for the target by the pinned rustc and not run.
    BuiltOnly,
    /// The standard library of the target is not installed here, so the
    /// harness was neither built nor run.
    NotBuilt,
}

/// Whether the pinned toolchain has the standard library of `triple`,
/// looked up from the repository root so that `rustup` selects the pinned
/// toolchain.
fn std_installed(triple: &str) -> bool {
    let output = Command::new("rustc")
        .current_dir(support::repo_root().as_std_path())
        .args(["--print", "sysroot"])
        .output()
        .expect("rustc runs");
    let sysroot = String::from_utf8(output.stdout).expect("utf8");
    Path::new(sysroot.trim())
        .join("lib/rustlib")
        .join(triple)
        .is_dir()
}

/// Run `rustc` from the repository root, so that `rustup` selects the
/// toolchain `rust-toolchain.toml` pins.
fn rustc(arguments: &[&str]) {
    let output = Command::new("rustc")
        .current_dir(support::repo_root().as_std_path())
        .args(arguments)
        .output()
        .expect("rustc runs");
    assert!(
        output.status.success(),
        "rustc {arguments:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn parse_lines(stdout: &str, cases: usize) -> Vec<Vec<i32>> {
    let rows: Vec<Vec<i32>> = stdout
        .lines()
        .map(|line| {
            let (_, lanes) = line.split_once(' ').expect("an index and lanes");
            lanes
                .split(',')
                .filter(|lane| !lane.is_empty())
                .map(|lane| lane.parse().expect("a lane"))
                .collect()
        })
        .collect();
    assert_eq!(rows.len(), cases, "one line per case");
    rows
}

/// The instructions of one target, run if the host can run them.
fn run_target(directory: &Path, target: usize, cases: &[Case]) -> Ran {
    let triple = TARGETS[target].triple;
    let source = directory.join(format!("harness_{target}.rs"));
    std::fs::write(&source, harness(cases)).expect("the harness");
    let source = source.to_str().expect("a path");
    // The executable carries the host's suffix (`.exe` on Windows), or the
    // host would not find it to run.
    let output = directory.join(format!("harness_{target}{}", std::env::consts::EXE_SUFFIX));
    let output = output.to_str().expect("a path");
    // A target the host executes natively is compiled for the host (no
    // `--target`, so it needs no extra standard library and runs on every
    // operating system of that architecture); any other target is compiled
    // with `--target`.
    let args = |extra: &[&'static str], cross: bool| -> Vec<String> {
        let mut args: Vec<String> = ["--edition=2021", "-C", "opt-level=1"]
            .into_iter()
            .map(str::to_owned)
            .collect();
        if cross {
            args.extend(["--target".to_owned(), triple.to_owned()]);
        }
        args.extend(extra.iter().map(|argument| (*argument).to_owned()));
        args
    };
    let compile = |mut arguments: Vec<String>, output: &str| {
        arguments.extend(["-o".to_owned(), output.to_owned(), source.to_owned()]);
        let borrowed: Vec<&str> = arguments.iter().map(String::as_str).collect();
        rustc(&borrowed);
    };
    match target {
        0 if cfg!(target_arch = "x86_64") && host_has_avx2() => {
            compile(args(&[], false), output);
            let ran = Command::new(output).output().expect("the harness runs");
            assert!(
                ran.status.success(),
                "{}",
                String::from_utf8_lossy(&ran.stderr)
            );
            Ran::Executed(parse_lines(
                &String::from_utf8(ran.stdout).expect("utf8"),
                cases.len(),
            ))
        }
        1 if cfg!(target_arch = "aarch64") => {
            compile(args(&[], false), output);
            let ran = Command::new(output).output().expect("the harness runs");
            assert!(
                ran.status.success(),
                "{}",
                String::from_utf8_lossy(&ran.stderr)
            );
            Ran::Executed(parse_lines(
                &String::from_utf8(ran.stdout).expect("utf8"),
                cases.len(),
            ))
        }
        2 if node_available() && std_installed(triple) => {
            let module = directory.join("harness_2.wasm");
            let module = module.to_str().expect("a path");
            compile(args(&["--crate-type=cdylib"], true), module);
            let driver = directory.join("driver.cjs");
            std::fs::write(&driver, NODE_DRIVER).expect("the driver");
            let ran = Command::new("node")
                .arg(&driver)
                .arg(module)
                .output()
                .expect("node runs");
            assert!(
                ran.status.success(),
                "{}",
                String::from_utf8_lossy(&ran.stderr)
            );
            Ran::Executed(parse_lines(
                &String::from_utf8(ran.stdout).expect("utf8"),
                cases.len(),
            ))
        }
        _ => {
            // The host cannot execute this target: build the harness for it,
            // so that the intrinsics at least compile under the attribute.
            // The architecture of the host builds natively; any other target
            // is built only where its standard library is installed.
            let native = (target == 0 && cfg!(target_arch = "x86_64"))
                || (target == 1 && cfg!(target_arch = "aarch64"));
            if !native && !std_installed(triple) {
                return Ran::NotBuilt;
            }
            let library = directory.join(format!("libharness_{target}.rlib"));
            compile(
                args(&["--crate-type=lib", "--cfg", "lib_build"], !native),
                library.to_str().expect("a path"),
            );
            Ran::BuiltOnly
        }
    }
}

fn host_has_avx2() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        std::arch::is_x86_feature_detected!("avx2")
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        false
    }
}

fn node_available() -> bool {
    Command::new("node")
        .arg("--version")
        .output()
        .is_ok_and(|output| output.status.success())
}

/// The roots' lowered programs, after checking the eligibility report of the
/// module declares exactly the nine instruction roots and their effects, and
/// the report.
fn lowered_roots(project: &P) -> (BTreeMap<&'static str, Lowered>, Json) {
    project.check_ok();
    let built = project.build_ok();
    let build_id = built.build_id.expect("built");
    let report: Json = serde_json::from_slice(
        &std::fs::read(
            project
                .build_dir(&build_id)
                .join("production/LexLeanStd/Target.eligibility.json")
                .as_std_path(),
        )
        .expect("the eligibility report"),
    )
    .expect("the report is JSON");
    let mut reported = report["roots"].as_array().expect("roots").clone();
    // The tenth root prices the nine (`GN-10`); it computes no lane.
    assert_eq!(
        reported.pop().map(|row| row["root"].clone()),
        Some(Json::from(format!("{MODULE}.{PRICE_ROOT}"))),
        "the price root follows the instruction roots"
    );
    assert_eq!(
        reported
            .iter()
            .map(|row| row["root"].as_str().expect("root").to_owned())
            .collect::<Vec<_>>(),
        INSTRUCTIONS
            .iter()
            .map(|instruction| format!("{MODULE}.{}", instruction.root))
            .collect::<Vec<_>>(),
        "the nine instruction roots are the module's other production roots, in order"
    );
    for (row, instruction) in reported.iter().zip(&INSTRUCTIONS) {
        let declared: Vec<&str> = row["declared_effects"]
            .as_array()
            .expect("declared effects")
            .iter()
            .map(|effect| effect.as_str().expect("an effect"))
            .collect();
        assert_eq!(
            declared, EFFECTS,
            "{}: the declared effects",
            instruction.root
        );
        let targets = row["targets"].as_array().expect("targets");
        assert_eq!(targets.len(), 1, "{}: one target", instruction.root);
        assert_eq!(targets[0]["target"], "rust-std");
        assert_eq!(targets[0]["status"], "eligible");
        let realized: Vec<&str> = targets[0]["effects"]
            .as_array()
            .expect("effects")
            .iter()
            .map(|effect| effect["effect"].as_str().expect("an effect"))
            .collect();
        assert_eq!(
            realized, EFFECTS,
            "{}: the declaration admits exactly the realized effects",
            instruction.root
        );
    }
    let checked = support::checked_project(project);
    let lowered = INSTRUCTIONS
        .iter()
        .map(|instruction| {
            (
                instruction.root,
                lower_in(&checked, project, MODULE, instruction.root),
            )
        })
        .collect();
    (lowered, report)
}

/// Run the case for `GN-09`.
///
/// # Panics
///
/// Panics when the case's assertion fails.
pub(super) fn run(id: &str) {
    assert_eq!(id, "GN-09");
    // D4: three targets, and the DSP target waits for stable, safe intrinsics.
    assert_eq!(
        TARGETS.map(|target| (target.triple, target.feature)),
        [
            ("x86_64-unknown-linux-gnu", "avx2"),
            ("aarch64-unknown-linux-gnu", "neon"),
            ("wasm32-unknown-unknown", "simd128"),
        ]
    );
    let project = P::stdlib();
    let (lowered, _) = lowered_roots(&project);

    // The evaluator on the corpus of every instruction.
    let cases: Vec<Vec<Case>> = (0..INSTRUCTIONS.len()).map(corpus).collect();
    let modeled: Vec<Vec<Vec<i64>>> = INSTRUCTIONS
        .iter()
        .enumerate()
        .map(|(code, instruction)| evaluate(&lowered[instruction.root], &cases[code]))
        .collect();

    // The instruction itself, target by target.
    let directory = tempfile::Builder::new()
        .prefix("lexlean-target-machine-")
        .tempdir()
        .expect("tempdir");
    let mut oracles: BTreeMap<usize, Vec<Vec<i64>>> = BTreeMap::new();
    for (target, description) in TARGETS.iter().enumerate() {
        let codes: Vec<usize> = (0..INSTRUCTIONS.len())
            .filter(|code| INSTRUCTIONS[*code].target == target)
            .collect();
        let target_cases: Vec<Case> = codes
            .iter()
            .flat_map(|code| cases[*code].iter().cloned())
            .collect();
        match run_target(directory.path(), target, &target_cases) {
            Ran::Executed(observed) => {
                let mut position = 0;
                for code in codes {
                    let rows: Vec<Vec<i64>> = observed[position..position + cases[code].len()]
                        .iter()
                        .map(|row| row.iter().map(|lane| i64::from(*lane)).collect())
                        .collect();
                    position += rows.len();
                    let disagreements: Vec<String> = rows
                        .iter()
                        .zip(&modeled[code])
                        .zip(&cases[code])
                        .filter(|((instruction, model), _)| instruction != model)
                        .map(|((instruction, model), (_, first, second))| {
                            format!("{first:?} {second:?}: instruction {instruction:?}, model {model:?}")
                        })
                        .collect();
                    assert!(
                        disagreements.is_empty(),
                        "{} on {} (+{}): the model and the instruction disagree on {} of {} cases: {:?}",
                        INSTRUCTIONS[code].root,
                        description.triple,
                        description.feature,
                        disagreements.len(),
                        rows.len(),
                        &disagreements[..disagreements.len().min(3)]
                    );
                    oracles.insert(code, rows);
                }
                eprintln!(
                    "GN-09: {} (+{}): executed {} cases",
                    description.triple,
                    description.feature,
                    target_cases.len()
                );
            }
            Ran::NotBuilt => {
                assert!(
                    !support::normative_host(),
                    "{}: the normative host builds or runs every target",
                    description.triple
                );
                eprintln!(
                    "GN-09: {} (+{}): not built on this host",
                    description.triple, description.feature
                );
            }
            Ran::BuiltOnly => {
                assert!(
                    !(target == 0 || target == 2) || !support::normative_host(),
                    "{}: the normative host runs this target (AVX2, and Node.js for SIMD128)",
                    description.triple
                );
                eprintln!(
                    "GN-09: {} (+{}): built, not executed on this host",
                    description.triple, description.feature
                );
            }
        }
    }

    // The gate is armed: a saturation bound one past the lane's maximum, and
    // the model disagrees with an instruction that ran.
    let (code, oracle) = oracles
        .iter()
        .find(|(code, _)| {
            // Each host executes at least one of the three saturating adds
            // (AVX2 `adds`, NEON `qaddq`, SIMD128 `add_sat`), all over
            // `saturate16`, the plant's site.
            ["Adds", "Qadd", "AddSat"]
                .iter()
                .any(|name| INSTRUCTIONS[**code].root.contains(name))
        })
        .map(|(code, oracle)| (*code, oracle.clone()))
        .expect("a saturating instruction ran");
    let planted = P::stdlib();
    let text = planted.read_raw("src/Target.lex.tex");
    let site = "(prim less_than (32767 : Int) value : Bool) then (32767 : Int)";
    assert_eq!(text.matches(site).count(), 1, "the plant site exists once");
    planted.write_raw(
        "src/Target.lex.tex",
        &text.replacen(
            site,
            "(prim less_than (32768 : Int) value : Bool) then (32768 : Int)",
            1,
        ),
    );
    let checked = support::checked_project(&planted);
    let root = lower_in(&checked, &planted, MODULE, INSTRUCTIONS[code].root);
    assert_ne!(
        evaluate(&root, &cases[code]),
        oracle,
        "a planted wrong saturation bound is detected by the differential"
    );

    // Under the pinned Lean the module's theorems and the certificates of its
    // roots are kernel-checked, and a false theorem is refused.
    if !support::lean_backed("GN-09") {
        return;
    }
    let falsified = P::stdlib();
    let text = falsified.read_raw("src/Target.lex.tex");
    let start = text
        .find("name: maddubs_saturates_high,")
        .expect("the theorem");
    let site = "(32767 : Int)";
    let offset = start
        + text[start..]
            .find(site)
            .expect("the statement names the saturated lane");
    let mut changed = text.clone();
    changed.replace_range(offset..offset + site.len(), "(32766 : Int)");
    falsified.write_raw("src/Target.lex.tex", &changed);
    {
        let _environment = support::env_shared();
        falsified.verify_fails_with("LLV7002");
    }
    let verified = support::verify_ok(&project);
    let attestation: Json = serde_json::from_slice(
        &std::fs::read(
            project
                .root
                .join(".lexlean/verified")
                .join(verified.attestation_id.to_hex())
                .join("attestation.json")
                .as_std_path(),
        )
        .expect("attestation"),
    )
    .expect("the attestation is JSON");
    let rows = attestation["declarations"]
        .as_array()
        .expect("declarations");
    for theorem in THEOREMS {
        let full = format!("{MODULE}.{theorem}");
        let row = rows
            .iter()
            .find(|row| row["name"].as_str() == Some(full.as_str()))
            .unwrap_or_else(|| panic!("`{full}` is audited"));
        assert!(
            row["observed"].as_array().is_some_and(|observed| observed
                .iter()
                .all(|axiom| matches!(axiom.as_str(), Some("propext" | "Quot.sound")))),
            "`{full}` observes no axiom but propext and Quot.sound: {row}"
        );
    }
}

/// The theorems of the module, which the kernel checks.
const THEOREMS: [&str; 20] = [
    "maddubs_saturates_high",
    "maddubs_saturates_low",
    "maddubs_exact",
    "madd_wraps",
    "madd_exact",
    "adds_saturates",
    "mull_widens",
    "mull_takes_eight",
    "paddl_widens",
    "qadd_saturates",
    "extmul_takes_low_eight",
    "dot_wraps",
    "add_sat_saturates",
    "price_reads_whole_instructions",
    "price_bytes",
    "price_empty",
    "extmul_price_counts_products",
    "instruction_price_names_nine",
    "worst_case_is_largest",
    "worst_case_bounded_by_operations",
];

// ---------------------------------------------------------------------------
// `GN-10`: the price of each instruction, held to the target.
//
// `GN-09` holds the semantics of an instruction to the instruction; the price
// it states (`price`: whole instructions over the lanes, and the bytes their
// operands read) would otherwise be held only to itself. Two oracles below
// check it (C9): the code the pinned rustc emits for each intrinsic, which
// fixes how many instructions one call is and how wide the registers it reads
// are, and the instruction's own execution, which fixes how many lanes of its
// first operand one instruction consumes.
// ---------------------------------------------------------------------------

/// The production root that prices instruction `code` over a lane count.
const PRICE_ROOT: &str = "instructionPrice";

/// The lane counts the price is read at: every count up to two and a half
/// registers of the widest instruction.
const PRICED_LANES: u64 = 80;

/// What the module charges one instruction: the lanes of its first operand
/// one instruction covers, and the bytes it reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Charge {
    lanes: u64,
    bytes: u64,
}

/// The mnemonic each intrinsic is, in the target's assembly syntax, and, for
/// WebAssembly, whose text names no operand, the number of `v128` operands it
/// pops (the SIMD proposal's signature of the instruction: two for each).
const MNEMONICS: [(&str, &str, u64); 9] = [
    ("_mm256_maddubs_epi16", "vpmaddubsw", 0),
    ("_mm256_madd_epi16", "vpmaddwd", 0),
    ("_mm256_adds_epi16", "vpaddsw", 0),
    ("vmull_s8", "smull", 0),
    ("vpaddlq_s16", "saddlp", 0),
    ("vqaddq_s16", "sqadd", 0),
    ("i16x8_extmul_low_i8x16", "i16x8.extmul_low_i8x16_s", 2),
    ("i32x4_dot_i16x8", "i32x4.dot_i16x8_s", 2),
    ("i16x8_add_sat", "i16x8.add_sat_s", 2),
];

/// The prices the module states, read by the evaluator from the lowered
/// root: for every instruction, `c` and `b` at every lane count up to
/// [`PRICED_LANES`], reduced to the charge of one instruction after checking
/// that `c` is the whole instructions that cover the lanes and `b` is `c`
/// times one instruction's bytes. A code past the nine has no price.
fn charges(lowered: &Lowered) -> Result<Vec<Charge>, String> {
    let price = |code: u64, lanes: u64| -> Json {
        let arguments = [
            Value::Nat {
                value: code.to_string(),
            },
            Value::Nat {
                value: lanes.to_string(),
            },
        ];
        let outcome = interp::run(&lowered.program, FUEL, lowered.entry(), &arguments);
        rust::observable(&outcome).unwrap_or_else(|| panic!("{code} {lanes}: {outcome:?}"))
    };
    let nat = |value: &Json| -> u64 {
        assert_eq!(value["kind"], "nat", "a natural number: {value}");
        value["value"]
            .as_str()
            .expect("digits")
            .parse()
            .expect("a count")
    };
    let mut charges = Vec::new();
    for code in 0..INSTRUCTIONS.len() as u64 {
        let rows: Vec<(u64, u64)> = (0..=PRICED_LANES)
            .map(|lanes| {
                let observed = price(code, lanes);
                assert_eq!(
                    observed["kind"], "some",
                    "code {code} is priced: {observed}"
                );
                let pair = &observed["value"];
                assert_eq!(pair["kind"], "pair", "a step vector: {pair}");
                (nat(&pair["left"]), nat(&pair["right"]))
            })
            .collect();
        let lanes = rows
            .iter()
            .rposition(|(operations, _)| *operations == 1)
            .ok_or_else(|| format!("code {code}: no lane count is one instruction"))?
            as u64;
        let bytes = rows[1].1;
        for (count, (operations, read)) in (0u64..).zip(&rows) {
            if *operations != count.div_ceil(lanes) || *read != operations * bytes {
                return Err(format!(
                    "code {code}: {count} lanes cost ({operations}, {read}), not whole instructions of {lanes} lanes and {bytes} bytes"
                ));
            }
        }
        charges.push(Charge { lanes, bytes });
    }
    let past = price(INSTRUCTIONS.len() as u64, 1);
    assert_eq!(
        past["kind"], "none",
        "a code past the nine has no price: {past}"
    );
    Ok(charges)
}

/// The source compiled to assembly for one target: one function per
/// instruction, each one call of the intrinsic under `#[target_feature]`, in
/// a crate that forbids unsafe code. `#[inline(never)]` keeps rustc from
/// treating a small leaf as cross-crate inlinable and emitting no code for
/// it, which it does on AArch64.
fn kernel(target: usize) -> String {
    let (module, feature, vector, unary) = match target {
        0 => ("x86_64", "avx2", "__m256i", None),
        1 => ("aarch64", "neon", "", Some(4)),
        _ => ("wasm32", "simd128", "v128", None),
    };
    let mut source = format!(
        "#![no_std]\n#![forbid(unsafe_code)]\n#![allow(dead_code, improper_ctypes_definitions)]\nuse core::arch::{module}::*;\n"
    );
    // The Rust ABI passes a WebAssembly `v128` through memory, and LLVM then
    // folds a widening load into the operation (`extmul_low` becomes two
    // `load8x8_s` and an `i16x8.mul`); the C ABI passes it as a `v128` value,
    // so the function body is the instruction itself.
    let abi = if target == 2 { "extern \"C\" " } else { "" };
    for (code, instruction) in INSTRUCTIONS.iter().enumerate() {
        if instruction.target != target {
            continue;
        }
        let intrinsic = MNEMONICS[code].0;
        let signature = match (target, code) {
            (1, 3) => "a: int8x8_t, b: int8x8_t) -> int16x8_t".to_owned(),
            (1, 4) => "a: int16x8_t) -> int32x4_t".to_owned(),
            (1, _) => "a: int16x8_t, b: int16x8_t) -> int16x8_t".to_owned(),
            _ => format!("a: {vector}, b: {vector}) -> {vector}"),
        };
        let call = if unary == Some(code) { "a" } else { "a, b" };
        let _ = writeln!(
            source,
            "#[inline(never)]\n#[target_feature(enable = \"{feature}\")]\npub {abi}fn lexlean_price_{code}({signature} {{ {intrinsic}({call}) }}"
        );
    }
    source
}

/// What the assembly of one function says about its instruction: how many
/// times the mnemonic occurs, and the bytes of the registers each occurrence
/// reads.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Emitted {
    occurrences: u64,
    bytes: Vec<u64>,
}

/// Split an operand list at the commas outside parentheses, so that an x86
/// memory operand such as `(%rax,%rcx,2)` stays one operand.
fn operands(text: &str) -> Vec<String> {
    let mut parts = vec![String::new()];
    let mut depth = 0usize;
    for character in text.chars() {
        match character {
            '(' => depth += 1,
            ')' => depth = depth.saturating_sub(1),
            ',' if depth == 0 => {
                parts.push(String::new());
                continue;
            }
            _ => {}
        }
        parts.last_mut().expect("a part").push(character);
    }
    parts
        .into_iter()
        .map(|part| part.trim().to_owned())
        .collect()
}

/// The bytes of an x86 vector register operand, or `None` for memory.
fn x86_width(operand: &str) -> Option<u64> {
    if operand.starts_with("%ymm") {
        Some(32)
    } else if operand.starts_with("%xmm") {
        Some(16)
    } else {
        None
    }
}

/// The bytes of an AArch64 vector operand in its arrangement (`v1.8b` is
/// eight one-byte lanes).
fn arrangement_width(operand: &str) -> u64 {
    let (_, arrangement) = operand.split_once('.').expect("an arrangement");
    let lanes: u64 = arrangement[..arrangement.len() - 1]
        .parse()
        .expect("a lane count");
    let size = match arrangement.chars().last().expect("a lane size") {
        'b' => 1,
        'h' => 2,
        's' => 4,
        'd' => 8,
        other => panic!("an arrangement of lane size {other}"),
    };
    lanes * size
}

/// The function of `code` in the assembly `text` for `target`, read as
/// [`Emitted`].
fn emitted(text: &str, target: usize, code: usize) -> Emitted {
    let label = format!("lexlean_price_{code}");
    let start = text
        .lines()
        .position(|line| line.ends_with(':') && line.contains(&label))
        .unwrap_or_else(|| panic!("the assembly defines {label}"));
    let body: Vec<&str> = text
        .lines()
        .skip(start + 1)
        .take_while(|line| !line.starts_with(".Lfunc_end") && line.trim() != "end_function")
        .collect();
    let mnemonic = MNEMONICS[code].1;
    let mut found = Emitted {
        occurrences: 0,
        bytes: Vec::new(),
    };
    for line in body {
        let line = line.trim();
        let (head, rest) = line.split_once(char::is_whitespace).unwrap_or((line, ""));
        if head != mnemonic {
            continue;
        }
        found.occurrences += 1;
        let parts = operands(rest);
        let read: u64 = match target {
            // AT&T order: the sources, then the destination, whose register
            // is the width a memory source is read at.
            0 => {
                let (destination, sources) = parts.split_last().expect("operands");
                let width = x86_width(destination).expect("a vector destination");
                sources
                    .iter()
                    .map(|source| x86_width(source).unwrap_or(width))
                    .sum()
            }
            // The destination, then the sources.
            1 => parts[1..]
                .iter()
                .map(|source| arrangement_width(source.as_str()))
                .sum(),
            // A stack machine: the instruction pops its `v128` operands.
            _ => MNEMONICS[code].2 * 16,
        };
        found.bytes.push(read);
    }
    found
}

/// The assembly of every modeled instruction of `target`, compiled by the
/// pinned rustc with optimization; `None` when the target's standard library
/// is not installed.
fn assembly(directory: &Path, target: usize) -> Option<String> {
    let triple = TARGETS[target].triple;
    if !std_installed(triple) {
        return None;
    }
    let source = directory.join(format!("kernel_{target}.rs"));
    std::fs::write(&source, kernel(target)).expect("the kernel");
    let output = directory.join(format!("kernel_{target}.s"));
    rustc(&[
        "--edition=2021",
        "--crate-type=lib",
        "-C",
        "opt-level=3",
        "--emit=asm",
        "--target",
        triple,
        "-o",
        output.to_str().expect("a path"),
        source.to_str().expect("a path"),
    ]);
    Some(std::fs::read_to_string(&output).expect("the assembly"))
}

/// The cases that find the lanes an instruction consumes: every lane of both
/// operands at one, then each lane of the first operand in turn at two.
/// Small lanes, so that no product or sum reaches a saturation or wrap edge
/// that would hide a change.
fn dependence_cases(code: usize) -> Vec<Case> {
    let instruction = &INSTRUCTIONS[code];
    let first = vec![1; instruction.first.0];
    let second = instruction
        .second
        .map_or_else(Vec::new, |(lanes, _)| vec![1; lanes]);
    let mut cases = vec![(code, first.clone(), second.clone())];
    for lane in 0..first.len() {
        let mut changed = first.clone();
        changed[lane] = 2;
        cases.push((code, changed, second.clone()));
    }
    cases
}

/// The lanes of the first operand whose change changes the result, from the
/// results of [`dependence_cases`].
fn dependent(rows: &[Vec<i64>]) -> Vec<u64> {
    let (base, changed) = rows.split_first().expect("the base case");
    (0u64..)
        .zip(changed)
        .filter(|(_, row)| *row != base)
        .map(|(lane, _)| lane)
        .collect()
}

/// Whether the charges agree with what the target says: one instruction per
/// intrinsic reading the charged bytes, and exactly the charged lanes of the
/// first operand consumed.
fn agree(
    charges: &[Charge],
    emitted: &BTreeMap<usize, Emitted>,
    consumed: &BTreeMap<usize, Vec<u64>>,
) -> Result<(), String> {
    for (code, charge) in charges.iter().enumerate() {
        let root = INSTRUCTIONS[code].root;
        if let Some(found) = emitted.get(&code) {
            if found.occurrences != 1 || found.bytes != [charge.bytes] {
                return Err(format!(
                    "{root}: the module charges one instruction reading {} bytes; the pinned rustc emits `{}` {} time(s) reading {:?}",
                    charge.bytes, MNEMONICS[code].1, found.occurrences, found.bytes
                ));
            }
        }
        if let Some(lanes) = consumed.get(&code) {
            if *lanes != (0..charge.lanes).collect::<Vec<_>>() {
                return Err(format!(
                    "{root}: the module charges one instruction per {} lanes; the result depends on lanes {lanes:?} of the first operand",
                    charge.lanes
                ));
            }
        }
    }
    Ok(())
}

/// The price root's lowered program, after checking in the module's
/// eligibility `report` that it is a production root eligible for `rust-std`
/// with exactly the effect it declares.
fn price_root(project: &P, report: &Json) -> Lowered {
    let name = format!("{MODULE}.{PRICE_ROOT}");
    let row = report["roots"]
        .as_array()
        .expect("roots")
        .iter()
        .find(|row| row["root"].as_str() == Some(name.as_str()))
        .unwrap_or_else(|| panic!("`{name}` is a production root"));
    assert_eq!(row["declared_effects"], serde_json::json!(["overflow"]));
    let targets = row["targets"].as_array().expect("targets");
    assert_eq!(targets.len(), 1, "one target");
    assert_eq!(targets[0]["target"], "rust-std");
    assert_eq!(targets[0]["status"], "eligible");
    let realized: Vec<&str> = targets[0]["effects"]
        .as_array()
        .expect("effects")
        .iter()
        .map(|effect| effect["effect"].as_str().expect("an effect"))
        .collect();
    assert_eq!(
        realized,
        ["overflow"],
        "the declaration admits exactly the realized effect"
    );
    let checked = support::checked_project(project);
    lower_in(&checked, project, MODULE, PRICE_ROOT)
}

/// Read the charges of a copy of the module with `site` replaced by
/// `planted`.
fn planted_charges(site: &str, planted: &str) -> Vec<Charge> {
    let project = P::stdlib();
    let text = project.read_raw("src/Target.lex.tex");
    assert_eq!(
        text.matches(site).count(),
        1,
        "the plant site `{site}` exists once"
    );
    project.write_raw("src/Target.lex.tex", &text.replacen(site, planted, 1));
    let checked = support::checked_project(&project);
    charges(&lower_in(&checked, &project, MODULE, PRICE_ROOT)).expect("the planted price reads")
}

/// Run the case for `GN-10`.
///
/// # Panics
///
/// Panics when the case's assertion fails.
pub(super) fn run_price(id: &str) {
    assert_eq!(id, "GN-10");
    let project = P::stdlib();
    let (lowered, report) = lowered_roots(&project);
    let charges = charges(&price_root(&project, &report)).expect("the module's price reads");

    let directory = tempfile::Builder::new()
        .prefix("lexlean-target-price-")
        .tempdir()
        .expect("tempdir");

    // The first oracle: the code the pinned rustc emits.
    let mut emitted_by_code: BTreeMap<usize, Emitted> = BTreeMap::new();
    for (target, description) in TARGETS.iter().enumerate() {
        match assembly(directory.path(), target) {
            Some(text) => {
                for code in
                    (0..INSTRUCTIONS.len()).filter(|code| INSTRUCTIONS[*code].target == target)
                {
                    emitted_by_code.insert(code, emitted(&text, target, code));
                }
                eprintln!(
                    "GN-10: {} (+{}): assembly read",
                    description.triple, description.feature
                );
            }
            None => {
                assert!(
                    !support::normative_host(),
                    "{}: the normative host compiles every target",
                    description.triple
                );
                eprintln!(
                    "GN-10: {} (+{}): the standard library is not installed; no assembly",
                    description.triple, description.feature
                );
            }
        }
    }

    // The second oracle: the instruction run, beside the model, on the cases
    // that find the lanes it consumes.
    let mut consumed: BTreeMap<usize, Vec<u64>> = BTreeMap::new();
    for (target, description) in TARGETS.iter().enumerate() {
        let codes: Vec<usize> = (0..INSTRUCTIONS.len())
            .filter(|code| INSTRUCTIONS[*code].target == target)
            .collect();
        let cases: Vec<Vec<Case>> = codes.iter().map(|code| dependence_cases(*code)).collect();
        for (code, cases) in codes.iter().zip(&cases) {
            let modeled = evaluate(&lowered[INSTRUCTIONS[*code].root], cases);
            consumed.insert(*code, dependent(&modeled));
        }
        let all: Vec<Case> = cases.iter().flatten().cloned().collect();
        match run_target(directory.path(), target, &all) {
            Ran::Executed(observed) => {
                let mut position = 0;
                for (code, cases) in codes.iter().zip(&cases) {
                    let rows: Vec<Vec<i64>> = observed[position..position + cases.len()]
                        .iter()
                        .map(|row| row.iter().map(|lane| i64::from(*lane)).collect())
                        .collect();
                    position += cases.len();
                    assert_eq!(
                        dependent(&rows),
                        consumed[code],
                        "{}: the model and the instruction consume the same lanes",
                        INSTRUCTIONS[*code].root
                    );
                }
                eprintln!(
                    "GN-10: {} (+{}): lanes consumed, executed",
                    description.triple, description.feature
                );
            }
            Ran::BuiltOnly | Ran::NotBuilt => {
                assert!(
                    !(target == 0 || target == 2) || !support::normative_host(),
                    "{}: the normative host runs this target (AVX2, and Node.js for SIMD128)",
                    description.triple
                );
                eprintln!(
                    "GN-10: {} (+{}): lanes consumed by the model only; not executed on this host",
                    description.triple, description.feature
                );
            }
        }
    }

    if let Err(disagreement) = agree(&charges, &emitted_by_code, &consumed) {
        panic!("the price disagrees with the target: {disagreement}");
    }

    // The gate is armed: SIMD128's widening product charged per sixteen
    // lanes, as if it consumed its whole register, and an instruction charged
    // half the bytes it reads.
    let lanes = planted_charges("    | Op.wasmExtmul => 8\n", "    | Op.wasmExtmul => 16\n");
    assert!(
        agree(&lanes, &emitted_by_code, &consumed).is_err(),
        "a planted lane count is detected"
    );
    // The byte count is read from assembly, which a host without the
    // standard library of any Linux target of D4 does not produce; the
    // normative host produces all three, so the plant is always armed there.
    let Some((site, planted)) = [
        (
            "    | Op.avx2Maddubs => 64\n",
            "    | Op.avx2Maddubs => 32\n",
        ),
        ("    | Op.neonMull => 16\n", "    | Op.neonMull => 8\n"),
        ("    | Op.wasmExtmul => 32\n", "    | Op.wasmExtmul => 16\n"),
    ]
    .into_iter()
    .zip([0usize, 3, 6])
    .find(|(_, code)| emitted_by_code.contains_key(code))
    .map(|(plant, _)| plant) else {
        assert!(
            !support::normative_host(),
            "the normative host reads assembly"
        );
        eprintln!("GN-10: no assembly on this host; the byte plant is not armed here");
        return;
    };
    let bytes = planted_charges(site, planted);
    assert!(
        agree(&bytes, &emitted_by_code, &consumed).is_err(),
        "a planted byte count is detected"
    );
}
