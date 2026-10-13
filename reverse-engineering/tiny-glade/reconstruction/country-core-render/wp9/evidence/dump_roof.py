#!/usr/bin/env python3
"""Full roof-domain bundle dump: every function with rva, every structure.
Usage: dump_roof.py <analysis_root>  (prints to stdout)
"""
import gzip, json, sys, os

SKEL = None
ANALYSIS = sys.argv[1]
SKEL = os.path.join(ANALYSIS, "skeleton", "B")

BUNDLES = [
    # (rel_path_dir, base)
    ("crates/country-core/src/resources/roofs", "roof_tiles"),
    ("crates/country-core/src/resources/roofs", "roof_shape"),
    ("crates/country-core/src/resources/roofs", "roof_tile_sim"),
    ("crates/country-core/src/resources/roofs", "roof_animation"),
    ("crates/country-core/src/resources/roofs", "roof_buffers"),
    ("crates/country-core/src/resources/roofs", "roof_noise"),
    ("crates/country-core/src/resources/roofs", "roof_ridge"),
    ("crates/country-core/src/resources/roofs", "roof_trigger_volume"),
    ("crates/country-core/src/resources/roofs", "roof_visual_shape"),
    ("crates/country-core/src/resources/roofs", "dormer_tiles"),
    ("crates/country-core/src/resources/roofs", "commands"),
    ("crates/country-core/src/resources", "color_id_roof"),
    ("crates/country-core/src/resources/history", "roof_edit"),
    ("crates/country-core/src/systems/render_loop/compute_passes", "roof_tiles_gravity"),
    ("crates/country-core/src/systems/render_loop/draw_passes/draw_shaders", "instanced_roof"),
    ("crates/country-core/src/systems/audio", "roof_audio"),
    ("crates/country-core/src/startup", "startup_roof"),
    # systems/roof crate
    ("crates/systems/roof/src", "create_roof"),
    ("crates/systems/roof/src", "delete_roof"),
    ("crates/systems/roof/src", "edit_roof"),
    ("crates/systems/roof/src", "update_roof_spatial"),
    ("crates/systems/roof/src", "simulate_roof_tile_for_audio"),
    ("crates/systems/roof/src", "wall_height_dt"),
    ("crates/systems/roof/src", "highlight_roof"),
    ("crates/systems/roof/src", "replay_roof"),
    ("crates/systems/roof/src", "deferred_record_roof_history"),
    ("crates/systems/roof/src", "ui_edit_roof"),
    ("crates/systems/roof/src", "ui_edit_roof_ridge_dir"),
    ("crates/systems/roof/src", "ui_edit_roof_ridge_length"),
    ("crates/systems/roof/src", "ui_edit_roof_tip"),
    ("crates/systems/roof/src", "roof_trigger_volume"),
]

def load_gz(p):
    with gzip.open(p, 'rt') as f:
        return json.load(f)

def load(p):
    with open(p, 'rt') as f:
        return json.load(f)

for rel, base in BUNDLES:
    d = os.path.join(SKEL, rel)
    print(f"\n########## {rel}/{base} ##########")
    pdbp = os.path.join(d, f"{base}.rs.pdb.json")
    if os.path.exists(pdbp):
        p = load(pdbp)
        print(f"lines={p.get('line_blocks')} inline={p.get('inline_sites')} proc={p.get('procedure_instances')} sha={p.get('source_sha256')}")
    sp = os.path.join(d, f"{base}.rs.structures.json.gz")
    if os.path.exists(sp):
        s = load_gz(sp)
        for e in s.get('named_symbol_entities', []):
            print(f"  NSYM {e['name']} | {e.get('definition_status')} | size={e.get('size_bytes')}")
        for e in s.get('observed_game_structures', []):
            print(f"  OBS {e.get('name')} | {e.get('definition_status')} | size={e.get('size_bytes')}")
            for fl in e.get('fields', []) or []:
                if isinstance(fl, dict):
                    print(f"      field off={fl.get('offset')} ty={fl.get('type')} desc={fl.get('description','')[:120]}")
    ep = os.path.join(d, f"{base}.rs.enriched.json.gz")
    if os.path.exists(ep):
        e = load_gz(ep)
        for f in e.get('functions', []):
            inst = f.get('compiled_instances', [])
            rvas = sorted(set(i.get('rva') for i in inst if i.get('rva')))
            lines = sorted(set(i.get('source_line') for i in inst if i.get('source_line')))
            cr = len(f.get('reference_callers', []))
            print(f"  FN {f['name']} | rva={rvas} line={lines} callers={cr}")
