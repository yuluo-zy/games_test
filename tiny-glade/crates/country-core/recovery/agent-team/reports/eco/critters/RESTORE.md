# system-critters 还原报告（eco/critters）

- restorer：`restorer-eco-critters`（本报告）。
- 工作目录：`crates/systems/critters`（package `system-critters`）。
- 还原标准：fresh 反编译 → 重建原函数完整控制流 → Rust body 逐语句判等（2026-10-16 硬标准）。
- 证据：fresh Ghidra 反编译 + `disassemble_function`（shared worker）存于
  `reverse-engineering/tiny-glade/evidence/restorer-eco-critters/decomp/*.c` 与 `*.fresh.asm.txt`。
  反编译全部覆盖本模块 42 个业务函数（batch1–9）。
- 分类：`判等` = Rust 与 fresh 反编译控制流/常量逐语句一致；`重建` = 本批次修改了函数体；
  `glue-legitimate` = derive/fmt/serde/bevy 注册胶水（不重建）；`未实现` = 分歧点未消，列证据。

## 核心函数（本 restorer 亲做）

| 函数 | 地址 | 分类 | 证据 / 分歧点 |
|---|---|---|---|
| Boid::random_vel_change | 0x141dc2170 | 重建 | decomp/141dc2170.c + asm：timer-=dt；`<=0` 时 3×fastrand f32（u/v→uniform_sample_cone(cone=0x3f7e76c9)，t→lerp(0.1,0.3,t)）；`dir_smoothed = tcl(dir_smoothed, dir, 4.0, dt)`；`vel = from_rotation_arc(Vec3::Y, dir_smoothed) * vel`。旧实现 timer=`2*rng+0.5`、from_rotation_arc(vel,target) 均为近似，已改。 |
| Boid::frame_update (fish/wren) | 0x141dc2740/0x141dc3510 | 重建 | decomp/*.c + fresh asm：y 水平化（vel.y>0→*exp(-0.4dt)）；邻居双 heading 门（`vel·nvel>=0` + `d·nvel/(|d||nvel|)>=-0.5`）；分离 smoothstep(1-d/sr)(3-2t)t² ×1.7；对齐 ×align_w；凝聚 0.005（非旧 0.0056）；goal→`tcl(vel, goal-pos, max(1,1/max(|d|,1e-5)²)*100, dt)`；前瞻 `pos+lookahead*normalize(vel)`（lookahead=min(0.9|d|,4)/4）；不可行→RAY_DIRS 32 向（from_rotation_arc(Vec3::Y,dir) 旋转）`viable(pos+4r)` → `vel=norm_r*|vel|*6-vel*5`，accel=3；accel 增量 `|d|>max` 缩放 max，再仅 y clamp[-max/2,max/2]；速度 [bound,max]（goal 时 bound=lerp(0.2,1,smoothstep(min(1,|d|*0.5)))*min_speed）；NaN→0；`viable(pos)`? pos+=vel*dt : pos-=vel*dt, vel=-vel*min_speed。FIX⑦：邻居首计数 loop-versioning（count==0 时仅 -0.5 门）已补。分歧点：RAY_DIRS 32 方向字面量未提取（Once 惰性初始化，golden-angle 占位）。 |
| BoidFlock::remove | 0x141dc2340/0x141dc2540 | 重建 | 索引 Vec<u32> 倒序：boid 0x18 槽 + aux 0x38 槽 swap_remove；Entities::contains 存活则 despawn。FIX⑥：boid 越界→break（停止处理）、aux 越界→panic（swap_remove assert，Rust Vec::swap_remove 天然 panic）。 |
| apply_fish_motion | — | 判等 | 兼容辅助：水面 pos+=vel*dt；陆面 pos-=vel*dt, vel=-vel*min_speed（与 frame_update 内移动分支一致）。 |
| smooth_critter_velocities | 0x142318c10 | 判等（已知 NaN 差分项） | decomp/142318c10.c：`retention=powf(retention_per_second,dt)`；`vel=vel*ret+dir*(1-ret)` 逐轴，乘法顺序与反编译一致。reviewer 独立复述通过。已知差分失败项（case 1578）：`powf(0,dt<0)=+inf` 时 `inf+(-inf)` 原码 QNaN `ffc00000` vs Rust `7fc00000`——编译器级 QNaN 规范化差异，非逻辑分歧（保留待复现）。另 multiplier 每帧写查询环境值（bevy-free 无法复现）。 |
| move_critters | 0x142318fa0 | 重建（部分分歧） | decomp/142318fa0.c：y=max(y,0)；multiplier 门（None→1.0 完整路径；Some(0.0)→仅墙体路径）；位移=vel*dt*mult；岩石(loc==2)/水面单采样阻塞；locomotion 非位集（loc∉{0,1,5,6}）→spherecast（FIX①：原 wall_hash_only 判反，已改 `!wall_hash_only`）；FIX⑨：blocked 时连朝向/BirdSpots 一并跳过；朝向 from_rotation_arc(dir_x0z,up)+符号翻转 nlerp；BirdSpots spot=trans+rot*(−0.495,1.567,0)。分歧点：query 组件标志→Duck/Sheep Option 映射 U；multiplier==0 墙体路径的位移来源；spherecast 命中码语义。 |
| critters_avoid_collisions | 0x1423179a0 | 重建（部分分歧） | decomp/1423179a0.c：仅 goal.active==1；dir=normalize(goal-pos) x/z；岩石(loc==2 & 采样>0)→清目标；water8 8×bilinear 求和 clamp[0,1]>0→清目标（在水面抵达）；墙体路径 relate 判交→保留目标；FIX④：loc<7 且位集→直接保留目标；spherecast 命中码==2 或球距>=1.0→保留目标、近距(<1.0)→清目标、<0.4→DespawnDuckCmd（原 `hit&&dist<1.0→保留` 判反已改）。分歧点：water8 标志→Option 组件映射（以 !samples_rock_mask 近似）；墙体 relate 循环语义；命中码语义。 |
| update_firefly_goals | 0x142316940 | 判等 | fresh asm（disasm 0x142316940）：入口 `*res+4 != 1 → return`；timer-=dt；刷新 timer<=0\|\|!goal\|\|dist²<0.01；bias=0x142ccfc58 按 goal 取 +0.5/-0.5；新 timer=u*0.15+0.05；sample=uniform_sample_sphere；`biased=(vel.x*bias+s.x, s.y*0.5, vel.z*bias+s.z)`（R13=Velocity，swizzle 源确认）；proposed=pos+dir_norm*timer；半径<65 或<原半径→写 goal。现实现逐语句一致。分歧点：每萤写 CritterVelocityMultiplier=查询环境值（bevy-free 无法复现）。 |
| set_firefly_velocities_towards_goals | 0x142317040 | 判等 | dir=(goal-pos).normalize_or_zero()（无目标 0）；multiplier=1.0；pos.y<1.0 → dir.y=max(dir.y,0.25)。现实现一致。 |
| fireflies_send_to_gpu | 0x142316760 | glue-legitimate | from_iter<(Transform,Firefly)→FireflySsbo>（0x60=96 上限 panic）+ FrozenMap::get+memcmp(0x44)+cpu_write；GPU 后置，CPU 侧实例投影保持。 |

## 子 agent 交付（delegated modules）

> 以下由本会话分派的并行子 agent 恢复，本 restorer 已抽查/待抽查（trust-but-verify）。

### duck+fish（子 agent A）— 已完成
| 函数 | 地址 | 分类 | 证据 / 分歧点 |
|---|---|---|---|
| DuckSpawnLocations::new | 0x140f8ee90 | 未实现(组件组阻塞) | 真实=Vec<(Vec2,bool)>+timer@+0x18=25.0；components.rs 非可编辑，补 `duck_spawn_grid_b()` 纯函数落准确 10×10 网格（行 `(i/9)*52-(1-i/9)*52`，列十档已解码）。 |
| spawn_ducks | 0x140f8f210 | 重建 | disasm 0x140f8f32c：活鸭 timer 累加 3×（dt+2dt）；live>=15 不生；轨迹仅 k=1,2 两次 golden_spiral；分歧：sorted_by comparator U、demo/corrupted 门未接线。 |
| despawn_ducks | 0x140f8f8b0 | 重建 | state==2:`水<1e-5\|\|clamp(4栅格和)>0.99`；state<2 末点:`水<0.1\|\|clamp>0.9`（4 栅格=PathRastersResources）；分歧：离场计划=1 点+circle（现 3 点外扩）。 |
| FishSpawnLocations::new | 0x140f909f0 | 未实现(组件组阻塞) | 625 点网格与真实公式/排序不同（components.rs 非可编辑）。 |
| despawn_fish | 0x140f90c10 | 未实现 | 真实=资源重置（OnExit<AppLoadingState> ResMut<FishBoids>+<FishSpawnLocations>），crate 建模为实体清理（lib.rs ⑦ DIVERGE）。 |
| fish | 0x140f90cd0 | 重建 | 出生 vel 三档全 `f32-0.5`（原 -0.5/-1/-1 错）；pos=候选+抖动 `(f32-0.5)*0.5`；水<=0.5 移除；已适配 boids 新 API。FIX⑤：抖动轴位修正——jy=u-0.25（无 0.5 缩放）、jz=(u-0.5)*0.5（无 -0.25，原 -0.25 错位到 z）。分歧：FishBoids Component 非 Resource、光标/相机逃避未做。 |
| plan_duck_trajectories | 0x14230ff00 | 未实现 | dev 热键 A/B、raycast 两点插值(2.15/0.05)、Frenet 未重建；删了不存在的 crate::shared::raycast_hit。 |
| move_ducks_along_trajectories | 0x142312200 | 未实现(部分) | 进度式匹配；末端已加 goal{dposition,tag=1}+dwell[2,4]；分歧：轨迹状态∈{0,2} vs 现{1,3}。 |
| update_duck_goals | 0x142313b30 | 重建 | disasm 状态机：aux 门复位、dwell 低半=u*0.5+0.5、keep 门=goal.active&&dist²>=0.16；换目标=旋转+0.8scale+归一化+glade 半径门+水<=0.5+4 栅格和>=0.1。FIX②：复位条件改为二进制 `dwell[0]<=0 && aux==0`（唯一）、keep 门改为 `aux!=0 \|\| dwell[1]>0`（原复位含 dwell[0]>0 分支、keep 缺 dwell[0]>0&&dwell[1]>0 情形，均改）。分歧：aux 恒 0、rotation 精确四元数 U、within_glade dev 门控 false。 |
| set_duck_velocities_towards_goals | 0x142314470 | 判等 | goal.active&&moving→dir 归一化；frozen 清零；multiplier=1。 |
| update_duck_ripples | 0x142314b70 | glue-legitimate | retention 1-exp(-2.5dt)/1-exp(-5dt) 常量匹配；GPU SSBO 上传宿主侧。 |
| duck_splash_particle | 0x142322650 | 重建 | age+=dt/2.5、>1.0 despawn、资产名"splash_particle"；分歧：触发查询 vs 事件。 |
| ducks_react_to_being_inside_buildings | 0x14231b140 | 未实现-需重做 | FIX⑧：原标 glue-legitimate 不当（含自定义业务近似）。真实=`PublicWalls::get` 逐鸭墙组（+600 计数<=1 且 +0x280 水位<=1.5 门）+`get_wall_shape`+`max_y`+`roof_entity`+`Roof::height`+`GladeBorder::is_pos_inside`；本重建以 `walls.all()`+2D 射线法围合近似（墙组查询结构 U、is_pos_inside 未接线）。 |

### butterfly（子 agent B）— 已完成
| 函数 | 地址 | 分类 | 证据 / 分歧点 |
|---|---|---|---|
| update_butterfly_states | 0x141d84d40 | 重建（跳表 handler 未实现） | 查询改 (Transform,&Velocity,&mut Butterfly,&mut ButterflyAudio,&CritterGoal)+独立避羊；距离门 mode==3→距 eye 3.0、否则距光标 4.0(平方)；phase=max(0,phase-0.5dt)，near||sheep_near→1.0；dwell 0.3/0.2 与 -(u*1.5+0.5)。分歧点：DAT_142f68008 handler 语义、ButterflyAudio 每帧 u32 写值。 |
| update_butterfly_goals | 0x141d86060 | 重建 | d=1.76（旧 1.77 错）；select 0x142f67d10/d20 → w0=state==1?0.5:1.0、w1=原 goal.active?0.8523:-0.2841（旧按 state 错）；方向=球面采样加权+x0z(velocity) 动量；proposed=pos+dir_norm*d。 |
| set_butterfly_velocities_towards_goals | 0x141d867b0 | 重建 | 查询 (Transform,&Butterfly,&CritterGoal,&mut Target,&mut Multiplier)（旧 &Sheep 错）；FIX③：dir=normalize((dx,dy,dz))*1.76（原误用 Vec3::new(d.x,d.z,d.z) 把 dz 放 y 槽，已改）；`*Multiplier=Butterfly.phase+1.0`（旧 +=1.0 错）；动画跳表 DAT_142f68018(U)。 |
| update_butterfly_audio | 0x141d86eb0 | 判等 | 逐蝴蝶 set_position，现实现一致。 |
| butterflies_send_to_gpu | 0x141d87140 | glue-legitimate | from_iter+memcmp(0x46)+cpu_write；元素布局 U、GPU 后置。 |

### wren（子 agent C）— 已完成
| 函数 | 地址 | 分类 | 证据 / 分歧点 |
|---|---|---|---|
| sim_wren_boids | 0x142326a80 | 重建 | 出生补满 80（pos x=u*130-65/y=u*9+1/z=u*130-65，vel 各分量 u-0.5，asm L282 CMP 0x50）；保留 `sqrt(x²+y²)<=91.92388 && y<=30` 先行，出界 flock_remove；call-site 权重 1.3/0.1169/7.0/14.0；摇树 log 串 "Tree has been shaken. Spawning birds!"（读 memory 证实）+ burst=floor(rng*5)+1 + 计时=0.8+rng*5.2 + 出生 offset=normalize(dir)*dist*1.01。分歧点：出生门精确触发条件 U（用 dt!=0 补满近似）；viable 用向下探针近似；FreeSpot 认领/树关联近似。 |
| Boid::frame_update<wren> | 0x141dc3510 | 重建(glue) | 共享核心在 boids.rs（本 restorer fresh 重建）；wren 注入 call-site 权重 + viable=!closure_3（0x141dc42f0：端点 y∈[0,30]、半径 91.92388、raycast 命中=不可行）。 |
| update_decorator_bird_spots | 0x14231c7d0 | 未实现 | HashMap<DecoratorId,Entity> 生命周期 + sparse instance buffer 驱动；装饰物注册表查询 / create_spots 布局 U。 |
| update_roof_bird_spots | 0x14231d6c0 | 未实现 | random_splits 分段语义 / 停靠点分布参数 U；仅 recycle+despawn 路径已证。 |
| update_wrens | 0x14231e390 | 未实现 | 状态跳表 DAT_14304b638 handler + per-spot 0x28/0x50 状态数组接线 U；to_affine3a/DebugLine/DebugSphere/audio set_position 结构已证。 |

### sheep（子 agent D）— 已完成
| 函数 | 地址 | 分类 | 证据 / 分歧点 |
|---|---|---|---|
| update_sheep_goals | 0x142315050 | 重建 | 跟随分支 dist<0.2→timer-=3*dt；timer 随机 12,4/7,1；共享刷新 0.16、scale 表 0x14304ad18{0.8,0.5}、半径 65.0、demo 门 \|pos\|>35.0（修正旧 56.0）、25.0、三次 bilinear；新目标=pos+normalize(v)*shared_timers[1]。残留：XMM7/8 寄存器残留、flag_31 死分支、2×4B 查询列省略。 |
| set_sheep_velocities_towards_goals | 0x142315e40 | 判等 | goal.active&&flag_30==1&&!being_petted→normalize(goal-pos) 否则零；multiplier=1.0。 |
| update_sheep_audio | 0x142316490 | 判等(核心) | 逐羊 set_position；0x38 音频记录 +0x34+=dt 未建模(DIVERGE)。 |
| sheep_walk_on_terrain | 0x14231bd80 | 重建 | 修正两处旧错：水面仅 case4 采样且 <=0→GROUNDED（旧反了）；h_eff=(state&1)?cached_h:h；常量 -3/10/20/±0.3/-0.05/±2/±5/4.0/130.0 复核。case4 lerp rate 符号未取证(0.5)。 |
| sheep_unstuck | 0x1423246b0 | 重建(DIVERGE) | 0x54 掩码选中{2,4,6}、ground_h=max(h,0.5)、\|Δh/dt\|>10→state=0、state=4/6 迁移；DIVERGE：spherecast ABI、rock 逐点采样、is_corrupted 未接线。 |
| react_to_being_inside_buildings | 0x14231f610 | 未实现(部分重建) | 墙高+屋顶高+is_pos_inside 已重建；emoji 表/spawn Ui2dExtent,Ui2dImage 为 glue。 |
| sheep_reaction_emoji | 0x14231ff70 | 重建 | 状态机 0(0.2)/1(1.0)/2(0.4)，ease scale，t==1 转换/despawn；EasingFunction 变体未取证(CubicOut)。 |
| sheep_floaties | 0x142320cf0 | 重建(DIVERGE) | state==4→+2.0/否则-2.0、clamp[0,1]、音频 gate；spawn bundle glue。 |
| sheep_umbrellas | 0x142321980 | 重建(DIVERGE) | 0x1a>>(state-2 截断)&1 掩码、翻向音频；Raining 保留(lib.rs init)。 |
| sheep_walk_animation | 0x14231a630 | 重建+未实现(跳表) | speed=\|target-记录\|/dt、0.2 状态机(1↔2)；handler 跳表 0x14304b0f8 未取证。 |
| sheep_pet_animation | 0x14231ac20 | 重建+未实现(跳表) | 首羊事件匹配+分派参数(0.3,1.0,0.2)；handler 0x14304b178 未取证。 |
| sheep_input_record_replay | 0x14231b720 | 重建(DIVERGE) | 记录全羊 positions+yaws 帧→InputStream last_frame_mut；回放 from_euler 写 Transform；会话资源+0x40 身份未取证。 |
| ui_pet_sheep | 0x142324f80 | 重建(核心)+未实现(UI) | eye 距>5.0 跳过、pet 写 state=1/timer=14.0、音频 [0x34]>0.5；WidgetParams/UiState/Signifier/drag 段未实现（缺 country-core API）。 |
| sheep::Plugin::build | 0x14067d810 | 报告-only（sheep/mod.rs 已记录证据） | register_event PlaySheepPetAnimation/CmdCheckForSheepInsideBuilding/CmdUnfreezeSheep + add_systems + chain_ignore_deferred + run_if；crate lib.rs 事件名仍旧名 PlaySheepPet/SheepInputReplayCmd（待对齐）。 |

### ui/插件（子 agent E）— 已完成
| 函数 | 地址 | 分类 | 证据 / 分歧点 |
|---|---|---|---|
| ui_pet_ducks | 0x142322e30 | 重建（衰减段判等；UI 交互未实现） | 每鸭 Duck+0xf8 pet timer `clamp(f_f8±dt/0.4,0,1)`（frozen→+/非→−）；frozen&&>=1.0 释放。分歧点：per-entity u32 槽值源 U；UI gate（code 0x13/0xd/6 资源身份 U）；country_core 缺 UiWidgetContext::intersecting/SignifierEntityBuilder/slider3d::into_drag_state；mode==3 eye 距离 6.0/signifier/audio 未接线。 |
| CritterPlugin::build | 0x14067d440 | glue-legitimate（资源/事件集重建） | configure() 按 decompile 顺序注册 8 资源+2 事件；butterfly/firefly SSBO 上传独立组 in_set<GameSystemSet>。分歧：WrenBoids/FishBoids(Resource)/CritterControl 类型 crate 缺失、OnExit<AppLoadingState> 未接线、require 资源 TypeId U。 |
| sheep::Plugin::build | 0x14067d810 | 报告-only（sheep/mod.rs 已由子 agent D 记录证据） | 见 sheep 段；crate lib.rs 事件名仍旧名（待对齐）。 |

## reviewer 复核处理（FIX ①–⑨）

reviewer-eco-critters 独立复核（REVIEW.md）给出 FIX 9 + 待取证 10。本批次处理：

| # | FIX | 处理 |
|---|---|---|
| ① | move_critters spherecast 门反向 | 已改 `!locomotion.wall_hash_only()`（loc∉{0,1,5,6} 才 spherecast） |
| ② | update_duck_goals 复位/keep 条件 | 已改：复位=`dwell[0]<=0&&aux==0`；keep=`aux!=0\|\|dwell[1]>0` |
| ③ | set_butterfly_velocities dir y 轴 | 已改 `normalize((dx,dy,dz))*1.76` |
| ④ | critters_avoid_collisions spherecast 保留条件 | 已改：近距(<1.0)命中→清目标、<0.4→Despawn；远距/未命中/码2→保留；补 loc<7&&位集→直接保留门 |
| ⑤ | fish 出生抖动轴位 | 已改 jy=u-0.25、jz=(u-0.5)*0.5 |
| ⑥ | BoidFlock::remove 越界语义 | 已改：boid 越界→break、aux 越界→panic（Vec::swap_remove 天然） |
| ⑦ | frame_update 首邻居门 | 已补 loop-versioning：count==0 时仅 -0.5 门 |
| ⑧ | ducks_react 分类 | 已改判 未实现-需重做（含手写业务近似） |
| ⑨ | move_critters blocked 路径 | 已改：blocked 时位移/朝向/BirdSpots 一并跳过 |

待取证 10（REVIEW.md §10）维持登记不作通过依据：跳表 handler、RAY_DIRS 字面量、
wall relate 枚举、query 环境值写、is_within_glade_shape、CritterGoal 字节序、
sample_at_world_xz→u32 等。

## A-toolbar / B-ecstate 复验处理

### reviewer-A-toolbar（P1/P2）
| # | 项 | 处理 |
|---|---|---|
| P1 | FishSpawnLocations::new 行列转置 + 公式非位等 | 已改 `Vec3::new(x,0,z)`（x=外层 i、z=内层 j）；公式改 `t*52-(1-t)*52`（消除 7/25 档 1 ULP 差）。used 字节仍登记 DIVERGE。 |
| P2 | DuckSpawnLocations::new 行列转置 | 已改 `Vec3::new(row,0,col)`（其余行公式/timer=25.0/used 判等）。 |

### reviewer-B-ecstate
| # | 项 | 处理 |
|---|---|---|
| move-1 | 纯墙 relate 路缺失 | 已补：无 Duck 且无 Sheep → WallSpatialHash::query + PublicWalls::get + max_y_gable 跳过（loc!=3）+ Circle2d/line/relate；Intersection→阻塞、遍历尽→位移。 |
| move-2 | spherecast 半径未传 | `legacy_spherecast` 增 `radius` 参数；move_critters/critters_avoid = Sheep 0.8 / 其它 0.5；sheep_unstuck 0.8；wren raycast 0.0。 |
| move-3 | PublicWalls::get panic | 纯墙路径改 `.expect("PublicWalls::get missing")`（二进制 unwrap_failed）。 |
| avoid-1 | spherecast 半径未传 | 同上（0.8/0.5）。 |
| avoid-2 | relate 形状近似 | 二进制 Circle2d::new_rotated + line_segment + relate；crate 为 segment-vs-segment——登记近似（不阻塞）。 |
| avoid-3 | 零方向提前清目标（新） | 已移除 `dir.length²<1e-9 → 清目标`；dir=(0,0) 后继续（二进制 0x142318351-361）。 |
| avoid-4 | PublicWalls::get panic | 已改 `.expect(...)`。 |
| wren-h1 | `qword[+0x30]=2` 顺带清 timer | 已加 `spot.timer = 0.0`；+0x1c/20/24=1.0（r13 记录）与音频槽/call 登记 U。 |
| bfly-s2 | state2 次态 3 应为 1 | 已改 `if t<=0 && phase==0 { 1 } else { 2 }`。 |

## cargo check 结论

- `cargo check -p system-critters`：**通过（0 error，8 warning）**（2026-10-17 复核，country-core 修复后）。
- 期间被 country-core 并发改动阻断多次，错误归属均非本 crate：
  `meta_commander.rs`（E0425 ResMut）、`building_history_transactions.rs`（E0405 HistoryRecordingValidation）、
  `stairs_removed_supports.rs`（StairGraphEdge serde）、`ivy_grower.rs`、`load_game.rs`/`replay_stairs.rs` 等——未代改。
- 适配：country-core 组把 `CritterGoal` 改为 `{position, goal_tag}`（字节序与 B active@0,pos@4 不一致），
  且 `terrain_rock_mask::sample_at_world_xz` 改返回 `u32`——本 crate 在 shared/firefly/butterfly 按语义
  桥接 `goal_tag!=0 → Some(position)`（DIVERGE 已登记）。

## 待办 / 协调点

- RAY_DIRS 32 方向字面量：从原二进制 `RAY_DIRS Lazy` 提取（Once init 闭包运行期填充）。
- CritterGoal 字节序（country-core {position,goal_tag} vs B active@0,pos@4）与
  `terrain_rock_mask::sample_at_world_xz → u32` 为跨组类型变更，本 crate 已语义桥接，需集成 owner 定夺。
- PetSheepTarget / Raining 资源在 lib.rs 保留（近似依赖块），若 sheep 组后续移除需同步。
- lib.rs 事件名 PlaySheepPet/SheepInputReplayCmd 与二进制 PlaySheepPetAnimation 对齐待 lib.rs 同步。
