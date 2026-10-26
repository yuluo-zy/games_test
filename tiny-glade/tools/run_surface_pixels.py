"""Run one frozen gpu_surface_collect EXE in two processes; never build or run cargo."""
import argparse
from datetime import datetime, timezone
import json
import os
from pathlib import Path
import re
import subprocess

from compare_surface_pixels import SCENE, SCENE_SHA256, artifact_path, compare, digest_string, sha256

TEST = 'real_lake_water_and_ice_record_collect_and_preserve_raw_pixel_evidence'
ROOT = Path(__file__).resolve().parents[1]
INSTALL = Path(r'D:\game\ljxsj_92385\Tiny Glade')


def dump(path, value):
    Path(path).write_text(json.dumps(value, indent=2, ensure_ascii=False) + '\n', encoding='utf-8')


def frozen_identity(metadata):
    metadata = Path(metadata).resolve(strict=True)
    frozen = json.loads(metadata.read_text(encoding='utf-8'))
    if frozen.get('schema') not in ('surface-frozen-build-v1', 'headless-frozen-build-v1'):
        raise ValueError('unsupported frozen metadata schema')
    if (not isinstance(frozen.get('source_commit'), str) or
            not re.fullmatch('[0-9a-fA-F]{40}|[0-9a-fA-F]{64}', frozen['source_commit']) or
            not digest_string(frozen.get('source_content_sha256'))):
        raise ValueError('frozen source commit/content SHA256 is required')
    exe = Path(frozen.get('executable', ''))
    if not exe.is_absolute():
        exe = metadata.parent / exe
    exe = exe.resolve(strict=True)
    if exe.parent != metadata.parent or exe.suffix.lower() != '.exe':
        raise ValueError('EXE must be frozen beside its metadata')
    files = frozen.get('files')
    if not isinstance(files, dict) or not files:
        raise ValueError('frozen EXE/PDB/DLL SHA256 inventory is required')
    if (files.get(exe.name) != frozen.get('executable_sha256') or
            exe.with_suffix('.pdb').name not in files):
        raise ValueError('frozen inventory must contain this EXE and matching PDB')
    actual = {}
    for name, expected in files.items():
        if not digest_string(expected):
            raise ValueError('invalid frozen SHA256: ' + str(name))
        path = artifact_path(metadata.parent, name)
        value = sha256(path)
        if value.lower() != expected.lower():
            raise ValueError('frozen artifact changed: ' + name)
        actual[name] = value
    return {'metadata': str(metadata), 'metadata_sha256': sha256(metadata),
            'executable': str(exe), 'executable_sha256': actual[exe.name], 'files': actual,
            'source_commit': frozen['source_commit'], 'source_content_sha256': frozen['source_content_sha256'],
            'fixture_expected_binary_blake3': frozen.get('binary_blake3'),
            'source_identity_scope': 'Build metadata records source identity; this runner does not rebuild or rehash a live checkout.'}


def environment(install, mode, destination, source):
    # Same isolation rule as perf_headless.environment: discard inherited TG_
    # and RHAPSODY_ overrides, then set only this correctness invocation's knobs.
    result = {name: value for name, value in os.environ.items()
              if not name.startswith(('TG_', 'RHAPSODY_'))}
    result.update(TG_INSTALL_ROOT=str(install), RHAPSODY_REQUIRE_VULKAN='1',
                  VK_INSTANCE_LAYERS='VK_LAYER_KHRONOS_validation',
                  TG_FRAME_TIMING='1', TG_TEST_SIZE='1280x720', TG_SURFACE_MODE=mode,
                  TG_SURFACE_OUT=str(destination), TG_SURFACE_SOURCE=source,
                  CARGO_INCREMENTAL='0', RUST_LOG='warn')
    return result


def scene_identity(install):
    path = Path(install) / SCENE
    actual = sha256(path)
    if actual != SCENE_SHA256:
        raise ValueError('installed original lake scene SHA256 differs from the reference')
    return {'path': str(path.resolve()), 'sha256': actual}


def log_errors(text, returncode):
    errors = []
    if returncode != 0:
        errors.append('test_process_nonzero_exit:' + str(returncode))
    if not re.search(r'\brunning 1 test\b', text) or TEST not in text:
        errors.append('requested_single_test_did_not_run')
    if not re.search(r'test result: ok\. 1 passed; 0 failed; 0 ignored;', text):
        errors.append('one_pass_zero_ignored_summary_missing')
    if re.search(r'VUID-|Validation Error|VALIDATION_ERROR|strict gameplay physical device|\bSKIP\b|\.\.\.\s+FAILED\b|test result:\s*FAILED', text, re.I):
        errors.append('validation_initialization_skip_or_failure_in_log')
    return errors


def seal_mode(destination, identity, mode):
    """Hash evidence immediately after one child exits, including failed runs."""
    evidence = {'schema': 'surface-artifact-sha256-v1', 'mode': mode, 'files': {},
                'binary_identity': identity, 'fixture_declared_binary_blake3': {},
                'blake3_independently_verified': False, 'errors': []}
    for season in ('water', 'ice'):
        folder = destination / season
        manifest = folder / 'manifest.json'
        try:
            evidence['files'][f'{season}/manifest.json'] = sha256(manifest)
            row = json.loads(manifest.read_text(encoding='utf-8'))
            if not isinstance(row, dict):
                raise ValueError('manifest must be an object')
            if (row.get('source') != identity['source_commit'] or row.get('season') != season or
                    row.get('surface_execution_mode') != mode):
                evidence['errors'].append(season + '_manifest_source_season_or_mode_changed')
            declared = row.get('binary_blake3')
            evidence['fixture_declared_binary_blake3'][season] = declared
            if not digest_string(declared):
                evidence['errors'].append(season + '_binary_declaration_missing')
            expected = identity.get('fixture_expected_binary_blake3')
            if expected is not None and declared != expected:
                evidence['errors'].append(season + '_binary_declaration_differs_from_frozen_metadata')
        except (OSError, ValueError, UnicodeError) as error:
            evidence['errors'].append(season + '_manifest_unreadable:' + str(error))
        # Fixed expected paths never trust the manifest to select evidence.
        for frame in (20, 30):
            for suffix in ('rgba', 'png'):
                name = f'{season}/frame-{frame:02}.{suffix}'
                try:
                    evidence['files'][name] = sha256(destination / name)
                except OSError as error:
                    evidence['errors'].append(name + '_missing:' + str(error))
    values = list(evidence['fixture_declared_binary_blake3'].values())
    if len(values) != 2 or values[0] != values[1]:
        evidence['errors'].append('water_ice_binary_declarations_differ_or_missing')
    dump(destination / 'artifacts.sha256.json', evidence)
    return evidence


def verify_sealed_files(folder, expected_sidecar_sha256):
    errors = []
    try:
        sidecar = folder / 'artifacts.sha256.json'
        if sha256(sidecar) != expected_sidecar_sha256:
            return ['artifact_sidecar_changed']
        evidence = json.loads(sidecar.read_text(encoding='utf-8'))
        for relative, expected in evidence['files'].items():
            path = (folder / relative).resolve()
            if not path.is_relative_to(folder.resolve()) or sha256(path) != expected:
                errors.append('sealed_artifact_changed:' + relative)
    except (OSError, ValueError, KeyError) as error:
        errors.append('sealed_artifact_unreadable:' + str(error))
    return errors


def run_pair(metadata, destination, install=INSTALL, root=ROOT, timeout=600):
    identity = frozen_identity(metadata)
    destination = Path(destination).resolve()
    if destination.exists():
        raise ValueError('use a fresh output directory; never mix prior capture evidence')
    install = Path(install).resolve(strict=True)
    root = Path(root).resolve(strict=True)
    scene = scene_identity(install)
    destination.mkdir(parents=True)
    result = {'schema': 'surface-frozen-pair-v1', 'started_utc': datetime.now(timezone.utc).isoformat(),
              'frozen_identity': identity, 'runs': [], 'comparisons': {}, 'errors': [], 'passes': False,
              'original_scene_identity': scene,
              'controller_sha256': sha256(Path(__file__)),
              'comparator_sha256': sha256(Path(__file__).with_name('compare_surface_pixels.py')),
              'acceptance_scope': 'Exact same-binary water/ice headless pixels. Visible ROI review, WSI and CPU acceptance remain separate.',
              'binary_binding': 'Controller launches this exact EXE and verifies frozen SHA256 inventory before/after each process. Manifest BLAKE3 is a fixture declaration, not an independently computed Python digest.'}
    dump(destination / 'pair.json', result)
    for mode in ('inline', 'collect'):
        folder = destination / mode
        folder.mkdir()
        pre = frozen_identity(metadata)
        if pre != identity:
            raise ValueError('frozen metadata/artifacts changed before process launch')
        if scene_identity(install) != scene:
            raise ValueError('installed original scene identity changed before process launch')
        command = [identity['executable'], '--exact', TEST, '--nocapture', '--test-threads=1']
        env = environment(install, mode, folder, identity['source_commit'])
        run = {'mode': mode, 'command': command, 'environment_overrides':
               {name: env[name] for name in ('TG_INSTALL_ROOT', 'RHAPSODY_REQUIRE_VULKAN',
                                            'VK_INSTANCE_LAYERS', 'TG_FRAME_TIMING', 'TG_SURFACE_MODE',
                                            'TG_SURFACE_SOURCE', 'TG_SURFACE_OUT')}}
        try:
            with (folder / 'process.log').open('w', encoding='utf-8') as output:
                process = subprocess.run(command, cwd=root, env=env, stdout=output,
                                         stderr=subprocess.STDOUT, timeout=timeout, check=False)
            run['returncode'] = process.returncode
        except subprocess.TimeoutExpired:
            run['returncode'] = None
            run['timeout'] = True
        except OSError as error:
            run['returncode'] = None
            with (folder / 'process.log').open('a', encoding='utf-8') as output:
                output.write('\ncontroller launch failed: ' + str(error) + '\n')
        # Subprocess.run kills/reaps a timed-out child before returning here.
        sealed = seal_mode(folder, identity, mode)
        run['artifact_sidecar_sha256'] = sha256(folder / 'artifacts.sha256.json')
        run['log_sha256'] = sha256(folder / 'process.log')
        run['errors'] = log_errors((folder / 'process.log').read_text(encoding='utf-8', errors='replace'),
                                  run['returncode']) + sealed['errors']
        try:
            if frozen_identity(metadata) != identity:
                run['errors'].append('frozen_identity_changed_after_process')
        except (OSError, ValueError) as error:
            run['errors'].append('frozen_identity_invalid_after_process:' + str(error))
        try:
            if scene_identity(install) != scene:
                run['errors'].append('original_scene_identity_changed_after_process')
        except (OSError, ValueError) as error:
            run['errors'].append('original_scene_invalid_after_process:' + str(error))
        result['runs'].append(run)
        if run['errors']:
            result['errors'].append(mode + '_run_invalid')
        dump(destination / 'pair.json', result)
        if run['errors']:
            return result
    for season in ('water', 'ice'):
        compared = compare(destination / 'inline' / season, destination / 'collect' / season)
        dump(destination / (season + '-compare.json'), compared)
        result['comparisons'][season] = {'passes': compared['passes'],
                                         'report_sha256': sha256(destination / (season + '-compare.json'))}
        if not compared['passes']:
            result['errors'].append(season + '_pixel_or_input_comparison_failed')
    for run in result['runs']:
        result['errors'].extend(run['mode'] + ':' + error for error in
                                verify_sealed_files(destination / run['mode'], run['artifact_sidecar_sha256']))
    declarations = []
    for mode in ('inline', 'collect'):
        evidence = json.loads((destination / mode / 'artifacts.sha256.json').read_text(encoding='utf-8'))
        declarations.extend(evidence['fixture_declared_binary_blake3'].values())
    if len(set(declarations)) != 1:
        result['errors'].append('processes_did_not_report_the_same_binary_declaration')
    result['passes'] = not result['errors']
    dump(destination / 'pair.json', result)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--metadata', required=True, type=Path)
    parser.add_argument('--out', required=True, type=Path)
    parser.add_argument('--install', type=Path, default=Path(os.environ.get('TG_INSTALL_ROOT', INSTALL)))
    parser.add_argument('--root', type=Path, default=ROOT)
    parser.add_argument('--timeout-seconds', type=int, default=600)
    args = parser.parse_args()
    if args.timeout_seconds <= 0:
        parser.error('timeout must be positive')
    result = run_pair(args.metadata, args.out, args.install, args.root, args.timeout_seconds)
    print(json.dumps(result, ensure_ascii=False, indent=2))
    return 0 if result['passes'] else 1


if __name__ == '__main__':
    raise SystemExit(main())
