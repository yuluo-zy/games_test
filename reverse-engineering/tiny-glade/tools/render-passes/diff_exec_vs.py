#!/usr/bin/env python3
"""Differential execution of VERTEX shaders: oracle vs_main against the current reconstruction.

tools/spv-diff --vs rewrites both modules so every output (gl_Position + each Location output) is
bit-copied into a capture SSBO indexed by gl_VertexIndex, draws 64 random-attribute points with
rasterizer discard, and compares the captured outputs slot by slot (f32 with tolerance, ints exact).

Usage: diff_exec_vs.py [--only SUBSTR] [--self-test] [--jobs N] [--profiles 0..7] [--out FILE]
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
            if t['entry'] == 'vs_main':
                out.append((rel, b['bin'], t))
    return out


def run_one(job, src, profiles, tmp, self_test):
    rel, binn, t = job
    tag = binn.replace('.bin', '')
    res = lambda v, **k: dict(file=rel, bin=binn, entry='vs_main', verdict=v, **k)
    a = tmp / (tag + '.a.spv')
    a.write_bytes(t['payload'])
    c = tmp / (tag + '.b.spv')
    if self_test:
        c.write_bytes(t['payload'])
    else:
        defines = sweep_all.defines_for(rel, binn) + ['TGV_%s' % sweep_all.tgv_id(binn)]
        cmd = [DXC, '-E', 'vs_main', '-T', 'vs_6_4'] + CANONICAL_FLAGS + [x for d in defines for x in ('-D', d)] + ['-Fo', str(c), '-I', str(src), str(src / rel)]
        p = subprocess.run(cmd, capture_output=True, text=True)
        if p.returncode or not c.exists():
            return res('ERROR', detail='dxc: ' + (p.stderr or '')[-200:])
    lines, crashed, err = diff_common.run_profiles([BIN, '--vs', str(a), str(c), 'vs_main'], profiles, ENV)
    if err:
        return res('ERROR', detail=err)
    if lines and lines[0].startswith('SKIP'):
        return res('SKIP', detail=lines[0][5:])
    if lines and lines[0].startswith('IFACE_MISMATCH'):
        return res('DIFF', diffs=[{'name': 'interface (output set differs)', 'detail': lines[0][:300]}])
    if 'DONE' not in lines:
        return res('ERROR', detail='no result')
    rs = [dict(kv.split('=', 1) for kv in re.findall(r'\w+=\S*', l)) for l in lines if l.startswith('RES')]
    diff = [x for x in rs if int(x['mismatch']) > 0]
    cap = [x for x in rs if x['name'] == '__capture']
    cov = sum(int(x['changed_a']) for x in cap)
    verdict = (diff_common.classify(diff) if diff else None) or ('CRASH' if crashed and cov == 0 else ('SAME' if cov > 0 else 'VACUOUS'))
    ev = None
    if verdict in ('DIFF', 'EDGE') and diff_common.FMA_RECHECK and not self_test:
        cs = diff_common.strict_variant(cmd, tmp / (tag + '.strict.spv'))

        def run(x, y, profs):
            ls, _cr, e = diff_common.run_profiles([BIN, '--vs', str(x), str(y), 'vs_main'], profs, ENV)
            return None if e or 'DONE' not in ls else [dict(kv.split('=', 1) for kv in re.findall(r'\w+=\S*', l)) for l in ls if l.startswith('RES')]
        ev = diff_common.fma_recheck(a, cs, run, profiles) if cs else dict(fma_proven=None, fma_strict_diff=None, fma_control_diff=None)
        verdict = diff_common.apply_fma_verdict(verdict, ev)
    return res(verdict, crashed_profiles=crashed, coverage=cov, **(ev or {}), diff_profiles=sorted({int(x['profile']) for x in diff}), diffs=[{k: x[k] for k in ('profile', 'name', 'mismatch', 'changed_a', 'changed_b', 'first')} for x in diff[:4]])


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--only')
    ap.add_argument('--self-test', action='store_true')
    ap.add_argument('--src', default=str(ROOT / 'reconstruction/shader-source/shaders'))
    ap.add_argument('--jobs', type=int, default=8)
    ap.add_argument('--profiles', nargs='*', type=int, default=list(range(16)))
    ap.add_argument('--out', default=str(ROOT / 'evidence/render-passes/integration/diff-exec-vs.json'))
    ap.add_argument('--no-fma-check', action='store_true', help='skip the strict-arithmetic / NoContraction recheck of DIFF/EDGE modules')
    a = ap.parse_args()
    diff_common.FMA_RECHECK = not a.no_fma_check
    tmp = Path(tempfile.mkdtemp(prefix='diffvs_'))
    js = jobs(a.only)
    with ThreadPoolExecutor(a.jobs) as ex:
        res = list(ex.map(lambda j: run_one(j, Path(a.src), a.profiles, tmp, a.self_test), js))
    json.dump(res, open(a.out, 'w'), indent=1)
    from collections import Counter
    print(Counter(r['verdict'] for r in res))
    for r in res:
        if r['verdict'] in ('DIFF', 'EDGE', 'ERROR', 'SKIP'):
            print(r['verdict'], r['file'], r['bin'][-22:-4], '|', r.get('detail') or r.get('diffs'))


if __name__ == '__main__':
    main()
