// Advisory DOD-8b snapshot comparison, never a live authorization control.
// Invoke: node .github/scripts/ci_batch_composition_snapshot.mjs [fresh-snapshot.json]
// The retained snapshot is a reference, not a signature or independently authenticated approval.
import { readFileSync } from "node:fs";
function validate(s, expected){
 const hold=reason=>({status:"HOLD",reason,authority:false});
 const eq=(x,y)=>JSON.stringify(x)===JSON.stringify(y), sha=x=>typeof x==="string"&&/^[a-f0-9]{40}$/.test(x);
 if(!s||!expected||s.schema!=="lexlean/batch-composition-snapshot/1"||s.repository!==expected.repository||s.release!==expected.release||s.base!==expected.base||!sha(s.base))return hold("base_or_repository");
 if(!s.candidate||!sha(s.candidate.head)||!sha(s.candidate.tree)||!eq(s.candidate,expected.candidate))return hold("candidate_identity");
 if(!Array.isArray(s.members)||s.members.length!==2||!eq(s.members,expected.members)||new Set(s.members.map(x=>x.pr)).size!==2||new Set(s.members.map(x=>x.head)).size!==2||!eq(s.candidate.parents,s.members.map(x=>x.head)))return hold("member_identity");
 if(!Array.isArray(s.live)||!eq(s.live,expected.live)||s.live.length!==2||s.live.some(x=>x.state!=="open"||x.draft!==true||x.base!==s.base||x.release!==s.release)||s.live[0].head!==s.candidate.head||s.live[1].head!==s.members[1].head)return hold("live_pr_changed");
 if(!Array.isArray(s.comparisons)||s.comparisons.length!==3||!eq(s.comparisons,expected.comparisons))return hold("inventory_changed");
 for(let i=0;i<3;i++){
  const x=s.comparisons[i];
  if(x.head!==[...s.members.map(m=>m.head),s.candidate.head][i]||x.base!==s.base||x.behind!==0||!Array.isArray(x.paths)||!x.paths.length||new Set(x.paths.map(p=>p.path)).size!==x.paths.length||x.paths.some(p=>typeof p.path!=="string"||!p.path||!["added","modified","removed"].includes(p.status)))return hold("inventory_incomplete");
 }
 const [a,b,c]=s.comparisons.map(x=>x.paths.map(p=>p.path)),union=[...new Set([...a,...b])].sort(),shared=a.filter(x=>b.includes(x)).sort();
 if(!eq(union,[...c].sort())||!eq(shared,s.overlap)||!eq(shared,expected.overlap))return hold("composition_delta");
 return {status:"SOURCE_SNAPSHOT_MATCH",reason:"Exact recorded identities and path union match; semantic resolution and full qualification remain required",authority:false};
}
function tests(snapshot){
 const clone=x=>JSON.parse(JSON.stringify(x));
 const cases=[["unchanged",s=>{},"SOURCE_SNAPSHOT_MATCH"],["base moved",s=>s.base="0".repeat(40),"HOLD"],["candidate changed",s=>s.candidate.head="0".repeat(40),"HOLD"],["tree changed",s=>s.candidate.tree="0".repeat(40),"HOLD"],["parent order",s=>s.candidate.parents.reverse(),"HOLD"],["missing parent",s=>s.candidate.parents.pop(),"HOLD"],["member changed",s=>s.members[1].head="0".repeat(40),"HOLD"],["duplicate member",s=>s.members[1]=s.members[0],"HOLD"],["source PR changed",s=>s.live[1].head="0".repeat(40),"HOLD"],["source PR closed",s=>s.live[1].state="closed","HOLD"],["candidate promoted",s=>s.live[0].draft=false,"HOLD"],["target changed",s=>s.live[0].release="main","HOLD"],["extra path",s=>s.comparisons[2].paths.push({path:"extra",status:"added"}),"HOLD"],["missing path",s=>s.comparisons[2].paths.pop(),"HOLD"],["duplicate path",s=>s.comparisons[2].paths.push(s.comparisons[2].paths[0]),"HOLD"],["status changed",s=>s.comparisons[2].paths[0].status="removed","HOLD"],["overlap missing",s=>s.overlap.pop(),"HOLD"],["behind",s=>s.comparisons[0].behind=1,"HOLD"],["foreign repository",s=>s.repository="other/repo","HOLD"]];
 return cases.map(([name,mut,expected])=>{let s=clone(snapshot);mut(s);let result=validate(s,snapshot);if(result.status!==expected||result.authority!==false)throw Error(name);return {name,expected,actual:result.status,authority:result.authority}});
}
const read=p=>JSON.parse(readFileSync(p,"utf8"));
const reference=read(".github/metrics/ci-batch-composition-snapshot-dod8-20261124.json").snapshot;
const result=process.argv[2]?validate(read(process.argv[2]),reference):{cases:tests(reference)};
process.stdout.write(JSON.stringify(result,null,2)+"\n");
if(result.status==="HOLD")process.exitCode=1;
