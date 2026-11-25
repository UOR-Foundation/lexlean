// Advisory synthetic discrete-event model, no normative authority. Run with node.
function runModel(){
 const assert=(v,m)=>{if(!v)throw Error(m)};
 const profiles=[{id:"fixture-heavy",build:12,other:5,repro:9,d:[22,18,15,8,7,4,3,1]},{id:"short-unit",build:4,other:3,repro:6,d:[9,7,5,4,4,3,2,1]}];
 // Separate Linux and native pools. External reservations are fixed, not inferred provider data.
 const scenarios=[{id:"unconstrained",linux:8,native:2,busy:0},{id:"linux-contention",linux:3,native:2,busy:8},{id:"single-linux-slot",linux:1,native:1,busy:0},{id:"native-tail",linux:4,native:1,busy:5,nativeDuration:75}];
 function graph(p,mode,s){
  const buckets=[[],[],[]];p.d.forEach((d,i)=>{let k=mode==="balanced"?buckets.reduce((best,b,j)=>b.reduce((a,t)=>a+t.duration,0)<buckets[best].reduce((a,t)=>a+t.duration,0)?j:best,0):Math.floor(i/3);buckets[k].push({id:"t"+i,duration:d})});
  return [{id:"build",pool:"linux",deps:[],work:p.build,tests:[]},{id:"native",pool:"native",deps:[],work:s.nativeDuration??20,tests:["native"]},{id:"repro",pool:"linux",deps:[],work:p.repro,tests:["repro"]},{id:"other",pool:"linux",deps:[],work:p.other,tests:["other"]},...buckets.map((b,i)=>({id:"lane"+i,pool:"linux",deps:["build"],work:b.reduce((a,t)=>a+t.duration,0),tests:b.map(t=>t.id),reserve:mode==="barrier"})),{id:"join",pool:"linux",deps:["native","repro","other","lane0","lane1","lane2"],work:1,tests:[]}];
 }
 function validate(p,g,s){
  assert(Number.isInteger(s.linux)&&s.linux>0&&Number.isInteger(s.native)&&s.native>0,"capacity");
  assert(Number.isFinite(s.busy)&&s.busy>=0,"external reservation");
  const by=Object.fromEntries(g.map(n=>[n.id,n]));assert(g.length===8&&Object.keys(by).length===8&&["build","native","repro","other","lane0","lane1","lane2","join"].every(k=>by[k]),"job inventory");
  const tests=g.flatMap(n=>n.tests);assert(JSON.stringify(tests.sort())===JSON.stringify([...p.d.map((_,i)=>"t"+i),"native","other","repro"].sort()),"test inventory");
  for(const n of g){assert(["linux","native"].includes(n.pool)&&n.work>0&&Number.isFinite(n.work),"job data");assert(n.deps.every(d=>by[d]&&d!==n.id)&&new Set(n.deps).size===n.deps.length,"dependencies");if(n.id.startsWith("lane")){assert(n.deps.length===1&&n.deps[0]==="build","construction dependency");assert(n.work===n.tests.reduce((a,t)=>a+p.d[Number(t.slice(1))],0),"work binding");}else if(n.id==="join"){assert(JSON.stringify([...n.deps].sort())===JSON.stringify(["native","repro","other","lane0","lane1","lane2"].sort()),"join dependency");}else {assert(!n.deps.length,"root dependency");}}
  assert(by.build.work===p.build&&by.repro.work===p.repro&&by.other.work===p.other&&by.native.work===(s.nativeDuration??20)&&by.join.work===1,"fixed work binding");
 }
 function simulate(p,g,s,order="producer-first"){
  validate(p,g,s);const waiting=new Map(g.map(n=>[n.id,n])),active=[],done={},rows=[];let now=0;
  const rank=n=>order==="lane-first"?(n.id.startsWith("lane")?0:n.id==="build"?1:2):(n.id==="build"?0:n.id.startsWith("lane")?1:2);
  while(waiting.size||active.length){
   for(let i=active.length-1;i>=0;i--){const r=active[i];if(r.end===now){done[r.id]=r;rows.push(r);active.splice(i,1);}}
   for(const r of active){if(r.start===null&&r.deps.every(d=>done[d])){r.start=now;r.end=now+r.work;}}
   const candidates=[...waiting.values()].filter(n=>n.reserve||n.deps.every(d=>done[d])).sort((a,b)=>rank(a)-rank(b)||a.id.localeCompare(b.id));
   for(const n of candidates){const cap=s[n.pool]-(n.pool==="linux"&&now<s.busy?1:0);if(active.filter(r=>r.pool===n.pool).length>=cap)continue;const ready=n.deps.every(d=>done[d]);const r={...n,allocated:now,start:ready?now:null,end:ready?now+n.work:null};active.push(r);waiting.delete(n.id);}
   if(!waiting.size&&!active.length)break;
   const future=active.filter(r=>r.end!==null&&r.end>now).map(r=>r.end);if(s.busy>now)future.push(s.busy);if(!future.length)return {status:"deadlock",reason:"allocated waiters prevent prerequisites from acquiring capacity",pending:[...waiting.keys()],allocated:active.map(r=>r.id)};
   now=Math.min(...future);
  }
  const runner=rows.reduce((a,r)=>a+r.end-r.allocated,0),work=rows.reduce((a,r)=>a+r.work,0);
  for(const r of rows){assert(r.deps.every(d=>done[d].end<=r.start),"precedence violation");assert(r.end-r.start===r.work,"work conservation");}
  for(const t of new Set(rows.flatMap(r=>[r.allocated,r.end,s.busy]))){for(const pool of ["linux","native"]){let count=rows.filter(r=>r.pool===pool&&r.allocated<=t&&t<r.end).length;assert(count<=s[pool]-(pool==="linux"&&t<s.busy?1:0),"overallocated capacity");}}
  return {status:"complete",elapsed:done.join.end,runner,work,idle:runner-work,rows};
 }
 let checks=0;const test=(v,m)=>{assert(v,m);checks++};const results=[];
 for(const p of profiles)for(const s of scenarios){
  const b=simulate(p,graph(p,"barrier",s),s),d=simulate(p,graph(p,"dependency",s),s),t=simulate(p,graph(p,"balanced",s),s),reverse=simulate(p,graph(p,"barrier",s),s,"lane-first");
  test(b.status==="complete"&&d.status==="complete"&&t.status==="complete","producer first terminates");
  test(b.work===d.work&&d.work===t.work,"work equal");
  test(d.idle===0&&t.idle===0&&b.runner>=d.runner,"no allocated dependency wait");
  test(d.rows.length===8&&t.rows.length===8,"coverage");
  if(s.linux===1)test(reverse.status==="deadlock","reverse order deadlock");
  if(s.id==="native-tail")test(t.elapsed===d.elapsed,"native critical path");
  results.push({profile:p.id,scenario:s.id,assumptions:s,barrier:b,dependency:d,balanced:t,reverseArrival:reverse.status});
 }
 // Negative controls assert exact reasons.
 function refuse(fn,msg){let e;try{fn()}catch(x){e=x}test(e?.message===msg,"expected "+msg+" got "+e?.message)}
 const p=profiles[0],s=scenarios[0];let g=graph(p,"balanced",s);g.find(n=>n.id==="lane0").work++;refuse(()=>simulate(p,g,s),"work binding");
 g=graph(p,"balanced",s);g.find(n=>n.id==="lane0").deps=[];refuse(()=>simulate(p,g,s),"construction dependency");
 g=graph(p,"balanced",s);g.find(n=>n.id==="join").deps.pop();refuse(()=>simulate(p,g,s),"join dependency");
 g=graph(p,"balanced",s);g.find(n=>n.id==="lane0").tests.push(g.find(n=>n.id==="lane1").tests[0]);refuse(()=>simulate(p,g,s),"test inventory");
 refuse(()=>simulate(p,graph(p,"balanced",s),{...s,linux:0}),"capacity");
 return {schema:"lexlean/advisory-finite-fleet/1",authority:false,checks,results,scope:"Synthetic discrete-event scheduling; fixed external reservation, nonpreemptive homogeneous pools, full work durations; no provider policy or actual workload parity asserted."};
}
console.log(JSON.stringify(runModel(),null,2));
