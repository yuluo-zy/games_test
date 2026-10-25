#!/usr/bin/env python3
"""从 skeleton/B 的 components 骨架文件提取业务过程 (VA + 源码行)。

用法:
  python -I components_scope.py --file mod.rs
  python -I components_scope.py --file drawable.rs --json out.json
数据源: skeleton/B/crates/country-core/src/components/<file>
"""
import argparse
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SKEL = ROOT / "skeleton" / "B" / "crates" / "country-core" / "src" / "components"

# 硬胶水（与 MODULE_GAP_ANALYSIS / AGENTS.md 口径一致）
GLUE = re.compile(
    r"::(fmt|eq|hash|partial_cmp|cmp|clone|clone_from|serialize|deserialize|"
    r"type_id|as_any|as_any_mut|visit_|schedule_system|apply|queue|"
    r"init_state|get_type_registration|register_component_hooks|drop)$"
)
SYM = re.compile(r"^// 函数符号: (.+)$")
LINE = re.compile(r"^// 源码行: (\d+)$")
RVA = re.compile(r"^// 编译实例 RVAs?: (.+)$")


def parse(path):
    rows, cur = [], None
    for raw in path.read_text(encoding="utf-8", errors="replace").splitlines():
        m = SYM.match(raw)
        if m:
            cur = {"name": m.group(1), "src_line": None, "rvas": []}
            rows.append(cur)
            continue
        if cur is None:
            continue
        m = LINE.match(raw)
        if m and cur["src_line"] is None:
            cur["src_line"] = int(m.group(1))
            continue
        m = RVA.match(raw)
        if m and not cur["rvas"]:
            cur["rvas"] = [t.strip() for t in m.group(1).split(",") if t.strip()]
    return rows


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--file", default="mod.rs")
    ap.add_argument("--json", default=None)
    ap.add_argument("--glue", action="store_true", help="保留胶水符号")
    args = ap.parse_args()

    rows = parse(SKEL / args.file)
    biz = [r for r in rows if args.glue or not GLUE.search(r["name"])]
    withva = sum(1 for r in biz if r["rvas"])
    print(f"# {args.file}: total {len(rows)} / listed {len(biz)} / withVA {withva}")
    for r in biz:
        va = r["rvas"][0].replace("0x", "") if r["rvas"] else "-"
        print(f"{va:>10}  L{str(r['src_line'] or 0):<5} {r['name'].split('country_core::')[-1]}")

    if args.json:
        Path(args.json).write_text(json.dumps(biz, ensure_ascii=False, indent=1), encoding="utf-8")


if __name__ == "__main__":
    main()
