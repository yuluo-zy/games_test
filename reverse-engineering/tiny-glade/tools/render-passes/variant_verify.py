#!/usr/bin/env python3
"""Verify -D variant bins (no-hzb / raster-no-color) that recompile_check cannot
run (no -D passthrough): manual dxc compile with the define + structural fingerprint
comparison per target module. Writes one JSON report per (hlsl, bin, define)."""
import json
import subprocess
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import spirv_parse

ROOT = Path('/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade')
SRC = ROOT / 'reconstruction/shader-source/shaders'
ORACLE = Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/compiled-shaders')
DXC = '/opt/homebrew/bin/dxc'
GRAMMAR = spirv_parse.Grammar(str(ROOT / 'skeleton' / '_tools' / 'spirv.core.grammar.json'))
CANON = ['-spirv', '-WX', '-Ges', '-fspv-target-env=vulkan1.1', '-HV', '2021', '-Zi', '-Qembed_debug']

HARDGATE = ('entries', 'local_size', 'capabilities', 'bindings')


def compile_one(hlsl, entry, profile, define, out_spv):
    cmd = [DXC, '-E', entry, '-T', profile] + CANON + ['-D', define,
           '-Fo', str(out_spv), '-I', str(SRC), str(hlsl)]
    p = subprocess.run(cmd, capture_output=True, timeout=300)
    return p.returncode, p.stderr.decode('utf-8', 'replace')


def fp_compare(t, c):
    diffs = {}
    for cat in ('entries', 'local_size', 'capabilities', 'bindings', 'opcode_histogram'):
        if t[cat] != c[cat]:
            if cat == 'opcode_histogram':
                tc, cc = dict(t[cat]), dict(c[cat])
                diffs[cat] = {'target_only': {k: v for k, v in tc.items() if cc.get(k) != v},
                              'candidate_only': {k: v for k, v in cc.items() if tc.get(k) != v}}
            else:
                diffs[cat] = {'target': t[cat], 'candidate': c[cat]}
    return diffs


def parse_bin(bin_path):
    mods = []
    blob = bin_path.read_bytes()
    for offset, payload in spirv_parse.iter_modules(blob):
        m = spirv_parse.parse_module(payload, GRAMMAR)
        cmd = None
        at = payload.find(b'dxc-cl-option')
        if at >= 0:
            end = payload.find(b'\x00', at)
            cmd = payload[at:end].decode('utf-8', 'replace')
        mods.append({'offset': offset, 'entry': m['entries'][0]['name'],
                     'profile': _profile_from_cmd(cmd), 'fp': spirv_parse.structural_fingerprint(m)})
    return mods


def _profile_from_cmd(cmd):
    if not cmd:
        return None
    toks = cmd.split()
    for i, t in enumerate(toks):
        if t == '-T' and i + 1 < len(toks):
            return toks[i + 1]
    return None


def main():
    jobs = json.loads(Path(sys.argv[1]).read_text())
    out_dir = Path(sys.argv[2])
    out_dir.mkdir(parents=True, exist_ok=True)
    report = {'tool': 'variant-verify', 'jobs': [], 'summary': {}}
    hg_pass = 0
    hg_fail = 0
    for job in jobs:
        hlsl = SRC / job['hlsl']
        binp = ORACLE / job['bin']
        define = job['define']
        targets = parse_bin(binp)
        entry_report = []
        for t in targets:
            with tempfile.TemporaryDirectory(prefix='varcheck_') as td:
                out_spv = Path(td) / 'mod.spv'
                rc, err = compile_one(hlsl, t['entry'], t['profile'], define, out_spv)
                if rc != 0:
                    entry_report.append({'entry': t['entry'], 'profile': t['profile'],
                                         'verdict': 'FAIL_DXC', 'stderr': err.strip()})
                    hg_fail += 1
                    continue
                c_fp = spirv_parse.structural_fingerprint(
                    spirv_parse.parse_module(out_spv.read_bytes(), GRAMMAR))
                diffs = fp_compare(t['fp'], c_fp)
                hg = [c for c in diffs if c in HARDGATE]
                if not hg:
                    hg_pass += 1
                else:
                    hg_fail += 1
                entry_report.append({'entry': t['entry'], 'profile': t['profile'],
                                     'hard_gate': 'PASS' if not hg else 'FAIL(' + ','.join(hg) + ')',
                                     'hist_residual': {c: len(diffs[c]['target_only']) if isinstance(diffs[c], dict) and 'target_only' in diffs[c] else 0 for c in ('opcode_histogram',)},
                                     'diff_categories': list(diffs.keys())})
        row = {'hlsl': job['hlsl'], 'bin': job['bin'], 'define': define, 'modules': entry_report}
        report['jobs'].append(row)
        print(row)
    report['summary'] = {'hard_gate_pass': hg_pass, 'hard_gate_fail': hg_fail,
                         'total_modules': hg_pass + hg_fail}
    out = out_dir / 'variant-verify.json'
    out.write_text(json.dumps(report, indent=2))
    print('wrote', out)
    print('hard_gate PASS:', hg_pass, ' FAIL:', hg_fail)


if __name__ == '__main__':
    main()
