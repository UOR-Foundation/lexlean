// #603 advisory replay, no acceptance authority.
import {readFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
const anchors=[
  [
    "ci-b1-630-terminal-20261126.json",
    "84307c7fa081415825f8be8a337d01bf04b2330b"
  ],
  [
    "ci-b1-630-postmerge-20261126.json",
    "d5d370fd2c087e89024f6b20b34047cff972ba22"
  ],
  [
    "ci-b1-baseline-freeze-20261125.json",
    "1ea15ebce8319ed1fd99ce37556ae4c7eddf5527"
  ]
];
function replay(docs){
 const [a,b,c]=docs,need=(v,m)=>{if(!v)throw Error(m)},t=s=>{const v=Date.parse(s)/1000;need(Number.isFinite(v),"invalid time");return v},sum=(xs,f)=>xs.reduce((n,x)=>n+f(x),0),same=(a,b)=>JSON.stringify(a)===JSON.stringify(b),ids=new Set();
 const h="b5bfe17ca44ecca3f6bf95798c7788e6254c790a",m="11d665cf4a27415dc4d34202d61984dfb841d353",tree="0fd20b0aa6fe55fffb8b445472161c87c4c88738";
 need(a.id===45757936558&&a.attempt===1&&a.run_latest_attempt===1&&a.head===h&&a.tree===tree&&a.landed===m&&a.base==="903797a784df34f0bc0c2910e1f68ad7163b1aa6"&&a.run_status==="completed"&&a.run_conclusion==="success","PR identity");
 need(b.head===m&&b.tree===tree&&b.qualified_head===h&&b.qualified_ci===a.id&&b.qualified_repro===a.repro.id,"postmerge binding");
 need(a.repro.id===45757936468&&a.repro.head===h&&a.repro.workflow===".github/workflows/reproducibility.yml"&&a.repro.status==="completed"&&a.repro.conclusion==="success","repro identity");
 function tally(rows,n,head,allowSkip){
  need(rows.length===n,"row count");let allocated=0,seconds=0,queue=0,skips=0;
  for(const j of rows){
   need(Number.isSafeInteger(j.id)&&j.id>0&&!ids.has(j.id)&&j.attempt===1&&j.status==="completed"&&typeof j.name==="string"&&j.name.length>0,"job identity");ids.add(j.id);
   if(head)need(j.head===head,"job head");
   if(j.conclusion==="skipped"){need(allowSkip&&j.runner===null,"allocated or unexpected skip");skips++;continue}
   need(j.conclusion==="success"&&Number.isSafeInteger(j.runner)&&j.runner>0&&j.labels.length>0,"invalid allocated execution");
   const d=t(j.completed)-t(j.started),q=t(j.started)-t(j.created);need(d>=0&&q>=0,"negative interval");allocated++;seconds+=d;queue+=q;
  }return {allocated,seconds,queue,skips};
 }
 const baseline=c.runs.find(x=>x.id===45659553448);need(baseline&&baseline.raw.length===70&&a.provider_rows===70&&same(a.raw.map(x=>x.name).sort(),baseline.raw.map(x=>x.name).sort())&&new Set(a.raw.map(x=>x.name)).size===70,"full roster");
 const p=tally(a.raw,70,null,false),re=tally(a.repro.raw,1,null,false);need(a.repro.raw[0].name==="repro","repro role");
 const ex=a.raw.find(x=>x.name==="bootstrap export"),legs=a.raw.filter(x=>/^just vv \(bdd-bootstrap-/.test(x.name));need(ex&&legs.length===28&&legs.every(x=>t(x.created)>=t(ex.completed)&&t(x.started)>=t(ex.completed)),"bootstrap ordering");
 const computed={allocated_seconds:p.seconds,bootstrap_seconds:sum(legs,x=>t(x.completed)-t(x.started)),queue_seconds:sum(legs,x=>t(x.started)-t(x.created)),pre_export_job_seconds:0,post_export_max_start_delay_seconds:Math.max(...legs.map(x=>t(x.started)-t(ex.completed))),gate_elapsed_seconds:Math.max(...a.raw.map(x=>t(x.completed)))-t(a.run_created),repro_seconds:re.seconds};
 need(Object.keys(computed).every(k=>computed[k]===a.metrics[k]),"PR metric mismatch");
 const expected=[[45780833433,19,"plan",18],[45780833444,1,"repro",0],[45780833387,1,"tag a cut",0]];
 need(b.runs.length===3&&new Set(b.runs.map(x=>x.id)).size===3,"push workflow roster");
 const push=expected.map(([id,n,name,skips])=>{const x=b.runs.find(x=>x.id===id);need(x&&x.total===n,"push count");const s=tally(x.raw,n,m,true);need(s.allocated===1&&s.skips===skips&&x.skips===skips&&x.raw.find(y=>y.runner>0).name===name&&s.seconds===x.allocated,"push role or allocation");return {id,...s}});
 need(b.reuse_log==="CI reuse: run 45757936558: complete gate accepted for "+h&&b.tag_log==="the workspace version is 0.4.0 on both sides: not a cut","log receipt");
 const total=sum(push,x=>x.seconds);need(total===b.total_allocated_seconds,"push total");
 return {scope:"Pinned sanitized receipt replay only; PR job rows omit individual head, provider authenticity and actual merge ancestry verified separately. Not a billing or causal outcome claim.",pr:{...Object.fromEntries(Object.entries(computed).filter(([k])=>k!=="queue_seconds")),bootstrap_queue_seconds:computed.queue_seconds,whole_run_queue_seconds:p.queue},push,totalPushSeconds:total,combinedSeconds:p.seconds+re.seconds+total,distinctRows:ids.size};
}
const docs=anchors.map(([name,blob])=>{const bytes=readFileSync(new URL('../'+name,import.meta.url));const hash=createHash('sha1').update('blob '+bytes.length+'\0').update(bytes).digest('hex');if(hash!==blob)throw Error('blob mismatch '+name);return JSON.parse(bytes)});
const tests=[["missing PR row",x=>x[0].raw.pop()],
["foreign PR identity",x=>x[0].head="a".repeat(40)],
["foreign push tree",x=>x[1].tree="a".repeat(40)],
["duplicate ID",x=>x[1].runs[1].raw[0].id=x[0].raw[0].id],
["wrong attempt",x=>x[0].raw[0].attempt=2],
["early bootstrap",x=>{const j=x[0].raw.find(y=>/^just vv \(bdd-bootstrap-/.test(y.name));j.created=j.started="2026-11-26T04:52:28Z"}],
["missing native obligation",x=>x[0].raw.find(y=>/aarch64-unknown/.test(y.name)).name="substitute"],
["allocated skip",x=>x[1].runs[0].raw.find(y=>y.conclusion==="skipped").runner=123],
["foreign push job",x=>x[1].runs[2].raw[0].head="a".repeat(40)],
["failed repro",x=>x[0].repro.raw[0].conclusion="failure"],
["false total",x=>x[1].total_allocated_seconds++],
["incorrect reuse proof",x=>x[1].reuse_log="CI reuse: unrelated"],
["missing workflow",x=>x[1].runs.pop()]];
const result=replay(docs);result.negativeControls=tests.map(([name,mut])=>{const x=JSON.parse(JSON.stringify(docs));mut(x);try{replay(x)}catch(e){return {name,refused:e.message}}throw Error('unexpected acceptance '+name)});console.log(JSON.stringify(result,null,2));
