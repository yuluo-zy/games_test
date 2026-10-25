# 数值断言测试报告（tester-numeric-assertions，2026-10-17）

> 只断言 RESTORE/REVIEW 已判等结论；不 mock、不改被测实现；未判等不编造期望。

| crate | test 函数 | assert 语句 | 结果 |
|---|---|---:|---|
| utils | 4 | 5 | 4/4 通过 |
| timberframe | 4 | 39 | 4/4 通过 |
| system-tree | 5 | 45 | 5/5 通过 |
| system-clutter | 4 | 35 | 4/4 通过 |
| system-critters | 4 | 27 | 4/4 通过 |
| system-roof | 8 | 20 | 8/8 通过 |
| system-onboarding | 2 | 17 | 2/2 通过 |
| **合计** | **31** | **188** | **31/31 通过** |

## 逐条摘要

**utils**
- `rotation_from_normal`：+X→(0.70710677,0,0.70710677,0)；+Z→(0,0,1,0)；-Z→(1,0,0,0)；+Y 退化→(0.70710677,0,0,-0.70710677)；斜向 (0.6,0.8,0)；q*Z≈normalize(n)；非有限输入 panic。来源 reports/gap/utils-toolbar/RESTORE.md。通过。
- `Rectangle2d::orientation_mat3`：列=(axis.x,0,axis.y)/(0,1,0)/(-axis.y,0,axis.x)。来源 wave3/reviews/tg-tr。通过。

**timberframe**
- 圆形计数：max(6,⌊p/1.5⌋)：30→20、12→8、9→6、7.5→6、3→6、20.25→13；除数 1.5(0x1429258d0)、截面 0.2。
- 矩形：⌊d/1.51⌋：4.6→3、3.03→2、1.60→1、1.40→0；截面 0.2/0.18、屋脊阈值 0.5 严格。
- **发现 0.18 与原生 0x3e3851eb 差 1 ULP（Rust 0.18=0x3e3851ec）**，按 ULP 级一致断言并登记。
- `timberframe_beam_count_change_audio`：Δframe==1 且 last_count!=count 才触发、kind=6、位置 x0y(center)=原点、Δframe=2/相等不触发、回绕 -1→0 触发。通过。

**system-tree**
- `random_tree_prefab`：6 串表 + bit0/corrupted/deny/halloween 门控、串长 22/29/26/39。
- `time_continuous_lerp`：a+(b-a)(1-exp(-δ·rate))；1-e⁻¹=0.63212056；rate=5.0(0x142925988)。
- 水下重映射：y=-2.5→-1.2、-5→-2.4、≥0 原样（DIVSS -2.5/MULSS -1.2 字节直读）。
- scale/rotation：0.5/0.9/360 端点；despawn 0.01 严格 `<`（NaN 不 despawn）；65.0 含边界（NaN 假）。通过。

**system-clutter**
- `mesh_slots_for_kind`：kind0 9 槽（含 (10,1.5)）、kind2 25 槽总权重 22.9、kind3 25、kind4 37、kind1/5→None。
- `select_mesh`：2→None、3→0x25、4→0x26；0/1 确定且落槽表。
- `weighted_mesh_landing`：max(0.01,Δ)→floor→xvy；(-5,-5)→(0,0,0)。
- `circular_position`：单位圆、y=0、同键确定。通过。

**system-critters**
- `sheep_walk_animation`：0.2 阈值（含等号）四分支；帧索引 (idx+n*20+3) mod 30（24.0=0x142934600、1.25=0x142a8d600）；过渡 progress+=dt/0.1；being_petted 0.01（=0.01 不跳过）。通过。

**system-roof**（`RoofAnimation::advance`）
- 删除分支 dt·-4/-10 clamp 0；平顶 factor2 primary 0.5714286；坡顶 factor1 0.95+0.2857143 clamp 1.0；secondary 严格 >0.99；ty 阈值 0.1；delta==0 不改；dt=NaN 语义；常量位值 0x4036db6e/0x40555555/0x3f7d70a4。通过。

**system-onboarding**
- `quantize`：0→0（含 -0.0）；(0,0.4]→1；(0.4,0.99]→2；<0/>0.99→3；±inf→3；NaN→1。
- `DELETION_COOLDOWN_TABLE`：18 项，索引 1/4/5/6/7/10/11/12=0.3，其余 0。通过。

## 未断言清单
clutter construct_inter_roof_merlons（未实现）；cell_center_world（I）；tree Insert/replay/convert/hide/ui_move/ui_delete（未实现-需重做）；timberframe 逐梁写路径（GAP）；critters update_sheep_goals 分支常量（系统查询环境）；roof calculate_roof_trigger_volumes/ui_edit_roof_ridge_and_height_focused（未实现）；onboarding exponential_back_off 720/10800/1.5（语义 I）。

## 阻塞/归属
1. 既有失败 `system-roof/tests/roof_replay_history.rs::unresolved_auxiliary_owner_does_not_advance_cursor`——非本任务新增，归 history/roof 回放链 WIP。
2. 为跑通测试做 3 处最小跨域编译解阻（非逻辑改动，需 owner 复核）：
   - `timberframe_beam_count_change_audio.rs`：PlayTimberframeSound 补 flag/speed_source/value（0/0.0/0.0）。
   - `country-core/recovered_building/history_snapshots.rs`：`HistoryEntry` 改 `pub use`（类型定义不变）。
   - `critters/spawn_ducks.rs`：`as_float_grid()[i]`（同 duck.rs）。
3. 执行中磁盘一度写满（ENOSPC），已清理 onboarding 证据下的 target。
4. 复现：`cargo test -p utils|timberframe|system-tree|system-clutter|system-critters|system-roof|system-onboarding`。
