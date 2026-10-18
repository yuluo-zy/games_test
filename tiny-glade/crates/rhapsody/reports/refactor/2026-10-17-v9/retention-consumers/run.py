import datetime,hashlib,importlib.util,json,os,pathlib,re,shutil,subprocess,sys
base=pathlib.Path('/Users/yuluo/Desktop/game/games_test/tiny-glade')
out=base/'crates/rhapsody/reports/refactor/2026-10-17-v9/retention-consumers'
out.mkdir(parents=True,exist_ok=True)
spec=importlib.util.spec_from_file_location('checks',base/'crates/rhapsody/tools/refactor_checks.py');checks=importlib.util.module_from_spec(spec);spec.loader.exec_module(checks)
env=os.environ.copy();env.update(CARGO_INCREMENTAL='0',CARGO_TERM_COLOR='never',DYLD_FALLBACK_LIBRARY_PATH='/opt/homebrew/lib',RHAPSODY_REQUIRE_VULKAN='1',VK_INSTANCE_LAYERS='VK_LAYER_KHRONOS_validation',VK_LAYER_VALIDATE_SYNC='1',TG_TEXT_REQUIRE_GPU='1',TG_TEXT_ORIGINAL_ROOT='/Users/yuluo/Desktop/game/Tiny Glade')
manifest={'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'source_start':checks.fingerprint(),'steps':[],'builds':[]}
shutil.copy2(__file__,out/'run.py')
for package,targets in [('tiny-glade',['render_host','core_raster_draw']),('tg-text',['gpu_pipeline'])]:
 before=checks.fingerprint();cmd=['cargo','test','-p',package,'--all-features']
 for target in targets:cmd+=['--test',target]
 cmd+=['--no-run','--message-format=json','--offline','--locked','-j','2']
 print('BUILD '+package,flush=True)
 with (out/(package+'-build.log')).open('w') as log:r=subprocess.run(cmd,cwd=base,env=env,stdout=log,stderr=subprocess.STDOUT)
 after=checks.fingerprint();manifest['builds'].append({'package':package,'command':cmd,'exit_code':r.returncode,'source_before':before,'source_after':after,'source_stable':before==after})
 (out/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
 if r.returncode:print('BUILD FAIL',flush=True);sys.exit(r.returncode)
 for line in (out/(package+'-build.log')).read_text().splitlines():
  try:item=json.loads(line)
  except json.JSONDecodeError:continue
  if item.get('reason')!='compiler-artifact' or not item.get('executable') or item['target']['name'] not in targets:continue
  name=item['target']['name'];binary=pathlib.Path(item['executable']);dest=out/'binaries'/name;dest.parent.mkdir(exist_ok=True);shutil.copy2(binary,dest);before=checks.fingerprint()
  print('RUN '+name,flush=True)
  with (out/(name+'.log')).open('w') as log:r=subprocess.run([str(dest),'--test-threads=1','--nocapture'],cwd=base,env=env,stdout=log,stderr=subprocess.STDOUT)
  output=(out/(name+'.log')).read_text(errors='replace');after=checks.fingerprint()
  record={'step':name,'exit_code':r.returncode,'binary_sha256':hashlib.sha256(dest.read_bytes()).hexdigest(),'source_before':before,'source_after':after,'source_stable':before==after,'validation_errors':len(re.findall(r'Validation Error|SYNC-HAZARD',output)),'skips':re.findall(r'^.*\bSKIP\b.*$',output,re.M)}
  manifest['steps'].append(record);(out/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n');print(json.dumps(record),flush=True)
manifest['source_end']=checks.fingerprint();manifest['finished_utc']=datetime.datetime.now(datetime.timezone.utc).isoformat();(out/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
sys.exit(any(x['exit_code'] or x['validation_errors'] or x['skips'] for x in manifest['steps']))
