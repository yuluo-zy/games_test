import os, re
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


cnt_fmt = Counter()  # (domain, fmt) -> n
cnt_by_dom = Counter()
for dirpath, _d, fns in os.walk(CH):
    for fn in fns:
        if not fn.endswith(".md"):
            continue
        rel = os.path.relpath(os.path.join(dirpath, fn), CH).replace("\\", "/")
        d = dom(rel)
        for line in open(os.path.join(dirpath, fn), encoding="utf-8", errors="replace"):
            if not line.startswith("|") or line.startswith("|---"):
                continue
            cells = [c.strip() for c in line.strip("|").split("|")]
            if len(cells) < 5 or not cells[0].startswith("`"):
                continue
            concl = cells[4]
            c = concl.lower()
            if c.startswith("o") or c.startswith("**o"):
                cnt_fmt[(d, "bold" if concl.startswith("**") else "plain")] += 1
                cnt_by_dom[d] += 1

print(f"{'domain':<16}{'plain O':>10}{'bold **O**':>12}{'total':>8}")
for d in sorted(cnt_by_dom):
    print(f"{d:<16}{cnt_fmt.get((d,'plain'),0):>10}{cnt_fmt.get((d,'bold'),0):>12}{cnt_by_dom[d]:>8}")
