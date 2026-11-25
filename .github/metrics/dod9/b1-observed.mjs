// Advisory measurement only, never acceptance authority.
// node .github/metrics/dod9/b1-observed.mjs --self-test
// node .github/metrics/dod9/b1-observed.mjs candidate.json EXPECTED_HEAD repro.json
import {readFileSync} from 'node:fs';
function measure(run, roster) {
  const need=(v,m)=>{if(!v)throw Error(m)}, time=v=>{const t=Date.parse(v);need(typeof v==='string'&&Number.isFinite(t),'invalid timestamp');return t/1000};
  need(Number.isSafeInteger(run.id)&&run.id>0&&Number.isSafeInteger(run.attempt)&&run.attempt>0&&/^[0-9a-f]{40}$/.test(run.head),'invalid run identity');
  need(Array.isArray(run.raw)&&run.raw.length===run.provider_rows&&new Set(run.raw.map(j=>j.id)).size===run.raw.length,'incomplete or duplicate provider inventory');
  need(run.raw.every(j=>Number.isSafeInteger(j.id)&&j.id>0&&j.attempt===run.attempt&&typeof j.name==='string'&&Number.isSafeInteger(j.runner)&&j.runner>=0),'invalid job identity/attempt');
  need(Array.isArray(roster)&&roster.length===28&&new Set(roster).size===28,'invalid frozen bootstrap roster');
  const names=roster.map(n=>`just vv (${n})`).sort(), legs=run.raw.filter(j=>/^just vv \(bdd-bootstrap-/.test(j.name));
  need(JSON.stringify(legs.map(j=>j.name).sort())===JSON.stringify(names),'bootstrap roster mismatch');
  const allocated=run.raw.filter(j=>j.runner>0&&j.status==='completed'&&j.conclusion!=='skipped');
  const intervals=allocated.map(j=>{const created=time(j.created),start=time(j.started),end=time(j.completed);need(created<=start&&start<=end,'nonmonotonic original-attempt timestamp');return {j,created,start,end}});
  const ex=intervals.filter(v=>v.j.name==='bootstrap export');need(ex.length===1&&ex[0].j.conclusion==='success','export missing or unsuccessful');
  const e=ex[0].end, boot=intervals.filter(v=>legs.includes(v.j)&&v.j.conclusion==='success');
  const sum=(rows,f)=>rows.reduce((a,b)=>a+f(b),0), last=intervals.length?Math.max(...intervals.map(v=>v.end)):null;
  const firstFailed=intervals.filter(v=>v.j.conclusion==='failure').map(v=>v.end).sort((a,b)=>a-b)[0];
  const points=boot.flatMap(v=>v.end>v.start?[[v.start,1],[v.end,-1]]:[]).sort((a,b)=>a[0]-b[0]||a[1]-b[1]);let active=0,peak=0;for(const [,d] of points){active+=d;peak=Math.max(peak,active)}
  return {id:run.id,attempt:run.attempt,head:run.head,complete:run.raw.every(j=>j.status==='completed'),allJobsSuccessful:run.raw.every(j=>j.status==='completed'&&j.conclusion==='success'&&j.runner>0),successfulLegs:boot.length,allLegsAfterExport:boot.length===28&&boot.every(v=>v.start>=e),allocatedSeconds:sum(intervals,v=>v.end-v.start),bootstrapSeconds:sum(boot,v=>v.end-v.start),preExportOverlapSeconds:sum(boot,v=>Math.max(0,Math.min(v.end,e)-v.start)),postExportSeconds:sum(boot,v=>Math.max(0,v.end-Math.max(v.start,e))),queueSeconds:sum(boot,v=>v.start-v.created),postExportStartDelaySeconds:sum(boot,v=>Math.max(0,v.start-e)),peakConcurrentCompletedSuccessfulLegs:peak,gateElapsedSeconds:run.raw.every(j=>j.status==='completed')?last-time(run.run_created):null,firstProviderFailureSeconds:firstFailed===undefined?null:firstFailed-time(run.run_created),lastBootstrapEnd:boot.length?new Date(Math.max(...boot.map(v=>v.end))*1000).toISOString():null,jobNames:run.raw.map(j=>j.name).sort()};
}
function compare(freeze,candidate,expectedHead,repro) {
  const need=(v,m)=>{if(!v)throw Error(m)};
  need(freeze.schema==='lexlean/dod6-b1-baseline-freeze/1','wrong baseline schema');
  const roster=freeze.candidate.roster, baselines=freeze.runs.map(x=>measure(x,roster));
  const reference=baselines.find(x=>x.id===45659553448);need(reference&&reference.allJobsSuccessful,'missing full reference roster');
  need(/^[0-9a-f]{40}$/.test(expectedHead)&&candidate.head===expectedHead,'candidate head mismatch');
  need(!baselines.some(x=>x.id===candidate.id),'baseline cannot be AFTER');
  need(candidate.attempt===1&&candidate.run_latest_attempt===1,'retry requires separate attempt-lineage reconciliation');
  const after=measure(candidate,roster);
  need(candidate.run_status==='completed'&&candidate.run_conclusion==='success'&&after.allJobsSuccessful&&after.allLegsAfterExport,'candidate not a complete successful dependency-scheduled run');
  need(JSON.stringify(after.jobNames)===JSON.stringify(reference.jobNames),'full job roster differs; reconcile approved obligations separately');
  need(repro&&repro.id!==candidate.id&&Number.isSafeInteger(repro.id)&&repro.id>0&&repro.head===expectedHead&&repro.workflow==='.github/workflows/reproducibility.yml'&&repro.status==='completed'&&repro.conclusion==='success','missing same-head independent repro receipt');
  const comparisons=baselines.filter(b=>b.complete&&b.successfulLegs===28).map(b=>({run:b.id,attempt:b.attempt,baselineAllJobsSuccessful:b.allJobsSuccessful,bootstrapDeltaSeconds:after.bootstrapSeconds-b.bootstrapSeconds,allocatedDeltaSeconds:after.allocatedSeconds-b.allocatedSeconds,elapsedDeltaSeconds:after.gateElapsedSeconds-b.gateElapsedSeconds,queueDeltaSeconds:after.queueSeconds-b.queueSeconds}));
  return {schema:'lexlean/advisory-b1-observed-comparison/1',authority:false,scope:'Observed differences only; source/host/queue/attempt confounding remains. Input receipts not independently authenticated; no artifact custody or qualification authority. Provider failure timestamp is not necessarily actionable failure.',after,comparisons};
}
function selfTest(freeze) {
  let checks=0;const eq=(a,b,m)=>{if(a!==b)throw Error(m+': '+a+' != '+b);checks++};const refusal=(fn,m)=>{let refused=false;try{fn()}catch{refused=true}eq(refused,true,m)};
  for(const run of freeze.runs){const m=measure(run,freeze.candidate.roster);for(const [a,b] of [['bootstrapSeconds','allocated_seconds'],['preExportOverlapSeconds','pre_export_job_seconds'],['postExportSeconds','post_export_job_seconds'],['queueSeconds','queue_seconds'],['allocatedSeconds','allocated_completed_job_seconds'],['successfulLegs','successful_legs']])eq(m[a],run.metrics[b],`${run.id} ${a}`)}
  // Synthetic positive control only: shift all bootstrap intervals after export.
  const candidate=JSON.parse(JSON.stringify(freeze.runs.find(x=>x.id===45659553448)));candidate.id=999999;candidate.head='a'.repeat(40);
  const end=Date.parse(candidate.raw.find(j=>j.name==='bootstrap export').completed);
  for(const j of candidate.raw.filter(j=>/^just vv \(bdd-bootstrap-/.test(j.name))){const d=Date.parse(j.completed)-Date.parse(j.started);j.created=j.started=new Date(end).toISOString();j.completed=new Date(end+d).toISOString()}
  const repro={id:999998,head:candidate.head,workflow:'.github/workflows/reproducibility.yml',status:'completed',conclusion:'success'};
  const apply=(mutate,rm=repro)=>{const c=JSON.parse(JSON.stringify(candidate));mutate(c);return compare(freeze,c,candidate.head,rm)};
  eq(apply(()=>{}).authority,false,'advisory only');eq(apply(()=>{}).after.allLegsAfterExport,true,'dependency scheduling');
  refusal(()=>apply(c=>c.head='b'.repeat(40)),'wrong head');
  refusal(()=>apply(c=>c.id=45659553448),'historical reuse');
  refusal(()=>apply(c=>c.run_status='in_progress'),'active run');
  refusal(()=>apply(c=>c.raw.pop()),'partial inventory');
  refusal(()=>apply(c=>c.raw[0].id=c.raw[1].id),'duplicate id');
  refusal(()=>apply(c=>c.raw[0].attempt=2),'wrong attempt');
  refusal(()=>apply(c=>c.run_latest_attempt=2),'retry');
  refusal(()=>apply(c=>c.raw.find(j=>j.name==='honesty').conclusion='failure'),'failed job');
  refusal(()=>apply(c=>c.raw.find(j=>j.name==='honesty').name='foreign'),'changed roster');
  refusal(()=>apply(c=>c.raw.find(j=>/^just vv \(bdd-bootstrap-/.test(j.name)).started=c.run_created),'early start');
  refusal(()=>apply(()=>{},null),'missing repro');
  refusal(()=>apply(()=>{},{...repro,head:'b'.repeat(40)}),'wrong repro head');
  return {checks,status:'PASS',syntheticAfterOnly:true,baselineRuns:freeze.runs.map(run=>measure(run,freeze.candidate.roster))};
}
const read=p=>JSON.parse(readFileSync(p,'utf8'));
const freeze=read(new URL('../ci-b1-baseline-freeze-20261125.json',import.meta.url));
const [candidate,head,repro]=process.argv.slice(2);
if(candidate==='--self-test') console.log(JSON.stringify(selfTest(freeze),null,2));
else {if(!candidate||!head||!repro) throw Error('candidate.json EXPECTED_HEAD repro.json required'); console.log(JSON.stringify(compare(freeze,read(candidate),head,read(repro)),null,2));}
