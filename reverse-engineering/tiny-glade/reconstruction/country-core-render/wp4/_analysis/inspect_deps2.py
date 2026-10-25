import json, os

BASE = "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src"
OUT = "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/reconstruction/country-core-render/wp4/_analysis"

dep = json.load(open(os.path.join(BASE, "render/mesh.rs.dependencies.json")))
print("--- imports sample ---")
imps = dep.get("imports")
print(type(imps), len(imps) if hasattr(imps, "__len__") else "?")
if isinstance(imps, list) and imps:
    print(json.dumps(imps[0], indent=1)[:1500])
print("--- functions sample ---")
funcs = dep.get("functions")
print(type(funcs), len(funcs) if hasattr(funcs, "__len__") else "?")
if isinstance(funcs, list) and funcs:
    print(json.dumps(funcs[0], indent=1)[:1500])
