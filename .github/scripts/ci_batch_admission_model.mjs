// DOD-8b advisory decision prototype; synthetic review assertions are NOT trusted evidence.
// No filesystem writes, CI dispatch, merge, or release authority. Node 22: node .github/scripts/ci_batch_admission_model.mjs
function assess(m,now){
 const fail=reason=>({route:"HOLD",reason,authority:false});
 const sha=x=>typeof x==="string"&&/^[0-9a-f]{40}$/.test(x);
 if(!m||typeof m!=="object"||!Number.isSafeInteger(now)||now<0)return fail("malformed");
 if(!/^release\/v[0-9]+\.[0-9]+$/.test(m.release)||!sha(m.base)||!sha(m.currentBase)||m.base!==m.currentBase||!sha(m.tree))return fail("release_or_base");
 if(!Array.isArray(m.members)||!m.members.length||m.members.length>4)return fail("size");
 const seen=new Set(),heads=new Set();
 for(const x of m.members){
  if(!x||!Number.isSafeInteger(x.pr)||x.pr<=0||seen.has(x.pr)||!sha(x.head)||heads.has(x.head)||x.release!==m.release)return fail("member_identity");
  if(!Number.isSafeInteger(x.admittedAt)||x.admittedAt<0||x.admittedAt>now)return fail("clock");
  if(!["urgent","defer","unknown"].includes(x.priority)||!["identity","release","control","ordinary","unknown"].includes(x.impact))return fail("classification");
  if(x.reviewedHead!==x.head||x.pathsComplete!==true||x.compatibilityReviewed!==true||x.trustReviewed!==true||x.preflightPassed!==true)return fail("missing_review_evidence");
  if(x.priority==="unknown"||x.impact==="unknown")return fail("unknown");
  if(!Array.isArray(x.requires)||new Set(x.requires).size!==x.requires.length||x.requires.some(n=>!seen.has(n)))return fail("dependency_order");
  seen.add(x.pr);heads.add(x.head);
 }
 const special=m.members.some(x=>x.priority==="urgent"||x.impact!=="ordinary");
 if(special)return m.members.length===1?{route:"DIRECT_FULL_CANDIDATE",reason:"urgent_or_special",authority:false}:fail("special_requires_separate_qualification");
 if(m.members.some(x=>x.priority!=="defer"))return fail("priority");
 if(m.members.length===4||now-Math.min(...m.members.map(x=>x.admittedAt))>=12*3600000)return {route:"FREEZE_FULL_CANDIDATE",reason:"size_or_age",authority:false};
 return {route:"STAGING_CANDIDATE",reason:"accumulate",authority:false};
}
function test(){
 const now=Date.parse("2026-11-24T20:00:00Z"),sha=n=>String(n).repeat(40);
 const member=(pr,n)=>({pr,head:sha(n),reviewedHead:sha(n),release:"release/v0.4",admittedAt:now-1000,priority:"defer",impact:"ordinary",pathsComplete:true,compatibilityReviewed:true,trustReviewed:true,preflightPassed:true,requires:[]});
 const fixture=()=>({release:"release/v0.4",base:sha(9),currentBase:sha(9),tree:sha(8),members:[member(1,1),member(2,2)]});
 const cases=[
 ["valid staging",m=>{}, "STAGING_CANDIDATE"],
 ["age boundary",m=>m.members[0].admittedAt=now-12*3600000,"FREEZE_FULL_CANDIDATE"],
 ["size boundary",m=>m.members.push(member(3,3),member(4,4)),"FREEZE_FULL_CANDIDATE"],
 ["urgent direct",m=>{m.members.pop();m.members[0].priority="urgent"},"DIRECT_FULL_CANDIDATE"],
 ["identity direct",m=>{m.members.pop();m.members[0].impact="identity"},"DIRECT_FULL_CANDIDATE"],
 ["ordered dependency",m=>m.members[1].requires=[1],"STAGING_CANDIDATE"],
 ["reversed dependency",m=>m.members[0].requires=[2],"HOLD"],
 ["unresolved dependency",m=>m.members[1].requires=[99],"HOLD"],
 ["duplicate dependency",m=>m.members[1].requires=[1,1],"HOLD"],
 ["base moved",m=>m.currentBase=sha(7),"HOLD"],
 ["wrong release",m=>m.members[1].release="release/v0.5","HOLD"],
 ["wrong target",m=>m.release="main","HOLD"],
 ["duplicate PR",m=>m.members[1].pr=1,"HOLD"],
 ["duplicate head",m=>{m.members[1].head=sha(1);m.members[1].reviewedHead=sha(1)},"HOLD"],
 ["stale review",m=>m.members[0].reviewedHead=sha(7),"HOLD"],
 ["invalid tree",m=>m.tree="unknown","HOLD"],
 ["unknown urgency",m=>m.members[0].priority="unknown","HOLD"],
 ["unknown impact",m=>m.members[0].impact="unknown","HOLD"],
 ["urgent mixed",m=>m.members[0].priority="urgent","HOLD"],
 ["identity mixed",m=>m.members[0].impact="identity","HOLD"],
 ["release mixed",m=>m.members[0].impact="release","HOLD"],
 ["control mixed",m=>m.members[0].impact="control","HOLD"],
 ["future clock",m=>m.members[0].admittedAt=now+1,"HOLD"],
 ["oversize",m=>m.members.push(member(3,3),member(4,4),member(5,5)),"HOLD"],
 ...["pathsComplete","compatibilityReviewed","trustReviewed","preflightPassed"].map(k=>["missing "+k,m=>delete m.members[0][k],"HOLD"])
 ];
 return cases.map(([name,mutate,expected])=>{const m=fixture();mutate(m);const got=assess(m,now);if(got.route!==expected||got.authority!==false)throw Error(name+": "+JSON.stringify(got));return {name,expected,actual:got.route,authority:got.authority}});
}
const cases = test();
process.stdout.write(JSON.stringify({cases,passed:cases.length},null,2)+"\n");
