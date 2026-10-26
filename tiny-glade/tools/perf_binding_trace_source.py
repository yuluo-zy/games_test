"""Frozen rich EXE OFF/ON recording/source investigation; no CPU acceptance."""
import argparse
import base64
from collections import defaultdict
from contextlib import contextmanager
import json
from pathlib import Path

import parse_native_pass_trace as native_trace
import perf_binding_scope_diagnostic as counts

runner = counts.runner
require = counts.require
CANONICAL_CONTRACT_SHA256 = '98d7d1cebad9556bb31fcc8fd03811a300ef2b4a5d76eaf65fe7ebf10a9437e4'


def tool_identities(canonical_layout=False):
    identities = counts.tool_identities()
    modules = (native_trace, canonical_contract()) if canonical_layout else (native_trace,)
    for module in modules:
        path = Path(module.__file__).resolve()
        identities[path.name] = counts.frozen_tools.digest(path)
    path = Path(__file__).resolve()
    identities[path.name] = counts.frozen_tools.digest(path)
    return dict(sorted(identities.items()))


@contextmanager
def trace_environment(canonical_layout=False):
    """The existing requested_environment still owns cleanup/bill/binding."""
    original = runner.environment
    def environment(diagnostic=False):
        env = original(diagnostic)
        require(diagnostic is True, 'trace/source requires rich diagnostic capture')
        env['RHAPSODY_SUBMIT_TRACE'] = '1'
        if canonical_layout:
            env['TG_PERF_CANONICAL_WALL_LAYOUT'] = '1'
        return env
    runner.environment = environment
    try:
        yield
    finally:
        runner.environment = original


def canonical_contract():
    # Optional cold controller dependency; historical/default captures do not
    # need a new canonical-capable EXE or this contract module.
    import perf_canonical_contract
    module_path = Path(perf_canonical_contract.__file__).resolve(strict=True)
    require(module_path == Path(__file__).resolve().with_name('perf_canonical_contract.py'),
            'canonical contract module is not the reviewed sibling tool')
    require(counts.frozen_tools.digest(module_path) == CANONICAL_CONTRACT_SHA256,
            'unreviewed canonical source/stamp contract module')
    return perf_canonical_contract


def canonical_source_evidence(args, frozen_identity):
    """Bind reviewed rich cold-hook bytes to this frozen source inventory.

    Read-only checkout bytes must match source.json, not merely resemble a
    current helper. Archive those exact bytes separately from frozen metadata.
    The source audit does not establish successful native owner preflight.
    """
    build_path = runner.PERF / 'headless-builds' / runner.label(args.build_label) / 'build.json'
    source_path = build_path.parent / 'source.json'
    require(counts.frozen_tools.digest(source_path) == frozen_identity['source_json_sha256'],
            'canonical frozen source inventory changed')
    source = json.loads(source_path.read_text(encoding='utf-8'))
    require(source.get('content_sha256') == frozen_identity['source_content_sha256']
            and counts.frozen_tools.signature(source['files']) == source['content_sha256'],
            'canonical source content identity differs')
    contract = canonical_contract()
    raw = {name: (runner.ROOT / name).read_bytes() for name in contract.REQUIRED}
    capability = contract.rich_source_contract(source['files'], raw)
    return {'schema': 'rich-optional-canonical-source-archive-v1',
            'source_json_sha256': frozen_identity['source_json_sha256'],
            'source_commit': source['commit'], 'source_content_sha256': source['content_sha256'],
            'contract': capability,
            'files': {name: {'sha256': contract.bytes_sha(value),
                             'raw_base64': base64.b64encode(value).decode('ascii')}
                      for name, value in raw.items()},
            'scope': 'Source-audited cold-hook capability only; actual fresh-process native owner checks still required'}


def canonical_preparation(report):
    """Require actual opt-in stamps; never infer them from source capability."""
    contract = canonical_contract()
    require(report.get('canonical_layout_input_policy') == contract.PREPARATION
            and report.get('measurement_class') == 'management_diagnostic'
            and report.get('clean_cpu_acceptance_eligible') is False,
            'requested rich canonical preparation policy was not honored')
    prep = contract.preparation_stamp(report.get('canonical_brick_layout'),
                                      report.get('input_before'), report.get('input_after'))
    return {'requested_TG_PERF_CANONICAL_WALL_LAYOUT': '1',
            'canonical_layout_input_policy': report['canonical_layout_input_policy'],
            'canonical_brick_layout': prep,
            'input_before_canonical_brick_layout': report['input_before']['canonical_brick_layout'],
            'input_after_canonical_brick_layout': report['input_after']['canonical_brick_layout'],
            'scope': 'Same original pre-warmup preparation stamp at both input endpoints; not a final/current per-frame layout observation'}


def wall_observations(report):
    bill = report.get('wall_dispatch_source_bill') or {}
    rows = report['frames']
    require(bill.get('schema') == 'wall-dispatch-source-bill-v1' and bill.get('valid') is True
            and bill.get('capacity') == bill.get('retained_observations') == report['samples'] + 1
            and bill.get('warmup_observations') == 0 and bill.get('native_dispatches_observed') is False
            and bill.get('source_phase') == 'after_App_update_before_prepare', 'invalid/missing Wall source bill')
    observations = bill.get('observations')
    require(isinstance(observations, list) and len(observations) == len(rows), 'incomplete Wall source observations')
    for observation, cpu in zip(observations, rows):
        require(all(observation.get(k) == cpu[k] for k in ('world_tick', 'receipt', 'attempt')),
                'Wall source/CPU receipt association differs')
        source = observation.get('source') or {}
        require(source.get('schema') == 'wall-dispatch-source-ranges-v1'
                and source.get('phase') == 'after_App_update_before_prepare'
                and source.get('native_dispatches_observed') is False, 'Wall source scope changed')
        capacity = counts.integer(source.get('capacity_elements'), 'Wall typed capacity')
        require(capacity == 0x200000, 'unexpected BrickTransformSsbo typed capacity')
        highlight = counts.integer(source.get('highlight_type'), 'Wall highlight type')
        require(highlight <= 255 and type(source.get('highlighted_allocation_selected')) is bool,
                'invalid Wall highlight selection')
        for role in ('normal', 'highlighted'):
            ranges = source.get(role)
            require(isinstance(ranges, list), 'Wall raw ranges missing')
            total = 0
            for pair in ranges:
                require(isinstance(pair, list) and len(pair) == 2
                        and all(type(v) is int and 0 <= v < 2**32 for v in pair)
                        and sum(pair) <= capacity, 'Wall range exceeds typed capacity or u32')
                total += pair[1]
            require(counts.integer(source.get(role + '_range_count'), role + ' range count') == len(ranges)
                    and counts.integer(source.get(role + '_instance_count'), role + ' instance count') == total,
                    'Wall source range totals differ')
        require(counts.integer(source.get('callback_dispatches_if_invoked'), 'Wall opportunities')
                == len(source['normal']) + len(source['highlighted']),
                'Wall callback opportunity total differs')
        require(source['highlighted_allocation_selected'] or source['highlighted'] == [],
                'unselected Wall allocation has highlighted ranges')
        require(highlight != 0 or not source['highlighted_allocation_selected'], 'zero highlight selected an allocation')
        digest = source.get('raw_ranges_blake3')
        require(isinstance(digest, list) and len(digest) == 32
                and all(type(v) is int and 0 <= v <= 255 for v in digest)
                and observation.get('raw_ranges_blake3_hex') == bytes(digest).hex(), 'Wall digest encoding differs')
    return observations


def differences(left, right, path=''):
    """Preserve every differing field; never turn a cost difference into zero."""
    if type(left) is not type(right):
        return [{'field': path, 'off': left, 'on': right}]
    if isinstance(left, dict):
        result = []
        for key in sorted(left.keys() | right.keys()):
            child = path + '.' + key if path else key
            if key not in left or key not in right:
                result.append({'field': child, 'off_present': key in left, 'on_present': key in right,
                               'off': left.get(key), 'on': right.get(key)})
            else:
                result.extend(differences(left[key], right[key], child))
        return result
    if isinstance(left, list):
        result = []
        if len(left) != len(right):
            result.append({'field': path + '.length', 'off': len(left), 'on': len(right)})
        for index in range(max(len(left), len(right))):
            if index >= len(left) or index >= len(right):
                result.append({'field': f'{path}[{index}]', 'off_present': index < len(left),
                               'on_present': index < len(right), 'off': left[index:] if index < len(left) else None,
                               'on': right[index:] if index < len(right) else None})
                break
            result.extend(differences(left[index], right[index], f'{path}[{index}]'))
        return result
    return [] if left == right else [{'field': path, 'off': left, 'on': right}]


def by_pass_name(trace):
    groups = defaultdict(list)
    for record in trace['records']:
        groups[record['name']].append(record)
    return {'presentation_only_not_semantic_match': True,
            'scope': 'Includes startup/warmup; equal names may describe distinct scopes/graphs/passes; no receipt join',
            'groups': [{
                'name': name, 'count': len(rows),
                'identities': [{k: row[k] for k in ('scope_id', 'graph', 'pass', 'command', 'line')} for row in rows],
                'native': {field: {'min': min(row['workload'][field] for row in rows),
                                   'max': max(row['workload'][field] for row in rows), 'count': len(rows)}
                           for field in native_trace.FIELDS},
            } for name, rows in sorted(groups.items())]}


def compare_sources(off, on, canonical_layout=False):
    # Both newly requested reports must prove the hook actually returned.
    # The original strict input comparison below still includes ALL raw stamps.
    if canonical_layout:
        canonical_preparation(off)
        canonical_preparation(on)
    a, b = counts.validate_report(off, False), counts.validate_report(on, True)
    left, right = wall_observations(off), wall_observations(on)
    for key in ('input', 'workload', 'management_modes'):
        require(a[key] == b[key], 'OFF/ON semantic input/default mode mismatch: ' + key)
    identities = lambda rows: [row['identity'] for row in rows]
    require(identities(a['actual_workload']['frames']) == identities(b['actual_workload']['frames']),
            'OFF/ON observation identities differ')
    native = []
    for index, (x, y) in enumerate(zip(a['actual_workload']['frames'], b['actual_workload']['frames'])):
        for field in counts.NATIVE:
            if x['native_commands'][field] != y['native_commands'][field]:
                native.append({'observation': index, **x['identity'], 'field': field,
                               'off': x['native_commands'][field], 'on': y['native_commands'][field]})
    actual = differences(a['actual_workload'], b['actual_workload'])
    wall = differences(left, right)
    return {'input_manifest_sha256': counts.accounting.signature(a['input']),
            'input_manifest_equal': True, 'actual_native_differences': native,
            'actual_workload_differences': actual, 'wall_source_differences': wall,
            'normalized_actual_workload_equal': not actual, 'wall_source_equal': not wall,
            'binding_counter_totals': {'off': a['counter_totals'], 'on': b['counter_totals']},
            'BLAKE3_scope': 'fixture-declared digest encoding checked; raw ordered range arrays compared directly; artifact SHA256 independently computed',
            'interpretation': 'Differences are investigation findings, never performance gains or permission to reclassify an earlier failed comparison'}


def capture_role(args, role, frozen_identity, tools, canonical_source=None):
    canonical_layout = getattr(args, 'canonical_layout', False)
    require(tool_identities(canonical_layout) == tools, 'trace/source controller dependencies changed')
    path = runner.PERF / 'headless-runs' / runner.label(args.series + '-' + role)
    record = None
    try:
        if canonical_layout:
            require(canonical_source is not None
                    and canonical_source_evidence(args, frozen_identity) == canonical_source,
                    'canonical source capability changed before capture')
        with trace_environment(canonical_layout):
            report, record = counts.capture_mode(args, role, frozen_identity, counts.tool_identities())
        wall_observations(report)
        if canonical_layout:
            record['canonical_preparation'] = canonical_preparation(report)
            require(canonical_source_evidence(args, frozen_identity) == canonical_source,
                    'canonical source capability changed during capture')
        parsed = native_trace.parse_trace((path / 'fixture.log').read_text(encoding='utf-8', errors='strict'))
        runner.dump(path / 'native-pass-trace.json', parsed)
        require(parsed.get('trace_valid') is True and parsed.get('errors') == []
                and parsed.get('submission_receipt_association') == 'not_available_scope_only',
                'native pass trace invalid or unexpected receipt claim')
        runner.dump(path / 'native-pass-name-view.json', by_pass_name(parsed))
        require(tool_identities(canonical_layout) == tools, 'trace/source controller dependencies changed during capture')
        require(counts.seal_capture(path) == record['artifacts'], 'raw capture changed during trace parsing')
        record.update(raw_run_valid=False, trace_valid=True,
                      trace_scope='recording scopes only, not GPU submission/completion or receipt association')
        return report, record
    finally:
        # Preserve failures as well as successes. Never modify raw run/session/log.
        counts.seal_capture(path)
        artifacts = {}
        for file in sorted(path.rglob('*')):
            if file.is_file() and not file.name.endswith('.sha256') and file.name != 'trace-source-artifacts.json':
                name = file.relative_to(path).as_posix()
                artifacts[name] = counts.frozen_tools.digest(file)
                file.with_name(file.name + '.sha256').write_text(artifacts[name] + '  ' + name + '\n', encoding='ascii')
        if path.is_dir():
            runner.dump(path / 'trace-source-artifacts.json', {'artifacts': artifacts})
        if record is not None:
            record['artifacts'] = artifacts


def run(args):
    require(args.scene == 'static' and args.warmup == 60 and args.samples == 30 and args.timeout > 0,
            'this investigation requires static/60 warmup/30 samples')
    destination = runner.PERF / 'binding-trace-source' / runner.label(args.series)
    paths = [runner.PERF / 'headless-runs' / runner.label(args.series + '-' + role) for role in ('off', 'on')]
    require(not any(path.exists() for path in paths), 'fresh per-role output directories required')
    build_path = runner.PERF / 'headless-builds' / runner.label(args.build_label) / 'build.json'
    _, identity = counts.frozen_tools.frozen_identity(build_path)
    canonical_layout = getattr(args, 'canonical_layout', False)
    tools = tool_identities(canonical_layout)
    destination.mkdir(parents=True, exist_ok=False)
    result = {'schema': 'binding-trace-source-diagnostic-v1', 'valid': False,
              'measurement_class': 'trace_source_diagnostic', 'diagnostic_only': True,
              'clean_cpu_acceptance': False, 'cpu_performance_evidence': False,
              'production_16ms_acceptance': False, 'formal_source_A_B': False,
              'same_frozen_binary_algorithm_ablation': True, 'original_count_comparison_reclassified': False,
              'frozen_build': identity, 'tools': tools,
              'settings': {'scene': 'static', 'warmup': 60, 'samples': 30, 'order': ['off', 'on'],
                           'etw': False, 'cpu_bill': True, 'RHAPSODY_SUBMIT_TRACE': '1'}, 'runs': []}
    try:
        canonical_source = None
        if canonical_layout:
            canonical_source = canonical_source_evidence(args, identity)
            archive_path = destination / 'canonical-source-archive.json'
            runner.dump(archive_path, canonical_source)
            result['canonical_source_archive'] = {'path': str(archive_path),
                'sha256': counts.frozen_tools.digest(archive_path), 'contract': canonical_source['contract']}
            result['settings']['TG_PERF_CANONICAL_WALL_LAYOUT'] = '1'
            result['canonical_preparation_scope'] = (
                'Both fresh processes explicitly request the same reviewed pre-warmup layout policy; '
                'runtime owner preflight success is required, not assumed; preparation is not final layout')
        reports = {}
        for role, path in zip(('off', 'on'), paths):
            entry = {'role': role, 'path': str(path), 'capture_completed': False}
            result['runs'].append(entry)
            runner.dump(destination / 'progress.json', result)
            if canonical_layout:
                reports[role], record = capture_role(args, role, identity, tools, canonical_source)
            else:
                reports[role], record = capture_role(args, role, identity, tools)
            entry.update(record, capture_completed=True)
            runner.dump(destination / 'progress.json', result)
        result['source_investigation'] = compare_sources(reports['off'], reports['on'], canonical_layout)
        if canonical_layout:
            require(canonical_source_evidence(args, identity) == canonical_source,
                    'canonical source capability changed before finalization')
            require(counts.frozen_tools.digest(archive_path) == result['canonical_source_archive']['sha256'],
                    'canonical source archive changed')
        require(tool_identities(canonical_layout) == tools, 'trace/source tools changed before finalization')
        result['valid'] = True
    except BaseException as error:
        result['error'] = repr(error)
        raise
    finally:
        for entry in result['runs']:
            sidecar = Path(entry['path']) / 'trace-source-artifacts.json'
            if sidecar.is_file():
                entry['artifact_manifest'] = {'path': str(sidecar), 'sha256': counts.frozen_tools.digest(sidecar)}
        runner.dump(destination / 'summary.json', result)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('build_label')
    parser.add_argument('series')
    parser.add_argument('--timeout', type=int, default=3600)
    parser.add_argument('--canonical-layout', action='store_true',
                        help='explicitly request reviewed rich fresh-boot canonical preparation in both diagnostic processes')
    args = parser.parse_args()
    args.scene, args.warmup, args.samples = 'static', 60, 30
    print(json.dumps(run(args), ensure_ascii=False, indent=2))


if __name__ == '__main__':
    main()
