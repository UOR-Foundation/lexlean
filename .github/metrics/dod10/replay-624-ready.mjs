// Advisory independent receipt verifier, #603/#624. NOT a production gate.
// Source head eeeb37bf072c88e124c7ac9b5cd4ce10fa286b1e; CI 193d77c8ffb7595e5f2627ce0a08b39d14f2841e;
// rows f35167d5551cec6db851140a6c3aa9955b91fb10; native lanes 8f848b1fa493e16ba224c74335c06eb5e0e04bd0;
// planner 8036447f5109226058995f6d55db291f4ca37aff;
// repro f1ca0057f94a71ba04f2080e598fdaad7aafa95c.
// Ready success roster derived from source. Skipped placeholder spelling is
// prospective for Ready; early skip roster observed on run 45900859536.
// Use only complete, paginated first-attempt provider receipts. Caller must
// authenticate provider data, establish run ID/event and verify custody separately.
// An unexpected provider shape is HOLD for inspection, never permission to merge.
const HEAD = "eeeb37bf072c88e124c7ac9b5cd4ce10fa286b1e";
const specs = {
  "early": {
    "workflow": ".github/workflows/ci.yml",
    "events": [
      "workflow_dispatch"
    ],
    "success": [
      "plan",
      "honesty",
      "just vv (rest)"
    ],
    "skipped": [
      "Intel test construction and local targets",
      "atlas (prefix)",
      "build and non-Lean tests (${{ matrix.target }})",
      "atlas (${{ matrix.shard }})",
      "Windows tests (${{ matrix.lane }})",
      "macOS arm64 tests (${{ matrix.lane }})",
      "atlas (closure)",
      "bootstrap export",
      "atlas (cert4-replay)",
      "Intel conformance (${{ matrix.lane }})",
      "build and non-Lean tests (aarch64-apple-darwin)",
      "build and non-Lean tests (x86_64-pc-windows-msvc)",
      "atlas (cert4)",
      "atlas (${{ matrix.shard }})",
      "atlas (${{ matrix.shard }})",
      "just vv (${{ matrix.part }})",
      "build and non-Lean tests (x86_64-apple-darwin)",
      "atlas (join)"
    ]
  },
  "ready": {
    "workflow": ".github/workflows/ci.yml",
    "events": [
      "pull_request",
      "workflow_dispatch"
    ],
    "success": [
      "plan",
      "honesty",
      "bootstrap export",
      "just vv (examples)",
      "just vv (rest)",
      "just vv (bdd-bootstrap-check)",
      "just vv (bdd-bootstrap-gnaf)",
      "just vv (bdd-bootstrap-print)",
      "just vv (bdd-bootstrap-runtime)",
      "just vv (bdd-bootstrap-lexicon-lre)",
      "just vv (bdd-bootstrap-lexicon-lse)",
      "just vv (bdd-bootstrap-project-text)",
      "just vv (bdd-bootstrap-tooling-latex)",
      "just vv (bdd-bootstrap-validate)",
      "just vv (bdd-bootstrap-lower)",
      "just vv (bdd-bootstrap-moves)",
      "just vv (bdd-bootstrap-bodies)",
      "just vv (bdd-bootstrap-succeed)",
      "just vv (bdd-bootstrap-loops)",
      "just vv (bdd-bootstrap-assemble)",
      "just vv (bdd-bootstrap-package)",
      "just vv (bdd-bootstrap-structural-grammar)",
      "just vv (bdd-bootstrap-surface-lex)",
      "just vv (bdd-bootstrap-surface-doc)",
      "just vv (bdd-bootstrap-surface-tooling)",
      "just vv (bdd-bootstrap-surface-generic)",
      "just vv (bdd-bootstrap-terms)",
      "just vv (bdd-compiler)",
      "just vv (bdd-preservation)",
      "just vv (bdd-preservation-lists)",
      "just vv (bdd-preservation-prims)",
      "just vv (bdd-preservation-std)",
      "just vv (bdd-preservation-runtime)",
      "just vv (bdd-preservation-support)",
      "just vv (bdd-production)",
      "just vv (bdd-verification)",
      "just vv (bdd-bootstrap-surface-module)",
      "just vv (bdd-bootstrap-diagnostics)",
      "just vv (bdd-bootstrap-cli-output)",
      "just vv (bdd-bootstrap-lock-text)",
      "just vv (bdd-bootstrap-content-id)",
      "just vv (bdd-bootstrap-manifest-text)",
      "just vv (bdd-bootstrap-grammar-proof)",
      "just vv (bdd-stdlib)",
      "just vv (bdd-bootstrap-semantic-print)",
      "just vv (bdd-bootstrap-semantic-read)",
      "just vv (bdd-bootstrap-core-json)",
      "just vv (bdd-bootstrap-core-print)",
      "just vv (bdd-bootstrap-core-read)",
      "just vv (bdd-bootstrap-grammar-math)",
      "just vv (bdd-bootstrap-proposition-terms)",
      "just vv (bdd-bootstrap-lexicon-schema)",
      "just vv (bdd-bootstrap-lse-scopes)",
      "just vv (bdd-bootstrap-lse-types)",
      "just vv (bdd-bootstrap-toml-read)",
      "just vv (bdd-bootstrap-package-ref)",
      "just vv (bdd-bootstrap-lexicon-entry)",
      "just vv (bdd-language-models)",
      "just vv (bdd-wasi)",
      "atlas (prefix)",
      "atlas (cert4)",
      "atlas (census-a)",
      "atlas (census-b)",
      "atlas (census-c)",
      "atlas (census-d)",
      "atlas (cert4-replay)",
      "atlas (closure)",
      "atlas (orbit)",
      "atlas (pairs-a)",
      "atlas (pairs-b)",
      "atlas (pairs-c)",
      "atlas (pairs-d)",
      "atlas (join)",
      "build and non-Lean tests (aarch64-unknown-linux-gnu)",
      "Windows tests (producer)",
      "Windows tests (lower)",
      "Windows tests (print)",
      "Windows tests (validate)",
      "Windows tests (moves)",
      "build and non-Lean tests (x86_64-pc-windows-msvc)",
      "macOS arm64 tests (producer)",
      "macOS arm64 tests (lower)",
      "macOS arm64 tests (print)",
      "macOS arm64 tests (validate)",
      "macOS arm64 tests (moves)",
      "build and non-Lean tests (aarch64-apple-darwin)"
    ],
    "skipped": [
      "Intel test construction and local targets",
      "Intel conformance (${{ matrix.lane }})",
      "build and non-Lean tests (x86_64-apple-darwin)"
    ]
  },
  "repro": {
    "workflow": ".github/workflows/reproducibility.yml",
    "events": [
      "pull_request",
      "workflow_dispatch"
    ],
    "success": [
      "repro"
    ],
    "skipped": []
  }
};
const require = (condition, reason) => { if (!condition) throw Error(reason); };
const equalNames = (actual, expected) => JSON.stringify([...actual].sort()) === JSON.stringify([...expected].sort());
function check(mode, r) {
  const s = specs[mode];
  require(s && r && typeof r === "object", "mode/receipt");
  require(r.head === HEAD && r.repository === "UOR-Foundation/lexlean", "head/repository");
  require(Number.isSafeInteger(r.id) && r.id > 0 && r.attempt === 1, "run identity/attempt");
  require(r.workflow === s.workflow && s.events.includes(r.event), "workflow/event");
  require(r.status === "completed" && r.conclusion === "success", "terminal success");
  require(Array.isArray(r.jobs) && r.total_count === r.jobs.length &&
    r.pages_complete === true, "complete paginated receipt");
  const ids = new Set();
  for (const j of r.jobs) {
    require(Number.isSafeInteger(j.id) && j.id > 0 && !ids.has(j.id), "duplicate/invalid job ID");
    ids.add(j.id);
    require(j.run_id === r.id && j.attempt === 1, "job run/attempt");
    require(j.status === "completed" && ["success","skipped"].includes(j.conclusion), "job terminal verdict");
  }
  const success = r.jobs.filter(j=>j.conclusion==="success").map(j=>j.name);
  const skipped = r.jobs.filter(j=>j.conclusion==="skipped").map(j=>j.name);
  require(equalNames(success,s.success), "exact success roster");
  require(equalNames(skipped,s.skipped), "exact skipped roster");
  return {mode, head:r.head, run:r.id, attempt:r.attempt, success:success.length, skipped:skipped.length,
    scope:"structural receipt match only; provider authenticity and authorization external"};
}
function selftest() {
  const fixture = mode => {
    const s = specs[mode];
    return {head:HEAD,repository:"UOR-Foundation/lexlean",id:1,attempt:1,workflow:s.workflow,event:s.events[0],
      status:"completed",conclusion:"success",pages_complete:true,total_count:s.success.length+s.skipped.length,
      jobs:[...s.success.map(name=>({name,conclusion:"success"})),...s.skipped.map(name=>({name,conclusion:"skipped"}))]
        .map((j,i)=>({...j,id:i+10,run_id:1,attempt:1,status:"completed"}))};
  };
  const good=Object.keys(specs).map(m=>check(m,fixture(m)));
  const plants=[
    ["wrong head",r=>r.head="bad"],["foreign repository",r=>r.repository="other/repo"],
    ["wrong attempt",r=>r.attempt=2],["foreign job",r=>r.jobs[0].run_id=2],
    ["wrong job attempt",r=>r.jobs[0].attempt=2],["duplicate job ID",r=>r.jobs[1].id=r.jobs[0].id],
    ["same-count name substitution",r=>r.jobs[1].name=r.jobs[0].name],
    ["missing successful row",r=>{r.jobs.splice(0,1);r.total_count--;}],
    ["unexpected skip",r=>r.jobs[0].conclusion="skipped"],
    ["unplanned Intel success",r=>r.jobs.at(-1).conclusion="success"],
    ["bad placeholder",r=>r.jobs.at(-1).name="not expected"],
    ["incomplete page",r=>r.pages_complete=false],["bad total",r=>r.total_count++],
    ["nonterminal",r=>r.jobs[0].status="in_progress"],["red run",r=>r.conclusion="failure"],
    ["wrong workflow",r=>r.workflow=".github/workflows/other.yml"],["wrong event",r=>r.event="push"]
  ];
  const rejected=plants.map(([name,mutate])=>{const r=fixture("ready");mutate(r);try{check("ready",r);}catch(e){return {name,refused:e.message};}throw Error("unexpected acceptance: "+name);});
  return {good,rejected};
}

// Raw provider adapter: fetch run and all attempt-1 job pages in order at
// per_page=50. This is structural validation, not provider authentication.
function normalizeRaw(raw) {
  const r=raw?.run, pages=raw?.pages;
  require(r && Array.isArray(pages) && pages.length>0,"raw receipt shape");
  require(r.repository?.full_name==="UOR-Foundation/lexlean" &&
    r.head_repository?.full_name===r.repository.full_name,"raw repository/head repository");
  require(r.head_sha===HEAD && r.run_attempt===1,"raw head/attempt");
  const total=pages[0].total_count;
  require(Number.isSafeInteger(total)&&total>0&&pages.length===Math.ceil(total/50),"raw page count");
  for(const [i,p] of pages.entries()) {
    require(p.total_count===total&&Array.isArray(p.jobs)&&p.jobs.length===Math.min(50,total-50*i),"raw page length/total");
  }
  const jobs=pages.flatMap(p=>p.jobs);
  require(jobs.every(j=>j.head_sha===HEAD&&j.run_id===r.id&&j.run_attempt===r.run_attempt),"raw job head/run/attempt");
  require(new Set(jobs.map(j=>j.id)).size===jobs.length,"raw duplicate job");
  return {head:r.head_sha,repository:r.repository.full_name,id:r.id,attempt:r.run_attempt,
    workflow:r.path,event:r.event,status:r.status,conclusion:r.conclusion,
    pages_complete:true,total_count:total,jobs:jobs.map(j=>({...j,attempt:j.run_attempt}))};
}
function rawSelftest() {
  const s=specs.ready, clone=x=>JSON.parse(JSON.stringify(x));
  const jobs=[...s.success.map(name=>({name,conclusion:"success"})),...s.skipped.map(name=>({name,conclusion:"skipped"}))]
    .map((j,i)=>({...j,id:i+10,run_id:1,run_attempt:1,head_sha:HEAD,status:"completed"}));
  const fixture=()=>({run:{head_sha:HEAD,repository:{full_name:"UOR-Foundation/lexlean"},head_repository:{full_name:"UOR-Foundation/lexlean"},id:1,run_attempt:1,path:s.workflow,event:"pull_request",status:"completed",conclusion:"success"},pages:[{total_count:jobs.length,jobs:clone(jobs.slice(0,50))},{total_count:jobs.length,jobs:clone(jobs.slice(50))}]});
  const good=check("ready",normalizeRaw(fixture()));
  const plants=[["missing page",r=>r.pages.pop()],["duplicate page",r=>r.pages[1]=clone(r.pages[0])],["short first page",r=>r.pages[0].jobs.pop()],["changed total",r=>r.pages[1].total_count++],["foreign job head",r=>r.pages[1].jobs[0].head_sha="foreign"],["foreign head repository",r=>r.run.head_repository.full_name="other/repo"],["duplicate job across pages",r=>r.pages[1].jobs[0].id=r.pages[0].jobs[0].id]];
  const rejected=plants.map(([name,mutate])=>{const r=fixture();mutate(r);try{check("ready",normalizeRaw(r));}catch(e){return {name,refused:e.message};}throw Error("unexpected acceptance "+name);});
  return {good,rejected};
}
import {readFileSync} from "node:fs";
const [mode,path]=process.argv.slice(2);
try {
  const input=mode==="--self-test"?null:JSON.parse(readFileSync(path,"utf8"));
  const result=mode==="--self-test"?{normalized:selftest(),raw:rawSelftest()}:check(mode,input.run&&input.pages?normalizeRaw(input):input);
  console.log(JSON.stringify(result,null,2));
} catch(e) { console.error("HOLD: "+e.message); process.exitCode=1; }
