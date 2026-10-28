"""Strict reader for the receipt-bound actual-workload producer (prospective).

It preserves raw evidence and partial coverage. Native API invocations never
become an estimate of indirect GPU draw counts or a complete upload total.
"""
import hashlib
import json
from pathlib import Path

import perf_canonical_contract as canonical

SCHEMA = 'headless-actual-workload-v1'
ALGORITHM_SELECTION = 'compiled_source_defaults_no_runtime_ablation'
COMMON_HARNESS_FILES = tuple('crates/tiny-glade/tests/cpu_measurement/'+name
                            for name in ('mod.rs', 'workload_probe.rs', 'gpu_diagnostic.rs'))
DRAW_APIS = ('direct_draw_api_calls', 'indexed_draw_api_calls', 'indirect_draw_api_calls',
    'indirect_count_draw_api_calls', 'indexed_indirect_draw_api_calls',
    'indexed_indirect_count_draw_api_calls')
DISPATCH_APIS = ('direct_dispatch_api_calls', 'indirect_dispatch_api_calls')
GRAPH_BYTES = ('staging_h2d_copy_recorded_bytes', 'immutable_producer_h2d_copy_recorded_bytes',
               'cached_d2d_copy_recorded_bytes')
DEVICE_BYTES = ('setup_submitted_payload_bytes', 'direct_region_submitted_payload_bytes',
                'transfer_submitted_payload_bytes', 'buffer_setup_submitted_payload_bytes')
COLLECT_FIELDS = ('tasks', 'waves', 'cross_pass_tasks', 'cross_pass_waves', 'failed_waves')
GRAPH_COST = ('graph_commands_recorded', 'graph_recording_chunks', 'graph_parallel_batches',
              'graph_batches', 'graph_culled_passes')
FIELD_GROUPS = {'native_commands': 'native-api-invocations-and-graph-copy-payload',
    'draw_list_lengths': 'actual-21-ECS-draw-list-lengths',
    'graph_retained_passes': 'retained-render-graph-passes',
    'graph_recording': 'successful-recording-statistics', 'collect': 'owner-TLS-joined-Collect-work',
    'queued_buffer_h2d': 'same-manager-successful-submit-payload',
    'device_upload': 'same-device-successful-native-submit-payload'}


def require(condition, message):
    if not condition:
        raise ValueError(message)


def read(path):
    return json.loads(Path(path).read_text(encoding='utf-8'))


def signature(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(',', ':'),
                                     ensure_ascii=False, allow_nan=False).encode()).hexdigest()


def digest(path):
    with Path(path).open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def checked_reference(ref):
    path = Path(ref['path']).resolve()
    require(digest(path) == ref['sha256'], 'producer review artifact SHA changed: '+str(path))
    return path


def uint(value, field):
    require(type(value) is int and 0 <= value <= (1 << 64)-1, 'invalid u64 '+field)
    return value


def flags(value, field, *, require_valid=True):
    require(isinstance(value, dict), 'missing producer object: '+field)
    if require_valid or 'valid' in value:
        require(value.get('valid') is True, 'producer invalid/missing valid: '+field)
    if 'overflow' in value:
        require(value['overflow'] is False, 'producer overflow: '+field)
    if 'errors' in value:
        require(type(value['errors']) is list and not value['errors'], 'producer errors: '+field)
    if 'invalid_reasons' in value:
        require(type(value['invalid_reasons']) is list and not value['invalid_reasons'],
                'producer invalid reasons: '+field)


def review_capability(path, build, source_files):
    """Bind audited field definitions to archived source, not arbitrary symbols.

    Review is an explicit human/source-audit prerequisite, not an eligibility
    bool supplied by a runtime report. Every referenced artifact is SHA checked.
    """
    cap = read(path)
    require(Path(canonical.__file__).resolve() == Path(__file__).with_name('perf_canonical_contract.py').resolve(),
            'shadowed canonical contract from another checkout')
    version = cap.get('schema')
    require(version in ('headless-producer-capability-v1', 'headless-producer-capability-v2')
            and cap.get('raw_schema') == SCHEMA, 'unreviewed/unknown workload capability')
    canonical_version = version == 'headless-producer-capability-v2'
    require(cap['source_content_sha256'] == build['source_content_sha256'],
            'producer capability belongs to another frozen source')
    require(cap['source_commit'] == build['source_commit'], 'producer review source commit differs')
    require(cap['algorithm_selection'] == ALGORITHM_SELECTION,
            'compiled-source default algorithm contract not reviewed')
    files = cap['source_files']
    require(isinstance(files, dict) and bool(files), 'producer source archive missing')
    for relative, ref in files.items():
        require(relative in source_files and ref['sha256'] == source_files[relative],
                'producer archived source differs from frozen manifest: '+relative)
        checked_reference(ref)
    require(all(name in files for name in COMMON_HARNESS_FILES), 'common harness source archive incomplete')
    common_mod = Path(files[COMMON_HARNESS_FILES[0]]['path']).read_text(encoding='utf-8')
    require(ALGORITHM_SELECTION in common_mod, 'common source lacks versioned algorithm selection marker')
    review = read(checked_reference(cap['review_report']))
    require(review.get('schema') == ('headless-producer-source-review-v2' if canonical_version else 'headless-producer-source-review-v1')
            and review['source_content_sha256'] == build['source_content_sha256']
            and review['source_commit'] == build['source_commit'], 'producer source review identity differs')
    require(review['source_sha256'] == {name: ref['sha256'] for name, ref in files.items()},
            'review does not cover exact producer source set')
    require(type(review['findings']) is list and not review['findings'], 'producer review has unresolved findings')
    require(set(cap['fields']) == set(FIELD_GROUPS) and cap['fields'] == review['fields'],
            'producer field capability differs from source review')
    for group, origin in cap['fields'].items():
        require(origin['meaning'] == FIELD_GROUPS[group] and origin['symbols']
                and type(origin['symbols']) is list
                and origin['source_paths'] and type(origin['source_paths']) is list,
                'unknown producer field semantics: '+group)
        require(all(type(symbol) is str and symbol for symbol in origin['symbols'])
                and all(name in files for name in origin['source_paths']), 'unbound producer symbol/source')
    require(cap['coverage'] == review['coverage'], 'producer coverage differs from reviewed source')
    require(review['algorithm_selection'] == ALGORITHM_SELECTION
            and review['runtime_ablation_supported'] is False
            and review['controller_disable_controls_absent_checked'] is True,
            'source-default/absent-ablation review missing')
    require(review.get('receipt_attribution_checked') is True
            and review.get('successful_prefix_failure_invalid_checked') is True
            and review.get('counter_overflow_checked') is True, 'producer review obligations incomplete')
    if canonical_version:
        canonical.capability_contract(cap, review, source_files)
    else:
        require(cap.get('canonical_source_contract') is None
                and review.get('canonical_source_contract') is None
                and canonical.HARNESS not in common_mod
                and 'before_first_gameplay_warmup' not in common_mod,
                'canonical producer cannot claim historical v1 contract')
    return cap


def harness_contract(cap):
    return cap.get('canonical_source_contract')


def common_source(cap, source_files):
    names = canonical.SHARED if harness_contract(cap) is not None else COMMON_HARNESS_FILES
    return {name: source_files[name] for name in names}


def harness(cap):
    return canonical.HARNESS if harness_contract(cap) is not None else canonical.LEGACY_HARNESS


def device_snapshot(snapshot, field):
    flags(snapshot, field)
    uint(snapshot.get('generation'), field+'.generation')
    require(snapshot['generation'] > 0, 'device generation is unavailable')
    for key in DEVICE_BYTES:
        require(uint(snapshot[key], field+'.'+key) < (1 << 64)-1,
                'reserved device counter overflow sentinel')
    return snapshot


def queued_snapshot(snapshot, field):
    flags(snapshot, field, require_valid=False)
    for key in ('manager_id', 'submitted_jobs', 'submitted_payload_bytes'):
        uint(snapshot[key], field+'.'+key)
    require(snapshot['manager_id'] != 0 or snapshot['submitted_jobs'] == snapshot['submitted_payload_bytes'] == 0,
            'zero manager with nonzero submitted payload')
    return snapshot


def validate_actual(session, cpu, cap):
    if harness_contract(cap) is not None:
        canonical.session_preparation(session)
    else:
        require(session.get('canonical_brick_layout') is None
                and session.get('canonical_layout_input_policy') is None,
                'historical v1 producer cannot be upgraded by a preparation annotation')
    raw = session.get('actual_workload')
    require(isinstance(raw, dict) and raw.get('schema') == SCHEMA,
            'actual_workload producer absent; legacy proof cannot be upgraded')
    flags(raw, 'actual_workload')
    require(type(raw.get('errors')) is list and not raw['errors'], 'workload errors absent or nonempty')
    samples = session['samples']
    require(type(samples) is int and samples > 0, 'invalid workload sample count')
    require(raw['capacity'] == raw['frame_count'] == samples+1
            and raw['truncated'] is False and len(raw['frames']) == samples+1,
            'workload frame/capacity mismatch or truncation')
    require(raw['coverage'] == cap['coverage'], 'raw workload coverage differs from audited capability')
    coverage = raw['coverage']
    require(type(coverage['included']) is list and type(coverage['excluded']) is list
            and coverage['no_all_upload_total'] is True
            and coverage['collect_cross_pass_is_subset'] is True
            and coverage['graph_chunks_are_not_collect_tasks'] is True,
            'unsupported coverage/completeness claim; new producer review required')
    # Current source explicitly has no all-upload total. This schema may be
    # used for attribution, but cannot silently satisfy the original full gate.
    pending = ['all-GPU-upload coverage is incomplete; current producer explicitly supplies no total',
               'native draw API invocations do not prove indirect GPU draw/instance counts or argument payload identity']
    require(len(session['frames']) == samples+1 and len(cpu['frames']) == samples,
            'CPU/workload observation count mismatch')
    flags(raw['collect_session_end'], 'collect_session_end')
    require(raw['collect_session_end']['overflow'] is False
            and raw['collect_session_end']['failed_waves'] == 0, 'Collect session failed')
    for key in COLLECT_FIELDS:
        uint(raw['collect_session_end'][key], 'collect_session_end.'+key)
    before_queue = queued_snapshot(raw['queued_upload_before_measurement'], 'queued before')
    before_device = device_snapshot(raw['device_upload_before_measurement'], 'device before')
    previous_collect = None
    semantics, payload, cost = [], [], []
    for index, row in enumerate(raw['frames']):
        label = 'workload frame'+str(index)
        flags(row, label)
        expected = session['frames'][index]
        for key in ('world_tick', 'receipt', 'attempt'):
            uint(row[key], label+'.'+key)
            require(row[key] == expected[key], 'workload/session receipt mismatch: '+key)
            if index:
                require(row[key] == cpu['frames'][index-1][key], 'workload/CPU receipt mismatch: '+key)
        native = row['native_commands']; flags(native, label+'.native_commands')
        for key in (*DRAW_APIS, *DISPATCH_APIS, *GRAPH_BYTES):
            uint(native[key], label+'.'+key)
        for key in (*GRAPH_COST, 'graph_retained_passes'):
            uint(row[key], label+'.'+key)
        require(row['graph_commands_recorded'] > 0, 'no successful graph commands recorded')
        lengths = row['draw_list_lengths']
        require(type(lengths) is list and len(lengths) == 21, '21 actual draw-list lengths required')
        for length in lengths: uint(length, label+'.draw_list_lengths')
        if session['scenario'] == 'many_draws':
            require(sum(lengths) >= 1000, 'many_draws actual list entries below1000')
        collected = row['collect']; flags(collected, label+'.collect')
        require(collected['overflow'] is False and collected['failed_waves'] == 0, 'failed/overflowed Collect work')
        for key in COLLECT_FIELDS: uint(collected[key], label+'.collect.'+key)
        require(collected['waves'] <= collected['tasks'] <= 8*collected['waves']
                and collected['cross_pass_waves'] <= collected['cross_pass_tasks'] <= 8*collected['cross_pass_waves']
                and collected['cross_pass_tasks'] <= collected['tasks']
                and collected['cross_pass_waves'] <= collected['waves'], 'Collect subset/wave bounds invalid')
        cumulative = row['collect_cumulative']; flags(cumulative, label+'.collect_cumulative')
        require(cumulative['overflow'] is False and cumulative['failed_waves'] == 0, 'Collect cumulative invalid')
        for key in COLLECT_FIELDS:
            uint(cumulative[key], label+'.collect_cumulative.'+key)
            previous = previous_collect[key] if previous_collect is not None else cumulative[key]-collected[key]
            require(0 <= previous <= cumulative[key] and cumulative[key]-previous == collected[key],
                    'Collect cumulative/delta mismatch')
        previous_collect = cumulative
        queued = queued_snapshot(row['queued_upload_snapshot'], label+'.queued')
        require(before_queue['manager_id'] == 0 or queued['manager_id'] == before_queue['manager_id'],
                'queued upload manager changed within run')
        queue_delta = {}
        for key, output in (('submitted_jobs', 'queued_buffer_h2d_submitted_jobs'),
                            ('submitted_payload_bytes', 'queued_buffer_h2d_submitted_payload_bytes')):
            uint(row[output], label+'.'+output)
            require(queued[key] >= before_queue[key]
                    and queued[key]-before_queue[key] == row[output], 'queued upload cumulative/delta mismatch')
            queue_delta[output] = row[output]
        device = device_snapshot(row['device_upload_snapshot'], label+'.device')
        delta = device_snapshot(row['device_upload_submitted_payload'], label+'.device_delta')
        require(device['generation'] == delta['generation'] == before_device['generation'], 'device generation changed')
        for key in DEVICE_BYTES:
            require(device[key] >= before_device[key] and device[key]-before_device[key] == delta[key],
                    'device upload cumulative/delta mismatch')
        before_queue, before_device = queued, device
        if index == 0:
            continue  # row0 is a true workload observation but not a CPU sample.
        attribution = {key: row[key] for key in ('world_tick', 'receipt', 'attempt')}
        attribution['app_epoch'] = cpu['frames'][index-1]['app_epoch']
        semantics.append({**attribution, 'native_draw_api_calls': {key: native[key] for key in DRAW_APIS},
            'graph_retained_passes': row['graph_retained_passes'], 'draw_list_lengths': lengths})
        payload.append({**attribution, 'graph': {key: native[key] for key in GRAPH_BYTES},
            'queued_buffer_h2d_submitted_payload_bytes': queue_delta['queued_buffer_h2d_submitted_payload_bytes'],
            'device': {key: delta[key] for key in DEVICE_BYTES}})
        cost.append({**attribution, 'collect': {key: collected[key] for key in COLLECT_FIELDS},
            'dispatch_api_calls': {key: native[key] for key in DISPATCH_APIS},
            'graph_recording': {key: row[key] for key in GRAPH_COST},
            'queued_buffer_h2d_submitted_jobs': queue_delta['queued_buffer_h2d_submitted_jobs']})
    queued_snapshot(raw['queued_upload_end'], 'queued end')
    device_snapshot(raw['device_upload_end'], 'device end')
    require(raw['queued_upload_end'] == before_queue and raw['device_upload_end'] == before_device,
            'final upload cumulative snapshot mismatch')
    require(raw['collect_session_end'] == previous_collect, 'Collect end includes unobserved work')
    return {'raw': raw, 'semantic_frames': semantics, 'payload_frames': payload, 'cost_frames': cost,
            'coverage_pending': pending, 'coverage': coverage,
            'native_api_note': 'six native draw API invocation categories, not physical GPU draw counts'}


def compare_workload(a, b, explanation=None):
    require(a['semantic_frames'] == b['semantic_frames'],
            'actual native draw API/pass/list workload differs')
    require(a['coverage'] == b['coverage'], 'A/B producer coverage differs')
    payload_equal = a['payload_frames'] == b['payload_frames']
    cost_equal = a['cost_frames'] == b['cost_frames']
    return {'semantic_equal': True, 'h2d_d2d_payload_equal': payload_equal,
        'cpu_task_and_recording_cost_equal': cost_equal,
        'coverage_pending': sorted(set(a['coverage_pending']+b['coverage_pending'])),
        'payload_review_required': not payload_equal,
        'cost_difference_review_required': not cost_equal,
        'cost_difference_explanation': explanation,
        'explanation_is_approval': False,
        'review_note': 'CPU task reduction may be a valid optimization; free text never accepts payload or task differences.'}
