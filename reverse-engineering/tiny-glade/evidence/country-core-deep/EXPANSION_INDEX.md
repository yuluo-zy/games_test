# country-core 缺口模块深挖 — 中央索引（expansion，2026-10-14）

承接 `INDEX.md`（systems/resources 八域已收齐）。本批针对
`country-core-audit/MODULE_GAP_ANALYSIS.md` §1「深度证据 0%」的模块，
按**结构化章节流**方法补齐。

## 0. 方法（与 2026-10-14 第三轮一致）

1. 由 B 骨架建**源码地图**：`SOURCE_MAP_EXPANSION.json`（53 文件 / 283 业务函数 / 166 内联 / 77 胶水）。
   分类规则：记录形式含 `compiled_procedure` → 业务；仅 `inline_instance` → 内联；名字命中硬胶水模式 → 胶水。
2. 逐文件生成章节骨架 → `chapters-expansion/<path>.md`（占位符清零后即权威入口）。
3. 每章 = 文件作用 → 业务函数表（地址/行/作用猜测→证实/逆向结论 O-I-U/证据）→ 内联推断 → 派生胶水分组。
4. 工具与已证实事实见 [PLAYBOOK.md](PLAYBOOK.md)；**不要改 reconstruction 源码**（本轮文档+证据 only）。

## 1. 批次总表

| 批次 | 模块 | 文件 | 业务函数 | owner |
|---|---|---|---|---|
| B1 | `components/*` + `utils/*` | 8 | 153 | agent-1 |
| B2 | `startup/*` + `input/*` + 根配置库 | 45 | 130 | agent-2 |
| **合计** | | **53** | **283** | |

（B1/B2 划分见 §2 / §3。）

## 2. B1（agent-1）：components + utils

| 文件 | 业务函数 | 机器码 | 备注 |
|---|---|---|---|
| `components/mod.rs` | 54 | 76 KiB | 全是组件的 `register_required_components`（Bevy `Component`/`Bundle` trait 实现） |
| `components/drawable.rs` | 36 | 25 KiB | `NaniBundle<T>`/`DrawableMeshBundle` 的 `component_ids`/`get_components`/register |
| `components/transient_mesh.rs` | 5 | — | TransientMesh 注册表 |
| `utils/dirty_tiles.rs` | 27 | 23 KiB | 地形脏区 pub/sub，渲染 C 层多 pass 依赖 |
| `utils/debug_viz.rs` | 22 | 7 KiB | 调试可视化 |
| `utils/geo.rs` | 6 | — | 几何工具 |
| `utils/load_json.rs` | 5 | 31 KiB | `load_json_as_mesh`，68 个调用方依赖 |
| `utils/mod.rs` | 5 | — | |
| `utils/audio_utils.rs` / `child_wall.rs` / `project_cursor_onto_ray.rs` | 4/1/3 | — | |

## 3. B2（agent-2）：startup + input + 根配置

| 文件组 | 文件 | 业务函数 | 备注 |
|---|---|---|---|
| `startup/mod.rs` 及加载器 | 13 | ~40 | 网格/着色器/纹理/预制体/图集入库生产者 |
| `startup/startup_*.rs` | 24 | ~28 | 各子系统启动 |
| `startup/glade_themes/*`、`init_*`、`load_*`、`spawn_*`、`clearing` | 8 | ~12 | |
| `input/*` | 4 | 28 | `input_stream`/`action_set`/`panic_repro_state` |
| `game_config.rs` / `game_key_bindings.rs` / `game_profile.rs` | 3 | 25 | 按键绑定、游戏配置 |

## 4. 章节清单

见 `chapters-expansion/`（文件名 = 源路径，`/`→`__`）。
每章填充后占位符 `<待填>` / `U` 必须清零或明确标注 I/U 依据。

## 5. 与既有证据的关系

- `components/*` 在 `MODULE_GAP_ANALYSIS.md` 是 0%/0%；本批是**首份**证据。
- `utils/dirty_tiles`、`utils/load_json`、`startup/*` 在 `RENDER_GAP_ANALYSIS.md` §5.6 列为
  「渲染范围之外、但渲染必需」。
- `utils/*` 与 `reconstruction/shared-foundation/crates/utils` 有重复实现（见
  RENDER_GAP_ANALYSIS §4.1），核对差异时以机器码为准。

## 6. 待办

- [ ] B1/B2 章节填充（agent）
- [ ] 跨批次：`components` 组件注册表 ↔ `startup` 资产入库的接线关系
- [ ] 与 `AUDIT_REPORT.md` 的 O 抽查口径对齐
