import hashlib,json,re
from pathlib import Path
root=Path('D:/game')
re_root=root/'reverse-engineering/tiny-glade'
evidence=re_root/'evidence/theme-static-2026-10-14'
out=evidence/'integration'
packages=['theme-defs','theme-fetcher','theme-server-proto']
# Read fresh at integration time, preserving unrelated work in shared ledgers.
def write_json(path,value):
 path.write_text(json.dumps(value,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
def hash_file(path):return hashlib.sha256(path.read_bytes()).hexdigest()
source_files=[]
for name in packages:
 for path in sorted((root/'tiny-glade/crates'/name).rglob('*')):
  if path.is_file(): source_files.append({'path':path.relative_to(root).as_posix(),'sha256':hash_file(path)})
test_log=(out/'test-delivery.log').read_text(encoding='utf-8-sig')
counts=re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored',test_log)
assert sum(int(x[0]) for x in counts)==26
assert all(int(x[1])==int(x[2])==0 for x in counts)
assert 'Finished' in (out/'clippy-delivery.log').read_text(encoding='utf-8-sig')
lock_before=(out/'gate-lock-before.txt').read_text(encoding='utf-8-sig').strip()
lock_after=(out/'gate-lock-after.txt').read_text(encoding='utf-8-sig').strip()
assert lock_before==lock_after==hash_file(root/'tiny-glade/Cargo.lock')
audit=json.loads((out/'dependency-audit.json').read_text(encoding='utf-8'))
assert not audit['mismatches'] and audit['lock_sha256']==lock_after
reopen=json.loads((out/'consumer-reopen.json').read_text(encoding='utf-8'))
assert reopen['ok'] and reopen['identity']['sha256']=='cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323'
summary={
 'schema_version':1,'date':'2026-10-14','baseline':'B','packages':packages,
 'source_root':'tiny-glade/crates','historical_shared_foundation_mirror_modified':False,
 'status':'observed_B_library_surface_statically_completed_and_candidate_tests_passed',
 'known_unimplemented_observed_B_library_algorithms':[],
 'full_original_behavior_equivalence_accepted':False,'original_game_consumer_integrated':False,
 'real_theme_service_requests':0,'original_game_execution':False,'original_pipeline_oracle_execution':False,
 'report':'evidence/theme-static-2026-10-14/REPORT.md',
 'validation':{'tests_passed':26,'tests_failed':0,'tests_ignored':0,'offline_cross_crate_tests':3,
  'clippy_all_targets_warnings_denied':True,'fmt_check_passed':True,'offline':True,'locked':True,
  'test_log':'integration/test-delivery.log','clippy_log':'integration/clippy-delivery.log',
  'lock_sha256':lock_after,'lock_unchanged_during_delivery_gate':True},
 'dependencies':{'original_lock':'ljxsj_92385/Tiny Glade/build-info/Cargo.lock','public_packages':audit['public_packages'],
  'identity_mismatches':0,'audit':'integration/dependency-audit.json'},
 'static_evidence':{'schema':'schema/REPORT.md','fetcher':'fetcher/REPORT.md',
  'consumer':'integration/consumer-prepare.json','consumer_strings':'integration/consumer-strings.json',
  'consumer_worker':'theme-consumer-20261014','consumer_overlay_saved_and_reopened':True,
  'original_database_modified':False,'historical_mac_overlays_synced':False},
 'remaining_boundaries':[
  'Unemitted author field/variant names and unit-error Display texts remain reconstruction names.',
  'Complete original HTTP/CAS/ZIP runtime oracle and all OS failure combinations have not been accepted.',
  'Real tiny_glade plugin, language/session/history/credits consumption and visuals are outside this three-crate delivery.',
  'Server-side API/unemitted sources and A/C behavior are not covered.'
 ],'source_files':source_files}
write_json(out/'summary.json',summary)
write_json(re_root/'tracking/theme-static-recovery.json',summary)
p=re_root/'tracking/project.json'
j=json.loads(p.read_text(encoding='utf-8'))
j['theme_static_recovery']={k:v for k,v in summary.items() if k not in ['source_files','static_evidence']}
j['theme_static_recovery']['summary']='evidence/theme-static-2026-10-14/integration/summary.json'
write_json(p,j)
scopes={
 'theme-defs':'B 已发射 PathBuf/两个 SystemTime/错误结果身份合同完整静态盘点；作者精确名称和 unit Display 仍未知。',
 'theme-fetcher':'B 已定位 HTTP/CAS/ZIP/活动发布库内逻辑静态补全；恢复处理顺序、分阶段时钟、mtime/ZIP边界与21处原日志。',
 'theme-server-proto':'B ActiveEvent/EventsResponse 原 serde 字段及对象/序列/required/duplicate/unknown 分支合同完整复核。'}
for filename in ['module-progress-overrides.json','module-progress.json']:
 p=re_root/'tracking'/filename
 j=json.loads(p.read_text(encoding='utf-8'))
 for row in j.get('modules',[]):
  for name in packages:
   if row.get('module')!='crates/'+name+'/src':continue
   row['recovered_scope']=scopes[name]
   row['verification_summary']='2026-10-14 实际 tiny-glade 三库 locked/offline 26测试、all-targets Clippy -D warnings、fmt通过；静态范围与完整原入口验收分开。'
   row['next_action']='原完整入口oracle与库外游戏主题consumer接线；A/C另验。'
   row['current_recovery_source_root']='tiny-glade/crates/'+name
   row['current_recovery_ledger']='tracking/theme-static-recovery.json'
   row['library_implementation_complete_within_observed_B_surface']=True
   row['full_module_accepted']=False
   for entry in ['evidence/theme-static-2026-10-14/REPORT.md','tracking/theme-static-recovery.json']:
    if entry not in row.setdefault('evidence',[]):row['evidence'].append(entry)
 write_json(p,j)
p=re_root/'tracking/foundation-business-functions.json'
j=json.loads(p.read_text(encoding='utf-8'))
for row in j.get('packages',j.get('crates',j.get('libraries',[]))):
 if row.get('name') not in packages:continue
 row['current_recovery_source_root']='tiny-glade/crates/'+row['name']
 row['library_implementation_complete_within_observed_B_surface']=True
 row['current_recovery_ledger']='tracking/theme-static-recovery.json'
 row['full_original_functionality_accepted']=False
 for feature in row.get('features',[]):
  feature['library_code_missing_within_observed_B_surface']=False
  feature['remaining']='完整原入口验收与库外游戏consumer；未发射作者文本及A/C未覆盖。'
  feature['implementation_task_ready']=False
  feature.setdefault('evidence',[]).append('evidence/theme-static-2026-10-14/REPORT.md')
write_json(p,j)
print(json.dumps({'tests':26,'public_dependencies':audit['public_packages'],'source_files':len(source_files),'lock_sha256':lock_after}))
