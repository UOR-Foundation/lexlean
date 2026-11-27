// DOD-8a replay of historical review facts; advisory only.
import { readFileSync } from "node:fs";
function screen(model,review,dependencies){
 const assert=(v,m)=>{if(!v)throw Error(m)};
 const map=new Map(review.map(x=>[x.pr,x]));assert(map.size===review.length&&map.size===model.rows.length&&model.rows.every(x=>map.has(x.pr)&&map.get(x.pr).release===x.release),"review coverage");
 const scenarios=model.scenarios.map(s=>{
 const rejected=s.batches.filter(b=>b.prs.length>1).map(b=>{
 let pairs=[];for(let i=0;i<b.prs.length;i++)for(let j=i+1;j<b.prs.length;j++){let a=map.get(b.prs[i]),c=map.get(b.prs[j]);let overlap=a.paths&&c.paths?a.paths.filter(x=>c.paths.includes(x)):null;pairs.push({a:a.pr,b:c.pr,overlap,known_order:dependencies.filter(d=>d.before===a.pr&&d.after===c.pr||d.before===c.pr&&d.after===a.pr)});}
 const blockers=b.prs.map(n=>({pr:n,class:map.get(n).classification,reason:map.get(n).reason}));
 return {prs:b.prs,blockers,pairs,disposition:"HOLD_NO_ADMISSION_EVIDENCE"};
 });
 return {size:s.size,age_hours:s.age_hours,naive_gate_count:s.gate_invocations,naive_multi_batches:s.multi_batches,approved_multi_batches:0,independent_gate_count_if_all_holds: model.rows.length,unapproved_batches:rejected};
 });
 assert(scenarios.every(s=>s.unapproved_batches.length===s.naive_multi_batches),"multi-batch coverage");
 return {schema:"lexlean/ci-batching-admission-review/1",scope:"Retrospective review only; unknown is not cheap, urgent or safe to batch by default. Existing whole-gate route is retained unless admitted; no live authority.",source_head:"e0e303abf2c37225e2e32d82a78d0f8dc7ed6d8c",selected_cost_minutes:model.independent_observed_selected_gate_and_repro_job_minutes,classification_counts:review.reduce((o,x)=>(o[x.classification]=(o[x.classification]||0)+1,o),{}),complete_listed_path_inventories:review.filter(x=>x.paths!==null).length,review,dependencies,scenarios,limits:["PR metadata/body and listed changed paths are historical observations, not a dependency closure. Partial/zero/omitted path listings deliberately remain unknown.","Known base ancestry is not in itself a blocker to ordered batching; some dependent PRs can share a final gate after review. Shared files do not prove conflict; disjoint files do not prove independence.","No member has explicit historical nonurgent staging approval or exact composite-tree receipt. The all-hold scenario is a conservative policy boundary, not a cost forecast or a conclusion that batching cannot work."]};
}
const read = p => JSON.parse(readFileSync(p, "utf8"));
const model = read(".github/metrics/ci-batching-sensitivity-dod8-20261124.json");
const receipt = read(".github/metrics/ci-batching-admission-review-dod8-20261124.json");
const actual = screen(model, receipt.review, receipt.dependencies);
if (JSON.stringify(actual) !== JSON.stringify(receipt)) throw Error("receipt differs");
process.stdout.write(JSON.stringify(actual, null, 2) + "\n");
