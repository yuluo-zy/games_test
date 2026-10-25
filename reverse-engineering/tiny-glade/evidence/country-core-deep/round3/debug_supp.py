import json, os, random
from collections import Counter

CH = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters"
OUT = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/round3"

DOMAINS = {
    "history": ["systems/history", "resources/history"],
    "wall": ["systems/wall", "resources/walls", "stairs_walls"],
    "session-save": ["systems/save", "resources/session", "systems/prefab", "resources/prefab", "loading_screen", "logo_animation"],
    "terrain-water": ["terrain", "water", "world_raster", "raster_stroke", "garden", "in_progress_curve", "drawing_input_smoother", "brush_sizes", "cursor_terrain_raycast", "mouse_terrain_raycast", "read_mouse_depth_value", "mesh_follow_terrain", "rocky_terrain", "glade_border", "garden_mask", "water_mask"],
    "ui-input": ["systems/ui", "ui_", "mode_manager", "mod_key", "input", "mouse_capture", "toolbar", "photo_mode", "onboarding", "gui"],
    "camera-time": ["camera_rig", "main_camera", "look_at_camera", "ui_camera_mode_gizmos", "time_of_day", "time_update", "weather", "daily", "glade_settings", "timelapse_replay", "arrow_gizmo_animation", "sparkle", "clear_canvas", "despawn"],
    "collision-eco": ["systems/collision", "stairs", "critters", "paths", "ivy", "clutter_meshes", "preplaced_autoclutter"],
    "audio-misc": ["audio", "steam", "debug", "shutdown", "feedback_report", "gpu_lut", "grading_lut", "color_icon", "color_id", "gothic_window_profile", "glade_color_customization", "image_formation", "placement_data", "texture_presets", "render_ssbo", "shader_refactoring_state", "window", "editing_highlight", "app_loading_state", "frozen_map", "meta_commander", "name_hash"],
    "render-loop": ["render_loop"],
    "roofs-rt": ["resources/roofs", "resources/rt", "resources/render", "tree_storage", "shape_intersections", "resources/mod", "resources/time"],
    "backend-misc": ["systems/backend", "systems/mod", "systems/utils", "systems/utils_intersect", "brush_preview"],
}


def dom(f):
    f = f.replace("\\", "/")
    for d, ks in DOMAINS.items():
        for k in ks:
            if k in f:
                return d
    return "??"


def extract(include_dirs):
    rows = []
    for dirpath, _d, fns in os.walk(CH):
        for fn in fns:
            if not fn.endswith(".md"):
                continue
            rel = os.path.relpath(os.path.join(dirpath, fn), CH).replace("\\", "/")
            top = rel.split("/")[0] if "/" in rel else "(top)"
            if top not in include_dirs:
                continue
            if dom(rel) != "??":
                continue
            for line in open(os.path.join(dirpath, fn), encoding="utf-8", errors="replace"):
                if not line.startswith("|") or line.startswith("|---"):
                    continue
                cells = [c.strip() for c in line.strip("|").split("|")]
                if len(cells) < 6 or not cells[0].startswith("`"):
                    continue
                if not cells[4].startswith("O"):
                    continue
                if not re.match(r"^0x[0-9a-fA-F]{8,16}$", cells[1]):
                    continue
                rows.append({"domain": "??", "rel": rel, "fn": cells[0], "addr": cells[1],
                             "claim": cells[4], "evid": cells[5] if len(cells) > 5 else ""})
    return rows


import re

r1 = json.load(open(os.path.join(OUT, "O-sample-42.json"), encoding="utf-8"))
excl = set((dom(e["rel"]), e["addr"]) for e in r1)

supp_dirs = ["components", "startup", "utils", "geometry", "(top)"]
pool = [r for r in extract(supp_dirs) if ("??", r["addr"]) not in excl]
print("supp pool size:", len(pool))
c = Counter(r["rel"].split("/")[0] if "/" in r["rel"] else "(top)" for r in pool)
print("by dir:", dict(c))
for r in pool:
    print(r["rel"], "|", r["addr"], "|", r["claim"][:70])
