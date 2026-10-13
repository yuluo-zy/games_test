import gzip, json, sys, os

BASE = "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src"
OUT = "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/reconstruction/country-core-render/wp4/_analysis/symbols"

os.makedirs(OUT, exist_ok=True)

def dump(rel):
    p = os.path.join(BASE, rel)
    if not os.path.exists(p + ".structures.json.gz"):
        print(f"MISSING {rel}")
        return
    d = json.loads(gzip.open(p + ".structures.json.gz", "rt").read())
    names = set()
    for e in d.get("named_symbol_entities", []):
        for occ in e.get("ipi_occurrences", []):
            s = occ.get("spelling", "")
            if s:
                names.add(s)
    unowned = []
    for t in d.get("unowned_type_records", []) or []:
        if isinstance(t, dict):
            unowned.append(t.get("spelling") or t.get("name") or str(t)[:80])
    out = OUT + "/" + rel.replace("/", "__") + ".txt"
    with open(out, "w") as f:
        for n in sorted(names):
            f.write(n + "\n")
        f.write(f"# unowned ({len(unowned)}):\n")
        for u in sorted(unowned):
            f.write(u + "\n")
    print(f"{rel}: {len(names)} named symbols -> {out}")

render_files = ["mesh.rs","shader.rs","load_texture.rs","render_config.rs","view_constants.rs",
    "frame_constants.rs","directional_light.rs","atmosphere.rs","fsr_plugin.rs","mod.rs",
    "debug_info_buffers.rs","adaptive_render_state.rs"]
for f in render_files:
    dump("render/" + f)
asset_files = ["shader_library.rs","ssbo_library.rs","mesh_library.rs","texture_library.rs","color_library.rs","mod.rs"]
for f in asset_files:
    dump("asset_libraries/" + f)
