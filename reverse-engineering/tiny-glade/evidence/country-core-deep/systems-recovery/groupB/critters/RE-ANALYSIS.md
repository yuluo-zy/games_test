# system-critters RE-ANALYSIS（组 B re-analyst，dbid tg_xb1，2026-10-16）

> 深挖范围：skeleton/B/crates/systems/critters（51 业务）中**无章节结论/证据不足**的函数。
> 本次新增反编译：50/51（唯一无 VA 的为 register_required_components 胶水，按 AGENTS.md 不计）。
> 前置证据（复用，不再推导）：`evidence/multi-baseline/critters-behavior-analysis.md`（duck 5 链、shared 3 函数、
> sheep_walk_on_terrain、update_sheep_goals）、`critters-extra-analysis.md`（wren_boids/fish/butterfly/firefly/spawn_ducks/startup_critters）、
> `evidence/core-recovery-20261015-adv/B-creatures-audio/sheep-SPEC.md`+`sheep-DELTA.md`（Sheep 组件、capture/apply、save 节）。
> 反编译落盘：`systems-recovery/groupB/decomp/decomp-<va>.txt`。O=机器码证实 / I=推断 / U=未知。

## 1. 本次深挖函数清单与结论

| VA | PDB 名 | 结论 | 关键契约（restorer 接口） |
|---|---|---|---|
| 0x142315e40 | set_sheep_velocities_towards_goals | **O 新** | 见 §2.1 |
| 0x142316490 | update_sheep_audio | **O 新** | 见 §2.2 |
| 0x14231a630 | sheep_walk_animation | **O 新** | 见 §2.3（此前完全空白） |
| 0x1423246b0 | sheep_unstuck | **O 新** | 见 §2.4（此前完全空白） |
| 0x14231ac20 | sheep_pet_animation | **O 新（结构）** | 见 §2.5 |
| 0x14231b720 | sheep_input_record_replay | **O 新** | 见 §2.6 |
| 0x142320cf0 / 0x142321980 | sheep_floaties / sheep_umbrellas | **O 新（结构）** | 见 §2.7 |
| 0x14231ff70 | sheep_reaction_emoji | **O 新** | 见 §2.8 |
| 0x14231b140 | ducks_react_to_being_inside_buildings | **O 新** | 见 §2.9 |
| 0x14231f610 | react_to_being_inside_buildings | **O 新** | 见 §2.13 |
| 0x142322650 | duck_splash_particle | **O 新（结构）** | 见 §2.10 |
| 0x14231c7d0 / 0x14231d6c0 | update_decorator_bird_spots / update_roof_bird_spots | **O 新（结构）** | 见 §2.11 |
| 0x14231e390 | update_wrens | **O 部分** | 音频 set_position + 地图操作；FUN_140b0xxxx/FUN_140ad5020 身份 U |
| 0x141dc2170 | Boid::random_vel_change | **O 复核**（与 extra-analysis 一致） | 补充：0x140caf380=RNG、0x140c90880=均匀采样、0x140c904d0=lerp、0x140c90600=转向、0x140d4ae30=arc 四元数 |
| 0x141dc2740 / 0x141dc3510 | Boid::frame_update<fish/wren> | **O 复核** | 补充：邻居 stride 6f32；wren 用 closure FUN_141dc42f0/FUN_141db7f20 边界（fish 用 bilinear 水面）；`*param_2==0`（fish）/`==1`（wren）判目标 |
| 0x141dc2340 / 0x141dc2540 | BoidFlock::remove<fish/wren> | **O 复核** | swap_remove（0x18 槽 + 0x38 槽双数组）+ 谓词 0x140ceafa0（是鸟/鱼）→ despawn |
| 0x140f90cd0 / 0x140f909f0 / 0x140f90c10 | fish / FishSpawnLocations::new / despawn_fish | O（extra-analysis 已覆盖） | despawn_fish 补：free 0x18 槽数组（*0x18）与 0x38 槽数组（0x38），重建成 0x28 大小的 FreshSpawnLocations 式记录 |
| 0x14067d440 / 0x14067d810 | critters::build / sheep::build | **O 部分** | §2.12 |
| duck/butterfly/firefly/wren_boids/spawn_ducks/shared 其余 | 已章节覆盖 | 复用 | — |

## 2. 关键契约

### 2.1 set_sheep_velocities_towards_goals @0x142315e40（O）
- 查询：`(Transform, &Sheep, &CritterGoal, &mut CritterVelocityTarget, &mut CritterVelocityMultiplier)` + 额外 4B 记录（写 query env u32，语义 U）。
- 逐羊：`CritterGoal.active(bit0) && Sheep+0x30(byte)==1` 且 **`!Sheep::being_petted()`**（0x14089d4c0：`movss xmm0,[rcx+0x58]; ucomiss xmm0,[rip+0x20c31a8]; seta al`，常量 **0x142960674 = 0.01f**）时：
  `VelocityTarget.dir = normalize(goal.pos - transform.pos)`（**含 y 分量**，与鸭不同）；否则 dir=0。
- `Multiplier = 1.0f`（0x3f800000）；另一 4B 记录 = query 环境值（同 move_critters 的 multiplier 语义 U）。
- 常量：1.0 = 0x142925904。

### 2.2 update_sheep_audio @0x142316490（O）
- 查询：`(Transform, &mut AudioLoop(0x38 槽), 4B 记录, ...)`；参数 2 = `Res<Time>`（dt@+0x28）。
- 逐羊：`AudioLoop::set_position(transform.pos)`（FUN_140a57da0）；`AudioLoop+0x34 (f32) += dt`（播放计时）；4B 记录 = env u32。
- 两个查询形状分支由 Query 过滤字节（+0x108）选择；无状态转移。

### 2.3 sheep_walk_animation @0x14231a630（O，**此前空白**）
- 查询：`(Option<&Transform>, &mut Sheep(0x68), 4B 记录)`；参数 2 = `Res<Time>`（dt@+0x28）。
- 逐羊：
  1. `speed = |transform.pos - Sheep.last_pos| / dt` → 写 **Sheep+0x48 (f32)**；`Sheep+0x4c/0x50/0x54 (Vec3) = transform.pos`（无 Transform 时保留旧值）。
  2. 状态转移（**Sheep+0x10 (u32) 动画状态**、**+0x14 (f32) 状态计时**）：
     - `speed <= 0.2`（0x1429cd068）：state==1 → **state=2, timer=0**；否则保持。
     - `speed > 0.2`：→ **state=1**（walking；state==1 保持 timer）。
  3. `!being_petted` 时按 `state` 跳表分发（0x14304b0f8，4 项都在函数内）：
     - state 0/3：无动画推进（跳过）。
     - **state 1（walking）**：`Sheep+0x5c (f32 动画计时) += dt` → `= fmodf(+0x5c, 1.25)`（0x142923f10=fmod 族别名，模 1.25）；
       `frame = trunc(clamp(24.0 * fmod_timer, 0..∞))`；**`Sheep+0x28 (u64) = (frame + 20*Sheep+0x20 + 3) mod 30`**（+0x20 = 每羊相位基址 u64；24 帧×1.25s=30 帧周期）。
     - **state 2（idle）**：`Sheep+0x14 += dt/0.1`（0x142925984=0.1）；`+0x5c += dt` + 同 frame 计算；
       `timer >= 1.0`（0x142925904）→ **state=3**，否则保持 2。
  - 常量汇总：0.2 阈值=0x1429cd068、1.25 模=0x142a8d600、24 帧=0x142934600、0.1=0x142925984、1.0=0x142925904。
- **Sheep 新增字段（此前空白区，O）**：+0x10 u32 动画状态、+0x14 f32 状态计时、+0x20 u64 相位、+0x28 u64 帧索引、
  +0x48 f32 速度、+0x4c..+0x54 Vec3 上一帧位置、+0x5c f32 动画计时。均不与 sheep-SPEC 已证字段（+0x30/+0x31/+0x34/+0x38/+0x58）冲突。

### 2.4 sheep_unstuck @0x1423246b0（O，**此前空白**）
- 参数：Query(Transform@0x30, 8B 状态记录, 4B 记录, &mut Sheep@0x68, 4B 记录)、Res(WaterRaster 双缓冲 +0x298)、预算资源 ×2、标志 param_7、TerrainHeightSsbo、SheepSpawn 相关资源（+0x58）、Time。
- **预算门控**：预算资源布局（**与 sheep_pet_animation/ducks_react 相同**）：`{+8/+0x10 子预算1, +0x18 消耗1, +0x28/+0x30 子预算2, +0x38 消耗2, +0x40 总额}`（u64×7）；
  `预算 = +0x40 - (saturating(+0x10-+0x18 换算) + saturating(+0x30-+0x38 换算))`（saturating 减）。两资源预算都耗尽且标志已置 1 → 执行；标志 0→1 后 return（每帧最多一次）。
- 逐羊（FrozenMap::get 校验 buffer 版本 0x3f 后）：
  - `RaycastWorld::legacy_spherecast(pos + (0, 1.0?y偏置 0x14295a0e8), r=1.0)`（0x1408ccc90；0x1429ddd00 = 射线参数）→ hit 才处理。
  - `rock = terrain_rock_mask::sample_at_world_xz`（0x1409ea800）；`water = bilinear_sample_at_world_xz<512>`（0x140a14df0，参数 130.0=0x142a7b658）。
  - 8B 状态记录 u32（CritterLocomotion 同族，掩码 `0x54U>>(s&31)&1` 选 {2,4,6}）状态机：
    - s==2 且 water>0：`func_0x000140ae8320(资源+0x58)` 非 0 → **s=4**；否则 **s=0**。
    - s==5 且 !rock && water<=0：`terrain_h = TerrainHeightsData::sample_at_world_xz<TerrainHeightSsbo>(0x141246fb0)`；
      `pos.y > terrain_h` → **s=6**。
    - s==2（dry）：`target_h = max(terrain_h, 0.5)`（0x142925870）；`(Sheep+0 byte&1) && |(target_h - Sheep+4)/dt| < 阈值(0x1429345a8/0x14292e980 位滤)` → **s=0**；
      然后**恒写 `Sheep+0 = 1; Sheep+1..3 = 0; Sheep+4 = target_h`**。
    - rock 或 s>6 或掩码位 0 → **s=0**。
  - **状态离开 2 时**：`Sheep+0x30==0`（move_state Idle）→ **清零 Sheep+0x34..+0x37（timeout_s+goal_change_timeout）**。
- **Sheep 新增字段**：+0 byte u8 地形标志（bit0=有地形目标）、+4 f32 地形高度目标。

### 2.5 sheep_pet_animation @0x14231ac20（O 结构）
- 预算资源同 §2.4（首个参数即预算 ResMut）。
- 查询 (Transform?, Sheep@0x68)；`lVar2 = Sheep+0x20`（相位键）；在 pet 列表（0x140 字节缓冲，8×u64/条）中查 **Sheep+0x20 成员**；
- 按 **Sheep+0x18 (u32) pet 状态** 跳表（0x14304b178，5 项全部落在函数内：0x14231afe4/0x14231b0b7/0x14231ae2a?/…）两种调用形参：
  成员命中（=pet 中）与未命中（=发起 pet）；参数 (常量, 1.0f, entity idx)。内部块语义（各状态 handler）U。
- **Sheep 新增字段**：+0x18 u32 pet 状态。

### 2.6 sheep_input_record_replay @0x14231b720（O）
- 参数 2 = ResMut（+0x1c tick 回写 = Mut 模式）；模式 = **资源+0x40 Option<u64>（判别 = XOR 0x8000…；值 1/2=record、3/4=replay）**。
- record：`Vec<Vec3>`（游戏布局 cap@+0/ptr@+8/len@+0x10，旧 cap 先 free(0xc))收集查询 Transform 位置；第二个查询收集 4B 记录 Vec（stride 4，free 用 len*4）。
- replay（模式 3/4，门控 `资源A+0xfb==0 && 资源B+0 !=1` 才执行）：`FUN_140949970(资源, 资源+0x48, …)` 取回记录（FrozenMap 类）；
  逐羊 zip 写 `Transform.translation = 记录 Vec3`；`FUN_140d4b440` = 事件写（EventWriter，事件类型 U）。
- 恐慌：record 取回失败 panic（UNK_14304b2b0/2c8 为 panic 位置指针）。

### 2.7 sheep_floaties / sheep_umbrellas @0x142320cf0 / 0x142321980（O 结构，两函数同构仅字符串/顺序差）
- 查询 + 音频：逐羊 `AudioCore::instantiate_audio("event:/SFX/Sheep/UmbrellaOpen"/"UmbrellaClose")`（FUN_140a55f40，0x14304b9f0/0x14304b9b8 len 0x1d/0x1e；umbrella 版顺序相反）、
  `AudioLoop::set_position`（0x140a57da0）、`start_if_not_playing`（0x140a57300）、`set_parameter`（0x14089a280）、Vec 收集（FUN_1428a57d0，0x78 槽）、
  每记录写 vtable 闭包 FUN_141dc08c0/0x68→FUN_141dc1e50/0x78→FUN_141dc06d0/0x80→FUN_141dc1e50（状态/更新闭包，语义 U）。
- `FUN_140c95ef0(&UNK_14304ba28, t)` = 曲线求值（控制字节 '\x05'）。
- 恐慌串：`called Result::unwrap() on an Err value`（sheep_floatie.rs / sheep_umbrella.rs，0x14304b940/0x14304ba50）。

### 2.8 sheep_reaction_emoji @0x14231ff70（O）
- 查询 (Transform@0x30, Sheep@0x68, CritterGoal@0x10?, 0x1c 槽 emoji 记录)；逐羊 emoji 记录（0x1c）：
  - `+0x0` 8B entity、`+0xc` f32 计时（+= dt）、`+0x18` byte 状态、`+0x10` 8B 缩放向量。
  - 状态机：**0**（淡入）：t=clamp(timer/0.2,0,1)；scale = curve(0x14304b890)(t)×向量；t==1 → state=1、timer=0。
    **1**（保持）：timer>=1.0 → state=2、timer=0。**2**（淡出）：t=clamp(timer/const(0x142925970),0,1)；
    scale = curve(1-t)；t==1 → `EntityCommands::despawn`（0x140d14770）。
- 每帧尾：`f32 = func_0x000140949c10(资源)`（曲线长/音频位，U）；`func_0x000140ac8780(entity, f32, t, 资源)` 写 0x10 槽记录
  （{u32@+0=env, f32@+4, Vec2@+8}）—— emoji 位置/尺度写入，语义 U。

### 2.9 ducks_react_to_being_inside_buildings @0x14231b140（O）
- 预算资源同 §2.4（**鸭预算**）。
- 外墙循环：`PublicWallState::max_y_gable`（0x140a91ef0）取墙（0x140a91ef0 签名 (walls, wall_id)）；墙记录条件 `1 < 段数(+600) || 0.1? < y(+0x280)`（0x1429258d0）。
- 逐鸭 (Duck@0x100, Transform@0x30)：`FUN_140b2a5d0`/`FUN_140b2a620` = 墙高度访问器（y 基 / max_y_gable f32）；
  `FUN_140a922b0` = 墙→内建位集映射（+0x14 槽记录，0xffffffff 哨兵；0x38 表 bitset >>6/&0x3f）；
  `fVar28 = TerrainHeight 类采样`（0x1408e3260）；`func_0x000140ad2950(wall, x, z)` = 楼内判定；
  **`楼内 && wall_y_base <= duck.y <= max_y_gable + fVar28` → `AudioLoop::start_if_not_playing`（鸭音频句柄在 Duck 组件内，按 entity idx×0x100 索引）**。

### 2.10 duck_splash_particle @0x142322650（O 结构）
- `EntityCommands::despawn` 前奏（0x140d14770，@0x14304bb78 参数）；`AssetMeshLibrary::get_handle_by_name("splash",6)`（0x14092d4f0）
  + `AssetShaderLibrary::get_raster_handle_by_name("splash_particle",0xf)`（0x140937aa0）；Vec 收集 + 闭包 0x141dc0de/0x141dc1e5（粒子）。
- 恐慌：UNK_14304bba8。

### 2.11 update_decorator_bird_spots / update_roof_bird_spots（O 结构）
- **update_decorator_bird_spots @0x14231c7d0**：门控 `func_0x000140c16a00()`（质量/设置）；HashMap 初始化（0x140c18130、容量 0x32）；
  逐装饰物 (Transform@0x30 + 0x10 槽 BirdSpot 源记录 {entity u32, 3×u32} )：quat 旋转偏移（fVar16/17/18=quat.xyz、fVar19..21=quat.w…）→ 世界坐标；
  哈希去重（0x1405b0ed0 哈希、FUN_141d91e30 查表）：同位置跳过；已占位（位集 + 0x14 槽映射 + 0x140ceafa0 谓词）→ despawn 旧；
  否则 `alloc(0x18)` 建 **BirdSpot 记录 {u32@+4=0, pos Vec3@+8, u32@+0x14=资产句柄}`**（句柄=FUN_140f86330），
  `Commands::spawn`（FUN_140f86340）+ 收集（FUN_141dc0f70）+ **HashMap insert（FUN_14261f6f0，键=entity）**；收尾 map 清 stale（FUN_14261f880）+ despawn。
- **update_roof_bird_spots @0x14231d6c0**：同构，源 = 屋顶几何（0x1408e33b0 屋顶查询族）；spawn/收集闭包相同（FUN_140f86340/FUN_141dc0f70）。

### 2.12 critters::build @0x14067d440 / sheep::build @0x14067d810（O 部分）
- 均为 Bevy Plugin build：init_resource/insert_resource 序列（FUN_140f7d430/FUN_140f80620 等 App builder 内部 API）；
  **Perchance RNG state 0xc0df 注入**（FUN_140f84330(param_2, 0xc0df, 0)）；`DuckSpawnLocations::new`（0x140f8ee90）、`FishSpawnLocations::new`（0x140f909f0）insert；
  HashMap 键查（0x1400dd6f0）；`add_systems`（FUN_141dbd790，@UNK_142badea0/142bade10）；sheep build 还 register_type（FUN_140f93cd0/FUN_140f92a40，类型键 0xebbff8344555b7ea/0x1173151c151d7a3b/0x1888c9e7607b4b94）。
  系统函数指针在数据表内（decomp 不直接可见）——**系统注册顺序需 restorer 以骨架 lib.rs 顺序 + 反汇编核对**。

### 2.13 react_to_being_inside_buildings @0x14231f610（O，emoji 出生端，与 §2.8 消费端闭环）
- 与 §2.9 同构的楼内判定链：`PublicWalls::get`（0x140a91ef0）→ 墙高度访问器（0x140b2a5d0/0x140b2a620）→ 墙→位集映射（0x140a922b0）→ 地形采样（0x1408e3260）→ `0x140ad2950` 楼内谓词。
- 逐羊 (Sheep@0x68)：楼内命中 → `func_0x000140c96240(*(int *)(Sheep+0x20) * 100 + …)` = **fastrand RNG 以 Sheep+0x20 播种**（0x140c96240=init_rng）；
  `alloc(0x140613c10)(len)` + memcpy（数据模板 UNK_14304b8c8+offset）→ 经 `FUN_141dc1110`（Commands::spawn 类）+ 列表 push（**UNK_143148fc8 = emoji 记录列表头**，0x1c 槽）→ 生成反应 emoji 实体。
- 即：**羊进楼 → 播 RNG 生成 emoji → sheep_reaction_emoji（§2.8）驱动 3 态动画（淡入 0.2s/保持 1.0s/曲线淡出）→ despawn**。

## 3. Sheep 组件完整字段表（O，跨 SPEC + 本次）

| 偏移 | 大小 | 字段 | 证据 |
|---|---|---|---|
| +0x00 | 1B | 地形标志（bit0=有地形目标） | sheep_unstuck 0x142324ddb |
| +0x04 | 4B | f32 地形高度目标 | 同上 |
| +0x08 | 4B | u32 行为状态（1=跟随） | update_sheep_goals（multi-baseline §4） |
| +0x0c | 4B | f32 目标计时 | 同上 |
| +0x10 | 4B | u32 动画状态（0/1/2/3） | sheep_walk_animation |
| +0x14 | 4B | f32 动画状态计时 | 同上 |
| +0x18 | 4B | u32 pet 状态 | sheep_pet_animation |
| +0x20 | 8B | u64 动画相位基址 / pet 列表键 | sheep_walk_animation / sheep_pet_animation |
| +0x28 | 8B | u64 动画帧索引（30 帧循环） | sheep_walk_animation |
| +0x30 | 1B | SheepMoveState 判别（0=Idle,1=Walking） | sheep-SPEC / set_sheep_velocities（==1 门） |
| +0x31 | 1B | force_reach_goal | sheep-SPEC |
| +0x34 | 4B | timeout_s | sheep-SPEC / unstuck 清零 |
| +0x38 | 4B | goal_change_timeout | sheep-SPEC / unstuck 清零 |
| +0x48 | 4B | f32 速度（帧距/dt） | sheep_walk_animation |
| +0x4c..+0x54 | 12B | Vec3 上一帧位置 | 同上 |
| +0x58 | 4B | being_petted f32（**> 0.01f** 谓词；常量 0x142960674=0x3c23d70a，本次 rip 解析校正） | 0x14089d4c0 / set_sheep_velocities / sheep_walk_animation 消费 |
| +0x5c | 4B | f32 动画计时（fmod 1.25 周期） | sheep_walk_animation |

stride 0x68；`+0x3c..+0x47`、`+0x60..+0x67` 仍 U（不填占位）。移动状态机 Idle↔Walking 的**转移驱动 = system_critters**（update_sheep_goals 写 +0x30/+0x34/+0x38；unstuck/walk_animation 消费/清写），
补上了 sheep-SPEC §4 T3 的取证缺口（O）。

## 4. 对 reconstruction/critters 既有实现的差异与补缺

- `src/sheep_terrain.rs`（已有）：与 sheep_unstuck 的 +0/+4 字段**不一致**——sheep_terrain 草稿的"羊地形状态 8B{state u32@+0, f32@+4}"对应**另一个 8B 状态记录**（CritterLocomotion 类），
  非 Sheep 组件字段；Sheep 自身 +0/+4 由 **sheep_unstuck** 写。restorer 须区分：Sheep 组件 vs 8B CritterLocomotion 状态组件。
- `src/duck.rs`/`src/shared.rs`/`src/wren_boids.rs`/`src/butterfly.rs`/`src/firefly.rs`（已有）：本次反编译复核一致，无校正。
- **firefly 惯性 bias 纠错（与 reviewer-B tg_vB1 交叉确认）**：`update_firefly_goals` 的 bias 双浮点常量 0xbf0000003f000000
  按 `goal.inactive` 取 **有目标 +0.5 / 无目标 -0.5**（RNE 0x142ccfc58 = `00 00 00 3f 00 00 00 bf`）。
  **critters-extra-analysis.md §4"校正 1"声称无目标应为 -1.0 是错的**；reconstruction/firefly.rs 草稿的 -0.5 正确，照文档"校正"实现会引入 bug。
- **未覆盖文件（restorer 新增）**：sheep_behavior.rs（set_velocities/audio 已给契约）、sheep_walk_animation.rs、sheep_unstuck.rs、sheep_floatie.rs、sheep_umbrella.rs、
  sheep_pet_animation.rs、sheep_input_record_replay.rs、sheep_react_to_being_inside_buildings.rs（react_to_being_inside_buildings @0x14231f610 未精读，见 §6）、
  ui_pet_sheep.rs、ui_pet_ducks.rs、wren.rs、spawn_ducks.rs（§5 已有）、duck_splash_particle.rs、ducks_react_to_being_inside_buildings.rs。

## 5. 跨领域线索

- 预算资源（7×u64 布局）被 sheep_unstuck/sheep_pet_animation/ducks_react_to_being_inside_buildings 共用——疑为 country-core 的 `SheepSpawnBudget`/`DuckSpawnBudget` 类资源（消费链登记）。
- `AudioLoop::start_if_not_playing`（0x140a57300）/`set_position`（0x140a57da0）/`set_parameter`（0x14089a280）= country_core audio_resources。
- `Sheep::being_petted` 0x14089d4c0 = country-core（sheep-SPEC 已证）；本次确认被 set_sheep_velocities/sheep_walk_animation 双消费。
- `RaycastWorld::legacy_spherecast` 0x1408ccc90、`terrain_rock_mask::sample_at_world_xz` 0x1409ea800、`TerrainHeightsData::sample_at_world_xz` 0x141246fb0、
  `bilinear_sample_at_world_xz<512>` 0x140a14df0、`PublicWallState::max_y_gable` 0x140a91ef0 = country-core 各资源（与 multi-baseline 一致）。
- BirdSpotManager HashMap 族：0x140c18130（new）、FUN_14291b030、FUN_141d91e30（get）、FUN_14261f6f0（insert）、FUN_14261f880（drain）——疑 country-core `resources/critters.rs`（sheep-SPEC §5 已登记）。
- 0x140ceafa0 = "是鸟/鱼实体" 谓词，被 BoidFlock::remove<fish>、BoidFlock::remove<wren>、update_decorator_bird_spots 共用（应落 shared）。

## 6. 仍 U（不臆造）

- ui_pet_sheep/ui_pet_ducks 的精确 UI 布局（文本 "sheep_pet"@0x14304be70 已证；按钮/射线细节 U）。
- update_wrens 的 FUN_140b0xxxx（0x140b02490/c0/e0/0x140b04e70/0x140b04f70/0x140b02520）与 FUN_140ad5020 身份。
- Boid::frame_update 内 0x140c904e0（NaN→0 随机换）、0x140c95ef0 曲线 id 0x14304b890 的曲线定义（country-core Curve 资产）。
- sheep_floaties/umbrellas 的 FUN_141dc08c0/FUN_141dc06d0/FUN_141dc1e50 闭包语义（漂浮/开伞状态机，疑本文件内私有函数）。
- 0x14067d440/0x14067d810 内 add_systems 的具体注册表（函数指针数据表，需反汇编核对骨架 lib.rs 顺序）。
