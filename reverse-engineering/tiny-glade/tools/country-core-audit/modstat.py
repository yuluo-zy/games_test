import os, sys; sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))  # -I 模式下显式加入本目录
"""按功能模块统计“业务逻辑挖掘度”（以 PDB 中 country_core 自有函数的机器码字节为分母）。
每个原函数地址归入一档（取最高档）:
  untouched 任何文档/证据/代码都没引用   cited 仅在合同/证据/注释里登记
  stub      对应 Rust 函数是 todo!/空/常量/Gap   shallow 有真实代码但行数 < 机器码字节/100
  solid     有真实代码且体量相称
“solid”只检查体量，不保证逻辑正确（合同抽查中约 1/4 的 O 结论是错的）。
输出 data/module_stats.tsv 与屏幕表格。用法: python3 -I modstat.py"""
import collections, os, re
from cca import CC_SRC, CC_WP, WORK, ADDR_RE, load_procs

procs = load_procs('country_core::')
RULES = [
 ('systems::wall|resources::walls|resources::stairs|geometry::instanced_wall|utils::curve|utils::child_wall|systems::collision|resources::in_progress', '墙体/楼梯/碰撞'),
 ('resources::roofs|systems::roof', '屋顶'),
 ('systems::terrain|resources::terrain|resources::water|systems::water|resources::world_raster|systems::world_raster|resources::rocky|startup::startup_terrain|startup::startup_water', '地形/水/世界栅格'),
 ('resources::critters|resources::tree|systems::critters|resources::vegetation|systems::forest|systems::garden|resources::ivy|systems::ivy|startup::startup_(sheep|critters|grass|meadow|ivy)|startup::spawn_preplaced|components::.*tree', '植被/生物/常春藤'),
 ('systems::history|resources::history|systems::save|resources::session|resources::prefab|systems::prefab|startup::startup_session|startup::load_prefabs|systems::timelapse', '历史/存档/会话/预制件'),
 ('systems::audio|resources::audio|startup::load_.*audio|startup::load_glade_audio|utils::audio|audio_events|asset_libraries::audio', '音频'),
 ('render::nani|systems::render_loop::draw_passes|asset_libraries::ssbo', 'nani+绘制着色器+SSBO库'),
 ('render::pass_builder|render::(ssbo|shadowed|sparse|uniform|shader|mesh|vao|indirect|frame_constants|view_constants|render_config|directional|dirty|atmosphere|adaptive|debug_info|fsr|load_texture|raster_single|utils|mod)|asset_libraries', '渲染基础设施(pass/buffer/资产)'),
 ('systems::render_loop', '渲染帧pass(compute/风格化/LUT)'),
 ('camera_rig|main_camera|look_at_camera|systems::time|systems::weather|resources::time|systems::mode_manager', '相机/时间/天气'),
 ('systems::ui|systems::backend|input|systems::mod_key|game_key|resources::(toolbar|ui_|gui|mouse|cursor|drawing|brush|raster_stroke|active_raster|terrain_stroke)|systems::(brush|arrow|clear_canvas|despawn|mesh_follow|utils)', 'UI/输入'),
 ('startup|app_loading|game_config|game_profile|meta_commander|resources::(steam|shutdown|daily|photo|glade|image|loading|logo|gpu_lut|grading|feedback|color|texture|clutter|placement|sparkle|shader_ref|gothic|window|shape|spatial|preplaced|stairs_walls|render)|systems::steam|systems::debug|utils|components|boundary|reused|lib', '启动/杂项资源/工具')]


def module(nm):
    s = '::'.join(re.sub(r'<.*', '', nm).split('::')[1:])
    for pat, n in RULES:
        if re.search(pat, s):
            return n
    return '其它'


cited = set()
for dp, _, fs in os.walk(CC_WP):
    for f in fs:
        if re.search(r'\.(rs|md|c|txt|json)$', f):
            for m in ADDR_RE.finditer(open(os.path.join(dp, f), errors='replace').read()):
                if int(m.group(1), 16) in procs:
                    cited.add(int(m.group(1), 16))

STUB_BODY = re.compile(r'(crate::)?(typed_gap::)?Gap|0(\.0)?(f32)?|false|true|Vec::new\(\)|Default::default\(\)|None|Ok\(\(\)\)|self|Vec3::ZERO|glam::Vec3::ZERO|&\[\]|\(\)')
UNTOUCHED, CITED, STUB, SOLID, SHALLOW = range(5)
impl = {}
fnre = re.compile(r'\bfn\s+(\w+)')
for dp, _, fs in os.walk(CC_SRC):
    for f in fs:
        if not f.endswith('.rs'):
            continue
        lines = open(os.path.join(dp, f), errors='replace').read().split('\n'); i = 0
        while i < len(lines):
            m = fnre.search(lines[i])
            if not (m and not lines[i].lstrip().startswith(('//', '*'))):
                i += 1; continue
            j = i - 1; doc = []
            while j >= 0 and lines[j].lstrip().startswith(('///', '//', '#[')):
                doc.append(lines[j]); j -= 1
            k = i; depth = 0; started = False; body = []
            while k < len(lines):
                body.append(lines[k]); depth += lines[k].count('{') - lines[k].count('}')
                started |= '{' in lines[k]
                if started and depth <= 0 or (not started and ';' in lines[k]): break
                k += 1
            text = '\n'.join(body); code = re.sub(r'//.*', '', text)
            inner = code[code.find('{') + 1:code.rfind('}')].strip() if '{' in code else ''
            il = [x for x in inner.split('\n') if x.strip()]
            stub = bool(re.search(r'todo!|unimplemented!', inner)) or inner == '' or STUB_BODY.fullmatch(inner) or ('Gap' in inner and len(il) <= 2)
            nl = len([x for x in il if not x.strip().startswith(('//', '/*', '*'))])
            for a in re.findall(r'0x(14[0-9a-fA-F]{7})', '\n'.join(doc) + '\n' + text):
                a = int(a, 16)
                if a in procs:
                    lvl = STUB if stub else (SOLID if nl >= procs[a][0] / 100 else SHALLOW)
                    rank = {STUB: 1, SHALLOW: 2, SOLID: 3}
                    if rank[lvl] > rank.get(impl.get(a), 0): impl[a] = lvl
            i = k + 1 if k > i else i + 1

b = collections.defaultdict(lambda: [0] * 5); nf = collections.defaultdict(int)
for a, (sz, nm) in procs.items():
    lv = impl.get(a, CITED if a in cited else UNTOUCHED)
    g = module(nm); b[g][lv] += sz; nf[g] += 1
rows = sorted(((g, sum(v), v) for g, v in b.items()), key=lambda r: -r[1])
hdr = ['module', 'KiB', 'procs', 'untouched', 'cited', 'stub', 'shallow', 'solid']
lines = ['\t'.join(hdr)]
print(f"{'模块':26s}{'KiB':>6s}{'函数':>6s}{'未触及':>8s}{'仅登记':>7s}{'桩':>6s}{'浅':>6s}{'扎实':>6s}")
tot = [0] * 5
for g, sz, v in rows:
    p = [v[UNTOUCHED], v[CITED], v[STUB], v[SHALLOW], v[SOLID]]
    print(f"{g:24s}{sz // 1024:6d}{nf[g]:6d}" + ''.join(f'{x / sz:8.0%}' for x in p))
    lines.append('\t'.join([g, str(sz // 1024), str(nf[g])] + [f'{x / sz:.4f}' for x in p]))
    for i in range(5): tot[i] += v[i]
S = sum(tot)
print('合计', f'{S // 1024} KiB', f'未触及 {tot[0] / S:.1%} 仅登记 {tot[1] / S:.1%} 桩 {tot[2] / S:.1%} 浅 {tot[4] / S:.1%} 扎实 {tot[3] / S:.1%}')
WORK.mkdir(parents=True, exist_ok=True)
open(WORK.parent / 'module_stats.tsv', 'w', encoding='utf8').write('\n'.join(lines) + '\n')
