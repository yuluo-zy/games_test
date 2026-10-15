# systems/decorator · decorator_visual 视觉族（非楼梯）B 版分析

> 由批次 3 分析 agent 产出，父级代为落盘。

范围：主干/stairs/ui_systems 由并行 agent 覆盖。证据 = Ghidra 持久 B 库只读反编译（`--worker shared`，无写入）；地址 = 骨架「编译实例 RVAs」+0x140000000；下述为 RVA。状态标注 O/I/U。

## 通用结构（跨函数复用）
- **WindowVisualEntityBuilder**（0x160 字节/条，全族批量 push）：+0x00 实体区（+0x18 排序键 subtype*1000+deco_id，+0x28 全局实例 hash）、+0x44 稀疏分配、+0x84/+0x94 子类型、**+0xa0 mesh 名 ptr / +0xa8 mesh 名 len**、**+0xb0 shader 名 ptr / +0xb8 shader 名 len**（len 即字符数，如 0x19/0x1e/0x24/0x2a/0x35）、+0xc0 pos、+0xd0 quat、+0xdc scale、+0xf0=1、+0xf4 DecoratorRank、+0x108..0x110 垂直范围。O
- **rhapsody ResourceAccessInfo** 向量：+0x10 len/+0x1c 计数，元素 0x14 字节 `(u32,u32,f32,f32,u8)` 碰撞/实例。O
- **DecoratorCollision 表**：`HashMap<DecoratorLibKey,DecoratorCollision>`，+0x30 为碰撞半尺寸相关值。O
- **DormerHoleSsbo**：0x30 字节/条；`::cottage()`/`::gothic()` 追加屋顶洞。O
- 多数生成系统先查 `DEV_OVERRIDE_IS_DEMO_MODE`，非零直接返回。O

## 一、状态/维护
1. **WallAttachedDecoTracker add/remove（4 单态）** 0x12ad540/0x12acd70/0x12ae1c0/0x12add20。类型（O，PDB）：`HashMap<WallId, WallDecoDiff<T>>`，`WallDecoDiff<T>=HashMap<DecoratorId, enum DecoDiff<T>>`。add：按 WallId（`DecoratorStorageAddr::as_wall`）hashbrown 开放寻址找/建外层桶，再按 DecoratorId 向内层插 40 字节 diff 值；维护 count/len。remove：缺失→panic_fmt（fmt 链 anyhow+walls+decorator）。`with_diffs`/`on_wall_*`/`DecoDiff::current`/`DecoDiffLog`/`DecoRemovalLog` 均内联（模块 1561）。Default(0x10ba60 实为 `impl$2::default<ChimneyAssemblyParams>`) 初始化 RandomState+匿名静态。
2. **chimney_assemble_bricks** 0x7f8db0(size117)：薄壳。签名 `(DerivedDecoratorInfoResources, Res<PublicWalls>, Res<AssetSsboLibrary>, ResMut<WallAttachedDecoTracker<ChimneyAssemblyParams>>, Res<TerrainHeightsData>, Res<UiSignifierStream>, EventWriter<PlayDecoratorSoundCmd>)`；推进计数→`with_diffs<ChimneyAssemblyParams,closure_env_0>`。主体内联（U）。
3. **door_stairs_assemble_bricks** 0x7ff130(size52)：同壳，`with_diffs<DoorStairsAssemblyParams,...>`。
4. **clear_out_visual_entities** 0x7f9260(size1295)：drain `HashMap<DecoratorId,Vec<Entity>>`（hashbrown 位图）逐实体 `EntityCommands::despawn`。
5. **clear_out_wall_holes** 0x7f98c0(size31)：推进两个洞缓冲计数 + `WallHoles::clear_for_next_frame`。
6. **maintain_dormer_tiles** 0x7ce920(size321)：二维瓦片网格差分（行/列 0x18 stride），删瓦片→`RawTable::remove_entry<u64,Box<dyn FnOnce>>`（rhapsody 稀疏分配注册表）+ drop `SparseInstanceBufferAllocation<RoofTileSsbo>`。

## 二、墙体/窗/门族
7. **generate_arrow_slits** 0x7bbee0(size7827)：subtype 1x1/1x2/1x3/0x17(半山墙) → `get_gothic_hat_bones` 尖拱骨骼→中点+`xz_projective_rotation_bad` 求变换；mesh `"decorators/arrow_slit_1x{1,2,3}(_halfdormer)"`（len 0x19/0x24）、shader `"nani_solid_vertex_color_window"`(0x1e)；碰撞 0x14 元素 1x1→(0.5,0.5)、1x2→(0.4,1.5)、1x3→两条(0.4,1.7)+(0.605,0.55)；0x17 追加 `DormerHoleSsbo::gothic`。
8. **generate_roof_chimneys** 0x7c0910(size4073)：subtype 0x15(21)；`as_chimney` 取高度；烟雾视觉（mesh 名 1/len0，shader len10）+ cuboid 碰撞（40 字节：`[0]=2,[1]=底座y-wall.max_y,[2..3]=xz,[4]=y_min,[5]=y_max,[6]=quat,[9]=高度+3`）。
9. **generate_wall_chimneys** 0x7c21b0(size9538)：subtype 0x13..0x14；vent_pipe 视觉(`"decorators/vent_pipe"` len0x14)；`box_trigger_volume`+`CurveSlice::extend_right_until` 求墙曲线重叠，clamp[1.5,5]，≥5 跳过；vent_pipe 三角→`new_indexed` 三角碰撞 + cuboid（`[0]=有碰撞flag,[1]=重叠量`）。
10. **generate_cottage_wall_windows** 0x7ebfd0(size11843)：窗+`add_lintels`(§14)+`flowerbed_xforms`(§13 随机花坛)+`DormerHoleSsbo::cottage`；mesh 名静态表（U）。
11. **generate_cottage_corner_windows** 0x7f66b0(size8168)：矩形角窗，`WallCornerAttachment::to_rectangle_corner_id`+`Rectangle2d::get_corner_normal/get_edge_dirs_from_corner_id`，quat `^0x8000` 符号翻转；+`add_lintels`+`has_cloth`。
12. **generate_cottage_balcony_doors** 0x7f3da0(size7226)：`DragState::is_started`、`check_for_door_stairs`(内联)、`bilinear_sample_at_world_xz<TerrainHeightFloatGrid>`(门腿接地)、`add_door_autoclutter`(内联)；mesh `"nani_balcony_door"`。
13. **flowerbed_xforms** 0x7ef7b0(size472)：`fn (subtype,height)->Vec<Transform>`；按族静态表取 mesh→positions+normals→zip→closure→Vec<Transform>。
14. **add_lintels** 0x7ef990(size2626)：`fVar20=(y+1.43)*0.5`；mesh `"decorators/setdressing_window_lintel"`(0x24)、shader nani_solid_vertex_color_window(0x1e)；scale 首尾 0.82/中部 0.64（subtype≥2），flag bit0 乘 height/0.9。
15. **generate_gothic_wall_windows** 0x7e87b0(size8889)：`WallStyle::is_halftimbered`+`get_gothic_hat_bones`+`get_window_holes`(内联)+`NameHash::compute`(mesh 名 name-hash)；shader `"nani_solid_vertex_color_bricks"`/`"nani_gothic_window_bricks"`(0x19)；+`DormerHoleSsbo::gothic`。
16. **get_gothic_hat_bones** 0x7eb500(size1181)：墙曲线采样两索引→边缘方向+法向（`1/SQRT`+`xvy`）→输出 2×2 旋转基 + 归一化边向。
17. **calc_gothic_half_dormer_profile_ws** 0x7eb160(size719)：反编译被内联打散，仅剩分配+for_each 骨架；生成半山墙剖面（U，需 P-code）。
18. **generate_gothic_balcony_doors** 0x7f0900(size9070)：`WallStyle::is_halftimbered`+`get_door_holes`(内联)+`add_door_autoclutter`+地形/`RasterBufferRef<512>` 接地；mesh `"nani_balcony_door"`。
19. **check_for_door_stairs**：内联（无独立符号；调用记录 0x7f2025→0x7f5e70）。检测门下方玩家楼梯（I）。U。

## 三、dormer 族
20. **generate_cottage_dormers** 0x7c7ac0(size9285)：subtype 2；`get_collision_mesh` 取碰撞；按形状(1/2/3)沿屋顶曲线(`as_curve3`+`get_coord_at_u`)采样 gable/hip 墙 quad（0x30），`1/SQRT` 法向+`xz_projective_rotation_bad`；`rng`+自研 hash(0xa0761d6478bd642f^0xe7037ed1a0b428db) 随机位；push 墙视觉(mesh DAT len0x2a)+窗玻璃`"decorators/window_cottage_1x1_dormer_glass"`(0x2a)+`DormerHoleSsbo::cottage`。
21. **generate_dormer_roofs** 0x7cb1a0(size9771)：差分 dormer 瓦片；subtype 掩码 `0x800186`(1,2,7,12,16,23)+0/WallStyle3；gothic(8)→`gothic_dormer_profile_curve`→变换到墙空间→`Curve::try_new_from_points`→`calc_gothic_half_dormer_profile_ws`+`get_gothic_hat_bones`→`assemble_gothic_dormer_tiles`；0x17→2 点剖面；其余→`assemble_dormer_roof_tiles`(内联)；写入 `SparseInstanceBuffer<RoofTileSsbo>`+`HashMap<Entity,InstanceHandle>`。
22. **get_collision_mesh** 0x128a0b0(size8051)：`DormerKey{i32 x,i32 z,i32 count,i8 roof_idx}`（round(dist*5) 量化）缓存 `HashMap<DormerKey,Arc<RaycastMeshData>>`；未命中→逆旋转矩阵+沿屋顶形状两侧 u 插值采样生成三角带→`new_convex`→插缓存。
23. **gothic_dormer_profile_curve** 0x7c7500(size392)：按 subtype 返回硬编码剖面（sub1 6 点、sub2 7 点、sub3 9 点，8 字节=2×f32；sub1 常量 0x3f6e147b00000000,0x3f47ae143e19999a,... ≈ (0,0.93),(0.15,0.78),(0.26,0.6),(0.32,0.4),(0.35,0.19),(0.35,0.35)）。
24. **generate_gothic_dormers** 0x7c5650(size6662)：subtype 8；高度常量 1.54(sub3)/1.2(sub2)/0.72(sub1)；mesh 名 `DAT_142a86e38`+子类型、shader `"nani_solid_vertex_color_(window)bricks"`(0x1e)；+`DormerHoleSsbo::gothic`。
25. 内联登记：`dormer_roof_height`/`is_bay_window`/`DormerKey::quantized_from`/`cottage_dormer_dims`/`get_dormer_panel_xforms` 均无独立符号。

## 四、旗帜/布条/灯笼/杂物
26. **generate_flag_connections** 0x7df470(size4852)：收集旗 CandidateInfo(0x50：+0x10 pos,+0x30 wall_id,+0x4d 可用)；case 2/3/4 朝向旋转（0.70710677 四元数+前伸 0.9/0.48/-0.7）；`greedily_connect`→每对连接求两骨骼最近点→对「A→最近点」「最近点→B」各调 `clothline`。
27. **greedily_connect** 0x7e1430(size3054)：按距离排序候选对；判据：异墙 dist²<min²、同墙 band(max²,min²)；贪心主循环：两端点容量<max_conn、`2|dz|<=水平距`、dist>0.02、`RaycastWorld::intersects_any` 无遮挡→连（`HashMap<(usize,usize),()>` 去重，计数+1）。
28. **clothline** 0x7e2510(size2552)：`n=max(2,ceil(dist/间距))` 等分插值，每次 `WindowVisualEntityBuilder::build<DecoratorTransientVisualState>`。
29. **generate_clothline_visuals** 0x7e5190(size8528)：读 `ClothInfo`(0x14/条)→CandidateInfo；mesh `"nani_cloth"`+NaniBundle/SolidVertexColorInstanceData/ChunkId 落 GPU 稀疏实例。
30. **generate_flags** 0x7ff6c0(size11875)：落位分型 mesh `"decorators/flag_terrain_support"`/`flag_wall`/`flag_roof_beam`/`flag_roof_cloth`/`flag_corner_beam(_no_connection)`；`RaycastWorld::legacy_raycast_w_params` 遮挡检测；+0x28 实例 hash。
31. **generate_lanterns** 0x78d320(size10195)：落位分型 `"decorators/lantern_wall"`/`lantern_flat_roof(_inside)`/`lantern_terrain(_support/_inside)`；shader `"nani_lantern_emissive"`；`Roof::is_flat` 判平顶。
32. **maintain_lantern_lights** 0x790400(size3533)：遍历灯笼 SSBO+屋顶瓦片位图；`rng*0.14+0.01` 闪烁阈值+时间窗(0.45..0.72)+`deny_load` 判定点亮；写 `(ColorId,bool)`；记录 `(DecoratorId,GpuLightId,ColorId,f32,Vec3)` 上限 0x8000；sort+`compare_light_lists` 双缓冲差分→`GraphicsDevice::update_buffer_bytes` 上传（SsboId→BufferState FrozenMap，名 hash 断言 0x35）。
33. **generate_trap_doors** 0x7be730(size4956)：仅平屋顶；subtype 0x0a/0x05；瓦片位图定位+`Circle2d::intersect_circle`/`Rectangle2d::contains_point_padding` 校验；cuboid 碰撞+视觉(mesh DAT 分型，len 0x1b/0x1c)+`DormerHoleSsbo::cottage`。
34. **generate_decorator_clutter** 0x7e3370(size4230)：杂物 subtype 集合(switch 0x00,07,0d,13,2b,0e,14,17,18,28,19,1a,29,22,23,24,2a)；`clutter_mesh_ty_data` 取网格+bbox→cuboid 碰撞（min 0.1）；push 视觉；每 subtype 挂高常量（case 3/4→-0.25/3.02；9→0.3；0x10→0.92/1.59；0x11/12→0.945；0x2c→1.56）。

## 五、总览
本族 = 墙/屋顶装饰的视觉+碰撞+屋顶洞差分生成集群，统一产出：①WindowVisualEntityBuilder 视觉记录（mesh/shader 名+len、位置/quat、垂直范围、等级）②DecoratorBlueprint 碰撞（new_cuboid/new_convex/new_indexed + 40 字节块存 blueprint[0x23]）③DormerHoleSsbo 屋顶洞（cottage/gothic）④WallAttachedDecoTracker 双层差分驱动 chimney/door-stairs 砖列（closure 内联）。
算法清单：差分重建 / 双层差分索引 / 砖列块装配 / dormer 剖面曲线(硬编码/采样) / DormerKey 量化缓存凸包 / 旗帜贪心连接(容量+高度+遮挡) / 布条等分插值 / 灯笼分型+闪烁双缓冲差分上传 / 门窗过梁(0.82/0.64)+花坛 / 屋顶洞 / 杂物 bbox 碰撞。

## 未知清单
1. `DecoDiff<T>` 变体名、40 字节 `Chimney/DoorStairsAssemblyParams` 布局（U）。
2. chimney/door-stairs closure_env_0 砖列主体（内联，U）。
3. `calc_gothic_half_dormer_profile_ws` 反编译打散，需 P-code（U）。
4. `check_for_door_stairs`/`add_door_autoclutter`/`get_door_holes`/`get_window_holes`/`dormer_roof_height`/`is_bay_window` 内联细节（U）。
5. 各窗/门/旗/灯笼 subtype→mesh 静态表（DAT_142a8xxxx，U）。
6. `assemble_dormer_roof_tiles` 瓦片间距常量（内联，U）。
7. `maintain_lantern_lights` 的 compare_light_lists/ColorId（内联，U）。
8. `get_gothic_hat_bones` 输出骨骼布局（U）。
9. `is_out_of_roof_bounds` 返回极性（I：u 区间是否覆盖矩形四角）。
10. `has_cloth` 触发条件（U）。
11. gothic_dormer_profile_curve 点对低/高字拆分语义（U）。
