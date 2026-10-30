"""Export the M02 terrain business-function review queue.

The base inventory intentionally excludes mechanical trait implementations and
generic owner-sensitive method names.  M02 is small enough to retain those
exclusions separately and manually identify cross-module source mappings.
"""

from __future__ import annotations

import collections
import json
import subprocess
from pathlib import Path


SCRIPT_DIR = Path(__file__).resolve().parent
ROOT = SCRIPT_DIR.parents[1]
REPORT_DIR = ROOT / "tiny-glade/reports/business/2026-10-23-module-recovery"
INVENTORY_PATH = REPORT_DIR / "B-MODULE-INVENTORY.json"
STEM = "M02-BUSINESS-FUNCTIONS-2026-10-30"


MANUAL_MAPPINGS = {
    "country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback::on_gpu_copy_issued": {
        "category": "GPU 回读与延迟发布",
        "location": "crates/country-core/src/systems/render_loop/compute_passes/read_terrain_data.rs:138",
        "resolution": "cross_module_mapped",
        "note": "由地形回读资源的跨模块定义承载；写入 changed_tick，随后 swap(0) 取走 pending，并以 OR 语义保留 copy_issued。",
    },
    "country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback::on_terrain_modified": {
        "category": "GPU 回读与延迟发布",
        "location": "crates/country-core/src/systems/render_loop/compute_passes/read_terrain_data.rs:134",
        "resolution": "cross_module_mapped",
        "note": "由地形回读资源的跨模块定义承载；对 pending 做 fetch_add(1)，由 draw/edit 等真实编辑消费者共同调用。",
    },
    "country_core::systems::terrain::impl$0::add_terrain_events": {
        "category": "地形系统注册与资源生命周期",
        "location": "crates/country-core/src/systems/terrain/mod.rs:238",
        "resolution": "same_file_impl_mapped",
        "note": "实现在 TerrainSystems trait impl 中；按原观察顺序注册 6 个地形事件。",
    },
    "country_core::systems::terrain::impl$0::add_terrain_systems": {
        "category": "地形系统注册与资源生命周期",
        "location": "crates/country-core/src/systems/terrain/mod.rs:205",
        "resolution": "same_file_impl_mapped",
        "note": "实现在 TerrainSystems trait impl 中；挂载工具 UI/编辑/回读链和历史录制系统。",
    },
    "country_core::systems::terrain::impl$0::insert_terrain_resources": {
        "category": "地形系统注册与资源生命周期",
        "location": "crates/country-core/src/systems/terrain/mod.rs:226",
        "resolution": "same_file_impl_mapped",
        "note": "实现在 TerrainSystems trait impl 中；初始化编辑状态、双高度场、hover 与 ScheduledTerrainReadback。",
    },
    "country_core::systems::terrain::ui_edit_terrain::MouseActivityTimer::new": {
        "category": "编辑资源构造",
        "location": "crates/country-core/src/systems/terrain/ui_edit_terrain.rs:83",
        "resolution": "generic_constructor_needs_review",
        "note": "没有同名 new；当前生产通过 Default 构造，字段初值已记录，但业务消费者和 new 是否对应独立原步骤仍待原证。",
    },
}


BODY_NOTES = {
    "history_record_terrain": "地形编辑历史录制系统；需要继续核对完整 typed history、游标与失败语义。",
    "delayed_terrain_changed_msg": "两帧软件延迟读取真实 GPU 发布副本，并分别提交权威高度场与视觉高度场；中间发送 OnTerrainChange。",
    "terrain_stroke_changed": "地形笔画变更后发出重算、修改通知与 StrokeChanged 声音事件。",
    "edit_terrain_stroke": "消费笔画添加/修改命令，更新地形权威数据，并请求重算和延迟回读。",
    "terrain_hover_idx": "从真实鼠标地形射线与 GPU 回读选择 hover 笔画，生产 HoveredTerrainStrokeId。",
    "ui_draw_terrain": "真实画布交互到地形绘制命令/编辑路径的 UI 生产端。",
    "ui_edit_terrain": "地形坡度、高度、falloff、焦点、历史哨兵、提示和装饰剔除等编辑交互主系统。",
    "ui_terrain_focus_mode": "地形 focus 模式的真实 UI 输入与模式切换衔接。",
}


def category_for(row: dict) -> str:
    if row["b_name"] in MANUAL_MAPPINGS:
        return MANUAL_MAPPINGS[row["b_name"]]["category"]
    leaf = row["b_name"].rsplit("::", 1)[-1]
    if leaf == "history_record_terrain":
        return "地形历史录制"
    if "delayed_terrain_changed_msg" in leaf:
        return "GPU 回读与延迟发布"
    if leaf in ("terrain_stroke_changed", "edit_terrain_stroke"):
        return "笔画编辑与变更传播"
    if leaf == "terrain_hover_idx":
        return "地形 hover 与拾取"
    if leaf in ("ui_draw_terrain", "ui_edit_terrain", "ui_terrain_focus_mode"):
        return "地形绘制与编辑 UI"
    return "其他地形业务"


def row_for_output(row: dict) -> dict:
    leaf = row["b_name"].rsplit("::", 1)[-1]
    manual = MANUAL_MAPPINGS.get(row["b_name"])
    candidates = row.get("candidates", [])
    if manual:
        current_location = manual["location"]
        resolution = manual["resolution"]
        note = manual["note"]
    else:
        current_location = (
            f"{candidates[0]['path']}:{candidates[0]['line']}" if candidates else None
        )
        resolution = "unique_body_candidate"
        note = BODY_NOTES.get(leaf, "需要原版语义、失败分支、生产接线和验收范围复核。")
    return {
        **row,
        "status": row.get("status", "generic_owner_review"),
        "business_category": category_for(row),
        "current_location": current_location,
        "resolution": resolution,
        "review_note": note,
    }


def main() -> None:
    inventory = json.loads(INVENTORY_PATH.read_text(encoding="utf-8"))
    included = [row for row in inventory["functions"] if row["module"] == "M02"]
    excluded = [
        row for row in inventory.get("excluded_associations", []) if row["module"] == "M02"
    ]
    mechanical = [
        row
        for row in excluded
        if row["reason"] == "mechanical_name_no_business_association"
    ]
    generic_owner = [
        row
        for row in excluded
        if row["reason"] == "generic_method_requires_owner_evidence"
    ]

    business_rows = [row_for_output(row) for row in included]
    business_rows += [row_for_output(row) for row in generic_owner]
    business_rows.sort(
        key=lambda row: (row["business_category"], row["b_path"], row["b_name"])
    )

    status_counts = collections.Counter(row["status"] for row in included)
    resolution_counts = collections.Counter(row["resolution"] for row in business_rows)
    category_counts = collections.Counter(row["business_category"] for row in business_rows)
    raw_groups = len(included) + len(excluded)
    commit = subprocess.check_output(
        ["git", "-C", str(ROOT), "rev-parse", "--short=9", "HEAD"], text=True
    ).strip()

    payload = {
        "audit": "M02 business-function lexical review queue with cross-module mapping",
        "date": "2026-10-30",
        "repo_commit": commit,
        "inventory_index_sha256": inventory["index_sha256"],
        "method": (
            "Consumes the current B-MODULE-INVENTORY.json and retains mechanical/"
            "generic-owner exclusions separately. The five lexical unmatched M02 "
            "functions were manually checked against current source. Source mapping "
            "is not original-equivalence, production-chain, or module-acceptance proof."
        ),
        "summary": {
            "raw_m02_groups": raw_groups,
            "inventory_included_groups": len(included),
            "business_relevant_groups": len(business_rows),
            "included_status_counts": dict(sorted(status_counts.items())),
            "excluded_mechanical_groups": len(mechanical),
            "excluded_generic_owner_groups": len(generic_owner),
            "resolution_counts": dict(sorted(resolution_counts.items())),
            "business_by_category": dict(sorted(category_counts.items())),
        },
        "business_functions": business_rows,
        "excluded_mechanical": mechanical,
        "module_boundary": {
            "statement": (
                "B-MODULE-INVENTORY 的 M02 主体只覆盖 systems::terrain 直接路径。"
                "地形高度资源、GPU compute pass、光标射线、渲染与共享历史仍位于 "
                "SHARED 或其它模块路径，不能据本清单宣称整个 M2 已做全函数审计。"
            ),
            "known_cross_module_families": [
                "resources/terrain_height.rs",
                "resources/cursor_terrain_raycast.rs",
                "resources/mouse_terrain_raycast.rs",
                "systems/render_loop/compute_passes/read_terrain_data.rs",
                "systems/render_loop/compute_passes/rasterize_terrain_stroke.rs",
                "systems/render_loop/compute_passes/highlight_active_terrain_stroke.rs",
                "tiny-glade/src/world_terrain.rs",
            ],
        },
    }

    REPORT_DIR.mkdir(parents=True, exist_ok=True)
    (REPORT_DIR / f"{STEM}.json").write_text(
        json.dumps(payload, ensure_ascii=False, indent=2) + "\n", encoding="utf-8"
    )

    lines = [
        "# M02 地形业务函数清单（2026-10-30）",
        "",
        "本清单沿用 M06 的调用链盘点口径，但 M02 直接函数组很少，因此保留被库存排除的"
        "机械实现和通用 owner 方法，并对 5 个词法未匹配项逐个核对当前源码。"
        "它用于定位人工审计对象，不表示原版算法等价或 M2 整模块验收。",
        "",
        f"- 源码提交：`{commit}`",
        f"- B 函数索引 SHA256：`{inventory['index_sha256']}`",
        "- 生成入口：[`tools/audit_m02_business_functions.py`](../../../tools/audit_m02_business_functions.py)",
        "- 基础库存：[`B-MODULE-INVENTORY.md`](B-MODULE-INVENTORY.md)",
        "- 机器可读结果：[`M02-BUSINESS-FUNCTIONS-2026-10-30.json`](M02-BUSINESS-FUNCTIONS-2026-10-30.json)",
        "",
        "## 口径与统计",
        "",
        "当前基础库存已预先排除 trait/serde/SystemParam 机械实现，以及 `new/get/insert/remove` "
        "这类无法只凭名称确定 owner 的通用方法。本报告把 M02 的这些排除项单独保留，"
        "避免丢失原业务 ID。",
        "",
        "| 指标 | 数量 |",
        "|---|---:|",
        f"| M02 原始函数组 | {raw_groups} |",
        f"| 基础库存直接保留 | {len(included)} |",
        f"| 唯一非空体候选 | {status_counts['body_candidate']} |",
        f"| 词法未匹配 | {status_counts['unmatched_review']} |",
        f"| 机械派生排除 | {len(mechanical)} |",
        f"| 通用 owner 方法单列 | {len(generic_owner)} |",
        f"| 业务相关函数组 | {len(business_rows)} |",
        "",
        "5 个词法未匹配项经源码核对后：3 个是 `TerrainSystems` 同一文件 trait impl，"
        "2 个迁移到共享 render 回读文件；没有继续处于“源码位置未知”的业务函数。"
        "但 `MouseActivityTimer::new` 仍属于通用构造语义待证项。",
        "",
        "## 业务功能分类",
        "",
        "| 功能域 | 数量 |",
        "|---|---:|",
    ]
    for category, count in sorted(
        category_counts.items(), key=lambda item: (-item[1], item[0])
    ):
        lines.append(f"| {category} | {count} |")

    lines += [
        "",
        "## 业务函数明细",
        "",
        "| 功能域 | 函数 | 词法状态 | 当前源码/结论 |",
        "|---|---|---|---|",
    ]
    for row in business_rows:
        location = f"`{row['current_location']}`" if row["current_location"] else "未定位"
        lines.append(
            f"| {row['business_category']} | `{row['b_name']}` | `{row['status']}` | "
            f"{location}<br>{row['review_note']} |"
        )

    lines += [
        "",
        "## 排除项",
        "",
        "### 机械派生或序列化",
        "",
    ]
    if mechanical:
        for row in mechanical:
            lines.append(
                f"- `{row['b_name']}`（`{row['b_path']}`，B ID `{', '.join(row['b_ids'])}`）"
            )
    else:
        lines.append("- 无")
    lines += ["", "### 通用 owner 方法", ""]
    if generic_owner:
        for row in generic_owner:
            lines.append(
                f"- `{row['b_name']}`（`{row['b_path']}`，B ID `{', '.join(row['b_ids'])}`）："
                "已保留为业务相关项，当前源码映射见上表。"
            )
    else:
        lines.append("- 无")

    lines += [
        "",
        "## 模块边界",
        "",
        payload["module_boundary"]["statement"],
        "",
        "至少还要把以下跨模块地形职责单独纳入 M2 语义审计：",
        "",
    ]
    lines.extend(
        f"- `{path}`" for path in payload["module_boundary"]["known_cross_module_families"]
    )
    lines += [
        "",
        "## 审计要求",
        "",
        "1. 非空函数体只证明源码位置存在，不能证明与原版算法、顺序、失败语义一致。",
        "2. `ScheduledTerrainReadback` 两个方法和三个 `TerrainSystems` impl 已定位，"
        "后续应直接审查其算法与真实 producer/consumer，不再列为名称缺失。",
        "3. `MouseActivityTimer::new` 当前无同名实现；需确认原版构造是否就是现有 "
        "`Default` 行为，并定位三个字段的真实消费者。",
        "4. 完整 M2 验收仍以真实输入、同一地形权威、GPU/碰撞/画面、历史、"
        "保存恢复和卸载链为准；本清单不改变该项状态。",
        "",
    ]
    (REPORT_DIR / f"{STEM}.md").write_text("\n".join(lines), encoding="utf-8")

    print(
        json.dumps(
            {
                "markdown": str(REPORT_DIR / f"{STEM}.md"),
                "json": str(REPORT_DIR / f"{STEM}.json"),
                "raw_groups": raw_groups,
                "business_relevant_groups": len(business_rows),
                "mechanical_excluded": len(mechanical),
                "generic_owner_reviewed": len(generic_owner),
            },
            ensure_ascii=False,
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
