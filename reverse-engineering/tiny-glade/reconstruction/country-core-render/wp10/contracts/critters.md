# WP10 契约 2/3 — `resources::critters`（小动物 CPU 状态）+ `systems/critters` 系统

版本 **B**。骨架证据：`skeleton/B/crates/country-core/src/resources/critters.rs{,.sheep.rs,.sheep/control.rs}`、
`country-core/src/systems/render_loop/resources/critters.rs`、`skeleton/B/crates/systems/critters/*`（系统 crate）。
反编译：Ghidra `0x142318fa0`(move_critters)/`0x142318c10`(smooth)/`0x142316940`(update_firefly_goals)/`0x142316760`(fireflies_send_to_gpu)/`0x141d87140`(butterflies_send_to_gpu)/`0x140f85bb0`(Duck::new)/`0x140f86330`+`0x140f86340`(BirdSpotManager)/`0x140f8f210`(spawn_ducks)/`0x140f8ee90`+`0x140f909f0`(Duck/FishSpawnLocations::new)。
标注：**O**=反编译/PDB 直接支持；**I**=调用链/名称组合；**U**=未恢复。

## 1. `resources::critters` 类型清单（O，structures `named_symbol_entities`，均 name_and_relations_only）

| 类型 | 角色 | 证据 |
|---|---|---|
| `CritterControl` | ECS 资源（全局控制开关） | `impl$0::default` 行 32（0x140f85b90 空 default）；`impl$6::drop` 行 293（0x14089a380 Arc+Vec 清理，语义 U）；`update_firefly_goals` 读 `*res+4 == 1` 门控（O） |
| `Critter` | 组件（小动物通用标记） | 名 O；字段 U |
| `CritterGoal` | 组件（目标点） | duck 恢复：`active u32 @+0`，`pos Vec3 @+4`（0x10 槽，reva 反汇编） |
| `CritterLocomotion` | 组件（步态 u32） | 值 2/3/4/6/7 + 位测（reva，shared.rs 草稿）；枚举语义 U |
| `CritterRng` | 资源（perchance 上下文） | `impl$15/23::clone_behavior`（行 87/136）；系统注入 RNG |
| `CritterVelocityTarget` | 组件 | `dir Vec3 @+0`，`retention_per_second f32 @+0xc`（0x10 槽） |
| `CritterVelocityMultiplier` | 组件（f32 @+0） | 查询闭包环境写 u32（语义 U） |
| `Velocity` | 组件（Vec3） | move_critters 查询（硬证据） |
| `Firefly` | 组件 | `goal_timer`（firefly 核心）；GPU 发送 count 0x60=96（O） |
| `Butterfly` | 组件 | GPU 发送 count 0x18=24（O） |
| `ButterflyAudio` | 组件（FMOD 事件） | `set_position` 行 144（0x140f85ba0：kind==1 时 `libfmod::EventInstance::set_3d_attributes`） |
| `Duck` | 组件（含 FMOD quack/takeoff/land） | `new` 行 208（0x140f85bb0：`event:/SFX/Ducks/{Quack,TakeOff,Land}`）；`being_petted` 行 136 |
| `DuckState` | 枚举（u8） | set_duck_velocities 只处理 ==2；语义 U |
| `Wren` | 组件（鹪鹩状态） | `nani_update_wren` 读（WP6）；本包细化 |
| `BirdSpotManager` | ECS 资源 | `gen_id` 行 317（0x140f86330：`+8 += 1`）；`create_spots` 行 322（0x140f86340：池复用/分配）；`recycle<closure_env$3>` 行 329 |
| `BirdSpots` | 组件 | `update_roof_bird_spots`/`update_decorator_bird_spots` 写 |
| `BirdSpot` | 记录 | 池元素（U 布局） |
| `BirdSpotRecycler` | 资源 | 池 Vec（create_spots 的 in_R8） |
| `Fish` | 组件 | `spawn_fish`/`despawn_fish`/`fish`（boids 驱鱼） |

### 1.1 `resources::critters::sheep`（O，serde）

`Sheep`（serde Serialize/Deserialize，`being_petted` 行 136？）、`SheepMoveState`、`SheepAudio`、`SheepAccessory`、`FloatieHolder`、`UmbrellaHolder`、
命令 `CmdCheckForSheepInsideBuilding`、`CmdUnfreezeSheep`。`_::impl$0::serialize/deserialize` = serde_json 自动派生（O 名，行为复用依赖）。

### 1.2 `resources::critters::sheep::control`（O，serde + 快照）

`SheepControlSnapshot`、`SheepSnapshot`、`capture_sheep_snapshot`（行 O；serde_json 序列化保存/回放路径）。

### 1.3 `render_loop::resources::critters`（GPU 侧 SSBO，名 O，布局 U）

`FireflySsbo`（96 实例，fireflies_send_to_gpu 经 `cpu_write<FireflySsbo>` 写，`FrozenMap::get<SsboId,Box<BufferState>>` 校验 + memcmp 0x44）、
`ButterflySsbo`（24 实例，memcmp 0x46）、`FireflyVisibilitySsbo`（`_critters_firefly_vis_query` 读）、`SheepAnimationSsbo`（sheep 动画 GPU 状态）。

## 2. 系统清单（`systems/critters` crate，函数名 O，行号/VA 见 enriched）

### shared（复用共享核心）
| 系统 | VA | 核心 |
|---|---|---|
| `critters_avoid_collisions` | 0x1423179a0 | 水/岩石/墙体/spherecast 避让；`<0.1` 命中 → `DespawnDuckCmd`；收尾清 `CritterGoal.active=0`（shared.rs 草稿） |
| `smooth_critter_velocities` | 0x142318c10 | `retention = retention_per_second.powf(dt)`；`vel = vel*retention + target.dir*(1-retention)` |
| `move_critters` | 0x142318fa0 | 查询 `(&mut Transform,&CritterLocomotion,&Velocity,Option<&CritterVelocityMultiplier>,Option<&Duck>,Option<&Sheep>,Option<&mut BirdSpots>)`；`y=max(y,0)`；`pos += vel*dt*mult`；朝向 `nlerp(rot, arc(dir,Y), 1-0.01^dt)` |

### firefly / butterfly（同构，逐类单态）
| 系统 | VA | 语义 |
|---|---|---|
| `update_firefly_goals` | 0x142316940 | `CritterControl.+4==1` 门控（O）；每实体 `goal_timer -= dt`，到期/抵达 → 3×`uniform_f32` + 球面采样选新目标（firefly.rs 核心） |
| `set_firefly_velocities_towards_goals` | 0x142317040 | 查询 `(Transform,<critter>,CritterGoal,&mut CritterVelocityTarget,&mut CritterVelocityMultiplier)`（反编译名解析为 Sheep——单态化泛型共用，I）；`dir = normalize(goal-pos)`，`y<1 → y=max(y,0.25)`，mult=1.0 |
| `fireflies_send_to_gpu` | 0x142316760 | `Vec<FireflySsbo>` from Query `(Transform,Firefly)`；断言 0x60=96；`FrozenMap::get` + 尺寸 memcmp + `cpu_write<FireflySsbo>` |
| `update_butterfly_states` | 0x141d84d40 | 蝴蝶状态机（姿态/拍翅，U 细节） |
| `update_butterfly_goals` | 0x141d86060 | 同 firefly 目标刷新 |
| `set_butterfly_velocities_towards_goals` | 0x141d867b0 | 同 firefly 速度定向 |
| `update_butterfly_audio` | 0x141d86eb0 | ButterflyAudio FMOD 位置同步 |
| `butterflies_send_to_gpu` | 0x141d87140 | `Vec<ButterflySsbo>`，断言 0x18=24，`cpu_write<ButterflySsbo>` |

### wren / wren_boids / boids / duck / fish（I 级，名 + 关键数值 O）
| 系统 | VA | 语义 |
|---|---|---|
| `update_wrens` | 0x14231e390 | `Query<(Entity,&mut Transform,&mut Wren)>` 更新鹪鹩（nani_update_wren 消费，WP6） |
| `update_roof_bird_spots` | 0x14231d6c0 | `Query<(SittableForBirds,Transform)>` + `ResMut<BirdSpotManager>` → 屋顶鸟位（roof shape 查询，O 闭包 ×3） |
| `update_decorator_bird_spots` | 0x14231c7d0 | 装饰物鸟位 |
| `sim_wren_boids` | 0x142326a80 | `ResMut<WrenBoids>` + `Local<WrenRng>` + `Query<(Transform,&mut Wren)>`；Boid flock 模拟 + `is_viable_landing_spot` + `lerp` |
| `Boid::{new,random_vel_change,apply_rules,unobstructed_dir,frame_update}` | 0x141dc2170/0x141dc3510 | boids 规则（聚集/分离/对齐，具体权重 U） |
| `BoidFlock::{push,clone_state,iter_mut,len,remove,drain}` | — | flock 容器 |
| `plan_duck_trajectories` | 0x14230ff00 | 鸭子轨迹规划（`PlanPurpose::is_for_entity`，闭包 ×8） |
| `move_ducks_along_trajectories` | 0x142312200 | `progress += dir*12.0*dt/segment_len`（0x41400000=12.0，O） |
| `update_duck_goals` | 0x142313b30 | 目标刷新：`dwell[2]` 倒数、`[1,4]` 区间、`u*0.5+0.5`（duck.rs 核心） |
| `set_duck_velocities_towards_goals` | 0x142314470 | 仅 `DuckState==2` 且 moving 时朝目标（duck.rs） |
| `update_duck_ripples` | 0x142314b70 | 水波特效（名 O，算法 U） |
| `duck_splash_particle` | — | 水花粒子 |
| `ducks_react_to_being_inside_buildings` | — | 室内鸭子反应（闭包 ×2） |
| `ui_pet_ducks` | — | UI 抚摸鸭子 |
| `fish` | 0x140f90cd0 | Boid 驱鱼（watermask 范围） |
| `spawn_fish` / `despawn_fish` | 0x140f90c10 / 0x140f90cd0 | `FishSpawnLocations::new`（0x140f909f0：25×25 网格，-52..52，625 点，每点 12B） |
| `DuckSpawnLocations::new` | 0x140f8ee90 | 9 列（x=-52..52，t=i/9）× 4 行（z=-52,-40.22,-28.9,-18.66），每点 12B |
| `spawn_ducks` | 0x140f8f210 | 门控 `_DEV_OVERRIDE_IS_DEMO_MODE==0` 且 `BackupSaveFileKind::is_corrupted()==false`（O）；按位置生成 Duck |
| `despawn_ducks` | 0x140f8f8b0 | 移除鸭子 |

## 3. 关键数值/边界（O 反编译）

- Firefly GPU 上限 = 0x60 = **96**；Butterfly = 0x18 = **24**（`from_iter` 后 `panic` 断言）。
- 鸭子目标距离平方阈值 0.16（0x3e23d70b）；新计时区间 `[1,4]`（u*3+1）；切换后半计时 `u*0.5+0.5`。
- 轨迹速度 12.0；朝向衰减底 0.01（`powf(0.01, dt)`）。
- `Duck::new` 三 FMOD 事件串：`event:/SFX/Ducks/{Quack,TakeOff,Land}`。
- `BirdSpotManager`：`+8` u32 计数器（gen_id）；create_spots 走池（BirdSpotRecycler）复用/分配。
- spawn_ducks 的 demo/损坏存档门控（`_DEV_OVERRIDE_IS_DEMO_MODE`、`BackupSaveFileKind::is_corrupted`）。

## 4. typed gap 汇总

- 各组件（Critter/Firefly/Butterfly/Duck/DuckState/Wren/Fish/BirdSpot/BirdSpots/Sheep*）精确字段布局与尺寸（除 duck/shared 已反汇编字段）。
- `CritterLocomotion` 枚举语义、`CritterVelocityMultiplier` 查询闭包环境 u32 语义。
- `BirdSpotManager`/`BirdSpotRecycler` 池布局；`BirdSpot` 字段。
- `update_butterfly_states`/`update_duck_ripples`/`sim_wren_boids` 内部算法细节（boids 权重等）。
- GPU SSBO（FireflySsbo/ButterflySsbo/FireflyVisibilitySsbo/SheepAnimationSsbo）实例布局。
- sheep 各系统（sheep_pet_animation/sheep_walk_animation/sheep_umbrella/sheep_floatie/sheep_unstuck/sheep_react_to_being_inside_buildings/sheep_terrain/ui_pet_sheep/sheep_input_record_replay）算法细节——本包登记名与生产者/消费者，不逐体重建。
