// DOD-8a retrospective sensitivity. No CI authority; no network or writes.
// Run from repository root: node .github/scripts/ci_batching_sensitivity.mjs
import { readFileSync } from "node:fs";
function model(inv,costs) {
  const assert=(ok,msg)=>{if(!ok)throw Error(msg)};
  const round=n=>Math.round(n*1000)/1000;
  const minute=60000;
  const costById=new Map(costs.run_costs.map(x=>[x.id,x]));
  assert(costById.size===costs.run_costs.length,"duplicate run costs");
  assert(inv.rows.length===inv.pr_count&&new Set(inv.rows.map(x=>x.number)).size===inv.pr_count,"incomplete PR inventory");
  const seen=new Set(), rows=[],excluded=[];
  for(const p of inv.rows){
    if(!/^release\/v\d+\.\d+$/.test(p.base_branch)){excluded.push({pr:p.number,reason:"not_release_line"});continue}
    const valid=x=>{const c=costById.get(x.id);return x.head_sha===p.head_sha&&x.status==="completed"&&x.conclusion==="success"&&c?.complete&&c.pr===p.number&&c.returned===c.job_count&&c.job_minutes>0&&Date.parse(x.created_at)>=Date.parse(p.created_at)&&Date.parse(x.created_at)<=Date.parse(p.merged_at)};
    const ci=p.runs.filter(x=>x.name==="ci"&&valid(x)&&costById.get(x.id).job_count>=60).sort((a,b)=>Date.parse(a.created_at)-Date.parse(b.created_at)||a.id-b.id)[0];
    const repro=ci&&p.runs.filter(x=>x.name==="reproducibility"&&valid(x)).sort((a,b)=>Math.abs(Date.parse(a.created_at)-Date.parse(ci.created_at))-Math.abs(Date.parse(b.created_at)-Date.parse(ci.created_at))||a.id-b.id)[0];
    if(!ci||!repro){excluded.push({pr:p.number,reason:"no_observed_large_success_and_positive_repro_pair_in_PR_window"});continue}
    if(seen.has(ci.id)||seen.has(repro.id)){excluded.push({pr:p.number,reason:"duplicate_run_identity"});continue}
    seen.add(ci.id);seen.add(repro.id);
    rows.push({pr:p.number,release:p.base_branch,arrival_proxy:ci.created_at,ci_run:ci.id,repro_run:repro.id,job_minutes:round(costById.get(ci.id).job_minutes+costById.get(repro.id).job_minutes)});
  }
  const releases=[...new Set(rows.map(x=>x.release))].sort();
  function batches(input,size,hours){
    const result=[];
    for(const release of [...new Set(input.map(x=>x.release))].sort()){
      const line=input.filter(x=>x.release===release).sort((a,b)=>Date.parse(a.arrival_proxy)-Date.parse(b.arrival_proxy)||a.pr-b.pr);
      let current=[],deadline=0;
      function flush(at,reason){
        assert(current.length>0,"empty batch");
        result.push({release,prs:current.map(x=>x.pr),trigger:new Date(at).toISOString(),reason,wait_minutes:current.map(x=>round((at-Date.parse(x.arrival_proxy))/minute)),individual_minutes:round(current.reduce((s,x)=>s+x.job_minutes,0)),proxy_gate_minutes:Math.max(...current.map(x=>x.job_minutes))});
        current=[];
      }
      for(const row of line){
        const at=Date.parse(row.arrival_proxy);
        if(current.length&&at>=deadline)flush(deadline,"age");
        if(!current.length)deadline=at+hours*60*minute;
        current.push(row);
        if(current.length===size)flush(at,"size");
      }
      if(current.length)flush(deadline,"age_at_cohort_boundary");
    }
    assert(result.flatMap(x=>x.prs).length===input.length&&new Set(result.flatMap(x=>x.prs)).size===input.length,"loss or duplicate");
    assert(result.every(x=>x.prs.length<=size&&x.wait_minutes.every(w=>w>=0&&w<=hours*60)),"queue bounds");
    return result;
  }
  const synthetic=[{pr:1,release:"release/a",arrival_proxy:"2026-01-01T00:00:00Z",job_minutes:100},{pr:2,release:"release/a",arrival_proxy:"2026-01-01T01:00:00Z",job_minutes:200},{pr:3,release:"release/a",arrival_proxy:"2026-01-01T08:00:00Z",job_minutes:100},{pr:4,release:"release/b",arrival_proxy:"2026-01-01T00:00:00Z",job_minutes:100}];
  const test=batches(synthetic,2,6);
  assert(test.length===3&&test[0].reason==="size"&&test[0].proxy_gate_minutes===200&&test[0].wait_minutes[0]===60&&test[1].wait_minutes[0]===360&&test[2].prs[0]===4,"synthetic size/age/isolation");
  const independent=round(rows.reduce((n,x)=>n+x.job_minutes,0));
  const preflight=10, scenarios=[];
  for(const size of [2,3,4])for(const hours of [6,12,24]){
    const groups=batches(rows,size,hours),wait=groups.flatMap(x=>x.wait_minutes).sort((a,b)=>a-b),multi=groups.filter(x=>x.prs.length>1);
    const base=groups.reduce((n,x)=>n+x.proxy_gate_minutes,0)+rows.length*preflight;
    const recovery=multi.reduce((n,x)=>n+x.proxy_gate_minutes,0);
    scenarios.push({size,age_hours:hours,gate_invocations:groups.length,multi_batches:multi.length,singletons:groups.length-multi.length,staging_wait_median_minutes:wait.length?wait[Math.ceil(wait.length/2)-1]:0,staging_wait_p95_minutes:wait.length?wait[Math.ceil(wait.length*.95)-1]:0,model_job_minutes_no_recovery:round(base),model_job_minutes_25pct_extra_gate_per_multi_batch:round(base+.25*recovery),model_job_minutes_one_extra_gate_every_multi_batch:round(base+recovery),model_reduction_pct_no_recovery:independent?round(100*(independent-base)/independent):null,batches:groups});
  }
  return {schema:"lexlean/ci-batching-sensitivity/1",scope:"DOD-8a retrospective scheduling sensitivity only; no eligibility or acceptance authority",source_pr_count:inv.pr_count,large_cost_proxy_pr_count:rows.length,releases,independent_observed_selected_gate_and_repro_job_minutes:independent,assumptions:["Final-head successful ci run with >=60 jobs is a COST proxy, not proof of whole-gate qualification; require complete positive cost and a successful positive-cost independent repro in PR lifetime.","Arrival is earliest selected successful CI creation, not actual ready-for-review; retrospective lookahead and selected successful sample bias.","All selected same-line PRs hypothetically compatible; actual source dependencies, urgency, identity changes and conflicts are unknown; do not use as admission decision.","Composite gate cost assumed max selected member gate+repro minutes; actual composite costs may be larger or smaller.","10 job-minutes per member preflight assumed; 25% recovery model is expected sensitivity not empirical failure rate; stress case adds one complete gate per multi-member batch.","Only selected gate+repro costs compared; other runs/failures/push costs and end-to-end queue contention, verification latency and merge overhead excluded; wait measures staging only.","Last incomplete batches flush at age limit outside observation if needed; separate release lines never mixed; no production dispatch or measured savings."],tests:"Synthetic size flush, age flush, separate release lines, max cost, preservation, uniqueness and wait bounds passed in JS runtime",rows,excluded,scenarios};
}
const inventory = JSON.parse(readFileSync(".github/metrics/ci-before-pr50-runs.json", "utf8"));
const costs = JSON.parse(readFileSync(".github/metrics/ci-before-pr50-job-costs-corrected.json", "utf8"));
process.stdout.write(JSON.stringify(model(inventory, costs), null, 2) + "\n");
