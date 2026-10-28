"""Acceptance-gate regressions; synthetic files only, no GPU or compiler."""
import copy
import json
import os
from pathlib import Path
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

import compare_surface_pixels as pixels
import run_surface_pixels as runner


def manifest(mode='collect', season='water'):
    lengths = [0] * 21
    lengths[15] = 1
    scene = {'schema': 2, 'draw_lists': lengths, 'meshes': [], 'nani': [], 'session': {'glade': season}}
    commands = [{'frame': index, 'pass_names': ['Ice plane' if season == 'ice' else 'Water plane'],
                 'input': {'frame_counter': index, 'game_time_sec': 1, 'game_time_frac_bits': index,
                           'delta_time_bits': 1015580809, 'inner_resolution': [1280, 736],
                           'render_overrides': 0, 'time_of_day_bits': 1, 'camera_bits': [0] * 16,
                           'water_raster_blake3': '2' * 64, 'scene': copy.deepcopy(scene)}}
                for index in range(30)]
    return {'schema': pixels.SCHEMA, 'season': season, 'scene': pixels.SCENE,
            'expected_original_scene_sha256': pixels.SCENE_SHA256,
            'scene_blake3': '1' * 64, 'water_raster_blake3': '2' * 64, 'wet_raster_pixels': 99,
            'initial_draw_list_lengths': lengths, 'frames': 30, 'fixed_step': [1, 60], 'world_scale': 1.0,
            'initial_scene_input': scene, 'commands': commands,
            'source': 'a' * 40, 'binary_blake3': '3' * 64, 'surface_execution_mode': mode,
            'actual_surface_execution': {'inline_calls': 30 if mode == 'inline' else 0,
                                         'inline_draws': 30 if mode == 'inline' else 0,
                                         'worker_calls': 30 if mode == 'collect' else 0,
                                         'worker_tasks': 30 if mode == 'collect' else 0,
                                         'worker_draws': 30 if mode == 'collect' else 0, 'owner_collect_tasks': 0},
            'pixels': [{'frame': frame, 'width': 1280, 'height': 720,
                        'file': f'frame-{frame}.rgba', 'blake3': '4' * 64} for frame in (20, 30)]}


def write_fixture(folder, row):
    folder.mkdir(parents=True, exist_ok=True)
    (folder / 'manifest.json').write_text(json.dumps(row), encoding='utf-8')
    for frame in (20, 30):
        (folder / f'frame-{frame}.rgba').write_bytes(bytes([1, 2, 3, 255]) * (1280 * 720))
        # Runner only hashes PNG evidence; image decoding/ROI review is separate.
        (folder / f'frame-{frame}.png').write_bytes(b'synthetic PNG evidence for hash test')


class PixelGateTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.root = Path(self.temp.name)
        self.a, self.b = self.root / 'a', self.root / 'b'
        self.left, self.right = manifest('inline'), manifest('collect')
        write_fixture(self.a, self.left)
        write_fixture(self.b, self.right)

    def tearDown(self):
        self.temp.cleanup()

    def save(self):
        for folder, row in ((self.a, self.left), (self.b, self.right)):
            (folder / 'manifest.json').write_text(json.dumps(row), encoding='utf-8')

    def test_same_binary_modes_pass_and_report_actual_raw_sha256(self):
        result = pixels.compare(self.a, self.b)
        self.assertTrue(result['passes'], result['input_mismatches'])
        self.assertEqual(result['comparison_kind'], 'same_binary_inline_vs_collect')
        self.assertEqual(result['frames'][0]['a']['raw_file_sha256'], pixels.sha256(self.a / 'frame-20.rgba'))
        self.assertIn('fixture declarations', result['hash_policy'])

    def test_missing_key_in_both_manifests_is_rejected(self):
        del self.left['initial_scene_input']
        del self.right['initial_scene_input']
        self.save()
        self.assertFalse(pixels.compare(self.a, self.b)['passes'])

    def test_inner_extent_requires_renderer_32_pixel_padding_not_output_dimensions(self):
        self.assertEqual(pixels.EXPECTED_INNER_EXTENT, [1280, 736])
        self.assertTrue(pixels.compare(self.a, self.b)['passes'])
        for extent in ([1280, 720], [1280, 704], [1280, 768], [1248, 736]):
            with self.subTest(extent=extent):
                self.left, self.right = manifest('inline'), manifest('collect')
                self.left['commands'][4]['input']['inner_resolution'] = extent
                self.right['commands'][4]['input']['inner_resolution'] = extent
                self.save()
                self.assertFalse(pixels.compare(self.a, self.b)['passes'])

    def test_two_empty_manifests_and_malformed_json_never_pass(self):
        for data in ('{}', '[]', '{bad json'):
            with self.subTest(data=data):
                (self.a / 'manifest.json').write_text(data)
                (self.b / 'manifest.json').write_text(data)
                self.assertFalse(pixels.compare(self.a, self.b)['passes'])

    def test_matching_missing_nested_scene_inputs_are_rejected(self):
        for field in ('meshes', 'nani', 'session'):
            with self.subTest(field=field):
                self.left, self.right = manifest('inline'), manifest('collect')
                del self.left['initial_scene_input'][field]
                del self.right['initial_scene_input'][field]
                self.save()
                self.assertFalse(pixels.compare(self.a, self.b)['passes'])
                self.left, self.right = manifest('inline'), manifest('collect')
                del self.left['commands'][7]['input']['scene'][field]
                del self.right['commands'][7]['input']['scene'][field]
                self.save()
                self.assertFalse(pixels.compare(self.a, self.b)['passes'])

    def test_duplicate_and_missing_capture_are_rejected(self):
        for shots in ([self.right['pixels'][0]] * 2, [self.right['pixels'][0]], []):
            with self.subTest(shots=len(shots)):
                self.right['pixels'] = shots
                self.save()
                self.assertFalse(pixels.compare(self.a, self.b)['passes'])

    def test_dimensions_and_truncated_bytes_are_rejected(self):
        self.right['pixels'][0]['width'] = 1279
        self.save()
        self.assertFalse(pixels.compare(self.a, self.b)['passes'])
        self.right['pixels'][0]['width'] = 1280
        self.save()
        (self.b / 'frame-20.rgba').write_bytes(b'\0')
        self.assertFalse(pixels.compare(self.a, self.b)['passes'])

    def test_capture_cannot_redirect_or_reuse_a_different_frames_file(self):
        with (self.b / 'frame-20.rgba').open('r+b') as stream:
            stream.write(b'X')
        self.left['pixels'][0]['file'] = 'frame-30.rgba'
        self.right['pixels'][0]['file'] = 'frame-30.rgba'
        self.save()
        result = pixels.compare(self.a, self.b)
        self.assertFalse(result['passes'])
        self.assertTrue(any('frame_file_binding' in error for error in result['capture_errors']))
        self.left['pixels'][0]['file'] = 'other.rgba'
        self.right['pixels'][0]['file'] = 'other.rgba'
        self.save()
        self.assertFalse(pixels.compare(self.a, self.b)['passes'])

    def test_no_real_worker_owner_fallback_or_reduced_draws_are_rejected(self):
        for field, value in (('worker_tasks', 0), ('owner_collect_tasks', 1), ('worker_draws', 29)):
            with self.subTest(field=field):
                self.right = manifest('collect')
                self.right['actual_surface_execution'][field] = value
                self.save()
                self.assertFalse(pixels.compare(self.a, self.b)['passes'])

    def test_real_pixel_change_rejects_even_when_declared_blake3_is_unchanged(self):
        path = self.b / 'frame-30.rgba'
        with path.open('r+b') as stream:
            stream.seek(4 * 123 + 1)
            stream.write(bytes([9]))
        result = pixels.compare(self.a, self.b)
        self.assertTrue(result['valid_same_input_comparison'])
        self.assertFalse(result['passes'])
        changed = next(row for row in result['frames'] if row['frame'] == 30)
        self.assertEqual(changed['changed_pixels'], 1)
        self.assertEqual(changed['max_channel_abs'], 7)
        self.assertNotEqual(changed['a']['raw_file_sha256'], changed['b']['raw_file_sha256'])

    def test_changed_clock_or_cross_mode_binary_identity_is_rejected(self):
        self.right['commands'][3]['input']['game_time_frac_bits'] += 1
        self.save()
        self.assertFalse(pixels.compare(self.a, self.b)['passes'])
        self.right = manifest('collect')
        self.right['binary_blake3'] = '7' * 64
        self.save()
        self.assertFalse(pixels.compare(self.a, self.b)['passes'])

    def test_different_output_pixels_and_digest_keep_same_input_but_reject(self):
        self.right['pixels'][1]['blake3'] = '8' * 64
        self.save()
        with (self.b / 'frame-30.rgba').open('r+b') as stream:
            stream.seek(4 * 123 + 1)
            stream.write(bytes([9]))
        result = pixels.compare(self.a, self.b)
        self.assertTrue(result['valid_same_input_comparison'])
        self.assertEqual(result['input_mismatches'], [])
        self.assertEqual(result['capture_errors'], [])
        self.assertFalse(result['exact_pixels_equal'])
        self.assertFalse(result['passes'])
        self.assertIn('frame_30_fixture_digest_declarations_differ', result['output_mismatches'])
        self.assertIn('frame_30_raw_pixels_differ', result['output_mismatches'])

    def test_actual_input_change_is_invalid_even_with_equal_output(self):
        self.right['commands'][8]['input']['frame_counter'] += 1
        self.save()
        result = pixels.compare(self.a, self.b)
        self.assertFalse(result['valid_same_input_comparison'])
        self.assertIn('commands', result['input_mismatches'])
        self.assertTrue(result['exact_pixels_equal'])
        self.assertFalse(result['passes'])

    def test_output_digest_declaration_alone_is_output_error_and_still_rejects(self):
        self.right['pixels'][0]['blake3'] = '9' * 64
        self.save()
        result = pixels.compare(self.a, self.b)
        self.assertTrue(result['valid_same_input_comparison'])
        self.assertTrue(result['exact_pixels_equal'])
        self.assertFalse(result['passes'])
        self.assertEqual(result['output_mismatches'], ['frame_20_fixture_digest_declarations_differ'])

    def test_capture_damage_does_not_mislabel_semantic_inputs(self):
        (self.b / 'frame-20.rgba').write_bytes(b'truncated')
        result = pixels.compare(self.a, self.b)
        self.assertTrue(result['valid_same_input_comparison'])
        self.assertTrue(result['capture_errors'])
        self.assertFalse(result['comparison_evidence_valid'])
        self.assertFalse(result['passes'])

    def test_binary_identity_failure_is_separate_and_still_rejects(self):
        self.right['binary_blake3'] = '7' * 64
        self.save()
        result = pixels.compare(self.a, self.b)
        self.assertTrue(result['valid_same_input_comparison'])
        self.assertIn('inline_collect_requires_same_source_and_binary', result['comparison_errors'])
        self.assertFalse(result['comparison_evidence_valid'])
        self.assertFalse(result['passes'])


class ControllerGateTests(unittest.TestCase):
    def test_installed_scene_sha256_is_checked_against_original_reference(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            path = root / pixels.SCENE
            path.parent.mkdir(parents=True)
            path.write_bytes(b'synthetic original scene')
            expected = pixels.sha256(path)
            with patch.object(runner, 'SCENE_SHA256', expected):
                self.assertEqual(runner.scene_identity(root)['sha256'], expected)
                path.write_bytes(b'changed same-install scene')
                with self.assertRaisesRegex(ValueError, 'original lake scene SHA256'):
                    runner.scene_identity(root)

    def test_clean_environment_and_strict_test_log_gate(self):
        with patch.dict(os.environ, {'TG_DRAW_COLLECT_CHUNK': '1', 'RHAPSODY_FAULT': 'yes',
                                     'TG_SURFACE_SOURCE': 'bad', 'PATH': 'retained'}):
            env = runner.environment(Path('install'), 'collect', Path('out'), 'a' * 40)
        self.assertNotIn('TG_DRAW_COLLECT_CHUNK', env)
        self.assertNotIn('RHAPSODY_FAULT', env)
        self.assertEqual(env['VK_INSTANCE_LAYERS'], 'VK_LAYER_KHRONOS_validation')
        self.assertEqual(env['PATH'], 'retained')
        good = f'running 1 test\ntest {runner.TEST} ... ok\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;'
        self.assertEqual(runner.log_errors(good, 0), [])
        for text, code in ((good.replace('1 passed', '0 passed'), 0),
                           (good.replace('0 ignored', '1 ignored'), 0), (good + '\nVUID-bad', 0),
                           (good + '\nSKIP init failure', 0), (good, 1)):
            with self.subTest(text=text, code=code):
                self.assertTrue(runner.log_errors(text, code))

    def test_frozen_inventory_refuses_changed_pdb(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            exe, pdb = root / 'gpu_surface_collect.exe', root / 'gpu_surface_collect.pdb'
            exe.write_bytes(b'frozen executable')
            pdb.write_bytes(b'frozen symbols')
            row = {'schema': 'surface-frozen-build-v1', 'executable': exe.name,
                   'executable_sha256': pixels.sha256(exe), 'source_commit': 'a' * 40,
                   'source_content_sha256': 'b' * 64,
                   'files': {exe.name: pixels.sha256(exe), pdb.name: pixels.sha256(pdb)}}
            metadata = root / 'build.json'
            metadata.write_text(json.dumps(row))
            self.assertEqual(runner.frozen_identity(metadata)['executable_sha256'], pixels.sha256(exe))
            pdb.write_bytes(b'changed symbols')
            with self.assertRaisesRegex(ValueError, 'frozen artifact changed'):
                runner.frozen_identity(metadata)

    def test_pair_uses_two_processes_and_seals_first_before_launching_second(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            exe, pdb = root / 'gpu_surface_collect.exe', root / 'gpu_surface_collect.pdb'
            exe.write_bytes(b'frozen executable')
            pdb.write_bytes(b'frozen symbols')
            metadata = root / 'build.json'
            metadata.write_text(json.dumps({'schema': 'surface-frozen-build-v1', 'executable': exe.name,
                'executable_sha256': pixels.sha256(exe), 'source_commit': 'a' * 40,
                'source_content_sha256': 'b' * 64,
                'files': {exe.name: pixels.sha256(exe), pdb.name: pixels.sha256(pdb)}}))
            output = root / 'result'
            modes = []

            def child(command, **kwargs):
                self.assertEqual(command[0], str(exe.resolve()))
                self.assertNotIn('cargo', command)
                mode = kwargs['env']['TG_SURFACE_MODE']
                if modes:
                    self.assertTrue((output / modes[0] / 'artifacts.sha256.json').is_file())
                modes.append(mode)
                for season in ('water', 'ice'):
                    write_fixture(Path(kwargs['env']['TG_SURFACE_OUT']) / season, manifest(mode, season))
                kwargs['stdout'].write(f'running 1 test\ntest {runner.TEST} ... ok\n'
                                       'test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;\n')
                return SimpleNamespace(returncode=0)

            with patch.object(runner.subprocess, 'run', side_effect=child) as process, \
                    patch.object(runner, 'scene_identity', return_value={
                        'path': str(root / pixels.SCENE), 'sha256': pixels.SCENE_SHA256}) as scene_gate:
                result = runner.run_pair(metadata, output, root, root)
            self.assertEqual(process.call_count, 2)
            self.assertEqual(scene_gate.call_count, 5)
            self.assertEqual(modes, ['inline', 'collect'])
            self.assertTrue(result['passes'], result['errors'])
            sidecar = output / 'inline' / 'artifacts.sha256.json'
            expected = pixels.sha256(sidecar)
            raw = output / 'inline' / 'water' / 'frame-20.rgba'
            with raw.open('r+b') as stream:
                stream.write(b'X')
            self.assertTrue(runner.verify_sealed_files(output / 'inline', expected))


if __name__ == '__main__':
    unittest.main()
