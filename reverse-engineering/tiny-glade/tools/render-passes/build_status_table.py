#!/usr/bin/env python3
"""Build reconstruction/shader-source/SHADER_STATUS.{md,json}: one row per HLSL file with category,
function, implementation kind (lifted / hand-written), structural gate and differential-execution
verdicts per (stage, variant), and notes. Pure aggregation of the evidence JSONs; run after
sweep_all.py + diff_exec*.py.
"""
import json, re, sys
from collections import Counter, defaultdict
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
sys.path.insert(0, str(HERE))
import sweep_all  # noqa: E402
import diff_common  # noqa: E402

SH = ROOT / 'reconstruction/shader-source/shaders'
INT = ROOT / 'evidence/render-passes/integration'
STAGE = {'cs_main': 'CS', 'vs_main': 'VS', 'ps_main': 'PS'}

NOTES = {  # file -> known remark (all backed by evidence in validation-pipeline.md / RECOVERY_TRACKER.md)
    'final_post.raster.hlsl': 'f0ad 变体的 ps_main 是无结果占位，dxc 会整段删除，结构门禁无法匹配；文件头保留 KNOWN EXCEPTION 说明',
    'lilypad.raster.hlsl': 'VS 曾在 14 号输入出现差异，根因不是 FMA 而是 dxc 把 (x*20)-(z*20) 因式分解为 (x-z)*20 改变舍入；batch_lift 用 spv_fmsub(precise 临时量) 保持两次乘法后，各变体 SAME',
    'reed.raster.hlsl': '同 lilypad：dxc 因式分解 (x*20)-(z*20) 导致差异；spv_fmsub 修复后各变体 SAME',
    'path_pebbles.cs.hlsl': 'spirv-cross 把 OpSRem 展开为 a-b*(a/b)，除数为 0 时 UB；batch_lift.fix_srem 修复后 SAME',
    'ice_plane.raster.hlsl': '曾因 half(NaN) 符号位(UB)出现差异，提升后各变体均 SAME',
    'taa.cs.hlsl': 'frexp 辅助函数未处理非规格化数（提升缺陷），已修复后 SAME',
    'tgsr.cs.hlsl': 'frexp 辅助函数未处理非规格化数（提升缺陷），4 个变体已修复后 SAME',
    'water/ice_refl_resolve.cs.hlsl': '已逐值证明：重建版用 dxc -Gis(严格 IEEE)编译并给原版/重建版全部浮点运算注入 NoContraction 后，0–63 号输入逐位一致(strict 差异 0)；默认执行的差异仅来自 lavapipe 对两种指令顺序的乘加融合不同(对照：原版默认 vs 原版禁融合 差 29 字)。另证：同一份重建版以 -Gis 严格顺序编译、允许驱动融合时与原版默认执行逐位一致(0 差异)，说明唯一差别是 dxc 对表达式的重结合；仍记为 EDGE(已证明)，因为默认 dxc 编译无法在 HLSL 里保留严格顺序又允许融合',
    'light_binning/bin_lights_to_cells.cs.hlsl': 'dxc 把 (6a)*(6a) 重排为 36*(a*a) 使球/格子边界判断翻转；对相关值加 precise 后两个变体 SAME',
    'dof/analyze.cs.hlsl': 'dxc 会给 quad 操作加 DerivativeGroupQuadsKHR，由 spv_strip_dgq.py 去除后一致',
    'water/spawn_waterwheels.cs.hlsl': '原版只有约 3 条指令(空壳)',
    'garden/marching_squares.cs.hlsl': '需 64–400 号输入才触发写入',
}


def load(name):
    p = INT / name
    return json.load(open(p)) if p.exists() else []


def describe(text):
    m = re.search(r'^// ==RECONSTRUCTED==.*$', text, re.M)
    lines = text[m.end():].splitlines()[1:] if m else []
    out = []
    for l in lines:
        if l.startswith('//'):
            out.append(l[2:].strip())
        elif out or l.strip():
            break
    s = ' '.join(x for x in out if x)
    if not s and m:  # fall back to the evidence text on the header line itself
        s = re.sub(r'^\S+\s+\S+\s*\|?\s*', '', m.group(0)[len('// ==RECONSTRUCTED=='):].strip())
        s = re.sub(r'^(evidence:|F-E|\|)\s*', '', s.strip())
    s = re.sub(r'target:[^|]*\|\s*', '', s)
    s = re.sub(r'(evidence|status):\s*', '', s)
    return (s[:150] + '…') if len(s) > 150 else s


def main():
    raw = [json.loads(l) for l in open(ROOT / 'evidence/render-passes/skeleton/raw-per-bin.jsonl')]
    fam_of, bins_of = {}, defaultdict(list)
    for b in raw:
        rel = (b.get('main_source') or '').split('shaders/', 1)[-1]
        fam_of[rel] = b.get('family', '')
        bins_of[rel].append(b['bin'])
    sweep = {}
    for r in load('fidelity-sweep.json')['results']:
        for m in r['modules']:
            sweep[(r['bin'], m['entry'])] = m
    exe = {}
    for n in ('diff-exec.json', 'diff-exec-ps.json', 'diff-exec-vs.json'):
        for x in load(n):
            exe[(x['bin'], x['entry'])] = x

    rows = []
    for f in sorted(SH.rglob('*.hlsl')):
        rel = str(f.relative_to(SH))
        text = f.read_text(errors='ignore')
        head = text[:2500]
        mods = []
        for b in bins_of.get(rel, []):
            tgv = sweep_all.tgv_id(b)
            for (bb, en), sm in sorted(sweep.items()):
                if bb != b:
                    continue
                x = exe.get((b, en))
                v = x['verdict'] if x else '-'
                if v == 'VACUOUS':
                    v = '空壳' if sm['target_ops'] <= 10 else 'VACUOUS'
                if x and v == 'SAME' and set(diff_common.CORE_PROFILES) <= set(x.get('crashed_profiles', [])):
                    v = 'SAME(弱:核心输入全崩溃)'
                lifted = ('TGV_%s' % tgv) in text and ('LIFTED' in text or 'PATCHED' in head)
                mods.append(dict(bin=b, variant=tgv, entry=en, stage=STAGE.get(en, en), ratio=sm['ratio'], gate=bool(sm['hard_pass']),
                                 exec=v, impl='提升' if lifted else '手写'))
        kind = ('头文件' if rel.endswith('.inc.hlsl') or '/inc/' in rel or rel.startswith('inc/') or not mods else
                '提升' if all(m['impl'] == '提升' for m in mods) else
                '手写' if all(m['impl'] == '手写' for m in mods) else '混合')
        rows.append(dict(file=rel, family=fam_of.get(rel, '(include)' if kind == '头文件' else ''), function=describe(text) or ('（原文件无说明；按路径：%s）' % rel), kind=kind,
                         modules=mods, note=NOTES.get(rel, '')))

    (ROOT / 'reconstruction/shader-source/SHADER_STATUS.json').write_text(json.dumps(rows, indent=1, ensure_ascii=False))
    allm = [m for r in rows for m in r['modules']]
    ex = Counter(m['exec'] for m in allm)
    ks = Counter(r['kind'] for r in rows)
    md = ['# 着色器重建状态总表', '',
          '由 `tools/render-passes/build_status_table.py` 从证据 JSON 汇总生成，勿手改。数据来源：`evidence/render-passes/integration/{fidelity-sweep,diff-exec,diff-exec-ps,diff-exec-vs}.json`。', '',
          '## 总览', '',
          '- HLSL 文件 %d 个：%s' % (len(rows), '，'.join('%s %d' % (k, v) for k, v in sorted(ks.items()))),
          '- 模块(bin×入口) %d 个，结构门禁通过 %d；执行验证：%s' % (len(allm), sum(m['gate'] for m in allm), '，'.join('%s %d' % (k, v) for k, v in sorted(ex.items()))),
          '', '## 列说明', '',
          '- **实现**：提升 = 由 spirv-cross 从原版 SPIR-V 提升（`batch_lift.py`，行为与原版一致，变量名是 SPIR-V 编号）；手写 = 人工重建，已通过执行验证；混合 = 一个文件里部分变体提升、部分手写；头文件 = 被包含的公共代码。',
          '- **门禁**：结构门禁（入口/local_size/绑定一致，指令比 ≥ 0.8）。',
          '- **执行**：在 lavapipe 上用同一批输入跑原版与重建版并比较所有输出。SAME 一致；EDGE 仅在极端输入不同（疑似 UB/浮点融合）；DIFF 正常输入也不同；空壳 = 原版几乎无指令；-= 未跑执行验证。',
          '- 输入：计算 0–63 号（marching_squares 到 400），像素 0–15 号（含深度附件观察 discard），顶点 0–15 号。', '']
    byfam = defaultdict(list)
    for r in rows:
        byfam[r['family'] or '(未分类)'].append(r)
    for fam in sorted(byfam):
        md += ['## %s' % fam, '', '| 文件 | 功能 | 实现 | 模块（阶段:变体 门禁/执行） | 备注 |', '|---|---|---|---|---|']
        for r in byfam[fam]:
            ms = '<br>'.join('%s:%s %s/%s' % (m['stage'], m['variant'][:8], '✓' if m['gate'] else '✗', m['exec']) for m in r['modules']) or '—'
            md.append('| `%s` | %s | %s | %s | %s |' % (r['file'], r['function'].replace('|', '/') or '—', r['kind'], ms, r['note']))
        md.append('')
    (ROOT / 'reconstruction/shader-source/SHADER_STATUS.md').write_text('\n'.join(md))
    print(len(rows), 'files', dict(ks), dict(ex))


if __name__ == '__main__':
    main()
