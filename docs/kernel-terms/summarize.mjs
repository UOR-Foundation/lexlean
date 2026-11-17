// Gathers the evidence run.sh left in each project's work dir into one record.
//   node summarize.mjs <out.json> <work dir>...
// Per project: what production did with each theorem and why, both kernels' verdicts, whether each produced
// proof rests on exactly its declared axioms, latency of each theorem's own part through the WebAssembly kernel with
// the library resident, sizes, and the planted faults. Totals over all projects follow.
import fs from "node:fs";
import path from "node:path";

const [out, ...works] = process.argv.slice(2);
const PHASE0 = ["reflexivity", "decide", "cases", "induction", "apply", "law"];
const quantile = (xs, q) => (xs.length ? xs[Math.min(xs.length - 1, Math.floor(q * (xs.length - 1) + 0.5))] : null);
const reasonClass = (r) => r.replace(/`[^`]*`/g, (s) => (/^`(term|type|proof form|primitive)/.test(s) ? s : "`…`"));

const projects = {};
const totals = { theorems: 0, produced: 0, phase0Theorems: 0, byForm: {}, refusals: {}, latenciesMs: [], axiomsExact: 0, axiomsDiffer: [], kernelRefused: [] };
for (const work of works) {
  const name = path.basename(work);
  const read = (f) => (fs.existsSync(path.join(work, f)) ? JSON.parse(fs.readFileSync(path.join(work, f), "utf8")) : null);
  const production = read("production.json");
  const wasm = read("wasm.json");
  if (!production || !wasm) { projects[name] = { incomplete: true }; continue; }
  const native = fs.existsSync(path.join(work, "native.txt")) ? fs.readFileSync(path.join(work, "native.txt"), "utf8") : "";
  const byForm = {};
  for (const r of production.rows) {
    const f = (byForm[r.form] ??= { theorems: 0, produced: 0 });
    f.theorems++;
    if (r.produced) f.produced++;
    else if (PHASE0.includes(r.form)) {
      const k = reasonClass(r.refused);
      totals.refusals[k] = (totals.refusals[k] ?? 0) + 1;
    }
  }
  const axioms = wasm.suffix.axioms ?? {};
  let exact = 0;
  for (const r of production.rows.filter((r) => r.produced)) {
    const seen = axioms[r.produced];
    if (seen && JSON.stringify(seen) === JSON.stringify([...r.axioms].sort())) exact++;
    else totals.axiomsDiffer.push({ theorem: r.theorem, declared: r.axioms, produced: seen ?? null });
  }
  const parts = wasm.parts ?? [];
  const latencies = parts.filter((p) => p.accepted).map((p) => p.medianMs).sort((a, b) => a - b);
  for (const p of parts.filter((p) => !p.accepted)) totals.kernelRefused.push({ project: name, theorem: p.theorem, output: p.output });
  const faults = read("faults.json");
  const produced = production.rows.filter((r) => r.produced).length;
  projects[name] = {
    theorems: production.rows.length,
    produced,
    byForm,
    produceMs: Math.round(production.produce_ms * 10) / 10,
    native: {
      whole: /== whole\n(Accepted [^\n]*)/.exec(native)?.[1] ?? native.split("\n").find((l) => l.startsWith("Refused")) ?? null,
      wholeMs: Number(/\nms (\d+)/.exec(native)?.[1] ?? NaN),
      suffix: native.trim().split("\n").pop(),
    },
    wasm: {
      whole: wasm.whole.accepted, wholeMs: wasm.whole.ms,
      suffix: wasm.suffix.accepted, suffixMs: wasm.suffix.ms,
      libraryLoadMs: wasm.library.loadMs, heapMB: wasm.heapMB,
      partsAccepted: parts.filter((p) => p.accepted).length, parts: parts.length,
      medianPartMs: quantile(latencies, 0.5), p90PartMs: quantile(latencies, 0.9), worstPartMs: latencies.at(-1) ?? null,
    },
    bytes: {
      library: wasm.library.bytes, libraryGzip: wasm.library.gzipBytes,
      suffix: wasm.suffix.bytes, suffixGzip: wasm.suffix.gzipBytes,
    },
    axiomsExact: exact,
    faults,
  };
  totals.theorems += production.rows.length;
  totals.produced += produced;
  totals.phase0Theorems += production.rows.filter((r) => PHASE0.includes(r.form)).length;
  for (const [f, v] of Object.entries(byForm)) {
    const t = (totals.byForm[f] ??= { theorems: 0, produced: 0 });
    t.theorems += v.theorems; t.produced += v.produced;
  }
  totals.latenciesMs.push(...latencies);
  totals.axiomsExact += exact;
}
const all = totals.latenciesMs.sort((a, b) => a - b);
totals.latency = { parts: all.length, medianMs: quantile(all, 0.5), p90Ms: quantile(all, 0.9), worstMs: all.at(-1) ?? null };
delete totals.latenciesMs;
totals.refusals = Object.fromEntries(Object.entries(totals.refusals).sort((a, b) => b[1] - a[1]));
const kernel = JSON.parse(fs.readFileSync(path.join(works[0], "wasm.json"), "utf8")).kernel;
fs.writeFileSync(out, JSON.stringify({ spec: "lexlean/kernel-terms-evidence/1", kernel, totals, projects }, null, 1) + "\n");
console.log(`${totals.produced} of ${totals.theorems} theorems produced; latency median ${totals.latency.medianMs} ms, worst ${totals.latency.worstMs} ms`);
