from pathlib import Path
import subprocess, hashlib, json, re, datetime, os
os.environ.setdefault("DYLD_FALLBACK_LIBRARY_PATH", "/opt/homebrew/lib")
os.environ.update(RHAPSODY_REQUIRE_VULKAN="1", VK_INSTANCE_LAYERS="VK_LAYER_KHRONOS_validation", VK_LAYER_VALIDATE_SYNC="1")
root=Path(__file__).resolve().parents[5]
report=Path(__file__).resolve().parent
def fingerprint():
 d=hashlib.sha256()
 paths=list((root/'crates/rhapsody').rglob('*.rs'))+list((root/'crates/rhapsody/assets/window_lab').rglob('*'))
 paths += [root/'Cargo.toml',root/'Cargo.lock',root/'crates/rhapsody/Cargo.toml',root/'tools/window-lab/Cargo.toml',root/'tools/window-lab/Cargo.lock']
 for p in sorted(set(paths)):
  if p.is_file(): d.update(str(p.relative_to(root)).encode()+b'\0'+p.read_bytes()+b'\0')
 return d.hexdigest()
checks=[
 ('default-gpu', ['cargo','test','-p','rhapsody','--offline','--lib','--tests','--no-fail-fast','-j','2','--','--test-threads=1','--nocapture']),
 ('no-default-check', ['cargo','check','-p','rhapsody','--offline','--no-default-features','--all-targets','-j','2']),
 ('consumer-targets', ['cargo','check','-p','rhapsody','-p','tg-text','-p','tiny-glade','--offline','--all-features','--all-targets','-j','2']),
 ('consumer-libs-final', ['cargo','check','-p','rhapsody','-p','tg-text','-p','tiny-glade','-p','country-core','--offline','--all-features','--lib','-j','2']),
 ('strict-clippy', ['cargo','clippy','-p','rhapsody','--offline','--all-features','--all-targets','--no-deps','-j','2','--','-D','warnings']),
 ('window-example-tests', ['cargo','test','-p','rhapsody','--offline','--all-features','--example','window_lab','-j','2','--','--nocapture']),
]
m={'recorded_at':datetime.datetime.now(datetime.timezone.utc).isoformat(),'source_fingerprint_scope':'rhapsody Rust sources including nested shader crates, window fixtures, workspace and window manifests/locks','source_sha256_start':fingerprint(),'checks':[]}
for name,cmd in checks:
 print('RUN '+name,flush=True)
 before=fingerprint()
 with (report/(name+'.log')).open('w') as f:
  p=subprocess.run(cmd,cwd=root,stdout=f,stderr=subprocess.STDOUT)
 row={'name':name,'command':cmd,'exit_code':p.returncode,'log':name+'.log','source_sha256_before':before,'source_sha256_after':fingerprint()}
 if name.endswith('gpu') or name.endswith('tests'):
  s=(report/(name+'.log')).read_text(); suites=re.findall(r'test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored',s)
  row['process_results']={'suites':len(suites),'passed':sum(int(x[1]) for x in suites),'failed':sum(int(x[2]) for x in suites),'ignored':sum(int(x[3]) for x in suites),'includes_cpu_tests':True}
 m['checks'].append(row)
 (report/'final-checks.json').write_text(json.dumps(m,ensure_ascii=False,indent=2)+'\n')
 print('DONE '+name+' '+str(p.returncode),flush=True)
m['source_sha256_end']=fingerprint(); m['source_unchanged_during_matrix']=m['source_sha256_start']==m['source_sha256_end'] and all(x['source_sha256_before']==m['source_sha256_start'] and x['source_sha256_after']==m['source_sha256_start'] for x in m['checks'])
(report/'final-checks.json').write_text(json.dumps(m,ensure_ascii=False,indent=2)+'\n')
print(json.dumps({'failed':[x['name'] for x in m['checks'] if x['exit_code']], 'stable':m['source_unchanged_during_matrix']}),flush=True)

raise SystemExit(1 if any(x['exit_code'] for x in m['checks']) else 0)
