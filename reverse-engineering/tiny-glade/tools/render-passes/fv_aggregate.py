#!/usr/bin/env python3
"""F-V aggregation: run recompile_check on every owned shader against its target bins
and collect per-shader validation results into evidence/render-passes/recovery/f-v/.

For each shader source path, finds its bins in compile-commands.json, runs the
validator (the F-V wrapper for push-constant targets), and records for each module:
bindings/entries/local_size match + opcode-histogram diff summary.
Include-only files are recorded as compile-test only (no main module to fingerprint).
"""
import json, subprocess, sys, os
from pathlib import Path

ROOT = Path('/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade')
GAME = '/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/compiled-shaders'
CC = ROOT / 'evidence/render-passes/validation/compile-commands.json'

MINE = [
    'shaders/roof_snow.raster.hlsl', 'shaders/roof_tiles_gravity.cs.hlsl',
    'shaders/leaf_particles.cs.hlsl', 'shaders/leaf_particle.raster.hlsl',
    'shaders/rain_particles.raster.hlsl', 'shaders/snow_particles.raster.hlsl',
    'shaders/sparkles.raster.hlsl', 'shaders/splash_particle.raster.hlsl',
    'shaders/sheep.raster.hlsl', 'shaders/lilypad.raster.hlsl',
    'shaders/lilypad_flower.raster.hlsl', 'shaders/reed.raster.hlsl',
    'shaders/flag_roof_cloth.raster.hlsl', 'shaders/flag_small.raster.hlsl',
    'shaders/flag_wall_cloth.raster.hlsl',
    'shaders/stairs_floor.inc.hlsl', 'shaders/smoke.inc.hlsl',
    'shaders/plaster.inc.hlsl', 'shaders/wooden_plank.inc.hlsl',
    'shaders/roof_material_id.inc.hlsl',
    'shaders/critters/butterfly.raster.hlsl', 'shaders/critters/duck_animated.raster.hlsl',
    'shaders/critters/firefly.raster.hlsl', 'shaders/critters/firefly_glow.raster.hlsl',
    'shaders/critters/firefly_vis_query.cs.hlsl', 'shaders/critters/nani_wren.raster.hlsl',
    'shaders/roof/dormer_hole.inc.hlsl', 'shaders/roof/roof_tile.inc.hlsl',
    'shaders/roof/stone_floor.inc.hlsl', 'shaders/roof/wooden_floor.inc.hlsl',
    'shaders/ivy/arch_noise_discard.inc.hlsl', 'shaders/ivy/instanced_ivy_branch.raster.hlsl',
    'shaders/ivy/instanced_ivy_leaf.raster.hlsl', 'shaders/ivy/ivy.raster.hlsl',
]

def module_summary(r):
    if r.get('dxc_returncode'):
        return {'verdict': 'FAIL_DXC', 'entry': r['entry'], 'profile': r['profile'],
                'stderr': (r.get('dxc_stderr') or '')[:200]}
    d = r.get('differences', {})
    bindings_ok = 'bindings' not in d
    entries_ok = 'entries' not in d
    local_ok = 'local_size' not in d
    hist = d.get('opcode_histogram', {})
    hist_n = {k: len(v.get('target_only', {})) for k, v in [('opcode_histogram', hist)]}
    return {
        'verdict': r['verdict'], 'entry': r['entry'], 'profile': r['profile'],
        'bindings_match': bindings_ok,
        'entries_match': entries_ok,
        'local_size_match': local_ok,
        'capabilities_only_diff': list(d.keys()) == ['capabilities'],
        'histogram_target_only_ops': len(hist.get('target_only', {})),
        'histogram_candidate_only_ops': len(hist.get('candidate_only', {})),
        'histogram_diff_sample': list(hist.get('target_only', {}).items())[:6],
    }

def main():
    os.chdir(ROOT)
    cc = json.load(open(CC))
    out = {'generated': '2026-10-13', 'owner': 'F-V', 'shaders': {}}
    # collect all bins per source path
    by_src = {}
    for c in cc['compile_commands']:
        src = c.get('main_source_path')
        if src in MINE:
            by_src.setdefault(src, []).append(c)
    for src, cmds in by_src.items():
        name = src.replace('shaders/', '')
        hlsl = ROOT / 'reconstruction/shader-source/shaders' / name
        if not hlsl.exists():
            out['shaders'][name] = {'status': 'MISSING', 'bins': [c['file'] for c in cmds]}
            continue
        entries = []
        for c in cmds:
            tool = 'recompile_check_fv.py' if src == 'shaders/roof_tiles_gravity.cs.hlsl' else 'recompile_check.py'
            out_json = str(ROOT / ('evidence/render-passes/recovery/f-v/_check_' + c['file'] + '.json'))
            proc = subprocess.run(
                ['python3', 'tools/render-passes/' + tool, str(hlsl), GAME + '/' + c['file'],
                 '-I', 'reconstruction/shader-source/shaders', '--out', out_json],
                capture_output=True, text=True, timeout=600)
            try:
                rep = json.load(open(ROOT / ('evidence/render-passes/recovery/f-v/_check_' + c['file'] + '.json')))
            except Exception as e:
                out['shaders'][name] = {'status': 'VALIDATOR_ERROR', 'bins': [c['file']],
                                        'error': str(e)[:120], 'stderr': proc.stderr[-200:]}
                continue
            entries.append({'bin': c['file'],
                           'modules': [module_summary(m) for m in rep['modules']]})
        out['shaders'][name] = {'status': 'reconstructed', 'bins': [c['file'] for c in cmds],
                                'bins_validated': entries}
    # include-only files with no main bin: record compile-test
    include_only = [s for s in MINE if s.endswith('.inc.hlsl')]
    done_srcs = set(out['shaders'].keys())
    for s in include_only:
        name = s.replace('shaders/', '')
        if name not in out['shaders']:
            out['shaders'][name] = {'status': 'include_only_compile_test', 'bins': []}
    out['summary'] = {
        'total_files': len(MINE),
        'reconstructed': sum(1 for v in out['shaders'].values() if v.get('status') == 'reconstructed'),
        'missing': sum(1 for v in out['shaders'].values() if v.get('status') == 'MISSING'),
        'include_only': sum(1 for v in out['shaders'].values() if v.get('status') == 'include_only_compile_test'),
        'b903_modules_bindings_match': sum(
            1 for v in out['shaders'].values()
            for b in v.get('bins_validated', []) if 'b903' in b['bin']
            for m in b['modules'] if m.get('bindings_match')),
        'b903_modules_total': sum(
            1 for v in out['shaders'].values()
            for b in v.get('bins_validated', []) if 'b903' in b['bin']
            for m in b['modules']),
    }
    Path('evidence/render-passes/recovery/f-v/per-shader-results.json').write_text(
        json.dumps(out, indent=2))
    print('wrote per-shader-results.json')

if __name__ == '__main__':
    main()
