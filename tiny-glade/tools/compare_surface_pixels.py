"""Strict original-lake RGBA comparison (stdlib only; no timing claim)."""
import argparse
import hashlib
import json
from pathlib import Path
import re

SCHEMA = 'surface-collect-correctness-v1'
SCENE = 'assets/starting-builds/full/05-rocky-lake/history.json'
SCENE_SHA256 = 'f85cba25b4a193d517402a57df3590323f3d388b4f8181899c4b7fed7272fec3'
OUTPUT_EXTENT = (1280, 720)
# frame_resolution::dimensions at world/TGSR scale 1: add 31, clear five
# low bits, then clamp to [32,10912]. This is padding, not output resizing.
EXPECTED_INNER_EXTENT = [max(32, min(10912, (axis + 31) & ~31)) for axis in OUTPUT_EXTENT]
INPUTS = ('schema', 'season', 'scene', 'expected_original_scene_sha256',
          'scene_blake3', 'water_raster_blake3', 'wet_raster_pixels',
          'initial_draw_list_lengths', 'frames', 'fixed_step', 'world_scale',
          'initial_scene_input', 'commands')


def sha256(path):
    with Path(path).open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def digest_string(value):
    return isinstance(value, str) and re.fullmatch('[0-9a-fA-F]{64}', value) is not None


def artifact_path(directory, name):
    if not isinstance(name, str) or Path(name).name != name or name in ('', '.', '..'):
        raise ValueError('artifact must name one file in the season directory')
    root = Path(directory).resolve()
    path = (root / name).resolve()
    if not path.is_relative_to(root):
        raise ValueError('artifact escapes its season directory')
    return path


def scene_input_valid(value):
    if not isinstance(value, dict) or value.get('schema') != 2:
        return False
    lengths = value.get('draw_lists')
    if (not isinstance(lengths, list) or len(lengths) != 21 or
            any(type(n) is not int or n < 0 for n in lengths) or lengths[15] <= 0):
        return False
    return (isinstance(value.get('session'), dict) and bool(value['session']) and
            isinstance(value.get('meshes'), list) and isinstance(value.get('nani'), list))


def manifest_errors(row, side):
    errors = []
    missing = [key for key in (*INPUTS, 'source', 'binary_blake3',
                               'surface_execution_mode', 'actual_surface_execution', 'pixels')
               if key not in row or row[key] is None]
    if missing:
        errors.append(side + '_missing_inputs:' + ','.join(missing))
    if (row.get('schema') != SCHEMA or row.get('season') not in ('water', 'ice') or
            row.get('scene') != SCENE or row.get('expected_original_scene_sha256') != SCENE_SHA256 or
            row.get('frames') != 30 or row.get('fixed_step') != [1, 60] or
            row.get('world_scale') != 1.0 or type(row.get('wet_raster_pixels')) is not int or
            row.get('wet_raster_pixels', 0) <= 0 or
            not digest_string(row.get('scene_blake3')) or
            not digest_string(row.get('water_raster_blake3')) or
            not digest_string(row.get('binary_blake3')) or
            not isinstance(row.get('source'), str) or row.get('source') in ('', 'unrecorded') or
            not scene_input_valid(row.get('initial_scene_input'))):
        errors.append(side + '_invalid_fixture_identity_or_workload')
    lengths = row.get('initial_draw_list_lengths')
    if (not isinstance(lengths, list) or len(lengths) != 21 or
            any(type(n) is not int or n < 0 for n in lengths) or lengths[15] <= 0):
        errors.append(side + '_invalid_initial_draw_lists')
    commands = row.get('commands')
    expected_draws = 0
    valid_commands = isinstance(commands, list) and len(commands) == 30
    expected_pass = 'Ice plane' if row.get('season') == 'ice' else 'Water plane'
    if valid_commands:
        for index, command in enumerate(commands):
            if not isinstance(command, dict) or command.get('frame') != index:
                valid_commands = False
                break
            inputs = command.get('input')
            names = command.get('pass_names')
            if not isinstance(inputs, dict) or not isinstance(names, list) or expected_pass not in names:
                valid_commands = False
                break
            required = ('frame_counter', 'game_time_sec', 'game_time_frac_bits',
                        'delta_time_bits', 'inner_resolution', 'render_overrides',
                        'time_of_day_bits', 'camera_bits', 'water_raster_blake3', 'scene')
            scene = inputs.get('scene')
            draws = scene.get('draw_lists') if isinstance(scene, dict) else None
            if (any(key not in inputs or inputs[key] is None for key in required) or
                    not isinstance(inputs.get('camera_bits'), list) or len(inputs['camera_bits']) != 16 or
                    inputs.get('inner_resolution') != EXPECTED_INNER_EXTENT or
                    inputs.get('water_raster_blake3') != row.get('water_raster_blake3') or
                    not scene_input_valid(scene)):
                valid_commands = False
                break
            expected_draws += draws[15]
    if not valid_commands:
        errors.append(side + '_invalid_per_frame_commands')
    actual = row.get('actual_surface_execution')
    counters = ('inline_calls', 'inline_draws', 'worker_calls', 'worker_tasks', 'worker_draws', 'owner_collect_tasks')
    valid_counters = (isinstance(actual, dict) and
                      all(type(actual.get(key)) is int and actual[key] >= 0 for key in counters))
    mode = row.get('surface_execution_mode')
    if valid_counters and valid_commands:
        if mode == 'inline':
            valid_counters = (actual['inline_calls'] == 30 and actual['inline_draws'] == expected_draws
                              and all(actual[key] == 0 for key in
                                      ('worker_calls', 'worker_tasks', 'worker_draws', 'owner_collect_tasks')))
        elif mode == 'collect':
            valid_counters = (actual['worker_calls'] == 30 and actual['worker_tasks'] > 0
                              and actual['worker_draws'] == expected_draws and actual['inline_calls'] == 0
                              and actual['inline_draws'] == 0 and actual['owner_collect_tasks'] == 0)
        else:
            valid_counters = False
    else:
        valid_counters = False
    if not valid_counters:
        errors.append(side + '_actual_surface_execution_invalid')
    return errors


def capture_map(row, side, errors):
    pixels = row.get('pixels')
    if (not isinstance(pixels, list) or len(pixels) != 2 or
            any(not isinstance(shot, dict) or type(shot.get('frame')) is not int for shot in pixels)):
        errors.append(side + '_missing_or_malformed_capture')
        return {}
    shots = {shot['frame']: shot for shot in pixels}
    if len(shots) != 2 or set(shots) != {20, 30}:
        errors.append(side + '_missing_or_duplicate_capture')
    if any(shot.get('file') != f'frame-{shot["frame"]:02}.rgba' for shot in pixels):
        errors.append(side + '_capture_frame_file_binding_invalid')
    return shots


def compare(a, b):
    a, b = Path(a), Path(b)
    result = {'schema': 'surface-collect-pixel-compare-v2', 'a': str(a), 'b': str(b),
              'input_mismatches': [], 'frames': [], 'passes': False,
              'hash_policy': 'SHA256 is independently computed from actual files. BLAKE3 fields are fixture declarations; Python does not verify them.',
              'binary_identity_scope': 'Standalone comparison checks manifest declarations; run_surface_pixels independently binds both processes to verified frozen EXE/PDB SHA256.',
              'limitations': ['Physical headless correctness only; WSI and CPU acceptance are separate.',
                              'Inspect visible water/ice ROI on saved captures before accepting representative surface coverage.']}
    errors = result['input_mismatches']
    rows = []
    for side, directory in (('a', a), ('b', b)):
        try:
            path = directory / 'manifest.json'
            value = json.loads(path.read_text(encoding='utf-8'))
            if not isinstance(value, dict):
                raise ValueError('manifest must be an object')
            result['manifest_' + side + '_sha256'] = sha256(path)
            rows.append(value)
        except (OSError, ValueError, UnicodeError) as error:
            errors.append(side + '_manifest_invalid:' + str(error))
            rows.append({})
    left, right = rows
    for side, row in zip(('a', 'b'), rows):
        errors.extend(manifest_errors(row, side))
    errors.extend(key for key in INPUTS if left.get(key) != right.get(key))
    modes = [row.get('surface_execution_mode') for row in rows]
    result.update(source_a=left.get('source'), source_b=right.get('source'), surface_modes=modes,
                  fixture_declared_binary_blake3=[row.get('binary_blake3') for row in rows])
    same_binary = left.get('source') == right.get('source') and left.get('binary_blake3') == right.get('binary_blake3')
    kind = 'same_binary_repeatability' if same_binary else 'candidate_regression'
    if modes[0] != modes[1]:
        kind = 'same_binary_inline_vs_collect'
        if set(mode for mode in modes if isinstance(mode, str)) != {'inline', 'collect'} or not same_binary:
            errors.append('inline_collect_requires_same_source_and_binary')
    result['comparison_kind'] = kind
    captures = [capture_map(row, side, errors) for side, row in zip(('a', 'b'), rows)]
    for frame in sorted(set(captures[0]) & set(captures[1])):
        evidence = {'frame': frame}
        data = []
        for side, directory, shots in zip(('a', 'b'), (a, b), captures):
            shot = shots[frame]
            try:
                path = artifact_path(directory, shot.get('file'))
                evidence[side] = {'raw_file': path.name, 'raw_file_sha256': sha256(path),
                                  'fixture_declared_blake3': shot.get('blake3'), 'bytes': path.stat().st_size}
                if ((shot.get('width'), shot.get('height')) != (1280, 720) or
                        path.stat().st_size != 1280 * 720 * 4 or not digest_string(shot.get('blake3'))):
                    raise ValueError('capture dimensions, byte count or declared digest are invalid')
                data.append(path.read_bytes())
            except (OSError, ValueError, TypeError) as error:
                errors.append(f'{side}_frame_{frame}_invalid:' + str(error))
                data.append(None)
        if all(value is not None for value in data):
            if captures[0][frame].get('blake3') != captures[1][frame].get('blake3'):
                errors.append(f'frame_{frame}_fixture_digest_declarations_differ')
            changed, maximum, total = 0, 0, 0
            if data[0] != data[1]:
                for offset in range(0, len(data[0]), 4):
                    pixel_changed = False
                    for channel in range(4):
                        delta = abs(data[0][offset + channel] - data[1][offset + channel])
                        pixel_changed |= delta != 0
                        maximum = max(maximum, delta)
                        total += delta
                    changed += pixel_changed
            evidence.update(changed_pixels=changed, pixels=1280 * 720,
                            max_channel_abs=maximum, mean_channel_abs=total / len(data[0]))
        result['frames'].append(evidence)
    result['valid_same_input_comparison'] = not errors
    result['exact_pixels_equal'] = len(result['frames']) == 2 and all(
        row.get('changed_pixels') == 0 for row in result['frames'])
    result['passes'] = result['valid_same_input_comparison'] and result['exact_pixels_equal']
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('a', type=Path)
    parser.add_argument('b', type=Path)
    parser.add_argument('--output', type=Path)
    args = parser.parse_args()
    result = compare(args.a, args.b)
    text = json.dumps(result, ensure_ascii=False, indent=2) + '\n'
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(text, encoding='utf-8')
    print(text, end='')
    return 0 if result['passes'] else 1


if __name__ == '__main__':
    raise SystemExit(main())
