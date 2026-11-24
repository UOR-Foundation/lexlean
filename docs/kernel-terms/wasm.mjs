// Judges a project's produced kernel terms with Lean 4.32.1's own C++ kernel compiled to single-threaded
// WebAssembly (foundry v2/in-tab-kernel), under Node, as a browser tab runs it:
//   whole    a fresh instance judges library + suffix, every declaration;
//   suffix   the library resident (admitted by address after the whole check), the suffix judged alone;
//   parts    each theorem's own part judged alone over the resident library, five times: median and worst.
//   node wasm.mjs <dist dir> <work dir>          writes <work>/wasm.json
import fs from "node:fs";
import path from "node:path";
import zlib from "node:zlib";
import { pathToFileURL } from "node:url";

const [dist, work] = process.argv.slice(2);
const factory = (await import(pathToFileURL(path.resolve(dist, "lean-kernel.mjs")).href)).default;
const wasmBinary = fs.readFileSync(path.resolve(dist, "lean-kernel.wasm"));
const read = (name) => fs.readFileSync(path.join(work, name));

async function instance(lines) {
  const m = await factory({
    instantiateWasm: (imports, done) => { WebAssembly.instantiate(wasmBinary, imports).then((r) => done(r.instance, r.module)); return {}; },
    print: (s) => lines.push(s), printErr: (s) => lines.push(s),
  });
  if (m.ccall("checker_ready", "number", [], []) !== 0) throw new Error("the checker did not initialize: " + lines.join(" "));
  return m;
}
function call(m, lines, entry, file, bytes) {
  m.FS.writeFile(file, bytes);
  const t0 = performance.now();
  let exit;
  try { exit = m.ccall(entry, "number", ["string"], [file]); } catch (e) { exit = 3; lines.push("trap: " + (e?.message ?? String(e))); }
  m.FS.unlink(file);
  return { exit, ms: performance.now() - t0 };
}
const axiomsOf = (lines) => Object.fromEntries(lines.filter((l) => l.startsWith("axioms ")).map((l) => {
  const [name, list] = l.slice(7).split(": ");
  return [name, list ? list.split(", ").filter(Boolean) : []];
}));

const result = { kernel: { wasmBytes: wasmBinary.length, wasmGzipBytes: zlib.gzipSync(wasmBinary, { level: 9 }).length } };
{
  const lines = [];
  const m = await instance(lines);
  const whole = Buffer.concat([read("library.ndjson"), read("suffix.ndjson")]);
  const r = call(m, lines, "check_whole", "/whole.ndjson", whole);
  result.whole = { accepted: r.exit === 0, exit: r.exit, ms: Math.round(r.ms), output: lines.join("\n").slice(-400) };
}
const lines = [];
const m = await instance(lines);
const library = read("library.ndjson");
const load = call(m, lines, "load_library", "/library.ndjson", library);
result.library = { bytes: library.length, gzipBytes: zlib.gzipSync(library, { level: 9 }).length, loadMs: Math.round(load.ms), exit: load.exit };
lines.length = 0;
const suffix = read("suffix.ndjson");
const s = call(m, lines, "check_suffix", "/suffix.ndjson", suffix);
result.suffix = { accepted: s.exit === 0, exit: s.exit, ms: Math.round(s.ms), bytes: suffix.length, gzipBytes: zlib.gzipSync(suffix, { level: 9 }).length, axioms: axiomsOf(lines), output: lines[lines.length - 1]?.slice(-400) };
result.parts = [];
const partsDir = path.join(work, "parts");
for (const f of fs.existsSync(partsDir) ? fs.readdirSync(partsDir).filter((f) => f.endsWith(".ndjson")).sort() : []) {
  const bytes = fs.readFileSync(path.join(partsDir, f));
  const theorem = fs.readFileSync(path.join(partsDir, f.replace(/\.ndjson$/, ".name")), "utf8");
  const times = [];
  let r;
  for (let i = 0; i < 5; i++) { lines.length = 0; r = call(m, lines, "check_suffix", "/part.ndjson", bytes); times.push(r.ms); }
  times.sort((a, b) => a - b);
  result.parts.push({ theorem, accepted: r.exit === 0, medianMs: +times[2].toFixed(2), worstMs: +times[4].toFixed(2), bytes: bytes.length, gzipBytes: zlib.gzipSync(bytes, { level: 9 }).length, output: r.exit === 0 ? undefined : lines[lines.length - 1]?.slice(-300) });
}
result.heapMB = Math.round(m.HEAPU8.length / 1048576);
fs.writeFileSync(path.join(work, "wasm.json"), JSON.stringify(result, null, 1));
const accepted = result.parts.filter((p) => p.accepted).length;
console.log(`wasm: whole ${result.whole.accepted ? "accepted" : "refused"} in ${result.whole.ms} ms; suffix ${result.suffix.accepted ? "accepted" : "refused"} in ${result.suffix.ms} ms; parts ${accepted}/${result.parts.length} accepted`);
