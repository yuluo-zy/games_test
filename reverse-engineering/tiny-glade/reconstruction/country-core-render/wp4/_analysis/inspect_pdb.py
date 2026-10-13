import json, os

BASE = "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src"
OUT = "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/reconstruction/country-core-render/wp4/_analysis"

files = ["render/mesh.rs","render/shader.rs","render/load_texture.rs","render/render_config.rs",
    "render/view_constants.rs","render/frame_constants.rs","render/directional_light.rs",
    "render/atmosphere.rs","render/fsr_plugin.rs","render/mod.rs","render/debug_info_buffers.rs",
    "render/adaptive_render_state.rs","asset_libraries/shader_library.rs","asset_libraries/ssbo_library.rs",
    "asset_libraries/mesh_library.rs","asset_libraries/texture_library.rs","asset_libraries/color_library.rs",
    "asset_libraries/mod.rs"]

out = []
for rel in files:
    p = os.path.join(BASE, rel + ".pdb.json")
    if not os.path.exists(p):
        out.append(f"### {rel}: NO pdb.json")
        continue
    d = json.load(open(p))
    out.append(f"### {rel}")
    for key in ["source_sha256","original_paths","modules"]:
        out.append(f"  {key}: {d.get(key)}")
    out.append("  keys: " + ", ".join(d.keys()))
    # procedures? look for "procedures"
    for k in d:
        if "procedure" in k.lower() or "function" in k.lower():
            v = d[k]
            out.append(f"  {k}: {len(v) if hasattr(v,'__len__') else v}")

with open(os.path.join(OUT, "pdb_summary.txt"), "w") as f:
    f.write("\n".join(out))
print("\n".join(out))
