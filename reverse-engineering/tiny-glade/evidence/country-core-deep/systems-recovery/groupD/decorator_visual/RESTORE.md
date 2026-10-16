# RESTORE: system-decorator decorator_visual（D1 restorer，tg_d1）

> 批次 2 组 D。恢复范围：`tiny-glade/crates/systems/decorator/src/decorator_visual/**`
> 22 文件几何组装族（SOURCE_MAP-systems.json `crate=decorator`、`decorator/src/decorator_visual/`）。
> 日期：2026-10-16。证据：`decomp/`（79 业务 VA + 10 兄弟接口 VA 的 fresh RE-MCP
> 反编译 + capdis 原字节手解）+ `sourcemap-decorator_visual.json` +
> `va-name-index.json`（全骨架 19,819 函数 VA→名索引）+ `alias-resolved.json`
> （61 个别名地址解析）。
>
> 记号：O = 机器码/反编译证实；I = 推断（注明依据）；U = 未恢复（显式登记，
> 不称已恢复）。所有 U 项可作 reviewer/re-analyst 复核入口。

## 1. 模块清单与业务函数闭合表

共 27 个 .rs 文件（含 cottage_dormers/ 子目录 5 文件），business 函数 79 个
（SOURCE_MAP 口径 54 个 + 内联宿主/闭包实例登记）。逐文件状态：

| 文件 | 业务函数（VA） | 恢复状态 |
|---|---|---|
| arrow_slit.rs | generate_arrow_slits 0x1407bbee0 | I 结构 + U 剖面装配 |
| chimney_assemble_bricks.rs | chimney_assemble_bricks 0x1407f8db0 | O 转发 |
| chimney_roof.rs | generate_roof_chimneys 0x1407c0910 | I 结构 + U 几何 |
| chimney_wall.rs | generate_wall_chimneys 0x1407c21b0 | I 结构 + U 几何 |
| cottage_balcony_door.rs | generate_cottage_balcony_doors 0x1407f3da0 / add_door_autoclutter 0x1407f5e70 | I 结构 + U |
| cottage_corner_window.rs | generate_cottage_corner_windows 0x1407f66b0 | I 结构 + U |
| cottage_dormers/assemble_dormer_roof_tiles.rs | generate_dormer_roofs 0x1407cb1a0 / closure_env 0x14128db20 / dormer_roof_height / is_bay_window | I 结构 + U |
| cottage_dormers/collision_geo.rs | get_collision_mesh 0x14128a0b0 / DormerKey::quantized_from | I + U |
| cottage_dormers/cottage_dormer.rs | generate_cottage_dormers 0x1407c7ac0 / **is_out_of_roof_bounds 0x1407ca6c0（O 全文）** / cottage_dormer_dims / get_dormer_panel_xforms | O+U |
| cottage_dormers/maintain_dormer_tiles_draw.rs | **maintain_dormer_tiles 0x1407ce920（O 全文）** | O+U |
| cottage_dormers/util_assemble_gothic_dormer.rs | assemble_gothic_dormer_tiles closure 0x141290b30 | I + U |
| cottage_wall_window.rs | generate_cottage_wall_windows 0x1407ebfd0 / **flowerbed_xforms 0x1407ef7b0（O 全文）** / add_lintels 0x1407ef990 | O+U |
| deco_differential_state.rs | **DecoDiff 构造器 0x14010ba60 / 0x14010b940（O 全文）** / with_each_deco_diff 闭包对 0x1412ad540 / 0x1412acd70 / 比较闭包对 0x1412ae1c0 / 0x1412add20 / 重装闭包对 0x1412ae660 / 0x1412af3a0 | O 构造器 + U 重装 |
| decorator_clutter.rs | generate_decorator_clutter 0x1407e3370 | I 结构 + U |
| door_stairs_assemble_bricks.rs | door_stairs_assemble_bricks 0x1407ff130 | O 转发 |
| flag.rs | generate_flags 0x1407ff6c0 | I 结构（O 子类型判别 + 蓝图写面）+ U 布几何 |
| flag_connections.rs | generate_flag_connections 0x1407df470 / greedily_connect 0x1407e1430 / clothline 0x1407e2510 | I 结构 + U |
| generate_clothline.rs | generate_clothline_visuals 0x1407e5190 | I 结构 + U |
| gothic_balcony_door.rs | generate_gothic_balcony_doors 0x1407f0900 / check_for_door_stairs 0x1407f3340 | I 结构 + U |
| gothic_dormer.rs | **gothic_dormer_profile_curve 0x1407c7500（O 全文，剖面控制点全解）** / generate_gothic_dormers 0x1407c5650 | O+U |
| gothic_wall_window.rs | generate_gothic_wall_windows 0x1407e87b0 / get_gothic_hat_bones 0x1407eb500 / calc_gothic_half_dormer_profile_ws 0x1407eb160 | I 结构 + U |
| lantern.rs | generate_lanterns 0x14078d320 / maintain_lantern_lights 0x140790400 / compare_light_lists closure 0x141257fa0 / closure_env 0x1412583d0 | I 结构 + U |
| maintain_visual_entities.rs | **clear_out_visual_entities 0x1407f9260（O 业务链）** / **clear_out_wall_holes 0x1407f98c0（O 手解）** | O |
| stairs.rs | **determine_segment_type 0x14080c810（O 全文）** / **densely_sample_spline_segment 0x140809ad0（O→utils 直通）** / derive_stair_node_params 0x14080c640（O 分派） / 其余 8 个 I 结构 + U | O+U |
| stairs_assemble.rs | **determine_ladder_support 0x1407de800（O capdis 全文）** / **structural_graph_subscribe 0x1407d4990（O）** / **create_stairs_collider 0x1407deb60（O）** / 其余 15 个 I 结构 + U | O+U |
| stairs_railings_assemble.rs | assemble_railings 0x1407f98e0 / railings_tips 0x1407fcc80 / construct_wooden_railing 0x1407fdf90 / add_corner_stone_railing_piece 0x1407febb0 | I 结构 + U |
| trap_doors.rs | generate_trap_doors 0x1407be730 | I 结构 + U |

## 2. O 级恢复细节（可直接验收）

### 2.1 maintain_visual_entities（0x1407f9260 / 0x1407f98c0）
- `clear_out_wall_holes` 0x1407f98c0（capdis 手解）：`ResMut<WallHoles>` +
  `ResMut<PrevWallHoles>` 尾跳 `WallHoles::clear_for_next_frame`（0x140f867c0）。
- `clear_out_visual_entities` 0x1407f9260：瞬态表 `Drain<DecoratorId, Vec<Entity>>`
  逐实体 despawn（内联宿主 impl$3::drain）+ 持久表清除
  （remove_persistent_entities 内联宿主）+ 墙洞帧推进。实体↔装饰关联靠
  DecoratorId 组件（RE-DECORATOR-STORAGE §2 一致）。

### 2.2 flowerbed_xforms（0x1407ef7b0，O 全文）
5 参面 `{out, subtype u8, mode u8, ctx, mesh_lib}`；跳表
（0x142a8ae68 / 0x142a8ae80，`subtype-1<3` 三键）按 mode 选网格名：
- mode 0/1：`decorators/window_cottage_{1x1,2x1,3x1}_flowerbed_locations`
- mode 3：`decorators/window_cottage_corner_{1x1,2x1,3x1}_flowerbed_locations`
`AssetMeshLibrary::get_by_name` → `Mesh::positions_without_indices`（元素 ÷3 =
Vec3 数）→ 逐花床变换。其余 mode/subtype fmt panic。

### 2.3 gothic_dormer_profile_curve（0x1407c7500，O 全文）
按 rank（+0x44）返回剖面控制点 Vec<Vec2>（game Vec {cap,ptr,len}）：
- rank 1：6 点 = {0,0.93},{0.15,0.78},{0.26,0.6},{0.32,0.4},{0.35,0.19} + {0.35,0}
- rank 2：7 点 = {0,1.07},{0.18,0.95},{0.34,0.81},{0.46,0.63},{0.54,0.44},
  {0.59,0.23} + {0.588,0}
- rank 3：9 点 = {0,1.45},{0.19,1.34},{0.36,1.19},{0.5,1.02},{0.61,0.82},
  {0.69,0.62},{0.73,0.4},{0.76,0.18} + {0.76,0}
其余 rank → fmt panic。常量 rne-read 0x142a867d0/0x142a86810/0x142a86840。

### 2.4 deco_differential_state 构造器（0x14010ba60 / 0x14010b940，O 全文）
`DecoDiff<C>` 96B 记录：`{header u32×6（{74.48f32,1,0,0,0,0}）, 代际 u64,
payload, {77.5f32,1}, 0,0, ptr, 0}`；线程本地代际计数器逐次递增。两泛型
实例（Chimney/DoorStairsAssemblyParams）同构。类型面（O 名）：
`DecoDiff::current`、`DecoDiffLog::with_each_deco_diff`、
`DecoRemovalLog::with_removed`。

### 2.5 stairs 小函数
- `determine_segment_type` 0x14080c810：坡度比 |Δh|/弦长 → 1 缓（≤0.25）/
  2 中（≤2.5）/ 3 陡；常量 0x142925950=0.25、0x1429cd064=2.5。
- `densely_sample_spline_segment` 0x140809ad0：utils::Spline2D::
  densely_sample_segment（utils 侧已按本函数 VA 恢复，B 模块 451 归属）。
- `determine_ladder_support` 0x1407de800（capdis 全文）：4 参面 → u8
  {0,1,2,3,4} 分支。
- `structural_graph_subscribe` 0x1407d4990：回调订阅注册。
- `create_stairs_collider` 0x1407deb60：kind==0x11 → RaycastWorld::
  remove_collider<StairGraphEdge>（0x140252ad0）；否则碰撞记录构建。
- `is_out_of_roof_bounds` 0x1407ca6c0（O 全文）：屋顶边界判定，模式表
  {2.1,0.1,0.15}（0x142a87240）+ 形状类型基 {0.5,0.1}。

### 2.6 转发器（O）
- `door_stairs_assemble_bricks` 0x1407ff130 → DoorStairs 差分重装闭包
  （0x1412af3a0）。
- `chimney_assemble_bricks` 0x1407f8db0 → Chimney 差分重装闭包（0x1412ae660）
  + DerivedDecoratorInfo::new。

### 2.7 flag（0x1407ff6c0，O 子类型判别 + 蓝图写面）
`subtype-0xd<2` → 旗（墙旗 0xd/屋旗 0xe）；==0xf → flag_connections 分支；
其余 fmt panic（flag.rs 源）。旗蓝图写面：`{1, 0, pos, pos, pos,
cloth_basis{0.5,-0.7071,0.7071,0.5}}`（O）。布基 = 四元数分量，旗布旋转合成
常量 0x142a88840..0x8c（数学链 O，语义 U）。

## 3. U 登记（显式未恢复 / 需补挖）

1. **所有 generate_* 大函数的参数排序 SystemParam 映射**：Ghidra 参数面为
   undefined8 复合，精确 Res/ResMut/Query 排序需 reviewer 结合调用点复核。
2. **Brick 装配几何主体**：chimney_assemble_bricks / door_stairs_assemble_bricks
   的砖列随机分割（random_splits 为 D1 按调用点承载，U）。
3. **旗布 / 布绳几何**：flag / flag_connections / generate_clothline 的布料
   下垂/落点数学主体（常量已登记）。
4. **窗口几何**：cottage/gothic 窗楣（add_lintels 12 参面）、hat 骨骼
   （get_gothic_hat_bones 6 参面）、半老虎窗剖面（calc_..._profile_ws）。
5. **楼梯主装配链**：playermade_stairs_assemble<true,false>（0x140008c90 /
   0x140018040 反编译超时 ×2）与 generate_playermade_stair_requests<false,true>
   （150+ callees）主体；construct_ladder / construct_stair_steps /
   gen_stair_floors / calculate_distortion_from_structural_graph 等。
6. **DormerKey 量化 / 碰撞网格**：collision_geo get_collision_mesh（递归细分）。
7. **maintain_dormer_tiles 写入 helper**：FUN_1405b0d70 / FUN_140053320 目标
   缓冲类型（0x18B 瓦元素布局）。
8. **90/54/63 段类型与装配参数**：compute_stair_assembly_params /
   derive_stair_node_params 载荷语义。
9. **core 缺口承载**（D1 于 crate 内承载，见 §5）：DormerHoleSsbo 48B 记录、
   RaycastMeshData::new_cuboid（native-physics 域）、clutter_mesh_ty_data
   （0x1408b8e90）、utils crate 的 random_splits/inverse_lerp/smoothstep/
   xz_projective_rotation_bad/xz_rotation_bad/IsoTransform。

## 4. 跨领域线索（供中央索引）

- 主要消费 core：DecoratorStorage（iter/iter_modified/iter_walls）、
  CachedDecoratorTransforms::transform、DerivedDecoratorSubtypes、
  DecoratorMeshLibrary、AssetMeshLibrary::get_by_name、PublicWalls::roof_entity、
  WallHoles::clear_for_next_frame、StairsState/StairGraphEdge、
  RaycastWorld::remove_collider、GladeType（经 ActiveSession）。
- 消费 utils crate：Spline2D::densely_sample_segment、Rectangle3d、Circle2d、
  wall_space、swizzlers、halver::to_f16（0x140c8ffb0）、AssetMeshLibrary。
- 兄弟模块接口（D1 消费契约，归各 owner）：blueprint::DecoratorBlueprints
  （insert/get 反编译 O）、decorator_visual_state（D3 已实现）、
  apply_decorator_op（D3 已实现）、utils/visual_entity_builder（D1 承载）。

## 5. 与既有代码/证据的差异与纠错

- **DormerHoleSsbo 归属**：B 中 country-core `resources/walls/dormers.rs`，
  core 侧未恢复（章节只登记构造器）；D1 于 system_decorator::utils 承载 48B
  记录 + dummy/gothic/cottage（capdis O）。建议 core-owner 后续回迁。
- **`utils::random_splits` 等**：B utils crate 函数，utils 侧未恢复；D1 于
  system_decorator::utils 承载（调用点 O），建议 utils-owner 回迁。
- **`RaycastMeshData::new_cuboid`**：B 有该构造器（generate_* callee），core
  只实现了 native-physics 域 new_convex；D1 以默认数据承载并登记 U。
- **DeferredDecoratorOpEvents 事件**：core decorator.rs 已章节化；D1 重装闭包
  消费面见 §2.4。

## 6. 验收门槛对照（DISPATCH §4）

- 签名/参数面：以 fresh 反编译 Ghidra 参数面 + 骨架 VA 为基，U 项显式登记
  （不称已恢复）。
- 实现落点：`tiny-glade/crates/systems/decorator/`（Cargo.toml/lib.rs 为组 D
  共享，D1 已并入 decorator_visual/blueprint/utils 声明与依赖并集）。
- 编译：system-decorator 目前 4 个错误全部在 D3 进行中文件
  （move_decorators_following_anchors.rs / audio/op_preview_audio.rs）；
  decorator_visual/**、blueprint.rs、utils/** 零错误。
