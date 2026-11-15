import ast,hashlib,json,os,signal,subprocess,time
from pathlib import Path
supervisor=Path('/workspace/lexlean-evidence/supervise-bootstrap-export.py');assert hashlib.file_digest(supervisor.open('rb'),'sha256').hexdigest()=='6550c6eec19ef2d7ff929790712e07d100ffdb4050df4bea9f99a3dc2ed6d36f'
ns=dict(Path=Path,os=os,signal=signal,time=time);tree=ast.parse(supervisor.read_text());exec(compile(ast.Module(body=[n for n in tree.body if isinstance(n,ast.FunctionDef)],type_ignores=[]),str(supervisor),'exec'),ns)
receipt=Path('/workspace/lexlean-evidence/bridge-final-conformance-build.json')
for _ in range(650):
 if receipt.exists():break
 time.sleep(1)
else:raise RuntimeError('build receipt missing')
b=json.loads(receipt.read_text());assert b['exit']==0 and b['head']=='cf5b2bc8bab4233acd3cca04a26e14e3b058aaa7',b
root=Path('/workspace/lexlean-rule30-bridge');head=subprocess.check_output(['git','-C',str(root),'rev-parse','HEAD'],text=True).strip();assert head==b['head'];assert not subprocess.check_output(['git','-C',str(root),'status','--porcelain'])
exe=Path('/workspace/lexlean-bridge-target/debug/deps/conformance-f91038edb0165b7a');tests=['conformance_vr_29','conformance_sp_17','conformance_rp_10','conformance_rp_11','conformance_pk_05','conformance_ns_01'];argv=[str(exe),'--exact',*tests,'--test-threads=1','--nocapture']
env=dict(os.environ);env.update(CARGO_HOME='/workspace/toolchains/cargo',RUSTUP_HOME='/workspace/toolchains/rustup',ELAN_HOME='/workspace/toolchains/elan');env['PATH']='/workspace/toolchains/cargo/bin:/workspace/toolchains/elan/bin:'+env['PATH'];log=Path('/workspace/lexlean-evidence/bridge-focused-current.log');assert not log.exists();start=time.monotonic();peak=0;reason=None
with log.open('wb') as out:
 p=subprocess.Popen(argv,cwd=root,env=env,stdout=out,stderr=subprocess.STDOUT,start_new_session=True)
 while p.poll() is None:
  peak=max(peak,sum(r['rss'] for r in ns['owned_session'](p.pid).values()))
  if time.monotonic()-start>600:reason='600-second focused test bound'
  elif peak>9*1024*1024:reason='9-GiB focused session RSS bound'
  elif log.stat().st_size>16*1024*1024:reason='16-MiB output bound'
  if reason:ns['terminate_session'](p.pid);break
  time.sleep(.25)
 code=p.wait()
 if ns['owned_session'](p.pid):reason=reason or 'remaining session children';ns['terminate_session'](p.pid)
 if log.stat().st_size>16*1024*1024:reason='16-MiB output bound'
text=log.read_text();passed=all('test '+n+' ... ok' in text for n in tests);record={'head':head,'argv':argv,'exit':code,'reason':reason,'all_six_named_tests_passed':passed,'seconds':time.monotonic()-start,'peak_session_rss_kib':peak,'executable_sha256':hashlib.file_digest(exe.open('rb'),'sha256').hexdigest(),'log_sha256':hashlib.file_digest(log.open('rb'),'sha256').hexdigest(),'compiler_accepted':False};Path('/workspace/lexlean-evidence/bridge-focused-current.json').write_text(json.dumps(record,indent=2)+'\n');print(json.dumps(record));raise SystemExit(0 if code==0 and not reason and passed else 1)
