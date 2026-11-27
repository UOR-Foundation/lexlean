// DOD-6 read-only receipt recomputation; no CI or release authority.
import { readFileSync } from "node:fs";
function compute(raw){
 const require=(x,m)=>{if(!x)throw Error(m)}; const num=s=>Date.parse(s); const seconds=j=>j.runner_allocated?(num(j.completed)-num(j.started))/1000:0;
 const by=new Map(raw.map(x=>[x.name,x]));require(by.size===7,"missing or duplicate input");
 function assemble(names){
  const parts=names.map(n=>by.get(n));const jobs=parts.flatMap(x=>x.jobs);
  require(parts.every(x=>x.total===parts[0].total&&x.expected_head===parts[0].expected_head),"pagination mismatch");
  require(jobs.length===parts[0].total&&new Set(jobs.map(x=>x.id)).size===jobs.length,"incomplete or duplicate job inventory");
  require(jobs.every(j=>j.head===parts[0].expected_head&&j.status==="completed"&&(!j.runner_allocated||(Number.isFinite(seconds(j))&&seconds(j)>=0))),"wrong head/nonterminal/invalid time");
  const active=jobs.filter(j=>j.runner_allocated);
  return {jobs,count:jobs.length,allocated:active.length,counts:jobs.reduce((o,j)=>(o[j.conclusion]=(o[j.conclusion]||0)+1,o),{}),seconds:active.reduce((n,j)=>n+seconds(j),0),execution_envelope_seconds:(Math.max(...active.map(j=>num(j.completed)))-Math.min(...active.map(j=>num(j.started))))/1000};
 }
 const prior=assemble(["prior_push"]),fixed=assemble(["fixed_push"]),first=assemble(["retry_first"]),second=assemble(["retry_second_1","retry_second_2","retry_second_3"]),repro=assemble(["qualified_repro"]);
 require(first.count===70&&second.count===70&&new Set(first.jobs.map(x=>x.name)).size===70&&new Set(second.jobs.map(x=>x.name)).size===70&&second.jobs.every(x=>x.conclusion==="success"),"retry completeness");
 const index=new Map(first.jobs.map(x=>[x.name,x]));require(second.jobs.every(x=>index.has(x.name)),"job identity drift");
 const retained=second.jobs.filter(x=>{const y=index.get(x.name);return x.conclusion==="success"&&y.conclusion==="success"&&x.started===y.started&&x.completed===y.completed&&x.runner_allocated===y.runner_allocated&&eq(x.labels,y.labels)});
 function eq(x,y){return JSON.stringify(x)===JSON.stringify(y)}
 const fresh=second.jobs.filter(x=>!retained.includes(x));require(retained.length===69&&fresh.length===1&&index.get(fresh[0].name).conclusion==="failure"&&fresh[0].runner_allocated,"unexpected retry closure");
 const kept=retained.reduce((n,x)=>n+seconds(x),0),lost=first.seconds-kept,repeat=fresh.reduce((n,x)=>n+seconds(x),0);
 require(first.seconds===kept+lost&&second.seconds===kept+repeat,"accounting mismatch");
 require(fixed.allocated===1&&fixed.jobs.find(j=>j.runner_allocated)?.name==="plan"&&fixed.counts.success===1&&fixed.counts.skipped===17,"unexpected postmerge shape");
 const compact=({jobs,...rest})=>rest;
 return {prior_postmerge_ci:compact(prior),fixed_postmerge_ci:compact(fixed),observed_postmerge_ci_seconds_difference:prior.seconds-fixed.seconds,observed_postmerge_ci_execution_envelope_seconds_difference:prior.execution_envelope_seconds-fixed.execution_envelope_seconds,retained_jobs:retained.length,retained_seconds:kept,lost_seconds:lost,repeated_seconds:repeat,retry_total_actual_seconds:first.seconds+repeat,qualified_repro:compact(repro),retry_original:compact(first),retry_latest_logical_inventory:compact(second),repeated_job:fresh[0].name,repeated_job_id:fresh[0].id,limits:"Prior push was cancelled after unnecessary allocation, fixed push succeeded; different qualified trees and outcome, not controlled counterfactual or billed minutes. Retried latest job IDs may change: retained identity is name plus exact execution timestamps, labels, conclusion. Does not include tag/repro push or earlier events in push-only comparison."};
}
const data=JSON.parse(readFileSync(".github/metrics/ci-production-recovery-dod6-20261124.json","utf8"));
const result=compute(data.raw);
if(JSON.stringify(result)!==JSON.stringify(data.summary))throw Error("receipt mismatch");
process.stdout.write(JSON.stringify(result,null,2)+"\n");
