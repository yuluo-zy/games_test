import json,hashlib,subprocess
from datetime import datetime,timezone
from pathlib import Path
OUT=Path(__file__).resolve().parent; PROJECT=OUT.parent.parent
def digest(p): return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def write(p,v): p.write_text(json.dumps(v,ensure_ascii=False,indent=2)+'\n')
def run(argv):
 r=subprocess.run(argv,capture_output=True,text=True)
 if r.returncode: raise RuntimeError(r.stdout+r.stderr)
 return r.stdout
m=json.loads((OUT/'mapping.json').read_text()); receipts=json.loads((OUT/'deliveries.json').read_text()); claims=json.loads((OUT/'claims.json').read_text())
assert (OUT/'final-review.md').is_file()
for r in receipts:
 assert digest(r['manifest'])==r['manifest_sha256']
 run(['python3',str(PROJECT/'skeleton/workflow-design/tiny-glade-recovery/scripts/workflow.py'),'check','--manifest',r['manifest']])
for p,h in json.loads((OUT/'dependency-closure.json').read_text())['files_sha256'].items(): assert digest(p)==h,p
releases=[]
for c in claims:
 current=json.loads((PROJECT/'skeleton/task-index/claims'/f"{c['task_id']}.json").read_text()); assert current['owner']=='abc-building-35'
 releases.append(json.loads(run(['python3',str(PROJECT/'skeleton/_tools/task_queue.py'),'release',c['task_id'],'--agent','abc-building-35','--reason','35组候选已冻结并完成限定独立源码审查；验证暂停、未生产整合，见 evidence/abc-building-35-2026-10-12/README.md'])))
write(OUT/'releases.json',releases)
summary={'completed_at_utc':datetime.now(timezone.utc).isoformat(),'logical_function_candidates':35,'original_source_file_groups':21,'version_mappings':105,'unique_native_ranges_per_version':33,'priority_counts':{'geometry':14,'building_interfaces':17,'history_interfaces':4},'status':'implemented_unverified','formatting_completed':True,'frozen_reference_integrity_checked':True,'build_run':False,'tests_run':False,'native_target_executed':False,'independent_review':True,'review':str(OUT/'final-review.md'),'review_sha256':digest(OUT/'final-review.md'),'production_integration':False,'signature_and_abi_gaps_recorded':True,'overview':str(OUT/'README.md'),'mapping':str(OUT/'mapping.json'),'delivery_manifests':receipts,'claims_released':len(releases)}
write(OUT/'summary.json',summary); write(PROJECT/'tracking/abc-building-35-candidates.json',{**summary,'functions':m})
note='''\n\n## 2026-10-12 按优先级恢复35组候选\n\n用户授权按优先级开始逆向还原。首批35组（几何14、建筑接口17、历史接口4）基于A原EXE/B原RNE/C原EXE及对应PDB取证，105个版本映射、每版33个不同原范围。已写21个合法文件组原路径的共享Rust候选，中文注释与ABC对应齐全；复用上批类型/函数，不改生产Cargo/lib。见 [交付总览](evidence/abc-building-35-2026-10-12/README.md) 与 [机器台账](tracking/abc-building-35-candidates.json)。\n\n原指令与限定真实消费者补证：动态signed32网格宽高，乘加回绕后符号扩展；holes closed路径record+17==0过滤，未过滤holes别名保留；DecoratorLibKey保留EAX/EDX32位返回槽；Vec/std/glam复用；Rectangle3d32B字段互证、Triangle使用Vec3而非Vec3A。完整作者ABI、容器/枚举全集和真实producer/consumer仍未关闭，不把叶接口完成称为完整建筑链恢复。\n\n独立合同审查和冻结候选源码审查已记录；仅rustfmt与冻结引用完整性检查，状态implemented_unverified，无构建/测试/oracle或生产整合，旧建筑保存断点及验收不改变。21份delivery与依赖闭包保留，21个claim释放。下一优先点为装饰物source/asset/subtype→位置/碰撞/开口→move/delete/历史传播，先补本批key/holes/Archivist真实owner，材料统计和其他history owner随后，GPU/场景后置。\n'''
for name in ['PROJECT_MEMORY.md','PROJECT_PROGRESS.md','TASK_BACKLOG.md']:
 p=PROJECT/name; p.write_text(p.read_text()+note)
print(json.dumps({'functions':35,'frozen':len(receipts),'released':len(releases),'status':summary['status']}))
