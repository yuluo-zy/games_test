import json, os

BASE = "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/B/crates/country-core/src"
OUT = "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/reconstruction/country-core-render/wp4/_analysis"

def summarize(rel):
    p = os.path.join(BASE, rel)
    dep = json.load(open(p + ".dependencies.json"))
    # shape?
    out_lines = []
    if isinstance(dep, dict):
        for k in dep.keys():
            out_lines.append(f"key: {k}")
        imports = dep.get("imports") or dep.get("edges") or []
        if isinstance(imports, list):
            out_lines.append(f"# imports entries: {len(imports)}")
            for imp in imports[:200]:
                if isinstance(imp, dict):
                    out_lines.append(json.dumps(imp, ensure_ascii=False)[:300])
        else:
            out_lines.append(str(imports)[:2000])
    return "\n".join(out_lines)

# First inspect shape of one dependencies.json
dep = json.load(open(os.path.join(BASE, "render/mesh.rs.dependencies.json")))
print("dependencies.json top keys:", list(dep.keys()) if isinstance(dep, dict) else type(dep))
