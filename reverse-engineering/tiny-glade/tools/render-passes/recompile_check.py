#!/usr/bin/env python3
"""Recompile a reconstructed HLSL shader and validate it against the original.

Tiny Glade ships 404 precompiled SPIR-V shaders under the distribution's
`compiled-shaders/`. Every module embeds its exact dxc invocation as the string
`dxc-cl-option: ...` plus, via `-Zi -Qembed_debug`, the source/include paths it
was built from. A shader is recompiled with the SAME dxc command and the result
is compared to the original module STRUCTURALLY, plus with a NORMALIZED sha256.

IMPORTANT (why raw hashes are wrong):
    dxc `-Zi -Qembed_debug` embeds the input file path and the temp output path
    into the SPIR-V module. Recompiling the same source from a different path
    therefore yields different bytes, and the original modules embed the paths
    used by the game's build (`shaders/...`). A byte-for-byte raw hash will
    differ even for a perfect reconstruction. The correct comparison is:
      1. structural fingerprint  (entry + stage, LocalSize, capability set,
         descriptor bindings set/binding/storage/name, opcode histogram)
      2. normalized hash         (source/temp path strings stripped)
    Never chase a phantom mismatch caused by raw-hash comparison alone.

Usage:
    recompile_check.py <reconstructed.hlsl> <target.bin>
        [--entry NAME]              only check modules whose entry matches NAME
        [--grammar PATH]            SPIR-V grammar json (default:
                                    skeleton/_tools/spirv.core.grammar.json)
        [-I DIR]                    include root (default:
                                    reconstruction/shader-source/shaders)
        [--dxc PATH]                dxc binary (default: $DXC or `dxc`)
        [--out FILE]                write JSON report
        [--keep]                    keep temp artifacts in ./recompile_tmp
        [--verbose]

Exit code: 0 if every checked module passes structurally, 1 otherwise.

Reconstructed source layout expectation:
    reconstruction/shader-source/shaders/<...>.{cs,raster}.hlsl
    reconstruction/shader-source/shaders/inc/<...>.hlsl
  i.e. the include root given to -I is `reconstruction/shader-source/shaders`,
  so the original `#include "inc/common.hlsl"` resolves as-is.
"""

import argparse
import hashlib
import json
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

# Local helpers (this directory). spirv_parse reuses the parsing logic from
# scripts/render-passes/parse_shader_catalog.py.
sys.path.insert(0, str(Path(__file__).resolve().parent))
import spirv_parse  # noqa: E402

DEFAULT_GRAMMAR = Path(__file__).resolve().parents[2] / 'skeleton' / '_tools' / 'spirv.core.grammar.json'
DEFAULT_INCLUDE_ROOT = Path('reconstruction/shader-source/shaders')

# Canonical flag set observed on every one of the 638 original module commands.
CANONICAL_FLAGS = ['-spirv', '-WX', '-Ges', '-fspv-target-env=vulkan1.1',
                   '-HV', '2021', '-Zi', '-Qembed_debug']

def normalized_sha256(payload):
    """sha256 of a module with source-path, temp-path and dxc-command strings
    stripped (via instruction-level canonicalization, see spirv_parse.canonicalize_module),
    so equal source/equal dxc/equal flags compare equal."""
    return hashlib.sha256(spirv_parse.canonicalize_module(payload)).hexdigest()


def run_dxc(dxc, hlsl_path, entry, profile, include_root, out_spv, keep_dir=None):
    """Run dxc with the canonical command and return (returncode, stderr)."""
    cmd = [str(dxc), '-E', entry, '-T', profile] + CANONICAL_FLAGS + \
          ['-Fo', str(out_spv), '-I', str(include_root), str(hlsl_path)]
    if keep_dir:
        cmd.append('-Fc')  # also dump disassembly for inspection when --keep
    try:
        proc = subprocess.run(cmd, capture_output=True, timeout=300)
    except FileNotFoundError:
        sys.exit('dxc not found at %s (set $DXC or use --dxc)' % dxc)
    return proc.returncode, proc.stderr.decode('utf-8', 'replace')


def compile_command_from_args(entry, profile, include_root):
    """Human-readable command string used (for the report / replay)."""
    return ('dxc -E %s -T %s %s -Fo <out> -I %s <source>' %
            (entry, profile, ' '.join(CANONICAL_FLAGS), include_root))


def compare_fingerprints(target_fp, cand_fp):
    """Return (matched, differences) for the structural fingerprint categories."""
    diffs = {}
    matched = True
    for cat in ('entries', 'local_size', 'capabilities', 'bindings', 'opcode_histogram'):
        t = target_fp[cat]
        c = cand_fp[cat]
        if t == c:
            continue
        matched = False
        if cat == 'opcode_histogram':
            tc = dict(t)
            cc = dict(c)
            missing = {k: v for k, v in tc.items() if cc.get(k) != v}
            extra = {k: v for k, v in cc.items() if tc.get(k) != v}
            diffs[cat] = {'target_only': missing, 'candidate_only': extra}
        elif cat == 'bindings':
            diffs[cat] = {'target_only': [b for b in t if b not in c],
                          'candidate_only': [b for b in c if b not in t]}
        elif cat == 'entries':
            diffs[cat] = {'target_only': [e for e in t if e not in c],
                          'candidate_only': [e for e in c if e not in t]}
        else:
            diffs[cat] = {'target': t, 'candidate': c}
    return matched, diffs


def compile_command_from_raw(raw):
    """Parse a `dxc-cl-option: ...` string into structured args.

    Returns {entry, profile, input_temp, output_temp, flags}. The first bare
    token is the original input path (a temp path in the shipped bins).
    """
    body = raw.split('dxc-cl-option:', 1)[1].strip() if raw and 'dxc-cl-option:' in raw else (raw or '').strip()
    toks = body.split()
    out = {'entry': None, 'profile': None, 'input_temp': None,
           'output_temp': None, 'flags': []}
    key = {'E': 'entry', 'T': 'profile', 'Fo': 'output_temp'}
    i = 0
    while i < len(toks):
        t = toks[i]
        if t in ('-E', '-T', '-Fo'):
            if i + 1 < len(toks):
                out[key[t[1:]]] = toks[i + 1]
            i += 2
        elif t == '-HV':
            out['flags'].append(t + ' ' + (toks[i + 1] if i + 1 < len(toks) else ''))
            i += 2
        elif t.startswith('-'):
            out['flags'].append(t)
            i += 1
        else:
            if out['input_temp'] is None:
                out['input_temp'] = t
            i += 1
    return out


def parse_target_bin(bin_path, grammar):
    blob = bin_path.read_bytes()
    mods = []
    for offset, payload in spirv_parse.iter_modules(blob):
        module = spirv_parse.parse_module(payload, grammar)
        if module is None:
            continue
        # recover the module's own dxc command (-E/-T) from the embedded string
        cmd = None
        at = payload.find(b'dxc-cl-option')
        if at >= 0:
            end = payload.find(b'\x00', at)
            cmd = payload[at:end].decode('utf-8', 'replace')
        cmd_args = compile_command_from_raw(cmd) if cmd else {}
        entry = cmd_args.get('entry') or (module['entries'][0]['name'] if module['entries'] else None)
        profile = cmd_args.get('profile')
        mods.append({
            'offset': offset,
            'entry': entry,
            'profile': profile,
            'command': cmd,
            'module': module,
            'payload': payload,
            'normalized_sha256': normalized_sha256(payload),
            'raw_sha256': spirv_parse.module_sha256(payload),
        })
    return mods


def format_binding(b):
    set_, binding, storage, name, kind = b
    return 'set=%s binding=%s storage=%s name=%s kind=%s' % (set_, binding, storage, name, kind)


def main():
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('hlsl', help='reconstructed HLSL source')
    ap.add_argument('bin', help='original .bin target')
    ap.add_argument('--entry', help='only check modules whose entry equals this')
    ap.add_argument('--grammar', default=str(DEFAULT_GRAMMAR))
    ap.add_argument('-I', dest='include_root', default=str(DEFAULT_INCLUDE_ROOT))
    ap.add_argument('--dxc', default=os.environ.get('DXC', shutil.which('dxc') or 'dxc'))
    ap.add_argument('--out', help='write JSON report to this path')
    ap.add_argument('--keep', action='store_true')
    ap.add_argument('--verbose', action='store_true')
    args = ap.parse_args()

    hlsl_path = Path(args.hlsl)
    bin_path = Path(args.bin)
    include_root = Path(args.include_root)
    if not hlsl_path.exists():
        sys.exit('reconstructed source not found: %s' % hlsl_path)
    if not bin_path.exists():
        sys.exit('target bin not found: %s' % bin_path)
    if not include_root.exists():
        sys.exit('include root not found: %s (see pipeline doc for layout)' % include_root)

    grammar = spirv_parse.Grammar(args.grammar)

    targets = parse_target_bin(bin_path, grammar)
    if not targets:
        sys.exit('no SPIR-V modules found in %s' % bin_path)
    if args.entry:
        targets = [t for t in targets if t['entry'] == args.entry]
        if not targets:
            sys.exit('no module with entry %s in %s' % (args.entry, bin_path.name))

    tempdir = Path(tempfile.mkdtemp(prefix='recompile_'))
    keep_dir = Path('recompile_tmp') if args.keep else None
    if keep_dir:
        keep_dir.mkdir(exist_ok=True)

    results = []
    any_fail = False
    for t in targets:
        out_spv = tempdir / ('module%d.spv' % t['offset'])
        rc, stderr = run_dxc(args.dxc, hlsl_path, t['entry'], t['profile'],
                             include_root, out_spv, keep_dir=keep_dir)
        if rc != 0:
            results.append({
                'module_index': t['offset'],
                'entry': t['entry'],
                'profile': t['profile'],
                'command': t['command'],
                'dxc_returncode': rc,
                'dxc_stderr': stderr.strip(),
                'verdict': 'FAIL_DXC',
            })
            any_fail = True
            continue

        payload = out_spv.read_bytes()
        cand_module = spirv_parse.parse_module(payload, grammar)
        if cand_module is None:
            results.append({'module_index': t['offset'], 'entry': t['entry'],
                            'verdict': 'FAIL_PARSE', 'detail': 'produced output is not SPIR-V'})
            any_fail = True
            continue

        target_fp = spirv_parse.structural_fingerprint(t['module'])
        cand_fp = spirv_parse.structural_fingerprint(cand_module)
        structural_ok, diffs = compare_fingerprints(target_fp, cand_fp)
        cand_norm = normalized_sha256(payload)
        norm_ok = cand_norm == t['normalized_sha256']

        if not structural_ok:
            verdict = 'FAIL'
            any_fail = True
        elif norm_ok:
            verdict = 'PASS_NORMALIZED'
        else:
            verdict = 'PASS_STRUCTURAL'

        results.append({
            'module_index': t['offset'],
            'entry': t['entry'],
            'profile': t['profile'],
            'command': t['command'],
            'replay_command': compile_command_from_args(t['entry'], t['profile'], include_root),
            'target': {
                'raw_sha256': t['raw_sha256'],
                'normalized_sha256': t['normalized_sha256'],
                'fingerprint': target_fp,
            },
            'candidate': {
                'raw_sha256': spirv_parse.module_sha256(payload),
                'normalized_sha256': cand_norm,
                'fingerprint': cand_fp,
            },
            'structural_match': structural_ok,
            'normalized_hash_match': norm_ok,
            'differences': diffs,
            'verdict': verdict,
        })

        if args.verbose or verdict != 'PASS_NORMALIZED':
            print('--- module %s  entry=%s profile=%s' % (t['offset'], t['entry'], t['profile']))
            print('    raw sha target  :', t['raw_sha256'])
            print('    raw sha candidate:', spirv_parse.module_sha256(payload))
            print('    normalized target :', t['normalized_sha256'])
            print('    normalized cand   :', cand_norm)
            print('    structural match  :', structural_ok, ' normalized match:', norm_ok)
            if diffs:
                for cat, d in diffs.items():
                    print('    diff[%s]:' % cat)
                    if isinstance(d, dict) and 'target_only' in d:
                        if cat == 'bindings':
                            for b in d['target_only']:
                                print('       -', format_binding(b))
                            for b in d['candidate_only']:
                                print('       +', format_binding(b))
                        else:
                            print('       target_only  :', d['target_only'])
                            print('       candidate_only:', d['candidate_only'])
                    else:
                        print('       ', d)

    if args.out:
        Path(args.out).parent.mkdir(parents=True, exist_ok=True)
        Path(args.out).write_text(json.dumps({
            'tool': 'recompile_check.py',
            'hlsl': str(hlsl_path),
            'bin': str(bin_path),
            'include_root': str(include_root),
            'dxc': args.dxc,
            'modules': results,
            'summary': {
                'checked': len(results),
                'pass_normalized': sum(1 for r in results if r['verdict'] == 'PASS_NORMALIZED'),
                'pass_structural': sum(1 for r in results if r['verdict'] == 'PASS_STRUCTURAL'),
                'fail': sum(1 for r in results if r['verdict'] in ('FAIL', 'FAIL_DXC', 'FAIL_PARSE')),
            },
        }, indent=2), encoding='utf-8')
        print('wrote', args.out)

    print('VERDICT:', 'PASS' if not any_fail else 'FAIL',
          '(structural', 'PASS' if not any_fail else 'FAIL', ';',
          'normalized', sum(1 for r in results if r['verdict'] == 'PASS_NORMALIZED'),
          '/', len(results), 'identical)')
    sys.exit(0 if not any_fail else 1)


if __name__ == '__main__':
    main()
