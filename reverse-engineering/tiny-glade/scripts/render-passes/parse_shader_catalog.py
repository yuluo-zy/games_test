#!/usr/bin/env python3
"""Tiny Glade original-compiled-shaders GPU pass inventory.

Parses all 404 SPIR-V .bin files under the distribution's compiled-shaders/
directory (filename inventory + tolerant SPIR-V word-level reflection) and
emits a machine-readable pass catalog.

Usage:
    parse_shader_catalog.py <shader_dir> <spirv_grammar.json> <out_catalog.json> <existing_reflection.json>

The existing_reflection.json (evidence/render-architecture/shader-reflection.json)
is cross-referenced: for the files it already covers, its verbatim module data
(bindings/capabilities/opcode_histogram) is reused and the file/module sha256 is
verified against this parser's own computation.

This is analysis-only evidence generation. It does not run the game.
"""

import collections
import hashlib
import json
import re
import struct
import sys
from pathlib import Path

SHADER_DIR = Path(sys.argv[1])
GRAMMAR_PATH = Path(sys.argv[2])
OUT_PATH = Path(sys.argv[3])
REFLECTION_PATH = Path(sys.argv[4])

# ---------------------------------------------------------------- grammar
_grammar = json.loads(GRAMMAR_PATH.read_text())
_OPS = {i['opcode']: i['opname'] for i in _grammar['instructions']}
_ENUMS = {}
for k in _grammar['operand_kinds']:
    _ENUMS[k['kind']] = {
        (int(v['value'], 0) if isinstance(v['value'], str) else v['value']): v['enumerant']
        for v in k.get('enumerants', [])
    }

def _enum(kind, value):
    return _ENUMS.get(kind, {}).get(value, str(value))

# ---------------------------------------------------------------- helpers
def _words_to_bytes(ws):
    return struct.pack('<' + 'I' * len(ws), *ws)


def _words_to_string(ws):
    raw = _words_to_bytes(ws)
    return raw.split(b'\x00')[0].decode('utf-8', errors='replace')


SPIRV_MAGIC = b'\x03\x02\x23\x07'

# opcode constants we care about
OP_ENTRY_POINT = 15
OP_EXEC_MODE = 16
OP_CAPABILITY = 17
OP_NAME = 5
OP_MEMBER_NAME = 6
OP_DECORATE = 71
OP_MEMBER_DECORATE = 72
OP_VARIABLE = 59
OP_TYPE_POINTER = 32
OP_TYPE_IMAGE = 25
OP_TYPE_SAMPLER = 26
OP_TYPE_SAMPLED_IMAGE = 27
OP_TYPE_STRUCT = 30
OP_TYPE_ACCEL = 5349  # OpTypeAccelerationStructureKHR (SPIR-V 1.4)

DEC_BLOCK = 'Block'
DEC_BUFFER_BLOCK = 'BufferBlock'
DEC_DESCRIPTOR_SET = 'DescriptorSet'
DEC_BINDING = 'Binding'

# ---------------------------------------------------------------- SPIR-V module parse
def parse_module(payload):
    n = len(payload) // 4
    w = struct.unpack('<' + 'I' * n, payload)
    if w[0] != 0x07230203:
        return None
    names = {}          # id -> OpName string
    member_names = {}   # (struct_id, member_idx) -> name
    deco = collections.defaultdict(list)   # id -> [{'kind','args'}]
    variables = {}      # var result id -> {'type': ptr_type_id, 'storage': storageclass}
    pointed = {}        # ptr type id -> pointee type id
    image_info = {}     # image type id -> {'dim','sampled'}
    sampler_types = set()
    sampledimage_types = set()
    struct_types = set()
    accel_types = set()
    entries = []
    modes = []
    caps = []
    opcount = collections.Counter()

    i = 5  # skip magic, version, generator, bound, schema
    while i < n:
        wc = w[i] >> 16
        op = w[i] & 0xffff
        if wc == 0 or i + wc > n:
            break
        a = w[i + 1:i + wc]
        opcount[_OPS.get(op, str(op))] += 1
        if op == OP_NAME:
            if len(a) >= 2:
                names[a[0]] = _words_to_string(a[1:])
        elif op == OP_MEMBER_NAME:
            if len(a) >= 3:
                member_names[(a[0], a[1])] = _words_to_string(a[2:])
        elif op == OP_DECORATE:
            if len(a) >= 2:
                deco[a[0]].append({'kind': _enum('Decoration', a[1]), 'args': list(a[2:])})
        elif op == OP_MEMBER_DECORATE:
            pass
        elif op == OP_VARIABLE:
            # operands: <id> result type, <id> result, Storage Class
            if len(a) >= 3:
                variables[a[1]] = {'type': a[0], 'storage': _enum('StorageClass', a[2])}
        elif op == OP_ENTRY_POINT:
            # operands: Execution Model, <id> Entry Point, Name, ... interface ids
            if len(a) >= 3:
                entries.append({'stage': _enum('ExecutionModel', a[0]), 'entry': _words_to_string(a[2:])})
        elif op == OP_EXEC_MODE:
            # operands: <id> Entry Point, Execution Mode, ... literals
            if len(a) >= 2:
                modes.append({'mode': _enum('ExecutionMode', a[1]), 'args': list(a[2:])})
        elif op == OP_CAPABILITY:
            if a:
                caps.append(_enum('Capability', a[0]))
        elif op == OP_TYPE_POINTER:
            # operands: <id> Result, Storage Class, Type
            if len(a) >= 3:
                pointed[a[0]] = a[2]
        elif op == OP_TYPE_IMAGE:
            # operands: <id> Result, Sampled Type, Dim, Depth, Arrayed, MS, Sampled, Image Format[, Access]
            if len(a) >= 8:
                image_info[a[0]] = {
                    'dim': _enum('Dim', a[2]),
                    'depth': a[3],
                    'arrayed': a[4],
                    'ms': a[5],
                    'sampled': a[6],
                    'format': _enum('ImageFormat', a[7]),
                }
        elif op == OP_TYPE_SAMPLER:
            if a:
                sampler_types.add(a[0])
        elif op == OP_TYPE_SAMPLED_IMAGE:
            if a:
                sampledimage_types.add(a[0])
        elif op == OP_TYPE_STRUCT:
            if a:
                struct_types.add(a[0])
        elif op == OP_TYPE_ACCEL:
            if a:
                accel_types.add(a[0])
        i += wc

    # ---- binding kind inference
    def infer_kind(var_type_id, storage):
        if storage == 'PushConstant':
            return 'PushConstant'
        t = pointed.get(var_type_id)
        if t in image_info:
            img = image_info[t]
            if img['sampled'] == 2:
                return 'StorageImage(%s)' % img['dim']
            if img['sampled'] == 1:
                return 'SampledImage(%s)' % img['dim']
            return 'Image(sampled=runtime)'
        if t in sampledimage_types:
            return 'SampledImage'
        if t in sampler_types:
            return 'Sampler'
        if t in accel_types:
            return 'AccelerationStructure'
        if storage == 'UniformConstant':
            if t in struct_types:
                return 'UniformConstant(struct)'  # bindless/buffer-ish carrier
            return 'UniformConstant(other)'
        if storage == 'Uniform':
            if t in struct_types:
                d = [x['kind'] for x in deco.get(t, [])]
                if DEC_BLOCK in d:
                    return 'UniformBuffer(UBO)'
                if DEC_BUFFER_BLOCK in d:
                    return 'StorageBuffer(SSBO,BufferBlock)'
                return 'Uniform-struct(no Block decor)'
            return 'Uniform(non-struct)'
        if storage == 'StorageBuffer':
            return 'StorageBuffer(SSBO)'
        return storage

    bindings = []
    for vid, info in variables.items():
        dset = None
        bind = None
        for d in deco.get(vid, []):
            if d['kind'] == DEC_DESCRIPTOR_SET:
                dset = d['args'][0] if d['args'] else None
            elif d['kind'] == DEC_BINDING:
                bind = d['args'][0] if d['args'] else None
        is_push = info['storage'] == 'PushConstant'
        if dset is not None or bind is not None or is_push:
            bindings.append({
                'name': names.get(vid),
                'storage': info['storage'],
                'set': dset,
                'binding': bind,
                'kind': infer_kind(info['type'], info['storage']),
            })

    bindings.sort(key=lambda b: (b['set'] if b['set'] is not None else 0,
                                 b['binding'] if b['binding'] is not None else 0))
    debug_names = [v for v in names.values()]
    return {
        'entries': entries,
        'execution_modes': modes,
        'capabilities': caps,
        'bindings': bindings,
        'debug_names': debug_names,
        'member_names': [{'struct_id': k[0], 'member': k[1], 'name': v}
                         for k, v in sorted(member_names.items())],
        'opcode_histogram': dict(opcount),
    }


def parse_bin(path):
    b = path.read_bytes()
    modules = []
    cursor = 0
    while True:
        at = b.find(SPIRV_MAGIC, cursor)
        if at < 0:
            break
        n = struct.unpack_from('<I', b, at - 4)[0]
        payload = b[at:at + 4 * n]
        if len(payload) != 4 * n:
            break
        m = parse_module(payload)
        if m is not None:
            m['offset'] = at
            m['sha256'] = hashlib.sha256(payload).hexdigest()
            modules.append(m)
        cursor = at + 4 * n
    return {
        'file_sha256': hashlib.sha256(b).hexdigest(),
        'size': len(b),
        'modules': modules,
    }


# ---------------------------------------------------------------- filename inventory
# _<pass>.<stage>.hlsl_<entry>[,_<variant>].bin
FN_RE = re.compile(r'^(.+)\.(cs|raster)\.hlsl_(.+?)\.bin$')
HEX16 = re.compile(r'^[0-9a-f]{16}$')


def parse_filename(name):
    m = FN_RE.match(name)
    if not m:
        return None
    pass_name, stage, tail = m.group(1), m.group(2), m.group(3)
    parts = tail.split('_')
    if stage == 'raster':
        # tail is a single 16-hex source-hash; entry point lives inside SPIR-V
        variant = tail if len(parts) == 1 and HEX16.match(tail) else tail
        entry_hint = None
    else:
        if len(parts) >= 2 and (parts[-1] == '0' or HEX16.match(parts[-1])):
            entry_hint = '_'.join(parts[:-1])
            variant = parts[-1]
        else:
            entry_hint = tail
            variant = None
    return {
        'pass': pass_name,
        'stage': stage,
        'entry_hint': entry_hint,
        'variant': variant,
    }


# ---------------------------------------------------------------- pass families
# ordered rules; first match wins. Every pass must land in exactly one family.
def classify_family(pass_name, stage):
    p = pass_name
    S = lambda *ss: any(p.startswith(s) for s in ss)
    E = lambda *ee: p in ee

    def nani_cull():
        return (S('_nani_cull_and_bucket')
                or E('_nani_generate_draw_lists', '_nani_clear_draw_lists', '_nani_clear_histogram',
                     '_nani_clear_subtotals', '_nani_histogram_prefix_sum', '_nani_subtotal_prefix_sum',
                     '_nani_group_draw_cmd_prefix_sum', '_nani_generate_dispatch_args',
                     '_nani_allocate_draw_commands', '_nani_fill_sorted_instance_list',
                     '_nani_fill_unsorted_instance_indirection', '_nani_fill_unsorted_instance_list',
                     '_nani_prefix_sum_unsorted_instances_per_atlas_entry',
                     '_nani_prepare_unsorted_draw_lists', '_nani_draw_indexed_indirect_to_dispatch'))

    def nani_raster():
        return p.startswith('_nani_') and not nani_cull()

    rules = [
        (nani_cull, 'nani_cull_bucket'),
        (lambda: E('_wall_generate_draw_lists', '_wall_reset_pass2_inst_counts'), 'wall_draw_lists'),
        (nani_raster, 'nani_raster_main'),
        (lambda: S('_deferred_'), 'deferred_raster'),
        (lambda: E('_mei_shadow', '_shadow_mask_mean', '_apply_screen_shadows'), 'shadow'),
        (lambda: E('_generate_hzb'), 'hzb'),
        (lambda: E('_extract_half_res_depth_normals', '_copy_depth_to_r'), 'depth_normals'),
        (lambda: E('_calculate_reprojection_map', '_rdi_temporal'), 'reprojection'),
        (lambda: S('_gi_'), 'gi'),
        (lambda: S('_ssgi_'), 'ssgi'),
        (lambda: S('_light_binning_'), 'light_binning'),
        (lambda: E('_taa') or S('_tgsr'), 'taa_tgsr'),
        (lambda: S('_fsr_') or E('_fxaa'), 'fsr_fxaa'),
        (lambda: S('_dof_'), 'dof'),
        (lambda: E('_apply_volumetrics'), 'volumetrics'),
        (lambda: S('_water_', '_waterfall', '_ice_plane') or E('_lut_water_noise'), 'water'),
        (lambda: E('_rasterize_buildings', '_rasterize_buildings_animate'), 'building_rasterize'),
        (lambda: S('_terrain_') or E('_terrain_derivative', '_terrain_lighting_texture',
                                     '_demo_terrain_overlay', '_timed_terrain_gizmo'), 'terrain'),
        (lambda: S('_paths_') or E('_path_pebble', '_path_pebbles'), 'path'),
        (lambda: S('_rocky_terrain_'), 'rocky_terrain_stairs'),
        (lambda: E('_upload_cpu_raster', '_read_terrain_stroke', '_mouse_terrain_raycast',
                   '_read_mouse_depth_value'), 'terrain'),
        (lambda: S('_grass', '_billboard_', '_clearing_', '_leaf_particle', '_ivy_', '_garden_')
         or E('_generate_grass', '_leaf_particles', '_lilypad', '_lilypad_flower', '_reed',
              '_stairs_foliage_exclusion'), 'vegetation_grass'),
        (lambda: S('_roof_', '_instanced_roof_', '_flag_'), 'roof'),
        (lambda: S('_critters_') or E('_rain_particles', '_snow_particles', '_sparkles',
                                      '_splash_particle', '_sheep'), 'critters_particles'),
        (lambda: E('_skydome', '_cloud', '_star'), 'sky_cloud'),
        (lambda: S('_egui', '_ui_2d') or E('_text', '_custom_cursor', '_dashed_line', '_timer_circle'), 'ui'),
        (lambda: S('_blit_') or E('_resolve_to_swapchain', '_final_post'), 'blit_resolve_swapchain'),
        (lambda: S('_painterly_', '_vermeer_') or E('_lut_gallery_thumbnail'), 'painterly_vermeer'),
        (lambda: S('_session_selection_', '_logo_') or E('_loading_screen'), 'session_ui'),
        (lambda: S('_screenshot_capture_') or E('_capture_panic_screenshot'), 'screenshot_capture'),
        (lambda: S('_blur', '_gauss_blur') or E('_generate_mips'), 'blur_mips'),
        (lambda: S('_prefix_scan_'), 'prefix_scan'),
        (lambda: E('_lut_brdf_fg'), 'lut'),
        (lambda: E('_normal_debug', '_texture_test', '_shader_refactoring', '_primary_rt_debug',
                   '_fill_buffer', '_painterly_debug_target'), 'debug_dev'),
        (lambda: S('_solid_vertex_color') or E('_vertex_color'), 'main_raster_simple'),
        (lambda: S('_wall_wall_'), 'wall_raster'),
        (lambda: True, 'unclassified'),
    ]
    for pred, fam in rules:
        if pred():
            return fam
    return 'unclassified'


# ---------------------------------------------------------------- main
def main():
    reflection = None
    if REFLECTION_PATH.exists():
        reflection = json.loads(REFLECTION_PATH.read_text())

    reflection_by_file = {}
    if reflection:
        for f in reflection['files']:
            reflection_by_file[f['file'].replace('\\', '/').split('/')[-1]] = f

    files = sorted(SHADER_DIR.glob('*.bin'))
    if not files:
        sys.exit('no .bin files found in %s' % SHADER_DIR)

    passes = collections.defaultdict(dict)
    used_reflection = []
    crosscheck = []
    for path in files:
        meta = parse_filename(path.name)
        if meta is None:
            crosscheck.append({'file': path.name, 'problem': 'unparsable filename'})
            continue
        parsed = parse_bin(path)
        pname = meta['pass']
        d = passes.setdefault(pname, {
            'stage': set(),
            'files': [],
        })
        d['stage'].add(meta['stage'])

        d['files'].append({
            'file': path.name,
            'stage': meta['stage'],
            'sha256': parsed['file_sha256'],
            'size': parsed['size'],
            'entry_hint': meta['entry_hint'],
            'variant': meta['variant'],
            'modules': parsed['modules'],
        })

        # cross-reference existing reflection (verbatim reuse + sha verify)
        ref = reflection_by_file.get(path.name)
        if ref is not None:
            used_reflection.append(path.name)
            ok = True
            ref_mods = ref['modules']
            my_mods = parsed['modules']
            if len(ref_mods) != len(my_mods):
                ok = False
            if ref.get('sha256') != parsed['file_sha256']:
                ok = False
            for rm, mm in zip(ref_mods or [], my_mods or []):
                if rm.get('sha256') != mm['sha256']:
                    ok = False
            crosscheck.append({'file': path.name, 'reused_reflection': True,
                               'file_sha256_match': ref.get('sha256') == parsed['file_sha256'],
                               'module_count_match': len(ref_mods) == len(my_mods),
                               'all_module_sha_match': ok})

    # normalize pass entries
    pass_list = []
    for pname in sorted(passes.keys()):
        d = passes[pname]
        stages = sorted(d['stage'])
        family = classify_family(pname, stages)
        files_sorted = sorted(d['files'], key=lambda x: x['file'])
        variant_count = len(files_sorted)
        pass_list.append({
            'pass': pname,
            'family': family,
            'stage': stages,
            'variant_count': variant_count,
            'variants': files_sorted,
        })

    families = collections.OrderedDict()
    for p in pass_list:
        families.setdefault(p['family'], []).append(p['pass'])

    total_files = sum(p['variant_count'] for p in pass_list)
    raster_files = sum(1 for p in pass_list for f in p['variants'] if 'raster' in f['stage'])
    compute_files = sum(1 for p in pass_list for f in p['variants'] if 'cs' in f['stage'])

    out = {
        'generated': '2026-10-13',
        'source_dir': str(SHADER_DIR),
        'file_count': len(files),
        'file_count_sum': total_files,
        'raster_file_count': raster_files,
        'compute_file_count': compute_files,
        'pass_families': {k: v for k, v in families.items()},
        'passes': pass_list,
        'cross_reference': {
            'reused_reflection_files': sorted(used_reflection),
            'checks': crosscheck,
        },
        'limitations': [
            'Reflection is static SPIR-V declaration/name evidence only; no runtime binding of variants, no GPU timeline.',
            'Binding kind is inferred from SPIR-V type declarations and may mislabel opaque carriers; names are OpName debug names.',
            'Raster files contain 2 SPIR-V modules (vertex+fragment); compute files contain 1 module (GLCompute).',
            'Variant hashes (16-hex) denote distinct variant compilations, not proof a given variant runs every frame.',
        ],
    }

    OUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    OUT_PATH.write_text(json.dumps(out, indent=2), encoding='utf-8')
    print('wrote', OUT_PATH)

    # console summary
    print('FILES:', len(files))
    for fam, ps in families.items():
        print('%-22s %3d passes  %s' % (fam, len(ps), ', '.join(ps[:8]) + (' ...' if len(ps) > 8 else '')))
    print('REUSED reflection files:', len(used_reflection))


if __name__ == '__main__':
    main()
