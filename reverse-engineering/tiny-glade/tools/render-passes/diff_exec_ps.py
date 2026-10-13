#!/usr/bin/env python3
"""Differential execution of PIXEL shaders: oracle ps_main vs the current reconstruction's ps_main.

A generated vertex shader draws one full-screen triangle and feeds every PS input (by location)
the same constant values for all three vertices, so the PS sees identical, well-covered inputs
per profile. Both PS variants are rendered through the same VS/pipeline state and compared:
every colour target plus any writable SSBO / storage image (tools/spv-diff --gfx).

Verdicts: SAME / DIFF / VACUOUS (nothing written) / SKIP / ERROR - see diff_exec.py.

Usage: diff_exec_ps.py [--only SUBSTR] [--self-test] [--jobs N] [--profiles 0 1 2 3] [--out FILE]
"""
import argparse, json, os, re, subprocess, sys, tempfile
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
sys.path.insert(0, str(HERE))
import spirv_parse  # noqa: E402
import diff_common  # noqa: E402
import sweep_all  # noqa: E402
from recompile_check import parse_target_bin, CANONICAL_FLAGS  # noqa: E402

ORACLE = ROOT / 'evidence/render-passes/shaders/oracle-bin'
BIN = os.environ.get('SPV_DIFF', '/tmp/spvdiff-target/release/spv-diff')
GRAMMAR = ROOT / 'skeleton/_tools/spirv.core.grammar.json'
ENV = diff_common.with_ms(dict(os.environ, LP_NUM_THREADS='0', VK_ICD_FILENAMES=os.environ.get('VK_ICD_FILENAMES', '/usr/share/vulkan/icd.d/lvp_icd.json')))
DXC = os.environ.get('DXC', 'dxc')


def rnd(seed):
    x = seed & 0xFFFFFFFFFFFFFFFF or 1
    while True:
        x ^= (x << 13) & 0xFFFFFFFFFFFFFFFF
        x ^= x >> 7
        x ^= (x << 17) & 0xFFFFFFFFFFFFFFFF
        yield (x >> 40) / float(1 << 24)


def gen_val(profile, idx, r, r2, kind):
    """Mirror of spv-diff gen_word for the generated VS constants (float view for 'f', int view otherwise)."""
    m = profile % 16
    fl, nt = {
        0: (r, int(r * 64)), 1: (r * 64.0, int(r * 4)), 2: ((r * 4.0, int(r * 1024)) if idx % 2 == 0 else (r * 16.0, int(r * 16))),
        3: (r * 2 - 1, int(r * 16)), 4: (r * 6.0, int(r * 32)), 5: (r * 8.0, int(r * 8)),
        6: ((1.0, 1) if idx % 16 in (0, 5, 10, 15) else (0.0, 0)), 7: (0.0, 0), 8: (1.0, 1), 9: (3.0e38, 0xFFFF),
        10: ((-1 if idx % 3 == 0 else 1) * 2.0 ** (int(r2 * 9) - 4), 1 << int(r2 * 9)),
        11: ((0.0, 0) if r2 < 0.75 else (r, int(r * 8))), 12: (0.45 + r * 0.1, int(r * 2)),
        13: ((idx % 64) / 64.0, idx % 64), 14: (r * 1.0e4, int(r * 65536)),
        15: ((r * 2 - 1) * 1e-6, (int(r * 3) - 1) & 0xFFFFFFFF),
    }[m]
    return fl, nt


def gen_vs(ins, profile):
    """HLSL for a full-screen-triangle VS producing constant varyings for each PS input location."""
    r = rnd(0x9E3779B97F4A7C15 ^ (profile * 0x1234567))
    fields, assigns = [], []
    for loc, kind, n, flat in ins:
        ty = {'f': 'float4', 'u': 'uint4', 'i': 'int4'}[kind]
        fields.append('    [[vk::location(%d)]] %s%s v%d : TEXCOORD%d;' % (loc, 'nointerpolation ' if kind != 'f' else '', ty, loc, loc))
        vals = []
        for c in range(4):
            fl, nt = gen_val(profile, loc * 4 + c, next(r), next(r), kind)
            if kind == 'f':
                vals.append('%.9g' % fl if ('e' in '%.9g' % fl or '.' in '%.9g' % fl) else '%.9g.0' % fl)
            elif kind == 'u':
                vals.append('%du' % (nt & 0xFFFFFFFF))
            else:
                v = nt & 0xFFFFFFFF
                vals.append('%d' % (v - (1 << 32) if v >= 1 << 31 else v))
        assigns.append('    o.v%d = %s(%s);' % (loc, ty, ', '.join(vals)))
    return ('struct VSOut {\n    float4 pos : SV_Position;\n%s\n};\n'
            'VSOut vs_main(uint vid : SV_VertexID)\n{\n    VSOut o;\n    float2 p = float2((vid << 1) & 2, vid & 2);\n'
            '    o.pos = float4(p * 2.0f - 1.0f, 0.5f, 1.0f);\n%s\n    return o;\n}\n') % ('\n'.join(fields), '\n'.join(assigns))


def jobs(only):
    g = spirv_parse.Grammar(str(GRAMMAR))
    out = []
    for line in open(ROOT / 'evidence/render-passes/skeleton/raw-per-bin.jsonl'):
        b = json.loads(line)
        if b['stage'] == 'cs':
            continue
        rel = (b.get('main_source') or '').split('shaders/', 1)[-1]
        if only and only not in rel:
            continue
        for t in parse_target_bin(ORACLE / b['bin'], g):
            if t['entry'] == 'ps_main':
                out.append((rel, b['bin'], t))
    return out


def run_one(job, src, profiles, tmp, self_test):
    rel, binn, t = job
    tag = binn.replace('.bin', '')
    res = lambda v, **k: dict(file=rel, bin=binn, entry='ps_main', verdict=v, **k)
    a = tmp / (tag + '.a.spv')
    a.write_bytes(t['payload'])
    c = tmp / (tag + '.b.spv')
    if self_test:
        c.write_bytes(t['payload'])
    else:
        defines = sweep_all.defines_for(rel, binn) + ['TGV_%s' % sweep_all.tgv_id(binn)]
        cmd = [DXC, '-E', 'ps_main', '-T', 'ps_6_4'] + CANONICAL_FLAGS + [x for d in defines for x in ('-D', d)] + ['-Fo', str(c), '-I', str(src), str(src / rel)]
        p = subprocess.run(cmd, capture_output=True, text=True)
        if p.returncode or not c.exists():
            return res('ERROR', detail='dxc: ' + (p.stderr or '')[-200:])
    io = subprocess.run([BIN, '--reflect-io', str(a)], capture_output=True, text=True, env=ENV).stdout.splitlines()
    if io and io[0].startswith('SKIP'):
        return res('SKIP', detail=io[0][5:])
    ins = [(int(x[1]), x[2], int(x[3]), x[4] == '1') for x in (l.split() for l in io if l.startswith('IN '))]
    all_res, crashed = [], []
    for prof in profiles:
        vs_hlsl = tmp / (tag + '.vs%d.hlsl' % prof)
        vs_hlsl.write_text(gen_vs(ins, prof))
        vs = tmp / (tag + '.vs%d.spv' % prof)
        p = subprocess.run([DXC, '-E', 'vs_main', '-T', 'vs_6_4', '-spirv', '-fspv-target-env=vulkan1.1', '-HV', '2021', '-Fo', str(vs), str(vs_hlsl)], capture_output=True, text=True)
        if p.returncode:
            return res('ERROR', detail='vs gen: ' + p.stderr[-200:])
        try:
            r = subprocess.run([BIN, '--gfx', str(vs), str(a), str(c), 'ps_main', str(prof)], capture_output=True, text=True, timeout=180, env=ENV)
        except subprocess.TimeoutExpired:
            crashed.append(prof)
            continue
        lines = r.stdout.splitlines()
        if lines and lines[0].startswith('SKIP'):
            return res('SKIP', detail=lines[0][5:])
        if 'DONE' not in lines:  # lavapipe crash on this profile (e.g. out-of-range bindless index): isolate it
            crashed.append(prof)
            continue
        all_res += [dict(kv.split('=', 1) for kv in re.findall(r'\w+=\S*', l)) for l in lines if l.startswith('RES')]
    diff = [x for x in all_res if int(x['mismatch']) > 0]
    cov = sum(int(x['changed_a']) for x in all_res)
    dep = {int(x['profile']): int(x['changed_a']) for x in all_res if x.get('name') == 'depth'}  # pixels written (not discarded) per profile; 262144-ish = full screen
    verdict = (diff_common.classify(diff) if diff else None) or ('CRASH' if len(crashed) == len(profiles) else ('SAME' if cov > 0 else 'VACUOUS'))
    ev = None
    if verdict in ('DIFF', 'EDGE') and diff_common.FMA_RECHECK and not self_test:
        cs = diff_common.strict_variant(cmd, tmp / (tag + '.strict.spv'))

        def run(x, y, profs):
            rows = []
            for pr in profs:
                try:
                    r = subprocess.run([BIN, '--gfx', str(tmp / (tag + '.vs%d.spv' % pr)), str(x), str(y), 'ps_main', str(pr)], capture_output=True, text=True, timeout=180, env=ENV)
                except subprocess.TimeoutExpired:
                    continue
                if 'DONE' in r.stdout.splitlines():
                    rows += [dict(kv.split('=', 1) for kv in re.findall(r'\w+=\S*', l)) for l in r.stdout.splitlines() if l.startswith('RES')]
            return rows
        ev = diff_common.fma_recheck(a, cs, run, profiles) if cs else dict(fma_proven=None, fma_strict_diff=None, fma_control_diff=None)
        verdict = diff_common.apply_fma_verdict(verdict, ev)
    return res(verdict, crashed_profiles=crashed, coverage=cov, depth_written=dep, **(ev or {}), diff_profiles=sorted({int(x['profile']) for x in diff}), diffs=[{k: x[k] for k in ('profile', 'name', 'kind', 'mismatch', 'changed_a', 'changed_b')} for x in diff[:4]])


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--only')
    ap.add_argument('--self-test', action='store_true')
    ap.add_argument('--src', default=str(ROOT / 'reconstruction/shader-source/shaders'))
    ap.add_argument('--jobs', type=int, default=8)
    ap.add_argument('--profiles', nargs='*', type=int, default=list(range(16)))
    ap.add_argument('--out', default=str(ROOT / 'evidence/render-passes/integration/diff-exec-ps.json'))
    ap.add_argument('--no-fma-check', action='store_true', help='skip the strict-arithmetic / NoContraction recheck of DIFF/EDGE modules')
    a = ap.parse_args()
    diff_common.FMA_RECHECK = not a.no_fma_check
    tmp = Path(tempfile.mkdtemp(prefix='diffps_'))
    js = jobs(a.only)
    with ThreadPoolExecutor(a.jobs) as ex:
        res = list(ex.map(lambda j: run_one(j, Path(a.src), a.profiles, tmp, a.self_test), js))
    json.dump(res, open(a.out, 'w'), indent=1)
    from collections import Counter
    print(Counter(r['verdict'] for r in res))
    for r in res:
        if r['verdict'] in ('DIFF', 'EDGE', 'ERROR', 'SKIP', 'CRASH'):
            print(r['verdict'], r['file'], r['bin'][-22:-4], '|', r.get('detail') or r.get('diffs'))


if __name__ == '__main__':
    main()
