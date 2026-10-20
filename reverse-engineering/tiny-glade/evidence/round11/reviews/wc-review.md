# WC 组第 11 轮复核（review）：`system-wall-constructor`

- 复核快照：2026-10-19 16:20（WC② 16:09 交付落盘后；期间仍在并发编译，见 §6）。
- 审查对象：契约 `evidence/round11/contracts/wc-constructor-CONTRACT.md`；交付
  `evidence/round11/deliveries/wc-constructor-DELIVERY.md`（16:09）；改动
  `crates/systems/wall-constructor/**`。
- 只读复核；未改 WC 源码。反编译证据直接核对
  `evidence/country-core-deep/systems-recovery/groupC/wall-constructor/decomp/**`
  与 `evidence/building-structures/**`。

## 结论

**不通过（当前快照）/ 有条件可继续。**

编译基线可过，交付也诚实保留了大部分 U 项与跨 crate 缺口；但本轮 9 个“移除 todo”
中有 **8 个没有生产消费者**，实际构造链仍是“首个墙变更即在 `construct_walls` panic”。
此外有 2 处**与反编译证据直接冲突的实现错误**（`add_proposal` 覆盖方向、
`propose` 的 flags bit0 内存来源），1 处门控变量与契约/反编译不一致
（`assemble_stone_pillar` 顶盖）。按契约 §5.1“真实输出 + 真实消费者 + 字节级对照”
的验收门，当前不能判为完成。

### 计数

- 契约 todo 39 → 当前 **32**（移除 9，新增 2：`process_segment`、`score_split_candidate`）。
- 9 个移除项中：**1** 个有消费者（`add_proposal`，但语义有错）；**8** 个无生产调用点
  （4 支撑梁、3 窗工具、`assemble_stone_pillar`）。
- 本轮确认水分/错误 **5** 项（§1 W1–W5）；预存空壳/占位 **10** 项（§1 P1–P10，
  不在本轮 diff，但仍在注册路径或被交付继承）。
- 跨 crate 阻断：`Floor`、`StairColliderRegistry`、`ElevationSupports/Stats`、
  `ArchSegments`、`CurvedHalfTimberDrawables`、`MaterialStats`、装饰器遍历/profile、
  rapier 石柱碰撞面。

---

## 1. 水分检查（`todo!` 是否被空体/默认值/伪造常量替换）

本轮没有“把 todo 改成空函数体”的新增项；`wall_constructor.rs::helpers` 从空体改为
委托到各自 todo（正确方向）。问题集中在“实现存在但无消费者”“语义与证据不符”：

| 编号 | 位置 | 问题 | 判定 |
|---|---|---|---|
| W1 | `place_pillars.rs:78` + 测试 `place_pillars.rs:335-346` | 反编译 0x1407534d0：`if new < old \|\| new == old return;`，即 **new > old 才替换**末提案；实现与交付/测试写成“new 严格更小才覆盖”，方向相反。`scores` 取 min 又是另一条路径（反编译确实取 min），代码把两者混为一谈。 | **确认错误（必须修）** |
| W2 | `construct_lintels/place_support_beams.rs:99` | 0x141fe09c0 的 bit0 来自 **grower 状态首字 `*in_RDX`**（见 `B-core/141fe09c0.c:46-50,62-74`），实现读的是 **输入点数组 `flags[index]&1`**。现有输入 flags 端点=8/内部=0，等价于几乎恒走“新方向”，cost 会系统性变成 `len*0.5`。 | **确认错误（必须修/报中央）** |
| W3 | `place_pillars.rs:233` | 反编译 0x14120d670 的 1.26 单段与 2.1 顶盖使用**同一个 `param_3`**（`decomp-0x14120d670.txt:121,183`）；契约写 `bottom+0.2>2.1`，实现用 `top+0.2`。两者必有一错，且交付未说明 param_3=bottom/top 的 ABI 依据。 | **未闭合（须取证）** |
| W4 | `place_pillars.rs:291-304` | 候选采样是占位：每 1.2m、`clamp(0,64)`、`score=p[1]`、index=曲线点号；与原件 0.1875 半砖偏移/0x88B 段/`find_stone_column_start_y` 不同。注释称“不产生未取证常量”“接到真实支柱原语”，但既用了未取证常量 1.2/64，也**没有调用 `assemble_stone_pillar`**。 | **确认水分（文档失实）** |
| W5 | 4 支撑梁 + 3 窗工具 + `assemble_stone_pillar` | 全部只有本文件/测试调用；生产链 `construct_lintels` 走 core `construct_lintel_supports`，窗工具无 ECS 消费者。交付 §1 标记“已完成”，未列消费者，违反契约 §5.1.4。 | **确认水分（交付口径）** |
| W6 | `place_support_beams.rs:42-50` | 注释称 40B 原件投影，但反编译把 tag 写在 +0x28（`141fe09c0.c:87`），`observed_bytes` 未投影 tag（0/1/2；2=无效哨兵）。 | 轻微（布局说明失实） |

预存空壳/占位（仍被交付继承；不在本轮 diff）：

| 编号 | 位置 | 现状 |
|---|---|---|
| P1 | `wall_constructor.rs:46-72` | `from_curve_resolved` 计算 rows/trim/normalize 后全部丢弃，**返回空 Vec**。`construct_walls` 的 96B 输出因此恒空。 |
| P2 | `construct_fences/process_segment.rs:12-58` | `emit_fences` 仅把位置写进 64B 前 12B，`rng_seed`/`spacing` 被 `let _` 丢弃；`process_fence_segment` 不产 plank/柱/rail。 |
| P3 | `construct_gates/systems.rs:133-155` | 136B door spawn payload 只填 scale/top[0]/wall/dirs/seed/29a；变换平移、mesh、shader 句柄为 0。 |
| P4 | `construct_arches/mod.rs:109-117` | 120B `ConstructGateCmd` 只填 wall/ordinal（16B），其余 104B 全 0；`segment_count` 被丢弃。 |
| P5 | `curved_halftimber/mod.rs:20-33` | 注册系统读事件后 `let _`，无任何实体/组件变化（无 todo，静默空转）。 |
| P6 | `construct_playermade_stairs/{brackets,magic_support,pillars_visual}.rs` | 注册系统仅 `let _` 丢弃输入与输出。 |
| P7 | `construct_elevation_supports/mod.rs:32` | `overhang_subscribe_to_tabloids()` 空体（非 GPU 项，应实现或显式 todo）。 |
| P8 | `utils/half_timber.rs:47,252` | `roof_outline` / `add_underwindow_pattern` 空体（文档写 U，但无 todo/panic，调用方无法察觉）。 |
| P9 | `construct_arches/profile.rs:15-51` | `create_stone_arch_profile` 仍为近似：重采样数 `segment_count.clamp(2,64)`，上限 64 无证据；契约 §5.3 要求校正。 |
| P10 | `country-core/.../material_stats.rs:15-23` | `MaterialStats` 方法为空 stub，却被标 O；拱/过梁/栅栏/柱统计全部丢失。 |

GPU 两个壳（`lib.rs:83-87`）仍为空，但交付/注释明确标为渲染边界后置，不计水分；
`debug_force_update_arches` 为 B 版 1B `ret`，空体判等。

---

## 2. 契约 39 处 `todo!` 逐项现状

> 行号为当前源码。状态口径：**仍 todo** / **部分**（有实现但无消费者或语义未闭合）/
> **真实**（实现 + 生产消费者 + 证据） / **移走**（由 core 等价实现承担）。

| # | 函数 | 状态 | 说明（文件:行 / 证据） |
|---|---|---|---|
| 1 | `add_wall_corners_new` | 仍 todo | `utils/wall_corners.rs:51`；`check_angle/find_corners` 已实现但仅测试调用，转角砖几何未闭合 |
| 2 | `add_skirts_new` | 仍 todo | `utils/wall_skirts.rs:16` |
| 3 | `stairs_signifiers_new` | 仍 todo | `utils/stairs_signifiers.rs:17` |
| 4 | `SupportBeamGrower::propose` | 部分 | `place_support_beams.rs:78-117`；无消费者；W2 读错 flags |
| 5 | `place_support_beams` | 部分 | 同文件 404-504；无消费者；生产由 core `construct_lintel_supports` 承担 |
| 6 | `add_diagonal_support` | 部分 | 同文件 316-370；与 core O 同构；无消费者 |
| 7 | `emit_support_beams` | 部分 | 同文件 511-550；同上 |
| 8 | `PillarProposals::add_proposal` | 部分 | `place_pillars.rs:59-93`；有消费者 `place_pillars`，但 W1 比较方向反 |
| 9 | `assemble_stone_pillar` | 部分 | `place_pillars.rs:155-240`；无消费者；W3 顶盖门控；`PillarEmitContext` 上游 U |
| 10 | `construct_wooden_pillar_w_stone_base` | 仍 todo | `util_pillar_construction.rs:21` |
| 11 | `construct_brick_columns` | 仍 todo | 同文件 30 |
| 12 | `construct_crossbrick_pillar` | 仍 todo | 同文件 39 |
| 13 | `construct_circle_bottom_layer` | 仍 todo | `circle_bottom_layer.rs:49` |
| 14 | `add_bottom_brick` | 仍 todo | 同文件 39 |
| 15 | `construct_circle_pillars` | 仍 todo | `circle_pillars.rs:39` |
| 16 | `construct_turret` | 仍 todo | `circle_turret.rs:20` |
| 17 | `find_overhang_support_segments` | 仍 todo | `find_overlap_segments.rs:33` |
| 18 | `detect_overhang_support_segments` | 仍 todo | `construct_elevation_supports/mod.rs:44` |
| 19 | `construct_circle_supports` | 仍 todo | 同文件 71（`ElevationSupports` 为 `Gap`） |
| 20 | `construct_rectangle_columns` | 仍 todo | 同文件 81 |
| 21 | `construct_rectangle_brackets` | 仍 todo | 同文件 89 |
| 22 | `construct_rect_bottom_layer` | 仍 todo | `rectangle_bottom_layer.rs:18` |
| 23 | `construct_rectangle_pillars` | 仍 todo | `rectangle_pillars.rs:128` |
| 24 | `construct_rectangle_small_brackets` | 仍 todo | `rectangle_small_brackets.rs:14` |
| 25 | `construct_rectangle_large_brackets` | 仍 todo | `rectangle_large_brackets.rs:13` |
| 26 | `maintain_stone_column_colliders` | 仍 todo | `maintain_stone_column_colliders.rs:26`；rapier 面未暴露 |
| 27 | `construct_floors` | 仍 todo | `construct_floors.rs:63`；`Floor` 组件缺失 |
| 28 | `construct_platform_stairs` | 仍 todo | `construct_platform_stairs/mod.rs:38`；core `platform_*` CPU 链已 O，可接 |
| 29 | `preprocess_curve` | 仍 todo | `construct_playermade_stairs/mod.rs:91` |
| 30 | `arches_n_walls::<true>` | 仍 todo | 同文件 105 |
| 31 | `process_playermade_stair_pillars_proposals` | 仍 todo | `pillars_process_proposals.rs:142`；`StairColliderRegistry` 缺失 |
| 32 | `maintain_plaster` | 仍 todo | `plaster_systems.rs:72` |
| 33 | `create_plaster` | 仍 todo | 同文件 84 |
| 34 | `split_wall_into_sides` | 仍 todo | 同文件 91 |
| 35 | `generate_plaster_mesh_and_entities` | 仍 todo | 同文件 101 |
| 36 | `half_timbering` | 仍 todo | `utils/half_timber.rs:38` |
| 37 | `overlapping_windows` | 部分 | `utils/half_timber.rs`（几何实现 + 测试）；无 ECS 消费者，SIMD 过滤由调用方承担 |
| 38 | `get_window_aabb` | 部分 | 同文件；门类 profile 由调用方给出；无消费者 |
| 39 | `check_for_overlap_with_all_windows` | 部分 | 同文件；无消费者 |

额外（契约外）：
- `process_segment`（`segmentation/calculate_wall_path_segmentation.rs:52`）已从伪
  `[0.0;4]` **降级为显式 todo**（诚实，但计入 32）。
- `score_split_candidate`（同文件 146）由近似公式 **降级为 todo**（诚实）。
- `add_splits_in_empty_range`（同文件 76）已实现且带测试，但**无调用点**。
- `ArchFunction::remap_t`（`profile.rs:70`）逐语句对齐反编译，但**无调用点**。
- `check_angle/find_corners`（`wall_corners.rs`）有测试，无生产调用。
- 原作本就空：无（本 39 项均非空实现）；GPU 两壳与 `debug_force_update_arches`
  在 39 之外，后置/1B ret 的判定合理。

---

## 3. 构造顺序与阶段输入/输出

`lib.rs:38-78` 注册顺序与契约一致：

- **Edit（4）**：init → calculate segmentation → sample intersections → split platform stairs。
- **Bricks（14 个 add_systems 项）**：construct_walls → arches → lintels → fences →
  construct_gates → update_gates_hashmap → place_pillars → curved_halftimber →
  platform_stairs → playermade arches → pillars visual → circle supports →
  rectangle columns → rectangle brackets（B5 两项合计对应 B1–B13）。
- **Holes（2）**：maintain_plaster → create_plaster。

但“有序注册”不等于“可运行链”：

1. `add_wall_constructor_systems` 在 workspace 内**没有任何调用点**（只在本文件导出）。
2. 注册系统所需的 `WallConstructorOutput` / `WallPathSegmentationMasks` /
   `WallPathSegmentationMasksMinusStairs` / `PillarProposals` / `PlatformStairsStorage`
   均**无 `init_resource`/`insert_resource`**；一旦调度即因缺资源 panic。
3. 首个 Bricks 系统 `construct_walls` 在任意 `OnWallChanged` 上调用
   `utils::wall_corners::add_wall_corners_new`（todo）→ **第一个墙事件即 panic**，
   B2–B13 实际不可达。
4. 各阶段真实输出核查：
   - E1：只调 `update_gable_heights`，缺两对 `{count,height}` 与高度数组（部分）。
   - E2：调用 core `wall_path_segmentation`，确产 `Mask`；输入用简化曲线代替
     path-preparation（I）。
   - E3/E4：空转（`let _ = curve.length` / `let _ = id`），无交点对/平台段输出。
   - B1：panic 前无输出；`from_curve_resolved` 本身返回空 Vec。
   - B2/B3/B4/B5：core 链能产 96B/64B 记录并写入 `WallConstructorOutput`，
     但 gate/spawn payload 只有部分字段；且被 B1 panic 阻断。
   - B6：只累积 proposal，不产 96B；无 `assemble_stone_pillar` 调用。
   - B7/B10：静默空转。
   - B8/B9/B11/B12/B13：todo panic。
   - H1/H2：todo panic。
   结论：**没有任何一条端到端真实输入→真实输出链可运行**。

---

## 4. 算法正确性抽查（对照契约常量/公式与反编译）

| 函数 | 结论 | 细节 |
|---|---|---|
| `ArchWalker::walk` 0x141232bb0 | 部分 | total/n/step/residual/tail 与反编译一致；但 `u` 只用普通 lerp+fract（`arch_walker.rs:141-156`），缺 core O 的 `periodic_delta`（0.5 跨缝折返），且把精确 1.0 固定为 1.0；与 `country-core/.../arches.rs:186-188` 不一致。归档为“逐指令重建”过头。 |
| `SupportBeamGrower::propose` 0x141fe09c0 | 错误 | 方向/长度/cost 公式、3.15 上限、point/index/dir/curvature 字段对齐；**bit0 来源错误**（W2）；tag 只建模 0/1，丢失反编译的 2=无效哨兵。 |
| `place_support_beams` 0x141242530 | 部分 | 循环、consume、join、0.85/0.5 对角与 core O 同构；但依赖错误 propose，且无消费者。 |
| `add_diagonal_support` / `emit_support_beams` | 真实但重复 | 与 core `structures.rs:502-554/375-463` 逐行相同（-0.225、0.2625、0.3375、0.1125+0.1875）；契约 §4.1 要求复用而非复制，且现无调用点。 |
| `PillarProposals::add_proposal` 0x1407534d0 | 错误 | 三向量 push、score NaN-aware min、整条 28B 覆盖顺序均对；**覆盖判据反向**（W1）。 |
| `assemble_stone_pillar` 0x14120d670 | 部分 | 分段/0.2/1.26/0.25/0.56/max(3,round)/0.28 jitter/0.01 ε/0.42±0.05/0.7±0.05/末段 0.7/0.84 与反编译一致；顶盖门控用 top（W3）；`PillarEmitContext`、`seed as u32+emitted` 仍 U。 |
| `ArchFunction::remap_t` 0x140765300 | 真实但无消费者 | mode0/镜像/k=clamp(inverse_lerp(2,1,blend))/inner/outer 逐式对齐（`decomp-0x140765300.txt`）；无调用点。 |
| `get_window_aabb` 0x141221da0 | 部分（边界） | 普通分支 width=max(bbx0+0.2,0.46)、height 差、'8' +0.2、':' +0.4/−0.2 对齐；门分支首末点+0.093 对齐；gothic profile 选择（`\t` vs `raw[0x44]`）交给调用方，属跨 crate 边界，当前无消费者。 |
| `overlapping_windows` / `check_for_overlap...` | 部分（边界） | 0.038 内收、dormer ±0.14、直接 intersects/contains、移位再测、20B 记录对齐；SIMD/subtype 过滤与 profile 供给未接线；移位条件与反编译的 `center+half*param` 结构需字节级复核。 |
| `add_splits_in_empty_range` 0x140751240 | 真实但无消费者 | floor(len/spacing)、n==1&&len<spacing*1.5→0、coord_at_u、flags|=8、最近端点桶对齐；无调用点。 |
| 其他抽查 | 仍近似 | `create_stone_arch_profile` 的 `clamp(2,64)`（P9）；`process_segment` 已降级 todo；`emit_fences` 12B 占位（P2）；门/拱 payload 部分零（P3/P4）。 |

---

## 5. 跨 crate 缺口（交付是否诚实登记）

交付 §5 列了 7 项，方向正确，且 **没有**在本 crate 伪造 core 布局。逐项复核：

| 需求 | 交付登记 | 复核 |
|---|---|---|
| `Floor` 组件 | 是 | 全 workspace 确实无 `struct Floor`（`construct_floors.rs:63` todo） |
| `ElevationSupports/ElevationSupportStats` | 是 | core 为 `Gap`（`elevation_support_stats.rs:12,16`） |
| 80B 装饰记录字节投影 | 是 | `DecoratorInfo` 不足；影响 platform stairs/窗 ECS 遍历 |
| `StairColliderRegistry` | 是 | 全 workspace 缺失（`pillars_process_proposals.rs:142`） |
| rapier 石柱碰撞面 | 是 | 未闭合 |
| `MaterialStats` 真实累计 | 是 | core 方法为空 stub（P10） |
| gothic profile 曲线公开面 | 是 | 未落库；窗工具依赖调用方 |
| `ArchSegments` / `CurvedHalfTimberDrawables` | 部分 | 交付 §5 未单列；契约 §4.2 要求，二者仍为 `Gap`，阻塞 UI/curved halftimber |
| 装饰器 SIMD 遍历/过滤 | **否** | 窗工具把过滤推给调用方，但调用方 API 尚不存在；交付未列入缺口 |

---

## 6. 编译与测试（实际输出）

- `cargo check -p system-wall-constructor`（15:19 快照，共享 target）：
  **exit 0**，`Finished dev profile`，7 warnings（`--lib generated 7 warnings`）。
- `CARGO_TARGET_DIR=target-review cargo check -p system-wall-constructor`
  （16:05–16:08 快照，含 16:07 前改动）：**exit 0**，7 warnings，2m34s。
- 16:10 之后针对 16:09 最终编辑的重跑：受多个 agent 共享 target 锁阻塞，
  复核截止时仍在 country-core 编译阶段，**未见 error**，但**未完整结束**。
  因此本报告不宣称“最终快照编译已验”，只记录两次已完成的 0 error。
- `cargo check -p tiny-glade`：共享 `target` 锁被其它 agent 长时间占用，
  复核截止未完成，**无实际输出**，不能据此宣称“无回退”。
- `cargo test -p system-wall-constructor --lib`（16:17，target-review）：
  **26 passed / 3 failed / exit 101**。失败项：
  - `construct_arches::profile::tests::mode_zero_is_mirrored_cubic`
  - `utils::half_timber::tests::overlap_check_detects_inside_and_shifted_points`
  - `utils::half_timber::tests::overlapping_windows_reports_shifted_flag`
  失败断言来自 16:09:38/16:09:50 编辑前的测试文本（当前源码已把期望改为
  `(0.5,0)`/点 1.2/expansion 2.0，与实现一致）；该次二进制为中间快照，
  **最终测试重跑未完成**。即：不能把“测试全绿”写入交付结论。

---

## 7. 缺口继承与交付诚实性

交付（16:09）总体比旧版诚实：明确保留 32 todo，逐类列证据缺口与跨 crate 需求，
没有把 GPU 后置项称完成。需要纠正/补记：

1. §1 把 8 个无生产消费者的函数标“已完成”；契约要求“真实消费者”。
2. §1 #6 的 `add_proposal` 语义与反编译相反（W1）。
3. §1 #7 写 `top+0.2>2.1`，契约写 `bottom+0.2`；未给 param 映射证据（W3）。
4. §1 #2 的 flags bit0 未说明来自 grower 状态（W2）。
5. 未登记：调度入口无调用点、5 个资源无 init、`construct_walls` 首事件 panic。
6. 未登记：`from_curve_resolved` 空返回、fences/gates/arches 的 `let _` 占位（P1–P4）。
7. §2 计数 32 与当前源码一致（39−9+2），但应注明 8 个“完成”项无消费者。

---

## 8. 必须修复清单（按优先级）

1. **[P0]** `add_proposal` 覆判据改为 `new_score > last.score()` 才替换；
   同步修正测试与交付说明（反编译 `0x1407534d0`）。
2. **[P0]** `propose` 的 bit0 改读 grower 状态（方向有效位），不是输入点
   `flags[index]`；补 tag=2 无效哨兵语义；或报中央修正 core `propose` 后复用。
3. **[P0]** `assemble_stone_pillar`：按 0x14120d670 的 `param_3/param_4` ABI 证明
   bottom/top；顶盖门控与 1.26 必须使用同一参数（契约/交付统一）。
4. **[P0]** 为 8 个无消费者实现接线或降级：支撑梁优先“复用 core + 删除重复”；
   `assemble_stone_pillar` 必须在 `place_pillars` 中真实调用并产 96B；
   窗工具需接装饰器遍历；`remap_t/add_splits` 需进真实剖面/分段链。
5. **[P0]** 调度：调用 `add_wall_constructor_systems` 的宿主或明确“未接 app”；
   `init_resource` 5 个资源；消除 `construct_walls` 首事件 panic（至少 corner/skirt/
   signifier 的 U 要显式隔离，不能阻断整条 Bricks 链）。
6. **[P1]** 清除/降级占位：`from_curve_resolved` 空返回、fences 12B、gate/spawn
   payload 零字段、arches 16/120 payload、curved_halftimber 与 playermade 空转。
   若暂不实现，改显式 todo 并登记，不得静默成功。
7. **[P1]** 增加证据驱动的字节级测试：96B BrickTransform（拱/柱）、64B plank
   （支撑梁/栅栏）、120B GateCmd、136B spawn，对照 `evidence/building-*` 冻结样本；
   现有测试只验证结构/计数。
8. **[P2]** 交付补“消费者”列、最终编译/测试原始输出与快照时间；删除
   “逐指令重建/已完成”中证据不足的表述。

---

## 9. 给①（契约/中央）的下一步方向

1. **把反编译事实写回契约**：propose 的 bit0 属 grower 状态；proposal tag 0/1/2；
   add_proposal `>` 覆盖；assemble param_3/param_4 的 bottom/top 与 2.1 门控；
   ArchWalker `wall_space::subtract + project_clamp` 的周期语义（与 core
   `periodic_delta/project_period` 对齐）。
2. **验收口径加“消费者/调用图”列**：每个“完成”必须给出生产调用点与落盘路径；
   孤立函数一律记 U 或“primitives-only”，不得计入 todo 减少的进度。
3. **确定 core/WC 边界**：支撑梁/过梁与 arches 已有 core O 链；优先修 core
   `propose` 的 bit0 并公开 `lintel_support_instances`，删除 WC 500 行重复实现，
   避免两套语义继续漂移。
4. **先打通一条端到端**：`construct_walls`（角/裙/指示符至少不 panic）→
   `from_curve_resolved` 真实 96B → `WallConstructorOutput`/SSBO；再按
   gates→fences→arches→pillars→elevation→stairs→plaster 逐段替换 todo。
5. **中央批量落跨 crate 合同**：`Floor`、`StairColliderRegistry`、
   `ElevationSupports/Stats`、`ArchSegments`、`CurvedHalfTimberDrawables`、
   `MaterialStats`、装饰器遍历/profile 曲线、rapier 石柱面。
6. **并发编译纪律**：每个 agent 固定独立 `CARGO_TARGET_DIR`，并在交付中记录
   该目录下的最终 `cargo check/test` 输出，避免共享锁导致“未验证”收口。
