#!/usr/bin/env python
"""Extract O-conclusion rows from country-core-deep chapters, grouped by domain.

Chapter table rows have 6 columns:
  | fn | addr | line | 作用(guess or claim) | 逆向结论 | 证据 |

Two conventions:
  A: 逆向结论 col == "O：<claim>"        -> claim = 逆向结论 col
  B: 逆向结论 col == "O" (bare)          -> claim = 作用 col (already the finding)

Emits TSV per domain: relpath, fn, addr, claim, evid, row_format
"""
import io, os, re, sys

ROOT = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/chapters"

DOMAIN_KEYS = {
    "collision-eco": [
        "resources/critters", "clutter_meshes", "preplaced_autoclutter",
        "resources/ivy", "systems/ivy", "resources/stairs", "stairs_walls",
        "systems/collision", "systems/paths", "ui_clutter", "decorator_collision",
    ],
    "ui-input": [
        "input/", "systems/backend/input", "mode_manager", "resources/mouse_capture",
        "resources/toolbar", "resources/ui_settings", "resources/photo_mode",
        "resources/onboarding", "resources/gui", "systems/ui/", "systems/ui_",
        "ui_2d_follow_mouse", "ui_camera_mode_gizmos",
    ],
    "audio-misc": [
        "resources/audio", "systems/audio", "utils/audio_utils", "steam",
        "resources/shutdown", "feedback_report", "gpu_lut", "grading_lut",
        "color_icon", "color_id", "gothic_window_profile", "glade_color_customization",
        "image_formation", "placement_data", "texture_presets", "render_ssbo",
        "shader_refactoring_state", "editing_highlight", "app_loading_state",
        "frozen_map", "meta_commander", "name_hash",
    ],
}

ADDR_RE = re.compile(r"^0x[0-9a-fA-F]+$")

GLUE_MARKERS = ["serde derive", "register_required_components", "ICF", "Bevy 组件注册",
                "Bevy注册", "复用依赖", "type_id", "schedule_system", "as_any",
                "drop_in_place", "component derive", "reflect", "FromWorld",
                "event derive", "derive（复用", "Bevy 系统注册", "impl$", "bevy_ecs 注册"]


def domain_of(relpath):
    p = "/" + relpath.replace("\\", "/").replace(".md", "")
    for dom, keys in DOMAIN_KEYS.items():
        for k in keys:
            if k in p:
                return dom
    return None


def main():
    rows = {d: [] for d in DOMAIN_KEYS}
    for dirpath, _dirs, files in os.walk(ROOT):
        for fn in files:
            if not fn.endswith(".md"):
                continue
            full = os.path.join(dirpath, fn)
            rel = os.path.relpath(full, ROOT)
            dom = domain_of(rel)
            if not dom:
                continue
            with io.open(full, encoding="utf-8", errors="replace") as f:
                for ln, line in enumerate(f, 1):
                    parts = [p.strip() for p in line.strip().split("|")]
                    # 6 cols: ['', fn, addr, line, guess, concl, evid, '']
                    if len(parts) < 7:
                        continue
                    fnname, addr = parts[1], parts[2]
                    if not ADDR_RE.match(addr):
                        continue
                    guess, concl, evid = parts[4], parts[5], parts[6]
                    if not concl.startswith("O"):
                        continue
                    # strip trailing evidence-path only in A format
                    if concl[:1] == "O" and (concl.startswith("O：") or concl.startswith("O:")):
                        claim = concl
                        fmt = "A"
                    else:  # bare O
                        claim = guess
                        fmt = "B"
                    if any(g in claim for g in GLUE_MARKERS):
                        continue
                    rows[dom].append((rel, fnname, addr, claim.replace("\t", " "), evid, fmt))
    outdir = os.path.dirname(os.path.abspath(__file__))
    for dom in DOMAIN_KEYS:
        outp = os.path.join(outdir, "claims-" + dom + ".tsv")
        with io.open(outp, "w", encoding="utf-8") as f:
            for r in sorted(rows[dom], key=lambda r: int(r[2], 16)):
                f.write("\t".join(r) + "\n")
        sys.stderr.write(f"{dom}: {len(rows[dom])} rows -> {outp}\n")


if __name__ == "__main__":
    main()
