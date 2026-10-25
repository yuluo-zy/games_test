# critters 审查核对表（reviewer-B tg_vB1 预置基线）

> 状态：**已审查**（2026-10-16，restorer-B 交付后独立审查；主 workspace 因 country-core
> session/save 域未提交修改 3 错误暂无法复验编译，restorer-B 临时 workspace 通过）。
> 判定分布：**P≈23 / F=6 / Q≈22**（51 业务；F1 已按中央裁决撤回，核对列逐行见下）。
> 机器码基准：critters-behavior-analysis.md / critters-extra-analysis.md + reviewer RNE 抽验。



> 本表业务函数清单与证据锚点来自 `SOURCE_MAP-systems.json`（skeleton PDB 元数据）+ `evidence/multi-baseline/*` 反汇编级分析 + country-core-deep 章节 + reviewer tg_vB1 独立 RNE 抽验。
> 交付后 reviewer 逐项核对：签名/参数面 vs 机器码；接口契约 vs core 类型；偷懒（空壳/假数据/直接复制 TODO 骨架）；缺失（本表函数未实现）。
> 核对列：P=通过 / F=失败 / Q=存疑待深挖。

| 函数 | VA | 核对 | 证据锚点 / 已知风险 |
| **boids.rs** | | | |
| `system_critters::boids::BoidFlock::remove<system_critters::fish::fish::closure_env$1>` | 0x141dc2340 | | BoidFlock::remove 2 实例(swap_remove+despawn 语义, wren/fish) / frame_update 2 实例(call-site 常量: fish sep_r=0.8 align=0.0584 min/max=0.9|3.0/4.0|8.0; wren sep_r=1.3 align=0.1169 7/14 — RNE 0x142328340/0x140f91600 已抽验) / random_vel_change(fastrand+uniform_sample_cone) |
| `system_critters::boids::BoidFlock::remove<system_critters::wren_boids::sim_wren_boids::closure_env$6>` | 0x141dc2540 | |  |
| `system_critters::boids::Boid::frame_update<system_critters::fish::fish::closure_env$0>` | 0x141dc2740 | |  |
| `system_critters::boids::Boid::frame_update<system_critters::wren_boids::sim_wren_boids::closure_env$3>` | 0x141dc3510 | |  |
| `system_critters::boids::Boid::random_vel_change` | 0x141dc2170 | |  |
| **butterfly.rs** | | | |
| `system_critters::butterfly::update_butterfly_states` | 0x141d84d40 | | 常量已抽验: 0x142f67d14=0.8529, 0x142f67d24=-0.2835, 0x142f67d30=1.77; 跳表 DAT_142f68008/0x142f68018 handler 语义未精读(证据 unknown) |
| `system_critters::butterfly::update_butterfly_goals` | 0x141d86060 | |  |
| `system_critters::butterfly::set_butterfly_velocities_towards_goals` | 0x141d867b0 | |  |
| `system_critters::butterfly::update_butterfly_audio` | 0x141d86eb0 | |  |
| `system_critters::butterfly::butterflies_send_to_gpu` | 0x141d87140 | |  |
| **duck.rs** | | | |
| `system_critters::duck::move_ducks_along_trajectories` | 0x142312200 | | 五段链 0x14230ff00->0x142312200->0x142313b30->0x142314470->0x142314b70; DuckState: 1=轨迹正向 2=自由游(set_vel 只处理==2) 3=轨迹反向末端 despawn; Duck+0xe0/0xe1/0xe4/0xe8/0xfc; 轨迹 Vec@+0x98/+0xa0 进度+0xec |
| `system_critters::duck::plan_duck_trajectories` | 0x14230ff00 | |  |
| `system_critters::duck::update_duck_goals` | 0x142313b30 | |  |
| `system_critters::duck::set_duck_velocities_towards_goals` | 0x142314470 | |  |
| `system_critters::duck::update_duck_ripples` | 0x142314b70 | |  |
| **duck_splash_particle.rs** | | | |
| `system_critters::duck_splash_particle::duck_splash_particle` | 0x142322650 | | 事件消费链: DespawnDuckCmd 发/收双方 — 消费方待核(证据 unknown) |
| **ducks_react_to_being_inside_buildings.rs** | | | |
| `system_critters::ducks_react_to_being_inside_buildings::ducks_react_to_being_inside_buildings` | 0x14231b140 | | 无既有重建, fresh 反编译 |
| **firefly.rs** | | | |
| `system_critters::firefly::fireflies_send_to_gpu` | 0x142316760 | | 惯性 bias: RNE 0x142ccfc58 打包 0x3f000000/0xbf000000 = 有目标+0.5/无目标-0.5 — 证据文档"校正1"(-1.0)是错的, reconstruction 草稿(-0.5)正确; set_vel 海拔下限 pos.y<=1.0 -> dir.y=max(dir.y,0.25) |
| `system_critters::firefly::update_firefly_goals` | 0x142316940 | |  |
| `system_critters::firefly::set_firefly_velocities_towards_goals` | 0x142317040 | |  |
| **fish.rs** | | | |
| `system_critters::fish::FishSpawnLocations::new` | 0x140f909f0 | | 出生 200 上限/每帧 20/625 网格[-52,52]; 陆面反弹 pos-=vel*dt, vel=-vel*min_speed; FishSpawnLocations::new 0x140f909f0; fish 0x140f90cd0 |
| `system_critters::fish::despawn_fish` | 0x140f90c10 | |  |
| `system_critters::fish::fish` | 0x140f90cd0 | |  |
| **lib.rs** | | | |
| `system_critters::duck::move_ducks_along_trajectories` | 0x142312200 | | impl$0::build 0x14067d440 插件注册(系统链顺序!) |
| `system_critters::impl$0::build` | 0x14067d440 | |  |
| **shared.rs** | | | |
| `system_critters::shared::critters_avoid_collisions` | 0x1423179a0 | | critters_avoid_collisions 0x1423179a0(legacy_spherecast 10槽序{0,1,3,7,6,4,11,2,12,13}, 球距<1.0保持/<0.1命中发DespawnDuckCmd) / smooth 0x142318c10(retention=powf(rps,dt)) / move 0x142318fa0(查询签名 7 列, y=max(y,0), rot=powf(0.01,dt) 混合, legacy_spherecast) |
| `system_critters::shared::smooth_critter_velocities` | 0x142318c10 | |  |
| `system_critters::shared::move_critters` | 0x142318fa0 | |  |
| **sheep/mod.rs** | | | |
| `system_critters::sheep::impl$0::build` | 0x14067d810 | | impl$0::build 0x14067d810 |
| **sheep/sheep_behavior.rs** | | | |
| `system_critters::sheep::sheep_behavior::update_sheep_goals` | 0x142315050 | | update_sheep_goals 0x142315050(Sheep+8==1 跟随: MainCamera mode==3 eye+forward*2 否则光标; +0x30 字节共享刷新; dist2>=0.16) / set_sheep_velocities 0x142315e40 / update_sheep_audio 0x142316490 未精读 |
| `system_critters::sheep::sheep_behavior::set_sheep_velocities_towards_goals` | 0x142315e40 | |  |
| `system_critters::sheep::sheep_behavior::update_sheep_audio` | 0x142316490 | |  |
| **sheep/sheep_floatie.rs** | | | |
| `system_critters::sheep::sheep_floatie::sheep_floaties` | 0x142320cf0 | | 0x142320cf0 未精读 |
| **sheep/sheep_input_record_replay.rs** | | | |
| `system_critters::sheep::sheep_input_record_replay::sheep_input_record_replay` | 0x14231b720 | | 0x14231b720 未精读 |
| **sheep/sheep_pet_animation.rs** | | | |
| `system_critters::sheep::sheep_pet_animation::sheep_pet_animation` | 0x14231ac20 | | 0x14231ac20; 对照 reconstruction/sheep_pet*.rs(C 基线 0x14293a350 五相动画, +0x58 amplitude>0.01=being_petted 已证实) |
| **sheep/sheep_react_to_being_inside_buildings.rs** | | | |
| `system_critters::sheep::sheep_react_to_being_inside_buildings::react_to_being_inside_buildings` | 0x14231f610 | | 0x14231f610/0x14231ff70/0x142320480 未精读 |
| `system_critters::sheep::sheep_react_to_being_inside_buildings::impl$0::register_required_components` | 0x142320480 | |  |
| `system_critters::sheep::sheep_react_to_being_inside_buildings::sheep_reaction_emoji` | 0x14231ff70 | |  |
| **sheep/sheep_terrain.rs** | | | |
| `system_critters::sheep::sheep_terrain::sheep_walk_on_terrain` | 0x14231bd80 | | 0x14231bd80: state 2=吸附/4=过渡(lerp+水采样<=0->2)/6=浮空(dy>-0.05->2)/default 行走(+4.0 偏移, clamp[-2,2]/[-5,5], 增益10x3, smooth=1-exp(-3dt)); 常量已抽验 |
| **sheep/sheep_umbrella.rs** | | | |
| `system_critters::sheep::sheep_umbrella::sheep_umbrellas` | 0x142321980 | | 0x142321980 未精读 |
| **sheep/sheep_unstuck.rs** | | | |
| `system_critters::sheep::sheep_unstuck::sheep_unstuck` | 0x1423246b0 | | 0x1423246b0 未精读(消费 legacy_spherecast) |
| **sheep/sheep_walk_animation.rs** | | | |
| `system_critters::sheep::sheep_walk_animation::sheep_walk_animation` | 0x14231a630 | | 0x14231a630 未精读 |
| **sheep/ui_pet_sheep.rs** | | | |
| `system_critters::sheep::ui_pet_sheep::ui_pet_sheep` | 0x142324f80 | | 0x142324f80 未精读 |
| **spawn_ducks.rs** | | | |
| `system_critters::spawn_ducks::despawn_ducks` | 0x140f8f8b0 | | DuckSpawnLocations::new 0x140f8ee90(10x10=100 点, x in [-52,52], z 10档) / spawn 0x140f8f210(门控非demo+非corrupted, 活鸭<15 3x速率, timer>=30) / despawn 0x140f8f8b0(DuckState==2 深/浅水越界->离场计划) |
| `system_critters::spawn_ducks::DuckSpawnLocations::new` | 0x140f8ee90 | |  |
| `system_critters::spawn_ducks::spawn_ducks` | 0x140f8f210 | |  |
| **ui_pet_ducks.rs** | | | |
| `system_critters::ui_pet_ducks::ui_pet_ducks` | 0x142322e30 | | 0x142322e30 未精读 |
| **wren.rs** | | | |
| `system_critters::wren::update_decorator_bird_spots` | 0x14231c7d0 | | update_decorator_bird_spots 0x14231c7d0 / update_roof_bird_spots 0x14231d6c0(BirdSpotManager::create_spots/recycle 消费) / update_wrens 0x14231e390 |
| `system_critters::wren::update_roof_bird_spots` | 0x14231d6c0 | |  |
| `system_critters::wren::update_wrens` | 0x14231e390 | |  |
| **wren_boids.rs** | | | |
| `system_critters::wren_boids::sim_wren_boids` | 0x142326a80 | | sim_wren_boids 0x142326a80: 80 上限, x=u*130-65 y=u*9+1, 摇树 rng*5+1, 半径 91.9/高 30 保留, RAY_DIRS 32 方向 closure_3; 调用点常量已抽验 |


## 审查判定明细（2026-10-16，tg_vB1）

### F 项（与机器码矛盾/未实现，需 restorer-B 修复）

1. **F1 已撤回（中央裁决 2026-10-16）**：being_petted 常量复核 = **0.01**
   （0x14089d4cc + 0x020c31a8 = **0x142960674** = 0x3c23d70a）。
   restorer-B `src/components.rs` 的 0x3c23d70a **正确，无需修复**；
   review-audit/章节正确。**纠错指向 core**：`resources/critters/sheep.rs` 的
   `f32::from_bits(0x292f)`（注释"常量 VA = 0x142930f74"）系 rip 解析算术错误——
   0x142930f74 的字节 2f290000 属相邻字符串区；core 应改 0x3c23d70a（转 integration owner）。
2. **F2 boids frame_update 缺加速度增量 clamp**（`src/boids.rs`）：机器码（Boid_frame_update_fish
   decompiled.c）`fVar22 = 1.0; 水>=0.5 && 有目标 → 1.5` 后
   `clamp(delta, -max, max/2)`（`-1.0*max` / `max*0.5` 双边界 + clamp::do_panic 断言）。
   reconstruction 草稿注释承认未实现，restorer 照抄残缺。fish/wren 两实例同缺。
3. **F3 sim_wren_boids 大缺口**（`src/wren_boids.rs`）：closure_3 可行点判定（RAY_DIRS
   32 方向 raycast，0x141dc42f0）用纯几何 `is_viable_landing_spot` 替代——无 raycast；
   FreeSpot 收集（L700）/ BirdHaven 树查询（L745）/ 树摇出生（L1231，"Tree has been
   shaken" 日志）/ 音频全缺；反射转向为自造随机水平方向（机器码 from_rotation_arc 转
   RAY_DIRS 方向）。
4. **F4 wren.rs 三函数为语义壳**：update_decorator_bird_spots / update_roof_bird_spots
   （机器码 Rectangle2d::get_edge_pts + roof_noise::apply_roof_noise + Roof::height +
   calc_roof_tip_center_ls + random_splits + BirdSpotManager::recycle，restorer 用宿主
   RoofBirdSpotHost 组件替换且 recycle 未接线）/ update_wrens（hop 状态机/阈值全部自造，
   PlacementDataSsbo::to_affine3a/音频/DebugLine 缺失）。
5. **F5 fish 实体转向伪实现**（`src/fish.rs`）：`lerp(target.vel)` 的 target 取
   `all[self_idx]` = 自身 → 恒等空操作。机器码 L786-812 转向目标未钉（re-analyst 需补挖）。
   另 min/max 档位选择（`BackupSaveFileKind::is_corrupted()` 谓词）未接，恒 (0.9,4.0)。
6. **F6 sheep_floatie / sheep_umbrella 生成空壳**：机器码证据明确 `Commands::spawn_empty` +
   `AudioCore::instantiate_audio`（skeleton 依赖注解），restorer 只推进现有实体，
   spawn 部分 `let _ = ...; let _ = &mut commands;` 空置。
7. **F7 duck_splash_particle 新请求不 spawn**：`for event in events.read() { let _ = event; }`
   ——机器码 spawn_empty + AssetMeshLibrary/AssetShaderLibrary 取句柄（"duck_shader"）。

### Q 项（存疑/宿主依赖/需 re-analyst 补挖；不影响交付先行）

- Q1 DuckState 0：机器码（move_ducks decompiled）字节 0 走正向（dir=+1，
  progress>0.25 转 1）；`from_byte(0)->FreeSwim` 与 `move_ducks` 的 `to_byte()==2->skip、
  0->continue` 分支不一致（0 被跳过）。
- Q2 move_ducks 末端写缺失：机器码到末端写 `Duck+0xe0=1` + `+0xe4=CONCAT44(2.0f,4.0f)`
  （restorer 只置 moving=true）；且 despawn 分支缺 `_DEV_OVERRIDE_IS_DEMO_MODE` 门控
  （发行默认不直接 despawn，走事件路径）。
- Q3 update_duck_goals 新目标时 `Duck+0xe8=0.1` 写入缺失（dwell[1]=0.1，语义 U）。
- Q4 plan_duck spawn 组件集缺 DrawableMeshBundle/ObjectGrassPush（机器码 tuple 含）；
  轨迹 lerp/raycast 受阻分支为壳（raycast_hit 恒 None，与 core 壳一致）。
- Q5 RaycastWorldHandle 壳：`slot_query` 恒 None → legacy_spherecast 永不命中 → 避让/
  DespawnDuckCmd 实际不触发。与 core RaycastWorld native-physics 未落地一致（项目现状），
  但**core 侧尚缺 legacy_spherecast（10 槽序）API**——装配层需按 LEGACY_SPHERECAST_SLOT_ORDER
  实现真实槽查询；critters 侧球检未按 locomotion 位测门控（机器码 {0,1,5,6}=纯墙避让无
  spherecast）。
- Q6 update_firefly_goals 查询含 &Sheep 未在证据钉死；CritterControlState.enabled 默认
  false → firefly 目标链恒不跑（需宿主初始化 enabled=1，机器码 `*res+4==1`）。
- Q7 butterfly 目标偏置的 goal-active 维：机器码 uVar33 按 goal active 再选
  -0.2835/0.8529（update_butterfly_goals decompiled），restorer 只实现 state 维。
- Q8 sheep_behavior：共享刷新新计时用固定 [1.0,1.0]（机器码 2xuniform_f32 产出）；
  `fVar13` 阈值 1.0 自造；SheepFollowTarget 组件替代机器码 Res<MainCamera> mode==3 判定。
- Q9 sheep_unstuck 阈值（0.5/8 次/±0.5 偏移/h+4）全部自造（证据仅依赖注解）。
- Q10 ui_pet_* 3.0 距离阈值自造（机器码 UiWidgetContext::intersecting/is_active 交互；
  toolbar 批次 3 后核对）；pet_animation 五相常量源自 C 版 0x14293a350，B 侧 0x14231ac20
  未独立核验。
- Q11 采样工具本地复刻：uniform_sample_disk/circle/sphere/golden_spiral 应为 utils crate
  （证据 VA 0x140c90700 族属 utils）；x0y/x0z 与 `utils::swizzlers` 重复定义（依赖复用违规，
  应改 import）。fish 邻居 self 匹配用距离 <1e-4 hack；wren spawn 帧预算 2 自造；
  WATER_SAMPLE_OFFSETS/DUCK_WATER_SAMPLES 偏移集为推断；despawn_ducks 离场轨迹自造 +
  立即 despawn（机器码推计划由 plan_duck 执行）。
- Q12 build 命名：`configure(world, schedule)` 替代 impl$0::build 0x14067d440 / sheep
  build 0x14067d810（项目装配约定内，登记；系统注册链序与 machine build 的 chain/in_set
  未逐项对机器码）。
- Q13 组件重定义：systems 重定义 Duck/Sheep/Butterfly/Firefly/Fish/Wren/BirdSpots/Velocity/
  CritterVelocityTarget/Multiplier/Locomotion，其中 **core 已有 `Sheep` 组件**
  （resources/critters/sheep.rs，TypeId 不同）→ ECS 装配时同名牌冲突风险，integration
  owner 裁决（建议 systems 复用 core Sheep 或 core 扩展）。

### 合并登记（等价合并，非缺失）

- BoidFlock::remove x2（fish/wren 0x141dc2340/0x141dc2540）→ `flock_remove` 单泛型；
- Boid::frame_update x2（0x141dc2740/0x141dc3510）→ `frame_update` 单泛型（调用点常量
  已逐项 RNE 抽验一致，除 F2 增量 clamp）。


## 复验轮判定（2026-10-16 第二轮，restorer-B 修复后）

> 主 workspace 编译：**三 crate 全部通过**（session/save 域错误已由组 C 闭合）。
> 判定：F2-F7 中 **F2/F6/F7 闭合（P）**；F3/F4/F5 结构闭合（P/Q，细节 Q 保留）。
> 剩余 F：无。剩余 Q（转 re-analyst/宿主装配层）：见下。

- **F2 boids 增量 clamp：P**。`frame_update` 新增 `accel_max` 参数，
  `clamp(vel-old, -max, max/2)` + `vel=old+delta`，与机器码（fVar22=1.0/1.5、
  双边界 clamp::do_panic）一致；fish 调用点 `水面 && 有目标(最近他鱼) → 1.5` ✓，
  wren 调用点 `有 FreeSpot 目标 → 1.5` ✓（wren 的 iVar9 判据为推断 Q）。
- **F3 sim_wren_boids：结构闭合（P/Q）**。RAY_DIRS OnceLock 32 方向 ✓、FreeSpot
  收集 ✓、BirdHaven 查询（Without<TreeMarkedForDeletion>）✓、树摇 burst(rng*5+1)+
  log ✓、不可行反射 `from_rotation_arc * (vel*-5.0)`（-5.0 常数 ✓）、出生音频
  instantiate ✓。Q：RAY_DIRS 精确方向值 I（DAT_14363aad0 运行时惰性初始化，文件
  零填充无法直读）；树摇条件（机器码 fVar91/fVar76>10.0 且竖直计时<=0）简化为单
  计时>10.0；event 路径 "event:/SFX/Wren/Chirp" 自造；raycast 经壳（native 装配层
  落地后行为生效）。
- **F4 wren.rs 池语义：结构闭合（P/Q）**。BirdSpotManager.gen_id(+8 返旧值)/
  create_spots(池复用)/recycle/recycle_all 真实实现 ✓；update_roof_bird_spots 接
  PublicWalls::roof_entity + Roof::height + calc_roof_tip_center_ls +
  apply_roof_noise + gen_id ✓，屋顶移除 → recycle_all + despawn ✓。Q：
  Rectangle2d 几何自造（ZERO+2.0 半宽，机器码取真实屋顶矩形）；装饰物鸟点位置
  entity_anchor 返回 ZERO（假值）；update_wrens hop 数字（0.5+r*2 等）仍自造；
  鸟点实体 spawn_empty 仍为宿主流。
- **F5 fish 转向：P/Q**。恒等 no-op 已消除（目标 = 最近他鱼 2D）；Q：目标来源为
  推断（机器码 L786-812 目标字段 U）；is_corrupted 档位谓词仍未接（恒 0.9/4.0）。
- **F6 floatie/umbrella owner 绑定：P**。深水羊 spawn（owner=Some(sheep)）+
  instantiate_audio + start_if_not_playing + 出水/雨停 despawn ✓。Q：event 路径
  "event:/SFX/Sheep/Float"/"event:/SFX/Sheep/Umbrella" 自造。
- **F7 duck_splash：P**。新请求 spawn SplashParticle + duck_shader 句柄缝 ✓。
