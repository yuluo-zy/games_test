"""Check current source/input identities and the joint R7 CPU/API contracts."""
from pathlib import Path
import hashlib
import json
import tomllib

ROOT = Path(__file__).resolve().parents[2]
BATCH = ROOT / 'evidence/render-recovery/parallel-batch-05'
EXPECTED = 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
INPUTS = {}
CHECKS = []

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def retain(path):
    path = path if path.is_absolute() else ROOT / path
    key = str(path.relative_to(ROOT)).replace('\\', '/') if path.is_relative_to(ROOT) else str(path)
    INPUTS[key] = {'path': str(path), 'sha256': sha(path)}
    return path

def load(relative):
    return json.loads(retain(Path(relative)).read_text(encoding='utf-8'))

def check(name, condition, detail):
    CHECKS.append({'name': name, 'ok': bool(condition), 'detail': detail})

def main():
    fourth = load('evidence/render-recovery/parallel-batch-04/integration.json')
    check('fourth_batch_gate', fourth['status'] == 'consistent_partial' and not fourth['failed_checks'], 'Fourth batch gate must pass independently before this batch is accepted.')
    fourth_stale = [x['path'] for x in fourth['inputs'].values() if sha(retain(Path(x['path']))) != x['sha256']]
    check('fourth_gate_inputs_current', not fourth_stale, fourth_stale)
    target = retain(ROOT / 'workspace/input/tiny-glade.exe')
    check('original_identity', sha(target) == EXPECTED, EXPECTED)

    queue = load('evidence/render-recovery/parallel-batch-05/queue-callers/validation.json')
    inventory = load('evidence/render-recovery/parallel-batch-05/queue-callers/callsite-inventory.json')
    queue_source = ROOT / 'reconstruction/render/work-packages/queue-callers-r7/src'
    check('queue_source_current', all(sha(retain(queue_source / name)) == digest for name, digest in queue['source_sha256'].items()), 'All caller candidate Rust source hashes match its native oracle.')
    check('queue_harness_current', sha(retain(BATCH / 'queue-callers/verify.py')) == queue['harness_sha256'], 'Native harness hash matches.')
    check('queue_original_entry', queue['status'] == 'verified_partial' and queue['target_sha256'] == EXPECTED and queue['original_entry_cases'] == 180 and queue['mismatches'] == 0, '180 complete ensure_active_cb caller-entry fixtures; creation/begin remain injected boundaries.')
    check('callgraph_inputs_current', all(sha(retain(ROOT / f'evidence/render-recovery/workgraph/{name}.json')) == inventory[f'{name}_sha256'] for name in ['functions', 'calls']), 'Direct caller evidence uses current indexed graph.')
    check('bounded_primary_callers', len(inventory['sites']) == 9 and all(x['observed_queue_tag'] == 0 for x in inventory['sites']) and queue['direct_begin_sites'] == 8 and queue['direct_submit_sites'] == 1, 'Only the nine indexed direct sites are established as primary; indirect/inlined callers remain unknown.')

    record = load('evidence/render-recovery/parallel-batch-05/record-resources/validation.json')
    stale = [name for name, digest in record['hashes'].items() if sha(retain(ROOT / name)) != digest]
    check('record_sources_and_dependencies_current', not stale, stale)
    check('record_native_joint_path', record['passed'] and record['original_sha256'] == EXPECTED and record['cases'] == 774 and record['failure_count'] == 0 and not record['old_108_cases_recounted'], '774 new nonempty-resource fixtures; previous 108 cases excluded.')
    check('record_nonempty_and_export', record['nonempty_pass_access_cases'] == 726 and record['no_access_export_fallback_cases'] == 48 and record['nonempty_resource_cases'] == record['cases'], 'Nonempty per-pass access and export fallback without last access are both covered.')
    check('record_transfer_and_callback_checks', record['record_to_submit_cases'] == 330 and record['rust_tests'] == 5 and all(x['exit_code'] == 0 for x in record['commands']), '330 original-command transfers plus five Rust tests, strict Clippy and format checks.')
    check('record_original_entries', set(record['original_entries']) == {'0x1402c1d40', '0x1402c3240', '0x1402c3d00', '0x140bd6a70', '0x140bebdf0'}, 'Record/helper/pre/tracking/submit originals execute; private device boundaries are separately declared.')
    check('queue_record_domain_consistency', record['begin_tags_observed'] == [0] and record['pool_selectors_observed'] == [0], 'Record begin selects Primary, consistent with bounded caller inventory; pool selector result is a fixture.')
    check('sequence_fixture_boundary', record['sequence_is_original_initialization'] is False and record['fixture_command_sequence'] == 7 and fourth['integration_boundaries'][0]['production_connected'] is False, 'Injected sequence 7 does not replace original seed 1 or prove production sequence adapter wiring.')

    pipeline = load('evidence/render-recovery/parallel-batch-05/pipeline-descriptors/native-validation.json')
    pipeline_validation = load('evidence/render-recovery/parallel-batch-05/pipeline-descriptors/validation.json')
    pipeline_source = ROOT / 'reconstruction/render/work-packages/pipeline-descriptors-r7'
    check('layout_source_current', sha(retain(pipeline_source / 'src/lib.rs')) == pipeline['source_sha256'], 'Layout/pool/compute candidate hash matches latest native oracle.')
    check('layout_original_entry', pipeline['binary_sha256'] == EXPECTED and pipeline['original_entry'] == '0x1418c2150' and pipeline['original_return'] == '0x1418c2668' and pipeline['cases'] == 4438 and pipeline['mismatches'] == 0, 'Complete persistent-layout CPU entry through actual Vulkan create-info capture and normal return.')
    check('layout_failure_stops', pipeline['vulkan_failure_branches'] == [-1, -2, -3, -13], 'Four Vulkan errors stop at original panic boundary; unwind not claimed.')
    reflected = load('evidence/render-recovery/parallel-batch-04/pipeline-shader-interfaces/validation.json')
    check('reflection_adapter_coverage', pipeline['catalog_declaration_cases'] == reflected['coverage']['modules'] == 638, '638 declaration fixtures are candidate adapters; sparse bindings and real pass selection remain separate unknowns.')
    boundaries = pipeline['boundaries']
    check('pool_and_compute_boundaries', boundaries['mismatches'] == 0 and boundaries['persistent_pool_entry_cases'] == 486 and boundaries['compute_layout_slice_cases'] == 256 and boundaries['cases'] == 742, '486 original pool entries with injected layouts; 256 injected-frame compute-layout CPU slices.')
    # Independently compare package identity, source and checksum against the original lock.
    original = tomllib.loads(retain(ROOT / 'tracking/original-Cargo.lock').read_text(encoding='utf-8'))['package']
    packages = tomllib.loads(retain(pipeline_source / 'Cargo.lock').read_text(encoding='utf-8'))['package']
    registry = [p for p in packages if 'source' in p]
    exact = all(any(all(p.get(k) == o.get(k) for k in ['name', 'version', 'source', 'checksum']) for o in original) for p in registry)
    check('original_dependency_reuse', exact and len(registry) == 21, 'Twenty-one dependency identities match original Cargo.lock, including ash/serde; no replacement library recovery.')
    # The final package validator records its complete local command outputs/hashes.
    pipeline_hashes = load('evidence/render-recovery/parallel-batch-05/pipeline-descriptors/checksums.json')
    check('pipeline_validation_inputs_current', bool(pipeline_hashes) and all(sha(retain(ROOT / name)) == digest for name, digest in pipeline_hashes.items()), 'Package validator records exact source/harness/dependency inputs.')
    pipeline_rust = load('evidence/render-recovery/parallel-batch-05/pipeline-descriptors/rust-validation.json')
    check('pipeline_build_checks', pipeline_validation['status'] == 'passed' and pipeline_validation['native_cases_total'] == 5184 and pipeline_rust['rust_tests'] == 2 and all(x['exit_code'] == 0 for x in pipeline_rust['commands']), 'Final pipeline candidate tests, Clippy/fmt outputs must all pass; all 5,184 native cases have zero differences.')
    mcp = load('evidence/render-recovery/parallel-batch-05/pipeline-descriptors/mcp-report.json')
    closes = [x for x in mcp['results'] if x.get('name') == 'close_database']
    check('pipeline_actual_mcp_closed', mcp['actual_connection'] is True and closes and all(x['args']['save'] is False and not x['result'].get('isError', False) for x in closes), 'Actual configured RE-MCP connection and successful unsaved close are recorded.')
    check('pipeline_binding_limits', pipeline_validation['reflection_linking_level'] == 'candidate_declaration_adapter_only_not_real_sparse_index_or_pass_binding' and pipeline_validation['full_shader_pipeline_create'] is False and pipeline_validation['descriptor_writes_recovered'] is False and pipeline_validation['real_vulkan_gpu_acceptance'] is False, 'Adapter coverage and CPU API capture do not claim complete pipeline/descriptor writes or real game-pass binding.')

    tooling = load('evidence/render-recovery/tooling/runner-validation.json')
    check('tooling_failure_and_cache_checks', tooling['failed'] == 0 and tooling['passed'] >= 27 and tooling['fixture_only'] is True and tooling['original_target_connected'] is False, 'Tool tests exercise a controlled local MCP fixture, separately from actual original analysis.')
    check('tooling_sources_current', all(sha(retain(Path(name))) == digest for name, digest in tooling['sources'].items()), 'Tested runner and local protocol fixture sources match their hashes.')
    retain(ROOT / 'scripts/render-recovery/mcp-evidence-job.mjs')
    retain(Path(__file__).resolve())
    failed = [x['name'] for x in CHECKS if not x['ok']]
    report = {'schema_version': 1, 'status': 'consistent_partial' if not failed else 'inconsistent', 'scope': 'R7 private caller -> nonempty sequential record/submit and descriptor layout/pool/compute API contracts; tooling and fourth-batch prerequisite', 'target_sha256': EXPECTED, 'inputs': INPUTS, 'checks': CHECKS, 'failed_checks': failed, 'counts': {'queue_caller_entry_cases': 180, 'nonempty_record_cases': 774, 'original_record_to_submit_transfers': 330, 'layout_entry_cases': 4438, 'layout_vulkan_failure_stops': 4, 'pool_entry_cases': 486, 'compute_injected_slice_cases': 256}, 'production_gpu_connected': False, 'remaining': ['complete queue-submit prefix and parallel recording', 'explicit production sequence consumer/seed adapter', 'multi-queue simultaneous caller reachability and bucket collision policy', 'temporal reset/drop and exception ownership', 'descriptor writes, shader module/full pipeline creation', 'country-core sparse binding/pipeline/game-pass selection', 'GPU execution and image/performance comparison']}
    (BATCH / 'integration.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps({'status': report['status'], 'checks': len(CHECKS), 'failed': failed}))
    if failed:
        raise SystemExit(1)

if __name__ == '__main__':
    main()
