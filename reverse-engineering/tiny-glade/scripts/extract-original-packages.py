"""原作随包 Cargo.lock 的依赖图；仅构建元数据，不推断运行功能覆盖。"""
from pathlib import Path
import tomllib, json, hashlib, collections
root = Path('D:/game/reverse-engineering/tiny-glade')
file = Path('D:/game/ljxsj_92385/Tiny Glade/build-info/Cargo.lock')
raw = file.read_bytes()
packages = tomllib.loads(raw.decode('utf-8'))['package']
by_name = collections.defaultdict(list)
for p in packages:
    by_name[p['name']].append(p)
def key(p):
    return p['name']+' '+p['version']+' '+p.get('source','path-or-local')
resolved, unresolved = {}, []
for p in packages:
    edges=[]
    for dependency in p.get('dependencies',[]):
        parts=dependency.split()
        matches=by_name[parts[0]]
        if len(parts)>1:
            matches=[x for x in matches if x['version']==parts[1]]
        if len(matches)==1:
            edges.append(key(matches[0]))
        else:
            unresolved.append(dict(parent=key(p),dependency=dependency,candidates=len(matches)))
    resolved[key(p)]=edges
roots=[key(p) for p in packages if p['name']=='tiny-glade']
closure=set()
todo=roots[:]
while todo:
    current=todo.pop()
    if current in closure:
        continue
    closure.add(current)
    todo.extend(resolved[current])
milestone_for={'system-roof':'M03/M04/M05/M06','system-wall-constructor':'M08','system-decorator':'M09',
  'system-water':'M10','paths':'M10','system-tree':'M13','forest':'M13','system-critters':'M13',
  'system-clutter':'M09/M13','system-onboarding':'M13','system-color':'M09/M12','kajiya':'M12',
  'shader-pipeline':'M12','shader-compiler':'M12','shader-variants':'M12','asset':'M12',
  'state-stream':'M07','toolbar':'M11/M13','country-core':'M07/M08/M09/M10/M11/M12/M13',
  'tiny-glade':'M14','utils':'跨模块算法依赖'}
local=[dict(name=p['name'],version=p['version'],dependencies=p.get('dependencies',[]),
            inside_tiny_glade_build_dependency_closure=key(p) in closure,
            tracking_assignment=milestone_for.get(p['name'],'待确认用途/分配'),runtime_behavior_verified=False)
       for p in packages if 'source' not in p]
report={'source':'build-info/Cargo.lock','source_sha256':hashlib.sha256(raw).hexdigest(),
  'package_count':len(packages),'source_absent_package_count':len(local),'tiny_glade_roots':roots,
  'build_dependency_closure_count':len(closure),'packages_without_source':local,'edges':resolved,
  'unresolved_edges':unresolved,'limitations':['无 source 字段只表示 path/local 构建条目，不能认定所有条目属于游戏作者',
  'Cargo.lock 包含构建、条件平台和工具依赖，依赖闭包不等于运行时加载清单',
  '里程碑分配是研究管理归属，不是已验证的功能/调用关系','不能据此计算整游戏还原百分比']}
(root/'tracking/original-packages.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf-8')
policy_file=root/'tracking/dependency-policy.json'
policy_by_name={p['name']:p for p in json.loads(policy_file.read_text(encoding='utf-8'))['packages']} if policy_file.exists() else {}
lines=['# 原作构建模块范围','',f"源：原作 `build-info/Cargo.lock`，SHA256 `{report['source_sha256']}`。",'',
  f"记录 {len(packages)} 个 package，{len(local)} 个条目没有 source 字段；Tiny Glade 构建依赖闭包 {len(closure)} 个，无法唯一解开的边 {len(unresolved)} 条。",'',
  '下表是构建条目索引，不是逆向待办清单。公开依赖及引擎复用；local/path先核实归属，通用SDK/库不得重新还原。游戏自有功能按TASK_BACKLOG推进，完整系统状态见PROJECT_PROGRESS。', '',
  '| 构建条目 | 版本 | 处于主包构建依赖闭包 | 历史管理归属 | 当前依赖边界 |','|---|---|---|---|---|']
for p in local:
    policy=policy_by_name.get(p['name'],{}).get('policy','')
    boundary='游戏自有候选；依赖内部排除，归属需源路径确认' if policy=='game_owned_candidate_verify_source_attribution' else '复用或先查上游/归属；不默认逆向'
    lines.append(f"| {p['name']} | {p['version']} | {'是' if p['inside_tiny_glade_build_dependency_closure'] else '否'} | {p['tracking_assignment']} | {boundary} |")
lines += ['', '完整图：[original-packages.json](tracking/original-packages.json)。依赖政策：[dependency-policy.json](tracking/dependency-policy.json)。源码任务：[TASK_BACKLOG.md](TASK_BACKLOG.md)。项目进度：[PROJECT_PROGRESS.md](PROJECT_PROGRESS.md)。','']
(root/'ORIGINAL_MODULE_SCOPE.md').write_text('\n'.join(lines),encoding='utf-8')
print(json.dumps({k:report[k] for k in ['package_count','source_absent_package_count','build_dependency_closure_count']}))
