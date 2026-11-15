"""Bound one full public bootstrap export without changing project child limits."""
import hashlib,importlib.util,json,os,signal,subprocess,sys,time
from pathlib import Path
sys.dont_write_bytecode=True
spec=importlib.util.spec_from_file_location('bounded','/workspace/lexlean-composed-qualification/diagnostics/composed-bootstrap-20261114/run.py');m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
exe=Path(sys.argv[1]).resolve();output=Path(sys.argv[2]).resolve();assert not output.exists()
log=output.with_suffix('.supervisor.log');receipt=output.with_suffix('.supervisor.json');assert not log.exists() and not receipt.exists() and not output.with_suffix('.supervisor-start.json').exists()
env={'HOME':os.environ['HOME'],'PATH':'/workspace/toolchains/cargo/bin:/workspace/toolchains/elan/bin:/usr/local/bin:/usr/bin:/bin','LANG':'C.UTF-8','LC_ALL':'C.UTF-8','ELAN_HOME':'/workspace/toolchains/elan','CARGO_HOME':'/workspace/toolchains/cargo','RUSTUP_HOME':'/workspace/toolchains/rustup'}
def owned_session(session):
 rows={}
 for path in Path('/proc').glob('[0-9]*/stat'):
  try:
   fields=path.read_text().rsplit(') ',1)[1].split()
   if int(fields[3])!=session or fields[0]=='Z':continue
   status=dict(line.split(':',1) for line in path.with_name('status').read_text().splitlines() if ':' in line)
   rows[int(path.parent.name)]={'group':int(fields[2]),'state':fields[0],'rss':int(status.get('VmRSS','0 kB').split()[0])}
  except (FileNotFoundError,ProcessLookupError):continue
 return rows

def terminate_session(session):
 # The helper owns a new session. Its Lean children own other groups in that
 # same session, so killing only the helper group would orphan proof work.
 for _ in range(100):
  rows=owned_session(session)
  if not rows:break
  for pid in rows:
   try:os.kill(pid,signal.SIGSTOP)
   except ProcessLookupError:pass
  stopped=owned_session(session)
  if all(row['state'] in ('T','t') for row in stopped.values()):break
  time.sleep(.01)
 for group in {row['group'] for row in owned_session(session).values()}:
  assert group!=os.getpgrp()
  try:os.killpg(group,signal.SIGKILL)
  except ProcessLookupError:pass
 for _ in range(100):
  rows=owned_session(session)
  if not rows:return
  for pid in rows:
   try:os.kill(pid,signal.SIGKILL)
   except ProcessLookupError:pass
  time.sleep(.01)
 raise RuntimeError(f'owned session {session} still has live processes: {list(owned_session(session))}')

argv=[str(exe),str(output)];started=time.monotonic();peak=0;reason=None
m.save(output.with_suffix('.supervisor-start.json'),{'argv':argv,'executable_sha256':m.digest(exe),'environment':env,'overall_seconds':3600,'tree_rss_kib':9*1024*1024,'output_bytes':16*1024*1024,'project_child_limits_unchanged':True,'accepted':False})
with log.open('wb') as stream:
 proc=subprocess.Popen(argv,env=env,stdout=stream,stderr=subprocess.STDOUT,start_new_session=True)
 while proc.poll() is None:
  peak=max(peak,sum(row['rss'] for row in owned_session(proc.pid).values()))
  if time.monotonic()-started>3600:reason='3600-second overall export supervision limit'
  elif peak>9*1024*1024:reason='9-GiB export process-tree RSS limit'
  elif log.stat().st_size>16*1024*1024:reason='16-MiB supervisor output limit'
  if reason:terminate_session(proc.pid);break
  time.sleep(.25)
 code=proc.wait()
 if owned_session(proc.pid):
  reason=reason or 'live owned session processes after helper exit'
  terminate_session(proc.pid)
 if log.stat().st_size>16*1024*1024:reason='16-MiB supervisor output limit'
m.save(receipt,{'argv':argv,'exit':code,'reason':reason,'seconds':time.monotonic()-started,'peak_tree_rss_kib':peak,'log_sha256':m.digest(log),'executable_sha256':m.digest(exe),'compiler_accepted':False})
print(receipt.read_text());sys.exit(1 if code or reason else 0)
