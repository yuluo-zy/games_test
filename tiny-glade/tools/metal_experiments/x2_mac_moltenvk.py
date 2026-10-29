#!/usr/bin/env python3
"""X2: run the headless GPU diagnostic on a Mac through MoltenVK (the current Vulkan backend).

Answers METAL_DESIGN.md X2: where Apple-GPU time goes per pass (so which of D13's e1/e2 and D15's
cuts matter), how many encoders/draws a frame costs, and which MoltenVK gaps show up. It is the B2
baseline of design section 1.2 as well.

The Windows controller (tools/perf_gpu_diagnostic.py) runs a frozen EXE and verifies its hashes; a
Mac has no such frozen build, so this runs `cargo test` directly. To keep the measurement identical,
the environment comes from the controller's own `environment()` function. Differences, all stated:
  * not a frozen/hash-checked binary: the run record says so (`frozen_build: false`);
  * Vulkan validation is optional (`--validation`): the Windows evidence used it, but it needs the
    Vulkan SDK layer installed;
  * no process-priority control.

    python3 tools/metal_experiments/x2_mac_moltenvk.py --install-root "<game install>" \
        --output .perf/x2-mac --scene static --quality quality [--repeats 3] [--isolate] [--stats]

STATUS: never run. tiny-glade cannot be built on the Linux machine this was written on, and the
macOS build of the headless fixture has not been exercised; expect to fix build errors on first use.
"""
import argparse
import json
import os
import platform
import shutil
import subprocess
import sys
import time

TOOLS = os.path.abspath(os.path.join(os.path.dirname(__file__), '..'))
REPO = os.path.dirname(TOOLS)
sys.path.insert(0, TOOLS)

MOLTENVK_ICD_CANDIDATES = (
    '/usr/local/share/vulkan/icd.d/MoltenVK_icd.json',
    '/opt/homebrew/share/vulkan/icd.d/MoltenVK_icd.json',
    os.path.expanduser('~/VulkanSDK/macOS/share/vulkan/icd.d/MoltenVK_icd.json'),
)


def find_icd(explicit):
    if explicit:
        return explicit if os.path.isfile(explicit) else None
    return next((p for p in MOLTENVK_ICD_CANDIDATES if os.path.isfile(p)), None)


def build_environment(args, session_path):
    import perf_gpu_diagnostic as controller  # the single source of truth for the TG_* variables
    capacity = args.capacity or min((args.warmup + args.samples + 1) * 128, 2000000)
    env = controller.environment(args.install_root, session_path, args.scene, args.warmup, args.samples, capacity,
                                 True, args.isolate, args.stats, args.quality)
    # The controller targets the Windows validation setup; relax what a Mac may not have.
    if not args.validation:
        env.pop('VK_INSTANCE_LAYERS', None)
        env.pop('RHAPSODY_REQUIRE_VULKAN', None)
    env['VK_ICD_FILENAMES'] = args.icd
    # Preserve the loader/library search path for MoltenVK and the Vulkan SDK.
    for key in ('DYLD_LIBRARY_PATH', 'DYLD_FALLBACK_LIBRARY_PATH', 'VULKAN_SDK', 'VK_LAYER_PATH', 'HOME', 'PATH',
                'CARGO_HOME', 'RUSTUP_HOME'):
        if key in os.environ:
            env.setdefault(key, os.environ[key])
    return env


def cargo_command(args):
    return ['cargo', 'test', '-p', 'tiny-glade', '--release', '--features', 'cpu-probe',
            '--test', 'original_gameplay_chain', '--',
            'cpu_measurement::profile_shipped_session_cpu', '--exact', '--ignored', '--nocapture', '--test-threads=1']


def one_run(args, index):
    directory = os.path.join(args.output, f'{args.scene}-{args.quality}-r{index}')
    os.makedirs(directory, exist_ok=False)
    session = os.path.join(directory, 'session.json')
    env = build_environment(args, session)
    command = cargo_command(args)
    record = {
        'schema': 'x2-mac-moltenvk-run-v1', 'frozen_build': False,
        'host': {'platform': platform.platform(), 'machine': platform.machine()},
        'settings': {'scene': args.scene, 'quality': args.quality, 'warmup': args.warmup, 'samples': args.samples,
                     'isolate': args.isolate, 'stats': args.stats, 'validation': args.validation},
        'command': command, 'icd': args.icd, 'started_utc': time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()),
        'git_head': subprocess.run(['git', '-C', REPO, 'rev-parse', 'HEAD'], capture_output=True, text=True).stdout.strip(),
        'git_dirty': bool(subprocess.run(['git', '-C', REPO, 'status', '--porcelain', '--untracked-files=no'],
                                         capture_output=True, text=True).stdout.strip()),
    }
    with open(os.path.join(directory, 'fixture.log'), 'w', encoding='utf-8') as log:
        proc = subprocess.run(command, cwd=REPO, env=env, stdout=log, stderr=subprocess.STDOUT, timeout=args.timeout)
    record['returncode'] = proc.returncode
    record['session_written'] = os.path.isfile(session)
    with open(os.path.join(directory, 'run.json'), 'w', encoding='utf-8') as handle:
        json.dump(record, handle, indent=1)
    return directory, record


def parse_args(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument('--install-root', required=True, help='path of the original game install (TG_INSTALL_ROOT)')
    parser.add_argument('--output', required=True)
    parser.add_argument('--scene', default='static', choices=('static', 'many_draws', 'local_edit', 'continuous_edit'))
    parser.add_argument('--quality', default='quality', choices=('performance', 'balanced', 'quality'))
    parser.add_argument('--repeats', type=int, default=1)
    parser.add_argument('--warmup', type=int, default=200)
    parser.add_argument('--samples', type=int, default=120)
    parser.add_argument('--capacity', type=int, default=0)
    parser.add_argument('--isolate', action='store_true', help='serialise passes: exclusive per-pass time')
    parser.add_argument('--stats', action='store_true', help='pipeline statistics per pass')
    parser.add_argument('--validation', action='store_true', help='enable the Khronos validation layer')
    parser.add_argument('--icd', help='path to MoltenVK_icd.json (searched in the usual places by default)')
    parser.add_argument('--timeout', type=float, default=3600)
    args = parser.parse_args(argv)
    args.install_root = os.path.abspath(args.install_root)
    return args


def main(argv=None):
    args = parse_args(argv)
    if platform.system() != 'Darwin':
        print('this runner is for macOS (MoltenVK)', file=sys.stderr)
        return 2
    if shutil.which('cargo') is None:
        print('cargo not found', file=sys.stderr)
        return 2
    args.icd = find_icd(args.icd)
    if not args.icd:
        print('MoltenVK ICD not found; install the Vulkan SDK or `brew install molten-vk`, or pass --icd', file=sys.stderr)
        return 2
    os.makedirs(args.output, exist_ok=True)
    failed = 0
    for index in range(1, args.repeats + 1):
        directory, record = one_run(args, index)
        ok = record['returncode'] == 0 and record['session_written']
        failed += not ok
        print(f"{'ok  ' if ok else 'FAIL'} {directory}")
    print('next: python3 docs/performance/evidence-gpu-pass-20261027/analyze_gpu_passes.py <dir>/session.json --output <dir>/ranking.json')
    return 1 if failed else 0


if __name__ == '__main__':
    sys.exit(main())
