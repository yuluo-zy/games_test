# system-critters 独立复核（reviewer-eco-critters）

- 复核人：`reviewer-eco-critters`。复核对象：`restorer-eco-critters` 的 `crates/systems/critters`（package `system-critters`）。
- 方法（2026-10-16 硬标准）：对每函数 **fresh Ghidra 反编译**（shared worker，证据存
  `reverse-engineering/tiny-glade/evidence/reviewer-eco-critters/decomp/*.fresh.c`，含 5 批 50 函数），
  独立复述原控制流，再与 Rust body 逐语句对照。关键争议点用 radare2 对 B patched EXE
  （sha256 cf048e27…6323）定点反汇编/read_memory 二次核验。
- 判定：`判等` = fresh 反编译下原控制流/常量与 Rust 逐语句一致（含已登记 query 环境值写等胶水级 DIVERGE）；
  `未实现-需重做` = 存在真实逻辑分歧（附分歧点），含 restorer 漏报的 bug；
  `胶水-非业务` = SSBO 上传/插件注册等，无业务逻辑；
  `未实现` = 确认 restorer 诚实标记（组件组阻塞/跳表 handler/UI API 缺失）。

## 一、判等（独立复述 → Rust 对照通过）

| 函数 | 地址 | 独立复述要点（fresh 反编译） | 对照结论 |
|---|---|---|---|
| smooth_critter_velocities | 0x142318c10 | `dt=*Res<Time>+0x28`；`retention=powf(ret@+0xc,dt)`；`vel=vel*ret+dir*(1-ret)`（x/y 8B 打包 + z 单独，CONCAT44 顺序与反编译一致）；写 stride-4 列=查询环境值(uVar6=Query+0x14) | 判等。核心逐语句一致；multiplier 回写=查询环境值未建模（DIVERGE，restorer 已登记）。NaN 符号位差（ffc00000 vs 7fc00000）为公式外的编译器级 QNaN 规范化，非逻辑分歧——成立 |
| update_firefly_goals | 0x142316940 | 入口 `*(*R8+4)!=1→return`；timer-=dt；刷新 `timer<=0\|\|!goal.active\|\|dist²<0.01`；bias=0x142ccfc58 按 goal 取 +0.5/-0.5（disasm 核验 `MOVSS XMM15,[RAX+RCX*4]`，RCX=goal==0）；timer=u*0.15+0.05；sample_sphere→`x0z(vel)`（0x140c90030，RCX=sample 槽/RDX=vel 列，R13 stride-0xc 列=Velocity）；biased=(vel.x*bias+s.x, s.y*0.5, vel.z*bias+s.z)（disasm 核验：MULPS 用 post-call vel.x/z，ADDPS 用 pre-call XMM9=(s.x,s.y*0.5)、ADDSS 用 XMM6=s.z）；proposed=pos+dir*timer；`sqrt(z²+x²)<65\|\|<old` 接受 | **判等**。restorer 的 vel-bias 公式经 disasm 证实正确（Ghidra fresh 反编译因 pre/post-call 值合并易误读，disasm 为权威）；query 环境值每萤写未建模（DIVERGE 已登记） |
| set_firefly_velocities_towards_goals | 0x142317040 | 查询含 `&Sheep`（init 明确）；dir=normalize((dx,dy,dz))（fVar15=dx/fVar17=dy(高字)/fVar14=dz，非 (dx,dz,dz)）；multiplier=1.0（0x3f800000）；`trans.y<1.0`→dir.y=max(0.25)；冻结/零方向 | 判等。Rust `d.normalize_or_zero()`=(dx,dy,dz) 与反编译一致；每萤 stride-4 查询环境写未建模（DIVERGE 已登记） |
| set_duck_velocities_towards_goals | 0x142314470 | 查询 6 列确认；`DuckState byte!=2` 跳过（do-while）；`goal.active&&Duck+0xe0==1`→dir=normalize((dx,dy,dz)) 否则零；multiplier=1.0；`Duck+0xfc==1`→VelocityTarget 清零 | 判等。frozen 门、DuckState 过滤、dir/zero 分支逐语句一致；查询环境写未建模（DIVERGE） |
| update_butterfly_audio | 0x141d86eb0 | 双分支遍历逐蝴蝶 `ButterflyAudio::set_position()` | 判等。无业务分歧 |
| set_sheep_velocities_towards_goals | 0x142315e40 | 查询 5 列；`goal.active&&Sheep+0x30==1&&!being_petted`→dir=normalize((dx,dy,dz)) 否则零；multiplier=1.0 | 判等。being_petted 门、flag_30 门、dir/zero 逐语句一致；查询环境写未建模（DIVERGE） |
| update_sheep_audio | 0x142316490 | 逐羊 `set_position`；另 `[0x38 音频记录]+0x34 += dt` + stride-4 查询环境写 | 判等(核心)。set_position 一致；+0x34+=dt 与查询环境写未建模（DIVERGE 已登记，音频内部计时） |
| sheep_walk_on_terrain | 0x14231bd80 | `smooth=1-exp(-3dt)`；switch(state)：2→pos.y=h；3→panic；4→tcl(pos.y,h,rate,dt)+bilinear<=0→state=2；6→dy=h-pos.y clamp[-0.3,0.3] v+=(c*20-v)*smooth, dy>-0.05→state=2,v=0；default→h_eff=(state&1)?cached_h:h, dy=(h_eff+4)-pos.y, h<=pos.y→clamp[-2,2] v+=(c*10-v)*smooth, 否则 pos.y=h clamp[-5,5] v+=(c*10-v)*smooth*3 | 判等。五分支逐语句与常量一致（restorer 修正的"水面仅 case4 采样、<=0→state=2"正确）；case4 lerp rate 符号 U（已登记，按 0.5） |
| random_vel_change | 0x141dc2170 | `timer@+0x30-=dt`；`<=0`→2×f32+uniform_sample_cone 写 dir@+0x24/0x2c、f32+lerp 写 timer；`time_continuous_lerp_vec3` 写 dir_smoothed@+0x18/0x20；`from_rotation_arc` 旋转 vel@+0xc | 判等（控制流）。cone=0x3f7e76c9、lerp(0.1,0.3)、rate=4.0 为调用点常量（restorer 证据 + 常量地址已登记） |
| update_butterfly_goals | 0x141d86060 | timer@+8-=dt；刷新 `timer<=0\|\|!active\|\|dist²<0.01`；timer=u*0.5+0.1；sample_sphere→x0z；`w0=state==1?0.5:1.0`（0x3f000000/0x3f800000 select）、`w1=prev_active?0x3f5a2e8c:0xbe91745d`；dir=(s.x*w0+vel.x*w1, s.y*w0*0.5, s.z*w0+vel.z*w1)（x0z(vel) 动量，同 firefly 模式；Ghidra 合并 pre/post-call 值易误读，disasm 为权威）；proposed=pos+dir*(timer*1.76)；y clamp<=8；`sqrt(z²+x²)<65\|\|<old` 接受 | 判等（核心）。权重/动量/1.76/y clamp/半径门与 Rust 逐语句一致；per-entity 4B 查询环境写未建模（DIVERGE 已登记） |
| apply_fish_motion | —（helper） | 与 frame_update 内移动分支（viable 水面 pos+=vel*dt；否则 pos-=vel*dt, vel=-vel*min_speed）同语义 | 判等（兼容辅助，非二进制函数本身） |

## 二、未实现-需重做（真实逻辑分歧；含 restorer 漏报）

| 函数 | 地址 | 分歧点（fresh 反编译为权威） | restorer 是否自报 |
|---|---|---|---|
| **move_critters** | 0x142318fa0 | ① **spherecast 门反向**：二进制 `(6<loc)\|\|(99U>>(loc&0x1f)&1)==0` 才进 spherecast 块（即 loc∉{0,1,5,6} 时 spherecast）；Rust `if !blocked && locomotion.wall_hash_only()`（loc∈{0,1,5,6} 时 spherecast）——**恰好相反**。loc∈mask 且有 Duck/Sheep 时二进制直接位移（无任何避让），Rust 却做 spherecast。② 无 Duck/Sheep 实体走 WallSpatialHash 墙路径（LAB_142319d00，relate 判交后可位移）；Rust 以 mult==0→blocked=true 简化，墙路径缺失且位移来源不对。③ blocked 跳转 LAB_142319506 跳过位移/朝向/BirdSpots 三件套；Rust 仅跳位移，朝向/BirdSpots 恒执行。④ 岩石路径二进制 `loc==2`：rock>0 阻塞**或**(水<=0 & water2 标志)；Rust 仅 rock>0。BirdSpots 偏移 (-0.495,1.567,0.0) 二进制确认（非纯推断） | 部分自报（组件标志 U、mult==0 墙路径、命中码语义、偏移推断）；**①反向未报** |
| **critters_avoid_collisions** | 0x1423179a0 | ① spherecast 保留目标条件不同：二进制 `hitcode==2 \|\| dist>=1.0`→保留；Rust `hit && dist<1.0`→保留（近距命中才保留，反了）。② 二进制在 dist<=0（无水）时先判 `loc<7 && 99>>(loc&31)&1!=0`→直接保留目标（loc∈mask 路径）；Rust 无此门。③ water8 标志=查询列 presence（非 `!samples_rock_mask` 近似）。④ 墙 relate 循环退出语义（relate==0 退出→清目标；全墙相交或耗尽→保留）依赖 LineRelation 枚举值，Rust `!=Disjoint→keep` 未取证。⑤ 收尾 `*goal.active=0`+stride-4 写=查询环境值 | 部分自报（water8 映射、relate 语义、命中码语义）；**①保留条件未报** |
| **set_butterfly_velocities_towards_goals** | 0x141d867b0 | ① **dir 竖直分量用错轴**：二进制 `dir=normalize((dx,dy,dz))*1.76`（fVar19=dy=goal.y-trans.y，高字；disasm 核验 `SUBPS XMM8,XMM10` 对 (x,y) 打包、`MOVSS XMM6,[R13+0xc]` 为 z）；Rust `Vec3::new(d.x,d.z,d.z)`——把 dz 放 y 槽，**y 分量错误**。② 动画跳表 DAT_142f68018 按 state 索引派发（`*(&DAT_142f68018 + state*4)`，tail-call），Rust 空实现。multiplier=phase+1.0 二进制确认（正确） | ①**未报**（restorer 声称"机器码把 transform.z 差分当竖直分量"，与 fresh 反编译/radare2 不符）；②已报 U |
| frame_update<fish/wren> | 0x141dc2740/0x141dc3510 | 邻居首计数 loop-versioning：二进制双循环，`count==0` 时只判 `d·nvel/(\|d\|\|nvel\|)>=-0.5`（跳过 `vel·nvel>=0`），计为邻居后进 count>0 循环才双门；Rust 对所有邻居恒双门——首邻居含 `vel·nvel<0` 时计数集合不同。RAY_DIRS 32 方向字面量未提取（golden-angle 占位）。其余（y 水平化 0.4、分离 smoothstep 1.7、对齐 align_w、凝聚 0.005、goal tcl rate=max(1,1/max(\|d\|,1e-5)²)*100、前瞻 viable、accel 增量 max 缩放+仅 y clamp[-max/2,max/2]、速度[bound,max]、NaN 归零、pos±=disp 移动、RAY_DIRS 4.0 搜索 6.0/-5.0 转向）逐语句一致 | 已报（loop-versioning、RAY_DIRS） |
| **update_duck_goals** | 0x142313b30 | **状态机条件错误（3/4 组合）**：二进制 moving==1 时复位条件=`dwell[0]<=0 && aux==0`（唯一）；Rust 复位条件=`(dwell[0]>0 && aux==0) \|\| (dwell[0]<=0 && dwell[1]<=0 && aux==0)`——`dwell[0]>0 && aux==0` 时二进制走 keep/pick 而 Rust 复位。keep 门：二进制=aux!=0 \|\| dwell[1]>0；Rust=`(dwell[0]<=0 && dwell[1]>0) \|\| aux!=0`——`dwell[0]>0 && dwell[1]>0 && aux==0` 时二进制 keep（dist²>=0.16 检查）而 Rust 直接换目标；`dwell[0]<=0 && dwell[1]>0 && aux==0` 时二进制复位而 Rust keep。仅 `dwell[0]<=0&&dwell[1]<=0&&aux==0` 一致。换目标块本身（scale=active?0.8:0、+0xe8=0.1、disk+旋转+x0y/x0z 归一化、glade 门（Rust 恒 false，二进制真调用 `is_within_glade_shape`）、水<=0.5、4 栅格和>=0.1）逐语句一致 | 部分自报（aux 门 U、within_glade dev 门控 false）；**复位/keep 条件错误未报** |
| BoidFlock::remove | 0x141dc2340/0x141dc2540 | 二进制索引越界 `idx>=len`→`swap_remove::assert_failed`（panic）；Rust `continue` 静默跳过。aux 越界：二进制 `idx>=aux_len`→panic；Rust `if idx<extras.len()` 条件跳过。正常索引路径（倒序、0x18/0x38 swap_remove、Entities::contains→despawn）一致；结尾二进制 dealloc 索引 Vec（Rust clear，语义等价） | 未自报（panic 路径分歧，正常态不可达，severity 低） |
| fish | 0x140f90cd0 | 出生微抖动：二进制 pos=(cand.x+(u-0.5)*0.5, **u-0.25**, cand.z+(u-0.5)*0.5)（jy=u-0.25 无 0.5 缩放，jz=(u-0.5)*0.5 无 -0.25）；Rust jy=(u-0.5)*0.5、jz=(u-0.5)*0.5-**0.25**——**-0.25 错位到 z 轴**。出生上限 200/每帧 20、水>0.5、vel 三档全 -0.5、frame_update 参数、墙推开 1.12 均确认 | 未自报（restorer 注释"z 另 -0.25"与反编译相反）；restorer 已修正 vel -0.5/-1/-1→三档 -0.5（正确） |
| ducks_react_to_being_inside_buildings | 0x14231b140 | 二进制：`PublicWalls::get` 逐鸭取墙组（count@+600<=1 且水位+0x280<=1.5 才进）+`get_wall_shape` 围合 +`max_y`+`roof_entity`+`Roof::height`+`GladeBorder::is_pos_inside`→播音。Rust：`walls.all()` 全墙遍历 + **手写 2D 射线法 point-in-polygon**（wall_contains）+ roof.height>y。**非胶水**——含自定义业务近似（墙围合判定与二进制机制不同） | **分类不当**（restorer 标 glue-legitimate；实为近似业务逻辑，应未实现-需重做） |

## 三、未实现（确认 restorer 诚实标记）

| 函数 | 地址 | 确认要点 |
|---|---|---|
| DuckSpawnLocations::new | 0x140f8ee90 | 二进制行公式 `(i/9)*52-(1-i/9)*52` 生成十档 **精确**（-52,-40.444443,-28.888889,…），components.rs 硬编码近似（-40.22/-28.56）且 timer 初值 0 vs 二进制 +0x18=25.0、used flag 语义；`duck_spawn_grid_b()` 精确未接线。未实现属实 |
| FishSpawnLocations::new | 0x140f909f0 | 公式等价，但 (x,z) 行列序转置（二进制 x=外层、z=内层；Rust x=内层、z=外层），候选顺序不同影响 `rng%len` 采样位；flag@+0 未建模。未实现属实 |
| despawn_fish | 0x140f90c10 | 二进制=资源重置（OnExit<AppLoadingState> ResMut<FishBoids>+<FishSpawnLocations> dealloc+重建）；crate 建模为实体清理。未实现属实 |
| plan_duck_trajectories | 0x14230ff00 | dev 热键 A/B、legacy_raycast 两点插值(2.15/0.05)、Frenet、完整 spawn bundle/retain 未重建。未实现属实 |
| move_ducks_along_trajectories | 0x142312200 | 分段推进/末端 goal+dwell 已建；Frenet 帧=简化线段姿态、轨迹状态 {0,2} vs {1,3} 映射 U。未实现(部分)属实 |
| update_decorator_bird_spots | 0x14231c7d0 | HashMap<DecoratorId,Entity> 生命周期 + create_spots 布局 U；Rust 空 body。未实现属实 |
| update_roof_bird_spots | 0x14231d6c0 | random_splits 分段/停靠分布 U；仅 recycle+despawn 路径。未实现属实 |
| update_wrens | 0x14231e390 | 状态跳表 DAT_14304b638 + per-spot 状态数组 U；Rust 空 body。未实现属实 |

## 四、胶水-非业务（确认）

| 函数 | 地址 | 确认要点 |
|---|---|---|
| fireflies_send_to_gpu | 0x142316760 | from_iter+memcmp(0x44)+cpu_write；GPU 后置，CPU 实例投影保持。胶水成立 |
| butterflies_send_to_gpu | 0x141d87140 | 同上（len 0x46）。胶水成立 |
| update_duck_ripples | 0x142314b70 | retention 1-exp(-2.5dt)/1-exp(-5dt) 常量匹配；SSBO 上传宿主侧。胶水成立 |
| CritterPlugin::build | 0x14067d440 | configure() 8 资源+2 事件注册顺序 + 系统装配近似；WrenBoids/FishBoids Resource/OnExit 未接线（DIVERGE 已登记）。胶水成立 |
| sheep::Plugin::build | 0x14067d810 | 报告-only；事件名 PlaySheepPet vs PlaySheepPetAnimation 待对齐。成立（注册语义确认） |

## 五、其余 重建(DIVERGE)/U（restorer 已诚实标记，抽查一致）

update_sheep_goals（主流程与 fresh 反编译一致：跟随分支 dist<0.2→timer-=3dt、timer>=0→(u1*12+4,u2*3+1)、timer<0→state=0+u*7+1、共享刷新 0.16/0.8·0.5 表/65/35/25、换目标 normalize(v)*shared_timers[1]；XMM7/XMM8 残留按沿用本羊值近似，U）；
sim_wren_boids（上限 80、x=u*130-65/y=u*9+1/z=u*130-65、保留半径 91.92388/y<=30、call-site 权重 1.3/0.1169/7/14 均确认；摇树出生近似/viable 向下探针/FreeSpot 认领近似）；
sheep_unstuck/sheep_floaties/sheep_umbrellas/sheep_walk_animation/sheep_pet_animation/sheep_input_record_replay/ui_pet_sheep/ui_pet_ducks/duck_splash_particle/spawn_ducks/despawn_ducks/react_to_being_inside_buildings/sheep_reaction_emoji — 均按报告标 重建(DIVERGE)/未实现(部分)/跳表 U，诚实无过度断言。

## 六、FIX 清单（需重做/修正，按严重度）

1. **move_critters spherecast 门反向**：`wall_hash_only()` 判反。二进制 loc∉{0,1,5,6} 才 spherecast；改 `if !blocked && !locomotion.wall_hash_only()`（shared.rs move_critters L125）。
2. **update_duck_goals 复位/keep 条件**：二进制复位=`dwell[0]<=0 && aux==0`；keep=aux!=0 \|\| dwell[1]>0。Rust 现复位条件含 `dwell[0]>0` 分支、keep 门缺 `dwell[0]>0&&dwell[1]>0` 情形（duck.rs update_duck_goal_core L359-369）。
3. **set_butterfly_velocities dir y 轴**：`Vec3::new(d.x,d.z,d.z)`→`Vec3::new(d.x,d.y,d.z)`（butterfly.rs L340）。
4. **critters_avoid_collisions spherecast 保留条件**：二进制 `hitcode==2 \|\| dist>=1.0`→保留；Rust `hit && dist<1.0`（shared.rs L258-259）。另补 loc∈mask 保留路径（L482 区）。
5. **fish 出生抖动 -0.25 轴位**：二进制 jy=u-0.25（无 0.5 缩放）、jz=(u-0.5)*0.5（无 -0.25）；Rust fish.rs L67-68 对调。
6. **BoidFlock::remove 越界语义**：二进制越界 panic（swap_remove assert），Rust continue/条件跳过——正常态等价，按 panic 语义对齐或登记（boids.rs flock_remove L320）。
7. **frame_update 首邻居门**：loop-versioning（count==0 单门）未模拟——按语义登记或补 first-pass 单门（boids.rs L170）。
8. **ducks_react_to_being_inside_buildings 分类**：glue-legitimate→未实现-需重做（墙围合判定为手写近似，非胶水）。
9. **move_critters blocked 路径**：二进制 blocked 时连朝向/BirdSpots 一并跳过；Rust 仅跳位移（shared.rs L136-161）。
10. 已登记的 U（跳表 handler、RAY_DIRS 字面量、wall relate 枚举、query 环境值写、is_within_glade_shape、CritterGoal 字节序、sample_at_world_xz→u32）维持待取证，不作通过依据。

## 七、过度断言核查

- restorer 报告整体诚实：判等 8 中 8 项独立核验通过（含 disasm 级确认的 firefly bias/vel 公式）；重建 26 中多数分歧点自报，但 **move_critters（①）、set_butterfly_velocities（①）、update_duck_goals（复位/keep）、fish（抖动轴）四处为漏报**，其中 move_critters spherecast 门反向与 update_duck_goals 状态机为实质逻辑错误，不得按"重建"通过。
- `判等` 项附带的"multiplier/查询环境值写"与"音频 +0x34+=dt"为 glue 级侧写，不影响业务判等——成立。
- smooth_critter_velocities NaN 符号位差异：公式/顺序一致，属差分测试已知失败项，非逻辑分歧——成立，但应保留待复现。
- `ducks_react_to_being_inside_buildings` 标 glue-legitimate **过度**（含手写业务近似），需改判。

## 八、结论

- 独立 fresh 反编译复核（50 函数全覆盖）+ radare2 定点核验（firefly bias、butterfly dir、move_critters 掩码、duck 状态机）。
- 判等 11（含 random_vel_change/update_butterfly_goals 核心/apply_fish_motion 辅助；8 项 restorer 判等全通过）。
- 未实现-需重做 10（move_critters、critters_avoid_collisions、set_butterfly_velocities、frame_update×2、update_duck_goals、BoidFlock::remove、fish、ducks_react_to_being_inside_buildings；含 4 处漏报 bug，FIX ①-⑨）。
- 未实现 8（restorer 诚实标记确认）。
- 胶水 5（fireflies/butterflies send_to_gpu、duck_ripples、两插件 build）。
- FIX 清单：9 项需重做 + 10 项待取证登记不通过。
- **过度断言**：move_critters/update_duck_goals 按"重建(部分分歧)"通过属过度——这两函数逻辑未还原到可判等，须按 FIX ①/② 重做后再验收；ducks_react_to_being_inside_buildings 分类需改判。

---

## 九、FIX 复核更新（2026-10-17，restorer 处理后二次核验）

restorer 已处理 FIX ①-⑨；`cargo check -p system-critters` = **0 error / 8 warning**（本复核者实测复核）。

| FIX | 状态 | 复核要点（fresh 反编译/radare2 对照） |
|---|---|---|
| ① move_critters spherecast 门反向 | **fix_verified** | shared.rs L128 `if !blocked && !locomotion.wall_hash_only()`——与二进制 `(6<loc)\|\|(99>>(loc&31)&1)==0` 一致（loc∈mask 直接位移，非成员才 spherecast）。岩石路径补 `rock \|\| water<=0`（水<=0 用 water2 恒真近似） |
| ② update_duck_goals 状态机 | **fix_verified** | duck.rs L360 复位=`dwell[0]<=0 && aux==0`（唯一）；L369 keep=`aux!=0 \|\| dwell[1]>0`——与我 fresh 反编译的 3 种组合逐一对照一致（dwell[0]>0&dwell[1]>0 现走 keep；dwell[0]<=0&dwell[1]>0 现复位） |
| ③ set_butterfly_velocities dir | **fix_verified** | butterfly.rs L342 `Vec3::new(d.x,d.y,d.z).normalize_or_zero()*1.76`——与反编译 `fVar19=dy=goal.y-trans.y` 高字、`CONCAT44(fVar16*fVar19*1.76,...)` 一致 |
| ④ critters_avoid_collisions | **fix_verified（有残留）** | shared.rs L264 loc<7&&mask→保留、L267 近距 hit&&dist<1.0→清目标（<0.4→DespawnDuckCmd）、远距/未命中→保留。**残留**：二进制 `hitcode==2→保留` 未建模（legacy_spherecast 丢弃命中码，待取证 ⑬ 命中码 2/3/7 语义）；`DespawnDuckCmd` 未判 duck 标志（water8 近似） |
| ⑤ fish 抖动轴位 | **fix_verified** | fish.rs L69 `jy=r-0.25`（无 0.5 缩放）、L70 `jz=(r-0.5)*0.5`（无 -0.25）——与反编译 `fVar46=u-0.25`（y 高字）、`fVar43=(u-0.5)*0.5`（z）一致 |
| ⑥ BoidFlock::remove 越界 | **部分验证** | aux 侧 `extras.swap_remove(idx)` 越界即 panic，与二进制 `swap_remove::assert_failed` 一致；**boid 侧 `idx>=len→break` 仍不 panic**（二进制 break 后落到 assert_failed panic）。正常态（索引恒合法）等价，错误路径语义仍有别——登记不阻塞 |
| ⑦ frame_update 首邻居单门 | **fix_verified** | boids.rs L170 `if count==0 { heading only } else { 双门 }`——与二进制双循环 loop-versioning（count==0 单 -0.5 门、count>0 加 vel·nvel>=0 门）一致 |
| ⑧ ducks_react 分类 | **fix_verified** | 模块头改标 `未实现-需重做`（墙组查询结构 U、GladeBorder 未接线、2D 射线法围合近似明确标注）——不再当作 glue |
| ⑨ move_critters blocked 三件套 | **fix_verified** | shared.rs L140 `if !blocked { 位移+朝向+BirdSpots }`——blocked 时整段跳过，与二进制 `goto LAB_142319506` 一致 |

**残留待取证（不阻塞，登记准确，无新过度断言）：**
- spherecast 命中码 2/3/7 语义（影响 ④ 的 hitcode==2 保留路径、move_critters 命中码==2 保留、sheep_unstuck）。
- move_critters mult==0 墙体路径位移来源（critters_avoid_collisions 承担，仍未建模位移）。
- 墙 relate 循环退出语义、query 环境值写、RAY_DIRS 字面量、is_within_glade_shape、CritterGoal 字节序/sample_at_world_xz→u32、动画跳表 handler——维持登记。
- ⑥ boid 越界 break-vs-panic 语义。

**复核结论：FIX ①-⑨ 主体 fix_verified（⑥ 部分、④ 残留 hitcode）；无新过度断言。cargo check 0 error/8 warning 属实。**
