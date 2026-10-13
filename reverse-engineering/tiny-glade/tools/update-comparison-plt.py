import hashlib
import json
import os

base = "."
path = os.path.join(base, "tracking/shared-foundation-business-comparison.json")
with open(path) as f:
    d = json.load(f)

PLT = "evidence/foundation-plt-2026-10-13"
REVIEW_NOTE = "本轮原生复核通过（2026-10-13 " + PLT + "/），无新库内缺代码。"

for feat in d["features"]:
    fid = feat.get("feature_id")
    if fid == "lang-lib:03":
        feat["validation_gaps"] = [
            "判别式0x8000000000000001=Ok已跨load_locale_impl/find_standard_app_root_path交叉验证，现有is_err语义正确（非开放项）。",
            *feat.get("validation_gaps", []),
        ]
        feat["reference_corrections"] = feat.get("reference_corrections", []) + [REVIEW_NOTE]
        feat["evidence"] = feat.get("evidence", []) + [
            PLT + "/functions/0x140b9c4a0.txt",
            PLT + "/functions/0x140b9dc80.txt",
        ]
    elif fid == "lang-lib:06":
        feat["validation_gaps"] = [
            "fresh asm(0x140b9c300)证实原版写失败静默丢弃无log调用；候选保留诊断log为登记偏差（用户确认保留），代码未改。",
            *feat.get("validation_gaps", []),
        ]
        feat["reference_corrections"] = feat.get("reference_corrections", []) + [REVIEW_NOTE]
        feat["evidence"] = feat.get("evidence", []) + [
            PLT + "/functions/0x140b9c300.txt",
            "evidence/shared-foundation-2026-10-13/paths-language/lang-root.asm.txt",
        ]
    elif fid == "tabloid:06":
        feat["implemented_scope"] = [
            "库内契约已闭合：tests/dirty_tiles.rs 以DirtyTiles语义（512B位网格 zero-empty+OR-apply，镜像0x140960ac0/0x140960ef0）验证 publish→订阅→take 全流程；真实类型实现仍在 country-core 库外。"
        ]
        feat["validation_gaps"] = [
            "fresh反编译修正通道归属：DirtyTiles publish@0x141cf6500/0x1425db920/0x141453350、subscribe@0x141247b10/0x141cf68e0/0x141292860/0x141454290、take@0x141247c90/0x1412929e0/0x141454590；ColliderDiff publish<ColliderDiff,ColliderDelta>@0x141453c00、subscribe@0x141454410/0x1412926e0。原合同5地址全标ColliderDiff族有误。"
        ]
        feat["source_mapping"] = feat.get("source_mapping", []) + [
            {"function": "tests/dirty_tiles.rs", "file": "reconstruction/shared-foundation/crates/tabloid/tests/dirty_tiles.rs"}
        ]
        feat["reference_corrections"] = feat.get("reference_corrections", []) + [REVIEW_NOTE]
        feat["evidence"] = feat.get("evidence", []) + [
            "reconstruction/shared-foundation/crates/tabloid/tests/dirty_tiles.rs",
            PLT + "/functions/0x141cf6500.txt",
            PLT + "/functions/0x141247c90.txt",
            PLT + "/functions/0x140960ac0.txt",
            PLT + "/functions/0x140960ef0.txt",
        ]
    elif fid.startswith("paths:"):
        feat["reference_corrections"] = feat.get("reference_corrections", []) + [REVIEW_NOTE]
    elif fid.startswith("lang-lib:"):
        feat["reference_corrections"] = feat.get("reference_corrections", []) + [REVIEW_NOTE]
    elif fid.startswith("tabloid:") and fid != "tabloid:06":
        feat["reference_corrections"] = feat.get("reference_corrections", []) + [REVIEW_NOTE]

# Recompute current source SHAs for the modified files; add the new test file.
def sha(path):
    with open(os.path.join(base, path), "rb") as f:
        return hashlib.sha256(f.read()).hexdigest()

for key in list(d["current_sources"].keys()):
    p = os.path.join(base, key)
    if os.path.exists(p):
        d["current_sources"][key] = sha(key)
d["current_sources"]["reconstruction/shared-foundation/crates/tabloid/tests/dirty_tiles.rs"] = sha(
    "reconstruction/shared-foundation/crates/tabloid/tests/dirty_tiles.rs"
)

d["source_modified"] = True
d["tests_rerun"] = True

with open(path, "w") as f:
    json.dump(d, f, ensure_ascii=False, indent=2)

print("updated; source_modified=", d["source_modified"], "tests_rerun=", d["tests_rerun"])
print("tabloid:06 scope:", d["features"][[x["feature_id"] for x in d["features"]].index("tabloid:06")]["implemented_scope"])
