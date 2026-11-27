// Historical sensitivity, not CI authority. Run: node .github/metrics/dod9/b1-historical.mjs
import {readFileSync} from 'node:fs';
function model(records){
 const assert=(v,m)=>{if(!v)throw Error(m)}, epoch=t=>Date.parse(t)/1000;let checks=0;const check=(v,m)=>{assert(v,m);checks++};
 const results=[];
 for(const record of records){
  const jobs=record.jobs.filter(j=>j.runner>0);check(jobs.length===record.total,"allocation completeness");
  const build=jobs.filter(j=>j.name==="bootstrap export");check(build.length===1&&build[0].conclusion==="success","export identity");const exportEnd=epoch(build[0].end);
  const lanes=jobs.filter(j=>/^just vv \(bdd-bootstrap-.*\)$/.test(j.name));check(lanes.length===28&&new Set(lanes.map(j=>j.name)).size===28&&lanes.every(j=>j.conclusion==="success"),"lane coverage");
  const other=jobs.filter(j=>!lanes.includes(j));const tail=other.reduce((a,b)=>epoch(a.end)>epoch(b.end)?a:b);const deadline=epoch(tail.end);
  const data=lanes.map(j=>{const start=epoch(j.start),end=epoch(j.end),duration=end-start,overlap=Math.max(0,Math.min(end,exportEnd)-start);assert(duration>0&&overlap<=duration,"interval");return {...j,duration,overlap,afterExport:duration-overlap};});
  const work=data.reduce((a,b)=>a+b.duration,0),overlap=data.reduce((a,b)=>a+b.overlap,0);const horizon=deadline-exportEnd;
  const scenarios=[];
  for(const cap of [1,2,3,4,5,8,10,14,28])for(const removed of [0,0.5,1]){
   // Remaining per-lane durations are sensitivity assumptions, not measured work.
   // LPT schedule assumes cap dedicated slots available from export completion; fixed other observed jobs.
   const slots=Array(cap).fill(0),sorted=data.map(j=>({id:j.id,duration:j.duration-j.overlap*removed})).sort((a,b)=>b.duration-a.duration||a.id-b.id);let scheduled=[];
   for(const j of sorted){const index=slots.indexOf(Math.min(...slots)),start=slots[index];slots[index]+=j.duration;scheduled.push({...j,slot:index,start,end:slots[index]});}
   const end=Math.max(...slots),total=scheduled.reduce((a,b)=>a+b.duration,0);check(scheduled.length===28&&Math.abs(total-(work-overlap*removed))<1e-6,"work/coverage");check(end>=Math.max(total/cap,...scheduled.map(j=>j.duration)),"schedule lower bound");
   scenarios.push({capacity:cap,removedOverlapFraction:removed,modeledRunnerSeconds:total,bootstrapFinishAfterExportSeconds:end,slackToFixedOtherTailSeconds:horizon-end,modeledGateDelaySeconds:Math.max(0,end-horizon)});
  }
  const safe=scenarios.filter(x=>x.removedOverlapFraction===0&&x.modeledGateDelaySeconds===0).map(x=>x.capacity);
  const native=other.filter(j=>/build and non-Lean|Intel conformance|Intel test construction/.test(j.name)).map(j=>({name:j.name,id:j.id,duration:epoch(j.end)-epoch(j.start),end:j.end,slackToTailSeconds:deadline-epoch(j.end),conclusion:j.conclusion})).sort((a,b)=>a.slackToTailSeconds-b.slackToTailSeconds);
  results.push({run:record.run,attempt:record.attempt,head:record.head,jobCount:record.total,conclusions:record.jobs.reduce((a,j)=>(a[j.conclusion]=(a[j.conclusion]||0)+1,a),{}),exportEnd:build[0].end,bootstrapJobs:data.length,observedBootstrapAllocatedSeconds:work,overlapUpperBoundSeconds:overlap,observedLastBootstrapEnd:new Date(Math.max(...data.map(x=>epoch(x.end)))*1000).toISOString(),otherTail:{id:tail.id,name:tail.name,end:tail.end},exportToOtherTailSeconds:horizon,minimumTestedDedicatedCapacityNoOverlapRemoval:safe.length?Math.min(...safe):null,native,scenarios});
 }
 return {schema:"lexlean/advisory-b1-historical-sensitivity/1",authority:false,checks,scope:"Historical first attempts; synthetic LPT schedule, dedicated constant post-export lane slots, other jobs frozen; overlap is interval upper bound, not idle time; no estimate of queue policy, actual B1 performance or qualification.",results};
}
console.log(JSON.stringify(model(JSON.parse(readFileSync(new URL('./b1-historical-input.json',import.meta.url),'utf8'))),null,2));
