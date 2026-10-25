import json, os, re

ANALYSIS = "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/reconstruction/country-core-render/wp4/_analysis"
DECOMP = os.path.join(ANALYSIS, "decomp")
os.makedirs(DECOMP, exist_ok=True)

def extract(name):
    d = json.load(open(os.path.join(ANALYSIS, f"result_{name}.json")))
    for step in d.get("results", []):
        if step.get("tool") != "get_functions":
            continue
        content = step["result"]["content"]
        for block in content:
            text = block["text"]
            try:
                obj = json.loads(text)
            except Exception:
                continue
            for addr, fn in obj.get("functions", {}).items():
                code = fn.get("decompiled_code", "")
                if not code:
                    continue
                sig = fn.get("signature", "")
                out = os.path.join(DECOMP, f"{name}_{addr}.c")
                with open(out, "w") as f:
                    f.write(f"// ADDRESS: {addr}\n// SIZE: {fn.get('size')}\n// SIGNATURE: {sig}\n")
                    f.write(code)
                print(f"{name} {addr}: {fn.get('name')} size={fn.get('size')} -> {os.path.basename(out)}")

for n in ["A"]:
    extract(n)
