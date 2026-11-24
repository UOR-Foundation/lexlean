// Advisory DOD-6 B1 accounting; run from checkout root with Node.
import {readFileSync} from "node:fs";
function measure(sources,prior,specs){
 const must=(ok,msg)=>{if(!ok)throw Error(msg)};must(sources.length===4&&new Set(sources.map(x=>x.id)).size===4&&new Set(sources.map(x=>x.run)).size===4,"duplicate or missing receipt");
 const output=[];
 for(const s of specs){let ci=s.name==="prior"?prior.prior_postmerge_ci:prior.fixed_postmerge_ci;let jobs=sources.filter(x=>x.cohort===s.name);must(jobs.length===2&&new Set(jobs.map(x=>x.role)).size===2,"coverage");let extra={};
 for(const x of jobs){must(["repro","tag"].includes(x.role)&&x.run===s[x.role]&&x.head===s.head&&x.status==="completed"&&x.conclusion==="success"&&x.allocated===true,"identity or status");let seconds=(Date.parse(x.completed)-Date.parse(x.started))/1000;must(Number.isFinite(seconds)&&seconds>=0,"duration");extra[x.role]=seconds}
 output.push({cohort:s.name,head:s.head,ci_run:s.ci,ci_seconds:ci.seconds,ci_allocated:ci.allocated,repro_seconds:extra.repro,tag_seconds:extra.tag,all_three_workflow_job_seconds:ci.seconds+extra.repro+extra.tag,allocated_jobs:ci.allocated+2,ci_outcome:s.name==="prior"?"cancelled":"success"});}
 return {rows:output,observed_difference_seconds:output[0].all_three_workflow_job_seconds-output[1].all_three_workflow_job_seconds,scope:"Three observed push workflows only; prior CI cancelled, distinct source trees, unweighted job seconds, not billed usage or causal savings. Independent repro is preserved; no production tag published."};
}
const read=p=>JSON.parse(readFileSync(p,"utf8"));
const old=read(".github/metrics/ci-production-recovery-dod6-20261124.json");
const data=read(".github/metrics/ci-complete-push-events-dod6-20261124.json");
const result=measure(data.raw,old.summary,data.source_specs);
if(JSON.stringify(result)!==JSON.stringify(data.summary))throw Error("receipt mismatch");
process.stdout.write(JSON.stringify(result,null,2)+"\n");
