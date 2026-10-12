"""统一复验候选工程，记录真实执行与输入快照；不把测试通过扩展为原作全等价。"""
from pathlib import Path
import datetime, hashlib, json, os, subprocess, sys, tomllib
ROOT=Path(__file__).resolve().parents[1]
LOG=ROOT/'evidence/functional-run-logs'
GPU_STATUS={'shader_production_gpu':'candidate_shader_production_gpu_passed',
 'raster_gpu':'candidate_raster_gpu_passed','allocator_gpu':'candidate_allocator_gpu_passed'}
def snapshot():
 paths=[ROOT/'Cargo.toml',ROOT/'Cargo.lock']
 for folder in ['src','tests','examples','scripts']:
  paths += [p for p in (ROOT/folder).rglob('*') if p.is_file() and p.suffix in {'.rs','.py','.json'}]
 return {p.relative_to(ROOT).as_posix():hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(paths)}
def main():
 LOG.mkdir(exist_ok=True)
 before=snapshot(); runs=[]
 env=dict(os.environ,PYTHONIOENCODING='utf-8')
 def run(label,command,timeout=300):
  print('正在验收：'+label,flush=True)
  started=datetime.datetime.now(datetime.timezone.utc).isoformat()
  try:
   result=subprocess.run(command,cwd=ROOT,env=env,text=True,encoding='utf-8',errors='replace',capture_output=True,timeout=timeout)
   code=result.returncode; output=result.stdout; error=result.stderr
  except subprocess.TimeoutExpired as exc:
   code=-1; output=str(exc.stdout or ''); error='超过单项验收时限：'+str(exc)
  (LOG/(label+'.log')).write_text(output+'\n'+error,encoding='utf-8')
  item={'label':label,'command':command,'started_utc':started,'exit_code':code,'log':(LOG/(label+'.log')).relative_to(ROOT).as_posix()}
  if label.startswith('gpu-') and code==0:
   try:
    item['actual_output']=json.loads(output)
    if not isinstance(item['actual_output'],dict) or item['actual_output'].get('status')!=GPU_STATUS.get(label.removeprefix('gpu-'),'passed'):
     item['invalid_success_status']=True; code=1; item['exit_code']=code
   except json.JSONDecodeError: item['json_output_error']=True; code=1; item['exit_code']=code
  runs.append(item)
  print(label+': '+('通过' if code==0 else '失败'),flush=True)
  return code==0
 original=tomllib.loads((ROOT.parents[1]/'tracking/original-Cargo.lock').read_text(encoding='utf-8'))
 current=tomllib.loads((ROOT/'Cargo.lock').read_text(encoding='utf-8'))
 identities={(p['name'],p['version'],p.get('source'),p.get('checksum')) for p in original['package']}
 deps=[p for p in current['package'] if 'source' in p]
 unexpected=[p for p in deps if (p['name'],p['version'],p.get('source'),p.get('checksum')) not in identities]
 ok=not unexpected
 for label,args in [('format',['fmt','--all','--','--check']),('tests',['test','--offline','--locked','--all-targets']),('clippy',['clippy','--offline','--locked','--all-targets','--','-D','warnings']),('build',['build','--offline','--locked','--examples'])]:
  if not run(label,['cargo',*args]): ok=False; break
 if ok:
  for name in ['raw_resource_smoke','vulkan_smoke','device_features_gpu','device_policy_gpu','shader_production_gpu','raster_gpu','attachment_gpu','temporal_gpu','queue_graph_gpu','swapchain_gpu','crash_gpu','allocator_gpu','pass_bindings_gpu','frame_queue_gpu']:
   if not run('gpu-'+name,[str(ROOT/'target/debug/examples'/(name+'.exe'))],90): ok=False
  for name,args in [('compare_resource.py',[]),('compare_pass_api.py',[]),('compare_chain.py',[]),('compare_queue_graph.py',[]),('generate_formats.py',['--validate']),('validate_shader_allocator.py',['--no-build']),('verify_graph_gpu.py',[]),('verify_raw_resources.py',[])]:
   if not run('native-'+Path(name).stem,[sys.executable,str(ROOT/'scripts'/name),*args]): ok=False
 after=snapshot(); changed=[p for p in before.keys()|after.keys() if before.get(p)!=after.get(p)]
 if changed: ok=False
 report={'timestamp_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'status':'passed_current_candidate_functional_scope' if ok else 'failed_or_inputs_changed','original_complete_equivalence':False,
  'dependency_check':{'external_packages':len(deps),'matched_original_lock':not unexpected,'unexpected':unexpected},'checks':runs,'input_hashes':after,'changed_during_run':changed,
  'limits':['验收只证明这些候选场景；不证明原全帧、所有GPU或原公共分叉行为。','原入口fixture/hook边界见各子系统报告；未执行完整原游戏对照。','真实GPU测试串行运行，辅助队列无本机硬件覆盖；硬件ray tracing不计入该原作构建的功能验收。']}
 (ROOT/'evidence/functional-project-validation.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
 subprocess.run([sys.executable,str(ROOT/'scripts/audit_progress.py')],check=True,env=env)
 print(json.dumps({'status':report['status'],'checks':len(runs),'changed_during_run':changed,'unexpected_dependencies':len(unexpected)},ensure_ascii=False),flush=True)
 if not ok: raise SystemExit(1)
if __name__=='__main__': main()
