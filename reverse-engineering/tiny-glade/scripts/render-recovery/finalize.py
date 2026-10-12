"""Register only evidenced renderer kernels and retain unfinished subsystem gates."""
from pathlib import Path
import json,hashlib,datetime,tomllib
ROOT=Path('D:/game/reverse-engineering/tiny-glade')
def read(p):return json.loads((ROOT/p).read_text(encoding='utf8'))
def digest(p):return hashlib.sha256((ROOT/p).read_bytes()).hexdigest()
core=read('evidence/render-recovery/resource-info-verification.json')
usage=read('evidence/render-leaves/verification.json')
deps=read('evidence/render-recovery/dependency-verification.json')
skeleton=read('evidence/render-skeleton/report.json')
assert core['ok'] and usage['ok'] and deps['ok']
assert usage['candidate_source_sha256']==digest('reconstruction/render/resource_usage.rs')
assert core['candidate_cargo_lock_sha256']==usage['renderer_cargo_lock_sha256']==digest('reconstruction/render/Cargo.lock')
for p,h in core['candidate_sources'].items():assert digest(p)==h
report={'ok':True,'source_sha256':core['source_sha256'],'total_cases':core['total_cases']+usage['cases'],
 'original_private_functions':5,'full_renderer_complete':False,
 'candidate_sources':{**core['candidate_sources'],'reconstruction/render/resource_usage.rs':usage['candidate_source_sha256']},
 'renderer_cargo_lock_sha256':digest('reconstruction/render/Cargo.lock'),
 'original_dependency_packages_verified':len(deps['packages']),
 'pdb_source_paths':skeleton['source_file_count'],'pdb_full_rust_signatures_restored':False,
 'subreports':['evidence/render-recovery/resource-info-verification.json','evidence/render-leaves/verification.json','evidence/render-recovery/dependency-verification.json','evidence/render-skeleton/report.json'],
 'validation':{'locked_release_build':True,'clippy_all_targets_deny_warnings':True,'main_project_cli_checks':6},
 'scope':'Five private CPU renderer leaf rules; typed/numeric API reconstruction, standard dependencies reused',
 'limitations':['No Vulkan device, command submission, shader algorithm, full resource graph or image validation','Raw observed records are not a complete original Rust ABI','Concurrency access classifier is an externally supplied original dependency boundary; local vk-sync-fork source revision remains unknown','Panic occurrence/partial state covered; original panic messages/unwind and allocator behavior excluded','Skeleton file/metadata counts do not count as implemented source']}
(ROOT/'evidence/render-recovery/verification.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
project=read('tracking/project.json')
mid=next(m for m in project['milestones']if m['id']=='M12')
mid['status']='in_progress';mid['source_gate']={'status':'verified_partial','scope':f"5个私有CPU资源图规则；{report['total_cases']}机器码案例；Vulkan命令/完整图/shader/画面未验收"}
mid['evidence']=list(dict.fromkeys(mid['evidence']+['reconstruction/render/Cargo.toml','reconstruction/render/resource_info.rs','reconstruction/render/resource_usage.rs','evidence/render-recovery/verification.json','evidence/render-recovery/findings.md','evidence/render-skeleton/report.json']))
mid['remaining']=['ResourceInfo::calculate完整入口及导入/导出/访问生产者','资源注册池、barrier生成、batch录制和提交/retire链','原vk-sync-fork 0.4.0本地来源/必要补丁核实；不重写公共库','游戏pass及shader算法、GPU/画面/性能对照']
project['local_algorithms']=[a for a in project['local_algorithms']if a['name']not in ['Renderer resource graph rules','Renderer Vulkan usage conversions']]
project['local_algorithms'] += [
 {'name':'Renderer resource graph rules','status':'verified_local','source':'reconstruction/render/resource_info.rs','report':'evidence/render-recovery/verification.json','scope':'访问链、生命周期事件、并发判断；依赖classifier外供；78867案例'},
 {'name':'Renderer Vulkan usage conversions','status':'verified_local','source':'reconstruction/render/resource_usage.rs','report':'evidence/render-recovery/verification.json','scope':'2个完整private用途转换叶函数；ash原版本复用；142149案例'}]
project['render_recovery_authorization']={'date':'2026-10-12','scope':'用户提出按PDB骨架和原反汇编恢复Rust并编译测试；本轮开始渲染CPU框架恢复','report':'evidence/render-recovery/findings.md'}
project['deferred_work']=['完整场景回放与运行交互验收仍后置','GPU/渲染按用户最新指示开始源码恢复；完整shader动态/画面验收尚未开展']
project['known_gaps']=[x.replace('完整游戏、场景回放与GPU尚未验收，后两项按用户要求后置','完整游戏、场景回放与GPU尚未验收；GPU源码按最新用户要求开始恢复')for x in project['known_gaps']]
project['next_actions']=list(dict.fromkeys(['渲染：恢复ResourceInfo::calculate及必要依赖fork来源、barrier/资源注册/执行器链；按模块做原码差分']+project['next_actions']))
(ROOT/'tracking/project.json').write_text(json.dumps(project,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
print(json.dumps({'ok':True,'private_functions':5,'cases':report['total_cases'],'dependency_packages':len(deps['packages'])}))

