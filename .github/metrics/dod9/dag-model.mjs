// Advisory synthetic model for #603 DOD-9. Run: node .github/metrics/dod9/dag-model.mjs
// No provider, normative detector, cross-language benchmark or billing authority.
function experiment() {
  "use strict";
  const assert = (x,m) => { if(!x) throw Error(m); };
  const profiles = [
    {name:"shared-fixture-heavy",setup:12,setupOther:5,tests:[["a",22],["b",18],["c",15],["d",8],["e",7],["f",4],["g",3],["h",1]],repro:9},
    {name:"short-unit-heavy",setup:4,setupOther:3,tests:[["a",9],["b",7],["c",5],["d",4],["e",4],["f",3],["g",2],["h",1]],repro:6}
  ];
  const clone=x=>JSON.parse(JSON.stringify(x));
  const same=(a,b)=>JSON.stringify([...a].sort())===JSON.stringify([...b].sort());
  function partition(p,balanced) {
    const bins=[[],[],[]], sum=b=>b.reduce((n,x)=>n+x[1],0);
    p.tests.forEach((t,i)=>{const index=balanced?bins.reduce((best,b,j)=>sum(b)<sum(bins[best])?j:best,0):Math.min(2,Math.floor(i/3));bins[index].push(t);});
    return bins;
  }
  function graph(p,mode) {
    const bins=partition(p,mode==="balanced");
    const nodes=[{id:"build",deps:[],duration:p.setup,tests:[]},{id:"repro",deps:[],duration:p.repro,tests:["repro"]},{id:"other",deps:[],duration:p.setupOther,tests:["other"]}];
    bins.forEach((b,i)=>nodes.push({id:"lane"+i,deps:["build"],duration:b.reduce((s,t)=>s+t[1],0),tests:b.map(t=>t[0]),reserve:mode==="barrier"}));
    nodes.push({id:"join",deps:["repro","other","lane0","lane1","lane2"],duration:1,tests:[]}); return nodes;
  }
  function validate(p,nodes) {
    const ids=nodes.map(n=>n.id);assert(ids.length===new Set(ids).size,"duplicate job");
    assert(same(ids,["build","repro","other","lane0","lane1","lane2","join"]),"job inventory");
    assert(same(nodes.flatMap(n=>n.tests),[...p.tests.map(t=>t[0]),"repro","other"]),"test inventory");
    for(const n of nodes){
      assert(Number.isFinite(n.duration)&&n.duration>0,"invalid duration");
      assert(n.deps.every(d=>ids.includes(d)&&d!==n.id)&&new Set(n.deps).size===n.deps.length,"dependency identity");
      if(n.id.startsWith("lane"))assert(same(n.deps,["build"]),"missing construction dependency");
      if(n.id==="join")assert(same(n.deps,["repro","other","lane0","lane1","lane2"]),"missing join dependency");
      if(n.id==="build"||n.id==="repro"||n.id==="other")assert(n.deps.length===0,"unexpected root dependency");
    }
  }
  // Unlimited runner availability: reserve models a job allocated before its prerequisite,
  // not a provider queue. No claim about finite fleet scheduling.
  function schedule(p,nodes) {
    validate(p,nodes);const rows={};let pending=[...nodes];
    while(pending.length){const ready=pending.filter(n=>n.deps.every(d=>rows[d]));assert(ready.length,"cycle");
      for(const n of ready){const start=Math.max(0,...n.deps.map(d=>rows[d].end));const allocated=n.reserve?0:start;rows[n.id]={start,end:start+n.duration,allocated,minutes:start+n.duration-allocated,id:n.id,tests:n.tests};}
      pending=pending.filter(n=>!rows[n.id]);
    }
    return {makespan:rows.join.end,runnerMinutes:Object.values(rows).reduce((s,r)=>s+r.minutes,0),rows};
  }
  function receipt(n,attempt,conclusion="success",source="frozen-A") {return {job:n.id,attempt,conclusion,source,tests:[...n.tests],duration:n.duration};}
  // Independent fixture authority defines the graph; receipts cannot define their own expected inventory.
  function accept(p,nodes,history,source,attempt) {
    validate(p,nodes);const known=new Map(nodes.map(n=>[n.id,n]));const latest=new Map(),seen=new Set();
    for(const r of history){assert(known.has(r.job),"foreign job");assert(r.source===source,"stale source");assert(Number.isInteger(r.attempt)&&r.attempt>0&&r.attempt<=attempt,"attempt");assert(!seen.has(r.job+":"+r.attempt),"duplicate receipt");seen.add(r.job+":"+r.attempt);assert(same(r.tests,known.get(r.job).tests),"receipt inventory");assert(r.duration===known.get(r.job).duration,"duration");assert(["success","failure","cancelled"].includes(r.conclusion),"unknown result");if(!latest.has(r.job)||latest.get(r.job).attempt<r.attempt)latest.set(r.job,r);}
    assert(latest.size===nodes.length,"missing receipt");
    for(const n of nodes){const r=latest.get(n.id);assert(r.conclusion==="success","failed latest execution");for(const d of n.deps)assert(latest.get(d).attempt<=r.attempt,"stale dependent execution");}
    return [...latest.values()].map(r=>[r.job,r.attempt]);
  }
  let tests=0;const positive=(fn)=>{fn();tests++};const refuses=(fn,reason)=>{let err;try{fn()}catch(e){err=e}assert(err&&err.message===reason,"expected "+reason+", got "+err?.message);tests++};
  const results=[];
  for(const p of profiles){
    const b=graph(p,"barrier"),d=graph(p,"dependency"),bal=graph(p,"balanced");
    const s=[b,d,bal].map(g=>schedule(p,g));
    positive(()=>assert(s[0].makespan===s[1].makespan&&s[0].runnerMinutes-s[1].runnerMinutes===3*p.setup,"reservation accounting"));
    positive(()=>assert(s[2].makespan<=s[1].makespan&&s[2].runnerMinutes===s[1].runnerMinutes,"partition accounting"));
    const h=bal.map(n=>receipt(n,1,["lane1","join"].includes(n.id)?"cancelled":"success"));
    const recovered=[...h,...bal.filter(n=>["lane1","join"].includes(n.id)).map(n=>receipt(n,2))];
    positive(()=>assert(accept(p,bal,recovered,"frozen-A",2).filter(x=>x[1]===1).length===5,"retained executions"));
    let g=clone(bal);g.find(n=>n.id==="lane0").deps=[];refuses(()=>schedule(p,g),"missing construction dependency");
    g=clone(bal);g.find(n=>n.id==="join").deps.pop();refuses(()=>schedule(p,g),"missing join dependency");
    g=clone(bal);g.find(n=>n.id==="lane0").tests.pop();refuses(()=>schedule(p,g),"test inventory");
    g=clone(bal);g.find(n=>n.id==="lane0").tests.push(g.find(n=>n.id==="lane1").tests[0]);refuses(()=>schedule(p,g),"test inventory");
    let bad=clone(recovered);bad[0].source="old";refuses(()=>accept(p,bal,bad,"frozen-A",2),"stale source");
    bad=clone(recovered);bad.push(clone(bad[0]));refuses(()=>accept(p,bal,bad,"frozen-A",2),"duplicate receipt");
    bad=clone(recovered);bad=bad.filter(r=>r.job!=="repro");refuses(()=>accept(p,bal,bad,"frozen-A",2),"missing receipt");
    bad=clone(recovered);bad.find(r=>r.job==="lane1"&&r.attempt===2).conclusion="failure";refuses(()=>accept(p,bal,bad,"frozen-A",2),"failed latest execution");
    bad=clone(recovered);bad.push(receipt(bal.find(n=>n.id==="build"),2));refuses(()=>accept(p,bal,bad,"frozen-A",2),"stale dependent execution");
    bad=clone(recovered);bad.find(r=>r.job==="lane1"&&r.attempt===2).tests=[];refuses(()=>accept(p,bal,bad,"frozen-A",2),"receipt inventory");
    bad=clone(recovered);bad.find(r=>r.job==="lane1"&&r.attempt===2).conclusion="skipped";refuses(()=>accept(p,bal,bad,"frozen-A",2),"unknown result");
    const retry=bal.filter(n=>["lane1","join"].includes(n.id)).reduce((a,n)=>a+n.duration,0);
    results.push({profile:p.name,fixture:p,barrier:s[0],dependencyReady:s[1],balanced:s[2],recovery:{firstAttemptAssumedAllocatedMinutes:s[2].runnerMinutes,fullRerunAdditionalMinutes:s[2].runnerMinutes,selectiveAdditionalMinutes:retry,retainedJobs:5,rerunJobs:2,latestFailureDetector:"lane1",selected:accept(p,bal,recovered,"frozen-A",2)},scope:"synthetic minutes, unlimited runners, assumed full allocation on first attempt, no actual execution or provider authority"});
  }
  return {schema:"lexlean/advisory-dag-model/1",authority:false,testsPassed:tests,results};
}
console.log(JSON.stringify(experiment(), null, 2));
