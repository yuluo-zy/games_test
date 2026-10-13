from pathlib import Path
import json,hashlib,tomllib,re
root=Path('D:/game/reverse-engineering/tiny-glade');out=Path(__file__).parent
def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()
contract=out/'CONTRACT.md';source=contract.read_text(encoding='utf8')
start=source.index('## 依赖版本缺口');end=source.index('## Utils 所有权交接与未完成')
source=source[:start]+'''## 依赖版本核对

tabloid 直接使用原 parking_lot=0.12.1；源码只调用 dependency API，未重写Mutex、RwLock、Arc、Vec。首次测试曾使用较新transitives，父owner已收到通知。收尾时共享锁已改变，最新 `tabloid-test.log` 明确编译原lock_api0.4.9、parking_lot_core0.9.3、smallvec1.13.2、scopeguard1.1.0、cfg-if1.0.0；同一三个合同测试再次通过。严格Clippy也已在该锁下复验通过，owning read guard表面确实兼容原lock_api0.4.9。机器报告逐dependency比较原版本/source/checksum，而不从direct pins推定完整workspace锁验收。

'''+source[end:];contract.write_text(source,encoding='utf8')
orig=tomllib.loads((root/'tracking/original-Cargo.lock').read_text(encoding='utf8'))['package']
candidate=tomllib.loads((root/'reconstruction/shared-foundation/Cargo.lock').read_text(encoding='utf8'))['package']
deps=[]
for name in ['parking_lot','lock_api','parking_lot_core','scopeguard','smallvec','cfg-if','autocfg','windows-sys','windows_x86_64_msvc']:
 for p in [p for p in candidate if p['name']==name]:
  matches=[q for q in orig if q['name']==name and q['version']==p['version']]
  deps.append({'name':name,'version':p['version'],'source':p.get('source'),'checksum':p.get('checksum'),'matches_original':any(all(q.get(k)==p.get(k)for k in ['version','source','checksum'])for q in matches)})
utils=root/'reconstruction/shared-foundation/crates/utils/src'
tabloid=root/'reconstruction/shared-foundation/crates/tabloid'
files=[tabloid/'Cargo.toml',tabloid/'src/lib.rs']
report={
 'schema':'shared-foundation-delivery-v1','owner':'foundation_utils_tabloid','date':'2026-10-13','baseline':'B',
 'status':'tabloid_implemented_contract_tested_original_partial;utils_handoff_unverified',
 'source_original_claimed':False,'modified_global_progress':False,
 'original_module':'crates/tabloid/src/lib.rs',
 'files':[{'path':str(p.relative_to(root)).replace('\\','/'),'sha256':sha(p)}for p in files],
 'original_functions':[
  {'name':'tabloid::Publisher<T>::publish','addresses':['0x141cf6500','0x1425db920','0x141453350','0x141453c00'],'implementation':'Publisher::publish','verification':'original_instruction_policy_review;3_candidate_contract_tests'},
  {'name':'tabloid::Publisher<T>::subscribe','addresses':['0x141247b10','0x141454290','0x141454410'],'implementation':'Publisher::subscribe','verification':'original_instruction_policy_review;candidate_contract_tests'},
  {'name':'tabloid::Subscriber<T>::take','addresses':['0x141247c90','0x1412929e0','0x141454590'],'implementation':'Subscriber::take','verification':'original_instruction_policy_review;candidate_contract_tests'},
  {'name':'private publication retain predicate','addresses':['0x141bb03f0'],'implementation':'Publisher::publish retain closure','verification':'original_instruction_policy_review'}],
 'dependencies':deps,
 'verification':{'test':'tabloid-test.log','test_count':3,'test_failed':0,'strict_clippy':'tabloid-clippy.log','native_full_crate_differential':False,'actual_game_consumer_integration':False,'full_workspace_validation':False},
 'gaps':['real DirtyTiles/ColliderDiff producers not wired','original Rust trait bounds/full API/ABI not established','candidate owning guard safety surface differs in representation; raw ABI intentionally not reconstructed','Default/new/Clone public surface not source-text verified','original full-runtime differential not executed'],
 'utils_handoff':{'path':str(utils),'ownership_released':True,'lib_owner':'parent','files':[{'path':p.name,'sha256':sha(p),'functions':re.findall(r'\b(?:pub\s+)?fn\s+(\w+)\s*[<(]',p.read_text(encoding='utf8'))}for p in sorted(utils.glob('*.rs'))if p.name!='lib.rs'],'validation':'native fixtures generated and embedded for six families; actual Rust fixture tests not run by this agent','remaining':'CONTRACT.md'},
 'evidence':{'raw_binary_sha256':'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03','raw_manifest':'native-manifest.json','PDB':'B-symbols.json','persistent_worker':'foundation-utils-tabloid','persistent_query':'selective-persistent.json','contract':'CONTRACT.md'},
 'cargo_lock_at_validation':{'path':'reconstruction/shared-foundation/Cargo.lock','sha256':sha(root/'reconstruction/shared-foundation/Cargo.lock')},
}
(out/'delivery.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf8')
print({'tabloid_sources':len(files),'utils_handoff_files':len(report['utils_handoff']['files']),'tabloid_dependencies_original':all(d['matches_original']for d in deps)})
