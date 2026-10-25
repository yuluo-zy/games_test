import json, os

BASE = "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/render-architecture"
OUT = "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/reconstruction/country-core-render/wp4/_analysis"

d = json.load(open(os.path.join(BASE, "public-symbols.json")))
print("total entries", len(d))

patterns = [
    "asset_libraries::shader_library::", "asset_libraries::ssbo_library::",
    "asset_libraries::mesh_library::", "asset_libraries::texture_library::",
    "asset_libraries::color_library::", "asset_libraries::vao_library::",
    "asset_libraries::mod::", "asset_libraries::meshwatch::",
    "render::mod::AvailableGraphicsDevices", "render::GraphicsDeviceResource",
    "render::shaderwatch::", "render::debug_info_buffers::",
    "render::adaptive_render_state::", "render::fsr_plugin::",
    "render::view_constants::", "render::frame_constants::",
    "render::directional_light::", "render::atmosphere::",
    "render::render_config::", "render::load_texture::",
    "render::mesh::", "render::shader::",
]

lines = []
for e in d:
    name = e.get("demangled") or ""
    if any(p in name for p in patterns):
        va = e.get("va")
        if va:
            lines.append(f"{va}  {name}")

with open(os.path.join(OUT, "wp4_public_symbols.txt"), "w") as f:
    f.write("\n".join(sorted(set(lines))))
print(f"{len(set(lines))} lines written")
