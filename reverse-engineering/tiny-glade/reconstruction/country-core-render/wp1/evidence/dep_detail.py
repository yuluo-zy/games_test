import json, sys
from pathlib import Path
root = Path("/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src/render/pass_builder")
mods = sys.argv[1:] or ["compute_pass_builder","draw_builder","pass_resource_access","shader_uri","bind_resources","uniform_binder","draw_stream","pass_resources","framebuffer_binding","draw_list"]
for m in mods:
    p = root / f"{m}.rs.dependencies.json"
    print("="*80); print(m)
    data = json.loads(p.read_text())
    for imp in data.get("imports", []):
        if isinstance(imp, dict):
            src = imp.get("source") or imp.get("origin") or imp.get("ownership") or "?"
            name = imp.get("import") or imp.get("path") or imp.get("symbol") or json.dumps(imp)[:120]
            print(f"  - {name}  [{src}]")
        else:
            print("  -", json.dumps(imp)[:160])
