#!/usr/bin/env python3
"""Round-2 O-conclusion sampler for country-core-deep chapters (v2).

Pools use BOTH conclusion conventions found in the corpus:
  A) cells[4] starts with "O"  (claim = cells[4])
  B) cells[4] is bold "**O**"  (claim = cells[3] 作用 column; common in startup/
     components/utils/geometry/top-level files, undercounted by chapter-audit.py)
Fixed seed 20261015, uniform per-domain sampling, excluding round-1 addresses
(O-sample-42.json). Adds a documented 10-row stratified supplement from
non-render ?? subdomains (components/startup/utils/geometry/top-level).
"""
import json, os, random, re

ROOT = r"D:/game/reverse-engineering/tiny-glade"
CH = os.path.join(ROOT, "evidence/country-core-deep/chapters")
OUT = os.path.join(ROOT, "evidence/country-core-deep/round3")

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

TARGET = {
    "??": 60, "wall": 30, "collision-eco": 15, "ui-input": 15,
    "terrain-water": 15, "history": 15, "render-loop": 15,
    "session-save": 10, "roofs-rt": 10, "camera-time": 10,
    "audio-misc": 5, "backend-misc": 5,
}


def domain_of(f):
    f = f.replace("\\", "/")
    for d, pats in DOMAINS.items():
        if any(p in f for p in pats):
            return d
    return "??"


def extract_rows():
    rows = {d: [] for d in list(DOMAINS) + ["??"]}
    for dirpath, _dirs, fns in os.walk(CH):
        for fn in fns:
            if not fn.endswith(".md"):
                continue
            p = os.path.join(dirpath, fn)
            rel = os.path.relpath(p, CH)[:-3].replace("\\", "/")
            d = domain_of(rel)
            with open(p, encoding="utf-8", errors="replace") as f:
                for line in f:
                    if not line.startswith("|") or line.startswith("|---"):
                        continue
                    cells = [c.strip() for c in line.strip("|").split("|")]
                    if len(cells) < 6 or not cells[0].startswith("`"):
                        continue
                    concl = cells[4]
                    if concl.startswith("**"):
                        if not re.match(r"^\*{2}O\*{2}", concl):
                            continue
                        claim = cells[3]
                        fmt = "B"
                    elif concl.startswith("O"):
                        claim = concl
                        fmt = "A"
                    else:
                        continue
                    addr = cells[1]
                    if not re.match(r"^0x[0-9a-fA-F]{8,16}$", addr):
                        continue
                    rows[d].append({
                        "domain": d, "rel": rel, "fn": cells[0], "addr": addr,
                        "claim": claim, "evid": cells[5], "fmt": fmt,
                    })
    return rows


def main():
    rows = extract_rows()
    for d in sorted(rows):
        print(f"{d:<16}{len(rows[d]):>5} O rows")

    r1 = json.load(open(os.path.join(OUT, "O-sample-42.json"), encoding="utf-8"))
    excl = set()
    for e in r1:
        excl.add((domain_of(e["rel"]), e["addr"]))
    print(f"\nexclusions (domain,addr): {len(excl)}")

    rng = random.Random(20261015)
    sample = []
    for d in sorted(TARGET):
        pool = [r for r in rows[d] if (d, r["addr"]) not in excl]
        n = TARGET[d]
        if len(pool) < n:
            print(f"WARN: {d} pool {len(pool)} < target {n}")
        chosen = rng.sample(pool, min(n, len(pool)))
        sample.extend(chosen)
        print(f"{d:<16}pool={len(pool):>4}  sampled={len(chosen):>3}")

    # stratified supplement: non-render ?? subdomains (untouched by round 1)
    supp_pool = [r for r in rows["??"]
                 if not r["rel"].startswith("render/") and ("??", r["addr"]) not in excl]
    supp = rng.sample(supp_pool, min(10, len(supp_pool)))
    for r in supp:
        r["supp"] = "non-render-top-level"
        sample.append(r)
    print(f"?? supplement (non-render top-level): {len(supp)}")

    sample.sort(key=lambda r: (r["domain"], r["addr"]))
    out = os.path.join(OUT, "O-sample-round2.json")
    json.dump(sample, open(out, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print(f"\ntotal: {len(sample)} -> {out}")


if __name__ == "__main__":
    main()
