#!/usr/bin/env python3
"""X1: translate the production-lowered SPIR-V to MSL with SPIRV-Cross and report.

Input is the directory written by the x1_lower_all example (one sub-directory per
pipeline, `lowered-<i>.spv` per stage, `meta.json`). Each stage is translated once per
profile; results record success, normalised failure reasons, and static MSL checks
against Metal's per-stage binding limits. With `--compile` (macOS, Xcode installed) the
MSL is also compiled with the Metal compiler, optionally with fast math off (D25).

    python3 tools/metal_experiments/x1/x1_msl_translate.py <lowered_dir> \
        --spirv-cross ~/tools/spirv-cross/spirv-cross --out <result_dir> \
        [--profiles plain,argbuf-tier2] [--platform macos|ios] [--compile] [--no-fast-math]

SPIRV-Cross is not vendored; build it with build_spirv_cross.sh (pinned tag) or point
--spirv-cross at your own build. The result records the binary's version string.
"""
import argparse
import collections
import concurrent.futures
import glob
import json
import os
import re
import shutil
import subprocess
import sys

PROFILES = {
    # No argument buffers: every resource gets a plain [[buffer/texture/sampler(n)]] slot.
    'plain': [],
    # Argument buffers, tier 2 (device-addressable resource arrays), what D8 proposes.
    'argbuf-tier2': ['--msl-argument-buffers', '--msl-argument-buffer-tier', '1'],
    # Tier 1, for devices without tier 2 argument buffers.
    'argbuf-tier1': ['--msl-argument-buffers', '--msl-argument-buffer-tier', '0'],
}
# Metal per-stage limits that matter here (Apple GPUs; verify against the device in X5).
LIMITS = {'buffer': 31, 'texture': 128, 'sampler': 16}
MSL_VERSION = '30000'  # 3.0.0, the minimum implied by D14 (Metal 3)


def normalise(message):
    """Collapse ids and numbers so equal failure reasons group together."""
    first = next((line for line in message.splitlines() if line.strip()), message).strip()
    first = re.sub(r'%\d+', '%N', first)
    first = re.sub(r'0x[0-9a-fA-F]+|\d+', 'N', first)  # one pass: hex must not be re-matched as digits
    return first[:200]


def static_checks(msl):
    """Highest slot index per resource class used by the entry point signature."""
    found = {}
    for kind in LIMITS:
        indices = [int(m) for m in re.findall(r'\[\[' + kind + r'\((\d+)\)\]\]', msl)]
        found[kind] = (max(indices) + 1) if indices else 0
    return found


def spirv_cross_version(binary):
    try:
        proc = subprocess.run([binary, '--revision'], capture_output=True, text=True, timeout=30)
        return (proc.stdout or proc.stderr).strip()
    except (OSError, subprocess.SubprocessError) as error:
        return f'unknown ({error})'


def translate_one(binary, spv, out_path, profile, platform, extra):
    cmd = [binary, spv, '--msl', '--msl-version', MSL_VERSION, '--output', out_path]
    if platform == 'ios':
        cmd.append('--msl-ios')
    cmd += PROFILES[profile] + extra
    proc = subprocess.run(cmd, capture_output=True, text=True, timeout=300)
    return proc.returncode, (proc.stderr or proc.stdout)


def compile_msl(path, platform, no_fast_math):
    sdk = 'iphoneos' if platform == 'ios' else 'macosx'
    cmd = ['xcrun', '-sdk', sdk, 'metal', '-c', path, '-o', os.devnull]
    if no_fast_math:
        cmd.append('-fno-fast-math')
    proc = subprocess.run(cmd, capture_output=True, text=True, timeout=600)
    return proc.returncode, proc.stderr


def stages_of(directory):
    for meta_path in sorted(glob.glob(os.path.join(directory, '*', 'meta.json'))):
        with open(meta_path, encoding='utf-8') as handle:
            meta = json.load(handle)
        base = os.path.dirname(meta_path)
        for stage in meta['stages']:
            spv = os.path.join(base, f"lowered-{stage['index']}.spv")
            if os.path.isfile(spv):
                yield meta, stage, spv


def run(args):
    binary = args.spirv_cross
    version = spirv_cross_version(binary)
    do_compile = args.compile and shutil.which('xcrun') is not None
    results = []
    work = []
    for meta, stage, spv in stages_of(args.directory):
        if args.filter and args.filter not in meta['file']:
            continue
        for profile in args.profiles:
            out_dir = os.path.join(args.out, profile, meta['bin'])
            os.makedirs(out_dir, exist_ok=True)
            work.append((meta, stage, spv, profile, os.path.join(out_dir, f"{stage['index']}.metal")))

    def job(item):
        meta, stage, spv, profile, out_path = item
        row = {'file': meta['file'], 'bin': meta['bin'], 'stage': stage['stage'], 'profile': profile}
        try:
            code, message = translate_one(binary, spv, out_path, profile, args.platform, args.extra)
        except subprocess.TimeoutExpired:
            row.update(status='timeout', reason='timeout')
            return row
        if code != 0 or not os.path.isfile(out_path):
            row.update(status='translate_fail', reason=normalise(message), detail=message[:600])
            return row
        with open(out_path, encoding='utf-8', errors='ignore') as handle:
            msl = handle.read()
        row['status'] = 'ok'
        row['slots'] = static_checks(msl)
        over = [k for k, v in row['slots'].items() if v > LIMITS[k]]
        if over:
            row['status'] = 'over_limit'
            row['reason'] = 'exceeds ' + ','.join(f"{k} {row['slots'][k]}>{LIMITS[k]}" for k in over)
        if do_compile and row['status'] == 'ok':
            code, message = compile_msl(out_path, args.platform, args.no_fast_math)
            if code != 0:
                row.update(status='metal_compile_fail', reason=normalise(message), detail=message[:600])
        return row

    with concurrent.futures.ThreadPoolExecutor(max_workers=args.jobs) as pool:
        results = list(pool.map(job, work))
    return version, do_compile, results


def render(version, compiled, results, args):
    lines = ['# X1 SPIR-V to MSL translation (generated)', '',
             f'- SPIRV-Cross: `{version}`', f'- platform: {args.platform}; MSL {MSL_VERSION}',
             f"- Metal compile step: {'run' + (' with -fno-fast-math' if args.no_fast_math else '') if compiled else 'not run'}",
             f'- stages: {len({(r["bin"], r["stage"]) for r in results})}', '',
             '| profile | ok | translate_fail | over_limit | metal_compile_fail | timeout |', '| --- | ---: | ---: | ---: | ---: | ---: |']
    for profile in args.profiles:
        rows = [r for r in results if r['profile'] == profile]
        count = collections.Counter(r['status'] for r in rows)
        lines.append(f"| {profile} | {count['ok']} | {count['translate_fail']} | {count['over_limit']} | "
                     f"{count['metal_compile_fail']} | {count['timeout']} |")
    for profile in args.profiles:
        failures = collections.Counter()
        examples = {}
        for r in results:
            if r['profile'] == profile and r['status'] != 'ok':
                key = (r['status'], r.get('reason', ''))
                failures[key] += 1
                examples.setdefault(key, r['file'])
        if failures:
            lines += ['', f'## {profile}: failure reasons', '', '| count | status | reason | example |', '| ---: | --- | --- | --- |']
            for (status, reason), count in failures.most_common():
                lines.append(f"| {count} | {status} | {reason.replace('|', '/')} | {examples[(status, reason)]} |")
    worst = collections.defaultdict(lambda: {k: 0 for k in LIMITS})
    for r in results:
        if r['status'] in ('ok', 'over_limit', 'metal_compile_fail') and 'slots' in r:
            for k, v in r['slots'].items():
                worst[r['profile']][k] = max(worst[r['profile']][k], v)
    lines += ['', '## Highest slot count per stage', '', '| profile | buffers | textures | samplers |', '| --- | ---: | ---: | ---: |']
    for profile, data in worst.items():
        lines.append(f"| {profile} | {data['buffer']} | {data['texture']} | {data['sampler']} |")
    return '\n'.join(lines) + '\n'


def parse_args(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument('directory', help='output of the x1_lower_all example')
    parser.add_argument('--spirv-cross', required=True)
    parser.add_argument('--out', required=True)
    parser.add_argument('--profiles', default='plain,argbuf-tier2', type=lambda s: s.split(','))
    parser.add_argument('--platform', choices=('macos', 'ios'), default='macos')
    parser.add_argument('--compile', action='store_true', help='also run the Metal compiler (needs xcrun)')
    parser.add_argument('--no-fast-math', action='store_true')
    parser.add_argument('--filter')
    parser.add_argument('--jobs', type=int, default=os.cpu_count() or 4)
    parser.add_argument('--extra', nargs=argparse.REMAINDER, default=[], help='extra spirv-cross options')
    args = parser.parse_args(argv)
    unknown = [p for p in args.profiles if p not in PROFILES]
    if unknown:
        parser.error(f'unknown profiles {unknown}; known: {sorted(PROFILES)}')
    return args


def main(argv=None):
    args = parse_args(argv)
    os.makedirs(args.out, exist_ok=True)
    version, compiled, results = run(args)
    with open(os.path.join(args.out, 'results.json'), 'w', encoding='utf-8') as handle:
        json.dump({'spirv_cross': version, 'compiled': compiled, 'rows': results}, handle, indent=1)
    text = render(version, compiled, results, args)
    with open(os.path.join(args.out, 'results.md'), 'w', encoding='utf-8') as handle:
        handle.write(text)
    sys.stdout.write(text)
    return 0


if __name__ == '__main__':
    sys.exit(main())
