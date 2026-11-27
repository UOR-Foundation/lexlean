// Advisory historical receipt replay. No acceptance authority.
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
const anchors=[
  [
    "ci-retry-execution-receipts-20261123.json",
    "84ef2b7ac57d7b570678ddf165c1ed27088b072d"
  ],
  [
    "ci-v05-619-terminal-attempt-lineage-20261124.json",
    "f8008443d4cf713fb188b45714981d06428883dd"
  ],
  [
    "ci-production-recovery-dod6-20261124.json",
    "7ff71f3213f083010ebb9daf55994ef595c4dfca"
  ]
];
function replay(docs){
 const [a,b,c]=docs;const need=(v,m)=>{if(!v)throw Error(m)},sec=t=>{const n=Date.parse(t)/1000;need(Number.isFinite(n),"bad timestamp");return n},round=x=>Math.round(x*1000)/1000;
 const ids=new Set();
 function index(rows,n,mode,head,run,attempt){
  need(rows.length===n,"incomplete roster");const map=new Map();
  for(const j of rows){
   need(Number.isSafeInteger(j.id)&&j.id>0&&!ids.has(j.id)&&typeof j.name==="string"&&!map.has(j.name),"duplicate/invalid identity");ids.add(j.id);
   need(j.status==="completed"&&["success","failure","cancelled"].includes(j.conclusion)&&Array.isArray(j.labels)&&j.labels.length,"invalid terminal receipt");
   if(mode==="legacy")need(j.run_attempt===attempt,"wrong legacy attempt");
   if(mode==="full")need(j.head_sha===head&&j.run_id===run&&j.run_attempt===attempt&&j.runner_id>0,"foreign attempt or missing runner");
   if(mode==="compact")need(j.head===head&&j.runner_allocated===true,"foreign compact job");
   const start=j.started_at??j.started,end=j.completed_at??j.completed,seconds=sec(end)-sec(start);
   need(seconds>=0,"negative interval");
   map.set(j.name,{seconds,start,end,conclusion:j.conclusion,key:JSON.stringify([start,end,j.conclusion,[...j.labels].sort()])});
  }return map;
 }
 const sum=m=>[...m.values()].reduce((s,j)=>s+j.seconds,0);
 function retry(f,l,id){
  need(f.size===l.size&&[...f.keys()].every(k=>l.has(k)),"changed roster");let retained=0,retainedSeconds=0,lostSeconds=0,repeatedSeconds=0;const changed=[];
  for(const [k,x] of f){const y=l.get(k);need(y.conclusion==="success","latest not green");if(x.key===y.key){retained++;retainedSeconds+=x.seconds}else{need(sec(y.start)>=sec(x.end),"overlapping interval");changed.push(k);lostSeconds+=x.seconds;repeatedSeconds+=y.seconds}}
  need(changed.length===1,"unexpected recovery scope");return {id,displayRows:f.size+l.size,distinctExecutions:f.size+l.size-retained,retained,retainedSeconds,lostSeconds,repeatedSeconds,actualSeconds:sum(f)+repeatedSeconds,phantomSeconds:retainedSeconds,changed};
 }
 need(a.run_id===45170496660&&a.run_attempt===2&&a.run_status==="completed"&&a.run_conclusion==="success"&&a.jobs.length===156,"legacy metadata");
 const old=retry(index(a.jobs.filter(x=>x.run_attempt===1),78,"legacy",null,null,1),index(a.jobs.filter(x=>x.run_attempt===2),78,"legacy",null,null,2),a.run_id);
 need(round(old.actualSeconds/60)===a.measured.actual_all_attempt_execution_minutes&&round(old.phantomSeconds/60)===a.measured.inflation_minutes,"legacy summary");
 need(b.observed_run.id===45316289312&&b.observed_run.run_attempt===2&&b.observed_run.status==="completed"&&b.observed_run.conclusion==="success"&&b.attempts.length===2,"#619 metadata");
 const full=b.attempts.map((v,i)=>{need(v.total_count===70&&v.run.id===b.observed_run.id&&v.run.head_sha===b.observed_run.head_sha&&v.run.run_attempt===i+1&&v.run.status==="completed","attempt metadata");return index(v.jobs,70,"full",v.run.head_sha,v.run.id,i+1)});
 const cut=retry(full[0],full[1],b.observed_run.id);need(cut.retained===b.lineage.retained_successes&&round(cut.actualSeconds/60)===b.lineage.completed_execution_minutes,"#619 summary");
 need(c.raw.length===7&&new Set(c.raw.map(x=>x.name)).size===7,"missing/duplicate page");
 const get=(name,h,n)=>{const v=c.raw.find(x=>x.name===name);need(v&&v.expected_head===h&&v.total===n,"wrong page metadata");return v};
 const h="603336bf2752ecbfa4d688ccb5d52b3585bfa64b";
 const second=[1,2,3].flatMap(i=>{const x=get("retry_second_"+i,h,70);need(x.jobs.length===(i===3?10:30)&&x.endpoint.endsWith("page="+i),"short/wrong page");return x.jobs});
 const fix=retry(index(get("retry_first",h,70).jobs,70,"compact",h),index(second,70,"compact",h),45437716141);need(fix.retainedSeconds===c.summary.retained_seconds&&fix.lostSeconds===c.summary.lost_seconds&&fix.repeatedSeconds===c.summary.repeated_seconds&&fix.actualSeconds===c.summary.retry_total_actual_seconds,"production summary");
 return {scope:"Historical retry replay only. Legacy receipt lacks runner IDs and assumes complete non-skipped terminal rows allocated. Non-retry push and repro pages are outside calculation. No billing, authenticity, acceptance or additive cohort claim.",runs:[old,cut,fix]};
}
const docs=anchors.map(([name,blob])=>{const bytes=readFileSync(new URL('../'+name,import.meta.url));const hash=createHash('sha1').update('blob '+bytes.length+'\0').update(bytes).digest('hex');if(hash!==blob)throw Error('blob mismatch '+name);return JSON.parse(bytes)});
const tests=[["missing page",x=>x[2].raw.pop()],
["short page",x=>x[2].raw.find(y=>y.name==="retry_second_2").jobs.pop()],
["duplicate job",x=>x[1].attempts[1].jobs[0].id=x[1].attempts[0].jobs[0].id],
["foreign head",x=>x[1].attempts[1].jobs[0].head_sha="a".repeat(40)],
["latest failure",x=>x[0].jobs.find(j=>j.run_attempt===2).conclusion="failure"],
["negative interval",x=>x[2].raw.find(y=>y.name==="retry_first").jobs[0].completed="2000-01-01T00:00:00Z"],
["changed roster",x=>x[1].attempts[1].jobs[0].name="foreign"],
["false total",x=>x[2].summary.retry_total_actual_seconds++]];
const result=replay(docs);
result.negativeControls=tests.map(([name,mut])=>{const x=JSON.parse(JSON.stringify(docs));mut(x);try{replay(x)}catch(e){return {name,refused:e.message}}throw Error('unexpected acceptance '+name)});
console.log(JSON.stringify(result,null,2));
