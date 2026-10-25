# WC 组 ② 可执行契约：`system-wall-constructor`

> 第 11 轮（2026-10-19）逆向取证产出。只读源码/证据；不改源码。
> 目标 crate：`D:/game/tiny-glade/crates/systems/wall-constructor`（53 文件 / 3,170 行）。
> 骨架：`D:/game/reverse-engineering/tiny-glade/skeleton/B/crates/systems/wall-constructor/src/**`（PDB 元数据，含 VA/签名状态/依赖注解）。
> 反编译/字节入口：`node scripts/mcp-job.mjs re_mcp_ghidra <job.json> <out.json>`（`programPath="/tiny-glade.exe"`）、`python scripts/rne-read.py`。
> 已有证据基线：`upper-modules-plan/systems/wall-constructor/ANALYSIS.md`、`BATCH3-LOGIC-REVIEW-wall-constructor.md`、`evidence/building-*` 约 30 目录。

## 0. 置信度与计数口径

- **O**：已恢复且有字节/案例验证，可直接复用（不含本 crate 内 `todo!`）。
- **I**：已反编译/分析，控制流/常量已定位但未逐语句重建。
- **U**：骨架存在但反编译/类型/宿主未闭合。
- 计数：crate 内 `todo!(` 调用 **39 处**（另有 3 处仅文档提及，文本共 42 处）。上层扫描“37 todo”为口径差异（`add_wall_corners_new`/`add_skirts_new` 可能被计为 wrapper），本契约以 39 为准。
- 骨架顶层业务符号 133 个；**37 个在 crate 源码中无文本痕迹**；其中 9 个已被 country-core 等价吸收，**28 个为真实本地缺口**（与上层扫描“28 无迹象”一致）。见 §3。
- 当前基线（2026-10-19 复核）：`cargo check -p system-wall-constructor` **通过**（exit 0，8 warnings，2m37s）。所有 `todo!` 均为运行期 panic 点，不阻塞编译。

---

## 1. 构造调用顺序（原版墙从形状/曲线到砖、拱、过梁、栅栏、门、平台楼梯、柱子、灰泥、半木）

### 1.1 调度阶段（O：`src/lib.rs:29-78` 按原 `WallCpuStage` 注册）

| 阶段 | 顺序 | 系统 | VA | 输入权威 | 输出 |
|---|---|---|---|---|---|
| `Edit` | E1 | `init_wall_height_and_arch_segment_height` | 0x14078b550 | `EventReader<OnWallChanged>`, `ResMut<PublicWalls>`, `Query<&Roof>` | 墙 0x2a8 记录内两对 `{count,height}` + 高度数组 |
| `Edit` | E2 | `calculate_wall_path_segmentation` | 0x14074f310 | `OnWallChanged`, `PublicWalls`, `UiSignifierStream` | `WallPathSegmentationMasks`（每墙 136B mask Vec） |
| `Edit` | E3 | `sample_wall_path_intersections` | 0x14074c720 | `PathRastersResources`(未落库), `PublicWalls`, `OnWallChanged` | 路径光栅×墙轮廓交点对/重建 Curve3 |
| `Edit` | E4 | `split_wall_path_segment_into_platform_stairs` | 0x140751960 | `OnWallChanged`/`CreateRoofCmd`/`SwitchRoofTypeCmd`, `PublicWalls`, `Query<&Roof>` | `HashMap<WallId, Vec<Mask>>` + 0x20B 对 + `DecoratorId` |
| `Bricks` | B1 | `construct_walls` | 0x140757800 | `OnWallChanged`, `PublicWalls`, `AssetSsboLibrary`, `Query<&Roof>`, `TerrainHeightsData` | 96B 砖 + 角/裙边/指示符 + `InstancedWall`/`WallHoleStorage` 写面 |
| `Bricks` | B2 | `construct_arches` | 0x14075f680 | mask, `PublicWalls`, `Roof`, SSBO | 96B 拱砖 + 120B `ConstructGateCmd` + `MaterialStats` |
| `Bricks` | B3 | `construct_lintels` | 0x14076d9f0 | `WallPathSegmentationMasksMinusStairs`, `PublicWalls`, mask | 64B 木板（支撑梁）+ third 高度回写 |
| `Bricks` | B4 | `construct_fences` | 0x140765830 | `OnWallChanged`, `PublicWalls`, SSBO | 64B 木板/柱 + `MaterialStats` |
| `Bricks` | B5 | `construct_gates` / `update_gates_hashmap` | 0x140756370 / 0x140755690 | `ConstructGateCmd`(120B+8B), `Gates`, `AssetMeshLibrary`, `TerrainHeightsData` | 96B 支撑砖 + 136B 门扇 spawn + `Gates` 表 |
| `Bricks` | B6 | `place_pillars` | 0x140753670 | `OnWallChanged`, `RecalculateWallBricks`, `CreateRoofCmd`, `PublicWalls` | 96B 石柱/64B 木柱 + `MaterialStats` |
| `Bricks` | B7 | `curved_halftimber_maintain_entities` | 0x1407706d0 | `PublicWalls`, `OnWallDeleted`, `DeleteRoofCmd`, `CurvedHalfTimberDrawables` | 半木构实体 despawn/spawn 同步 |
| `Bricks` | B8 | `construct_platform_stairs` | 0x1407744d0 | `WallHoles`, `PlatformStairsStorage`, `PublicWalls`, SSBO | 96B 砖 + holes + cells/碰撞 + stats |
| `Bricks` | B9 | `construct_playermade_stairs_arches_n_walls::<true>` | 0x140496800 | `StairsArchSupportRequests`, `TerrainHeightsData`, `StairsPillarProposals` | 96B 拱/墙砖 + 44B 支柱提议 |
| `Bricks` | B10 | `construct_playermade_stair_pillars_visuals` | 0x14077d520 | `StairsPillarProposals`, `StairsVisualState`, terrain | 96B 支柱实例 |
| `Bricks` | B11 | `construct_circle_supports` | 0x140788160 | `RecalculateCircleSupports`, `PublicWalls` | `ElevationSupports` + 石柱碰撞 |
| `Bricks` | B12 | `construct_rectangle_columns` | 0x140788a80 | 同上（矩形） | `ElevationSupports` + 石柱 |
| `Bricks` | B13 | `construct_rectangle_brackets` | 0x1407896b0 | 同上 | 小/大托架 96B/64B |
| `Holes` | H1 | `maintain_plaster` | 0x14075b810 | `WallPlasters`, `PublicWalls`, `WallHoles`, `Roof`, 三类 plaster 命令 | 抹灰实体生命周期 + mesh |
| `Holes` | H2 | `create_plaster` | 0x14075d460 | `WallPlasters`, `PublicWalls`, `CreatePlasterCmd` | 抹灰 mesh/实体 |
| GPU | — | `gpu_clear_wall_gate_pieces` | 0x140773c00 | SSBO 注册表 + `Gates` | `SparseInstanceBufferWriter::clear<WoodenPlankSsbo>`（后置） |
| GPU | — | `gpu_flush_plank_buffers` | 0x140774010 | SSBO | `gpu_flush<WoodenPlankSsbo>/<SparkleSsbo>`（后置） |

> 调度顺序按 `lib.rs` 注释与 `WallCpuStage` 集合为 **O**；原版各 stage 内真实集合序为 **I**（`chain()` 顺序来自库登记，未被机器码逐条验证）。

### 1.2 数据流总链（形状/曲线 → 输出）

```
InnerWalls / PublicWalls(权威 0x2a8 记录)
  └─ E1 wall height init: 两对 {count,height} + 高度数组(-2.5/-0.46/0.96/0.5/0.28/0.95)
  └─ E2 segmentation: prepared 点 + path values + shape + special
        → 136B WallPathSegmentationMask(flags/coords/loop range/length)
  └─ E3 intersection sampling: 4×512² 光栅(u8→u16*257) + 衰减/平滑 → 交点对/Curve3
  └─ E4 platform-stair segment split: threshold_range 切段 → mask+0x20B 对
  └─ B1 construct_walls:
        WallConstructor::from_curve(0x140769760)
          rows(split_into_random_rows .46/.046) → trim_rows → perturb_row_ends(1.99)
          → normalize_holes → select_native_row_holes → consume_wall_proposal
          → resolve_hole_overlap → cut_wall_bricks → emit 96B BrickTransformSsbo
        + add_wall_corners(0x141215540) / add_skirts(0x1412221f0)
        + stairs_signifiers_new(0x141223d10) / half_timbering(0x141218e30) / roof_outline(0x141217be0)
        → InstancedWall new/update, WallHoleStorage finalize, WallTriggerVolume::recompute
  └─ B2 arches: mask → regular(shape 0/1)/freehand(shape 2)
        → create_stone_arch_profile(0x1407645f0)+remap_t(0x140765300)
        → ArchWalker::walk<closure$3>(0x141232bb0) → 96B 拱砖
        → has_roof&&single → 120B ConstructGateCmd
  └─ B3 lintels: mask → materialize_wall_segments(104B) → prepare_lintel_support
        → SupportBeamGrower propose/accept/consume (cost=len*(‖Δdir‖+.5), ≤3.15)
        → add_diagonal_support / emit_support_beams(ArchWalker step 3.15)
        → 64B WoodenPlankSsbo + third=max(heights-.2)
  └─ B4 fences: Pub296==1 → split_curve_based_on_angle(40°) → process_segment
        (perchance, spacing .7→0x3fce147a, post -.1Y) → emit_fences → 64B
  └─ B5 gates: ConstructGateCmd → construct_wooden_gate_doors(0x141210110)
        → 96B 支撑砖 + DoorMeshInProgress::add_quad(6 顶点/quad) → into_mesh
        → 136B spawn(transform16..56, mesh64, shader72, top80..104, WallId104,
          dirs112..128, seed128, 29a132) + Gates 表
  └─ B6 pillars: 0x88B 提案 → LoopRange/CurveSlice → assemble_stone_pillar
        (1.26 分段/0.56 层/0.25 间隙/随机砖宽) 或 lintel 木柱 → 96B/64B
  └─ B8 platform stairs: 80B 装饰源 → blockers → loop range → cells → 96B/holes
  └─ B9-B10 playermade: preprocess_curve → ArchWalker walk + raycast
        → StairsPillarProposals 44B → merge(<0.01) → 96B
  └─ B11-B13 elevation supports: circle/rect → circle_categorize / bottom layer /
        pillars / turret / brackets → ElevationSupports + 96B/64B
  └─ H1-H2 plaster: holes_no_duplicates → split_wall_into_sides → mesh 实体
```

---

## 2. 39 个 `todo!` 逐个契约

> 行号为当前 crate 源码行；VA 为 B 版文件内 VA；`骨架` 为对应 PDB 文件。
> “缺证”仅列阻碍逐语句重建的项；其余常量/公式可直接按表实现。

### 2.1 墙角 / 裙边 / 阶梯指示符（B1 内、GPU 前）

| # | 函数 | 位置 | VA | 签名 | 返回 | 依赖 | 常量/公式（可复现） | 缺证 | 级别 |
|---|---|---|---|---|---|---|---|---|---|
| 1 | `add_wall_corners_new` | `utils/wall_corners.rs:14` | 0x141215540（closure 0x141217a20） | `(points:&[Vec3], out:&mut Vec<[u8;96]>)` | 追加 96B 角砖 | 相邻点 `get_point_u`、`Curve` 采样、`Triangle::new` 链、`find_corners`(inlinee 70916)、`check_angle`(inlinee 70917)、`resolve_hole_overlap` | 夹角阈值 ±10°：−0.1736482 / +0.17364822；随机砖宽 0.92±0.16；末砖 0.76；aabb 合并；96B seed/index/@0x5c 标志 | 逐点遍历/闭包序、`check_angle` 判据、@0x5c 枚举 | I |
| 2 | `add_skirts_new` | `utils/wall_skirts.rs:16` | 0x1412221f0 | 同上 | 追加 96B 裙边砖 | `split_height_into_skirts`、`Curve::get_coord_at_u`、`Aabb2::from_center_dims`、`holes_no_duplicates`、`resolve_hole_overlap`、`WallSpaceCoord::to_world_space` | 裙阈值 `wall_y+3.5`；行距 0.69；每裙 `ceil((u1-u0)*h/0.69)+random_splits_into`；累计 `abs` 距离 `inverse_lerp`；@0x54/58=(中点±半宽)*h | 矩形 8 点/非矩形 [0,1] 分支精确 u、裙分层规则 | I |
| 3 | `stairs_signifiers_new` | `utils/stairs_signifiers.rs:17` | 0x141223d10（704 行） | `(wall:u64, points:&[Vec3], out:&mut Vec<[u8;96]>)` | 追加 96B 踏板 | `get_point`/`get_approx_coord_from_pos`、`tuple_windows`、`TerrainHeightsData::sample_at_world_xz`、`SubtractiveBoolean1D::new_range/subtract`、`holes_no_duplicates` | 楼梯段判据 `y-terrain<0.5 && max_y-y>=3.0`；HoleType 过滤 {0,1,3,4}；每阶 `ceil(seg_len*h/0.69)+random_splits_into`；顶高 `max_y-0.79-0.1-0.3`；@0x54 两行 u、@0x5c=4；2 阶循环；常量 0.69 | SubtractiveBoolean1D 区间语法、`get_approx_coord_from_pos` 精确定义 | I |

**验收门**：`cargo check -p system-wall-constructor`；96B 记录字段序与 `evidence/building-structures` 样本逐字节；消费者 = `construct_walls` 的 SSBO/`InstancedWall` 写面（GPU 上传后置，CPU 记录必须可产）。

### 2.2 柱 / 支撑梁 / 立面支撑（B3、B6、B11-B13）

| # | 函数 | 位置 | VA | 签名 | 返回 | 依赖 | 常量/公式 | 缺证 | 级别 |
|---|---|---|---|---|---|---|---|---|---|
| 4 | `SupportBeamGrower::propose` | `construct_lintels/place_support_beams.rs:41` | 0x141fe09c0（463B） | `(&mut self, point:Vec3, dir:Vec3, index:usize)` | 写 proposal 40B | flags | `flags[next]&0xf!=0 → stopped`；`dir=normalize(点差)`；已有方向投影；`cost=len*(‖Δdir‖+0.5)`（len≥1）；index@0/XZlen@8/dir@12/curvature@24/point@28/tag@40 | 已有方向的存储与投影公式、join 语义 | O（ANALYSIS 与重构公式一致） |
| 5 | `place_support_beams` | 同文件:53 | 0x141242530 | `(&mut Grower, &[[f32;3]], &mut Vec<[u8;64]>)` | 64B 木板 | 双 Grower | 0/end 向中 propose，cost 小者 consume；len≥1.0 发射/≤3.15/join；分开各 `len*0.85` 对角；join 且 total≥1.4175 双侧 0.5 | 双游标终止条件、join 结构 | I |
| 6 | `add_diagonal_support` | 同文件:65 | 0x141243740 | `(&[[f32;3]], &[[f32;3]], &[f32], &mut Vec<[u8;64]>)` | 64B | `extend_xz`、arch 高度采样、48B packer | `extend_xz` 到 (index,fraction)；锚高 `sample_height-0.2`；方向=采样点差(Δy−0.2)；轴 side×0.2 / dir×(len−0.2) / third×0.2 | 48B packer 字段 | I |
| 7 | `emit_support_beams` | 同文件:75 | 0x141243c20 | `(&[Vec3], f32, &mut Vec<[u8;64]>)` | 64B | `rdp_support_indices`(core O)、`walk_lintel`(core O) | ArchWalker step=0x40499999(≈3.15)、tail .1；plank 在 height−.225；x 厚 .225、z 厚 0x3eaccccc(≈0.3375)；斜撑预算每侧 len*0.85、join whole*0.5；水平扩展 length+0.1125+0.1875 | closure 索引对绑定 | I |
| 8 | `place_pillars` 内 `add_proposal` | `place_pillars.rs:38` | 0x1407534d0 | `(index:i32, height:f32)` | 更新三向量 | — | `PillarProposals{Vec<PillarPlacementProposal 0x1cB>, Vec<f32> scores, Vec<usize> keys}`；新 key 三向量 push；同 key NaN-aware min；新 score 更小替换末提案 | 0x1cB 记录字段布局、score 来源 | I |
| 9 | `assemble_stone_pillar` | 同文件:54 | 0x14120d670（closure$4 0x14120d340） | `(x,z,height,seed,&mut Vec<[u8;96]>)` | 96B 石柱砖 | fastrand、`random_splits` | `bottom+0.2≤1.26` 单段否则双段且 `1.2<abs(Δ)` 标记；`span=(top-bottom)-0.25`；`count=ceil(span/0.56)`；逐对 −0.2 偏移 + jitter（0.42/0.7，顶段固定 0.7/0.84）；`bottom+0.2>2.1` 盖砖；常量 1.26/0.56/0.25/0.42/0.7/0.84/2.1 | 单/双段分段点、jitter 选择位 | I |
| 10 | `construct_wooden_pillar_w_stone_base` | `construct_elevation_supports/util_pillar_construction.rs:21` | 0x141fe2150 | `(base:Vec3, height:f32, &mut Vec<[u8;96]>)` | 96B/64B | SSBO(0x3d) | 高度钳 1.1；半高轴立杆；`len>1.25` 石基（inverse_lerp 2 段、0.3 缩斜撑）；再逐 0x18B 记录 `construct_crossbrick_pillar`；常量 1.1/1.25/0.5/0.3/0.2 | 16B 柱源/0x18B 记录字段、SSBO 锁写序 | I |
| 11 | `construct_brick_columns` | 同文件:30 | 0x141fe3240 | `(&[Vec3], f32, &mut Vec<[u8;96]>)` | 96B | FrozenMap(0x3a) | FrozenMap→锁→writer 末项→0x40B 柱表逐 `construct_crossbrick_pillar` | 0x40B 表布局 | I |
| 12 | `construct_crossbrick_pillar` | 同文件:39（`util_cross_brick_pillar.rs` re-export） | 0x142418f50（15.9KB） | `(base:Vec3, height:f32, &mut Vec<[u8;96]>)` | 96B | `random_splits`、`Rectangle2d` 四边、fastrand、`xz_rotation_bad` | `count=ceil((top-bottom)/(size*0.4))`；`split*top+(1-split)*bottom`；逐切点四边随机砖；层高 0.4 | 四边偏移/rotation 参数、size 源 | I |
| — | `circle_categorize` | `.../circle_categorize.rs:122` | 0x14124af60 | **已实现（O）** | bool | core 几何 | 见文件；阈值 1.75/1.25、`s*0.6+(1-s)*0.3`、网格 `ceil(2r)`、`ratio<threshold` NaN→true | — | O |
| 13 | `construct_circle_bottom_layer` | `.../circle_bottom_layer.rs:49` | 0x14124b2c0 | `(center:Vec3, radius:f32, count:usize, &mut Vec<[u8;96]>)` | 96B | `Circle2d::perimeter/get_pos_at_u`、`random_splits_into`、SSBO BufferState | 入参 circle center@0、`radius+0.28`@8；`count=ceil(perimeter/0.69)`；halftimber 早退；逐对 `add_bottom_brick`；常量 0.69/0.28 | BufferState 锁链、u 序列推进 | I |
| 14 | `add_bottom_brick` | 同文件:39 | 0x141fe1de0（11 参，circle/rect 共用） | 当前壳 `(pos,rot,width,depth,&mut Vec<[u8;96]>)` | 96B | fastrand、`x0y`×2、`xvy`、`render::impl$1::from`(0x60B 记录)、`instanced_wall::impl$1::from`/`impl$3::default`、`SparseInstanceBufferWriter::push<BrickTransformSsbo>` | `fastrand::Rng::f32` 抖动；0x60B 记录装配；失败置脏字节 1；常量族 0x142925870/0x1429cd068/0x142925970/0x142a81ca0/0x1429258e0/0x1429258f0/0x1429345b0/0x142fcb7f0 | 11 参 ABI、闭包环境、常量逐条值 | U |
| 15 | `construct_circle_pillars` | `.../circle_pillars.rs:39` | 0x14124ba30（1016 行） | 当前列 7 个闭包参数 | — | terrain/raycast/shape | `count=round(clamp01(ratio)*7.0+(1-clamp01(ratio))*4.0)`（实测 7.0/4.0；旧 1.2/0.2 已否）；`t=i/count`；`terrain+max(0,ratio) ≤ wall+0x280`；`find_stone_column_start_y`；smoothstep `((h-y+_UNK)/5.0)`→`(3-2t)t²`→`*1.5+偏移`；外移 −0.4；`wall+0x298` 半木二路（0.5×3 常量 + `wall+0x280·k` 计数）；墙条目 stride 0x50 entity@+0x38 → PublicWalls→shape contains | 三向 raycast 净空参数、halftimber 常量族、0x50B 实体校验 | I |
| 16 | `construct_turret` | `.../circle_turret.rs:20` | 0x14124cc30（1902 行） | `(center:Vec3, radius:f32, height:f32, &mut Vec<[u8;96]>)` | 96B | Circle2d/Aabb2/Curve/CurveU/random_splits_into/Nani | 圆形逐层砖环 + 地形/墙高门控 + `wall+0x298` 半木二路 | **层数、收分率、逐层计数公式**（无证据，须 fresh 定点） | U |
| 17 | `find_overhang_support_segments` | `.../find_overlap_segments.rs:33` | 0x141248620（43KB） | `(&[Vec3], &mut Cache, wall:u64) -> Option<[u8;40]>` | 40B 段 | Rectangle2d/Circle2d/Curve/CurveU/CurveUCircular/`line_x_shape_overlap`/glade+墙检查/多 closure | 几何核已定位 | 输出结构、阈值、closure 语义 | U |
| 18 | `detect_overhang_support_segments` | `.../mod.rs:44` | 0x1407867d0（4258B） | `(ResMut<Subscriptions>, ResMut<Cache>)` | ElevationSupports 写面 | tabloid `Subscriber<DirtyTiles>`、`closure$2`(0x140787d80 清两 WoodenPlankSsbo) | 脏 → cache.new → 墙表 0x2a8：`fVar4@-0x20==0` → clear+insert+bucket；否则 Circle/Rect aabb ∩ DirtyTiles → closure+find+bucket | `fVar4` 判据、ElevationSupports 字段 | I |
| 19 | `construct_circle_supports` | `.../mod.rs:71` | 0x140788160（2305B） | 无参壳（真实含 3 资源） | — | `circle_categorize`/turret/bottom/pillars | 0x28B 队列→墙 0x2a8→`is_circle`；`clear_and_get_mut`；true→bottom+indexmap bucket `{0,4,0,wall_id,uVar13}`+pillars；false→turret；0x20B 记录去重复制进 ElevationSupports | ElevationSupports/Stats 布局（core 为 Gap） | I |
| 20 | `construct_rectangle_columns` | `.../mod.rs:81` | 0x140788a80（2674B） | 无参壳 | — | rect_bottom+rectangle_pillars | is_rectangle→clear+rect_bottom+rectangle_pillars+ElevationSupports | 同 20 + 矩形 4 角/raycast | I |
| 21 | `construct_rectangle_brackets` | `.../mod.rs:89` | 0x1407896b0（2626B） | 无参壳 | — | small/large brackets | 矩形→small；墙高 `(0x2a8-0x1c)≥1.6` → large | 同上 | I |
| 22 | `construct_rect_bottom_layer` | `.../rectangle_bottom_layer.rs:18` | 0x14124eb80 | `(min:Vec3, max:Vec3, &mut Vec<[u8;96]>)` | 96B | `add_bottom_brick` | halftimber→四角 plank；否则 5 段边：`count=ceil(len/0.69)`、`jitter=0.32/len`、`random_splits_into`→`get_point_u`×2+lerp | 5 段边顺序/角接 | I |
| 23 | `construct_rectangle_pillars` | `.../rectangle_pillars.rs:128` | 0x14124fcb0 | `(&[Vec3], f32, &mut Vec<[u8;96]>)` | 96B | `find_stone_column_start_y`(已实现)、`legacy_raycast_w_params`、`Rectangle3d` | 4 角 `get_point`→terrain→`terrain+0.05≤wall+0x280`；逐墙 0x50→get→shape `is_pos_inside`；角法向 3 向 raycast；命中掩码 `0x6d84>>(code&0x1f)&1`；minDist≥2.3；smoothstep `((wall_h-y-5)/5).clamp01`→`(3-2t)t²`→`*1.5+(1-t)`；外移 −0.4；0x40B 提案→`Rectangle3d::new`→0x20B | Rectangle2d/3d 实参组合、0x40/0x20 字段、0x6d84 全集 | I |
| 24 | `construct_rectangle_small_brackets` | `.../rectangle_small_brackets.rs:14` | 0x141250da0（7701B/1137 行） | `(&[Vec3], &mut Vec<[u8;96]>)` | 96B | fastrand、`stairs::visual` | 托架几何参数/墙高门控/角部装配 | **未逐语句**（fresh 已取） | I |
| 25 | `construct_rectangle_large_brackets` | `.../rectangle_large_brackets.rs:13` | 0x141253fc0 | `(&[Vec3], &mut Vec<[u8;64]>)` | 64B | `Rectangle2d` 角、`RayBoxIntersection::is_hit`、terrain、`valid_enclosure`、`Curve` | 角部斜撑+横梁 | **几何参数未定位** | I |
| 26 | `maintain_stone_column_colliders` | `.../maintain_stone_column_colliders.rs:26` | 0x14078a6e0 | `(ResMut<Storage>, ResMut<RaycastWorld>, EventReader<OnWallDeleted>, EventWriter<StoneColumnsChanged>)` | 碰撞体撤除 | RaycastWorld 6 调用、Triangle 相交 | — | rapier 后端暴露面（core-api-missing） | I/U |

**验收门**：柱/托架输出 96B/64B 记录字节级一致；消费者 = `ElevationSupports`（core 需先落布局）、`AssetSsboLibrary`、`StoneColumnCollisionStorage`。GPU 上传后置；rapier 碰撞后置（非 GPU，但依赖 backend）。

### 2.3 拱 / 过梁 / 地板

| # | 函数 | 位置 | VA | 签名 | 返回 | 依赖 | 常量/公式 | 缺证 | 级别 |
|---|---|---|---|---|---|---|---|---|---|
| 27 | `construct_floors` | `construct_floors.rs:63` | 0x1407661e0 | `(Res<PublicWalls>, EventReader<OnWallChanged>, EventReader<OnWallDeleted>, Local<FloorStorage>, Commands)` | Floor 实体 | `NaniBundle<StoneFloorInstanceData>`、`Floor`(缺) | 先 deleted 后 changed；`remove_entry`+despawn；`0.0<height` 且 `*(u32)(wall-0x48)<2`；shape 首 qword bit0==0 → `log2f(max(shape+0xc,1.0)*3.0)`；命令载荷 0xb8、bundle 0x48@+8 | **`Floor` 组件布局**、StoneFloorInstanceData ABI | I（core-type-gap） |

**注**：`construct_arches`/`construct_lintels` 本体已有可用核心链（O），其内部支撑梁 4 个 todo 见 #4-#7；`create_stone_arch_profile`/`remap_t` 为已实现近似体（需按 0x1407645f0/0x140765300 校正：`remap_t` 非 0 分支为 `inverse_lerp`，当前实现是 `1-(1-t)²`，**判为近似，须重写**）。

验收门：拱 96B/120B 命令、过梁 64B 与 `evidence/building-arches|structures` 样本一致；消费者 = `WallConstructorOutput`、`ConstructGateCmd`、MaterialStats。

### 2.4 平台楼梯 / 玩家楼梯

| # | 函数 | 位置 | VA | 签名 | 返回 | 依赖 | 常量/公式 | 缺证 | 级别 |
|---|---|---|---|---|---|---|---|---|---|
| 28 | `construct_platform_stairs` | `construct_platform_stairs/mod.rs:38` | 0x1407744d0 | `(ResMut<WallHoles>, ResMut<PlatformStairsStorage>, Res<PublicWalls>, Res<UiSignifierStream>, Res<AssetSsboLibrary>)` | 96B 砖/holes/cells | core `platform_stairs.rs` 全族（collect/prepare/cell/geometry/holes，O） | 80B 装饰源 `raw[0x4e]==0x3f && raw[0x2a]∉{2,3,4}`；blockers `coordinate=[f(8),f(12)]`、terrain 差<3.0；`process_loop_range_into_wallpath_segments`(行456)；`update_phys`(535)/`add_colision_geo`(564) | 80B 记录字段、碰撞接线 | O(CPU 链)/I(ECS) |
| 29 | `preprocess_curve` | `construct_playermade_stairs/mod.rs:91` | 0x140780e20（5.3KB；closure$0 0x140782f50） | `(&[Vec3]) -> Vec<Vec3>`（真实输出结构含 6 qword curve + Vec<(usize,u8)>） | 段/曲线族 | `Curve::try_new_from_points/slice_into`、`try_resample_approx_pt_count_remap` | `step0=min(0.368/curve_len,0.2)`；flag[0]==0→step0=0；`t0=if flag[1]!=0 {1-step0} else {1}`；两路模式字节；`min(2/fVar12,fVar14)`；`roundf((len-base)*0.25)` 段；密度 `min(2/len,0.45)`；记录 2×8B（计数+kind） | closure_0 语义、模式位组合、输出字段类型 | I |
| 30 | `construct_playermade_stairs_arches_n_walls<const WALLS>` | 同文件:105 | 0x140496800 / walk 0x14048ab50 / closure$6<true> 0x14048b320 | `(ResMut<StairsArchSupportRequests>, Res<UiSignifierStream>, Res<TerrainHeightsData>, ResMut<StairsPillarProposals>, ResMut<WallConstructorOutput>)` | 96B + 44B 提议 | 9 closure 单态 + from_curve + raycast | 模块文档 §1-9 已列全流程（SSBO 0x3a 校验、StairsGraphEdge hash、preprocess、smoothstep `((h-min)/1.5-1.5)/1.5`+0.5、raycast 码 0x11/0x01/0x02、±0.56*0.5*dir、min1e6/max0.0） | `ArchWalker::walk` 精确体、from_curve 中段、closure 单态逐语句 | I |
| 31 | `process_playermade_stair_pillars_proposals` | `.../pillars_process_proposals.rs:142` | 宿主 0x140178a70（bevy impl$3::call_mut；inlinee 10287） | 6 参：`ResMut<StairsPillarProposals>, Res<UiSignifierStream>, ResMut<RaycastWorld>, ResMut<StairColliderRegistry>, ResMut<RtWorld>, EventWriter<UpdateRtWorldEvent>` | 碰撞体 + rt 事件 | `merge_nearby_proposals`（已 O，含测试） | merge 距离 <0.01，size_x max/size_z min，kind==2 终止；`Rectangle3d::new` size=`{(size_x-size_z)+0.3, +0x18-0.5}`（0x3f0f5c29=0.56）；`box_trigger_volume`；`spec_extend<Triangle>` | **`StairColliderRegistry` 缺**；`UpdateRtWorldEvent` 现已存在（core `resources/rt.rs:172`）；0x18 字段语义 | I（core-type-gap，剩余 1 类型） |

### 2.5 灰泥 / 半木 / 窗工具

| # | 函数 | 位置 | VA | 签名 | 返回 | 依赖 | 常量/公式 | 缺证 | 级别 |
|---|---|---|---|---|---|---|---|---|---|
| 32 | `maintain_plaster` | `plaster_systems.rs:72` | 0x14075b810（5273B/53KB） | 9 参（现有壳） | 实体生命周期/网格 | `Decorators::iter_removed_ids`、`WallPlasters`、`PlasterPeelAmount`、`time_continuous_lerp`、`holes_no_duplicates` | puffin；延迟 buffer 回收；δ=lerp peel；`abs(new-old)≥0.01`；`try_get_wall_shape`(center)/max_y/xvy→0x28B bucket(tag 9@+0x20,+0x1c=1.0,+0x24)；dirty Vec；removed→despawn；PublicWalls sort_by_key；is_fence 门控；mesh 生成 | peel 曲线/时长、0x28B 字段、WallSide/网格 | I |
| 33 | `create_plaster` | 同文件:84 | 0x14075d460（1686B） | 4 参（现有壳） | mesh/实体 | `WallHoles::as_finalized`(stride 0x68)、`holes_no_duplicates`、mesh 表(stride 0x20) | 逐 `CreatePlasterCmd`(40B)；墙记录 stride 0x2a8（缺失 unwrap）；→generate | 40B 命令布局、mesh 表键 | I |
| 34 | `split_wall_into_sides` | 同文件:91 | 0x14075dc60（3130B） | `(&[[f32;3]], &mut Vec<[f32;4]>)` | WallSide 列表 | `Rectangle2d::get_point_u` | WallSide 24B=0x18 `{u_begin+1e-7,u_end-1e-7,height,y offset,u32 grid,u8 roof_flag}`；墙 `+0x258<2` 与 bit0 门控；4 侧；`divps(...,0.1)` round + `psubusw 0x00020002` 下限；gable 双高 overlay | `WallSide::new`/`sd_box` 细节、roof_flag 枚举 | I |
| 35 | `generate_plaster_mesh_and_entities` | 同文件:101 | 0x1412269b0（71.6KB 泛型单态） | `(&[(f32,f32,f32,f32,u8,u8)], &mut Vec<[f32;3]>) -> usize` | mesh + 实体 | `Roof::is_flat_w_no_rim/is_flat`、`WallSpaceCoord::with_y_offset/to_world_space`、`Rectangle2d/Circle2d::get_normal_at_u`、`Aabb2::center/half_dims`、`BufferDesc::new_gpu_only`、`Buffer::device_address` | 世界顶点/法线/buffer；`mirror_holes_as_needed`；`sd_box` 有向距离场 | **mesh 拓扑/UV/buffer 布局**（71.6KB） | U |
| 36 | `half_timbering` | `utils/half_timber.rs:45` | 0x141218e30（27KB/5494 指令） | `(Option<&[[f32;3]]>, &[f32], &[f32], Option<[u8;2]>, bool, &mut Vec<[u8;64]>)` | 64B WoodenPlank | try_get_wall_shape、`Roof::gable_sides/is_flat`、SSBO WoodenPlank(0x3d/61B 名)、`split_height_into_skirts`(core) | puffin 早退；wall+0x180→+0x18 LOCK；10 qword→0x60B 结构；is_flat 分支；`while wall+0x280 + 1.0 <= fVar110`；扩容 0x15 阈值分流 0x14122cd30/0x14122f250 | **逐梁坐标/closure 中段（~0x6860B 截断）**、+0x180 布局、SSBO 记录布局 | U |
| 37 | `overlapping_windows` | 同文件:59 | 0x1412218a0 | `(&[f32;4], &[f32;4]) -> bool` | bool | DecoratorStorage SIMD 掩码 | subtype 过滤 `byte[10]<2 && byte[0xe]-0x38<3`；`get_window_aabb` ±0.038（dormer 'g' 且 leaf[0xc]==1 再 ±0.14）；`Aabb2::intersects`→ResourceAccessInfo(stride 0x14) | ResourceAccessInfo 布局、装饰器迭代 API | I |
| 38 | `get_window_aabb` | 同文件:68 | 0x141221da0 | `(&[f32;4]) -> [f32;4]` | Aabb2 | gothic profile curves | 门分支：`dst_as_wall && raw+0x46=='9'` → gothic_door/half_dormer_profile_curve，profile 末点 → `[x-x0,y0+y,y0,..+0.093]`；普通 `bbx_ls + Aabb2::from_center_dims` | 两 profile 函数落库、0.093 语义 | I |
| 39 | `check_for_overlap_with_all_windows` | 同文件:75 | 0x141221f70 | `(&[f32;4], &[[f32;4]]) -> bool` | bool | get_window_aabb | `Aabb2::contains` 中心/侧向/扩展三测试 | 三测试顺序 | I |

> 同文件中 `roof_outline`（0x141217be0）与 `add_underwindow_pattern`（0x141fd1c30）当前为**空壳**（非 todo!），但同属未恢复业务，见 §5 复核项。

**验收门**：抹灰 mesh 顶点/法线与 `evidence/building-*` 对照；半木 64B plank 与 WoodenPlankSsbo 观测；消费者 = 渲染 mesh/实体/Nani；GPU buffer 后置。

---

## 3. 28 个“无迹象”骨架函数（37 无文本痕迹 − 9 个 core 等价吸收）

### 3.1 9 个已由 country-core 等价吸收（不重复实现，登记映射）

| 骨架符号 | 原 VA | core 等价实现 | 判定 |
|---|---|---|---|
| `calc_path_wall_segmentation` | 0x14074f7a0 | `systems/wall/construction/segmentation.rs::wall_path_segmentation` | 改名/合并 |
| `calculate_wall_path_segments` | inlinee | 同上 + `materialize_wall_segments` | 合并 |
| `calculate_wall_path_segments_impl` | 0x14122ff90 | `structures.rs::materialize_wall_segments` | 改名/合并 |
| `wall_constructor::normalize_holes` | inlinee | `bricks.rs::normalize_wall_holes` | 改名 |
| `wall_constructor::split_brick_by_holes` | inlinee | `bricks.rs::cut_wall_bricks`/`resolve_hole_overlap` | 合并 |
| `utils::resolve_hole_overlap::check_for_hole_overlap` | inlinee | `bricks.rs::resolve_hole_overlap` 内部 | 内联合并 |
| `sample_wall_path_intersections::threshold_curve_path_mask` | inlinee | `wall_state/path_preparation.rs::prepare_wall_path` 内部 | 内联合并 |
| `...::add_point_with_mask_to_output` | inlinee | 同上 | 内联合并 |
| `segmentation::add_split_with_offset` | inlinee | `segmentation.rs::add_splits` | 内联合并 |

### 3.2 28 个真实本地缺口（精确 28 个独立符号；均为 `inlinee_declaration`，需按宿主深挖）

| # | 符号 | 骨架文件 | 宿主/深挖入口（VA） | 判定与动作 |
|---|---|---|---|---|
| 1 | `SupportBeamGrower::accept` | `construct_lintels/place_support_beams.rs` | `place_support_beams` 0x141242530（行 532） | 内联；cost 小者 consume/join |
| 2 | `DoorMeshInProgress::add_quad` | `construct_gates/mod.rs` | **已发射 0x141214ed0**；宿主 `construct_wooden_gate_doors` 0x141210110 / `construct_gates` 0x140756370 | 真缺口（有独立 RVA）；每 quad 顶点序 (0,1,2,0,2,3)，positions/colors/uvs stride 0xc/0xc/8；按 core `construct_gate_cpu` 输出接线（ANALYSIS B1.2 O） |
| 3 | `utils::wall_corners::check_angle` | `utils/wall_corners.rs` | `add_wall_corners` 0x141215540（inlinee 70917） | 内联；±10°（±0.1736482）判据 |
| 4 | `construct_pillar_stub` | `construct_playermade_stairs/pillars_visual.rs` | `..._pillars_visuals` 0x14077d520（inlinee 24718） | 内联；柱高/数量待定 |
| 5 | `construct_sewer_bars` | `construct_gates/systems.rs` | `construct_gates` 0x140756370（行 152） | 内联；sewer 分支 |
| 6 | `create_stone_arch_profile_impl` | `construct_arches/profile.rs` | `create_stone_arch_profile` 0x1407645f0（行 62） | 内联；随 profile 校正 |
| 7 | `create_wall` | `construct_walls.rs` | `construct_walls` 0x140757800（行 424） | 内联；`InstancedWall` 创建 |
| 8 | `curve_to_arch_ws` | `arch_walker.rs` | `construct_arches::closure$3` 0x141231af0；`construct_arches` 0x14075f680 | 内联；arch 砖发射 |
| 9 | `door_arch_heights` | `construct_gates/mod.rs` | `construct_wooden_gate_doors` 0x141210110 | 内联；门拱高组 |
| 10 | `door_bottom_curve` | 同上 | 0x141210110 | 内联；门底曲线 |
| 11 | `door_mesh` | 同上 | 0x141210110（closure$0） | 内联；门扇 mesh |
| 12 | `door_mesh_gap` | 同上 | 0x141210110 | 内联；双叶间距 |
| 13 | `emit_pillar_at_point` | `place_pillars.rs` | `place_pillars` 0x140753670（行 276） | 内联；单点柱装配 |
| 14 | `ArchWalker::emit_segment_bricks` | `arch_walker.rs` | `walk` 4 单态 0x141232bb0 / 0x141233380 / 0x141231ea0 / 0x14048ab50（inst=8） | 内联；随 `walk` 精确重写 |
| 15 | `utils::wall_corners::find_corners` | `utils/wall_corners.rs` | `add_wall_corners` 0x141215540（inlinee 70916） | 内联；角点分类 |
| 16 | `init_magic_supports` | `construct_playermade_stairs/brackets.rs` | `construct_playermade_stair_brackets` 0x1407834f0（行 246） | 内联 |
| 17 | `DoorMeshInProgress::into_mesh` | `construct_gates/mod.rs` | `construct_wooden_gate_doors` 0x141210110 | 内联；mesh 组装 |
| 18 | `is_sewer_gate` | `construct_gates/systems.rs` | `construct_gates` 0x140756370（行 143） | 内联；sewer 门控 |
| 19 | `mirror_holes_as_needed` | `plaster_systems.rs` | `generate_plaster_mesh_and_entities` 0x1412269b0（inlinee 70970） | 内联 |
| 20 | `offset_door_bottom_curve` | `construct_gates/mod.rs` | 0x141210110 | 内联 |
| 21 | `place_brick_steps` | 同上 | 0x141210110 | 内联；台阶砖 |
| 22 | `place_fence_piece` | `construct_fences/process_segment.rs` | `process_segment` 0x141238d80 | 内联；plank/post/lean/rail |
| 23 | `place_ladder` | `construct_gates/mod.rs` | 0x141210110 | 内联；梯子 |
| 24 | `sd_box` | `plaster_systems.rs` | `generate_plaster_mesh*` 0x1412269b0 | 内联；有向距离场 |
| 25 | `segment_length` | `construct_gates/mod.rs` | 0x141210110 | 内联 |
| 26 | `vec_push_within_capacity_assert` | `plaster_systems.rs` | 0x1412269b0（行 972） | 内联；容量断言 |
| 27 | `wooden_lintel_profile` | `construct_lintels/profile.rs` | `construct_lintels` 0x14076d9f0（profile 行 12） | 内联；过梁剖面 |
| 28 | `wrap_around` | `utils/half_timber.rs` | `half_timbering` 0x141218e30（行 1872） | 内联；半木环绕 |

**另 4 个辅助内联体（文本有提及、但无独立实现，建议登记；不计入“无迹象 28”）**：`PillarProposals::empty`（0x140753670 行 82）、`WallSide::new`（0x14075dc60 inlinee 24542）、`generate_plaster_mesh`（0x1412269b0 行 316）、`maintain_deferred_plaster_buffer_disposal`（0x14075b810 inlinee 24528）。

---

## 4. 跨 crate 契约（只列需求；**不修改 country-core**）

### 4.1 已存在、可直接消费（O）

| 类型/资源 | 位置 | 被谁用 |
|---|---|---|
| `PublicWalls` / `PublicWallState` | `resources/walls/public_walls.rs`, `public_wall_state.rs` | 全部系统 |
| `WallId`（inner u64） | `resources/walls/mod.rs:35` | 全部 |
| `PublicWallBuildingState` | `resources/walls/publication_fields.rs` | `shape()/float_0x284/288/observed_294/296/29a` |
| `InnerWalls` | `resources/walls/inner_walls.rs` | UI 控制 |
| `WallHoles` / `WallPlasters` | `resources/walls/wall_holes.rs`, `public_walls.rs:284` | plaster/floors |
| `Gates` | `resources/walls/wooden_gates.rs` | construct_gates |
| `StoneColumnCollisionStorage` / `StoneColumnsChanged` | `resources/walls/wall_stone_columns.rs` | elevation supports |
| `TerrainHeightsData` / `TerrainHeightGrid512` | `resources/terrain_height.rs`, `publication_source.rs` | gates/pillars |
| `RaycastWorld` / `RaycastSimpleResult` / `SubworldColliderHandle` | `systems/collision/world.rs` | pillars/stairs |
| `RtWorld` / `RtMeshHandle` / `UpdateRtWorldEvent` | `resources/rt.rs` | stair proposals |
| `StairsPillarProposals/StairPillarProposal` / `StairsArchSupportRequests` / `StairsBracketsRequests` | `resources/stairs_walls.rs` | playermade |
| `StairsVisualState` / `GameTime` | `resources/stairs/visual.rs`, `time.rs` | playermade |
| `WallPathSegmentationMask` / `WallLoopRange` | `systems/wall/construction/segmentation.rs` | segmentation/arches/lintels |
| `MaterializedWallSegment` / `WallSegmentSource` | `structures.rs` | lintels/gates |
| `AssetSsboLibrary` / `AssetMeshLibrary` | `asset_libraries/*` | all outputs |
| `Decorators` / `iter_removed_ids` | `resources/walls/decorator_storage.rs` | plaster |
| `WallShapeSnapshot` / `Circle2d` / `Rectangle2d` / `Rectangle3d` | core/utils geometry | supports |
| `PathRasters512` + `prepare_wall_path` | `systems/wall/wall_state/path_preparation.rs` | intersection sampling |
| core 构造链：`wall_path_segmentation`, `materialize_wall_segments`, `construct_lintel_supports`, `construct_regular_wall_arches`, `construct_fences_cpu`, `construct_gate_cpu`, `platform_*` | `systems/wall/construction/*` | 本 crate wrapper |

### 4.2 需要 core 补齐/扩展（**报中央，不在 WC 改**）

| 需求 | 现状 | 阻塞的 todo | 期望 API/布局 |
|---|---|---|---|
| `Floor` 组件 | **缺**（全 workspace 无） | #27 `construct_floors` | wall_id + shape + floor height + byte@0x88=0；命令载荷 0xb8 |
| `ElevationSupports` / `ElevationSupportStats` | `Gap`（`resources/walls/elevation_support_stats.rs:12`） | #18-#21 | 0x28B stats 记录 + 0x20B 支柱记录字段 |
| `StairColliderRegistry` | **缺** | #31 | `RawTable<StairGraphEdge, Vec<Vec3>>`：hash_one→探测→命中直用/未命中 reserve_rehash 插入空 Vec |
| `ArchSegments` | `Gap` | `ui_arch_controls`/arch 编辑 | 拱段可读布局 |
| `CurvedHalfTimberDrawables` | `Gap` | `curved_halftimber_maintain_entities` | stride 0x10 drawables + 每墙 0x28 砖表 |
| `MaterialStats` | 方法为空 stub（`material_stats.rs:17-23`） | arches/lintels/fences/pillars | 真实累计 `set_arches_segments_count/add_arches_length/add_gate_length/set_gates_segments_count/set_fences_segments_count/seed/set_pillar_placements` |
| `Domain` 公共几何 helper | 部分私有 | #1-#3,#22,#38 | `random_splits_into`(已有 O)、`split_height_into_skirts`、`WallSpaceCoord::to_world_space`、`xz_rotation_bad`、`x0y/xvy`、`get_approx_coord_from_pos`、`SubtractiveBoolean1D` 公开面 |
| rapier native-physics 后端 | `platform_rapier_backend.rs` 有抽象，但石柱撤除面未闭合 | #26 | `StoneColumnCollisionStorage` 碰撞体增删接口 |
| `Roof` 读取 | `Roof::observed`/`wall_id` 已有；gable 剖面 | #36 | `gable_sides/is_flat/is_flat_w_no_rim/profile_curve_normalized/height` 公共访问器 |
| `DecoratorStorage` 迭代 | `Decorators` 有 `iter_removed_ids` | #37/#38 | SIMD 掩码迭代 + subtype 字节访问 |

---

## 5. 验收门（每个 todo 的“可编译 + 有真实输出 + 有消费者”）

### 5.1 全局门

1. `cargo check -p system-wall-constructor` 通过；新增 `cargo test -p system-wall-constructor` 覆盖每个恢复函数至少 1 个数值/布局用例。
2. 输出记录必须**字节级**可对照：96B `BrickTransformSsbo`、64B `WoodenPlankSsbo`、120B `ConstructGateCmd`、136B door spawn、40B/44B proposal、104B segment、136B mask。
3. 禁止 `let _ = ...` 吞掉真实输出；禁止恒 0/恒空壳；RNG 须 `fastrand::Rng` 同序列（`random_splits_into` 已 O）。
4. 每个函数必须落到真实消费者：
   - 96B/64B → `WallConstructorOutput`（本 crate）→ `AssetSsboLibrary`（渲染边界）。
   - `ConstructGateCmd` → `construct_gates` → `Gates`/实体。
   - Floor/Plaster/CurvedHalfTimber → 实体 spawn + 组件。
   - ElevationSupports → core 资源 + 碰撞。
   - MaterialStats → 真实统计（非空 stub）。
5. 不复制 core 已恢复逻辑：优先调用 §4.1 的 O 函数；若确认不等价，登记差异后再本地实现。

### 5.2 GPU/后置清单（允许保留边界，但 CPU 输出必须先真实）

| 项 | 处理 |
|---|---|
| `gpu_clear_wall_gate_pieces` 0x140773c00 / `gpu_flush_plank_buffers` 0x140774010 | 纯 GPU；保留壳，接 rhapsody 后实现 |
| `SparseInstanceBufferWriter::push<BrickTransformSsbo/WoodenPlankSsbo>` | CPU 记录先产出；上传后置 |
| Nani/TransientMesh/DrawableMeshBundle 句柄 | 实体先 spawn 占位，句柄接线后置 |
| 石膏 GPU buffer（`BufferDesc::new_gpu_only`/`device_address`） | mesh 拓扑/UV CPU 先恢复 |
| rapier 石柱碰撞体 | core 后端落地后 |
| `debug_force_update_arches` 0x14003bb80 | B 为 1B `ret` 存根，空体判等（O），无需实现 |

### 5.3 已知近似体必须重写（不计“已完成”）

- `arch_walker.rs::walk`：4 处结构性分歧（travelled 推进、周期 u、尾砖、双点列 min 计数）。
- `arch_walker.rs::new`：已对（O）。
- `construct_arches/profile.rs::remap_t`：真实非 0 分支为 `inverse_lerp`，当前 `1-(1-t)²` 不判等。
- `create_stone_arch_profile`：点数/trim 语义需按 0x1407645f0 校正。
- `segmentation/*::process_segment`/`add_splits_in_empty_range`/`score_split_candidate`：当前输出 `[0.0;4]`/比例近似，须按 O 公式重写。
- `init_wall_height_and_arch_segment_height`：当前只调 `update_gable_heights`，缺两对 `{count,height}`/高度数组。
- `construct_walls`：缺 `from_curve` 中段（row→proposal）与真实 `InstancedWall` 写面。
- `construct_fences::emit_fences`/`process_fence_segment`：当前只写位置，缺 plank/柱/lean/rail/统计。
- `construct_playermade_stairs` 三个非 todo 系统（brackets/magic/pillars_visual）为近似壳，须升级。
- `curved_halftimber_maintain_entities`/`wooden_floor_borders`/`roof_outline`/`add_underwindow_pattern`：空壳或近似。
- `ui_arch_controls`/`do_ui_arch_controls`/`ui_lintel_controls`：只做模式/交互门控，缺剖面三角/`InnerWalls::update_arch_type/height`/CompoundEditBuilder/History。

---

## 6. ② 实现清单（≤20 步，按依赖顺序；先几何基础再上层构造）

1. **基座核对（只读 + 中央报备）**：确认 core `random_splits_into`、`split_height_into_skirts`、`WallSpaceCoord::to_world_space`、`xz_rotation_bad`、`x0y/xvy`、`get_approx_coord_from_pos`、`SubtractiveBoolean1D`、`Curve::get_coord_at_u/get_point_u` 的公开面；缺则报中央，不本地复制。
2. **`ArchWalker::new/walk` 精确重写**（0x14074c2d0 + 4 单态 0x141232bb0/0x141233380/0x141231ea0/0x14048ab50），恢复 `emit_segment_bricks`/`curve_to_arch_ws` 等价；单测 count/尾砖/周期 u。
3. **`init_wall_height_and_arch_segment_height`**（0x14078b550）：两对 `{count,height}` + 高度数组 + gable 剖面（常量 -2.5/-0.46/0.96/0.5/0.28/0.95）。
4. **分段精确化**：`process_segment`/`add_splits_in_empty_range`/`score_split_candidate`（0x140750420/0x140751240/0x140751610）对齐 core `wall_path_segmentation`；补 `materialize_wall_segments` 接线。
5. **`from_curve` 中段（row→proposal）**：`perturb_row_ends`/`split_horizontally`（已 O）→ `normalize_wall_holes` → `consume_wall_proposal` → `resolve_hole_overlap` → `emit_resolved_wall_brick`；输出 96B 可对照。
6. **`construct_walls` 全链**：`create_wall` + `InstancedWall`/`WallHoleStorage`/`WallTriggerVolume` + 屋顶分支 + 事件。
7. **`add_wall_corners` + `find_corners/check_angle`**（0x141215540）：±10°、0.92±0.16、末砖 0.76。
8. **`add_skirts` + `split_height_into_skirts`**（0x1412221f0）：3.5 阈值、0.69 行距、累计距离 inverse_lerp。
9. **`stairs_signifiers_new` + `SubtractiveBoolean1D`**（0x141223d10）：3.0 判据、0.69、@0x5c=4。
10. **拱族**：`create_stone_arch_profile`/`remap_t` 校正 + `construct_arches` shape2 分支 + closure$3 96B 发射 + 120B GateCmd。
11. **过梁/支撑梁**：`propose`/`accept`/`place_support_beams`/`add_diagonal_support`/`emit_support_beams`/`wooden_lintel_profile`（0x141fe09c0/0x141242530/0x141243740/0x141243c20），复用 core `construct_lintel_supports`。
12. **门族**：`construct_gates` shell 完成（sewer 分支 + 136B payload + `Gates`），`add_quad`/`into_mesh` 接 core `construct_gate_cpu` 输出。
13. **栅栏精确化**：`emit_fences`/`process_segment`/`place_fence_piece`（0x1412388f0/0x141238d80）perchance 序列/柱 −0.1Y/统计。
14. **地板**：core 落 `Floor` 后实现 `construct_floors`（0x1407661e0）生命周期 + log2 缩放 + NaniBundle。
15. **柱原语**：`add_proposal`/`assemble_stone_pillar`/`emit_pillar_at_point`/`construct_wooden_pillar_w_stone_base`/`construct_brick_columns`/`construct_crossbrick_pillar`。
16. **`place_pillars` 全链**（0x140753670）：0x88B 提案/LoopRange/CurveSlice/木柱 vs 石柱/统计。
17. **立面支撑族**：`construct_circle_supports`/`rectangle_columns`/`rectangle_brackets` + bottom layers + `add_bottom_brick` + circle/Rectangle pillars + turret + `find_overhang_support_segments`/`detect_overhang_support_segments`/`overhang_subscribe_to_tabloids`/`maintain_stone_column_colliders`。
18. **平台楼梯**：`construct_platform_stairs` + `process_loop_range_into_wallpath_segments`/`update_phys`/`add_colision_geo`，复用 core `platform_*` 链。
19. **玩家楼梯**：`preprocess_curve` → `arches_n_walls`（9 closure + from_curve + raycast）→ pillars visual/proposals（core 落 `StairColliderRegistry` 后）→ brackets/magic_support。
20. **灰泥 + 半木 + UI + curved_halftimber + GPU 壳**：`maintain/create_plaster`、`split_wall_into_sides`、`generate_plaster_mesh_and_entities`、`half_timbering`、窗工具三件、`roof_outline`、`add_underwindow_pattern`、`curved_halftimber_maintain_entities`、`wooden_floor_borders`、UI 三系统；最后接 GPU 两壳。

> 每步完成即跑 `cargo check -p system-wall-constructor` + 该步单测；不满足“真实输出 + 消费者”不得标记完成。

---

## 7. 取证入口速查

- 骨架符号/VA：`skeleton/B/crates/systems/wall-constructor/src/<file>.rs` 的 `// 函数符号:` / `首条位置` / `编译实例 RVAs`。
- 反编译：`re_mcp_ghidra` job 内 `get_functions(functions=[{address:"0x..."}], fields=["decompiled_code","callees"], program="tiny-glade.exe")`；共享只读，勿写库。
- 原字节：`python scripts/rne-read.py`（按 VA/RVA）。
- 既有证据：`evidence/building-arches|structures|segmentation|gates|fences|lintels|platform-stairs|holes|constructor`；`upper-modules-plan/systems/wall-constructor/ANALYSIS.md` §1-§5。
- 旧 `/tmp/wc-dec|wall-a-dec|wallagent/dc` 路径来自上一取证环境，本机需重新生成。
