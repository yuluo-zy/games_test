#!/usr/bin/env python3
"""Differential execution of compute shaders: oracle bin vs the current reconstruction.

Unlike sweep_all.py (instruction counts), this actually RUNS both SPIR-V modules on lavapipe with
identical seeded random inputs (tools/spv-diff) and compares every writable output.

Verdicts per module:
  SAME          outputs equal (within float tolerance) AND the oracle wrote something (coverage > 0)
  DIFF          at least one output binding differs
  VACUOUS       neither run changed any output from its initial fill (early-out / bad inputs): no information
  SKIP          harness cannot model the module's resources (reason given)
  ERROR         crash / timeout / compile failure

Usage: diff_exec.py [--only SUBSTR] [--src DIR] [--jobs N] [--profiles 0 1 2 3] [--out FILE]
Needs: $DXC, LD_LIBRARY_PATH (dxc), VK_ICD_FILENAMES=.../lvp_icd.json, the spv-diff binary built with
  CARGO_TARGET_DIR=/tmp/spvdiff-target cargo build --offline --release  (in tools/spv-diff)
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


def jobs(only):
    g = spirv_parse.Grammar(str(GRAMMAR))
    out = []
    for line in open(ROOT / 'evidence/render-passes/skeleton/raw-per-bin.jsonl'):
        b = json.loads(line)
        if b['stage'] != 'cs':
            continue
        rel = (b.get('main_source') or '').split('shaders/', 1)[-1]
        if only and only not in rel:
            continue
        for t in parse_target_bin(ORACLE / b['bin'], g):
            out.append((rel, b['bin'], t))
    return out


def run_one(job, src, profiles, tmp, self_test=False):
    rel, binn, t = job
    tag = binn.replace('.bin', '')
    a = tmp / (tag + '.a.spv')
    a.write_bytes(t['payload'])
    c = tmp / (tag + '.b.spv')
    if self_test:
        c.write_bytes(t['payload'])
    defines = sweep_all.defines_for(rel, binn) + ['TGV_%s' % sweep_all.tgv_id(binn)]
    cmd = [os.environ.get('DXC', 'dxc'), '-E', t['entry'], '-T', t['profile']] + CANONICAL_FLAGS + \
          [x for d in defines for x in ('-D', d)] + ['-Fo', str(c), '-I', str(src), str(src / rel)]
    p = subprocess.run(cmd, capture_output=True, text=True) if not self_test else subprocess.CompletedProcess(cmd, 0, '', '')
    if p.returncode or not c.exists():
        return dict(file=rel, bin=binn, entry=t['entry'], verdict='ERROR', detail='dxc: ' + (p.stderr or '')[-200:])
    if not self_test:  # dxc >= 1.9 adds DerivativeGroupQuadsKHR to quad-op compute shaders; the original has none (see spv_strip_dgq.py)
        import spv_strip_dgq
        stripped, _n = spv_strip_dgq.strip(c.read_bytes())
        c.write_bytes(stripped)
    lines, crashed, err = diff_common.run_profiles([BIN, str(a), str(c), t['entry']], profiles, diff_common.with_ms(dict(os.environ, LP_NUM_THREADS='0', VK_ICD_FILENAMES=os.environ.get('VK_ICD_FILENAMES', '/usr/share/vulkan/icd.d/lvp_icd.json'))), timeout=25)
    if err:
        return dict(file=rel, bin=binn, entry=t['entry'], verdict='ERROR', detail=err)
    if lines and lines[0].startswith('SKIP'):
        return dict(file=rel, bin=binn, entry=t['entry'], verdict='SKIP', detail=lines[0][5:])
    if 'DONE' not in lines:
        return dict(file=rel, bin=binn, entry=t['entry'], verdict='ERROR', detail='no result')
    res = [dict(kv.split('=', 1) for kv in re.findall(r'\w+=\S*', l)) for l in lines if l.startswith('RES')]
    diff = [x for x in res if int(x['mismatch']) > 0]
    cov = sum(int(x['changed_a']) for x in res)
    verdict = (diff_common.classify(diff) if diff else None) or ('CRASH' if crashed and cov == 0 else ('SAME' if cov > 0 else 'VACUOUS'))
    if verdict == 'DIFF' and rel in diff_common.PROVEN_FMA:
        verdict = 'EDGE'
    return dict(file=rel, bin=binn, entry=t['entry'], verdict=verdict, crashed_profiles=crashed, coverage=cov,
                diff_profiles=sorted({int(x['profile']) for x in diff}), diffs=[{k: x[k] for k in ('profile', 'binding', 'name', 'mismatch', 'changed_a', 'changed_b')} for x in diff[:4]])


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--self-test', action='store_true', help='candidate = oracle itself (determinism control)')
    ap.add_argument('--only')
    ap.add_argument('--src', default=str(ROOT / 'reconstruction/shader-source/shaders'))
    ap.add_argument('--jobs', type=int, default=4)
    ap.add_argument('--profiles', nargs='*', type=int, default=list(range(16)))
    ap.add_argument('--out', default=str(ROOT / 'evidence/render-passes/integration/diff-exec.json'))
    a = ap.parse_args()
    tmp = Path(tempfile.mkdtemp(prefix='diffexec_'))
    js = jobs(a.only)
    with ThreadPoolExecutor(a.jobs) as ex:
        res = list(ex.map(lambda j: run_one(j, Path(a.src), a.profiles, tmp, a.self_test), js))
    json.dump(res, open(a.out, 'w'), indent=1)
    from collections import Counter
    print(Counter(r['verdict'] for r in res))
    for r in res:
        if r['verdict'] in ('DIFF', 'EDGE', 'ERROR', 'SKIP'):
            print(r['verdict'], r['file'], r['bin'][-22:], '|', r.get('detail') or r.get('diffs'))


if __name__ == '__main__':
    main()
