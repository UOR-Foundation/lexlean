// Nonqualifying synthetic receipt contract. Run with node.
function exercise(){
 const assert=(v,m)=>{if(!v)throw Error(m)};
 const source={repo:"UOR-Foundation/lexlean",head:"frozen",tree:"tree",run:7,host:"aarch64-unknown-linux-gnu",attempt:2};
 const inventory=[{target:"lib",name:"same_name",ignored:false,lane:"producer"},{target:"doc",name:"same_name",ignored:false,lane:"producer"},{target:"conformance",name:"BS-07",ignored:false,lane:"lower"},{target:"conformance",name:"BS-08",ignored:false,lane:"moves"},{target:"conformance",name:"BS-09",ignored:true,lane:"moves"}];
 const key=x=>JSON.stringify([x.target,x.name]);const lanes=["producer","lower","moves"];
 // Selected provider records are independent input; this model does not authenticate or fetch them.
 const selected=lanes.map((lane,i)=>({lane,attempt:i===1?2:1,job:10+i,artifact:20+i,digest:"sha256:"+String(i+1).repeat(64),conclusion:"success",host:source.host,run:source.run,head:source.head,repo:source.repo,tree:source.tree}));
 const receipts=selected.map(p=>({...p,exit:0,results:inventory.filter(x=>x.lane===p.lane).map(x=>({target:x.target,name:x.name,status:x.ignored?"ignored":"ok"}))}));
 const clone=x=>JSON.parse(JSON.stringify(x));
 function validate(expected,selection,receipts,here){
  assert(new Set(expected.map(key)).size===expected.length&&expected.length>0,"ambiguous inventory");
  assert(expected.every(x=>lanes.includes(x.lane)&&typeof x.ignored==="boolean"),"invalid plan");
  const map=new Map(expected.map(x=>[key(x),x]));const seen=new Set();const unique=(a,f)=>new Set(a.map(f)).size===a.length;
  assert(selection.length===lanes.length&&receipts.length===lanes.length&&unique(selection,x=>x.lane)&&unique(receipts,x=>x.lane),"lane roster");
  assert(unique(selection,x=>x.job)&&unique(selection,x=>x.artifact),"duplicate provider identity");
  for(const lane of lanes){
   const p=selection.find(x=>x.lane===lane),r=receipts.find(x=>x.lane===lane);assert(p&&r,"missing lane");
   assert(Number.isInteger(p.attempt)&&p.attempt>=1&&p.attempt<=here.attempt,"invalid attempt");
   assert(Number.isInteger(p.job)&&p.job>0&&Number.isInteger(p.artifact)&&p.artifact>0&&/^sha256:[0-9a-f]{64}$/.test(p.digest),"provider identity");
   for(const f of ["repo","head","tree","run","host"])assert(p[f]===here[f],"foreign source or host");
   assert(p.conclusion==="success","provider failure");
   for(const f of ["repo","head","tree","run","host","attempt","job","artifact","digest","lane"])assert(r[f]===p[f],"receipt not selected");
   assert(r.exit===0,"command failure");
   for(const x of r.results){
    const k=key(x),e=map.get(k);assert(e,"foreign test");assert(e.lane===lane,"wrong lane");assert(!seen.has(k),"duplicate execution");assert(x.status===(e.ignored?"ignored":"ok"),"changed test outcome");seen.add(k);
   }
  }
  assert(seen.size===map.size,"missing test");return seen.size;
 }
 let count=0;const cases=[];function test(name,mutate,reason){const e=clone(inventory),p=clone(selected),r=clone(receipts),h=clone(source);mutate(e,p,r,h);let error=null,result;try{result=validate(e,p,r,h)}catch(x){error=x.message}assert(error===reason,`${name}: expected ${reason}, got ${error}`);cases.push({name,expected:reason??"accept",actual:error??"accept",result});count++}
 test("valid retained attempt and identical test names in different targets",()=>{},null);
 test("zero provider attempt",(_e,p,r)=>{p[0].attempt=r[0].attempt=0},"invalid attempt");
 test("unselected older receipt",(_e,_p,r)=>{r[1].attempt=1},"receipt not selected");
 test("swapped lanes",(_e,_p,r)=>{const a=r[1].results;r[1].results=r[2].results;r[2].results=a},"wrong lane");
 test("producer absorbs conformance",(_e,_p,r)=>{r[0].results.push(r[1].results.pop())},"wrong lane");
 test("duplicate plan identity",(e)=>e.push({...e[0]}),"ambiguous inventory");
 test("duplicate execution",(_e,_p,r)=>r[0].results.push({...r[0].results[0]}),"duplicate execution");
 test("missing test",(_e,_p,r)=>r[2].results.pop(),"missing test");
 test("missing lane",(_e,_p,r)=>r.pop(),"lane roster");
 test("wrong host",(_e,p,r)=>{p[0].host=r[0].host="x86"},"foreign source or host");
 test("failed provider",(_e,p)=>p[0].conclusion="failure","provider failure");
 test("nonzero command",(_e,_p,r)=>r[0].exit=1,"command failure");
 test("ignored changed to ok",(_e,_p,r)=>r[2].results[1].status="ok","changed test outcome");
 test("failed test masked",(_e,_p,r)=>r[0].results[0].status="FAILED","changed test outcome");
 test("duplicate artifact",(_e,p)=>p[1].artifact=p[0].artifact,"duplicate provider identity");
 test("digest substitution",(_e,_p,r)=>r[0].digest="sha256:"+"f".repeat(64),"receipt not selected");
 test("wrong tree",(_e,p,r)=>{p[0].tree=r[0].tree="old"},"foreign source or host");
 return {schema:"lexlean/advisory-host-receipt-contract/1",authority:false,checks:count,cases,scope:"Synthetic contract model only. Independent expected inventory and provider selection assumed, not authenticated; no actual archive bytes or command transcripts validated, no workflow integration."};
}
console.log(JSON.stringify(exercise(),null,2));
