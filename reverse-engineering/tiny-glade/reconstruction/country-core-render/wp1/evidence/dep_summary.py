import json, sys
from pathlib import Path
root = Path("/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src/render/pass_builder")
mods = sys.argv[1:] or ["compute_pass_builder","pass_resource_access","draw_builder","bind_resources","pass_resources","uniform_binder","framebuffer_binding","draw_list","draw_stream","shader_uri"]
for m in mods:
    p = root / f"{m}.rs.dependencies.json"
    print("="*70); print(m)
    data = json.loads(p.read_text())
    # find structure
    def find_imports(x, path=""):
        if isinstance(x, dict):
            for k,v in x.items():
                if "import" in k.lower() or "own" in k.lower() or "source" in k.lower():
                    pass
                find_imports(v, path+"/"+k)
        elif isinstance(x, list):
            for i,v in enumerate(x):
                find_imports(v, path+f"[{i}]")
    # top-level keys
    print("  top keys:", list(data.keys())[:10] if isinstance(data,dict) else type(data))
    # try to find imports list
    if isinstance(data, dict):
        imp = data.get("imports") or data.get("external_imports") or data.get("calls")
        if isinstance(imp, dict):
            print("  imports keys:", list(imp.keys())[:15])
        elif isinstance(imp, list):
            print("  imports len:", len(imp))
