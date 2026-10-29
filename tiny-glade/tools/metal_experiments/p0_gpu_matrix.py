#!/usr/bin/env python3
"""P0 baseline: drive the existing GPU diagnostic controller over a run matrix (Windows).

Runs `tools/perf_gpu_diagnostic.py` for every combination of scene x quality preset x mode,
repeated R times, interleaved (repeat 1 of every combination, then repeat 2, ...) so slow
drift on the machine is spread over the combinations instead of landing on one of them.
That gives the three-round dispersion METAL_DESIGN.md section 0.5 needs for its thresholds.

Modes
  command   command-span baseline (no pass spans, no isolation): production-like whole-frame
            GPU time from command spans.
  isolated  pass spans + isolation + pipeline statistics: per-pass exclusive time (the data in
            docs/performance/evidence-gpu-pass-20261027/).

The CPU render-phase baseline is NOT driven here: it uses the A/B series tooling
(tools/perf_headless_series.py), which needs a calibration and producer reviews that this
script must not invent. Run that separately for the same frozen build.

    python -B tools/metal_experiments/p0_gpu_matrix.py \
        --build-json .perf/headless-builds/BUILD/build.json --output .perf/p0-gpu-matrix \
        [--scenes static,many_draws] [--qualities quality,balanced,performance] [--repeats 3] [--dry-run]

Re-running resumes: a run whose directory already exists is skipped (a failed run's directory
is kept for inspection; delete it to retry). `matrix.json` indexes every run.
"""
import argparse
import itertools
import json
import os
import sys
import types

TOOLS = os.path.abspath(os.path.join(os.path.dirname(__file__), '..'))
SCENES = ('static', 'many_draws', 'local_edit', 'continuous_edit')
QUALITIES = ('quality', 'balanced', 'performance')
MODES = ('command', 'isolated')


def plan(scenes, qualities, modes, repeats):
    """Ordered list of run descriptors; repeat is the outermost loop (interleaving)."""
    runs = []
    for repeat in range(1, repeats + 1):
        for scene, quality, mode in itertools.product(scenes, qualities, modes):
            runs.append({'scene': scene, 'quality': quality, 'mode': mode, 'repeat': repeat,
                         'name': f'{scene}-{quality}-{mode}-r{repeat}'})
    return runs


def namespace_for(run, args):
    isolated = run['mode'] == 'isolated'
    return types.SimpleNamespace(
        build_json=args.build_json, output=os.path.join(args.output, run['name']), workspace=args.workspace,
        install_root=args.install_root, scene=run['scene'], warmup=args.warmup, samples=args.samples,
        capacity=args.capacity, pass_spans=isolated, isolate=isolated, stats=isolated,
        quality=run['quality'], timeout=args.timeout)


def parse_list(value, allowed, label):
    items = value.split(',')
    bad = [i for i in items if i not in allowed]
    if bad:
        raise argparse.ArgumentTypeError(f'unknown {label} {bad}; allowed {list(allowed)}')
    return items


def parse_args(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument('--build-json', required=True)
    parser.add_argument('--output', required=True)
    parser.add_argument('--workspace', default=os.path.dirname(TOOLS))
    parser.add_argument('--install-root', default=r'D:\game\ljxsj_92385\Tiny Glade')
    parser.add_argument('--scenes', default=','.join(SCENES), type=lambda v: parse_list(v, SCENES, 'scene'))
    parser.add_argument('--qualities', default=','.join(QUALITIES), type=lambda v: parse_list(v, QUALITIES, 'quality'))
    parser.add_argument('--modes', default=','.join(MODES), type=lambda v: parse_list(v, MODES, 'mode'))
    parser.add_argument('--repeats', type=int, default=3)
    parser.add_argument('--warmup', type=int, default=200)
    parser.add_argument('--samples', type=int, default=120)
    parser.add_argument('--capacity', type=int, default=200000)
    parser.add_argument('--timeout', type=float, default=3600)
    parser.add_argument('--dry-run', action='store_true', help='print the plan and exit')
    args = parser.parse_args(argv)
    if args.repeats < 1:
        parser.error('--repeats must be >= 1')
    return args


def main(argv=None):
    args = parse_args(argv)
    runs = plan(args.scenes, args.qualities, args.modes, args.repeats)
    if args.dry_run:
        for index, run in enumerate(runs, 1):
            print(f"{index:3d}/{len(runs)} {run['name']}")
        print(f'{len(runs)} runs')
        return 0
    sys.path.insert(0, TOOLS)
    import perf_gpu_diagnostic as controller  # imported late: dry-run needs no Windows-only pieces
    os.makedirs(args.output, exist_ok=True)
    index_path = os.path.join(args.output, 'matrix.json')
    index = json.load(open(index_path, encoding='utf-8')) if os.path.isfile(index_path) else []
    seen = {entry['name'] for entry in index}
    failures = 0
    for number, run in enumerate(runs, 1):
        target = os.path.join(args.output, run['name'])
        if run['name'] in seen or os.path.exists(target):
            print(f"{number}/{len(runs)} skip {run['name']} (exists)")
            continue
        print(f"{number}/{len(runs)} run  {run['name']}", flush=True)
        entry = dict(run)
        try:
            record = controller.run(namespace_for(run, args))
            entry.update(valid=bool(record.get('valid')), directory=run['name'])
        except Exception as error:  # a failed run is data; keep going and report at the end
            entry.update(valid=False, error=f'{type(error).__name__}: {error}')
            failures += 1
        index.append(entry)
        with open(index_path, 'w', encoding='utf-8') as handle:
            json.dump(index, handle, indent=1)
    print(f'done: {len(index)} indexed, {failures} failed this session')
    return 1 if failures else 0


if __name__ == '__main__':
    sys.exit(main())
