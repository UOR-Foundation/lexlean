// Experimental DOD-8 attempt-aware selection policy. NOT wired to CI or authorization.
// Inputs must come from a trusted, atomic provider collector and actual byte/receipt verifier.
// complete/receiptVerified flags are adapter outputs, never assertions accepted from a PR artifact.
// Executions are canonical actual origin executions, not provider-carried duplicate job records.
// A separate normative join must still verify all contents and fail on errors.
import {readFileSync} from 'node:fs';
export function resolveIntelRecovery(envelope,history,verifiedArtifacts){
 const assert=(condition,message)=>{if(!condition)throw Error(message)};
 const sha=/^[a-f0-9]{40}$/,digest=/^[a-f0-9]{64}$/,positive=x=>Number.isSafeInteger(x)&&x>0;
 const roles=["construction","moves","validate","lower","print"],consumers=roles.slice(1);
 assert(envelope&&positive(envelope.repo)&&positive(envelope.run)&&sha.test(envelope.head)&&sha.test(envelope.tree)&&sha.test(envelope.workflow)&&positive(envelope.attempt),"invalid frozen envelope");
 const same=x=>x.repo===envelope.repo&&x.run===envelope.run&&x.head===envelope.head&&x.tree===envelope.tree&&x.workflow===envelope.workflow;
 assert(Array.isArray(history)&&history.length===envelope.attempt,"missing attempt history");
 const latest=new Map(),seenJobs=new Set(),seenArtifacts=new Set(),seenNames=new Set();
 for(let index=0;index<history.length;index++){
  const page=history[index],attempt=index+1;
  assert(same(page)&&page.attempt===attempt&&page.complete===true&&page.terminal===true&&Array.isArray(page.executions),"unverified attempt inventory");
  assert(index!==0||page.executions.length===roles.length,"incomplete initial execution");
  assert(page.executions.length>0,"empty retry");
  const seenRoles=new Set();
  for(const job of page.executions){
   assert(same(job)&&job.attempt===attempt&&roles.includes(job.role)&&positive(job.id)&&!seenJobs.has(job.id)&&!seenRoles.has(job.role),"foreign or duplicate execution");
   seenJobs.add(job.id);seenRoles.add(job.role);
   assert(["success","failure","cancelled"].includes(job.conclusion),"nonterminal or unknown execution verdict");
   if(job.conclusion==="success"){
    assert(positive(job.artifactId)&&digest.test(job.archiveDigest)&&digest.test(job.constructionDigest),"missing successful artifact identity");
    assert(job.artifactName==="intel-"+job.role+"-a"+attempt,"mutable artifact name");
    assert(!seenArtifacts.has(job.artifactId)&&!seenNames.has(job.artifactName),"reused artifact identity");
    seenArtifacts.add(job.artifactId);seenNames.add(job.artifactName);
   }
   latest.set(job.role,job);
  }
  if(seenRoles.has("construction"))assert(consumers.every(role=>seenRoles.has(role)),"construction replacement requires all lanes");
 }
 assert(latest.size===roles.length,"incomplete effective inventory");
 const construction=latest.get("construction");
 assert(construction.conclusion==="success","latest construction unsuccessful");
 assert(Array.isArray(verifiedArtifacts)&&verifiedArtifacts.length===roles.length&&new Set(verifiedArtifacts.map(x=>x.id)).size===roles.length,"missing/duplicate verified artifact");
 const selected=roles.map(role=>{
  const j=latest.get(role);
  assert(j.conclusion==="success","latest execution unsuccessful: "+role);
  assert(j.constructionDigest===construction.constructionDigest,"mixed construction digest");
  assert(role==="construction"?j.constructionAttempt===j.attempt:j.constructionAttempt===construction.attempt&&j.attempt>=construction.attempt,"wrong construction origin");
  const a=verifiedArtifacts.find(x=>x.id===j.artifactId);
  assert(a&&same(a)&&a.role===role&&a.name===j.artifactName&&a.jobId===j.id&&a.attempt===j.attempt&&!a.expired&&a.archiveDigest===j.archiveDigest&&a.observedArchiveDigest===j.archiveDigest&&a.constructionDigest===construction.constructionDigest&&a.constructionAttempt===construction.attempt,"artifact provenance/bytes mismatch: "+role);
  assert(a.receiptVerified===true&&a.inventoryVerified===true,"unverified receipt or inventory: "+role);
  return {role,executionAttempt:j.attempt,constructionAttempt:construction.attempt,jobId:j.id,artifactId:a.id,archiveDigest:a.archiveDigest};
 });
 return {verdict:"READY_FOR_NORMATIVE_JOIN",selected,retained:selected.filter(x=>x.executionAttempt<envelope.attempt).length,acceptanceAuthority:false};
}
const fixture={
  "envelope": {
    "repo": 1335579506,
    "run": 45520783138,
    "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
    "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
    "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
    "attempt": 2
  },
  "history": [
    {
      "repo": 1335579506,
      "run": 45520783138,
      "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
      "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
      "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
      "attempt": 1,
      "complete": true,
      "terminal": true,
      "executions": [
        {
          "repo": 1335579506,
          "run": 45520783138,
          "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
          "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
          "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
          "role": "construction",
          "attempt": 1,
          "id": 1,
          "conclusion": "success",
          "artifactId": 101,
          "artifactName": "intel-construction-a1",
          "archiveDigest": "0000000000000000000000000000000000000000000000000000000000000001",
          "constructionDigest": "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
          "constructionAttempt": 1
        },
        {
          "repo": 1335579506,
          "run": 45520783138,
          "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
          "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
          "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
          "role": "moves",
          "attempt": 1,
          "id": 2,
          "conclusion": "success",
          "artifactId": 102,
          "artifactName": "intel-moves-a1",
          "archiveDigest": "0000000000000000000000000000000000000000000000000000000000000002",
          "constructionDigest": "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
          "constructionAttempt": 1
        },
        {
          "repo": 1335579506,
          "run": 45520783138,
          "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
          "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
          "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
          "role": "validate",
          "attempt": 1,
          "id": 3,
          "conclusion": "success",
          "artifactId": 103,
          "artifactName": "intel-validate-a1",
          "archiveDigest": "0000000000000000000000000000000000000000000000000000000000000003",
          "constructionDigest": "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
          "constructionAttempt": 1
        },
        {
          "repo": 1335579506,
          "run": 45520783138,
          "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
          "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
          "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
          "role": "lower",
          "attempt": 1,
          "id": 4,
          "conclusion": "cancelled",
          "artifactId": 104,
          "artifactName": "intel-lower-a1",
          "archiveDigest": "0000000000000000000000000000000000000000000000000000000000000004",
          "constructionDigest": "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
          "constructionAttempt": 1
        },
        {
          "repo": 1335579506,
          "run": 45520783138,
          "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
          "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
          "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
          "role": "print",
          "attempt": 1,
          "id": 5,
          "conclusion": "success",
          "artifactId": 105,
          "artifactName": "intel-print-a1",
          "archiveDigest": "0000000000000000000000000000000000000000000000000000000000000005",
          "constructionDigest": "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
          "constructionAttempt": 1
        }
      ]
    },
    {
      "repo": 1335579506,
      "run": 45520783138,
      "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
      "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
      "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
      "attempt": 2,
      "complete": true,
      "terminal": true,
      "executions": [
        {
          "repo": 1335579506,
          "run": 45520783138,
          "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
          "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
          "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
          "role": "lower",
          "attempt": 2,
          "id": 6,
          "conclusion": "success",
          "artifactId": 106,
          "artifactName": "intel-lower-a2",
          "archiveDigest": "0000000000000000000000000000000000000000000000000000000000000006",
          "constructionDigest": "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
          "constructionAttempt": 1
        }
      ]
    }
  ],
  "verifiedArtifacts": [
    {
      "repo": 1335579506,
      "run": 45520783138,
      "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
      "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
      "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
      "id": 101,
      "name": "intel-construction-a1",
      "jobId": 1,
      "role": "construction",
      "attempt": 1,
      "expired": false,
      "archiveDigest": "0000000000000000000000000000000000000000000000000000000000000001",
      "observedArchiveDigest": "0000000000000000000000000000000000000000000000000000000000000001",
      "constructionDigest": "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
      "constructionAttempt": 1,
      "receiptVerified": true,
      "inventoryVerified": true
    },
    {
      "repo": 1335579506,
      "run": 45520783138,
      "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
      "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
      "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
      "id": 102,
      "name": "intel-moves-a1",
      "jobId": 2,
      "role": "moves",
      "attempt": 1,
      "expired": false,
      "archiveDigest": "0000000000000000000000000000000000000000000000000000000000000002",
      "observedArchiveDigest": "0000000000000000000000000000000000000000000000000000000000000002",
      "constructionDigest": "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
      "constructionAttempt": 1,
      "receiptVerified": true,
      "inventoryVerified": true
    },
    {
      "repo": 1335579506,
      "run": 45520783138,
      "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
      "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
      "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
      "id": 103,
      "name": "intel-validate-a1",
      "jobId": 3,
      "role": "validate",
      "attempt": 1,
      "expired": false,
      "archiveDigest": "0000000000000000000000000000000000000000000000000000000000000003",
      "observedArchiveDigest": "0000000000000000000000000000000000000000000000000000000000000003",
      "constructionDigest": "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
      "constructionAttempt": 1,
      "receiptVerified": true,
      "inventoryVerified": true
    },
    {
      "repo": 1335579506,
      "run": 45520783138,
      "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
      "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
      "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
      "id": 105,
      "name": "intel-print-a1",
      "jobId": 5,
      "role": "print",
      "attempt": 1,
      "expired": false,
      "archiveDigest": "0000000000000000000000000000000000000000000000000000000000000005",
      "observedArchiveDigest": "0000000000000000000000000000000000000000000000000000000000000005",
      "constructionDigest": "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
      "constructionAttempt": 1,
      "receiptVerified": true,
      "inventoryVerified": true
    },
    {
      "repo": 1335579506,
      "run": 45520783138,
      "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
      "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
      "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
      "id": 106,
      "name": "intel-lower-a2",
      "jobId": 6,
      "role": "lower",
      "attempt": 2,
      "expired": false,
      "archiveDigest": "0000000000000000000000000000000000000000000000000000000000000006",
      "observedArchiveDigest": "0000000000000000000000000000000000000000000000000000000000000006",
      "constructionDigest": "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
      "constructionAttempt": 1,
      "receiptVerified": true,
      "inventoryVerified": true
    }
  ]
};
const rebuilt={
  "envelope": {
    "repo": 1335579506,
    "run": 45520783138,
    "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
    "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
    "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
    "attempt": 2
  },
  "history": [
    {
      "repo": 1335579506,
      "run": 45520783138,
      "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
      "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
      "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
      "attempt": 1,
      "complete": true,
      "terminal": true,
      "executions": [
        {
          "repo": 1335579506,
          "run": 45520783138,
          "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
          "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
          "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
          "role": "construction",
          "attempt": 1,
          "id": 1,
          "conclusion": "success",
          "artifactId": 101,
          "artifactName": "intel-construction-a1",
          "archiveDigest": "0000000000000000000000000000000000000000000000000000000000000001",
          "constructionDigest": "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
          "constructionAttempt": 1
        },
        {
          "repo": 1335579506,
          "run": 45520783138,
          "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
          "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
          "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
          "role": "moves",
          "attempt": 1,
          "id": 2,
          "conclusion": "success",
          "artifactId": 102,
          "artifactName": "intel-moves-a1",
          "archiveDigest": "0000000000000000000000000000000000000000000000000000000000000002",
          "constructionDigest": "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
          "constructionAttempt": 1
        },
        {
          "repo": 1335579506,
          "run": 45520783138,
          "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
          "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
          "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
          "role": "validate",
          "attempt": 1,
          "id": 3,
          "conclusion": "success",
          "artifactId": 103,
          "artifactName": "intel-validate-a1",
          "archiveDigest": "0000000000000000000000000000000000000000000000000000000000000003",
          "constructionDigest": "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
          "constructionAttempt": 1
        },
        {
          "repo": 1335579506,
          "run": 45520783138,
          "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
          "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
          "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
          "role": "lower",
          "attempt": 1,
          "id": 4,
          "conclusion": "cancelled",
          "artifactId": 104,
          "artifactName": "intel-lower-a1",
          "archiveDigest": "0000000000000000000000000000000000000000000000000000000000000004",
          "constructionDigest": "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
          "constructionAttempt": 1
        },
        {
          "repo": 1335579506,
          "run": 45520783138,
          "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
          "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
          "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
          "role": "print",
          "attempt": 1,
          "id": 5,
          "conclusion": "success",
          "artifactId": 105,
          "artifactName": "intel-print-a1",
          "archiveDigest": "0000000000000000000000000000000000000000000000000000000000000005",
          "constructionDigest": "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
          "constructionAttempt": 1
        }
      ]
    },
    {
      "repo": 1335579506,
      "run": 45520783138,
      "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
      "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
      "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
      "attempt": 2,
      "complete": true,
      "terminal": true,
      "executions": [
        {
          "repo": 1335579506,
          "run": 45520783138,
          "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
          "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
          "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
          "role": "construction",
          "attempt": 2,
          "id": 10,
          "conclusion": "success",
          "artifactId": 110,
          "artifactName": "intel-construction-a2",
          "archiveDigest": "000000000000000000000000000000000000000000000000000000000000000a",
          "constructionDigest": "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
          "constructionAttempt": 2
        },
        {
          "repo": 1335579506,
          "run": 45520783138,
          "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
          "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
          "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
          "role": "moves",
          "attempt": 2,
          "id": 11,
          "conclusion": "success",
          "artifactId": 111,
          "artifactName": "intel-moves-a2",
          "archiveDigest": "000000000000000000000000000000000000000000000000000000000000000b",
          "constructionDigest": "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
          "constructionAttempt": 2
        },
        {
          "repo": 1335579506,
          "run": 45520783138,
          "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
          "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
          "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
          "role": "validate",
          "attempt": 2,
          "id": 12,
          "conclusion": "success",
          "artifactId": 112,
          "artifactName": "intel-validate-a2",
          "archiveDigest": "000000000000000000000000000000000000000000000000000000000000000c",
          "constructionDigest": "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
          "constructionAttempt": 2
        },
        {
          "repo": 1335579506,
          "run": 45520783138,
          "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
          "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
          "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
          "role": "lower",
          "attempt": 2,
          "id": 13,
          "conclusion": "success",
          "artifactId": 113,
          "artifactName": "intel-lower-a2",
          "archiveDigest": "000000000000000000000000000000000000000000000000000000000000000d",
          "constructionDigest": "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
          "constructionAttempt": 2
        },
        {
          "repo": 1335579506,
          "run": 45520783138,
          "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
          "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
          "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
          "role": "print",
          "attempt": 2,
          "id": 14,
          "conclusion": "success",
          "artifactId": 114,
          "artifactName": "intel-print-a2",
          "archiveDigest": "000000000000000000000000000000000000000000000000000000000000000e",
          "constructionDigest": "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
          "constructionAttempt": 2
        }
      ]
    }
  ],
  "verifiedArtifacts": [
    {
      "repo": 1335579506,
      "run": 45520783138,
      "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
      "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
      "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
      "id": 110,
      "name": "intel-construction-a2",
      "jobId": 10,
      "role": "construction",
      "attempt": 2,
      "expired": false,
      "archiveDigest": "000000000000000000000000000000000000000000000000000000000000000a",
      "observedArchiveDigest": "000000000000000000000000000000000000000000000000000000000000000a",
      "constructionDigest": "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
      "constructionAttempt": 2,
      "receiptVerified": true,
      "inventoryVerified": true
    },
    {
      "repo": 1335579506,
      "run": 45520783138,
      "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
      "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
      "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
      "id": 111,
      "name": "intel-moves-a2",
      "jobId": 11,
      "role": "moves",
      "attempt": 2,
      "expired": false,
      "archiveDigest": "000000000000000000000000000000000000000000000000000000000000000b",
      "observedArchiveDigest": "000000000000000000000000000000000000000000000000000000000000000b",
      "constructionDigest": "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
      "constructionAttempt": 2,
      "receiptVerified": true,
      "inventoryVerified": true
    },
    {
      "repo": 1335579506,
      "run": 45520783138,
      "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
      "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
      "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
      "id": 112,
      "name": "intel-validate-a2",
      "jobId": 12,
      "role": "validate",
      "attempt": 2,
      "expired": false,
      "archiveDigest": "000000000000000000000000000000000000000000000000000000000000000c",
      "observedArchiveDigest": "000000000000000000000000000000000000000000000000000000000000000c",
      "constructionDigest": "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
      "constructionAttempt": 2,
      "receiptVerified": true,
      "inventoryVerified": true
    },
    {
      "repo": 1335579506,
      "run": 45520783138,
      "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
      "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
      "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
      "id": 113,
      "name": "intel-lower-a2",
      "jobId": 13,
      "role": "lower",
      "attempt": 2,
      "expired": false,
      "archiveDigest": "000000000000000000000000000000000000000000000000000000000000000d",
      "observedArchiveDigest": "000000000000000000000000000000000000000000000000000000000000000d",
      "constructionDigest": "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
      "constructionAttempt": 2,
      "receiptVerified": true,
      "inventoryVerified": true
    },
    {
      "repo": 1335579506,
      "run": 45520783138,
      "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
      "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
      "workflow": "3eeaa65755e2e12499f513590ff014db84404bdc",
      "id": 114,
      "name": "intel-print-a2",
      "jobId": 14,
      "role": "print",
      "attempt": 2,
      "expired": false,
      "archiveDigest": "000000000000000000000000000000000000000000000000000000000000000e",
      "observedArchiveDigest": "000000000000000000000000000000000000000000000000000000000000000e",
      "constructionDigest": "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
      "constructionAttempt": 2,
      "receiptVerified": true,
      "inventoryVerified": true
    }
  ]
};
const run=s=>resolveIntelRecovery(s.envelope,s.history,s.verifiedArtifacts);
const mutants=[s=>s.history.pop(),
s=>s.history[1].complete=false,
s=>s.history[1].terminal=false,
s=>s.history[1].executions[0].conclusion="failure",
s=>s.history[1].executions[0].id=1,
s=>s.history[1].executions[0].attempt=1,
s=>s.history[1].executions[0].head="0".repeat(40),
s=>s.history[1].executions[0].tree="0".repeat(40),
s=>s.history[1].executions[0].workflow="0".repeat(40),
s=>s.history[1].executions[0].artifactName="intel-lower",
s=>s.history[1].executions[0].artifactId=101,
s=>s.history[1].executions[0].constructionAttempt=2,
s=>s.history[1].executions[0].constructionDigest="d".repeat(64),
s=>s.verifiedArtifacts.pop(),
s=>s.verifiedArtifacts[1].id=s.verifiedArtifacts[0].id,
s=>s.verifiedArtifacts[1].expired=true,
s=>s.verifiedArtifacts[1].observedArchiveDigest="0".repeat(64),
s=>s.verifiedArtifacts[1].receiptVerified=false,
s=>s.verifiedArtifacts[1].inventoryVerified=false,
s=>s.verifiedArtifacts[1].jobId=99,
s=>s.verifiedArtifacts[1].attempt=2,
s=>s.verifiedArtifacts[1].repo=1,
s=>s.verifiedArtifacts[1].constructionAttempt=2,
s=>s.history[1].executions[0].role="construction",
s=>s.history[1].executions.push({...s.history[1].executions[0],id:99}),
s=>s.history[0].executions.pop()];
for(const m of mutants){const s=structuredClone(fixture);m(s);let failed=false;try{run(s)}catch{failed=true}if(!failed)throw Error('mutant survived')}
if(run(fixture).retained!==4||run(rebuilt).retained!==0)throw Error('wrong retention');
console.log(JSON.stringify(run(process.argv[2]?JSON.parse(readFileSync(process.argv[2],'utf8')):fixture),null,2));
