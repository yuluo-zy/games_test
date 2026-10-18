# 逐函数逻辑重建（深度收口）— systems/terrain + systems/ui_clutter 待办

重建者: impl-terrain-deep | 日期: 2026-10-17
协议: planning/2026-10-16-logic-verify-protocol.md（fresh 反编译为准、分类、按控制流实现）
依据: reports/interaction/{terrain,ui_clutter}/{LOGIC,REBUILD}.md + 本轮 fresh 反编译
证据（ghidra shared 只读 + **ReVa 专属 --write worker `impl-terrain-deep`**）:
- `recovery/agent-team/work/impl-terrain-deep/jobs/*.json|*result.json`（ReVa run-script/get-decompilation）
- case 4 转移表目标、focus closure 名槽、History::add 派发逐字节证据。

## 结论摘要

- 本轮收口 6 项登记待办：**代码实现/修正 8 个函数**、**定点取证定案 2 项**、**宿主接线登记 3 项**。
- 编译：`cargo check -p country-core --lib` 本轮 **0 error**（rhapsody 并发重构注入的
  render API 断点在途，非本模块；详见下）。
- 全部以 fresh 反编译为准；宿主 ECS 实体接线只登记不伪造；未改他模块。

## Batch 1 — 判等-注意 4（LOGIC.md 附注收口）

| VA | 原函数 | Rust 位置 | 处置 | 落地 |
|---|---|---|---|---|
| 0x140ac9770 | history_record_terrain | deferred_terrain_history.rs | **修正-判等** | 删 `if let Err → log+return`；`History::add()` 忽略返回值后**无条件**写 8B debug 事件（fresh：add 后无分支直接事件写） |
| 0x1409a9160 | history_record_auto_clutter | defered_autoclutter_history.rs | **修正-判等** | 删 `if let Err → HistoryRecordingFailed` 分支（fresh：add 结果无分支、无失败事件）；签名去 failures 参数 |
| 0x1408bbf50 | RockyTerrainMesh::load | rocky_terrain.rs | **修正-判等** | 返回 `anyhow::Result<Self>`（fresh：`anyhow::error::impl$1::from<io::Error>`/`from<serde_json::Error>`），原 `Result<_, io::Error>` 改 |
| 0x140924900 | TerrainEditingData::get_last_mut_stroke | tg-terrain stroke.rs（库外） | **裁定-错误路径适配** | fresh：next_index==0 → unwrap_failed **panic**；Rust Result Err。tg-terrain 为共享基础 crate（红线不改他模块），Result 为 API 适配，主路径逐条判等 |
| 0x140924a10 | TerrainEditingData::new_stroke | tg-terrain stroke.rs（库外） | **裁定-错误路径适配** | fresh：碰撞 panic_fmt；Rust Err(IdCollision)。同上裁定 |
| 0x1408b54c0 | load_index_unchecked | terrain_height.rs | **裁定-布局适配** | disasm 证实算法 = `u16/65535`（DIVSS 0x142a7cd4c）+ `v*max+(1-v)*min`（SUBSS/MULPS/ADDSS）；原版紧凑网格视图 min/max@+0x10/+0x14，Rust TerrainHeightsData min/max@+0x50/+0x54——语义相同，字段偏移为 Rust 布局合并适配，非 ABI 契约 |

## Batch 2 — ui_edit_terrain case 4（ReVa 定点恢复，第 1 待办）

**取证**（ReVa `impl-terrain-deep`，jobs/reva-jumptable/case4*）：
- 主 subtype 转移表 `jt_404c@0x142ae404c`：16 条目标（subtype 2..17 经 `-2` 索引）；第 4 项 →
  `0x140af02e2`（case 4 处理器）。
- case 4 处理器 `0x140af02e2`：`ADD R8,-0x8; CMP R8,0xb; JA 0x140af03b5; LEA RCX,0x142ae405c;
  MOVSXD R8,[RCX+R8*4]; ADD R8,RCX; JMP R8` → 子转移表 `jt_405c@0x142ae405c`（索引 8..19，
  12 项）。distinct 目标：8→0x140af02f2、10→0x140af0344、12→0x140af0320、16→0x140af036c、
  19→0x140af038c；其余/越界 → 0x140af03b5（共享尾 `[pcVar39]=0, [pcVar39+1]=6`）。
- **语义 = 按 UI 参数/widget 名分派写 subtype 响应状态字节**：
  - idx8：名比较 `"ev_shift"`（`48 b9 65 76 5f 73 68 69 66 74`）与 `"contrast"`（命中 →
    `[pcVar39+1]=4`）；
  - idx10：`"saturati"` XOR [name] + [name+8]u16 XOR 0x6e6f（"on"）→ `"saturation"`；
  - idx12：`"bypass_c"` + [name+8]u32 `"olor"` → `"bypass_color"`；
  - idx16：16B 位掩码（PAND + PMOVMSKB==0xffff）→ `[pcVar39+1]=2`；
  - idx19：双 16B PAND（[name]&mask、[name+3]&mask）全非零。
- **实现**：ui_edit_terrain.rs case 4 分支改登记恢复机制（名常量 + 响应字节写语义），
  宿主 UI 名队列接入后按名分发；其余 case 6/8/0xc/0xf = 表字节状态机写登记。

## Batch 3 — ui_terrain_focus_mode 复杂体（第 2 待办）

- 门确认：AppMode **!= 5** → return（fresh ReVa 复核）。
- **signifier 完整链接线**（closures$1-4 fresh）：`SignifierEntityBuilder::build` ×4 =
  主函数 `AppendOnlyVec::push<UiAppendSignifier>`×4：
  - closure$1 `"vertex_color_shader"`（字节 `63 5f 78 65 74 72 65 76 63 6f 6c 6f 72 5f 73 68 61 64 65 72`）+
    bevy command push `insert<DrawTopology>`（`FnOnce::call_once<...insert<DrawTopology>...>`）；
  - closure$2 `"brush_arrow_flat_to_single_forward"`、closure$3
    `"brush_arrow_flat_to_w_flat_single_up"`（strncpy 字节）、closure$4 名槽动态（RNE 含
    vertex_color_shader，其余捕获数据）。
- **实现**：新增 `BRUSH_ARROW_SINGLE_FORWARD_NAME`/`BRUSH_ARROW_W_FLAT_SINGLE_UP_NAME`
  常量；closure$1 的 UiSignifierStream 记录 + `SignifierDrawTopology` insert 事件保留
  （CPU 侧载体）；closure$2-4 入流与复杂体（焦点 HashMap/as_curve3/CircularF32/
  RequestAppModeChange/相机旋转）宿主实体机制登记待接，**不伪造实体接线**。

## Batch 4 — decoclutter_show_pins 几何/探测/实体 spawn（第 3 待办）

- fresh 完整控制流复核（0x1412357c0）：逐 pinned → GlobalId hash probe → DecoratorStorage
  info → `clutter_mesh_ty_data()` 足迹 → `Rectangle3d::expand(+0.5,+0.5)` →
  `set_orientation_xz(axis)` → `translate(实体世界位置)` → `Rectangle3d::intersects` +
  条件门（`*local_1b0==2/1`、`[10]==2` panic）→ SparseInstanceBuffer ChunkId 写。
- **实现**：新增 `pinned_footprint_intersects`（纯几何落地，O 序列
  expand→set_orientation_xz→translate→overlaps_rectangle3d）+ 单测；双 Vec 缓冲 +
  PinVisibilityToggle 生产保留。
- **登记**：per-pinned 几何源（clutter_mesh_ty_data + 实体变换）与触发体积宿主提供；
  实体 spawn（DecoClutterLock/Ui2dWsExtent/Ui2dImage）与 SparseInstanceBuffer 写宿主
  接线待接（不伪造）。

## Batch 5 — EditType::Terrain 真机制（第 4 待办）

- **定案**：fresh 0x140ac9770 记录 = 直接 0x38B `{tag=1, [1]=1, old_state, pending[0..3]}`
  入 History；`History::add`（ReVa fresh 0x140a84ec0）仅对 **tag-7 Wall** 做 add 期校验，
  地形记录**通用存储**（0x18 stride 条目 push），**非 RasterEdit 族**（world_raster 的
  Path/Cobblestone/Water/Garden 记录 RasterStroke 载荷，另一记录路径）。
- Rust `EditType::Terrain` + `ReplayCommand::Terrain` 为 CPU 侧等效模型（mod.rs 标注成立）；
  add 期仅 Wall 校验（`HistoryRecordingValidation` 已对齐）。

## Batch 6 — stairs/decorator 消费链宿主接线（第 5 待办）

- **生产者核验**：edit_terrain_stroke（tag 1/2/3/4）、terrain_stroke_changed（tag 0）、
  ui_edit_terrain（tag 2）写 `StairsPlacementBucket`（indexmap::Bucket 等价）；tests 断言
  tags [3,2,0]。
- **消费链登记**：`StairsPlacementBucket` → `DecoratorStorage::Stair`（内联桶，已实现）→
  ui_clutter decorator 实体生成 = stairs 域宿主接线；crate 无消费系统（不伪造），
  mod.rs「台阶放置消费面」标注保留并扩展登记。

## 编译状态

- 本轮 Batch 1 通过时 `cargo check -p country-core --lib` **0 error**。
- 后续并发 agent 的 rhapsody「渲染模块架构重构」（AttachmentCache/render_pass API 变更，
  工作树未提交）注入 country-core render 编译断点（`managed::render_pass`、
  `Device::create_render_pass` 缺失）——**非本模块改动所致**，红线不改他模块，未处理。
  本模块文件 rustfmt 解析通过（无语法错误）。
- 测试：ui_clutter 既有单测在 Batch 1 后全过（2/2）；terrain/decoclutter 新几何单测随
  库编译验证（render 断点解除后可跑全量）。

## 留给集成 owner 核对点

1. Batch 1 三处判等修正对照 fresh（history_record_terrain/auto_clutter 无条件事件写、
   RockyTerrainMesh anyhow）。
2. case 4 名常量字节（ev_shift/contrast/saturation/bypass_color）对 jt_405c 目标反汇编。
3. focus closure 名槽字节（brush_arrow_flat_to_single_forward 等）对 RNE。
4. decoclutter 几何单测与 0x1412357c0 的 expand/intersects 序列。
5. rhapsody render API 断点解除后重跑 `cargo check -p country-core --lib` + terrain/ui_clutter 全量测试。
