import gzip, json, os

BASE = "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src"
OUT = "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/reconstruction/country-core-render/wp4/_analysis"

def functions(rel):
    p = os.path.join(BASE, rel + ".enriched.json.gz")
    if not os.path.exists(p):
        return []
    d = json.loads(gzip.open(p, "rt").read())
    out = []
    for f in d.get("functions", []):
        name = f.get("name")
        # compiled instances / host addresses
        insts = f.get("compiled_instances", []) or []
        addrs = []
        for inst in insts:
            addrs.append(inst.get("address") or inst.get("rva") or inst.get("host_rva"))
        hosts = f.get("inline_hosts", []) or []
        out.append({"name": name, "type_index": f.get("type_index"),
                    "compiled": len(insts), "instances": insts[:3], "inline_hosts": len(hosts)})
    return out

files = ["render/render_config.rs","render/mod.rs","render/shader.rs","render/view_constants.rs",
    "render/frame_constants.rs","render/directional_light.rs","render/atmosphere.rs",
    "asset_libraries/shader_library.rs","asset_libraries/ssbo_library.rs",
    "asset_libraries/mesh_library.rs","asset_libraries/texture_library.rs",
    "asset_libraries/color_library.rs","asset_libraries/mod.rs"]

with open(os.path.join(OUT, "enriched_functions.txt"), "w") as f:
    for rel in files:
        funcs = functions(rel)
        f.write(f"### {rel}: {len(funcs)} functions\n")
        for fn in funcs:
            f.write(f"  {fn['name']} | type={fn['type_index']} compiled={fn['compiled']} inline_hosts={fn['inline_hosts']}\n")
            for inst in fn['instances']:
                f.write(f"      instance: {json.dumps(inst)[:180]}\n")
print("done")
