# ivy 实时复合交付（A 植被，2026-10-16）

实现者：A 组实时复合 agent。依据 `ivy-DELTA.md` 的"合成 agent 应关闭的真实剩余缺口"逐项关闭。
源码：`crates/country-core/src/{resources/ivy,systems/ivy.rs,systems/ivy}`。

编译状态：cargo check 下本组文件 0 错误。全 crate 剩余错误均属其它链在制域
（`resources/roofs/geometry/*.rs` 的 E0433、`RoofTypeChanged` E0422/E0425），非本组。
workspace 根 Cargo.toml 正被并行重构（新成员缺目录、country-core↔system-roof 依赖循环、
roof crate 源码 staged 删除），cargo check 无法稳定跑通全 crate —— 已报中央，未改任何组外文件。

## 真实代码接线

| DELTA 项 | 文件 | 改动 | 机器码依据 |
|---|---|---|---|
| G2.3 spawner 参数三处纠偏 | `systems/ivy/ivy_spawner.rs` | `count_range` 起点 `20`（`0x14`，栈参 [rsp+0x28]）；`scalar_range` 按 path：cursor `0.0..3.0` / random `0.0..1.0`（表 0x142ab3218=3.0/1.0，rne-read 验证）；`source_60` 传真实墙 id（`wall_id.0`，栈参 [rsp+0xf8]） | SPEC G2.3；rne-read 0x142ab3218 |
| G2.1 预算 | 同上 | 门改 `total_point_count + 0x13 < 0x9c40`（`IvyStorage::total_point_count()` 新增，0x14094d09b..a7）；补每帧 `min(0x46, 0x9c41 - total)` 点上限（0x14094db6e..b9e），`spawn_validated_root` 按 `maximum_point_count` 递减预算 | 0x14094d09b..a7 / 0x14094db6e..b9e |
| G1.5 方向种子 | `systems/ivy/ivy_grower.rs` | `get_direction(tail, ivy.source_60)` → `get_direction(tail, ivy.id)` | 0x14094b4bd `mov r8,[r14-0x70]`（=记录+0x20 ivy id） |
| G1.1 守卫字节 | `systems/ivy/ivy_grower.rs` + `resources/ivy/ivy_storage.rs` | 守卫 toggle 从 `Local<GrowerState>.guard` 移到 `IvyStorage.flag_48`（bit0，每帧 xor，置位才跑闭包）；`GrowerState` 删 `guard` 字段 | 0x14094c09f..c0c0（`movzx [rbx+0x48]; xor cl,1; test cl,1`） |
| G6 洞/跳墙约束 | `systems/ivy/ivy_grower.rs` | 新增 `hole_blocks_growth`（`Res<PrevWallHoles>::wall_holes(wall_id)` → 逐 `kind==0` 洞 → AABB 展开 `(-0.1,-1.0,0.1,1.0)` → 探针(u 距离, tip.y) 盒包含）；命中写 `ivy.flag_8c=1` 并跳过本段加点；下一主循环消费并清零 flag_8c；补 **0.42 落点门**（`(next-tail).len() > 0.42` 拒绝，含 re-anchor 候选） | 0x14094b7be..b86e、0x14094ae63..96、0x14094bd38/bf26；rne-read 0x142ab31b0=0.42、0x142ab31a0=(-0.1,-1,0.1,1) |
| G3.4 根点弹根 | `systems/ivy/ivy_pruner.rs` | `ivy_pruner` 重构为"尾点无效删尾 + 根点无效 pop_front"两段；pop_front 后 `ivy.root` 副本用新 front 覆盖，叶子清理同删尾 | SPEC 3.3/3.4（0x140998335..b0、0x140998430..711） |
| G3.2 is_point_still_valid 判据 | `systems/ivy/ivy_pruner.rs` | **判据定案 = interpolated 带**（`(-0.01/h, 1+0.01/h)`，对 spec 3.2 的"wall_y+带常数"），补 `seg_count >= 2` 检查 | decomp-0x140998dc0（`fVar23=0.01/[wall-0x270]`、`interpolated<=-band` 返回 0、`< band+1` 有效）；rne-read 0x142960674=0.01、0x1429258f0=0x80000000 符号翻转变换 |
| G2.4 ActionSet pressed 门 | `systems/ivy/ivy_spawner.rs` | 新增 `Res<MainActionSet>`，`ivy_spawn` 数字动作（`DIGITAL_NAMES[36]`）`pressed()` 门；`register_ivy_storage` 补 `MainActionSet`/`PrevWallHoles` init | SPEC G2.4；action_set.rs DIGITAL_NAMES |
| G8 send_to_gpu | `systems/ivy/ivy_send_to_gpu_{segments,leaf}.rs` | `arch_height` 由 0.0 占位改为真实墙 arch（`ivy.source_60`/`leaf.source_18` → `wall_arch_height`，墙缺失回退 0.0）；`seed=frame` 保留为候选并标注待取证；D 组 render 交接点登记于模块头 | shader 布局对齐（instanced_ivy_branch/leaf.raster.hlsl） |

## candidate 链测试

- 既有测试全保留：`ivy_grower` 的相位循环/事件、`ivy_spawner` 的垂直采样/近似 u、`ivy_pruner` 的 leaf 清理、`send_to_gpu` 的 48B 打包。
- 更新签名处的测试：`segment_packs_to_48_bytes` / `leaf_packs_to_48_bytes` 传 `&PublicWalls::default()`，并断言缺失墙时 `arch_height == 0.0`。
- 未新增系统级跑测：workspace 无法稳定 `cargo test`（见编译状态）；`cargo check` 已覆盖本组全部文件无错误。

## 原侧验证

- `rne-read.py` 实读常量：0x142ab3218 = 3.0/1.0（scalar 表）、0x142ab31b0 = 0.42（落点门）、0x142ab31a0 = (-0.1,-1.0,0.1,1.0)（盒展开）、0x142960674 = 0.01（pruner 带）、0x1429258f0 = 0x80000000（符号翻转）、0x142960690 = 0.0（含对既有 VERTICAL_MIN=0.000928482 的疑点登记，见仍缺项）。
- 方向种子：0x14094b4bd 传 `[r14-0x70]` = 记录+0x20（ivy id），与 event 载荷同源（0x14094b27e）。
- 守卫字节：0x14094c09f..c0c0 对 `[rbx+0x48]` 逐帧 xor bit0 且置位才进闭包。
- G6：0x14094b7be..b86e 洞 slice 循环（0x140f86d00 返回 ptr/end、20B 条目、`[+0x10]` 字节非 0 跳过）+ 0x141240140 盒包含 + `mov byte [r14-4],1`；0x14094ae63..96 flag_8c 消费；0x14094bd38/bf26 0.42。
- pruner：decomp-0x140998dc0 完整核对 interpolated 带判据；`[wall-0x290]>=2` 与 `[wall-0x270]>0` 在 decomp 中逐条吻合。
- garden 无改动（P2 定论见 garden-DELIVERY.md）。

## 仍缺项（诚实 remaining）

- `SPAWN_COUNT_RANGE.end`（count_range 上限）：栈 [rsp+0x30] 是存储指针非计数，语义待取证；现用 20..40，注释锁定。
- `PrevWallHoles::wall_holes` 探针精确变换（`u * [wall-0x14]` + 0x14003bb80）：本组用 (u 距离, tip.y) 近似，盒坐标空间一致，精确 SIMD 待取证。
- flag_8c 消费的 leaf 清理谓词：本组直接清零（可观察行为=洞命中暂停一帧）；原生"list 查 + 清零"的 list 身份待取证。
- ActionSet pressed 键绑定：`ivy_spawn`（槽 36）是候选，具体原生按钮/映射待取证。
- `is_point_still_valid` 的 wall 附加采样点（±0.1 AABB terrain grid）仍待取证。
- send_to_gpu `seed`（frame 候选）与 `wall_normal`（切向垂线）真值待取证；GPU 真机执行后置，消费系统由 D 组 render 挂链（交接点已在模块头登记）。
- random 根法向偏移 0.68 vs SPEC 0.63（0x1429258d4）：维持 0.68（现实现），冲突待取证。
- `VERTICAL_MIN = 0.000928482`（ivy_spawner.rs）声称在 0x142960690，rne-read 实读该址为 0.0：疑点登记，未改动（避免影响既有 spawn 测试）。
- 22 系统链精确顺序（G9）：仍待取证，`add_ivy_systems` 保持 7 系统 CPU 链。
- 常量表 `0.42` 语义（落点最大距离）已实装；SPEC 常量表其余已逐一 rne-read 验证。

## 组外申请记录

- 已向中央 agent（a8a2e9731c4957436）发送：编译状态、ivy/garden 完成清单、garden audio 130.0 门仲裁申请、workspace 重构阻塞报告。
- 未修改任何组外文件。临时验证期间对 `crates/systems/roof/` 的 stash/restore 已原样恢复（`git status` 复查一致）。
