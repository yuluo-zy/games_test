import hashlib
import json
import os

path = "tracking/shared-foundation-business-comparison.json"
with open(path) as f:
    d = json.load(f)


def sha(p):
    with open(p, "rb") as f:
        return hashlib.sha256(f.read()).hexdigest()


changed = []
for key in d["current_sources"]:
    if os.path.exists(key):
        new = sha(key)
        if d["current_sources"][key] != new:
            changed.append(key)
        d["current_sources"][key] = new
    else:
        print("missing on disk (left as recorded):", key)

test = "reconstruction/shared-foundation/crates/tabloid/tests/dirty_tiles.rs"
d["current_sources"][test] = sha(test)

with open(path, "w") as f:
    json.dump(d, f, ensure_ascii=False, indent=2)

print("SHA updated for:", changed, "+", test)
