import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))  # -I 模式下显式加入本目录
"""渲染范围的依赖分层: 把过程归到组件，剔除“泛型对 trait 方法的分派”边，求 SCC 并按最长路径分层。
输出屏幕报告 + data/render_layers.json。需先运行 calls.py。用法: python3 -I layers.py"""
import collections, json, re
from cca import CALLS_JSON, WORK, load_procs

sys.setrecursionlimit(20000)
procs = load_procs()
raw = json.load(open(CALLS_JSON))
calls = {int(k, 16): [int(x, 16) for x in v['callees']] for k, v in raw.items()}
indirect = {int(k, 16): v['indirect'] for k, v in raw.items()}

RULES = [('render::ssbo', 'A1 ssbo/buffer基元'), ('render::shadowed_buffer', 'A1 ssbo/buffer基元'), ('render::dirty', 'A1 ssbo/buffer基元'),
 ('render::sparse_instance_buffer', 'A2 sparse_instance_buffer'),
 ('render::uniform', 'A1 uniform/frame/view常量'), ('render::frame_constants', 'A1 uniform/frame/view常量'), ('render::view_constants', 'A1 uniform/frame/view常量'),
 ('render::pass_builder::shader_uri', 'A1 shader_uri/shader程序'), ('render::shader', 'A1 shader_uri/shader程序'),
 ('render::vao', 'A1 mesh/vao/杂项'), ('render::mesh', 'A1 mesh/vao/杂项'), ('render::load_texture', 'A1 mesh/vao/杂项'),
 ('render::nani::mesh_atlas', 'A2 mesh_atlas'),
 ('resources::render', 'A3 资产库(SSBO/shader/mesh/纹理)'), ('asset_libraries', 'A3 资产库(SSBO/shader/mesh/纹理)'),
 ('render::pass_builder', 'A4 pass_builder(资源声明/绑定/draw/compute)'), ('render::raster_single', 'A4 pass_builder(资源声明/绑定/draw/compute)'),
 ('systems::render_loop::clear_rg', 'A4 帧pass辅助(clear/copy)'), ('systems::render_loop::rg_buffer', 'A4 帧pass辅助(clear/copy)'), ('systems::render_loop::RenderContext', 'A4 帧pass辅助(clear/copy)'),
 ('render::nani', 'N nani(GPU驱动)'), ('systems::render_loop::draw_passes', 'D draw_passes'),
 ('systems::render_loop::compute_passes', 'C compute帧pass'), ('systems::render_loop', 'S 风格化/预览/LUT/菜单背景'), ('render', 'A1 mesh/vao/杂项')]
EXT_KNOWN = ('frozen', 'puffin', 'parking_lot_core', 'toolbar', 'hotwatch', 'vk_sync_fork', 'anyhow', 'bumpalo', 'log', 'uuid', 'lang_lib')


def comp(nm):
    if nm.startswith('tiny_glade::render_loop'):
        return 'H 帧编排器 render_world'
    P = re.sub(r'<.*', '', nm).split('::')
    if P[0] != 'country_core':
        t = P[0]
        if t == 'rhapsody': return 'EXT rhapsody'
        if t.startswith('bevy'): return 'EXT bevy'
        if t == 'utils': return 'EXT utils(本地)'
        if t in ('shader_pipeline', 'shader_compiler', 'shader_variants'): return 'EXT shader-pipeline'
        return 'EXT ' + t if t in EXT_KNOWN else 'EXT std/常用库'
    s = '::'.join(P[1:])
    for pre, c in RULES:
        if s.startswith(pre): return c
    return 'OUT country_core::' + '::'.join(P[1:3])


C = lambda a: comp(procs[a][1])
OK = lambda c: not c.startswith(('EXT', 'OUT'))


def trait_dispatch(a, t):
    """调用方是泛型实例，被调方的模块路径出现在调用方的类型参数里 => 对 trait 方法的分派，不是向上依赖。"""
    na, nt = procs[a][1], procs[t][1]
    args = na[len(re.sub(r'<.*', '', na)):]
    mod = '::'.join(re.sub(r'<.*', '', nt).split('::')[:-2])
    return bool(args) and len(mod.split('::')) >= 3 and mod in args


def norm(nm):
    for _ in range(8): nm = re.sub(r'<[^<>]*>', '', nm)
    return nm


def scc_layers(graph, nodes):
    idx, low, st, on, sccs, c = {}, {}, [], set(), [], [0]

    def sc(v):
        idx[v] = low[v] = c[0]; c[0] += 1; st.append(v); on.add(v)
        for w in graph[v]:
            if w not in idx: sc(w); low[v] = min(low[v], low[w])
            elif w in on: low[v] = min(low[v], idx[w])
        if low[v] == idx[v]:
            k = []
            while True:
                w = st.pop(); on.discard(w); k.append(w)
                if w == v: break
            sccs.append(k)
    for v in nodes:
        if v not in idx: sc(v)
    co = {v: i for i, s in enumerate(sccs) for v in s}; memo = {}

    def depth(i):
        if i not in memo:
            memo[i] = 1 + max([depth(co[w]) for v in sccs[i] for w in graph[v] if co[w] != i], default=-1)
        return memo[i]
    return sccs, {i: depth(i) for i in range(len(sccs))}


real = collections.defaultdict(collections.Counter); disp = 0
cnt = collections.Counter(); size = collections.Counter(); ext = collections.defaultdict(collections.Counter); out = collections.defaultdict(collections.Counter)
for a, cs in calls.items():
    ca = C(a); cnt[ca] += 1; size[ca] += procs[a][0]
    for t in cs:
        ct = C(t)
        if ct.startswith('EXT') and OK(ca): ext[ca][ct[4:]] += 1
        elif ct.startswith('OUT') and OK(ca): out[ca][ct[18:]] += 1
        elif ct != ca and OK(ca) and OK(ct):
            if trait_dispatch(a, t): disp += 1
            else: real[ca][ct] += 1
L = sorted(c for c in cnt if OK(c))
print(f'函数 {len(calls)} | 真实依赖边 {sum(sum(v.values()) for v in real.values())} | trait分派边(已剔除) {disp} | 间接调用点 {sum(indirect.values())}')
sccs, dep = scc_layers({v: set(real[v]) for v in L}, L)
cyc = [s for s in sccs if len(s) > 1]
print('组件级环:', cyc or '无')
rows = []
print('\n== 自底向上构建层次')
for i in sorted(range(len(sccs)), key=lambda i: (dep[i], sccs[i][0])):
    for v in sccs[i]:
        deps = sorted({w.split()[0] for w in real[v]})
        print(f' 层{dep[i]} {v:44s} 函数{cnt[v]:4d} {size[v] // 1024:4d}KiB  依赖: {",".join(deps) or "-"}')
        rows.append({'layer': dep[i], 'component': v, 'procs': cnt[v], 'KiB': size[v] // 1024, 'depends_on': deps})
print('\n== 对外部库的调用边数(非 std)')
for c in sorted(ext):
    print(f' {c:44s} std:{ext[c]["std/常用库"]:5d}  ' + '  '.join(f'{k}:{v}' for k, v in ext[c].most_common() if k != 'std/常用库')[:150])
print('\n== 对渲染范围外 country_core 模块的依赖')
for c in sorted(out):
    print(f' {c:44s}', dict(out[c].most_common(6)))


def sublayers(target):
    members = [a for a in calls if C(a) == target]; U = collections.defaultdict(set)
    for a in members:
        U[norm(procs[a][1])]
        for t in calls[a]:
            if C(t) == target and norm(procs[t][1]) != norm(procs[a][1]) and not trait_dispatch(a, t):
                U[norm(procs[a][1])].add(norm(procs[t][1]))
    for u in list(U):
        for w in U[u]: U[w]
    ss, d = scc_layers(U, list(U))
    lv = collections.Counter()
    for i, s in enumerate(ss): lv[d[i]] += len(s)
    return len(U), dict(sorted(lv.items())), sum(1 for s in ss if len(s) > 1)


print('\n== 层内(泛型去重)独立单元数 / 各子层单元数 / 互递归环数')
inner = {}
for t in L:
    if cnt[t] > 20:
        inner[t] = sublayers(t); print(f' {t:46s}', inner[t])
WORK.mkdir(parents=True, exist_ok=True)
json.dump({'layers': rows, 'inner': {k: {'units': v[0], 'sublayers': v[1], 'cycles': v[2]} for k, v in inner.items()},
           'trait_dispatch_edges': disp, 'indirect_call_sites': sum(indirect.values())}, open(WORK / 'render_layers.json', 'w'), ensure_ascii=False, indent=1)
