#!/usr/bin/env python3
"""Full-tree compile: compile every non-inc reconstructed main .hlsl in the
shaders/ tree with its canonical dxc command (entry/profile per module from
compile-commands.json). Reports per-file rc and first dxc error."""
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path('reconstruction/shader-source/shaders')
DXC = '/opt/homebrew/bin/dxc'
FLAGS = ['-spirv', '-WX', '-Ges', '-fspv-target-env=vulkan1.1', '-HV', '2021', '-Zi', '-Qembed_debug']

cc = json.load(open('evidence/render-passes/validation/compile-commands.json'))
cmds = cc['compile_commands']

# main_source_path -> list of (binfile, entry, profile)
by_main = {}
for c in cmds:
    main = c['main_source_path']
    for m in c['modules']:
        by_main.setdefault(main, []).append((c['file'], m['entry'], m['profile']))


def is_main(rel):
    p = ROOT / rel
    if rel.startswith('inc/'):
        return False
    if '/inc/' in '/' + rel:
        return False
    if rel.endswith('.inc.hlsl'):
        return False
    if not rel.endswith('.hlsl'):
        return False
    return p.exists()


# Iterate only reconstructed files that ARE mains in compile-commands
results = []
for key in sorted(by_main.keys()):
    rel = key.replace('shaders/', '', 1)
    if not is_main(rel):
        continue
    src = ROOT / rel
    entries = by_main.get(key)
    seen = set()
    mod_results = []
    ok = fail = 0
    for binfile, entry, profile in entries:
        if (entry, profile) in seen:
            continue
        seen.add((entry, profile))
        out = '/tmp/tree_build.spv'
        try:
            proc = subprocess.run(
                [DXC, '-E', entry, '-T', profile] + FLAGS + ['-Fo', out, '-I', str(ROOT), str(src)],
                capture_output=True, timeout=300)
        except FileNotFoundError:
            sys.exit('dxc not found')
        if proc.returncode == 0:
            ok += 1
            mod_results.append({'entry': entry, 'profile': profile, 'rc': 0})
        else:
            fail += 1
            err = proc.stderr.decode('utf-8', 'replace').strip()
            mod_results.append({'entry': entry, 'profile': profile, 'rc': proc.returncode,
                                'first_error': err[:600] if err else '(no stderr)'})
    results.append({'file': rel, 'rc': 0 if fail == 0 else 1, 'detail': None,
                    'modules_checked': len(mod_results), 'pass': ok, 'fail': fail,
                    'modules': mod_results})

pass_files = sum(1 for r in results if r['rc'] == 0)
fail_files = [r for r in results if r['rc'] == 1]
total = len(results)
print(f'=== TREE BUILD: mains={total} pass_files={pass_files} fail_files={len(fail_files)}')
for r in sorted(fail_files, key=lambda x: x['file']):
    err = ''
    for m in r.get('modules', []):
        if m.get('rc', 0) != 0:
            err = m.get('first_error', '')
            break
    print(f'FAIL {r["file"]}: {err[:200]}')

out = {
    'tool': 'tree_build_compile',
    'dxc': DXC,
    'date': '2026-10-13',
    'total_mains': total,
    'pass_files': pass_files,
    'fail_files': len(fail_files),
    'results': results,
}
Path('evidence/render-passes/integration/tree-build-report.json').parent.mkdir(parents=True, exist_ok=True)
Path('evidence/render-passes/integration/tree-build-report.json').write_text(json.dumps(out, indent=1))
print('wrote evidence/render-passes/integration/tree-build-report.json')
