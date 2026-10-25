import json, os

BASE = "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/render-architecture"
OUT = "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/reconstruction/country-core-render/wp4/_analysis"

seeds = json.load(open(os.path.join(BASE, "symbol-seeds.json")))

patterns = [
    "render::mesh::", "render::shader::", "render::render_config::", "render::view_constants::",
    "render::frame_constants::", "render::directional_light::", "render::atmosphere::",
    "render::fsr_plugin::", "render::load_texture::", "render::debug_info_buffers::",
    "render::adaptive_render_state::", "asset_libraries::shader_library::",
    "asset_libraries::ssbo_library::", "asset_libraries::mesh_library::",
    "asset_libraries::texture_library::", "asset_libraries::color_library::",
    "render::mod::AvailableGraphicsDevices", "render::GraphicsDeviceResource",
    "render::shaderwatch::", "render::ShaderWatch",
]

matches = []
for e in seeds:
    name = e.get("demangled", "")
    if any(p in name for p in patterns):
        matches.append(e)

out_lines = []
for e in sorted(matches, key=lambda x: x.get("va", "")):
    out_lines.append(f"{e.get('va','?'):16} {e.get('demangled','')}")

with open(os.path.join(OUT, "wp4_seed_addresses.txt"), "w") as f:
    f.write("\n".join(out_lines))
print(f"{len(matches)} matches written")
print("\n".join(out_lines[:60]))
