import datetime, hashlib, importlib.util, json, os, re, shutil, subprocess
from pathlib import Path
root=Path('/Users/yuluo/Desktop/game/games_test/tiny-glade')
out=root/'crates/rhapsody/reports/refactor/2026-10-17-v9/final-short-followups'
out.mkdir(parents=True,exist_ok=True)
spec=importlib.util.spec_from_file_location('checks',root/'crates/rhapsody/tools/refactor_checks.py'); checks=importlib.util.module_from_spec(spec); spec.loader.exec_module(checks)
common=['--offline','--locked','-j','2']
cases=[('pipeline-cpu',['cargo','test','-p','rhapsody','--all-features','--lib']+common+['pipeline::tests::','--','--test-threads=1','--nocapture']),('tracked-cpu',['cargo','test','-p','rhapsody','--all-features','--lib']+common+['resource::tracked::tests::standalone_cell_capacity_follows_the_last_recording_owner','--','--exact','--test-threads=1','--nocapture']),('gpu-timing',['cargo','test','-p','rhapsody','--all-features','--test','gpu_timing']+common+['--','--test-threads=1','--nocapture']),('pipeline-cache',['cargo','test','-p','rhapsody','--all-features','--test','pipeline_cache']+common+['--','--test-threads=1','--nocapture']),('window-cpu',['cargo','test','-p','rhapsody','--all-features','--example','window_lab']+common+['--','--test-threads=1','--nocapture'])]
env=os.environ.copy(); env.update(CARGO_INCREMENTAL='0',CARGO_TERM_COLOR='never',DYLD_FALLBACK_LIBRARY_PATH='/opt/homebrew/lib',RHAPSODY_REQUIRE_VULKAN='1',VK_INSTANCE_LAYERS='VK_LAYER_KHRONOS_validation',VK_LAYER_VALIDATE_SYNC='1')
m={'source_before':checks.fingerprint(),'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'checks':[],'scope':'Short domain followups. Unregistered range-state/oracle drafts may change; no production consumers are changed during these checks. Not final acceptance.'}
shutil.copy2(__file__,out/'run.py')
for name,command in cases:
 print('RUN',name,flush=True)
 log=out/(name+'.log')
 with log.open('w') as f: code=subprocess.run(command,cwd=root,env=env,stdout=f,stderr=subprocess.STDOUT,timeout=600).returncode
 s=log.read_text(errors='replace'); binaries=[]
 for b in dict.fromkeys(re.findall(r'^\s*Running .+? \(([^)]+)\)\s*$',s,re.M)):
  p=Path(b); p=p if p.is_absolute() else root/p
  if p.is_file():
   d=out/'binaries'/name/p.name; d.parent.mkdir(parents=True,exist_ok=True); shutil.copy2(p,d); binaries.append({'path':str(d.relative_to(out)),'sha256':hashlib.sha256(d.read_bytes()).hexdigest()})
 errors=len(re.findall(r'Validation Error|SYNC-HAZARD|VUID[^\n]*ERROR',s,re.I)); skips=re.findall(r'^.*\bSKIP\b.*$',s,re.M)
 totals=[sum(int(r[i]) for r in re.findall(r'test result: \w+\. (\d+) passed; (\d+) failed; (\d+) ignored;',s)) for i in range(3)]
 status='FAIL' if code or errors else 'BLOCKED' if skips else 'PASS'
 m['checks'].append(dict(name=name,command=command,exit_code=code,validation_errors=errors,skips=skips,status=status,totals=totals,binaries=binaries,log=log.name))
 (out/'checks.json').write_text(json.dumps(m,ensure_ascii=False,indent=2)+'\n'); print('DONE',name,status,totals,flush=True)
 if status!='PASS': break
m['source_after']=checks.fingerprint();m['source_stable']=m['source_before']==m['source_after'];(out/'checks.json').write_text(json.dumps(m,ensure_ascii=False,indent=2)+'\n')
raise SystemExit(0 if len(m['checks'])==len(cases) and all(c['status']=='PASS' for c in m['checks']) else 1)
