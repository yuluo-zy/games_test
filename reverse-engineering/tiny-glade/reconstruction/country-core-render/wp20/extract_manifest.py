#!/usr/bin/env python3
"""Extract WP20 skeleton PDB manifest: procedures (name/va/code_size/public_aliases)
and named symbol entities (types) from the B skeleton .pdb.jsonl.gz + structures.json.gz."""
import gzip, io, json, sys
from pathlib import Path

SK = Path("/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src/systems")

# WP20 unit set: module sub-path -> list of unit file stems (relative to systems/)
UNITS = {
    "backend/input": ["calculate_input_delta", "input_record_replay", "input_replay_for_trailers",
                       "reset_input_state", "translate_input", "translate_input_to_cursor_moved",
                       "update_cursor_position"],
    "camera_rig": ["butterfly", "camera_body", "free", "normal", "walk"],
    "": ["main_camera", "look_at_camera", "arrow_gizmo_animation", "brush_preview",
         "clear_canvas", "despawn", "mesh_follow_terrain", "mod_key", "utils", "utils_intersect",
         "time_of_day", "time_update", "timelapse_replay", "weather_system", "world_raster",
         "mode_manager", "prefab", "ui_2d_follow_mouse", "ui_camera_mode_gizmos",
         "ui_edit_focus", "ui_edit_freehand_walls", "ui_focus_mode_shape_height",
         "ui_move_freehand_walls", "ui_move_shapes", "ui_raise_shape"],
    "ui_clutter": ["debug_spawn_clutter", "decoclutter_show_pins", "defered_autoclutter_history",
                   "rmb_autoclutter", "ui_autoclutter", "ui_focus_decoclutter", "mod"],
    "collision": [],  # wp16-owned; manifest only for reference
}

out = {}

for sub, stems in UNITS.items():
    base = SK / sub if sub else SK
    for stem in stems:
        pdb_gz = base / f"{stem}.rs.pdb.jsonl.gz"
        struct_gz = base / f"{stem}.rs.structures.json.gz"
        rel = f"{sub}/{stem}.rs" if sub else f"{stem}.rs"
        ent = {"procedures": [], "types": [], "rel": rel}
        if pdb_gz.exists():
            try:
                with gzip.open(pdb_gz, "rt", encoding="utf-8", errors="replace") as f:
                    for line in f:
                        line = line.strip()
                        if not line:
                            continue
                        d = json.loads(line)
                        if d.get("event") == "symbol" and d.get("class") == "procedure":
                            nm = d.get("name", "")
                            # keep only country_core::systems procedures owned by this file
                            if nm.startswith("country_core::systems::"):
                                va = d.get("va")
                                aliases = [a["name"] for a in d.get("public_aliases", [])
                                           if a.get("flags") and "Function" in a["flags"]]
                                ent["procedures"].append({
                                    "name": nm, "va": va, "code_size": d.get("code_size"),
                                    "type_index": d.get("type_index"),
                                    "source_line": d.get("source_line"),
                                    "real_aliases": [a for a in aliases if "country_core" in a][:4],
                                })
            except Exception as e:
                ent["pdb_error"] = repr(e)
        if struct_gz.exists():
            try:
                with gzip.open(struct_gz, "rt", encoding="utf-8", errors="replace") as f:
                    d = json.load(f)
                for nse in d.get("named_symbol_entities", []):
                    nm = nse.get("name", "")
                    ent["types"].append({
                        "name": nm,
                        "kind": nse.get("kind"),
                        "size": nse.get("size_bytes"),
                        "def_status": nse.get("definition_status"),
                    })
            except Exception as e:
                ent["struct_error"] = repr(e)
        out[rel] = ent

json.dump(out, sys.stdout, ensure_ascii=False, indent=1)
