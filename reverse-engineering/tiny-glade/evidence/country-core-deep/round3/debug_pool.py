import json, os, re
from collections import Counter

CH = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters"
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


c = Counter()
for dirpath, _d, fns in os.walk(CH):
    for fn in fns:
        if not fn.endswith(".md"):
            continue
        rel = os.path.relpath(os.path.join(dirpath, fn), CH).replace("\\", "/")
        if dom(rel) != "??":
            continue
        top = rel.split("/")[0] if "/" in rel else "(top)"
        c[top] += 1
print("?? files by top dir:", dict(c))

s = json.load(open(os.path.join(os.path.dirname(__file__), "O-sample-round2.json"), encoding="utf-8"))
c2 = Counter()
for r in s:
    if r["domain"] == "??":
        top = r["rel"].split("/")[0] if "/" in r["rel"] else "(top)"
        c2[top] += 1
print("?? sampled by top dir:", dict(c2))

# per-domain pools: what remains after round1 exclusion
r1 = json.load(open(os.path.join(os.path.dirname(__file__), "O-sample-42.json"), encoding="utf-8"))
excl = set()
for e in r1:
    excl.add((dom(e["rel"]), e["addr"]))
print("excl size", len(excl))

# show non-render ?? rows in sample
for r in s:
    if r["domain"] == "??" and not r["rel"].startswith("render/"):
        print("nonrender:", r["rel"], r["addr"])
