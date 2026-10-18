# system-critters 剩余实现 RESTORE（restorer-critters-remaining，2026-10-17）

- 范围：`crates/systems/critters` 剩余函数；硬标准逐语句判等（fresh 反编译 + 定点反汇编 + 常量实读）。
- 证据目录：`work/restorer-critters-remaining/{jobs,results,decomp}/`
  （Ghidra shared 只读 fresh：`01-targets` 13 函数反编译、`02/03` 反汇编、`08/10/12/13/15`
  常量与字符串实读）。B patched EXE 身份沿用项目默认。
- 未改 shared/country-core；既有 FIX（wren 三轴/anim×7、water2=Sheep、butterfly flag 基线、
  avoid 两位置 4+4 采样）**未回退**（`shared.rs`/`butterfly.rs` 未触碰）。
- 验证：`cargo check -p system-critters` **0 error**；`cargo test -p system-critters`
  4/4 通过（numeric_assertions）。

## 三态总表

| 函数 | VA | verdict | 关键依据 / 分歧 |
|---|---|---|---|
| despawn_fish | 0x140f90c10 | **判等**（资源重置） | 清空 `ResMut<FishBoids>` 两段存储 + `FishSpawnLocations::new()` 覆盖；`insert_resource<FishBoids>` 符号 0x140f84e80 证实 Resource。crate 以实体组件投影 boid 0x18 槽，资源侧表示仍被重置；不再每帧跑（真实注册面 `OnExit<AppLoadingState>`） |
| plan_duck_trajectories | 0x14230ff00 | **重建** | 主路径逐语句：plan 结构 `Vec<Trajectory>`+offset+key+age；采样 `len==1 → (p+off*0.5, shape(0.5))`、`len>1 → t=(idx/count)*0.05+2.15, mid=lerp(x/z), y=shape(|first.xz-mid.xz|)`；closure_env_5=identity(0x14230fa40 空体)/closure_env_4=`log1p(max((t/3)³,1e-10))`(0x14230fa90)；`legacy_raycast` 命中码!=2 且命中距²<=段长² → 重采样并重置轨迹；末点 `x²+z²>25350` → done；spawn 反转点 + `DuckState=0`（bundle 常量 local_2d8=0）+ `"duck_shader"` 句柄；keyed 计划更新鸭 `progress=1.0, state=3`；`retain(age<0x12d)`。登记：`Duck::new` 前的 uniform_f32 实参 U；资产 unwrap 以 Option 缝承载；DEV_OVERRIDE/按钮/光标为 host seam |
| move_ducks_along_trajectories | 0x142312200 | **重建** | 状态映射 0/1→正向、2 跳过、3 反向；`target=dir*0.5+0.5`；结束帧 `state=2`（反向 despawn+state=3）、goal=`pos+(rot*Z).xz`、`moving/aux=1/0`、`dwell=(4.0,2.0)`（0x142b2e410 实读）、multiplier=1；移动帧 chord=首末水平距、`progress += dir*12*dt/chord`（12.0@0x1429cd2d4）、`t=(dir>0?p:p²)(len-1)`、`CurveInternalCoord::new_for_point_count`、正向水平/反向 3D 方向（x0y 0x140c9cdc0）、`Velocity=(x0y(lerp-old))/(2dt)`（0/0.3 分支、NaN 保留）、`f_f4=clamp01(inverse_lerp(1-20/chord,1,progress))`、`ObjectGrassPush=2*smoothstep(clamp01(2*f4-1))`、音频三分支、`f4>0.25 && state==0 → state=1 + EventWriter<SpawnSplashParticle>(entity)`、`f_f0 += (4s2+1-s2)dt`（a=1-50/chord,b=1-20/chord）。登记 U：`from_rotation_arc` 两端向量（内联 glam，寄存器未闭合）、AudioLoop 两处 set_parameter、`set_volume(WaterAudioGrid)`（core 无 FloatGridCells 实现）、demo gate |
| update_decorator_bird_spots | 0x14231c7d0 | **重建** | 符号签名证实 `Query<(&SittableForBirds,&Transform)>, ResMut<BirdSpotManager>, Local<DecoratorBirdSpots=HashMap<u32,Entity>>, Query<&BirdSpots>, Commands`；世界点=translation+rot*offset；三轴有序相等（NaN 不等）→ 保持；否则 remove_entry+despawn+重建（gen_id→create_spots→spawn insert<BirdSpots>）；retain 存活 id。类型 gap：`SittableForBirds` 在 core 仅注册未定义，crate 以同名组件承载 |
| update_roof_bird_spots | 0x14231d6c0 | **未实现（部分重建）** | 已落：事件驱动（RecalculateRoofCmd/OnWallChanged）、roof_entity 缺失→recycle、形状门（截面 tag==1 且 !is_flat）、RoofRidgeDims::from + calc_roof_tip_center_ls + Rectangle2d + get_edge_pts、边距>0.5 门。未实现：init_rng 种子实参、random_splits count/amplitude、逐点 fastrand f32/Roof::height/xvy/apply_roof_noise 采样平面、ClutterVersion::is_initial 选边、UiSignifierStream 调试与 wren 重定位。解锁：fresh 反汇编逐寄存器闭合上述实参 |
| ducks_react_to_being_inside_buildings | 0x14231b140 | **未实现（需重做，部分重建）** | 已落：state==2 门、`GladeBorder::is_pos_inside` 门（可选资源）、逐墙 2D 围合、`y <= max_y + Roof::height` 上界、`start_if_not_playing`。分歧：墙组 `+600 计数<=1 && +0x280 水位<=1.5` 在 core `PublicWallState` 无字段（近似为 all() 遍历）；逐轮廓点 y 与水面比较未建模 |
| duck_splash_particle | 0x142322650 | **判等** | age `+= dt/2.5`；`<=1.0` clamp01（MAXSS/MINSS、NaN 保留）否则 despawn；`EventReader<SpawnSplashParticle>`（载荷 Entity/EventInstance 16B；注册符号 0x140f94300）→ `Query<&Transform>` → `"splash_particle"` mesh/shader（后者 unwrap）→ spawn `(DrawableMeshBundle, SplashParticle, DontCastShadow)`；组件 f32@+8 保持 |
| ui_pet_ducks | 0x142322e30 | **判等（核心段）+ 未实现（UI）** | 每鸭 `clamp(f_f8 ± dt/0.4,0,1)`（frozen→+/非→−），`frozen && f_f8>=1.0` → 释放（frozen=0）；NaN 语义与 MAXSS/MINSS 一致。UI 段（0x13+mode3/0xd/6+子字节 gate、`format!("pet_duck{entity}")`、WidgetParams/UiState::add/intersecting/is_active、signifier、`slider3d::into_drag_state` → state3 frozen+音频 / state4 head signifier）未实现：core 缺 `UiWidgetContext::intersecting`/`slider3d::into_drag_state` |
| ui_pet_sheep | 0x142324f80 | **未实现（部分重建）** | 已落：同一 gate 的控制流注释、mode==3 时 `dist(eye,sheep)>5.0` 跳过、`PetSheepTarget` 记录。**已删除**旧版无条件写 `behavior_state=1/goal_timer=14.0`（该写只发生在 drag 状态 4 分支，无 into_drag_state 不可伪造）。签到/`format!("pet_sheep{+0x20}")`/intersecting/ws_to_cs(0.075→clamp1)/signifier/WindowFocusEvent(DecoratorPet 载荷)/``Sheep+8={1,14.0}` 写未实现 |
| sheep_unstuck | 0x1423246b0 | **重建** | fresh 修正：内层分支 `bucket=clamp_u32(state-2,0..3)`（u32 **回绕**：state 0/1/5/7+ → bucket3 高度比较）；state2 水>0 → corrupted?4:0；ddbb 记录 (flag=1,h=max(h,0.5)) 与 `|Δh/dt|>10`；岩石+{2,4,6} → state=0；**尾部不再清地面记录**（仅 state!=2 且 flag_30==0 清 shared_timers）；cd1 只清记录低 4 字节（h 保留）。DIVERGE：spherecast ABI 命中码、rock 逐点采样、is_corrupted |
| sheep_floatie | 0x142320cf0 | **判等** | 符号签名证实双查询 With/Without<Sheep>；首帧 spawn DrawableMeshBundle+SheepAccessory；整份 Transform 复制；state==4 与 audio_started 翻转**各播一次** `"event:/SFX/Sheep/UmbrellaClose"`（0x14304b9b8,len30）；`progress=clamp(progress±2*dt)`；`scale=BackOut(progress)*羊 scale.x`（0x14304ba28 tag=5 实读，纠正旧 CubicOut）；visible=state==4 |
| sheep_umbrella | 0x142321980 | **判等** | 同构；`mask=0x1a>>clamp(state-2,..3)&1`（u32 回绕）；翻转播同一事件（0x14304bac8,len30）；`BackOut`（0x14304bb38 tag=5）；三轴 scale=ease*羊 scale.x；visible=mask |
| sim_wren_boids | 0x142326a80 | **重建** | 符号签名证实 `Raycasting, Res<GameTime>, Res<CritterControl>, Res<AudioCore>, Res<UiSignifierStream>, ResMut<WrenBoids>, Query<(&mut Transform,&mut Wren)>, Local<WrenRng>, Query<&BirdSpots>, Query<(Entity,&mut BirdHaven,&PrefabInstanceData),Without<TreeMarkedForDeletion>>, Query<(&PrefabInstanceData,&BirdHaven),Added<TreeMarkedForDeletion>>, Commands`。已落：`CritterControl+5` 门（!=0 模拟；==0 清 WrenBoids+逐实体 despawn）；补满 80（x=u*130-65,y=u*9+1,z=u*130-65; vel=u-0.5）；音频 `"event:/SFX/Birds/takeoff"`（0x14304c383,len24）；`WrenBoids` 资源三数组（boid/states/entities）；FreeSpot 池 + `HashMap<Entity,FreeSpot>` 认领（<=8.0）；保留条件 sqrt(x²+y²)<=91.92388 && y<=30；frame_update(1.3/0.1169/7/14)；swap_remove 三数组+despawn；haven 摇树 burst/timer/出生式。登记 U：FreeSpot 释放时机、`Added<TreeMarkedForDeletion>` 动作（现仅释放冠球内认领）、NaniBundle<WrenInstanceData> 实例描述（host render 缝）、UiSignifierStream 调试、32 向 viable 表 |

## 计数（按函数面）

- **判等 5**：despawn_fish、duck_splash_particle、sheep_floatie、sheep_umbrella、ui_pet_ducks（核心段）。
- **重建 6**：plan_duck_trajectories、move_ducks_along_trajectories、update_decorator_bird_spots、
  sheep_unstuck、sim_wren_boids（+ update_roof_bird_spots 的结构段）。
- **未实现 3**：update_roof_bird_spots（采样段）、ducks_react_to_being_inside_buildings
  （墙组/水位字段）、ui_pet_sheep（UI 接线）。

## 关键证据

- 系统签名/类型：`find_functions` 泛型符号（`InfallibleSystemWrapper`/`FunctionSystemState`）
  给出 12 个系统的完整参数面（含 `WrenBoids`、`Local<DecoratorBirdSpots>`、`EventWriter<SpawnSplashParticle>`、
  `CritterControl+5`、`SittableForBirds` 0x10 布局）。
- 常量实读：0x142b2e410=(4,2)；0x1429cd2d4=12；0x14295c698=-20；0x1429cd070=50；
  0x142925950=0.25；0x14295a0e4=-1；0x1429258e0=(0.5,0.5)；0x142930d70=0.05；0x14304a7b8=2.15；
  0x14304ba28/0x14304bb38 tag=5(BackOut)；字符串 0x14304b9b8/0x14304bac8=UmbrellaClose、
  0x14304c383=Birds/takeoff。
- 闭包：closure_env_5=identity、closure_env_4=`log1p(max((t/3)³,1e-10))`、
  retain=age<0x12d（0x141dbd970）。

## 残留/解锁条件（汇总）

1. `from_rotation_arc` 端点（move_ducks/plan 姿态）：fresh 内联 glam，寄存器未闭合 → 以
   `(Vec3::Y, dir)` 落地并登记。
2. `WaterAudioGrid` 双线性采样：core 未实现 `FloatGridCells` → move_ducks `set_volume` 跳过。
3. roof bird spot 采样实参（init_rng 种子/random_splits/height/noise）→ 采样段未实现。
4. ducks_react 的墙组 `+600` 计数与 `+0x280` 水位：core `PublicWallState` 无字段。
5. UI 面：`UiWidgetContext::intersecting`、`slider3d::into_drag_state`、DecoratorPet 事件载荷。
6. sim_wren_boids：NaniBundle 构造、FreeSpot 释放时机、tree-deleted 分支、signifier 调试。
