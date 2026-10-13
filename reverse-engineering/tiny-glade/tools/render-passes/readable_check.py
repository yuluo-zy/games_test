#!/usr/bin/env python3
"""Validate a human-readable HLSL rewrite against its original transpiled source.

The core "no logic change" guarantee: compiling the ORIGINAL source and the
REWRITTEN source with the identical dxc command (same entry/profile/defines)
must yield byte-identical SPIR-V after normalization. Renaming variables and
adding comments never changes codegen; if this check fails the rewrite altered
arithmetic or control flow.

Usage:
    readable_check.py --orig <original.hlsl> --new <rewritten.hlsl> \
        --bin <oracle.bin> [-I <include_root>] [-D TGV_x]... [--entry NAME]
        [--out REPORT.json]

Also accepts --manifest MANIFEST.json --hlsl <key> to check every module of a
shader against all of its oracle bins at once:
    readable_check.py --manifest readable-manifest.json --hlsl gauss_blur.cs.hlsl

The include root passed with -I is used for BOTH compiles, so a header-rewrite
mismatch (rewritten tree) shows up against the original tree as well.
Exit code: 0 if every checked module matches, 1 otherwise.
"""
import argparse
import hashlib
import json
import struct
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import recompile_check as rc
import spirv_parse  # noqa: F401

DEFAULT_GRAMMAR = Path(__file__).resolve().parents[2] / 'skeleton' / '_tools' / 'spirv.core.grammar.json'
DEFAULT_INCLUDE_ROOT = Path('reconstruction/shader-source/shaders')

# Debug / source-info opcodes that carry path strings, line numbers or names.
# They are emitted under `-Zi -Qembed_debug` and change whenever comments shift
# lines or paths differ, so they are stripped before hashing to get a "pure
# logic" fingerprint: renaming variables and adding Chinese comments must never
# alter the result.
_DEBUG_OPS = {0, 2, 3, 4, 5, 6, 7, 8, 317, 330, 1113}  # Nop/Source*/Name*/String/Line/NoLine/ModuleProcessed/DxcDebugString


def strip_debug(payload):
    """Re-encode a module without debug/source-info instructions."""
    words = list(struct.unpack('<' + 'I' * (len(payload) // 4), payload))
    n = len(words)
    if n == 0 or words[0] != 0x07230203:
        return payload
    out = words[:5]
    i = 5
    while i < n:
        wc = words[i] >> 16
        op = words[i] & 0xffff
        if wc == 0 or i + wc > n:
            break
        if op not in _DEBUG_OPS:
            out.extend(words[i:i + wc])
        i += wc
    return struct.pack('<' + 'I' * len(out), *out)


def logic_hash(payload):
    """sha256 of a module stripped of all debug info -> logic-only identity."""
    return hashlib.sha256(strip_debug(payload)).hexdigest()


def compile_hash(dxc, hlsl_path, entry, profile, include_root, defines):
    with tempfile.TemporaryDirectory(prefix='readable_') as td:
        out_spv = Path(td) / 'out.spv'
        ret, err = rc.run_dxc(dxc, hlsl_path, entry, profile, include_root, out_spv, defines=defines)
        if ret != 0:
            return None, err
        payload = out_spv.read_bytes()
        # Single-module blob: hash the (sole) module payload, debug stripped.
        hashes = [logic_hash(p) for _, p in spirv_parse.iter_modules(payload)]
        return hashes[0] if hashes else None, err


def check_module(dxc, orig, new, entry, profile, include_root, new_include_root, defines, grammar):
    ha, ea = compile_hash(dxc, orig, entry, profile, include_root, defines)
    hb, eb = compile_hash(dxc, new, entry, profile, include_root, defines)
    if ha is None or hb is None:
        return {'entry': entry, 'ok': False, 'reason': 'dxc-error',
                'orig_stderr': (ea or '')[:300], 'new_stderr': (eb or '')[:300]}
    file_ok = ha == hb  # rewrite of the file body alone must not change logic
    res = {'entry': entry, 'ok': file_ok, 'file_ok': file_ok,
           'orig_hash': ha, 'new_hash@origroot': hb}
    if new_include_root and str(new_include_root) != str(include_root):
        hc, ec = compile_hash(dxc, new, entry, profile, new_include_root, defines)
        full_ok = ha == hc  # whole rewritten tree (incl. rewritten headers) must not change logic
        res.update({'full_ok': full_ok, 'new_hash@newroot': hc,
                    'new_stderr@newroot': (ec or '')[:200],
                    'ok': file_ok and full_ok,
                    'reason': None if (file_ok and full_ok) else 'logic-changed'})
        if file_ok and not full_ok:
            res['reason'] = 'header-rewrite-changed-logic'
        if not file_ok:
            res['reason'] = 'file-rewrite-changed-logic'
    return res


def main():
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument('--orig', help='original HLSL source')
    ap.add_argument('--new', help='rewritten HLSL source')
    ap.add_argument('--bin', help='oracle .bin (for entry/profile recovery)')
    ap.add_argument('--manifest', help='readable-manifest.json')
    ap.add_argument('--hlsl', help='manifest key (path relative to shaders/)')
    ap.add_argument('--entry', help='only check modules whose entry equals this')
    ap.add_argument('-I', dest='include_root', default=str(DEFAULT_INCLUDE_ROOT),
                    help='include root for the ORIGINAL source (default: shader-source/shaders)')
    ap.add_argument('--new-include-root', dest='new_include_root', default=None,
                    help='include root for the REWRITTEN source (default: same as -I). '
                         'When the rewritten tree has its own headers, pass the new tree '
                         'root here so header rewrites are exercised too.')
    ap.add_argument('-D', '--define', action='append', default=[], help='extra -D passed to dxc')
    ap.add_argument('--grammar', default=str(DEFAULT_GRAMMAR))
    ap.add_argument('--dxc', default='/opt/homebrew/bin/dxc')
    ap.add_argument('--out', help='write JSON report to this path')
    args = ap.parse_args()

    include_root = Path(args.include_root)
    new_include_root = Path(args.new_include_root) if args.new_include_root else include_root
    grammar = rc.spirv_parse.Grammar(args.grammar)
    results = []
    errors = []

    if args.manifest:
        manifest = json.load(open(args.manifest))
        if args.hlsl not in manifest:
            sys.exit('hlsl key %r not in manifest' % args.hlsl)
        modules = manifest[args.hlsl]
        for m in modules:
            if args.entry and m['entry'] != args.entry:
                continue
            defines = args.define + m['defines']
            res = check_module(args.dxc, args.orig, args.new,
                               m['entry'], m['profile'], include_root, new_include_root, defines, grammar)
            res['bin'] = m['bin']
            res['defines'] = defines
            results.append(res)
    else:
        if not (args.orig and args.new):
            sys.exit('need --orig --new (and --bin for entry/profile)')
        bin_path = Path(args.bin)
        targets = rc.parse_target_bin(bin_path, grammar)
        for t in targets:
            if args.entry and t['entry'] != args.entry:
                continue
            res = check_module(args.dxc, args.orig, args.new,
                               t['entry'], t['profile'], include_root, new_include_root, args.define, grammar)
            res['bin'] = bin_path.name
            res['defines'] = args.define
            results.append(res)

    ok = all(r['ok'] for r in results)
    report = {'tool': 'readable_check.py',
              'orig': str(args.orig), 'new': str(args.new),
              'include_root': str(include_root),
              'new_include_root': str(new_include_root),
              'dxc': args.dxc,
              'modules': results,
              'summary': {'checked': len(results),
                          'pass': sum(1 for r in results if r['ok']),
                          'fail': sum(1 for r in results if not r['ok']),
                          'file_ok': sum(1 for r in results if r.get('file_ok') is True),
                          'full_ok': sum(1 for r in results if r.get('full_ok') is True),
                          'verdict': 'PASS' if ok and results else 'FAIL'}}
    out = args.out or 'readable_check.json'
    json.dump(report, open(out, 'w'), ensure_ascii=False, indent=1)
    print('summary:', report['summary'])
    for r in results:
        if not r['ok']:
            print('  FAIL %s [%s] defines=%s reason=%s' % (r.get('bin', '?'), r['entry'], r.get('defines'), r.get('reason', 'hash-mismatch')))
    sys.exit(0 if ok else 1)


if __name__ == '__main__':
    main()
