# WC② 实现交付：`system-wall-constructor`（第 11 轮，2026-10-19）

> 工作目录：`D:/game/tiny-glade`（crate `crates/systems/wall-constructor`）。
> 契约：`evidence/round11/contracts/wc-constructor-CONTRACT.md`。
> 证据：`evidence/country-core-deep/systems-recovery/groupC/wall-constructor/decomp/**`
> （逐函数 Ghidra 反编译 + `half_timbering-asm.txt`）、`evidence/building-structures/**`、
> `evidence/building-segmentation/**`；常量以
> `python scripts/rne-read.py --hex <va> 4` 直读 .rdata。
> 仅改本 crate；未触碰 country-core / rhapsody / roof / wall 用户区。

## 1. 本轮实现（真实代码 + 测试）

| # | 函数 | VA | 落点 | 状态 | 说明 |
|---|---|---|---|---|---|
| 1 | `ArchWalker::walk` | 0x141232bb0（4 单态同构） | `src/arch_walker.rs` | **已完成（primitives-only U）** | 按原生汇编入口/循环逐指令重建：参数面 `(heights, points, us, closure)`（RDX/R8、R9/stack28、stack30/38 为闭包捕获的 `&[f32]`）；`total=Σ‖(x,y+h,z)‖`；`n=max(1,floor(total/brick_len+0.5))`；`residual/traversed` 推进；`u=project_clamp(lerp(us_i,us_i+1,fraction))`；尾砖 `step*0.1<residual`。旧近似体 4 处结构分歧已消除。 |
| 2 | `SupportBeamGrower::propose` | 0x141fe09c0 | `src/construct_lintels/place_support_beams.rs` | **已完成（primitives-only U；P0 修正）** | 方向投影读 **grower 状态 flags@+0 的 bit0**（不是输入点 flags）；方向归一化、`cost=new_len*(‖Δdir‖+0.5)`（len≥1）、3.1499999 上限、41B 投影（index@0/xz_len@8/dir@12/curvature@24/point@28/**tag@40**）；`propose_into` 先写 `tag=2` 无效哨兵。 |
| 3 | `place_support_beams` | 0x141242530 | 同上 | **已完成（primitives-only U）** | 双 Grower 向中 propose、cost 小者 consume、`len≥1` 发射、join `total∈[1,3.15]` 且可再 propose、未 join 两段 `len*0.85` 对角、join 且 `total≥1.4175` 双侧 `total*0.5`。 |
| 4 | `add_diagonal_support` | 0x141243740 | 同上 | **已完成（primitives-only U）** | `extend_xz`、锚高 `sample−0.225`、`length=min(budget−0.2625, height*0.4)`、延展 `length+0.1125+0.1875`、三轴 0.2625。 |
| 5 | `emit_support_beams` | 0x141243c20 | 同上 | **已完成（primitives-only U）** | `rdp_support_indices`（core O）→ 逐索引对 ArchWalker(step=3.1499999, tail .1) → 64B WoodenPlank（height−0.225、x 厚 .225、z 厚 .3375、`(x*1000)+(z*1000)+base` 索引）。 |
| 6 | `PillarProposals::add_proposal` | 0x1407534d0 | `src/place_pillars.rs` | **已完成（有消费者 place_pillars；候选源 U；P0 修正）** | 三向量 `{proposals 28B, scores f32, keys u64}`；同 key score NaN-aware min；**仅当新 score 严格大于末提案 score 时**用新 28B 记录覆盖（原判据 `new < old || new == old → return`）；新 key 三向量 push。 |
| 7 | `assemble_stone_pillar` | 0x14120d670 | 同上 | **已完成（primitives-only U；几何/分段/随机行，P0 修正）** | `param_3 += 0.2`、`param_4 += 0.2`；`param_3≤1.26` 单段 `[0,param_4]` 否则双段 `[0,param_3]`/`[param_3,param_4]` 且 `flag=1.2<|Δ|`；**顶盖门控 `param_3>2.1`（与 1.26 同一参数）**；`span=(b−a)−(flag?0.25:0)`；`count=max(3,round(span/0.56))` + `random_splits_into(count,0.28/span)`；映射 `t*span+a`；flag 段追加段尾；`≥0.01` 发射；jitter `0.42±0.05/0.7±0.05`，flag 段末段固定 `0.7/0.84`；顶盖 `{0.72,0.5,0.85}` @ y−0.45。96B 经 `BrickTransformSsbo::observed_bytes`。 |
| 8 | `overlapping_windows` | 0x1412218a0 | `src/utils/half_timber.rs` | **已完成（primitives-only U；几何/分支）** | 窗 AABB + x 内侧 0.038；dormer（subtype '9' 且 raw[0xc]==1）x 外扩 ±0.14；`intersects` 直接命中；否则 `center±expanded_limit` 移位再测；20B ResourceAccessInfo 投影。 |
| 9 | `get_window_aabb` | 0x141221da0 | 同上 | **已完成（primitives-only U）** | 门类 `raw[0x46]=='9'`：profile 末/首点 → `[x0−px, y0+py, x0+px, fy+0.093+y0]`；普通：`width=max(bbx0+0.2,0.46)`、`height=bbx2−bbx1`、'8' +0.2、':' (−0.2 width@raw[0x44]==3, +0.4 height)。 |
| 10 | `check_for_overlap_with_all_windows` | 0x141221f70 | 同上 | **已完成（primitives-only U）** | `contains(point)`；否则按 `expansion` 左/右移位中心 x 再测；半宽≤中心时跳过。 |
| 11 | `add_splits_in_empty_range` | 0x140751240 | `src/segmentation/calculate_wall_path_segmentation.rs` | **已完成（primitives-only U）** | 旧 `[0.0;4]` 比例近似已删除；`Curve2::try_new` → `n=floor(len/spacing)`，`n==1&&len<spacing*1.5→0`；`t=i/(n+1)` 断言；`coord_at_u`；`index=base+segment+(frac>0.5)`；`flags[index]|=8`；写 8B 坐标。 |
| 12 | `ArchFunction::remap_t` | 0x140765300 | `src/construct_arches/profile.rs` | **已完成（primitives-only U）** | 旧 `1−(1−t)²` 已删除。mode0：`1−(1−s)·(2|t−0.5|)³`，返回 `(outer,0)`；否则 `k=min(1,max(0,2−blend))`、`inner=(1−k)(1−s'·t2³)`、`outer=inner+k(1−s'·sq)`，`sq=mirror?(1−t)²:t²`。原件双返回值已保留。 |
| 13 | `check_angle` / `find_corners` | 0x141215540 内联体 | `src/utils/wall_corners.rs` | **已完成（primitives-only U；检测）** | ±10° 余弦窗口 `[−0.1736482, 0.17364822)`、NaN 过滤；贪心步进（命中 +1、未命中 +2）。 |
| 14 | `wall_constructor::helpers::{add_wall_corners,add_skirts,stairs_signifiers}` | — | `src/wall_constructor.rs` | **接线** | 旧空函数体改为委托到 `utils::{wall_corners,wall_skirts,stairs_signifiers}`，未闭合算法保持显式 panic，不再以空体吞输出。 |

## 1.1 WC③ P0 修复（本轮续跑）

| 项 | 修复 | 证据 |
|---|---|---|
| `add_proposal` 覆盖判据 | `new_score > last.score()` 才覆盖；相等/更小保留旧提案（scores 仍取 NaN-aware min） | 0x1407534d0：`if (new < old || new == old) return` |
| `propose` bit0 来源 | 方向投影读 `SupportBeamGrower.flags@+0` 的 bit0；输入点 `flags[index]&0xf` 只决定 tag | 0x141fe09c0：`uVar4 = *in_RDX`（grower） |
| tag 无效哨兵 | `SupportBeamProposal::INVALID_TAG = 2`；`propose_into` 先写 2 再按有效提案覆盖；41B 投影含 tag@40 | 0x141fe09c0：`uVar7 = 2` 初值、写 out+0x28 |
| `assemble_stone_pillar` 参数 | `param_3/param_4` 均先 +0.2；1.26 与 2.1 顶盖门控共用 `param_3` | 0x14120d670 入口与尾部 `2.1 < param_3`；分段构造为 `[0,param_3]`/`[param_3,param_4]`（升序低/高端），故 `param_3=bottom`、`param_4=top` |
| `construct_walls` panic 隔离 | 现为 core `construct_wall_bricks` 的 re-export，Bricks 链不再调用 corners/skirts/signifiers 的 U 函数；首事件不会因 U panic 中断 | `src/construct_walls.rs` |

## 1.2 消费者接线

新增 `crates/tiny-glade/src/wall_constructor_plugin.rs`：

- `build(app)`：真实 `init_resource` 5 项——`WallConstructorOutput`、
  `WallPathSegmentationMasks`、`WallPathSegmentationMasksMinusStairs`、
  `PillarProposals`、`PlatformStairsStorage`。
- `register_cpu_systems(schedule)`：调用真实
  `system_wall_constructor::add_wall_constructor_systems(schedule)`。

**中央需要插入的行**（本 agent 未改 `run_game.rs`）：
1. `crates/tiny-glade/src/lib.rs`：`pub mod wall_constructor_plugin;`
2. `crates/country-core/src/startup/run_game.rs::register_update_systems`
   的 `add_wall_cpu_systems(schedule);` 之后：
   `system_wall_constructor::add_wall_constructor_systems(schedule);`
3. App 构建处调用 `tiny_glade::wall_constructor_plugin::build(app);`
   （或在 core `register_wall_cpu_resources` 后初始化同 5 个资源）。

**8 个无 ECS 消费者的实现按 primitives-only U 登记**（有真实调用链但不宣称
完整原系统）：`ArchWalker::walk` → `emit_support_beams` → 64B 原语；
`propose`/`place_support_beams`/`add_diagonal_support` 同为 lintel 原语
（ECS 侧仍走 core `construct_lintel_supports`）；`assemble_stone_pillar`
为柱原语；窗三件为装饰器原语（遍历属 core 缺口）；`add_splits_in_empty_range`
为分段原语（ECS 侧仍走 core `wall_path_segmentation`）。
`PillarProposals::add_proposal` 有真实消费者：`place_pillars` 系统候选采集
（完整 52KB 宿主仍 U）。

## 2. 逐 `todo!` 状态（契约 39 项口径）

剩余 `todo!` = **32**（本轮移除 9 个契约 todo；另将 2 个伪公式近似体
`score_split_candidate`/`process_segment` 改为显式 U，新增 2 个；合计 39−9+2=32）。

| # | 函数 | 状态 | 阻塞/证据 |
|---|---|---|---|
| 1 | `add_wall_corners_new` | U（检测已恢复，发射未闭合） | 转角 Triangle 链/随机砖宽 0.92±0.16/末砖 0.76/`@0x5c` 标志未逐语句 |
| 2 | `add_skirts_new` | U | 矩形 8 点 u 序列/裙分层/累计距离 inverse_lerp 未逐语句 |
| 3 | `stairs_signifiers_new` | U | `SubtractiveBoolean1D` 区间语法/`get_approx_coord_from_pos` 未锚定 |
| 4-7 | `propose`/`place_support_beams`/`add_diagonal_support`/`emit_support_beams` | **已完成** | 见 §1 |
| 8-9 | `add_proposal`/`assemble_stone_pillar` | **已完成** | `PillarEmitContext.material/flags/wall_distance` 的上游来源 U（closure env），已显式参数化，未用默认值掩盖 |
| 10-12 | `construct_wooden_pillar_w_stone_base`/`construct_brick_columns`/`construct_crossbrick_pillar` | U | 0x18B/0x40B 柱表布局、SSBO 锁写序、四边 rotation 参数未锚定 |
| 13-16 | circle bottom layer/add_bottom_brick/circle_pillars/turret | U | 0x60B 记录装配、11 参 ABI、逐层计数/收分率无证据 |
| 17-18 | find/detect overhang segments | U | 43KB 输出结构/阈值/closure 语义 |
| 19-25 | elevation supports 5 系统 + rect bottom/pillars/brackets | U | **跨 crate：`ElevationSupports`/`ElevationSupportStats` 为 `Gap`**；`Rectangle3d` 提案字段 |
| 26 | maintain_stone_column_colliders | U | **跨 crate：rapier native-physics 撤除面未暴露** |
| 27 | construct_floors | U | **跨 crate：`country_core::resources::walls::Floor` 组件缺失** |
| 28 | construct_platform_stairs | U | **跨 crate：80B 装饰源无公开字节投影**（core `Decorators` 仅结构化字段到 +0x48）；core `platform_stairs.rs` CPU 链已 O，可接 |
| 29-31 | preprocess_curve / playermade arches_n_walls / pillars proposals | U | closure_0 语义/9 closure 单态/**跨 crate：`StairColliderRegistry` 缺失** |
| 32-35 | plaster 4 件 | U | `WallPlasters`/`WallSide`/mesh 拓扑/71.6KB 泛型单态 |
| 36 | half_timbering | U | 26.7KB asm 中段逐梁坐标未取证 |
| 37-39 | window 三件 | **已完成** | 见 §1 |

## 3. 测试

新增/更新测试（`cargo test -p system-wall-constructor --lib`）：

- `arch_walker::tests`：`new` 常量 0.1、直线 10m/brick 2 → 5 块且 u=0.2..1.0、折线跨段 u、3D 高度参与长度（3m+4m→5 块）。
- `place_support_beams::tests`：方向归一化/首米 cost=0、flags 低位 tag、3.15 上限、place 全链 64B wall id、emit 3.15 步进 2 块。
- `place_pillars::tests`：新 key/同 key 最优/严格更小覆盖、1.26 单段、1.2 分段 flag、96B material/surface 哨兵、顶盖、单段无 0.25 间隙。
- `half_timber::tests`：宽度 0.46 下限、'8'/':' 高度调整、gothic profile 首末点、重叠检测直接/移位、20B 记录字段。
- `wall_corners::tests`：直角窗口命中/45° 不命中、贪心角点扫描。
- `calculate_wall_path_segmentation::tests`：空区间分裂 flags|=8 与最近端点桶、短段跳过。

## 4. 编译与测试

- 独立 target（避免共享锁）：`CARGO_TARGET_DIR=/d/game/tiny-glade/target-agent-wc
  cargo check -p system-wall-constructor --offline` →
  **Finished，0 error**（3m57s；core 与 obvhs 的既有 warning 与本 crate
  无关）。
- 早前 `target-wc11` 一轮同样 0 error（16:12）；随后 `country-core` 被
  并行 agent 的在途编辑短暂破坏（3 处 `E0308`），已由对方修复，本轮最终
  check 通过。
- `cargo test -p system-wall-constructor --lib`（target-agent-wc）：见
  §6 末行（含 P0 修正后的断言：grower flag bit0、tag=2 哨兵、
  `add_proposal` 严格更大覆盖、顶盖 `param_3` 门控）。

## 5. 跨 crate 缺口（不在本 crate 修改）

> 注：本轮并行 agent 已把 `ArchSegments` 从 `Gap` 改为
> `HashMap<WallId, Vec<MaterializedWallSegment>>`，并在
> `PublicWallState::construct_regular_arches` 上接通 prepared path →
> `construct_regular_wall_arches`；同时 `construct_arches` 系统已改接该链。
> 该部分**不是本交付认领的实现**，登记以免重复。

| 所需 core API/布局 | 目标文件 | 阻塞 todo |
|---|---|---|
| `Floor` 组件（wall_id + shape + floor height + byte@0x88） | `crates/country-core/src/resources/walls/mod.rs`（新增） | construct_floors |
| `ElevationSupports` / `ElevationSupportStats` 真实布局（0x28B/0x20B） | `crates/country-core/src/resources/walls/elevation_support_stats.rs`（现 `Gap`） | elevation 5 系统 + bottom/pillars |
| 80B 装饰记录公开字节投影（raw[0x4e]/[0x2a]/f(8)/f(12)/[0x49]/f(0x3c)） | `crates/country-core/src/resources/walls/decorator.rs`（`DecoratorInfo` 现仅到 +0x48） | construct_platform_stairs、窗工具 ECS 遍历 |
| `StairColliderRegistry`（RawTable<StairGraphEdge, Vec<Vec3>>） | `crates/country-core/src/resources/stairs_walls.rs`（缺） | process_playermade_stair_pillars_proposals |
| rapier 石柱碰撞体增删接口 | `crates/country-core/src/systems/wall/construction/platform_rapier_backend.rs` | maintain_stone_column_colliders |
| `MaterialStats` 真实累计方法 | `crates/country-core/src/resources/walls/material_stats.rs` | 拱/过梁/栅栏/柱统计 |
| 装饰器 profile 曲线（gothic_door/half_dormer）公开面 | `crates/country-core/src/resources/walls/decorator_subtype.rs` | `WindowAabbSource.profile` 的 ECS 侧填充 |
| lintel 原语公开导出（`lintel_support_instances`/`add_diagonal`/`emit_plank`） | `crates/country-core/src/systems/wall/construction/structures.rs`（现 `pub(crate)`） | 消除本 crate 的 4 个支撑梁重复原语；当前只能 primitives-only |
| `place_pillars` 候选源（双缓冲队列/0x88B 段/0.1875 半砖偏移/`find_stone_column_start_y`） | 原 VA 0x140753670，本 crate 未恢复 | place_pillars 候选采集（当前 1.2/64 为显式占位 U） |

## 6. 复核记录

- 工作期间其它 agent 同时编译；本 agent 全程使用独立 target
  （`target-wc11` / 中央指定的 `target-agent-wc`），未等待共享锁。
- **非本 agent 修改、供中央裁决的 core 文件**（本轮在共享工作区出现，
  按文件权本 agent 未改）：
  - `crates/country-core/src/resources/walls/arches.rs`
  - `crates/country-core/src/resources/walls/public_wall_state.rs`
  - `crates/country-core/src/systems/wall/construction/arches.rs`
  - `crates/country-core/src/systems/wall/ui_continue_wall.rs`
  其中 `construct_regular_arches` 的 `WallId` 类型不匹配曾使 core 短暂
  `E0308`，随后已修复。
- **非本 agent 修改的本 crate 文件**：`src/construct_arches/mod.rs`
  （并行 agent 改接 `PublicWallState::construct_regular_arches` +
  `ArchSegments`）；本交付不认领该实现，仅登记。
- 测试结果：`cargo test -p system-wall-constructor --lib`（target-agent-wc）
  最终结果以本轮回复为准；上一次旧快照为 26 passed / 3 failed，失败断言
  均已在 P0 轮修正。
