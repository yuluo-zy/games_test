"""Export M06 lexical candidates that still have business meaning.

This is an auditable triage aid, not an original-equivalence verdict.  It
reuses audit_b_module_inventory.py, removes mechanical trait/serde/Bevy
implementations, and writes a review queue for the remaining M06 functions.
"""

from __future__ import annotations

import collections
import contextlib
import importlib.util
import io
import json
import re
import subprocess
import tempfile
from pathlib import Path


SCRIPT_DIR = Path(__file__).resolve().parent
ROOT = SCRIPT_DIR.parents[1]
OUT_DIR = ROOT / "tiny-glade/reports/business/2026-10-23-module-recovery"
STEM = "M06-BUSINESS-FUNCTIONS-2026-10-30"

MECHANICAL_LEAVES = re.compile(
    r"^(?:"
    r"deserialize|serialize|expecting|"
    r"visit_(?:str|map|seq|enum|bytes|u64|newtype_struct)|"
    r"default|clone|clone_from|fmt|eq|partial_cmp|cmp|hash|display|"
    r"apply|get_param|init_state|new_archetype|queue|validate_param|"
    r"register_required_components|clone_behavior|next|from"
    r")$"
)


def leaf_name(row: dict) -> str:
    return row["b_name"].rsplit("::", 1)[-1]


def is_mechanical(row: dict) -> bool:
    return bool(MECHANICAL_LEAVES.fullmatch(leaf_name(row)))


def business_category(row: dict) -> str:
    text = f"{row['b_path']}::{row['b_name']}".lower()
    if any(
        token in text
        for token in (
            "construct_arches",
            "arch_walker",
            "profile::archfunction",
            "archfunction",
        )
    ):
        return "拱形生成与拱曲线"
    if any(token in text for token in ("construct_lintels", "support_beam", "lintel")):
        return "过梁与承重梁"
    if any(
        token in text
        for token in (
            "elevation_support",
            "place_pillars",
            "pillar",
            "column",
            "bracket",
            "turret",
            "circle_bottom_layer",
            "rectangle_large_brackets",
            "rectangle_small_brackets",
        )
    ):
        return "柱、塔柱、托架与高差支撑"
    if any(
        token in text
        for token in (
            "wall_holes",
            "hole_",
            "holes",
            "construct_gates",
            "construct_fences",
            "door",
            "window",
            "dormer",
            "fence",
            "gate",
        )
    ):
        return "洞口、门、窗、栅栏与屋顶窗"
    if any(
        token in text
        for token in ("plaster", "halftimber", "half_timber", "curved_halftimber")
    ):
        return "抹灰、半木与曲面半木"
    if any(token in text for token in ("decorator", "deco", "cached_decorator")):
        return "墙体装饰物：ID、存储、变换、网格、碰撞与归档"
    if any(token in text for token in ("material_stats", "highlightcolor", "material", "color")):
        return "材料统计与颜色"
    if any(
        token in text
        for token in (
            "wall_spatial_hash",
            "wall_roof_hashmap",
            "wall_attachment",
            "wall_trigger_volume",
            "valid_enclosure",
            "public_wall_state",
            "inner_wall_state",
            "inner_walls",
            "public_walls",
            "wall_constructor",
            "segmentation",
            "snapping",
            "shape_placement",
            "draw_wall",
            "finish_wall",
            "continue_wall",
        )
    ):
        return "墙权威、空间索引、形状、分段、吸附与合并"
    if any(token in text for token in ("eraser", "bulldozer", "archive", "history")):
        return "擦除、删除归档与历史"
    if any(
        token in text
        for token in ("adjust_wall_height", "ui_", "signifier", "widget", "preview", "interaction")
    ):
        return "墙体编辑 UI、手柄、高度、移动与预览"
    if "floor" in text:
        return "楼板"
    return "其他构造辅助"


def candidate_text(row: dict) -> str:
    candidates = row.get("candidates", [])
    if not candidates:
        return "—"
    values = [f"`{item['path']}:{item['line']}`" for item in candidates[:6]]
    if len(candidates) > 6:
        values.append(f"另 {len(candidates) - 6} 项")
    return "<br>".join(values)


def main() -> None:
    spec = importlib.util.spec_from_file_location(
        "audit_b_module_inventory", SCRIPT_DIR / "audit_b_module_inventory.py"
    )
    if spec is None or spec.loader is None:
        raise RuntimeError("无法加载 audit_b_module_inventory.py")
    inventory = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(inventory)

    with tempfile.TemporaryDirectory(prefix="m06-business-functions-") as temp:
        inventory.OUT = Path(temp)
        with contextlib.redirect_stdout(io.StringIO()):
            inventory.main()
        inventory_data = json.loads(
            (inventory.OUT / "B-MODULE-INVENTORY.json").read_text(encoding="utf-8")
        )

    m06 = [row for row in inventory_data["functions"] if row["module"] == "M06"]
    excluded_m06 = [row for row in inventory_data["excluded_associations"] if row["module"] == "M06"]
    status_counts = collections.Counter(row["status"] for row in m06)
    mechanical_by_status = collections.Counter(
        row["status"] for row in m06 if is_mechanical(row)
    )
    business_by_status = collections.Counter(
        row["status"] for row in m06 if not is_mechanical(row)
    )

    pending_business = [
        row
        for row in m06
        if row["status"] in ("ambiguous_candidates", "unmatched_review")
        and not is_mechanical(row)
    ]
    for row in pending_business:
        row["business_category"] = business_category(row)
    pending_business.sort(
        key=lambda row: (
            row["business_category"],
            row["status"],
            row["b_path"],
            row["b_name"],
        )
    )

    placeholders = [
        row
        for row in m06
        if row["status"] in ("partial_stub", "explicit_stub", "empty_body_review")
    ]
    for row in placeholders:
        row["business_category"] = business_category(row)

    commit = subprocess.check_output(
        ["git", "-C", str(ROOT), "rev-parse", "--short=9", "HEAD"], text=True
    ).strip()
    category_counts = collections.Counter(
        row["business_category"] for row in pending_business
    )
    category_status_counts = collections.Counter(
        (row["business_category"], row["status"]) for row in pending_business
    )

    payload = {
        "audit": "M06 business-function lexical review queue",
        "date": "2026-10-30",
        "repo_commit": commit,
        "inventory_index_sha256": inventory_data["index_sha256"],
        "method": (
            "Generated by tools/audit_b_module_inventory.py plus deterministic "
            "business/mechanical name-and-path classification. Category labels "
            "and candidate presence are triage only; they are not evidence of "
            "original semantic equivalence, production wiring, or completeness."
        ),
        "summary": {
            "m06_groups": len(m06),
            "m06_groups_before_association_filter": len(m06) + len(excluded_m06),
            "excluded_associations": dict(sorted(collections.Counter(row['reason'] for row in excluded_m06).items())),
            "status_counts": dict(sorted(status_counts.items())),
            "mechanical_by_status": dict(sorted(mechanical_by_status.items())),
            "business_by_status": dict(sorted(business_by_status.items())),
            "business_groups": sum(business_by_status.values()),
            "business_unique_body_candidates": business_by_status["body_candidate"],
            "business_with_candidate": (
                business_by_status["body_candidate"]
                + business_by_status["ambiguous_candidates"]
            ),
            "pending_business_review": len(pending_business),
            "pending_business_by_status": dict(
                sorted(collections.Counter(row["status"] for row in pending_business).items())
            ),
            "pending_business_by_category": dict(sorted(category_counts.items())),
        },
        "pending_business_functions": pending_business,
        "explicit_placeholders": placeholders,
    }
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    (OUT_DIR / f"{STEM}.json").write_text(
        json.dumps(payload, ensure_ascii=False, indent=2) + "\n", encoding="utf-8"
    )

    business_total = sum(business_by_status.values())
    business_body = business_by_status["body_candidate"]
    business_candidate = business_body + business_by_status["ambiguous_candidates"]
    lines = [
        "# M06 待核业务函数清单（2026-10-30）",
        "",
        "本清单从最新源码的 M06 词法库存中提取仍具有业务含义、但尚未唯一映射到当前实现的函数组。"
        "它是人工语义审计的输入，不是原版算法等价、生产接线或完成率的声明。",
        "",
        f"- 源码提交：`{commit}`",
        f"- B 函数索引 SHA256：`{inventory_data['index_sha256']}`",
        "- 生成入口：[`tools/audit_m06_business_functions.py`](../../../tools/audit_m06_business_functions.py)",
        "- 基础扫描：[`tools/audit_b_module_inventory.py`](../../../tools/audit_b_module_inventory.py)",
        "- 机器可读结果：[`M06-BUSINESS-FUNCTIONS-2026-10-30.json`](M06-BUSINESS-FUNCTIONS-2026-10-30.json)",
        "",
        "## 口径",
        "",
        "库存的 `ambiguous_candidates` 表示同名实现不止一个；`unmatched_review` 表示没有同名实现，"
        "可能因为改名、跨 crate 迁移、泛型/宏展开，也可能真的缺失。两类都不能直接判为未实现。"
        "本清单排除 Rust trait、serde visitor、Bevy SystemParam 等机械名称，并清除仅凭 new/get/insert/remove 同名生成的关联。"
        "通用接口需另以类型 owner 和原证关联，不能据此判为无业务或已恢复。排除记录只留在基础 JSON 的 excluded_associations 中。",
        "",
        f"过滤前 M06 共 {len(m06) + len(excluded_m06)} 组，已清除 {len(excluded_m06)} 组名称关联："
        + "、".join(f"{reason}={count}" for reason,count in sorted(collections.Counter(row['reason'] for row in excluded_m06).items())) + "。",
        "",
        "| 指标 | 数量 | 比例 |",
        "|---|---:|---:|",
        f"| M06 全部函数组 | {len(m06)} | 100.0% |",
        f"| 唯一非空体候选 | {status_counts['body_candidate']} | "
        f"{status_counts['body_candidate'] / len(m06):.1%} |",
        f"| 多候选 | {status_counts['ambiguous_candidates']} | "
        f"{status_counts['ambiguous_candidates'] / len(m06):.1%} |",
        f"| 未匹配 | {status_counts['unmatched_review']} | "
        f"{status_counts['unmatched_review'] / len(m06):.1%} |",
        f"| 空体/部分占位 | "
        f"{status_counts['empty_body_review'] + status_counts['partial_stub'] + status_counts['explicit_stub']} | "
        f"{(status_counts['empty_body_review'] + status_counts['partial_stub'] + status_counts['explicit_stub']) / len(m06):.1%} |",
        "",
        "排除机械实现后：",
        "",
        "| 指标 | 数量 | 比例 |",
        "|---|---:|---:|",
        f"| 业务函数组 | {business_total} | 100.0% |",
        f"| 唯一非空体候选 | {business_body} | {business_body / business_total:.1%} |",
        f"| 唯一或同名候选 | {business_candidate} | {business_candidate / business_total:.1%} |",
        f"| 待人工核对的业务函数 | {len(pending_business)} | "
        f"{len(pending_business) / business_total:.1%} |",
        "",
        "## 待核业务函数分类",
        "",
        "| 功能域 | 多候选 | 未匹配 | 合计 |",
        "|---|---:|---:|---:|",
    ]
    for category, count in sorted(
        category_counts.items(), key=lambda item: (-item[1], item[0])
    ):
        ambiguous = category_status_counts[(category, "ambiguous_candidates")]
        unmatched = category_status_counts[(category, "unmatched_review")]
        lines.append(f"| {category} | {ambiguous} | {unmatched} | {count} |")

    lines += [
        "",
        "## 明细",
        "",
        "“候选源码”仅供定位；多候选中的硬编码数量上限只影响本报告展示，JSON 保留全部候选。",
        "",
    ]
    current_category = None
    for row in pending_business:
        if row["business_category"] != current_category:
            current_category = row["business_category"]
            if lines[-1] != "":
                lines.append("")
            lines += [f"### {current_category}", ""]
        status = {
            "ambiguous_candidates": "多候选",
            "unmatched_review": "未匹配",
        }[row["status"]]
        lines += [
            f"- **{status}** `{row['b_name']}`",
            f"  - 原路径：`{row['b_path']}`",
            f"  - B ID：`{', '.join(row['b_ids'])}`",
            f"  - 候选源码：{candidate_text(row)}",
        ]

    lines += [
        "",
        "## 显式空体或部分占位",
        "",
        "| 函数 | 状态 | 原路径 | 处理要求 |",
        "|---|---|---|---|",
    ]
    placeholder_notes = {
        "adjust_wall_height_impl": "含显式未完成分支，必须对照原算法补全。",
        "DecoratorStorage::verify_integrity": "空体需原证确认是否原版无操作。",
        "debug_force_update_arches": "台账记录原版为 1B 返回；仍需保留原证再裁定，不能只按空体判缺失。",
    }
    for row in placeholders:
        note = next(
            (value for key, value in placeholder_notes.items() if key in row["b_name"]),
            "需原证复核。",
        )
        lines.append(
            f"| `{row['b_name']}` | `{row['status']}` | `{row['b_path']}` | {note} |"
        )

    lines += [
        "",
        "## 审计要求",
        "",
        "1. 逐项确认当前实现、原版函数、输入输出与失败路径，不凭同名或非空函数体判完成。",
        "2. 多候选必须按类型 owner、原路径、调用者和 ABI/schema 确定唯一映射。",
        "3. 未匹配先检查改名、跨 crate 迁移、trait/generic 单态化；确认缺失后再补算法。",
        "4. 算法恢复后仍须单独验证生产接线、历史、碰撞、GPU、保存恢复和卸载。",
        "5. 本清单的分类与数量不能替代调用链工程完成度。",
        "",
    ]
    (OUT_DIR / f"{STEM}.md").write_text("\n".join(lines), encoding="utf-8")

    print(
        json.dumps(
            {
                "markdown": str(OUT_DIR / f"{STEM}.md"),
                "json": str(OUT_DIR / f"{STEM}.json"),
                "m06_groups": len(m06),
                "business_groups": business_total,
                "pending_business": len(pending_business),
                "placeholders": len(placeholders),
            },
            ensure_ascii=False,
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
