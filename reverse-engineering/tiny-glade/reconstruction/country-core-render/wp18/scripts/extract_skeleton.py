#!/usr/bin/env python3
"""Extract essential facts from draw_shaders skeleton/B files (read-only)."""
import json, gzip, re, sys
from pathlib import Path

DIR = Path("/Users/yuluo/Desktop/game/reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src/systems/render_loop/draw_passes/draw_shaders")

mods = [
    "billboard_forest.rs", "clearing_shaders.rs", "clouds.rs", "common_includes.rs",
    "draw_bricks.rs", "floor_shader.rs", "grass_shader.rs", "instanced_critters.rs",
    "instanced_ivy.rs", "leaf_particle_draw.rs", "path_pebble_shader.rs",
    "rocky_terrain_draw.rs", "stairs_draw.rs", "stars.rs",
]

out = {}
for m in mods:
    rs = (DIR / m).read_text(errors="replace")
    # function symbol + source line + rva
    funcs = []
    for mm in re.finditer(r"函数符号: (country_core[^\n]+)", rs):
        funcs.append(mm.group(1).split("::")[-1])
    lines = [int(x) for x in re.findall(r"源码行: (\d+)", rs)]
    rvas = [x for x in re.findall(r"编译实例 RVAs: (0x[0-9a-f]+)", rs)]
    # compile unit module ids
    _cus = re.findall(r"编译单元: ([0-9, ]+)", rs)
    cus = [int(x) for x in _cus[0].split(",")] if _cus else []
    # namespaces
    ns = re.findall(r"use (\w+); // namespace candidate", rs)
    # dependency annotations: lines mentioning definition_source under /home/h3 (game sources)
    game_calls = sorted(set(
        re.findall(r"country_core::([a-z0-9_:$]+|render::pass_builder::draw_builder::impl\$2::\w+)", rs)
    ))
    draw_builder_methods = sorted(set(
        re.findall(r"draw_builder::impl\$2::(\w+)", rs)
    ))
    rhapsody = sorted(set(re.findall(r"rhapsody::([a-z0-9_:$]+)", rs)))

    # enriched json
    gz = DIR / (m + ".enriched.json.gz")
    candidates = []
    system_interfaces = []
    if gz.exists():
        d = json.loads(gzip.open(gz).read().decode())
        for c in d.get("file_interface_candidates", []):
            sig = c.get("function_type_spelling", "")
            params = c.get("parameter_type_spellings", [])
            sis = c.get("system_interfaces", [])
            candidates.append({"signature": sig, "params": params})
            system_interfaces.append([(s.get("kind"), s.get("type")) for s in sis])

    out[m] = {
        "functions": funcs,
        "source_lines": lines,
        "rvas": rvas,
        "compile_units": cus,
        "namespaces": sorted(set(ns)),
        "game_calls": game_calls,
        "draw_builder_methods": draw_builder_methods,
        "rhapsody": rhapsody,
        "file_candidates": candidates,
        "system_interfaces": system_interfaces,
    }

sys.stdout.write(json.dumps(out, ensure_ascii=False, indent=1))
