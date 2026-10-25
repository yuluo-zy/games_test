"""Window Lab functional, performance and long-run acceptance using one saved binary."""
import argparse
import datetime
import hashlib
import json
import math
import os
from pathlib import Path
import re
import shutil
import statistics
import subprocess
import time
from refactor_checks import WORKSPACE, fingerprint as source_fingerprint

def fingerprint():
    return source_fingerprint("window")


def stability_reasons(samples):
    steady = [sample for sample in samples if sample.get('phase') == 'frame_complete']
    if len(steady) < 10:
        return ['insufficient long-run resource stability samples']
    tail = steady[-10:]
    reasons = []
    for field in ('graph_resources', 'graph_commands', 'graph_command_pools', 'named_resources'):
        if len({sample.get(field) for sample in tail}) != 1:
            reasons.append(f'{field} did not stabilize in the last ten observations')
    # Compare envelopes rather than individual reload phases. A bounded transient
    # spike is allowed; an increasing late-run memory/cache envelope is not.
    for field in ('known_cpu_bytes', 'registered_budget_bytes',
                  'application_vk_memory_bytes', 'native_memory_blocks'):
        values = [sample.get(field) for sample in tail]
        if any(not isinstance(value, int) or value < 0 for value in values):
            reasons.append(f'{field} stability measurements are absent')
        elif max(values[5:]) > max(values[:5]):
            reasons.append(f'{field} grew in the final stability window')
    return reasons


def workload_identity(report):
    # Asset and device identities must agree even for a historical renderer.
    return {field: report.get(field) for field in
            ('fixture_hash', 'source_hash', 'readback_hash', 'device', 'driver_version_raw')}


def main():
    parser = argparse.ArgumentParser(__doc__)
    executable = parser.add_mutually_exclusive_group(required=True)
    executable.add_argument('--binary', type=Path)
    executable.add_argument('--build', action='store_true')
    parser.add_argument('--build-manifest', type=Path)
    parser.add_argument('--reference-binary', type=Path)
    parser.add_argument('--reference-identity', type=Path)
    parser.add_argument('--suite', choices=('functional', 'performance', 'long-run'), required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--long-run-seconds', type=int, default=1800)
    parser.add_argument('--long-run-case', choices=('stable', 'effects', 'resize', 'all'), default='all')
    parser.add_argument('--performance-pair', choices=('whole-plan', 'subgraph-cache', 'result-cache', 'all-cache', 'aliasing', 'dynamic-rendering', 'bindless', 'raster-migration', 'multi-queue', 'pipeline-disk', 'all'), default='all')
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=True)
    source = fingerprint()
    harness = {}
    for script in (Path(__file__), Path(__file__).with_name('refactor_checks.py')):
        destination = args.output / script.name
        if script.resolve() != destination.resolve():
            shutil.copy2(script, destination)
        harness[script.name] = hashlib.sha256(script.read_bytes()).hexdigest()
    env = os.environ.copy()
    env.update(DYLD_FALLBACK_LIBRARY_PATH='/opt/homebrew/lib', RHAPSODY_REQUIRE_VULKAN='1',
        VK_LAYER_VALIDATE_SYNC='1', CARGO_INCREMENTAL='0')
    binary = args.output.resolve() / 'window_lab'
    if args.build:
        command = ['cargo', 'build', '-p', 'rhapsody', '--all-features', '--example', 'window_lab', '--offline', '--locked', '-j', '2']
        with (args.output / 'build.log').open('w') as log:
            code = subprocess.run(command, cwd=WORKSPACE, env=env, stdout=log, stderr=subprocess.STDOUT).returncode
        if code or fingerprint() != source:
            raise SystemExit('build failed or its source changed; see build.log')
        shutil.copy2(WORKSPACE / 'target/debug/examples/window_lab', binary)
        build = dict(source=source, source_scope='rhapsody-window-and-shader-dependencies', binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(), command=command)
    else:
        if not args.build_manifest:
            parser.error('--binary requires --build-manifest to verify its source and identity')
        build = json.loads(args.build_manifest.read_text())
        if args.binary.resolve() != binary:
            shutil.copy2(args.binary, binary)
        if build['source'] != source or build['binary_sha256'] != hashlib.sha256(binary.read_bytes()).hexdigest():
            raise SystemExit('saved binary does not match its build manifest/current source')
    (args.output / 'build-manifest.json').write_text(json.dumps(build, indent=2) + '\n')
    reference = None
    reference_identity = None
    if args.reference_binary:
        if not args.reference_identity:
            parser.error('--reference-binary requires its saved --reference-identity')
        reference_identity = json.loads(args.reference_identity.read_text())
        reference = args.output.resolve() / 'reference_window_lab'
        shutil.copy2(args.reference_binary, reference)
        if hashlib.sha256(reference.read_bytes()).hexdigest() != reference_identity['sha256']:
            raise SystemExit('reference executable SHA does not match saved identity')
    manifest = dict(started_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(), source=source,
        binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(), source_scope='rhapsody-window-and-shader-dependencies', harness_sha256=harness, reference=reference_identity, runs=[], comparisons=[],
        scope='Application GPU readback, validation and presentation teardown. Screen pixels, keyboard and Windows are separate pending checks.')
    jobs = []
    if args.suite == 'functional':
        jobs = [(name, ['--scene', scene] + options, 960 if name == 'showcase' else 1000 if name == 'wsi' else 900 if name.endswith('-repeat') else 360, [], 'current') for name, scene, options in (
            ('showcase', 'showcase', []),
            ('fixture', 'fixture', ['--script']), ('wsi', 'fixture', ['--wsi-stress']),
            ('raster-dynamic', 'raster', ['--rendering', 'dynamic']), ('raster-reload', 'raster', ['--rendering', 'dynamic', '--shader-script']),
            ('raster-repeat', 'raster', ['--rendering', 'dynamic', '--shader-script', '--repeat-script']),
            ('wsi-repeat', 'fixture', ['--wsi-stress', '--repeat-script']),
            ('temporal', 'temporal', ['--scene-script']), ('transfer', 'transfer', []),
            ('transient-off', 'transient', ['--transient-aliasing', 'off']), ('transient-on', 'transient', ['--transient-aliasing', 'on']),
            ('result', 'result', []),
            ('queues-off', 'multi-queue', ['--queues', 'off', '--sharing', 'exclusive']),
            ('queues-prefer', 'multi-queue', ['--queues', 'prefer', '--sharing', 'exclusive']),
            ('queues-exclusive', 'multi-queue', ['--queues', 'require', '--sharing', 'exclusive']),
            ('queues-concurrent', 'multi-queue', ['--queues', 'require', '--sharing', 'concurrent']),
            ('transient-physical-off', 'transient', ['--transient-aliasing', 'off', '--transient-attachments', '32', '--transient-scale', '8']),
            ('transient-physical-on', 'transient', ['--transient-aliasing', 'on', '--transient-attachments', '32', '--transient-scale', '8']),
        )]
    elif args.suite == 'performance':
        pairs = (
            ('whole-plan', 'fixture', ['--plan-cache', 'off'], ['--plan-cache', 'on'], 'current', 'current'),
            # Whole-plan hits would hide local-plan analysis. Keep that layer
            # disabled on both sides for the isolated subgraph comparison.
            ('subgraph-cache', 'result', ['--plan-cache', 'off', '--subgraph-cache', 'off', '--result-cache', 'off'], ['--plan-cache', 'off', '--subgraph-cache', 'on', '--result-cache', 'off'], 'current', 'current'),
            ('result-cache', 'result', ['--result-cache', 'off'], ['--result-cache', 'on'], 'current', 'current'),
            ('all-cache', 'result', ['--plan-cache', 'off', '--subgraph-cache', 'off', '--result-cache', 'off'], ['--plan-cache', 'on', '--subgraph-cache', 'on', '--result-cache', 'on'], 'current', 'current'),
            ('aliasing', 'transient', ['--transient-aliasing', 'off'], ['--transient-aliasing', 'on'], 'current', 'current'),
            # Isolated historical toggles use the same saved executable for both sides.
            ('dynamic-rendering', 'raster', ['--rendering', 'legacy'], ['--rendering', 'dynamic'], 'reference', 'reference'),
            ('bindless', 'raster', ['--rendering', 'dynamic'], ['--rendering', 'dynamic', '--bindless'], 'reference', 'reference'),
            # This is explicitly the net renderer migration, rather than an isolated bindless gain.
            ('raster-migration', 'raster', ['--rendering', 'dynamic'], ['--rendering', 'dynamic'], 'reference', 'current'),
            ('multi-queue', 'multi-queue', ['--queues', 'off'], ['--queues', 'require'], 'current', 'current'),
            ('pipeline-disk', 'raster', ['--rendering', 'dynamic'], ['--rendering', 'dynamic'], 'current', 'current'),
        )
        for name, scene, off, on, off_binary, on_binary in pairs:
            if args.performance_pair not in (name, 'all'):
                continue
            if 'reference' in (off_binary, on_binary) and not reference:
                parser.error('the requested renderer comparison requires a saved reference executable and identity')
            for repeat in range(3):
                # Alternate order to avoid a systematic cold/thermal bias.
                sides = (('off', off, off_binary), ('on', on, on_binary))
                for label, switches, binary_kind in (sides if repeat % 2 == 0 or name == 'pipeline-disk' else reversed(sides)):
                    jobs.append((f'{name}-{label}-{repeat+1}', ['--scene', scene] + switches, 3300,
                        ['--warmup', '300', '--samples', '3000'], binary_kind))
    else:
        if args.long_run_seconds < 1800:
            parser.error('delivery long-run requires at least 1800 seconds')
        frames = 10000
        cases = [('stable', ['--scene', 'fixture']), ('effects', ['--scene', 'raster', '--shader-script', '--repeat-script', '--rendering', 'dynamic']),
                 ('resize', ['--scene', 'fixture', '--wsi-stress', '--repeat-script'])]
        jobs = [(f'long-{name}', switches + ['--min-run-seconds', str(args.long_run_seconds)], frames, [], 'current') for name, switches in cases
                if args.long_run_case in (name, 'all')]
    for name, switches, frames, samples, binary_kind in jobs:
        directory = args.output / name
        directory.mkdir(parents=True, exist_ok=True)
        report_path = directory / 'report.json'
        run_binary = binary if binary_kind == 'current' else reference
        cache_path = directory / 'pipeline.cache'
        cache_seed_missing = False
        if name.startswith('pipeline-disk-'):
            if cache_path.exists():
                cache_path.unlink()  # This runner's generated cache, never a user input.
            if '-on-' in name:
                cold_cache = args.output / name.replace('-on-', '-off-') / 'pipeline.cache'
                cache_seed_missing = not cold_cache.is_file()
                if not cache_seed_missing:
                    shutil.copy2(cold_cache, cache_path)
        command = [str(run_binary), *switches, '--frames', str(frames), '--fps', '60' if args.suite == 'long-run' else '240',
            '--gpu-timing', 'on', '--report', str(report_path.resolve()), '--capture', str((directory / 'readback.ppm').resolve()),
            '--pipeline-cache', str(cache_path.resolve()), '--require-validation', *samples]
        if binary_kind == 'current' and args.suite != 'performance':
            command += ['--gpu-pass-timing', 'on']
        before = fingerprint()
        started = time.monotonic()
        with (directory / 'run.log').open('w') as log:
            print('RUN', name, flush=True)
            try:
                code = subprocess.run(command, cwd=WORKSPACE, env=env, stdout=log, stderr=subprocess.STDOUT,
                    timeout=max(600, args.long_run_seconds * 3) if args.suite == 'long-run' else 900).returncode
            except subprocess.TimeoutExpired:
                code = 124
        elapsed = time.monotonic() - started
        after = fingerprint()
        output = (directory / 'run.log').read_text(errors='replace')
        validation = len(re.findall(r'\bValidation Error\b|SYNC-HAZARD', output, re.IGNORECASE))
        report = json.loads(report_path.read_text()) if report_path.exists() else {}
        status = 'PASS'
        reasons = []
        if code or validation or report.get('status') != 'pass' or report.get('validation_error_count', 1):
            status = 'FAIL'; reasons.append('execution/report/validation failed')
        presented = report.get('presented_frames', 0)
        frames_match = presented >= frames if args.suite == 'long-run' else presented == frames
        if not frames_match or report.get('pixel_verified_frames') != presented:
            status = 'FAIL'; reasons.append('requested frames were not all presented and verified')
        if not report.get('presentation_shutdown_complete') or report.get('retained_wsi_generations', 1):
            status = 'FAIL'; reasons.append('presentation teardown not proved')
        if binary_kind == 'current':
            teardown = report.get('teardown_memory') or {}
            if any(teardown.get(field) != 0 for field in ('registered_allocations', 'registered_resource_bytes',
                    'known_cpu_bytes', 'registered_budget_bytes')):
                status = 'FAIL'; reasons.append('registered resources or CPU storage remain after teardown')
            if not report.get('stability_samples') or len(report['stability_samples']) > 128:
                status = 'FAIL'; reasons.append('bounded resource stability samples are absent')
            if args.suite != 'performance' and (not report.get('gpu_pass_timings') or
                    len(report['gpu_pass_timings']) > 1024):
                status = 'FAIL'; reasons.append('bounded per-pass GPU measurements are absent')
        if args.suite == 'performance':
            if report.get('performance', {}).get('graph_framework', {}).get('samples') != 3000:
                status = 'FAIL'; reasons.append('insufficient warm performance samples')
            if name.startswith('pipeline-disk-'):
                expected_cache = 'loaded' if '-on-' in name else 'missing'
                if cache_seed_missing or (report.get('scene') or {}).get('pipeline_cache_status') != expected_cache:
                    status = 'FAIL'; reasons.append('pipeline cache did not exercise the requested cold/warm state')
                if any(not isinstance(report.get(field), (int, float)) or
                       not math.isfinite(report[field]) or report[field] <= 0
                       for field in ('initialization_seconds', 'first_present_seconds')):
                    status = 'FAIL'; reasons.append('startup measurements are absent or invalid')
                cold = report.get('performance', {}).get('cold_start', {})
                if any(cold.get(field, {}).get('samples') != 300 for field in ('total_frame', 'graph_framework')):
                    status = 'FAIL'; reasons.append('insufficient first-300-frame measurements')
        if args.suite == 'long-run' and (report.get('play_seconds', 0) < args.long_run_seconds or
                report.get('minimum_run_seconds') != args.long_run_seconds):
            status = 'FAIL'; reasons.append('long-run duration too short')
        if args.suite == 'long-run':
            growth = stability_reasons(report.get('stability_samples', []))
            if growth:
                status = 'FAIL'; reasons.extend(growth)
        if name == 'showcase':
            showcase = report.get('showcase') or {}
            if (showcase.get('producer_frames') != [presented] * 4 or
                    showcase.get('verified_frames') != presented or
                    showcase.get('history_frames') != presented or
                    showcase.get('effect_publications', 0) < 2 or
                    showcase.get('uploaded_payload_rounds', 0) < 1 or
                    showcase.get('changed_pixels', 0) <= 0 or
                    not showcase.get('shutdown_complete') or
                    report.get('resize_events', 0) < 2 or
                    report.get('minimized_iterations', 0) < 1):
                status = 'FAIL'; reasons.append('continuous composite workload or interleaved window actions were not observed')
        if name in ('raster-repeat', 'long-effects'):
            scene = report.get('scene') or {}
            if scene.get('shader_frame_boundary_publications', 0) < 2 or scene.get('shader_rejected_edits', 0) < 2:
                status = 'FAIL'; reasons.append('repeated shader changes were not observed')
        if name in ('wsi-repeat', 'long-resize'):
            stress = report.get('wsi_stress') or {}
            if stress.get('resize_completions', 0) <= 100 or stress.get('completed_cycles', 0) <= 20:
                status = 'FAIL'; reasons.append('repeated native window changes were not observed')
            if len(stress.get('events', [])) > 140 or stress.get('minimize_completions') != stress.get('restore_completions'):
                status = 'FAIL'; reasons.append('window event history is unbounded or minimize/restore did not drain')
        if before != source or after != source:
            status = 'INVALID_SOURCE_CHANGED'; reasons.append('source differs from saved binary build snapshot')
        manifest['runs'].append(dict(name=name, command=command, binary_kind=binary_kind,
            binary_sha256=hashlib.sha256(run_binary.read_bytes()).hexdigest(), status=status, reasons=reasons, exit_code=code,
            elapsed_seconds=elapsed, source_before=before, source_after=after, report=str(report_path.relative_to(args.output))))
        (args.output / 'checks.json').write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + '\n')
        print('DONE', name, status, flush=True)
    if args.suite == 'performance':
        for group in sorted({run['name'].rsplit('-', 2)[0] for run in manifest['runs']}):
            values = {}
            selected = [run for run in manifest['runs'] if run['name'].startswith(group + '-')]
            workloads = [workload_identity(json.loads((args.output / run['report']).read_text()))
                         for run in selected if run['status'] == 'PASS']
            same_workload = (len(workloads) == 6 and
                             all(value is not None and value != '' for value in workloads[0].values()) and
                             all(identity == workloads[0] for identity in workloads))
            reference_pair = any(run['binary_kind'] == 'reference' for run in selected)
            cpu_metric = 'graph_framework' if reference_pair else 'framework_including_declarations'
            metric_names = None
            for label in ('off', 'on'):
                reports = [json.loads((args.output / run['report']).read_text()) for run in selected
                    if run['status'] == 'PASS' and run['name'].startswith(group + '-' + label + '-')]
                if len(reports) != 3:
                    continue
                distributions = []
                for report in reports:
                    performance = report['performance']
                    metrics = {'cpu': performance.get(cpu_metric, {})}
                    metrics.update({f"gpu_queue_{queue['queue_index']}": queue['sum_of_command_spans_per_frame']
                        for queue in performance.get('gpu_command_spans_by_queue', [])})
                    distributions.append(metrics)
                # Compare only distributions with all 3000 requested warm samples.
                common = set.intersection(*(set(metrics) for metrics in distributions))
                complete = {metric for metric in common if all(metrics[metric].get('samples') == 3000 for metrics in distributions)}
                metric_names = complete if metric_names is None else metric_names & complete
                values[label] = {metric: {p: statistics.median(metrics[metric][p] for metrics in distributions)
                    for p in ('p95_ms', 'p99_ms')} for metric in complete}
            comparison = dict(name=group, cpu_metric=cpu_metric,
                scope='Median of three warm CPU distributions and per-physical-queue GPU command spans; readback/WSI/reference reported separately. Historical binaries lack joint declaration+execution CPU distributions.',
                threshold=0.05, values=values, workload_identities=workloads,
                same_workload_and_device=same_workload)
            if group == 'multi-queue':
                comparison['gpu_scope'] = 'The common graphics queue contains different work after scheduling across queues. Per-queue spans show where work runs; they do not establish whole-frame GPU latency or a throughput improvement.'
            if group == 'pipeline-disk':
                startup = {}
                cold_frames = {}
                for label in ('off', 'on'):
                    reports = [json.loads((args.output / run['report']).read_text()) for run in selected
                        if run['status'] == 'PASS' and run['name'].startswith(group + '-' + label + '-')]
                    if len(reports) == 3:
                        startup[label] = {field: [report.get(field) for report in reports]
                            for field in ('initialization_seconds', 'first_present_seconds')}
                        cold_frames[label] = {field: {p: statistics.median(
                            report['performance']['cold_start'][field][p] for report in reports)
                            for p in ('p95_ms', 'p99_ms')} for field in ('total_frame', 'graph_framework')}
                comparison['startup_seconds'] = startup
                comparison['first_300_frames'] = cold_frames
                comparison['startup_scope'] = 'Three process launches per side: empty application VkPipelineCache versus a saved file. Operating-system and opaque driver caches are uncontrolled; three launch times are not a p99 distribution.'
            if same_workload and len(values) == 2 and metric_names and 'cpu' in metric_names and any(metric.startswith('gpu_queue_') for metric in metric_names):
                comparison['regression'] = {metric: {p: values['on'][metric][p] / values['off'][metric][p] - 1
                    if values['off'][metric][p] else None for p in ('p95_ms', 'p99_ms')} for metric in sorted(metric_names)}
                comparison['threshold_exceeded'] = any(value is not None and value > 0.05
                    for metrics in comparison['regression'].values() for value in metrics.values())
                comparison['default_enabled'] = group in ('whole-plan', 'subgraph-cache', 'result-cache', 'all-cache', 'raster-migration')
                comparison['reference_only'] = group in ('dynamic-rendering', 'bindless')
                comparison['status'] = ('FAIL_DEFAULT_REGRESSION' if comparison['default_enabled'] else
                    'REFERENCE_REGRESSION' if comparison['reference_only'] else 'REGRESSION_OPT_IN') if comparison['threshold_exceeded'] else 'PASS'
                if any(not math.isfinite(value) or value <= 0
                       for side in values.values() for metric in metric_names
                       for value in side[metric].values()):
                    comparison['status'] = 'INCOMPLETE'
                    comparison['reason'] = 'nonpositive or nonfinite tail measurements cannot establish a relative regression gate'
            else:
                comparison['status'] = 'INCOMPLETE'
                if not same_workload:
                    comparison['reason'] = 'six matching nonempty fixture/source/readback/device/driver identities are required'
            manifest['comparisons'].append(comparison)
    (args.output / 'checks.json').write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + '\n')
    return 0 if all(run['status'] == 'PASS' for run in manifest['runs']) and all(
        comparison['status'] in ('PASS', 'REGRESSION_OPT_IN', 'REFERENCE_REGRESSION') for comparison in manifest['comparisons']) else 1


if __name__ == '__main__':
    raise SystemExit(main())
