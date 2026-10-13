import gzip, json, sys
from pathlib import Path

root = Path("/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src/render/pass_builder")
mods = ["compute_pass_builder","pass_resource_access","draw_builder","bind_resources","pass_resources","uniform_binder","framebuffer_binding","draw_list","draw_stream","shader_uri"]
for m in mods:
    p = root / f"{m}.rs.structures.json.gz"
    print("="*80)
    print(f"### {m}")
    try:
        with gzip.open(p, "rt") as f:
            data = json.load(f)
    except Exception as e:
        print("ERR", e); continue
    if isinstance(data, dict):
        ent = data.get("named_symbol_entities") or data.get("entities") or data
    else:
        ent = data
    if isinstance(ent, dict):
        items = ent.get("named_symbol_entities")
        if items is None:
            # maybe direct dict of entities
            items = ent
        if isinstance(items, dict):
            for k,v in items.items():
                print(f"  {k}: {json.dumps(v)[:600]}")
        elif isinstance(items, list):
            for v in items:
                print(f"  {json.dumps(v)[:600]}")
        else:
            print("  raw:", json.dumps(ent)[:1200])
    else:
        print("  list:", json.dumps(ent)[:1200])
