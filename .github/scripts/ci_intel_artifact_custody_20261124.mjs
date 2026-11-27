// DOD-8 advisory custody audit of observed Intel attempt 1. Provider metadata/log correlation only: not artifact byte validation, cryptographic attestation or release authority.
import {readFileSync} from 'node:fs';
const frozen={
  "repoId": 1335579506,
  "run": 45520783138,
  "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
  "tree": "e736f044c95fdade9b7c995faf27c406ec77505e",
  "attempt": 1,
  "artifacts": [
    {
      "id": 14987342669,
      "name": "intel-lower",
      "digest": "sha256:796842f7fbeea2fa83c0d6d352e86e59cd248d26141072afbbf99718f7cda065",
      "bytes": 2122,
      "expired": false,
      "expires": "2026-12-09T03:01:21Z",
      "created": "2026-11-25T03:01:22Z",
      "run": 45520783138,
      "repoId": 1335579506,
      "headRepoId": 1335579506,
      "head": "fa2280e226649e7d8fccacbad981f12faf88b602"
    },
    {
      "id": 14986960971,
      "name": "intel-moves",
      "digest": "sha256:bc37f7c7a14400e51267560d1c12364193b5512b52f76528bb541cef20435d82",
      "bytes": 8715,
      "expired": false,
      "expires": "2026-12-09T02:39:51Z",
      "created": "2026-11-25T02:39:53Z",
      "run": 45520783138,
      "repoId": 1335579506,
      "headRepoId": 1335579506,
      "head": "fa2280e226649e7d8fccacbad981f12faf88b602"
    },
    {
      "id": 14986834970,
      "name": "acceptance-gate-evidence-intel",
      "digest": "sha256:d7e858edc0d88c53d6787dc515d839d8fe585bfdd48ea26434b263e25f0c3b51",
      "bytes": 238,
      "expired": false,
      "expires": "2026-12-09T03:02:07Z",
      "created": "2026-11-25T03:02:08Z",
      "run": 45520783138,
      "repoId": 1335579506,
      "headRepoId": 1335579506,
      "head": "fa2280e226649e7d8fccacbad981f12faf88b602"
    },
    {
      "id": 14986615423,
      "name": "intel-validate",
      "digest": "sha256:84acdbfeb4761403fc1043b6a056907f9f79e60b30bfd5399624e3d8bbf5df28",
      "bytes": 5238,
      "expired": false,
      "expires": "2026-12-09T02:36:54Z",
      "created": "2026-11-25T02:36:56Z",
      "run": 45520783138,
      "repoId": 1335579506,
      "headRepoId": 1335579506,
      "head": "fa2280e226649e7d8fccacbad981f12faf88b602"
    },
    {
      "id": 14983924751,
      "name": "intel-print",
      "digest": "sha256:ebb307a5e0cf3da941da8e4387cb8e3c196abaaa203d1c205ba4d829f3afdb45",
      "bytes": 4552,
      "expired": false,
      "expires": "2026-12-09T02:07:30Z",
      "created": "2026-11-25T02:07:32Z",
      "run": 45520783138,
      "repoId": 1335579506,
      "headRepoId": 1335579506,
      "head": "fa2280e226649e7d8fccacbad981f12faf88b602"
    },
    {
      "id": 14980014236,
      "name": "intel-construction",
      "digest": "sha256:3f16b5c1764cc8229851bd561fcd59a6381d3e3d39b0ec816214e7674854ab53",
      "bytes": 73339999,
      "expired": false,
      "expires": "2026-12-09T01:07:26Z",
      "created": "2026-11-25T01:07:33Z",
      "run": 45520783138,
      "repoId": 1335579506,
      "headRepoId": 1335579506,
      "head": "fa2280e226649e7d8fccacbad981f12faf88b602"
    }
  ],
  "jobs": [
    {
      "id": 138893594271,
      "name": "Intel test construction and local targets",
      "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
      "attempt": 1,
      "status": "completed",
      "conclusion": "success",
      "start": "2026-11-25T00:51:24Z",
      "end": "2026-11-25T01:07:37Z",
      "upload": [
        {
          "start": "2026-11-25T01:07:25Z",
          "end": "2026-11-25T01:07:33Z",
          "status": "completed",
          "conclusion": "success"
        }
      ],
      "lines": [
        "2026-11-25T01:07:32.4606070Z SHA256 digest of uploaded artifact is 3f16b5c1764cc8229851bd561fcd59a6381d3e3d39b0ec816214e7674854ab53",
        "2026-11-25T01:07:33.0003700Z Artifact intel-construction successfully finalized. Artifact ID 14980014236"
      ]
    },
    {
      "id": 138900594197,
      "name": "Intel conformance (moves)",
      "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
      "attempt": 1,
      "status": "completed",
      "conclusion": "success",
      "start": "2026-11-25T01:07:43Z",
      "end": "2026-11-25T02:39:57Z",
      "upload": [
        {
          "start": "2026-11-25T02:39:46Z",
          "end": "2026-11-25T02:39:53Z",
          "status": "completed",
          "conclusion": "success"
        }
      ],
      "lines": [
        "2026-11-25T01:09:18.7389220Z SHA256 digest of downloaded artifact is 3f16b5c1764cc8229851bd561fcd59a6381d3e3d39b0ec816214e7674854ab53",
        "2026-11-25T02:39:52.4992560Z SHA256 digest of uploaded artifact is bc37f7c7a14400e51267560d1c12364193b5512b52f76528bb541cef20435d82",
        "2026-11-25T02:39:53.0470010Z Artifact intel-moves successfully finalized. Artifact ID 14986960971"
      ]
    },
    {
      "id": 138900594220,
      "name": "Intel conformance (validate)",
      "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
      "attempt": 1,
      "status": "completed",
      "conclusion": "success",
      "start": "2026-11-25T01:07:41Z",
      "end": "2026-11-25T02:37:00Z",
      "upload": [
        {
          "start": "2026-11-25T02:36:50Z",
          "end": "2026-11-25T02:36:56Z",
          "status": "completed",
          "conclusion": "success"
        }
      ],
      "lines": [
        "2026-11-25T01:08:38.0241920Z SHA256 digest of downloaded artifact is 3f16b5c1764cc8229851bd561fcd59a6381d3e3d39b0ec816214e7674854ab53",
        "2026-11-25T02:36:55.9147220Z SHA256 digest of uploaded artifact is 84acdbfeb4761403fc1043b6a056907f9f79e60b30bfd5399624e3d8bbf5df28",
        "2026-11-25T02:36:56.4327370Z Artifact intel-validate successfully finalized. Artifact ID 14986615423"
      ]
    },
    {
      "id": 138900594352,
      "name": "Intel conformance (print)",
      "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
      "attempt": 1,
      "status": "completed",
      "conclusion": "success",
      "start": "2026-11-25T01:20:58Z",
      "end": "2026-11-25T02:07:35Z",
      "upload": [
        {
          "start": "2026-11-25T02:07:29Z",
          "end": "2026-11-25T02:07:31Z",
          "status": "completed",
          "conclusion": "success"
        }
      ],
      "lines": [
        "2026-11-25T01:22:02.3405130Z SHA256 digest of downloaded artifact is 3f16b5c1764cc8229851bd561fcd59a6381d3e3d39b0ec816214e7674854ab53",
        "2026-11-25T02:07:31.4783120Z SHA256 digest of uploaded artifact is ebb307a5e0cf3da941da8e4387cb8e3c196abaaa203d1c205ba4d829f3afdb45",
        "2026-11-25T02:07:31.9692520Z Artifact intel-print successfully finalized. Artifact ID 14983924751"
      ]
    },
    {
      "id": 138900594300,
      "name": "Intel conformance (lower)",
      "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
      "attempt": 1,
      "status": "completed",
      "conclusion": "cancelled",
      "start": "2026-11-25T01:07:40Z",
      "end": "2026-11-25T03:01:27Z",
      "upload": [
        {
          "start": "2026-11-25T03:01:19Z",
          "end": "2026-11-25T03:01:21Z",
          "status": "completed",
          "conclusion": "success"
        }
      ],
      "lines": [
        "2026-11-25T01:08:53.1539690Z SHA256 digest of downloaded artifact is 3f16b5c1764cc8229851bd561fcd59a6381d3e3d39b0ec816214e7674854ab53",
        "2026-11-25T03:01:21.5669690Z SHA256 digest of uploaded artifact is 796842f7fbeea2fa83c0d6d352e86e59cd248d26141072afbbf99718f7cda065",
        "2026-11-25T03:01:21.8849700Z Artifact intel-lower successfully finalized. Artifact ID 14987342669"
      ]
    },
    {
      "id": 138946044173,
      "name": "build and non-Lean tests (x86_64-apple-darwin)",
      "head": "fa2280e226649e7d8fccacbad981f12faf88b602",
      "attempt": 1,
      "status": "completed",
      "conclusion": "failure",
      "start": "2026-11-25T03:01:29Z",
      "end": "2026-11-25T03:02:10Z",
      "upload": [
        {
          "start": "2026-11-25T03:02:07Z",
          "end": "2026-11-25T03:02:08Z",
          "status": "completed",
          "conclusion": "success"
        }
      ],
      "lines": [
        "2026-11-25T03:02:01.9476320Z SHA256 digest of downloaded artifact is ebb307a5e0cf3da941da8e4387cb8e3c196abaaa203d1c205ba4d829f3afdb45",
        "2026-11-25T03:02:01.9478145Z SHA256 digest of downloaded artifact is 84acdbfeb4761403fc1043b6a056907f9f79e60b30bfd5399624e3d8bbf5df28",
        "2026-11-25T03:02:01.9479919Z SHA256 digest of downloaded artifact is 796842f7fbeea2fa83c0d6d352e86e59cd248d26141072afbbf99718f7cda065",
        "2026-11-25T03:02:01.9485085Z SHA256 digest of downloaded artifact is bc37f7c7a14400e51267560d1c12364193b5512b52f76528bb541cef20435d82",
        "2026-11-25T03:02:03.4022419Z SHA256 digest of downloaded artifact is 3f16b5c1764cc8229851bd561fcd59a6381d3e3d39b0ec816214e7674854ab53",
        "2026-11-25T03:02:08.1611414Z SHA256 digest of uploaded artifact is d7e858edc0d88c53d6787dc515d839d8fe585bfdd48ea26434b263e25f0c3b51",
        "2026-11-25T03:02:08.5695261Z Artifact acceptance-gate-evidence-intel successfully finalized. Artifact ID 14986834970"
      ]
    }
  ]
};
function audit(s){
 const req=(v,m)=>{if(!v)throw Error(m)},sha=/^sha256:[0-9a-f]{64}$/;
 const roles=[["intel-construction","Intel test construction and local targets"],["intel-moves","Intel conformance (moves)"],["intel-validate","Intel conformance (validate)"],["intel-print","Intel conformance (print)"],["intel-lower","Intel conformance (lower)"],["acceptance-gate-evidence-intel","build and non-Lean tests (x86_64-apple-darwin)"]];
 req(s.repoId===1335579506&&s.run===45520783138&&s.head==="fa2280e226649e7d8fccacbad981f12faf88b602"&&s.tree==="e736f044c95fdade9b7c995faf27c406ec77505e"&&s.attempt===1,"foreign envelope");
 req(s.artifacts.length===6&&s.jobs.length===6&&new Set(s.artifacts.map(a=>a.id)).size===6&&new Set(s.jobs.map(j=>j.id)).size===6,"missing/duplicate records");
 const records=roles.map(([name,jobName])=>{
  const aa=s.artifacts.filter(a=>a.name===name),jj=s.jobs.filter(j=>j.name===jobName);req(aa.length===1&&jj.length===1,"missing/duplicate role "+name);const a=aa[0],j=jj[0];
  req(a.run===s.run&&a.repoId===s.repoId&&a.headRepoId===s.repoId&&a.head===s.head&&j.head===s.head&&j.attempt===s.attempt&&j.status==="completed","foreign artifact/job");
  req(!a.expired&&sha.test(a.digest)&&a.bytes>0&&Date.parse(a.expires)>Date.parse(j.end),"expired/malformed artifact");
  req(j.upload.length===1&&j.upload[0].status==="completed"&&j.upload[0].conclusion==="success","upload not successful");
  const u=j.upload[0],inside=t=>Date.parse(t)>=Date.parse(u.start)-1000&&Date.parse(t)<=Date.parse(u.end)+1000;
  const d=j.lines.map(l=>l.match(/^(\S+) SHA256 digest of uploaded artifact is ([a-f0-9]{64})\s*$/)).filter(Boolean);
  const f=j.lines.map(l=>l.match(/^(\S+) Artifact ([\w-]+) successfully finalized\. Artifact ID (\d+)\s*$/)).filter(Boolean);
  req(d.length===1&&f.length===1&&inside(d[0][1])&&inside(f[0][1])&&"sha256:"+d[0][2]===a.digest&&f[0][2]===name&&Number(f[0][3])===a.id&&Date.parse(d[0][1])<=Date.parse(f[0][1]),"upload evidence mismatch "+name);
  const downloads=j.lines.map(l=>l.match(/^(\S+) SHA256 digest of downloaded artifact is ([a-f0-9]{64})\s*$/)).filter(Boolean);
  return {name,job:j.id,artifact:a.id,digest:a.digest,bytes:a.bytes,conclusion:j.conclusion,downloadDigests:downloads.map(m=>"sha256:"+m[2]),originAttempt:j.attempt};
 });
 const construction=records[0];req(construction.conclusion==="success","construction failed");
 for(const r of records.slice(1,5))req(r.downloadDigests.length===1&&r.downloadDigests[0]===construction.digest,"wrong construction download "+r.name);
 req(records.slice(1,4).every(r=>r.conclusion==="success")&&records[4].conclusion==="cancelled"&&records[5].conclusion==="failure","unexpected observed verdicts");
 return {records,retainedCandidates:records.slice(0,4).map(r=>r.artifact),rejected:[records[4].artifact,records[5].artifact],constructionDigest:construction.digest,verdict:"HOLD: provider-metadata custody is correlated, but existing verifier requires same attempt; artifact bytes and receipts not verified",requiredFutureDesign:["attempt-qualified immutable artifact names","bind construction-origin attempt independently from each lane-execution attempt","trusted provider job/attempt and artifact-ID/digest resolution","byte-level archive/receipt verification and complete join; no name-only lookup or stale-success fallback"]};
}
const mutants=[s=>s.artifacts.pop(),
s=>s.jobs[1].id=s.jobs[0].id,
s=>s.artifacts[0].head="wrong",
s=>s.artifacts[0].repoId=1,
s=>s.artifacts[0].expired=true,
s=>s.artifacts[0].digest="sha256:"+"0".repeat(64),
s=>s.jobs[1].lines=s.jobs[1].lines.filter(l=>!l.includes("downloaded artifact")),
s=>s.jobs[1].lines.push(s.jobs[1].lines.find(l=>l.includes("successfully finalized"))),
s=>s.jobs[1].attempt=2,
s=>s.jobs[4].conclusion="success",
s=>s.jobs[0].upload[0].start="2028-01-01T00:00:00Z",
s=>s.jobs[2].upload[0].conclusion="failure",
s=>s.artifacts[0].id=1];
for(const m of mutants){const s=structuredClone(frozen);m(s);let failed=false;try{audit(s)}catch{failed=true}if(!failed)throw Error('mutant survived')}
console.log(JSON.stringify(audit(process.argv[2]?JSON.parse(readFileSync(process.argv[2],'utf8')):frozen),null,2));
