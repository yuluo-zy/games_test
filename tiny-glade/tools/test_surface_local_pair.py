import copy
import json
from pathlib import Path
import tempfile
import unittest

from run_surface_local_pair import raw_delta, semantic_errors, audit_season, physical_extent_from_manifest


def valid_manifest():
    words=[0]*1060
    words[36:38]=[1280,736]
    source={'nominal_extent':[1280,720],'effective_output_extent':[1280,720],
            'physical_inner_extent':[1280,736],'alignment':32,
            'world_scale_bits':0x3f800000,'tgsr_scale_bits':0x3f800000}
    return {'world_scale':1.0,'fixed_step':[1,60],
            'pixels':[{'frame':n,'width':1280,'height':720} for n in (20,30)],
            'commands':[{'frame':i,'input':{'frame_constant_words':list(words),
                         'inner_resolution':[1280,736],'resolution_source':copy.deepcopy(source),
                         'temporal_source_resources':{'upscaling_mode':'Off'}}} for i in range(30)]}


def valid_frame(ice=False):
    draw = {'kind': {'kind': 3, 'flag': 1}, 'native_stencil_test': False,
            'bindings': [{'binding': 0, 'value': {'kind': 'dynamic_uniform', 'size': 4}}],
            'dynamic_constants': [[0, [1, 2, 3, 4]]]}
    name = 'Ice plane' if ice else 'Water plane'
    p = {'pass': name, 'observation_valid': True, 'draws': [draw]}
    execution = [{'requested_mode': 'inline', 'valid': True, 'inline_calls': 1,
                  'inline_draws': 1, 'worker_calls': 0, 'worker_tasks': 0,
                  'worker_draws': 0, 'owner_collect_tasks': 0},
                 {'requested_mode': 'collect', 'valid': True, 'inline_calls': 0,
                  'inline_draws': 0, 'worker_calls': 1, 'worker_tasks': 1,
                  'worker_draws': 1, 'owner_collect_tasks': 0}]
    return {'frame': 20, 'modes': ['inline', 'collect'],
            'ordered_raster_declarations': [[copy.deepcopy(p)], [copy.deepcopy(p)]],
            'actual_retained_pass_order': [name, name], 'actual_surface_execution': execution,
            'undefined_history_read_possible': False,
            'actual_temporal_inputs': [{'name': name, 'a_previous_rg': i, 'b_previous_rg': i,
                                       'a_current_rg': 10+i, 'b_current_rg': 20+i,
                                       'a_history_valid': True, 'b_history_valid': True}
                                      for i, name in enumerate(('Ice shadows', 'water refr temporal',
                                                                 'ice refl ray len'))] if ice else []}


class SemanticTests(unittest.TestCase):
    def test_same_real_inputs_and_modes_accepted(self):
        self.assertEqual(semantic_errors(valid_frame(), 'water', 'inline-collect'), [])
        self.assertEqual(semantic_errors(valid_frame(True), 'ice', 'inline-collect'), [])

    def test_equal_unsupported_draws_are_invalid(self):
        frame = valid_frame()
        for branch in frame['ordered_raster_declarations']:
            branch[0]['draws'][0]['kind'] = {'unsupported_draw_kind': 'future'}
        self.assertTrue(semantic_errors(frame, 'water', 'inline-collect'))

    def test_real_constant_difference_rejected(self):
        frame = valid_frame()
        frame['ordered_raster_declarations'][1][0]['draws'][0]['dynamic_constants'][0][1][0] ^= 1
        self.assertIn('ordered_raster_declarations_differ', semantic_errors(frame, 'water', 'inline-collect'))

    def test_missing_constant_bytes_both_modes_rejected(self):
        frame = valid_frame()
        for branch in frame['ordered_raster_declarations']:
            branch[0]['draws'][0]['dynamic_constants'] = []
        self.assertTrue(semantic_errors(frame, 'water', 'inline-collect'))

    def test_owner_fallback_and_zero_tasks_rejected_despite_claimed_valid(self):
        for key, value in (('worker_tasks', 0), ('owner_collect_tasks', 1), ('worker_draws', 0)):
            frame = valid_frame()
            frame['actual_surface_execution'][1][key] = value
            self.assertTrue(semantic_errors(frame, 'water', 'inline-collect'))

    def test_stencil_enabled_invalid_even_if_identical(self):
        frame = valid_frame()
        for branch in frame['ordered_raster_declarations']:
            branch[0]['draws'][0]['native_stencil_test'] = True
        self.assertTrue(semantic_errors(frame, 'water', 'inline-collect'))

    def test_missing_retained_surface_invalid(self):
        frame = valid_frame()
        frame['actual_retained_pass_order'].pop()
        self.assertTrue(semantic_errors(frame, 'water', 'inline-collect'))

    def test_all_three_real_histories_required(self):
        frame = valid_frame(True)
        frame['actual_temporal_inputs'].pop()
        self.assertTrue(semantic_errors(frame, 'ice', 'inline-collect'))

    def test_history_previous_flags_and_current_isolation(self):
        for key, value in (('b_previous_rg', 99), ('b_history_valid', False), ('b_current_rg', 10)):
            frame = valid_frame(True)
            frame['actual_temporal_inputs'][0][key] = value
            self.assertTrue(semantic_errors(frame, 'ice', 'inline-collect'))

    def test_unresolved_original_undefined_history_invalid(self):
        frame = valid_frame(True)
        frame['undefined_history_read_possible'] = True
        self.assertTrue(semantic_errors(frame, 'ice', 'inline-collect'))

    def test_raw_one_bit_is_zero_tolerance_failure(self):
        self.assertEqual(raw_delta(b'\x00'*8, b'\x00'*8)['changed_pixels'], 0)
        self.assertEqual(raw_delta(b'\x00'*8, b'\x00'*7+b'\x01'),
                         {'changed_bytes': 1, 'changed_pixels': 1, 'tolerance': 0})
        with self.assertRaises(ValueError):
            raw_delta(b'123', b'123')


class PhysicalExtentTests(unittest.TestCase):
    def test_original_alignment_keeps_all_native736_rows(self):
        self.assertEqual(physical_extent_from_manifest(valid_manifest(),20),[1280,736])

    def test_consistent_shrink_cannot_change_fixed_nominal_workload(self):
        manifest=valid_manifest()
        row=manifest['commands'][19]['input']
        row['inner_resolution']=[1,1]
        row['frame_constant_words'][36:38]=[1,1]
        row['resolution_source']['physical_inner_extent']=[1,1]
        with self.assertRaises(ValueError):
            physical_extent_from_manifest(manifest,20)

    def test_source_config_cannot_crop_to_nominal720(self):
        manifest=valid_manifest()
        row=manifest['commands'][19]['input']
        row['inner_resolution']=[1280,720]
        row['frame_constant_words'][36:38]=[1280,720]
        row['resolution_source']['physical_inner_extent']=[1280,720]
        with self.assertRaises(ValueError):
            physical_extent_from_manifest(manifest,20)

    def test_actual_native_constant_words_must_match_config(self):
        manifest=valid_manifest()
        manifest['commands'][19]['input']['frame_constant_words'][37]=720
        with self.assertRaises(ValueError):
            physical_extent_from_manifest(manifest,20)

    def test_resolution_source_scale_and_upscaling_are_bound(self):
        for field,value in (('world_scale_bits',0x3f000000),('tgsr_scale_bits',0x3f000000),('alignment',16)):
            manifest=valid_manifest()
            manifest['commands'][19]['input']['resolution_source'][field]=value
            with self.assertRaises(ValueError):
                physical_extent_from_manifest(manifest,20)

    def test_nominal_window_and_fixed_scale_cannot_be_relabelled(self):
        manifest=valid_manifest()
        manifest['pixels'][0]['height']=736
        with self.assertRaises(ValueError):
            physical_extent_from_manifest(manifest,20)
        manifest=valid_manifest()
        manifest['world_scale']=0.5
        with self.assertRaises(ValueError):
            physical_extent_from_manifest(manifest,20)


class ArtifactTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.folder = Path(self.temp.name)
        sub = self.folder / 'same-prefix'
        sub.mkdir()
        (self.folder/'manifest.json').write_text(json.dumps(valid_manifest()),encoding='utf-8')
        self.report = {'schema': 'same-prefix-surface-correctness-v1', 'passes': True,
                       'pixel_tolerance': 0, 'no_additional_read_wait_ns': 0, 'frames': []}
        for index in (20, 30):
            frame = valid_frame()
            frame.update(frame=index, valid_same_prefix_comparison=True, passes=True, raw=[], comparisons=[])
            for branch in (0, 1):
                for role, format_, aspect in (('color', 122, 1), ('depth', 130, 2)):
                    name = f'frame-{index}-{branch}-{role}.raw'
                    (sub / name).write_bytes(bytes(1280*736*4))
                    frame['raw'].append({'branch': branch, 'role': role, 'file': name,
                                         'extent': [1280, 736], 'bytes': 1280*736*4,
                                         'format': format_, 'aspect': aspect,
                                         'receipt_owner_confirmed': True})
            for role in ('color', 'depth'):
                frame['comparisons'].append({'role': role, 'changed_bytes': 0, 'changed_pixels': 0, 'tolerance': 0})
            self.report['frames'].append(frame)

    def tearDown(self):
        self.temp.cleanup()

    def audit(self):
        path = self.folder / 'same-prefix/local-comparison.json'
        path.write_text(json.dumps(self.report), encoding='utf-8')
        import hashlib
        return audit_season(self.folder, 'water', 'inline-collect',
                            lambda p: hashlib.sha256(Path(p).read_bytes()).hexdigest())

    def test_actual_equal_raw_files_accepted(self):
        self.assertTrue(self.audit()['passes'])

    def test_raw_failure_cannot_be_hidden_by_claimed_pass(self):
        path = self.folder / 'same-prefix/frame-20-1-color.raw'
        with path.open('r+b') as stream:
            stream.write(b'\x01')
        report = self.audit()
        self.assertFalse(report['passes'])
        self.assertIn('frame20_color_strict_raw_pixels_differ', report['errors'])

    def test_native_format_or_aspect_substitution_rejected(self):
        self.report['frames'][0]['raw'][0]['format'] = 97
        self.assertFalse(self.audit()['passes'])

    def test_manifest_cannot_select_another_path(self):
        self.report['frames'][0]['raw'][0]['file'] = '../other.raw'
        self.assertFalse(self.audit()['passes'])

    def test_physical_padded_bottom_rows_are_not_cropped(self):
        path=self.folder/'same-prefix/frame-20-1-color.raw'
        with path.open('r+b') as stream:
            stream.seek((735*1280+1279)*4+3)
            stream.write(b'\x01')
        self.assertIn('frame20_color_strict_raw_pixels_differ',self.audit()['errors'])

    def test_readback_descriptor_and_size_must_match_native736(self):
        self.report['frames'][0]['raw'][0].update(extent=[1280,720],bytes=1280*720*4)
        self.assertFalse(self.audit()['passes'])


if __name__ == '__main__':
    unittest.main()
