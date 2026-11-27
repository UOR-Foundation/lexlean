// #603 DOD-10 advisory offline replay; no network or merge authority.
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
const anchors = [
  {
    "name": "ci-before-pr50-runs.json",
    "blob": "6978133b5e434f922ec1cd807e9a741165af3ab7"
  },
  {
    "name": "ci-before-pr50-job-costs-corrected.json",
    "blob": "eac107477745061bc74cb307746dc18b6548f3a7"
  },
  {
    "name": "ci-b2-composite-607-609-20261126.json",
    "blob": "b5bf3aa36e40296ea6a1dd44ca4a17d33e620919"
  },
  {
    "name": "ci-b2-composite-607-609-postmerge-20261126.json",
    "blob": "a0058c1f417a5d98a9e857ec03aab71c94628cfa"
  },
  {
    "name": "ci-dod6-terminal-621-623-20261126.json",
    "blob": "6c689141e4cc523baffa1c51a6c7dd4e51d8bfd4"
  }
];
function replay(docs) {
  const [inv,cost,...receipts]=docs;
  const must=(ok,msg)=>{if(!ok)throw Error(msg)};
  const round=x=>Math.round(x*1000)/1000;
  const ms=t=>{const n=Date.parse(t);must(Number.isFinite(n),"invalid timestamp");return n};
  const quantile=(v,p)=>{must(v.length>0,"empty sample");const i=(v.length-1)*p,k=Math.floor(i);return v[k]+(v[Math.min(k+1,v.length-1)]-v[k])*(i-k)};
  must(inv.pr_count===50&&inv.rows.length===50&&cost.expected_runs===128&&cost.run_costs.length===128,"baseline counts");
  const ids=new Map(),prIds=new Set(),totals=new Map(),platforms={},seen=new Set(),durations=[];
  for(const p of inv.rows){
    must(!prIds.has(p.number)&&p.run_list_complete===true&&p.runs.length===p.run_total_reported,"PR inventory");
    prIds.add(p.number);totals.set(p.number,0);
    for(const w of p.runs){must(w.head_sha===p.head_sha&&!ids.has(w.id),"baseline head or duplicate run");ids.set(w.id,p.number)}
    const relevant=p.runs.filter(w=>w.event==="pull_request"&&["ci","reproducibility"].includes(w.name)&&w.conclusion!=="skipped");
    const passed=relevant.filter(w=>w.status==="completed"&&w.conclusion==="success"&&ms(w.updated_at)<=ms(p.merged_at));
    const ci=passed.filter(w=>w.name==="ci"),re=passed.filter(w=>w.name==="reproducibility");
    if(ci.length&&re.length){const start=Math.min(...relevant.map(w=>ms(w.created_at)));const end=Math.max(Math.min(...ci.map(w=>ms(w.updated_at))),Math.min(...re.map(w=>ms(w.updated_at))));must(end>=start,"negative green interval");durations.push((end-start)/60000)}
  }
  must(ids.size===128,"baseline distinct run count");
  for(const c of cost.run_costs){
    must(ids.get(c.id)===c.pr&&!seen.has(c.id)&&c.complete===true&&c.job_count===c.returned&&Number.isFinite(c.job_minutes)&&c.job_minutes>=0,"baseline cost inventory");
    seen.add(c.id);totals.set(c.pr,totals.get(c.pr)+c.job_minutes);
    for(const [k,v] of Object.entries(c.by_platform_minutes)){must(Number.isFinite(v)&&v>=0,"invalid platform minutes");platforms[k]=(platforms[k]??0)+v}
  }
  const v=[...totals.values()].sort((a,b)=>a-b);durations.sort((a,b)=>a-b);
  const baseline={prs:v.length,runs:seen.size,runner_minutes:round(v.reduce((a,b)=>a+b,0)),median_runner_minutes:round(quantile(v,.5)),p90_runner_minutes:round(quantile(v,.9)),platform_minutes:Object.fromEntries(Object.entries(platforms).map(([k,n])=>[k,round(n)])),premerge_green:{count:durations.length,median_minutes:round(quantile(durations,.5)),p90_minutes:round(quantile(durations,.9))}};
  const allRuns=new Set(),allJobs=new Set(),out=[];
  for(const d of receipts)for(const x of d.runs){
    const meta=x.run??x,m=x.metrics,jobs=x.raw,id=meta.id;
    must(!allRuns.has(id)&&meta.status==="completed"&&meta.attempt===1,"duplicate/nonterminal/non-first attempt");
    allRuns.add(id);must(jobs.length===(m.count??m.total_jobs),"receipt roster length");
    let allocated=0,seconds=0,queue=0,counts={},latest=-Infinity;
    for(const j of jobs){
      must(!allJobs.has(j.id)&&(j.attempt===undefined||j.attempt===1)&&(j.status===undefined||j.status==="completed"),"duplicate or foreign attempt job");
      allJobs.add(j.id);counts[j.conclusion]=(counts[j.conclusion]??0)+1;
      latest=Math.max(latest,ms(j.completed));
      if(j.conclusion==="skipped"){must(!j.runner,"allocated skip");continue}
      if(j.runner>0){const elapsed=(ms(j.completed)-ms(j.started))/1000,wait=(ms(j.started)-ms(j.created))/1000;must(elapsed>=0&&wait>=0,"invalid allocated interval");allocated++;seconds+=elapsed;queue+=wait}
    }
    const wall=(latest-ms(meta.created))/1000;
    must(seconds===m.allocated_seconds&&wall===m.wall_seconds,"receipt duration mismatch");
    if(m.allocated!==undefined)must(allocated===m.allocated,"allocation mismatch");
    if(m.success!==undefined)must((counts.success??0)===m.success&&(counts.skipped??0)===m.skipped,"conclusion mismatch");
    if(m.counts!==undefined)must(Object.keys({...counts,...m.counts}).every(k=>counts[k]===m.counts[k]),"conclusion map mismatch");
    if(m.queue_seconds!==undefined||m.summed_queue_seconds!==undefined)must(queue===(m.queue_seconds??m.summed_queue_seconds),"queue mismatch");
    out.push({id,head:meta.head,conclusion:meta.conclusion,allocated,allocated_seconds:seconds,wall_seconds:wall,counts});
  }
  must(allRuns.size===12,"unexpected terminal receipt run count");
  return {scope:"Frozen summary + first-attempt terminal receipts only. No provider authenticity, causal savings, or final AFTER qualification.",baseline,receipts:out};
}
const docs=anchors.map(({name,blob})=>{const bytes=readFileSync(new URL('../'+name,import.meta.url));const hash=createHash('sha1').update('blob '+bytes.length+'\0').update(bytes).digest('hex');if(hash!==blob)throw Error('source blob mismatch: '+name);return JSON.parse(bytes)});
console.log(JSON.stringify(replay(docs),null,2));
