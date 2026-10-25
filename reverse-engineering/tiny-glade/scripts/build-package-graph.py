"""Build a self-contained B Cargo package graph with source-ledger overlays."""
import datetime
import hashlib
import json
from pathlib import Path
import re
import tomllib

ROOT = Path(__file__).resolve().parents[1]
lock_path = ROOT / 'tracking/original-Cargo.lock'
lock_bytes = lock_path.read_bytes()
lock_hash = hashlib.sha256(lock_bytes).hexdigest()
index = json.loads((ROOT / 'tracking/original-packages.json').read_text(encoding='utf-8'))
assert lock_hash == index['source_sha256'], 'original lock identity mismatch'
packages = tomllib.loads(lock_bytes.decode())['package']
progress = json.loads((ROOT / 'tracking/module-progress.json').read_text(encoding='utf-8'))
policy = json.loads((ROOT / 'tracking/dependency-exclusions.json').read_text(encoding='utf-8'))
policies = {p['name']: p['policy'] for p in policy['packages']}
aliases = {'system-tree': 'crates/systems/tree-tool', 'forest': 'crates/systems/forest',
           'timberframe': 'crates/systems/timberframe', 'toolbar': 'crates/systems/toolbar',
           'shader-pipeline': 'crates/rhapsody/crates/shader-pipeline'}
roles = {
    'tiny-glade': '应用入口', 'country-core': '共享游戏状态、资源与系统', 'rhapsody': '自有渲染图与Vulkan执行',
    'utils': '游戏自有曲线、几何与工具', 'asset': '资产数据结构', 'asset-pipe': '资产处理', 'asset-pipe-bin': '资产构建工具',
    'shader-compiler': 'Shader编译与反射', 'shader-pipeline': 'Shader管线数据', 'shader-variants': 'Shader变体',
    'tabloid': '消息/订阅工具', 'paths': '路径与平台目录', 'lang-lib': '本地化数据', 'tg-text': '文本绘制',
    'motomoto': '存档二进制读写', 'tg-package-integrity': '安装包完整性检查',
    'theme-defs': '主题数据', 'theme-fetcher': '主题获取', 'theme-server-proto': '主题协议',
    'toolbar': '编辑工具协调', 'forest': '森林系统', 'timberframe': '木构系统',
    'ash': 'Vulkan API', 'bevy_app': '应用与插件', 'bevy_ecs': '实体组件与调度', 'glam': '向量与矩阵',
    'fastrand': '随机数', 'serde': '序列化', 'rkyv': '归档格式', 'ron': 'RON格式', 'winit': '窗口与事件',
    'gpu-allocator': 'GPU分配器，原local分叉待核实', 'vk-sync-fork': 'Vulkan同步依赖，原分叉待核实',
}
systems = {'system-clutter':'自动杂物','system-color':'颜色','system-critters':'小生物','system-decorator':'装饰与附着',
           'system-glade-border':'世界边界','system-onboarding':'引导','system-roof':'屋顶','system-tree':'树木工具',
           'system-wall-constructor':'墙与建筑生成','system-water':'水域'}
roles.update(systems)
core_names = {'tiny-glade','country-core','rhapsody','utils','asset','asset-pipe','shader-compiler','shader-pipeline',
              'shader-variants','tabloid','paths','lang-lib','tg-text','motomoto','tg-package-integrity',
              'theme-defs','toolbar','forest','timberframe',*systems}
external_versions = {'ash':'0.38.0+1.3.281','bevy_app':'0.16.0','bevy_ecs':'0.16.0','glam':'0.29.3',
                     'fastrand':'1.8.0','serde':'1.0.200','rkyv':'0.7.39','ron':'0.8.1','winit':'0.29.15'}

STAGES = [('analysis', '分析'), ('implementation', '实现'), ('integration', '接线'),
          ('verification', '原作对照'), ('acceptance', '完整验收')]
evidence_by_path = {e['path']: e for e in progress['evidence']}
stage_acceptance = json.loads((ROOT / 'tracking/package-graph-stage-acceptance.json').read_text(encoding='utf-8'))
assert stage_acceptance['baseline'] == 'B'


def accepted_gate(gate):
    return (gate.get('baseline') == 'B' and bool(gate.get('scope'))
            and bool(gate.get('evidence'))
            and all((ROOT / f).is_file() for f in gate['evidence']))


def recovery_progress(modules, is_dependency):
    """Workflow states, never a percentage inferred from files or functions."""
    if is_dependency:
        return {'state': 'excluded', 'label': '依赖复用', 'stages': []}
    # shader-compiler is an explicit B toolchain contract without a recovered PDB source path.
    relevant = [m for m in modules if ('B' in m['versions'] or m['module'] == 'shader-compiler')
                and m['status'] not in ('dependency_boundary', 'metadata_only')]
    started_states = {'analysis_partial', 'draft_partial', 'candidate_partial',
                      'integrated_partial', 'reported_missing'}
    started = any(m['status'] in started_states or m['full_module_accepted'] for m in relevant)
    accepted = bool(relevant) and all(m['full_module_accepted']
                                    and accepted_gate(m['full_module_acceptance'] or {}) for m in relevant)
    has_code = any(m['status'] in ('draft_partial', 'candidate_partial', 'integrated_partial')
                   and any((ROOT / f).is_file() for f in m['restoration_files']) for m in relevant)
    has_wiring = any(m['status'] == 'integrated_partial' for m in relevant)
    # Original comparisons are separate from compile checks and candidate tests.
    has_comparison = any(
        (m.get('interface_progress') or {}).get('direct_original_entry_compared_partial', 0)
        or any(b.get('status') == 'reported_verified_local' for b in m.get('function_bindings', []))
        or any((evidence_by_path.get(f, {}).get('reported_status') or {}).get('ok') is True
               for f in m['evidence'] if 'verification' in f)
        for m in relevant if m['status'] != 'reported_missing')
    partial = [started, has_code, has_wiring, has_comparison, False]
    stages = []
    for (key, label), touched in zip(STAGES, partial):
        # Optional explicit, scoped phase gates; no auto-promotion from partial status.
        gates = [stage_acceptance['modules'].get(m['module'], {}).get(key, {}) for m in relevant]
        touched = touched or any(g.get('status') == 'in_progress' and accepted_gate(g) for g in gates)
        complete = accepted or (key != 'acceptance' and bool(gates) and all(
            g.get('status') == 'complete' and accepted_gate(g) for g in gates))
        state = 'complete' if complete else 'in_progress' if touched else 'pending'
        stages.append({'key': key, 'label': label, 'state': state})
    missing = sum(m['status'] == 'reported_missing' for m in relevant)
    return {'state': 'complete' if accepted else 'in_progress' if started else 'pending',
            'label': '完整验收' if accepted else '恢复中' if started else '未登记开始',
            'stages': stages, 'registeredModules': len(relevant), 'missingModules': missing}


def package_key(p):
    return f"{p['name']} {p['version']} {p.get('source', 'path-or-local')}"


def source_prefix(name):
    if name in aliases:
        return aliases[name] + '/src'
    if name.startswith('system-'):
        return 'crates/systems/' + name.removeprefix('system-') + '/src'
    return 'crates/' + name + '/src'


def architecture_layer(name, local):
    if name == 'tiny-glade': return 0
    if name in systems or name in ('forest','timberframe','toolbar'): return 1
    if name == 'country-core': return 2
    if name in ('rhapsody','tg-text','shader-compiler','shader-pipeline','shader-variants'): return 3
    return 4 if local else 5


nodes = []
for p in packages:
    name = p['name']
    local = 'source' not in p
    prefix = source_prefix(name)
    modules = [m for m in progress['modules'] if m['module'] == name
               or m['module'] == prefix or m['module'].startswith(prefix + '/')]
    states = {m['status'] for m in modules}
    is_dependency = (not local or policies.get(name) == 'dependency_reuse'
                     or states == {'dependency_boundary'})
    if is_dependency:
        state, label = 'reuse', '依赖复用' if not local else '依赖/分叉边界'
    elif 'integrated_partial' in states:
        state, label = 'partial', '部分接线'
    elif 'candidate_partial' in states or 'draft_partial' in states:
        state, label = 'candidate', '部分候选'
    elif 'reported_missing' in states:
        state, label = 'missing', '交付记录/缺失'
    elif 'analysis_partial' in states:
        state, label = 'analysis', '部分分析'
    else:
        state, label = 'unassessed', '待评估'
    nodes.append({'id':package_key(p), 'name':name, 'version':p['version'], 'local':local,
        'source':p.get('source','path/local（原位置未从lock恢复）'), 'role':roles.get(name,'依赖/私有包，详见原定义与来源'),
        'core':name in core_names and local, 'external':name in external_versions and p['version']==external_versions[name],
        'layer':architecture_layer(name,local), 'policy':policies.get(name,'ownership_unresolved'),
        'status':state, 'statusLabel':label, 'progress':recovery_progress(modules,is_dependency),
        'modules':[{'path':m['module'],'scope':m['recovered_scope'],
            'status':m['status'],'gaps':m['gaps'], 'next':m['next_action'],
            'integration':m['integration'], 'verification':m['verification_summary'],
            'evidence':m['evidence'],
            'detail':'tracking/module-progress/'+re.sub(r'[^\w.-]+','__',m['module'])+'.md'} for m in modules],
        'inRoot':next((q['inside_tiny_glade_build_dependency_closure'] for q in index['packages_without_source'] if q['name']==name),True)})
node_ids = {n['id'] for n in nodes}
edges = []
for parent, dependencies in index['edges'].items():
    assert parent in node_ids, parent
    for dependency in dependencies:
        assert dependency in node_ids, dependency
        edges.append({'from':parent,'to':dependency})
assert len(nodes) == 901 and len({n['id'] for n in nodes}) == 901
assert len({(e['from'],e['to']) for e in edges}) == len(edges)
data = {'baseline':'B', 'progressUpdatedAt':progress['updated_at'],
        'date':datetime.datetime.now(datetime.timezone(datetime.timedelta(hours=8))).isoformat(),
        'lockHash':lock_hash, 'nodes':nodes, 'edges':edges, 'unresolved':index['unresolved_edges'],
        'limits':['Cargo.lock是构建依赖，不能区分build/dev/target依赖或证明运行调用顺序。',
                  '职责分层是阅读布局；依赖深度仅按当前显示的实际边计算。',
                  '同名多版本按version/source区分，不合并。五段进度是阶段状态，不是功能完成百分比。',
                  '橙色斜纹表示阶段已有局部恢复；绿色实心仅表示显式阶段验收。整包完成需要全部已登记B模块完整验收。',
                  '灰色表示未登记开始，不断言从未研究；外部依赖不计逆向进度。',
                  'Bevy分叉按用户说明记录；精确仓库/commit仍未核实。']}
json_text = json.dumps(data,ensure_ascii=False,separators=(',',':')).replace('</','<\\/')
template = (ROOT / 'tools/package-graph/template.html').read_text(encoding='utf-8')
assert template.count('__PACKAGE_DATA__') == 1
output = ROOT / 'package-dependencies.html'
output.write_text(template.replace('__PACKAGE_DATA__', json_text), encoding='utf-8')
manifest = {'baseline':'B','lock_sha256':lock_hash,'nodes':len(nodes),'direct_edges':len(edges),
            'local_packages':sum(n['local'] for n in nodes),'core_packages':sum(n['core'] for n in nodes),
            'unresolved_edges':len(index['unresolved_edges']), 'html_sha256':hashlib.sha256(output.read_bytes()).hexdigest(),
            'inputs':{f:hashlib.sha256((ROOT/f).read_bytes()).hexdigest() for f in ['tracking/original-packages.json',
                'tracking/module-progress.json','tracking/dependency-exclusions.json',
                'tracking/package-graph-stage-acceptance.json','scripts/build-package-graph.py',
                'tools/package-graph/template.html']}}
(ROOT/'tracking/package-graph.json').write_text(json.dumps(manifest,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print(json.dumps(manifest,ensure_ascii=False))
