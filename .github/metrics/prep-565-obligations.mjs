// Advisory source port for #565; NOT a complete integration or a gate.
// Usage: node .github/metrics/prep-565-obligations.mjs <new-output-directory> from a Git checkout containing both pins.
// Only generates four prospective files outside the working tree. No source writes.
// Identity migration, §17.22 normative prose, Rust cases/dispatch, pins/locks/oracles,
// independent inventory and runtime qualification remain REQUIRED before adoption.
import { execFileSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import { resolve, dirname, relative, isAbsolute } from "node:path";
import { createHash } from "node:crypto";
const BASE="8d3ab0a55a3d4ac6357403c03760e79c5d22f64a", DONOR="488933f9bb4dac4f34ade1ec0d7ba02936b33593";
const PINS={
  "model/ids.toml": [
    "bc9c77f7847999b9c5a1da63ffe77371b82168f6",
    "beef028830e0abc66baeaa0e3c23e9e0929e5b98"
  ],
  "SPEC.md": [
    "b701f36981200ceb537397fba522d810be470aa2",
    "86cd01c0b45d4b324161310f7f982b823679233c"
  ],
  "features/suites/stdlib.feature": [
    "7c9f0da2e22b75dc4a977dd2285e7eb9d4483caa",
    "3f9beebf9a4b864e06545e6661f3ec6263c3dd75"
  ],
  "features/suites/examples.feature": [
    "111a8b7432f7ad37b03c79f8943fa0a0fd4d5daf",
    "84a7aa524bf6078ebff9bd0e9820f4d3ab5a566a"
  ]
};
function compose(cur,don) {
 const must=(v,m)=>{if(!v)throw Error(m)};
 const unique=(a,m)=>{must(a.length===1,m);return a[0]};
 function blocks(s,re,idOf) {
  const starts=[...s.matchAll(re)];
  return starts.map((x,i)=>({id:idOf(x,s.slice(x.index,starts[i+1]?.index??s.length)),text:s.slice(x.index,starts[i+1]?.index??s.length)}));
 }
 const models=s=>blocks(s,/^\[\[id\]\]\n/gm,(_,b)=>b.match(/^id = "([^"]+)"/m)?.[1]);
 const scenarios=s=>blocks(s,/^  @([A-Z]+-\d+) @build\n/gm,x=>x[1]);
 const tables=s=>[...s.matchAll(/^\| `([A-Z]+-\d+)` \|[^\n]*\n/gm)].map(x=>({id:x[1],text:x[0]}));
 const additions=[["EX-10","EX-09","examples"],["SL-15","SL-14","stdlib"],["SL-17","SL-16","stdlib"]];
 const out={...cur},changes=[];
 for(const [id,after,suite] of additions) {
  const p="model/ids.toml", f="features/suites/"+suite+".feature";
  const row=unique(models(don[p]).filter(x=>x.id===id),"donor register "+id);
  const statement=JSON.parse(unique(row.text.split("\n").filter(x=>x.startsWith("statement = ")),"statement").slice(12));
  const sc=unique(scenarios(don[f]).filter(x=>x.id===id),"donor scenario "+id);
  const tab=unique(tables(don["SPEC.md"]).filter(x=>x.id===id),"donor table "+id);
  must(sc.text.split("\n")[1]==="  Scenario: "+statement,"scenario text drift "+id);
  must(tab.text.startsWith("| `"+id+"` | `"+suite+"` | "+statement+" |"),"table text drift "+id);
  for(const [path,parser,fragment] of [[p,models,row],[f,scenarios,sc],["SPEC.md",tables,tab]]) {
    const found=parser(out[path]);must(!found.some(x=>x.id===id),"duplicate target "+id);
    const anchor=unique(found.filter(x=>x.id===after),"missing/duplicate anchor "+after+" in "+path);
    must(out[path].split(anchor.text).length===2,"nonunique anchor");
    const insertion=anchor.text.endsWith("\n\n")||path==="SPEC.md"?fragment.text:"\n"+fragment.text;
    out[path]=out[path].replace(anchor.text,anchor.text+insertion);
    changes.push({path,id,after,insertion,anchor:anchor.text});
  }
 }
 const before=models(cur["model/ids.toml"]),after=models(out["model/ids.toml"]);
 must(before.length===519&&after.length===522,"register count");
 must(new Set(after.map(x=>x.id)).size===522,"register duplicates");
 for(const [path,parser] of [["model/ids.toml",models],["SPEC.md",tables],["features/suites/stdlib.feature",scenarios],["features/suites/examples.feature",scenarios]]){
   const old=parser(cur[path]),fresh=parser(out[path]);
   for(const x of old)must(fresh.find(y=>y.id===x.id)?.text.trimEnd()===x.text.trimEnd(),"existing row/scenario changed "+x.id);
 }
 const reverse={...out};
 for(const c of [...changes].reverse()){
   must(reverse[c.path].split(c.anchor+c.insertion).length===2,"nonunique reversal");
   reverse[c.path]=reverse[c.path].replace(c.anchor+c.insertion,c.anchor);
 }
 for(const p of Object.keys(cur))must(reverse[p]===cur[p],"existing bytes changed "+p);
 return {out,changes:changes.map(({path,id,after,insertion})=>({path,id,after,characters:insertion.length}))};
}
function selftest(cur,don) {
 const result=compose(cur,don), rejected=[];
 for(const [name,mut] of [["duplicate target",c=>c["model/ids.toml"]=result.out["model/ids.toml"]],["missing anchor",c=>c["SPEC.md"]=c["SPEC.md"].replace("| `EX-09` |","| `EX-99` |")],["scenario mismatch",(c,d)=>d["features/suites/examples.feature"]=d["features/suites/examples.feature"].replace("The `cart` example is the shopping cart","The `cart` example is a shopping cart")]]){const c={...cur},d={...don};mut(c,d);try{compose(c,d)}catch(e){rejected.push({name,reason:String(e)});continue}throw Error("plant passed "+name)}
 return {changes:result.changes,rejected};
}
const git=(...args)=>execFileSync("git",args,{maxBuffer:16*1024*1024});
const blob=bytes=>createHash("sha1").update("blob "+bytes.length+"\0").update(bytes).digest("hex");
const load=(ref,side)=>Object.fromEntries(Object.entries(PINS).map(([p,shas])=>{const b=git("show",ref+":"+p);if(blob(b)!==shas[side])throw Error("source identity "+p);return [p,b.toString("utf8")]}));
try {
 if(process.argv.length!==3)throw Error("new output directory required");
 const dest=resolve(process.argv[2]), root=git("rev-parse","--show-toplevel").toString().trim(), rel=relative(root,dest);
 if(rel===""||(!rel.startsWith(".."+(process.platform==="win32"?"\\":"/"))&&rel!==".."&&!isAbsolute(rel)))throw Error("output must be outside working tree");
 const cur=load(BASE,0),don=load(DONOR,1),report=selftest(cur,don),{out}=compose(cur,don);
 mkdirSync(dest);for(const [p,s] of Object.entries(out)){const f=resolve(dest,p);mkdirSync(dirname(f),{recursive:true});writeFileSync(f,s,{flag:"wx"})}
 writeFileSync(resolve(dest,"receipt.json"),JSON.stringify({base:BASE,donor:DONOR,report,status:"INCOMPLETE_NONQUALIFYING_SOURCE_PORT"},null,2)+"\n",{flag:"wx"});
 console.log(JSON.stringify(report,null,2));
}catch(e){console.error("HOLD: "+e.message);process.exitCode=1}
