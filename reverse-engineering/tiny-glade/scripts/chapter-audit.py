"""Chapter-audit runner: per-domain conclusion quality + cross-domain conflict scan.

Usage: python scripts/chapter-audit.py
Reads evidence/country-core-deep/chapters/**; writes
evidence/country-core-deep/round3/AUDIT-<ts>.json and prints summary.
"""
import json, os, re, time
from collections import defaultdict

ROOT = r"D:/game/reverse-engineering/tiny-glade"
CH = os.path.join(ROOT, "evidence/country-core-deep/chapters")
OUT = os.path.join(ROOT, "evidence/country-core-deep/round3")
os.makedirs(OUT, exist_ok=True)

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

def domain_of(f):
    f = f.replace("\\", "/")
    for d, pats in DOMAINS.items():
        if any(p in f for p in pats):
            return d
    return "??"

stats = defaultdict(lambda: [0, 0, 0, 0, 0])  # O I U empty other
rows = []
for dirpath, _, fns in os.walk(CH):
    for fn in fns:
        if not fn.endswith(".md"):
            continue
        p = os.path.join(dirpath, fn)
        rel = os.path.relpath(p, CH)[:-3].replace("\\", "/")
        d = domain_of(rel)
        txt = open(p, encoding="utf-8", errors="replace").read()
        for line in txt.splitlines():
            if not line.startswith("|") or line.startswith("|---"):
                continue
            cells = [c.strip() for c in line.strip("|").split("|")]
            if len(cells) < 5 or not cells[0].startswith("`"):
                continue
            concl = cells[4] if len(cells) > 4 else ""
            cl = concl.lower()
            addr = cells[1] if len(cells) > 1 else ""
            if concl == "" or "待证" in concl:
                stats[d][3] += 1
                rows.append({"kind": "empty", "domain": d, "file": rel,
                             "fn": cells[0][:110], "addr": addr})
            elif cl.startswith("o"):
                stats[d][0] += 1
            elif cl.startswith("i"):
                stats[d][1] += 1
                rows.append({"kind": "I", "domain": d, "file": rel,
                             "fn": cells[0][:110], "addr": addr})
            elif cl.startswith("u"):
                stats[d][2] += 1
                rows.append({"kind": "U", "domain": d, "file": rel,
                             "fn": cells[0][:110], "addr": addr})
            else:
                stats[d][4] += 1

print(f"{'domain':<16}{'O':>5}{'I':>5}{'U':>5}{'empty':>7}{'other':>7}")
for d in sorted(stats):
    a = stats[d]
    print(f"{d:<16}{a[0]:>5}{a[1]:>5}{a[2]:>5}{a[3]:>7}{a[4]:>7}")

ts = time.strftime("%Y%m%d-%H%M%S")
json.dump({"stats": {d: dict(zip(["O", "I", "U", "empty", "other"], stats[d])) for d in stats},
           "rows": rows},
          open(os.path.join(OUT, f"AUDIT-{ts}.json"), "w", encoding="utf-8"),
          ensure_ascii=False, indent=1)
print(f"\nrows: {len(rows)} (empty={sum(1 for r in rows if r['kind']=='empty')}, "
      f"I={sum(1 for r in rows if r['kind']=='I')}, U={sum(1 for r in rows if r['kind']=='U')})")
print("saved:", os.path.join(OUT, f"AUDIT-{ts}.json"))
