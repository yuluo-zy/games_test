import gzip, json
from pathlib import Path

root = Path("/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src/render/pass_builder")
mods = ["compute_pass_builder","pass_resource_access","draw_builder","bind_resources","pass_resources","uniform_binder","framebuffer_binding","draw_list","draw_stream","shader_uri"]
for m in mods:
    p = root / f"{m}.rs.enriched.json.gz"
    print("="*100)
    print(f"### {m}")
    try:
        with gzip.open(p, "rt") as f:
            data = json.load(f)
    except Exception as e:
        print("ERR", e); continue
    def walk(x, depth=0):
        if depth > 6: return
        if isinstance(x, dict):
            for k,v in x.items():
                if isinstance(v,(dict,list)):
                    print("  "*depth + f"{k}:")
                    walk(v, depth+1)
                else:
                    s = str(v)
                    print("  "*depth + f"{k}: {s[:300]}")
        elif isinstance(x, list):
            print("  "*depth + f"[list len={len(x)}]")
            for i,v in enumerate(x[:8]):
                walk(v, depth+1)
    walk(data)
