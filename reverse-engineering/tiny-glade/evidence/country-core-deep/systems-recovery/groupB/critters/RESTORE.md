# system-critters RESTORE.md（组 B restorer，2026-10-16）

crate 落点：`tiny-glade/crates/systems/critters/`（workspace 已挂载，`cargo check -p system-critters` 通过）。
证据基线：B patched EXE（reva/radare2/RNE oracle），`critters-behavior-analysis.md` +
`critters-extra-analysis.md` + `reconstruction/critters/**` + chapters/resources/critters.rs.md。

## 1. 覆盖（SOURCE_MAP 51 业务，全部落位）

| 文件 | 业务函数（VA） | 本 crate 实现 |
|---|---|---|
| boids.rs | BoidFlock::remove×2 (0x141dc2340/0x141dc2540)、Boid::frame_update×2 (0x141dc2740/0x141dc3510)、random_vel_change (0x141dc2170) | boids.rs：frame_update/random_vel_change/flock_remove/uniform_sample_cone |
| butterfly.rs | update_butterfly_states 0x141d84d40、goals 0x141d86060、velocities 0x141d867b0、audio 0x141d86eb0、send_to_gpu 0x141d87140 | butterfly.rs 全 5 |
| duck.rs | plan 0x14230ff00、move 0x142312200、goals 0x142313b30、velocities 0x142314470、ripples 0x142314b70 | duck.rs 全 5 |
| ducks_react_to_being_inside_buildings.rs | 0x14231b140 | 实现（墙围合 + Roof::height 覆盖） |
| duck_splash_particle.rs | 0x142322650 | 实现（粒子生命周期 + 请求事件） |
| firefly.rs | send_to_gpu 0x142316760、goals 0x142316940、velocities 0x142317040 | firefly.rs 全 3 |
| fish.rs | FishSpawnLocations::new 0x140f909f0、despawn_fish 0x140f90c10、fish 0x140f90cd0 | fish.rs 全 3 |
| lib.rs | impl$0::build 0x14067d440 | lib.rs `configure`（资源/事件/40 系统链） |
| shared.rs | avoid 0x1423179a0、smooth 0x142318c10、move 0x142318fa0 | shared.rs 全 3 |
| spawn_ducks.rs | despawn 0x140f8f8b0、DuckSpawnLocations::new 0x140f8ee90、spawn 0x140f8f210 | spawn_ducks.rs 全 3 |
| ui_pet_ducks.rs | 0x142322e30 | 实现（PetDuckTarget + 音频缝） |
| wren.rs | spots×2 0x14231c7d0/0x14231d6c0、update_wrens 0x14231e390 | wren.rs 全 3 |
| wren_boids.rs | sim_wren_boids 0x142326a80 | wren_boids.rs（出生/上限 80/随机转向/可行点/出界清理） |
| sheep/mod.rs | impl$0::build 0x14067d810 | sheep/mod.rs（模块面） |
| sheep_behavior.rs | goals 0x142315050、velocities 0x142315e40、audio 0x142316490 | 全 3 |
| sheep_floatie.rs | 0x142320cf0 | 实现 |
| sheep_input_record_replay.rs | 0x14231b720 | 实现（InputStream<SheepInputFrame> 包装） |
| sheep_pet_animation.rs | 0x14231ac20 | 实现（五阶段 pet 状态机 = reconstruction sheep_pet.rs） |
| sheep_react_to_being_inside_buildings.rs | 0x14231f610、0x142320480、0x14231ff70 | 实现（emoji 生成/生命周期） |
| sheep_terrain.rs | 0x14231bd80 | 实现（= reconstruction sheep_terrain.rs 弹簧阻尼） |
| sheep_umbrella.rs | 0x142321980 | 实现 |
| sheep_unstuck.rs | 0x1423246b0 | 实现（legacy_spherecast + 地形传送） |
| sheep_walk_animation.rs | 0x14231a630 | 实现（being_petted 门控行走周期） |
| ui_pet_sheep.rs | 0x142324f80 | 实现（PetSheepTarget） |

## 2. 与 reconstruction 映射（复用不重挖）

- boids 规则/参数（sep_r 0.8/1.3、align 0.0584/0.1169、min/max 档、凝聚 0.0056、分离 1.7、
  邻居半径 4.0、heading 过滤）：与 reconstruction/critters/src/{fish,wren_boids}.rs 逐式一致。
- firefly 惯性 bias：**有目标 +0.5 / 无目标 -0.5**（RNE 0x142ccfc58 双浮点 0xbf0000003f000000；
  reviewer-B 抽验纠正——critters-extra §4"校正 1"的 -1.0 是误读）。以 reconstruction/firefly.rs 为准。
- smooth_velocity / move_critter_core / face_movement / decide_avoidance：
  reconstruction/critters/src/shared.rs 一致。
- sheep_terrain 弹簧阻尼（增益 10/20、clamp [-2,2]/[-5,5]/[-0.3,0.3]、-0.05 阈值、+4 身位）：
  reconstruction/critters/src/sheep_terrain.rs 一致。
- sheep_pet 五阶段（0→1→2→3→4，幅度@0x58、控制@0x60）：reconstruction/critters/sheep_pet.rs 一致。
- Duck/DuckState 语义（1=正向/2=自由游/3=反向；plan 出生 progress=1.0、state 3）：
  critters-extra §6 一致。

## 3. 契约与缝（consumer）

- 核心消费：`resources::critters::CritterGoal`（16B Option<Vec3>）、`GameTime`（delta@+0x28）、
  `WorldRaster<512>::as_ref → RasterBufferRef<512>::bilinear_sample_at_world_xz(x,z,130,130)`、
  `TerrainHeightsData::sample_at_world_xz`、`terrain_rock_mask::sample_at_world_xz`、
  `WallSpatialHash::query_with_aabb_clamped + segments_for_wall`、`PublicWalls::get/roof_entity`、
  `PublicWallState::max_y_gable/max_y/simplified_curve_2d`、`Roof::is_flat/is_flat_w_no_rim/height`、
  `mode_manager::AppMode`、`camera_rig::main_camera::MainCamera`、`CursorTerrainRaycast::position`。
- render_loop consumer：`ButterflySsbo`（24 上限，anim_t = Butterfly+0x14 phase_ref）、
  `FireflySsbo`（96 上限）——`butterflies_send_to_gpu`/`fireflies_send_to_gpu` 产实例数据，
  缓冲上传宿主接线（GPU 后置）。
- 装配缝（host 注入资源）：`RaycastWorldHandle`（legacy_spherecast 契约：固定 10 槽序
  {0,1,3,7,6,4,11,2,12,13}、紧凑结果 marker@+0x30；非 native 壳返回未命中）、
  `TerrainRockMask`、音频 `CritterAudioSink`（blanket impl 到 core `AudioInstance<S>`，
  各方法 = 已证原生调用 0x140a57300 等）、`DuckAudio`（Quack/TakeOff/Land，Duck::new 装载）。

## 4. 编译与验证

- `cargo check -p system-critters`：通过（20 警告：未用导入/未用变量，无错误）。
- 验证方式：临时 workspace（tmp-tg-check，country-core 以 group C 进行中状态 + 验证 stub
  tiles/surface/ridge/edit_rules/RoofTypeChanged 补齐）编译通过；country-core 的
  systems/roof 集成完成后应在主 workspace 直接编译（届时删除 temp）。
- 功能测试：reconstruction 侧既有单元测试仍可用（语义核心未变）。

## 5. 未决（不伪造）

- CritterVelocityMultiplier 语义（各系统写查询环境值，unknown）；Butterfly 跳表 handler
  （DAT_142f68008/018）精确分支（状态动画归宿主渲染域）；Duck+0xe8=0.1 写（dwell[1] 槽
  推断）；fish `is_corrupted` 速度档（宿主注入）；wren 摇树逻辑（证据仅确认日志/上限）；
  ui_pet_* 的 signifier 流（toolbar 批次）；BoidFlock 0x38 槽精确布局（以 turn_timer 投影）。

## 6. FIX 交接（reviewer-B 21 条，2026-10-16 第二轮）

F1（being_petted 0.01）中央裁决撤回——0.01 正确（0x14089d4c0 → ucomiss 0x142960674 =
0x3c23d70a；reviewer 的 1.48e-41 为 rip 解析错误），未改动。

| # | 修复 | 落点 |
|---|---|---|
| F2 | boids 加速度增量 clamp [-max, max/2]（max=1.0 / 有目标 1.5；clamp::do_panic 断言） | boids.rs `frame_update(.., accel_max)`；fish/wren 调用点按"水>=0.5 且有目标"选 1.5 |
| F3 | sim_wren_boids 补齐：RAY_DIRS 32 方向（OnceLock 固定表）、closure_3 raycast 可行点、FreeSpot 收集（BirdSpots 未占用）、BirdHaven 树查询 + 摇树出生（日志 + burst rng*5+1 + 80 上限）、出生/循环音频（CritterAudioCore 缝）、反射 = from_rotation_arc 转 RAY_DIRS | wren_boids.rs 重写 |
| F4 | wren.rs 三函数真实实现：create_spots 池分配 + recycle 回池（屋顶移除）、Rectangle2d::get_edge_pts + roof_noise::apply_roof_noise + Roof::height + calc_roof_tip_center_ls + random_splits 边沿采样、update_wrens 音频 start/stop + 光标飞离 | wren.rs 重写 + bird_spots.rs recycle/create_spots |
| F5 | fish 实体转向 target = 最近他鱼（非自身）；min/max 档位谓词（is_corrupted）仍宿主缝 | fish.rs `nearest_other` |
| F6 | sheep_floatie/sheep_umbrella 真实生成：spawn_empty + AudioCore::instantiate_audio + start_if_not_playing；出水/雨停 despawn；owner 绑定 | sheep_floatie.rs / sheep_umbrella.rs 重写 |
| F7 | duck_splash_particle 新请求 spawn 粒子 + 资产句柄（AssetMeshLibrary/AssetShaderLibrary "duck_shader"；经 MeshLibraryHandle/ShaderLibraryHandle 缝） | duck_splash_particle.rs |

编译：temp workspace 三 crate 全通过；已同步 `tiny-glade/crates/systems/critters`。
