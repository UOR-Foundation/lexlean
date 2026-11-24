// DOD-8b advisory evidence; not authorization or provider authentication.
// Run from checkout: node .github/scripts/ci_batch_correction_snapshot_20261124.mjs [independently-captured-observation.json]
// A corrective descendant invalidates the earlier frozen candidate, even if all paths still match.
// Provider compare and six blob pairs were observed independently. The connector does not expose the full commit tree or parent list; obtain those separately before qualification.
import { readFileSync } from "node:fs";
const reference={
  "schema": "lexlean/batch-correction-observation/1",
  "repository": "UOR-Foundation/lexlean",
  "live": [
    {
      "number": 609,
      "head": "a1a0db87b7625937a9be7784823906fe2409b67e",
      "base": "d553cbf42f4d51673f6e4040da2337dda4627c09",
      "release": "release/v0.4",
      "draft": true,
      "state": "open"
    },
    {
      "number": 607,
      "head": "d3d63f6a2e22a145db08c51927fb13d02e631296",
      "base": "d553cbf42f4d51673f6e4040da2337dda4627c09",
      "release": "release/v0.4",
      "draft": true,
      "state": "open"
    }
  ],
  "comparisons": [
    {
      "base": "d553cbf42f4d51673f6e4040da2337dda4627c09",
      "head": "a1a0db87b7625937a9be7784823906fe2409b67e",
      "merge_base": "d553cbf42f4d51673f6e4040da2337dda4627c09",
      "behind": 0,
      "ahead": 29,
      "paths": [
        {
          "path": ".github/scripts/check_regen_writers.py",
          "status": "added"
        },
        {
          "path": ".github/scripts/test_check_regen_writers.py",
          "status": "added"
        },
        {
          "path": ".github/scripts/test_export_custody_guard.py",
          "status": "added"
        },
        {
          "path": ".github/scripts/test_tag_gate_poll.py",
          "status": "added"
        },
        {
          "path": ".github/workflows/ci.yml",
          "status": "modified"
        },
        {
          "path": ".github/workflows/regenerate.yml",
          "status": "modified"
        },
        {
          "path": ".github/workflows/tag.yml",
          "status": "modified"
        },
        {
          "path": "crates/conformance/src/cases/bootstrap.rs",
          "status": "modified"
        },
        {
          "path": "crates/conformance/src/cases/repository.rs",
          "status": "modified"
        },
        {
          "path": "crates/conformance/src/export_custody.rs",
          "status": "modified"
        },
        {
          "path": "VERIFICATION.md",
          "status": "modified"
        }
      ]
    },
    {
      "base": "96f35dce8cb5b6b3ef8bf0bd06f974d32ed6e334",
      "head": "a1a0db87b7625937a9be7784823906fe2409b67e",
      "merge_base": "96f35dce8cb5b6b3ef8bf0bd06f974d32ed6e334",
      "behind": 0,
      "ahead": 2,
      "paths": [
        {
          "path": ".github/scripts/test_check_regen_writers.py",
          "status": "modified"
        },
        {
          "path": ".github/workflows/ci.yml",
          "status": "modified"
        },
        {
          "path": ".github/workflows/regenerate.yml",
          "status": "modified"
        },
        {
          "path": "crates/conformance/src/cases/repository.rs",
          "status": "modified"
        },
        {
          "path": "VERIFICATION.md",
          "status": "modified"
        }
      ]
    },
    {
      "base": "d3d63f6a2e22a145db08c51927fb13d02e631296",
      "head": "a1a0db87b7625937a9be7784823906fe2409b67e",
      "merge_base": "d3d63f6a2e22a145db08c51927fb13d02e631296",
      "behind": 0,
      "ahead": 21,
      "paths": [
        {
          "path": ".github/scripts/check_regen_writers.py",
          "status": "added"
        },
        {
          "path": ".github/scripts/test_check_regen_writers.py",
          "status": "added"
        },
        {
          "path": ".github/scripts/test_export_custody_guard.py",
          "status": "added"
        },
        {
          "path": ".github/workflows/ci.yml",
          "status": "modified"
        },
        {
          "path": ".github/workflows/regenerate.yml",
          "status": "modified"
        },
        {
          "path": "crates/conformance/src/cases/bootstrap.rs",
          "status": "modified"
        },
        {
          "path": "crates/conformance/src/cases/repository.rs",
          "status": "modified"
        },
        {
          "path": "crates/conformance/src/export_custody.rs",
          "status": "modified"
        },
        {
          "path": "VERIFICATION.md",
          "status": "modified"
        }
      ]
    }
  ],
  "unchanged": [
    {
      "path": ".github/scripts/check_regen_writers.py",
      "old": "ed0fb7941da8a1784d3f069ed497f8f6c1f3b3d1",
      "new": "ed0fb7941da8a1784d3f069ed497f8f6c1f3b3d1"
    },
    {
      "path": ".github/scripts/test_export_custody_guard.py",
      "old": "44ac4b9742f95ba82d5f6ede3f54fb37914ba175",
      "new": "44ac4b9742f95ba82d5f6ede3f54fb37914ba175"
    },
    {
      "path": ".github/scripts/test_tag_gate_poll.py",
      "old": "dff38f6537cf8273aafc3849e940786c3f304de2",
      "new": "dff38f6537cf8273aafc3849e940786c3f304de2"
    },
    {
      "path": ".github/workflows/tag.yml",
      "old": "4074d9f1c19ba44c5d98fc038491cc5fe2e72320",
      "new": "4074d9f1c19ba44c5d98fc038491cc5fe2e72320"
    },
    {
      "path": "crates/conformance/src/cases/bootstrap.rs",
      "old": "4010c485f81a5d70f26534f7d9e99a41018e7274",
      "new": "4010c485f81a5d70f26534f7d9e99a41018e7274"
    },
    {
      "path": "crates/conformance/src/export_custody.rs",
      "old": "a889651022f58202dc5a68fd8dab96887a78ef8d",
      "new": "a889651022f58202dc5a68fd8dab96887a78ef8d"
    }
  ]
};
function validate(s,reference){
 const hold=reason=>({status:"HOLD",reason,authority:false});
 const same=(a,b)=>JSON.stringify(a)===JSON.stringify(b);
 if(!s||s.schema!=="lexlean/batch-correction-observation/1"||s.repository!==reference.repository)return hold("schema_or_repository");
 if(!same(s.live,reference.live)||s.live.some(p=>!p.draft||p.state!=="open")||s.live.length!==2)return hold("pr_identity_or_state");
 if(!same(s.comparisons,reference.comparisons)||s.comparisons.length!==3)return hold("comparison_changed");
 const [full,delta,member]=s.comparisons;
 if(s.comparisons.some(c=>c.behind!==0||c.merge_base!==c.base||c.head!==s.live[0].head||new Set(c.paths.map(p=>p.path)).size!==c.paths.length)||full.base!==s.live[0].base||full.base!==s.live[1].base||member.base!==s.live[1].head||delta.ahead!==2||full.paths.length!==11||delta.paths.length!==5)return hold("ancestry_or_inventory");
 if(!same(s.unchanged,reference.unchanged)||s.unchanged.length!==6||s.unchanged.some(b=>!(/^[a-f0-9]{40}$/.test(b.old))||b.old!==b.new))return hold("blob_custody");
 const partition=[...delta.paths.map(p=>p.path),...s.unchanged.map(p=>p.path)].sort();
 if(!same(partition,full.paths.map(p=>p.path).sort()))return hold("partition");
 return {status:"CORRECTION_SNAPSHOT_MATCH",authority:false,reason:"Ancestry and observed source inventory match. Full tree, semantic review, compiled and hosted qualification are separate."};
}
function tests(s){const cases=[["baseline",x=>{}],["moved candidate",x=>x.live[0].head="0".repeat(40)],["moved member",x=>x.live[1].head="0".repeat(40)],["promoted",x=>x.live[0].draft=false],["moved base",x=>x.live[0].base="0".repeat(40)],["member closed",x=>x.live[1].state="closed"],["wrong merge base",x=>x.comparisons[2].merge_base="0".repeat(40)],["behind",x=>x.comparisons[1].behind=1],["extra correction",x=>x.comparisons[1].paths.push({path:"other",status:"modified"})],["missing path",x=>x.comparisons[0].paths.pop()],["missing blob",x=>x.unchanged.pop()],["changed blob",x=>x.unchanged[0].new="0".repeat(40)],["foreign repository",x=>x.repository="other/repo"]];return cases.map(([name,mut],i)=>{const x=JSON.parse(JSON.stringify(s));mut(x);const got=validate(x,s);if(got.status!==(i===0?"CORRECTION_SNAPSHOT_MATCH":"HOLD")||got.authority!==false)throw Error(name);return {name,status:got.status,authority:got.authority};});}
const result=process.argv[2]?validate(JSON.parse(readFileSync(process.argv[2],"utf8")),reference):{cases:tests(reference)};
console.log(JSON.stringify(result,null,2));
if(result.status==="HOLD")process.exitCode=1;
