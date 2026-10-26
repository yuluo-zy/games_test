"""Run/audit one frozen same-prefix diagnostic, never build or run Cargo.

Local success cannot reclassify any whole-scene, WSI, CPU, or long-run result.
"""
import argparse
from datetime import datetime, timezone
import json
from pathlib import Path
import re
import subprocess
import sys

TEST = 'real_lake_surface_same_prefix_zero_tolerance'
PAIRS = ('inline-inline', 'collect-collect', 'inline-collect', 'collect-inline')


def require_local_build(identity):
    from surface_frozen_v2 import LOCAL_ADAPTER_SHA, LOCAL_POLICY_SHA
    proof = identity.get('build_evidence', {})
    if (proof.get('schema') != 'surface-v2-cold-build-evidence-local-v1' or
            proof.get('adapter_sha256') != LOCAL_ADAPTER_SHA or
            proof.get('local_adapter_policy_sha256') != LOCAL_POLICY_SHA or
            proof.get('fixture_entry') != TEST or
            proof.get('fixture_mode') != 'same-prefix-local-correctness'):
        raise ValueError('same-prefix runner requires its exact reviewed local-v2 build/ENTRY proof')


def physical_extent_from_manifest(manifest, number):
    """Bind native readback to actual per-frame inputs and original32 alignment.

    Nominal dimensions/scale are fixed acceptance inputs, never arbitrary
    manifest knobs. For this original Off/scale1 path the physical image is
    1280x736. A720-row crop or self-consistent1x1 substitution is invalid.
    """
    nominal = [1280,720]
    if manifest.get('world_scale') != 1.0 or manifest.get('fixed_step') != [1,60]:
        raise ValueError('nominal scale/fixed-step changed')
    captures=manifest.get('pixels',[])
    if (len(captures)!=2 or {p.get('frame') for p in captures}!={20,30} or
            any([p.get('width'),p.get('height')]!=nominal for p in captures)):
        raise ValueError('nominal output must remain1280x720')
    commands=manifest.get('commands',[])
    if len(commands)!=30 or [c.get('frame') for c in commands]!=list(range(30)):
        raise ValueError('complete original30 input observations required')
    recorded=commands[number-1]['input']
    source=recorded.get('resolution_source',{})
    expected=[max(32,min(10912,(axis+31)&~31)) for axis in nominal]
    words=recorded.get('frame_constant_words',[])
    if (len(words)!=1060 or any(type(v) is not int or not 0<=v<=0xffffffff for v in words) or
            recorded.get('inner_resolution')!=expected or words[36:38]!=expected or
            source.get('nominal_extent')!=nominal or source.get('effective_output_extent')!=nominal or
            source.get('physical_inner_extent')!=expected or source.get('alignment')!=32 or
            source.get('world_scale_bits')!=0x3f800000 or source.get('tgsr_scale_bits')!=0x3f800000 or
            recorded.get('temporal_source_resources',{}).get('upscaling_mode')!='Off'):
        raise ValueError('native config/constants extent differs from original aligned scale1 target')
    return expected


def raw_delta(a, b, texel_bytes=4):
    if len(a) != len(b) or len(a) % texel_bytes:
        raise ValueError('raw dimensions/texel bytes differ')
    return {'changed_bytes': sum(x != y for x, y in zip(a, b)),
            'changed_pixels': sum(a[i:i + texel_bytes] != b[i:i + texel_bytes]
                                  for i in range(0, len(a), texel_bytes)), 'tolerance': 0}


def semantic_errors(frame, season, pair):
    errors = []
    modes = pair.split('-')
    raster = frame.get('ordered_raster_declarations', [])
    if (frame.get('modes') != modes or len(raster) != 2 or
            any(not isinstance(branch, list) or len(branch) != 1 for branch in raster)):
        return ['missing_or_wrong_ordered_branches']
    if raster[0] != raster[1]:
        errors.append('ordered_raster_declarations_differ')
    name = 'Ice plane' if season == 'ice' else 'Water plane'
    if frame.get('actual_retained_pass_order', []).count(name) != 2:
        errors.append('both_surface_passes_not_retained')
    execution = frame.get('actual_surface_execution', [])
    if len(execution) != 2:
        return errors + ['actual_branch_execution_missing']
    for branch, (passes, mode, actual) in enumerate(zip(raster, modes, execution)):
        p = passes[0]
        draws = p.get('draws', [])
        if p.get('pass') != name or p.get('observation_valid') is not True or not draws:
            errors.append(f'{branch}_unsupported_or_empty_surface_observation')
        for draw in draws:
            if draw.get('native_stencil_test') is not False or draw.get('kind', {}).get('kind') != 3:
                errors.append(f'{branch}_unsupported_stencil_or_draw_kind')
            constants = draw.get('dynamic_constants', [])
            uniform_count=sum(b.get('value',{}).get('kind')=='dynamic_uniform' for b in draw.get('bindings',[]))
            if not constants or len(constants)!=uniform_count:
                errors.append(f'{branch}_native_constant_block_set_missing')
            for binding in draw.get('bindings', []):
                value = binding.get('value', {})
                if value.get('kind') != 'dynamic_uniform':
                    continue
                blocks = [row[1] for row in constants
                          if isinstance(row, list) and len(row) == 2 and row[0] == binding.get('binding')]
                if (len(blocks) != 1 or len(blocks[0]) != value.get('size') or not blocks[0] or
                        any(type(v) is not int or not 0 <= v <= 255 for v in blocks[0])):
                    errors.append(f'{branch}_native_constant_bytes_missing')
        n = len(draws)
        expected = ({'inline_calls': 1, 'inline_draws': n, 'worker_calls': 0,
                     'worker_tasks': 0, 'worker_draws': 0, 'owner_collect_tasks': 0}
                    if mode == 'inline' else
                    {'inline_calls': 0, 'inline_draws': 0, 'worker_calls': 1,
                     'worker_draws': n, 'owner_collect_tasks': 0})
        if (actual.get('requested_mode') != mode or actual.get('valid') is not True or
                any(actual.get(k) != v for k, v in expected.items()) or
                mode == 'collect' and type(actual.get('worker_tasks')) is not int or
                mode == 'collect' and actual.get('worker_tasks', 0) <= 0):
            errors.append(f'{branch}_requested_mode_not_actual_execution')
    histories = frame.get('actual_temporal_inputs', [])
    expected_names = {'Ice shadows', 'water refr temporal', 'ice refl ray len'} if season == 'ice' else set()
    if len(histories) != len(expected_names) or {h.get('name') for h in histories} != expected_names:
        errors.append('temporal_pair_set_differs_from_original')
    for h in histories:
        if (h.get('a_previous_rg') != h.get('b_previous_rg') or
                type(h.get('a_history_valid')) is not bool or
                h.get('a_history_valid') != h.get('b_history_valid') or
                h.get('a_current_rg') == h.get('b_current_rg')):
            errors.append('temporal_owner_flag_or_current_isolation_differs')
    if frame.get('undefined_history_read_possible') is not False:
        errors.append('original_undefined_history_read_not_resolved')
    return errors


def audit_season(folder, season, pair, sha256):
    errors, evidence = [], {}
    report_path = folder / 'same-prefix' / 'local-comparison.json'
    evidence[str(report_path.relative_to(folder))] = sha256(report_path)
    report = json.loads(report_path.read_text(encoding='utf-8'))
    manifest_path=folder/'manifest.json'
    manifest=json.loads(manifest_path.read_text(encoding='utf-8'))
    evidence['manifest.json']=sha256(manifest_path)
    if (report.get('schema') != 'same-prefix-surface-correctness-v1' or
            report.get('pixel_tolerance') != 0 or report.get('no_additional_read_wait_ns') != 0):
        errors.append('unsupported_local_protocol')
    frames = report.get('frames', [])
    if len(frames) != 2 or {f.get('frame') for f in frames} != {20, 30}:
        errors.append('observation_frames_missing_or_duplicated')
    for frame in frames:
        try:
            physical_extent=physical_extent_from_manifest(manifest,frame['frame'])
        except (ValueError,KeyError,IndexError,TypeError) as error:
            errors.append('invalid_physical_extent_source:'+str(error))
            continue
        raw_bytes=physical_extent[0]*physical_extent[1]*4
        errors.extend(f'frame{frame.get("frame")}:{e}' for e in semantic_errors(frame, season, pair))
        entries = frame.get('raw', [])
        if len(entries) != 4 or {(r.get('branch'), r.get('role')) for r in entries} != {
                (b, r) for b in (0, 1) for r in ('color', 'depth')}:
            errors.append('raw_role_set_missing_or_duplicated')
            continue
        data = {}
        for row in entries:
            role, branch, number = row['role'], row['branch'], frame['frame']
            expected_name = f'frame-{number}-{branch}-{role}.raw'
            expected_format, expected_aspect = (122, 1) if role == 'color' else (130, 2)
            if (row.get('file') != expected_name or row.get('extent') != physical_extent or
                    row.get('bytes') != raw_bytes or row.get('format') != expected_format or
                    row.get('aspect') != expected_aspect or row.get('receipt_owner_confirmed') is not True):
                errors.append('raw_native_format_range_or_owner_invalid')
                continue
            path = folder / 'same-prefix' / expected_name
            evidence[str(path.relative_to(folder))] = sha256(path)
            data[(branch, role)] = path.read_bytes()
            if len(data[(branch, role)]) != raw_bytes:
                errors.append('raw_native_byte_count_invalid')
        declared = {r.get('role'): r for r in frame.get('comparisons', [])}
        for role in ('color', 'depth'):
            if (0, role) not in data or (1, role) not in data:
                continue
            actual = raw_delta(data[0, role], data[1, role])
            expected = dict(actual, role=role)
            if declared.get(role) != expected:
                errors.append('fixture_raw_delta_declaration_differs')
            if actual['changed_bytes']:
                errors.append(f'frame{frame["frame"]}_{role}_strict_raw_pixels_differ')
        if (frame.get('valid_same_prefix_comparison') is not True or frame.get('passes') is not True):
            errors.append('fixture_local_verdict_failed')
    if report.get('passes') is not True:
        errors.append('fixture_series_verdict_failed')
    return {'errors': errors, 'files': evidence, 'passes': not errors}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, required=True)
    parser.add_argument('--metadata', type=Path, required=True)
    parser.add_argument('--install', type=Path, required=True)
    parser.add_argument('--pair', choices=PAIRS, required=True)
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--timeout-seconds', type=int, default=600)
    args = parser.parse_args()
    root, install = args.root.resolve(strict=True), args.install.resolve(strict=True)
    # Read the existing identity/environment/seal implementation unchanged.
    sys.path.insert(0, str(root / 'tools'))
    import run_surface_pixels as original
    from compare_surface_pixels import sha256
    identity, scene = original.frozen_identity(args.metadata), original.scene_identity(install)
    require_local_build(identity)
    out = args.out.resolve()
    if out.exists() or args.timeout_seconds <= 0:
        raise ValueError('positive timeout and a fresh evidence directory are required')
    out.mkdir(parents=True)
    report = {'schema': 'same-prefix-surface-frozen-run-v1', 'pair': args.pair,
              'started_utc': datetime.now(timezone.utc).isoformat(), 'frozen_identity': identity,
              'original_scene_identity': scene, 'errors': [], 'passes': False,
              'controller_sha256': sha256(Path(__file__)),
              'original_runner_sha256': sha256(root / 'tools/run_surface_pixels.py'),
              'acceptance_scope': 'local same-prefix surface only; whole-scene/WSI/CPU remain open'}
    env = original.environment(install, 'collect', out, identity['source_commit'])
    env['TG_SURFACE_LOCAL_PAIR'] = args.pair
    command = [identity['executable'], '--exact', TEST, '--nocapture', '--test-threads=1']
    report.update(command=command, environment_overrides={k:v for k,v in env.items()
                  if k.startswith(('TG_', 'RHAPSODY_')) or k == 'VK_INSTANCE_LAYERS'})
    original.dump(out / 'run.json', report)
    try:
        with (out / 'process.log').open('w', encoding='utf-8') as log:
            process = subprocess.run(command, cwd=root, env=env, stdout=log,
                                     stderr=subprocess.STDOUT, timeout=args.timeout_seconds, check=False)
        report['returncode'] = process.returncode
    except (subprocess.TimeoutExpired, OSError) as error:
        report['returncode'] = None
        report['errors'].append('test_launch_or_timeout:' + str(error))
    text = (out / 'process.log').read_text(encoding='utf-8', errors='replace')
    if (report['returncode'] != 0 or not re.search(r'\brunning 1 test\b', text) or TEST not in text or
            not re.search(r'test result: ok\. 1 passed; 0 failed; 0 ignored;', text) or
            re.search(r'VUID-|Validation Error|VALIDATION_ERROR|strict gameplay physical device|\bSKIP\b|\.\.\.\s+FAILED\b', text, re.I)):
        report['errors'].append('actual_gpu_test_failed_missing_skipped_or_validation_error')
    seal = original.seal_mode(out, identity, 'collect')
    report['errors'].extend(seal['errors'])
    # Add the new local files to the same independent SHA256 inventory.
    for season in ('water', 'ice'):
        try:
            manifest=json.loads((out/season/'manifest.json').read_text(encoding='utf-8'))
            if (manifest.get('schema')!='surface-same-prefix-fixture-v1' or
                    manifest.get('local_surface_comparison_enabled') is not True or
                    manifest.get('local_surface_pair')!=args.pair):
                report['errors'].append(season+':wrong_local_fixture_protocol')
            audit = audit_season(out / season, season, args.pair, sha256)
            report[season] = audit
            report['errors'].extend(season + ':' + e for e in audit['errors'])
            seal['files'].update({f'{season}/{k}':v for k,v in audit['files'].items()})
        except (OSError, ValueError, KeyError, TypeError) as error:
            report['errors'].append(season + ':invalid_local_evidence:' + str(error))
    seal['files']['process.log'] = sha256(out / 'process.log')
    # Even an early failed native run keeps an independently sealed inventory
    # of every fixed-path local artifact that was actually produced.
    for season in ('water','ice'):
        for number in (20,30):
            for branch in (0,1):
                for role in ('color','depth'):
                    relative=f'{season}/same-prefix/frame-{number}-{branch}-{role}.raw'
                    path=out/relative
                    if path.is_file():
                        seal['files'][relative]=sha256(path)
        relative=f'{season}/same-prefix/local-comparison.json'
        if (out/relative).is_file():
            seal['files'][relative]=sha256(out/relative)
    original.dump(out / 'artifacts.sha256.json', seal)
    sidecar = sha256(out / 'artifacts.sha256.json')
    report['artifact_sidecar_sha256'] = sidecar
    report['errors'].extend(original.verify_sealed_files(out, sidecar))
    if original.frozen_identity(args.metadata) != identity or original.scene_identity(install) != scene:
        report['errors'].append('frozen_binary_or_original_scene_changed_during_run')
    report['passes'] = not report['errors']
    original.dump(out / 'run.json', report)
    return 0 if report['passes'] else 1


if __name__ == '__main__':
    raise SystemExit(main())
