// Planted faults through the browser path (SPEC.md §18.10): each fault is planted in the IR production reads (the
// work dir's snapshot.json), the producer writes its suffix over the library export, and Lean's kernel compiled to
// WebAssembly judges that suffix with the library resident. Every fault must end in a refusal, by the kernel or, for
// an omitted case, by the producer.
//   node faults.mjs <dist dir> <work dir> <xtask binary>      writes <work>/faults.json
//
//   altered-statement  a produced reflexivity theorem takes another's statement: its proof still checks, its binding to
//                      the theorem Lean elaborated must not;
//   swapped-branch     a produced cases proof whose branches differ has two constructor labels swapped;
//   false-decide       a produced decide theorem's literal right side is increased by one;
//   omitted-case       a produced cases proof loses its last branch.
import fs from "node:fs";
import path from "node:path";
import { execFileSync } from "node:child_process";
import { pathToFileURL } from "node:url";

const [dist, work, xtask] = process.argv.slice(2);
const factory = (await import(pathToFileURL(path.resolve(dist, "lean-kernel.mjs")).href)).default;
const wasmBinary = fs.readFileSync(path.resolve(dist, "lean-kernel.wasm"));
const lines = [];
const m = await factory({
  instantiateWasm: (imports, done) => { WebAssembly.instantiate(wasmBinary, imports).then((r) => done(r.instance, r.module)); return {}; },
  print: (s) => lines.push(s), printErr: (s) => lines.push(s),
});
if (m.ccall("checker_ready", "number", [], []) !== 0) throw new Error("the checker did not initialize");
const library = fs.readFileSync(path.join(work, "library.ndjson"));
m.FS.writeFile("/library.ndjson", library);
if (m.ccall("load_library", "number", ["string"], ["/library.ndjson"]) !== 0) throw new Error("the library did not load: " + lines.join(" "));

const snapshot = JSON.parse(fs.readFileSync(path.join(work, "snapshot.json"), "utf8"));
const production = JSON.parse(fs.readFileSync(path.join(work, "production.json"), "utf8"));
const produced = new Set(production.rows.filter((r) => r.produced).map((r) => r.theorem));
const theorems = snapshot.modules.flatMap((mod) => (mod.semantic?.declarations ?? [])
  .filter((d) => d.kind === "theorem" && produced.has(`${mod.lean_module}.${d.name}`))
  .map((d) => ({ lean: `${mod.lean_module}.${d.name}`, decl: d })));
const natRight = (d) => d.statement.kind === "eq" && d.statement.right.kind === "nat";
const differingCases = (d) => d.proof.kind === "cases" && d.proof.branches.length >= 2
  && new Set(d.proof.branches.map((b) => JSON.stringify(b.proof))).size > 1;

// Each plant names its theorem and edits that theorem's declaration in a fresh copy of the snapshot.
const plants = [];
const refl = theorems.filter((t) => t.decl.proof.kind === "reflexivity" && t.decl.parameters.length === 0);
if (refl.length >= 2) {
  const other = structuredClone(refl[1].decl.statement);
  plants.push({ fault: "altered-statement", theorem: refl[0].lean, expect: "binding", edit: (d) => { d.statement = other; } });
}
const cases = theorems.find((t) => differingCases(t.decl));
if (cases) plants.push({ fault: "swapped-branch", theorem: cases.lean, expect: "proof", edit: (d) => {
  const [a, b] = d.proof.branches;
  [a.constructor, b.constructor] = [b.constructor, a.constructor];
} });
const decide = theorems.find((t) => t.decl.proof.kind === "decide" && natRight(t.decl));
if (decide) plants.push({ fault: "false-decide", theorem: decide.lean, expect: "proof", edit: (d) => {
  d.statement.right.value = String(BigInt(d.statement.right.value) + 1n);
} });
const anyCases = theorems.find((t) => t.decl.proof.kind === "cases" && t.decl.proof.branches.length >= 2);
if (anyCases) plants.push({ fault: "omitted-case", theorem: anyCases.lean, expect: "producer", edit: (d) => { d.proof.branches.pop(); } });

const text = fs.readFileSync(path.join(work, "snapshot.json"), "utf8");
const declarationOf = (copy, lean) => copy.modules.flatMap((mod) => (mod.semantic?.declarations ?? [])
  .filter((d) => d.kind === "theorem" && `${mod.lean_module}.${d.name}` === lean))[0];
const result = [];
for (const plant of plants) {
  const copy = JSON.parse(text);
  const before = JSON.stringify(declarationOf(copy, plant.theorem));
  plant.edit(declarationOf(copy, plant.theorem));
  if (JSON.stringify(declarationOf(copy, plant.theorem)) === before) throw new Error(`the ${plant.fault} plant changed nothing`);
  const dir = fs.mkdtempSync(path.join(work, "fault-"));
  fs.writeFileSync(path.join(dir, "snapshot.json"), JSON.stringify(copy));
  execFileSync(xtask, ["kernel-terms", "produce", dir, path.join(work, "library.ndjson")], { stdio: "pipe" });
  const planted = JSON.parse(fs.readFileSync(path.join(dir, "production.json"), "utf8"));
  const row = planted.rows.find((r) => r.theorem === plant.theorem);
  let verdict;
  if (!row.produced) verdict = { refusedBy: "producer", reason: row.refused };
  else {
    lines.length = 0;
    m.FS.writeFile("/suffix.ndjson", fs.readFileSync(path.join(dir, "suffix.ndjson")));
    const exit = m.ccall("check_suffix", "number", ["string"], ["/suffix.ndjson"]);
    const said = lines.join("\n");
    const target = plant.expect === "binding" ? `LexLeanKernel.bound.${plant.theorem}` : `LexLeanKernel.${plant.theorem}`;
    verdict = exit === 0 ? { refusedBy: null, accepted: true } : { refusedBy: "kernel", names: said.includes(target) ? target : said.slice(-200), reason: said.split("\n").pop().slice(0, 300) };
  }
  result.push({ fault: plant.fault, theorem: plant.theorem, ...verdict });
  console.log(`fault ${plant.fault} on ${plant.theorem}: ${verdict.refusedBy ? "refused by " + verdict.refusedBy : "ACCEPTED"}`);
  fs.rmSync(dir, { recursive: true });
}
fs.writeFileSync(path.join(work, "faults.json"), JSON.stringify(result, null, 1));
