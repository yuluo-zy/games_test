#!/usr/bin/env python3
"""Full-tree readable-check sweep.

For every shader file in the readable-manifest, compile the ORIGINAL source
(under shader-source/shaders) and the REWRITTEN source (under
human-readable-shaders/shaders) for every oracle variant module, strip debug
info, and require identical logic hashes — both with the original include root
(file rewrite neutrality) and with the rewritten tree include root (header
rewrites included). Headers have no own bin; they are exercised through the
consumers' full_ok checks.

Usage:
    readable_sweep.py [--out sweep.json] [--skip-missing] [--fail-fast]
Exit code: 0 if all checked modules pass.
"""
import argparse
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
MANIFEST = ROOT / 'evidence/render-passes/validation/readable-manifest.json'
OLD = ROOT / 'reconstruction/shader-source/shaders'
NEW = ROOT / 'reconstruction/human-readable-shaders/shaders'

sys.path.insert(0, str(Path(__file__).resolve().parent))
import readable_check as rcheck


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--out', default=str(ROOT / 'evidence/render-passes/validation/readable-sweep.json'))
    ap.add_argument('--skip-missing', action='store_true',
                    help='skip files that have no rewritten version yet')
    ap.add_argument('--fail-fast', action='store_true')
    args = ap.parse_args()

    manifest = json.load(open(MANIFEST))
    results = []
    total_mod = passed = 0
    for hlsl_rel, modules in sorted(manifest.items()):
        orig = OLD / hlsl_rel
        new = NEW / hlsl_rel
        if not new.exists():
            if args.skip_missing:
                continue
            results.append({'hlsl': hlsl_rel, 'ok': False, 'reason': 'missing-rewrite'})
            continue
        file_mods = []
        for m in modules:
            res = rcheck.check_module('/opt/homebrew/bin/dxc', orig, new,
                                      m['entry'], m['profile'],
                                      OLD, NEW, m['defines'],
                                      rcheck.rc.spirv_parse.Grammar(rcheck.DEFAULT_GRAMMAR))
            res['bin'] = m['bin']
            file_mods.append(res)
            total_mod += 1
            passed += 1 if res['ok'] else 0
        ok = all(r['ok'] for r in file_mods)
        results.append({'hlsl': hlsl_rel, 'ok': ok, 'modules': file_mods,
                        'modules_checked': len(file_mods),
                        'modules_pass': sum(1 for r in file_mods if r['ok']),
                        'reason': None if ok else 'logic-changed'})
    report = {'tool': 'readable_sweep.py',
              'checked_files': len(results),
              'files_pass': sum(1 for r in results if r['ok']),
              'files_fail': sum(1 for r in results if not r['ok']),
              'modules_checked': total_mod,
              'modules_pass': passed,
              'modules_fail': total_mod - passed,
              'files': results}
    json.dump(report, open(args.out, 'w'), ensure_ascii=False, indent=1)
    print('files: %d  pass %d  fail %d | modules: %d pass %d fail %d'
          % (report['checked_files'], report['files_pass'], report['files_fail'],
             total_mod, passed, total_mod - passed))
    for r in results:
        if not r['ok']:
            print('  FAIL %s (%s)' % (r['hlsl'], r.get('reason', 'logic-changed')))
            if args.fail_fast:
                sys.exit(1)
    sys.exit(0 if report['files_fail'] == 0 else 1)


if __name__ == '__main__':
    main()
