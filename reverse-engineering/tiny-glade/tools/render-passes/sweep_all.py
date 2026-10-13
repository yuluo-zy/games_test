#!/usr/bin/env python3
"""Full-tree fidelity sweep: recompile every reconstructed main against its oracle bin(s).

For each bin in skeleton/raw-per-bin.jsonl whose main_source exists in the tree, runs
recompile_check.py (parallel) and records, per module:
  - hard gate  : entry / local_size / bindings / capabilities all equal
  - fidelity   : candidate/target ratio of "real" instruction counts (debug, type,
                 constant and decoration instructions excluded)
Outputs one JSON (default evidence/render-passes/integration/fidelity-sweep.json) and a
ranked markdown table. Needs $DXC (and LD_LIBRARY_PATH for libdxcompiler.so) set.

Usage: sweep_all.py [--bins DIR] [--jobs N] [--only SUBSTR] [--out FILE]
"""
import argparse, json, os, subprocess, sys, tempfile
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]  # tiny-glade/
SRC = ROOT / 'reconstruction/shader-source/shaders'
IGNORED = {'OpString', 'OpSource', 'OpSourceContinued', 'OpLine', 'OpNoLine', 'OpName',
           'OpMemberName', 'OpModuleProcessed', 'OpExtInstImport', 'OpCapability',
           'OpExtension', 'OpEntryPoint', 'OpExecutionMode', 'OpMemoryModel', 'OpLabel',
           'OpSourceExtension'}
IGN_PREFIX = ('OpType', 'OpConstant', 'OpDecorate', 'OpMemberDecorate', 'OpSpecConstant')


def real_count(hist):
    return sum(v for k, v in hist.items() if k not in IGNORED and not k.startswith(IGN_PREFIX))


def fp_get(m, side):
    return m[side]['fingerprint']


import re as _re


def tgv_id(bin_name):
    """Variant macro id of a bin: <hash> or 0; non-cs_main entries are prefixed (cs_pass1 vs cs_pass2)."""
    m = _re.search(r'\.hlsl_(?:(.+)_)?([0-9a-f]{16}|0)\.bin$', bin_name)
    if not m:
        return '0'
    return (m.group(1) + '_' if m.group(1) and m.group(1) != 'cs_main' else '') + m.group(2)


VARIANTS = json.load(open(HERE / 'variant_defines.json'))


def defines_for(hlsl_rel, bin_name):
    out = []
    for v in VARIANTS:
        if v['hlsl'] in hlsl_rel and v['bin'] in bin_name:
            out += v['defines']
    return out


def run_one(job):
    hlsl, binp, out = job
    dfl = [x for d in defines_for(str(hlsl.relative_to(SRC)), binp.name) for x in ('-D', d)]
    dfl += ['-D', 'TGV_%s' % tgv_id(binp.name)]
    p = subprocess.run([sys.executable, str(HERE / 'recompile_check.py'), str(hlsl), str(binp),
                        '-I', str(SRC), '--out', str(out)] + dfl,
                       capture_output=True, text=True, cwd=ROOT, timeout=600)
    try:
        rep = json.load(open(out))
    except Exception:
        return {'bin': binp.name, 'hlsl': str(hlsl.relative_to(SRC)), 'error': (p.stderr or p.stdout)[-400:]}
    mods = []
    for m in rep.get('modules', []):
        row = {'entry': m.get('entry'), 'verdict': m.get('verdict')}
        try:
            t, c = fp_get(m, 'target'), fp_get(m, 'candidate')
            tc, cc = real_count(t['opcode_histogram']), real_count(c['opcode_histogram'])
            th, ch = t['opcode_histogram'], c['opcode_histogram']
            keys = [k for k in set(th) | set(ch) if k not in IGNORED and not k.startswith(IGN_PREFIX)]
            dist = sum(abs(th.get(k, 0) - ch.get(k, 0)) for k in keys) / max(1, tc)
            row.update(hist_dist=round(dist, 3), target_ops=tc, cand_ops=cc, ratio=round(cc / tc, 3) if tc else None,
                       hard_pass=all(t[k] == c[k] for k in ('entries', 'local_size', 'bindings')),
                       caps_match=t['capabilities'] == c['capabilities'])
        except Exception:
            row['note'] = m.get('verdict')
        mods.append(row)
    return {'bin': binp.name, 'hlsl': str(hlsl.relative_to(SRC)), 'modules': mods}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--bins', default=str(ROOT / 'evidence/render-passes/shaders/oracle-bin'))
    ap.add_argument('--jobs', type=int, default=os.cpu_count() or 4)
    ap.add_argument('--only')
    ap.add_argument('--out', default=str(ROOT / 'evidence/render-passes/integration/fidelity-sweep.json'))
    a = ap.parse_args()
    tmp = Path(tempfile.mkdtemp(prefix='sweep_'))
    jobs, missing = [], []
    for line in open(ROOT / 'evidence/render-passes/skeleton/raw-per-bin.jsonl'):
        b = json.loads(line)
        ms = (b.get('main_source') or '').split('shaders/', 1)[-1]
        binp = Path(a.bins) / b['bin']
        hlsl = SRC / ms
        if not binp.exists() or not hlsl.exists():
            missing.append(b['bin']); continue
        if a.only and a.only not in ms:
            continue
        jobs.append((hlsl, binp, tmp / (b['bin'] + '.json')))
    with ThreadPoolExecutor(a.jobs) as ex:
        res = list(ex.map(run_one, jobs))
    json.dump({'results': res, 'missing': missing}, open(a.out, 'w'), indent=1)
    rows = []
    for r in res:
        for m in r.get('modules', []):
            rows.append((m.get('ratio') if m.get('ratio') is not None else -1, r['hlsl'], m['entry'], r['bin'][-22:], m.get('hard_pass'), m.get('verdict')))
    rows.sort(key=lambda x: x[0])
    errs = [r for r in res if 'error' in r]
    print('jobs', len(jobs), 'missing', len(missing), 'errors', len(errs))
    for r in errs[:15]:
        print('ERR', r['hlsl'], r['bin'], r['error'].strip().splitlines()[-1][:160] if r['error'].strip() else '')
    bad = [x for x in rows if x[0] != -1 and (x[0] < 0.8 or x[0] > 1.25)]
    print('modules', len(rows), 'ratio outside [0.8,1.25]:', len(bad))
    for x in bad:
        print('%5.2f %-58s %-10s %s hard=%s' % (x[0], x[1], x[2], x[3], x[4]))


if __name__ == '__main__':
    main()
