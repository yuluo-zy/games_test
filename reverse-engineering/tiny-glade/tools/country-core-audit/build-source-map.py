#!/usr/bin/env python3
"""Generate a source map + chapter skeletons for country-core modules that the
2026-10-14 SOURCE_MAP.json did not cover (components/startup/utils/input/...).

Rule (same as the original map, see PROJECT_MEMORY 2026-10-14):
  - record form contains `compiled_procedure`  -> business (real VA)
  - record form is inline-only                 -> inline (no own VA)
  - name matches a hard-glue pattern           -> glue (derive / dependency shim)

Usage:
  python -I build-source-map.py                      # report only
  python -I build-source-map.py --write              # write map + chapter stubs
  python -I build-source-map.py --write --dirs components startup
"""
import argparse
import json
import re
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SKEL = ROOT / "skeleton" / "B" / "crates" / "country-core" / "src"
OUT_MAP = ROOT / "evidence" / "country-core-deep" / "SOURCE_MAP_EXPANSION.json"
OUT_CH = ROOT / "evidence" / "country-core-deep" / "chapters-expansion"

DEFAULT_DIRS = [
    "components", "startup", "utils", "input",
    "game_config.rs", "game_key_bindings.rs", "game_profile.rs",
]

# hard-glue name patterns (derive / std / serde / bevy plumbing only)
GLUE = re.compile(
    r"::(drop_in_place|fmt|eq|ne|hash|partial_cmp|cmp|clone|clone_from|"
    r"serialize|deserialize|deserialize_in_place|visit_|type_id|type_name|"
    r"as_any|as_any_mut|as_ref|as_mut|borrow|deref|index|into_iter|from_iter|"
    r"schedule_system|apply|queue|init_state|get_type_registration|"
    r"register_component_hooks|register_type|drop|default)$"
)

SYM = re.compile(r"^// 函数符号: (.+)$")
LINE = re.compile(r"^// 源码行: (\d+)$")
RVA = re.compile(r"^// 编译实例 RVAs?: (.+)$")
FORM = re.compile(r"^// 记录形式: (.+?)(?:;.*)?$")


def parse_stub(path: Path):
    rows, cur = [], None
    for raw in path.read_text(encoding="utf-8", errors="replace").splitlines():
        m = SYM.match(raw)
        if m:
            cur = {"name": m.group(1), "src_line": 0, "rvas": [], "form": ""}
            rows.append(cur)
            continue
        if cur is None:
            continue
        m = FORM.match(raw)
        if m and not cur["form"]:
            cur["form"] = m.group(1)
            continue
        m = LINE.match(raw)
        if m and not cur["src_line"]:
            cur["src_line"] = int(m.group(1))
            continue
        m = RVA.match(raw)
        if m and not cur["rvas"]:
            cur["rvas"] = [t.strip() for t in m.group(1).split(",") if t.strip()]
    return rows


def classify(r):
    if GLUE.search(r["name"]):
        return "glue"
    if "compiled_procedure" in r["form"]:
        return "business"
    if "inline" in r["form"]:
        return "inline"
    return "dep"


def collect(dirs):
    files = []
    for d in dirs:
        p = SKEL / d
        if p.is_file():
            files.append(p)
        elif p.is_dir():
            files.extend(sorted(p.rglob("*.rs")))
    return files


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--write", action="store_true")
    ap.add_argument("--dirs", nargs="*", default=None)
    args = ap.parse_args()

    dirs = args.dirs or DEFAULT_DIRS
    files = collect(dirs)

    known = set()
    orig = ROOT / "evidence" / "country-core-deep" / "SOURCE_MAP.json"
    if orig.exists():
        known = set(json.loads(orig.read_text(encoding="utf-8")).keys())

    smap, chapters = {}, {}
    stats = defaultdict(int)
    for f in files:
        rel = f.relative_to(SKEL).as_posix()
        rows = parse_stub(f)
        funcs = []
        for r in rows:
            k = classify(r)
            stats[k] += 1
            va_full = [hex(0x140000000 + int(v, 16)) for v in r["rvas"]]
            funcs.append({
                "name": r["name"],
                "kind": k,
                "va": va_full,
                "rva": r["rvas"],
                "line": r["src_line"],
                "form": r["form"],
            })
        smap[rel] = {"file": rel, "functions": funcs}
        chapters[rel] = funcs

    nb = sum(1 for v in smap.values() for f in v["functions"] if f["kind"] == "business")
    nva = sum(1 for v in smap.values() for f in v["functions"] if f["kind"] == "business" and f["va"])
    print(f"files={len(files)}  business={nb} (withVA={nva})  "
          + " ".join(f"{k}={v}" for k, v in sorted(stats.items())))
    missing = [r for r in smap if r not in known]
    print(f"files already in original SOURCE_MAP: {len(smap)-len(missing)}/{len(smap)}; new: {len(missing)}")
    for r in sorted(missing):
        n = sum(1 for f in smap[r]["functions"] if f["kind"] == "business")
        print(f"  NEW {r}  business={n}")

    if not args.write:
        return

    OUT_MAP.write_text(json.dumps(smap, ensure_ascii=False, indent=1), encoding="utf-8")
    print(f"wrote {OUT_MAP}")

    for rel, funcs in sorted(chapters.items()):
        out = OUT_CH / (rel.replace("/", "__") + ".md")
        out.parent.mkdir(parents=True, exist_ok=True)
        biz = [f for f in funcs if f["kind"] == "business"]
        inl = [f for f in funcs if f["kind"] == "inline"]
        glue = [f for f in funcs if f["kind"] == "glue"]
        body = [f"# {rel}", "", "> 文件作用：**<待填>**", "",
                f"函数总计 {len(funcs)}：业务 {len(biz)} / 内联无独立符号 {len(inl)} / 派生胶水 {len(glue)}", "",
                "## 业务函数", "",
                "| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |",
                "|---|---|---|---|---|---|"]
        for f in sorted(biz, key=lambda x: (x["line"], x["name"])):
            va = f["va"][0] if f["va"] else "—"
            nm = f["name"].split("country_core::")[-1]
            body.append(f"| `{nm}` | {va} | {f['line']} | <待填> | U | — |")
        body += ["", "## 内联函数名称推断", ""]
        for f in sorted(inl, key=lambda x: (x["line"], x["name"])):
            body.append(f"- `{f['name'].split('country_core::')[-1]}` (L{f['line']}, {f['form']})")
        body += ["", "## 派生胶水分组", ""]
        for f in sorted(glue, key=lambda x: (x["line"], x["name"])):
            body.append(f"- `{f['name'].split('country_core::')[-1]}` (L{f['line']})")
        out.write_text("\n".join(body) + "\n", encoding="utf-8")
    print(f"wrote {len(chapters)} chapter stubs -> {OUT_CH}")


if __name__ == "__main__":
    main()
