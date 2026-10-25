# critters 补全：wren_boids / fish / butterfly / firefly / spawn_ducks / startup_critters（B 主基线，反汇编级证据）

2026-10-13。B patched EXE（sha256 cf048e27…6323），RNE 发行原件（sha256 f507a6ce…d03）作机器码 oracle。
radare2/RNE 对 20 个新入口逐一抽点反汇编（`evidence/toolchain/critters-extra-radare.json`），与 B 地址一致，无 patch 偏移。

## 证据来源（本次精读）

- **reva**（B 持久库，worker `critter-extra-owner`，只读脚本，programPath=/tiny-glade.exe）：20 个目标函数
  逐函数反编译 C + 全线性反汇编（分 0x800 窗口）。结果落盘 `evidence/toolchain/critters-extra/{fn}.{decompile.json,decompiled.c,disasm.json}`。
  sim_wren_boids 反编译 stdout 截断（65536B），保留部分存 `sim_wren_boids.decompiled.c.partial`，完整靠 10 窗口 disasm。
- **radare2**（RNE oracle，worker `critter-extra-r2`）：20 入口抽点 + 33 个 .data 常量 hexdump
  （`critters-extra-radare.json`、`critters-extra-r2-consts.json`）。
- **reva 常量读**（struct 解码）：`.data` f32 读取全 0（worker Memory 读失败），改用 RNE hexdump 解码。
- **PDB 行号表**：skeleton/B/crates/systems/critters/src/{wren_boids,fish,butterfly,firefly,spawn_ducks}.rs.pdb.jsonl.gz
  与 country-core startup_critters.rs。函数→源码行：sim_wren_boids L56–533、fish L92–334、
  update_butterfly_states L43–253 / goals L256–331 / velocities L334–378 / audio L381–385、
  update_firefly_goals L46–112 / set_velocities L115–136、spawn_ducks L80–197 / despawn L206–278、
  DuckSpawnLocations L54–75、FishSpawnLocations L63–81。boids.rs：frame_update L329–387、
  random_vel_change L389–403、BoidFlock::remove L114–124。

## 横切证据

- **RNG 二分**：goals 系（duck/fish/butterfly/firefly/sheep 目标）用 `perchance::PerchanceContext::uniform_f32`
  （0x141625ae0）；boids 随机转向与 **spawn 位置**（wren/fish 出生）用 `fastrand::Rng::f32`。两者并存，不混用。
- **出生采样工具**：fish/duck 出生用 `uniform_sample_disk/circle/sphere`、`golden_spiral`、`radical_inverse`（游戏自有 utils）；boids 转向用 `uniform_sample_cone`。
- **dt**：一律 `*Res<Time> + 0x28` f32。

---

## 1. wren_boids（sim_wren_boids 0x142326a80，19140B，源码 56–533）

### 1.1 顶层结构（partial decompile + 10 窗口 disasm）
- L340 分支 `*(char *)(*in_R8 + 5)`：非 0 时执行主模拟；==0 时清空全部 Wren 实体（逐 Wren `Commands::despawn`，`Entities::contains` 门控）→ 发行构建的"wren 开关"。
- **出生**（L385–515）：`dt != 0`（Time+0x28）时逐帧补出生。上限 `0x4f < uVar58 → break`（**80 只**）。位置 `fastrand::f32`：
  `x = u*130 - 65`（0x43020000/0xc2820000）、`y = u*9 + 1`（0x41100000/0x3f800000）、`z` 同理。
  出生 = `Commands::spawn<(Wren, NaniBundle<WrenInstanceData>, PrevTransform)>` + 音效（AudioCore::instantiate_audio）+ 写 Boid（0x18：pos Vec3 + vel Vec3）+ 0x38 槽记录。
- **FreeSpot 收集**（L700）：`from_iter<FreeSpot, FlatMap<QueryIter<BirdSpots>, FilterMap<closure_1>>>`——取停靠点。
- **BirdHaven 树查询**（L745）：`multiunzip<FilterMap<QueryIter<(Entity, BirdHaven, PrefabInstanceData), Without<TreeMarkedForDeletion>>, closure_env_2>, (Vec<Sphere>, Vec<Entity>)>`——取树庇护球体。
- **主循环**（L1014+）：对每只现存 Wren，`SQRT(x²+y²) <= 0x42b7d907 (91.9)` 且 `y <= 0x41f00000 (30)` 才保留（飞出该半径即被 BoidFlock::remove）；处理停靠点/树摇。
- **摇树出生**（L1231+）：`fVar91/fVar76 > 10.0`（0x41200000）且竖直计时 <=0 → log "Tree has been shaken! Spawning b…"；`burst = rng*5+1` 只鸟（每只：`Commands::spawn<Wren 三元组>` + 写 Boid）。Boid 数量仍受 80 上限（`(ulonglong)plVar55[2] < 0x50`）。
- **BoidFlock::remove<wren closure_env$6>**（L1124）清理 dead 实体（swap_remove + despawn）。

### 1.2 boids 群体规则（Boid::frame_update<closure_env>，boids.rs 329–387；fish/wren 两实例反编译一致）
参数经栈传（call site 常量，见下）：
- **邻居判定**：dist² `< 16.0`（0x41800000，**半径 4.0**），且 **heading 过滤** `self.vel · neighbor.vel >= 0`（同向才计入）；
  第二条件 `(dir·nvel)/sqrt(...) >= -1.0`（cos 恒真，等价 NaN 门）。
- **分离**：dist² `< sep_r²`（sep_r=鱼 0.8 / 鹪鹩 1.3）时
  `sep -= normalize(pos-self) * smoothstep(1 - dist/sep_r)`；平滑用 `(4-2t)t²`。
- **对齐**：`vel = mean_nvel * align_w + vel * (1-align_w)`（align_w=鱼 0.0584 / 鹪鹩 0.1169）。
- **凝聚**：`vel += normalize(mean_pos - self_pos) * |vel| * 0.0056`（0x3ba3d70a）。
- **组合权重**：LAB 合并处 `vel += sep * 1.7`（0x3fd9999a）。
- **速度约束**：加速度 delta clamp 到 [-max, max/2]（max=1.0，有目标 1.5）；最终 `|vel|` clamp 到 **[min, max]**（min=鱼 0.9/3.0、鹪鹩 7.0；max=鱼 4.0/8.0、鹪鹩 14.0；鱼的值由 `BackupSaveFileKind::is_corrupted()` 谓词选）。
- **NaN → 归零**：任一分量 0x7f7fffff 越界即清 vel。
- **位移**：`pos += vel * dt`（dt=in_stack_00000060）。
- **水面/地面转向**：fish 实例对 `bilinear_sample_at_world_xz<RasterBufferRef<512>>` 采样 `>= 0.5` 判水；
  非水 → `pos -= vel*dt` 且 `vel = -vel * 0.9`（反弹回水）。wren 实例对 `closure_3`（RAY_DIRS 32 方向 + Raycasting::raycast）判"可行点"；
  不可行 → 用 `from_rotation_arc` 转向 RAY_DIRS 方向（`fVar12*40c00000 + fVar36*c0a00000`，含 -5.0 常数）。
- **RAY_DIRS**：const 数组 0x200 字节 = 32 个 Vec3，`DAT_14363aad0` futex Once 惰性初始化。

### 1.3 call-site 权重常量（disasm 证实）
- **fish frame_update**（0x140f9164d 前）：[RSP+0x40]=0x3f4ccccd=**0.8（sep_r）**、[RSP+0x48]=0x3d6f9db2=**0.0584（align_w）**、
  [RSP+0x30]/[RSP+0x38]=min/max speed（is_corrupted 选 0.9/4.0 或 3.0/8.0）、[RSP+0x58]=dt。
- **wren frame_update**（0x1423283e5 前）：[RSP+0x40]=0x3fa66666=**1.3（sep_r）**、[RSP+0x48]=0x3def9db2=**0.1169（align_w）**、
  [RSP+0x30]/[RSP+0x38]=**7.0/14.0**、[RSP+0x58]=dt。
- 凝聚 0.0056、分离 1.7 均为帧内常量（0x142f7d784/0x142f7d7a4 等 .data 只存广播值）。

### 1.4 Boid::random_vel_change（boids.rs 389–403，0x141dc2170）
- `timer@+0x30 -= dt`；`<=0` → 2×`fastrand::f32`+`uniform_sample_cone` 取新随机锥向，`fastrand::f32`+`lerp` 重设 timer；
  随后 `from_rotation_arc`+四元数旋转把 vel（Boid+0xc）转回该方向。→ 每随机间隔选一次随机转向。

### 1.5 BoidFlock::remove（boids.rs 114–124）
对索引列表（u32 Vec）倒序遍历：校验索引 → boid 0x18 槽与 0x38 槽 swap_remove → `Entities::contains` 非空则 despawn。批清理。

### 1.6 sim_wren_boids::closure_3（0x141dc42f0，RAY_DIRS 谓词）
ray 端点在 y∈[0,30] 且（glade 标志==0 或 半径<=91.9）时 `Raycasting::raycast`；`DEBUG_SHOW_BIRD_RAYCASTS` 时画 DebugLine。

---

## 2. fish（fish 0x140f90cd0，7533B，源码 92–334；boids 见 §1.2/1.3）

- **出生**（L166–330）：`Boid 数量 < 200`（plVar33[6]<200）时逐帧补，每帧最多 **0x14=20** 次（`while(lVar27 != 0x14)`）。
  候选点 hash 采样 FishSpawnLocations 网格；`bilinear_sample_at_world_xz` 采样 `> 0.5` 才生（水面）。
  出生 = `Commands::spawn<(Fish, NaniBundle<SolidVertexColorInstanceData>)>` + 写 Boid{pos, vel}（0x18）。
  初始 vel：x/y = `fastrand::f32 + fVar42(-0.5)`、`fVar51`（0x142a171f4=-1.0）+…；z = `fastrand::f32 - 1.0`。
- **每帧 Boid 更新**（L417–588）：每 boid `Boid::frame_update<fish closure_env_0>`；随后 4× 水面采样求 clamp[0,1]；
  `< 0.1`（0x3dcccccd）→ `WallSpatialHash::query` 逐墙段，投影距离 `< fVar42`（0x3f8f5c29≈1.12）→ `time_continuous_lerp_vec3` 推开。
- **Fish 实体转向**（L786–812）：byte==2 的 Fish 记录，2D 距离 `< 2.0`（0x40000000）→ 速度朝目标 lerp（`*0x40a00000`=5.0 / `*0x41000000`=8.0 两档常数）。
- **光标/相机逃避**（L814–863）：距光标 `< fVar51`（由设置值 uVar5∈0..15 查 switch：case1/8/0xb/0xd→4.0；case2/3→`*res+2.3`；case4/5/10/0xc→3.97；case0→0）→ 反向 lerp 逃开；
  `MainCamera::mode()==3` 且距 eye `< 2.0` → 逃开相机。
- 结构（证据）：Boid 0x18{pos Vec3, vel Vec3}；Fish 记录 stride 0x30（转向前提 byte==2）；FishSpawnLocations 网格 **25×25=625** 点在 [-52,52]²（0x42500000=52/0x41c00000=24，鱼 63–81）。

---

## 3. butterfly（源码 43–385；Butterfly 组件含状态机跳表）

- **update_butterfly_states**（0x141d84d40，43–253）：
  查询 `(Transform, &Velocity, &mut Butterfly, &mut ButterflyAudio, &CritterGoal)`；同时收集 Sheep pos（Vec2）避让。
  距离门：相机 mode==3 → 距 eye 阈值 4.0（0x40400000）；否则距光标阈值 8.0（0x40800000）。远距才推进状态机。
  状态机为**跳表**：`DAT_142f68008` 按 `Butterfly.state(u32 @+0)` 索引，各状态 handler 收 `(fVar25, fVar30, flag&8.0)`；
  Butterfly 记录：`[3]` dwell 计时 f32、`[4]` 相位 f32、`[5]` 相位参考 f32。
  计时：dwell `<=0 → 重置 = uniform_f32*0.3+0.2`（0x3e99999a/0x3e4ccccd）；相位 `|0.5 - [5]|` 在 [0.2,0.9) 外 → 跳表分支。
- **update_butterfly_goals**（0x141d86060，256–331）：
  `timer@+2 -= dt`；`timer<=0 || goal.inactive || dist2<0.16` → 新 timer=`uniform*0.5+0.1`；
  2×`uniform_f32`+`uniform_sample_sphere`+`x0z` → 方向；**按状态选竖向偏置**：state==0x1 → (-0.2835, 0.5)，
  否则 → (0.8529, 1.0)（0x142f67d24=0xbe91745d、0x142f67d14=0x3f5a2e8c）；`dir = normalize(加权采样)`；
  `proposed = pos + dir * (timer*1.77)`（0x3fe147ae）；y clamp `<= 8.0`（0x41000000）；
  校验 `sqrt(z²+x²) < 65.0 || < 原半径`（0x42820000）→ 接受，否则清 goal。
- **set_butterfly_velocities_towards_goals**（0x141d867b0，334–378）：
  查询 `(Transform, Sheep, CritterGoal, &mut CritterVelocityTarget, &mut CritterVelocityMultiplier)`；
  `dir = normalize(goal - pos) * 1.77` 写 VelocityTarget；`multiplier += 1.0`（每帧累加，语义 unknown）；
  随后按 `Butterfly.state` 跳 `DAT_142f68018`（动画/姿态 handler，收 (…, 0.5)）。
- **update_butterfly_audio**（0x141d86eb0，381–385）：逐蝴蝶 `ButterflyAudio::set_position`。
- **butterflies_send_to_gpu**（0x141d87140，472B）：SSBO 上传（gpu_glue，见前轮分组）。

---

## 4. firefly（firefly.rs 草稿已存在，本轮反汇编**校正**）

- **update_firefly_goals**（0x142316940，46–112）：入口 `*(char *)(*in_R8 + 4) != '\x01' → return`（control 门，与草稿一致）。
  每萤：`timer -= dt`；刷新条件同 duck/butterfly（timer<=0 || goal.inactive || dist2<0.16）。
  **校正 1**：惯性 bias 为 `0xbf0000003f000000` 双浮点按 `goal.inactive` 取——**有目标 +0.5、无目标 -1.0**；
  现有 firefly.rs 草稿写 `goal.is_none() → -0.5`，应为 **-1.0**（反汇编/hexdump 证实）。方向 = normalize(`sample*(1+bias)`，z 额外 +0.5)。
  新 timer=`uniform*0.15+0.05`（0x3e19999a/0x3d4ccccd，与草稿一致）；`proposed = pos + dir*timer`；
  校验 `sqrt(z²+x²)<65 || < 原半径`（0x42820000）。
- **set_firefly_velocities_towards_goals**（0x142317040，115–136）：查询 `(Transform, Sheep, CritterGoal, VelocityTarget, Multiplier)`；
  `dir = normalize(goal-pos)`；`multiplier = 1.0`（0x3f800000）；**海拔下限**：`pos.y <= 1.0` 时 `dir.y = max(dir.y, 0.25)`（0x3e800000）。
  与草稿逐字段一致，无校正。
- startup 出生 96 只（见 §5）；`fireflies_send_to_gpu`（0x142316760）SSBO。

---

## 5. spawn_ducks / startup_critters

### 5.1 DuckSpawnLocations::new（0x140f8ee90，54–75）
10×10 网格候选，共 0x4b0/0xc=**100** 点，x∈[-52,52]（0x42500000=52/0x41100000=9 分 10 档），
第二维 z 取 10 档：-52,-40.22,-28.56,-17.33,-5.78,5.78,17.33,28.56,40.22,52（0xc2500000…0x42500000）。
资源尾 `+0x18 = 25.0`（0x41c80000，语义 unknown，疑水面检测半径）。

### 5.2 spawn_ducks（0x140f8f210，80–197）
- 门控：非 demo 且 `BackupSaveFileKind::is_corrupted()==false`。
- 计时：资源 `+0x18` 累加 dt；**活鸭数 < 15**（0xf）时 3×速率累加；`timer >= 30.0`（0x41f00000）才生，生后归 0。
- 候选：`sorted_by<IterMut<(Vec2,bool)>>`（鸭子出生点，已用标志）；逐点 `bilinear 水采样 > 0.99`（0x3f7d70a4）跳过、
  `TerrainHeights::sample` 高度 `> 0` 跳过（须水面）；取中 → `uniform_f32`+`golden_spiral`+`x0y` 构 3 点轨迹 → `from_iter<Trajectory>`；
  `uniform_f32`+`uniform_sample_circle` 随机圆偏移 → `Vec::push<TrajectoryPlan>`（0x30 槽，被 plan_duck 消费）；标记候选已用。

### 5.3 despawn_ducks（0x140f8f8b0，206–278）
逐 Duck：`DuckState==2` 且水面采样（首点 + 4 采样求和）判断——深水/浅水越界 → 生成离场 TrajectoryPlan（push 资源，0x30 槽）；
`DuckState<2` 且带轨迹（Duck+0xa0!=0）同理（4+1 采样，`0.86 < 水 || 采样 < 0.1` → 离场）；`DuckState==3` 不生成。
→ 离场计划由 plan_duck 热键 B 路径 spawn/执行（同前轮 plan 链）。

### 5.4 startup_critters（0x1409d49a0，5361B）
- **蝴蝶 24 只**（0x18）：`Commands::spawn<(Transform, Butterfly, ButterflyAudio, Critter, CritterLocomotion, CritterGoal, CritterVelocityTarget, Velocity, CritterVelocityMultiplier)>`，位置/姿态 `fastrand`；ButterflyAudio = instantiate_audio("Butterfly_Shoosh" @0x142ac22f4)。
- **萤火虫 96 只**（0x60）：同组台（Firefly 替换 Butterfly，无 ButterflyAudio）；位置 `uniform_sample_disk*65`（0x42820000）+ y 随机 + `z=1.0*65`；Firefly 记录含 `+0x70=1.0`。
- 资源门：`*in_stack_00000040+0x1f==1`（critter 开关）、`*in_stack_00000048+0x58 != 0x04`（质量级 != 4）。
- 尾部 load_json_as_mesh / load_mesh_into_library / load_shader_into_library ×多（蝴蝶 glow / 萤火 glow / 萤火 网格）。

---

## 6. 前轮 unknown 的补钉

- **DuckState 语义（反汇编确认 1/2/3）**：move_ducks L370–410 中 `lVar26 = DuckState-1`：
  - `DuckState==2`（lVar26==1）→ move_ducks **跳过**（`while(lVar26==1)`），由 update_duck_goals/set_duck_velocities 走自由目标；
  - `DuckState==1` → 沿轨迹**正向**（dir=+1），到末端（progress==1）写 **state 2**（转自由游）；
  - `DuckState==3` → 沿轨迹**反向**（dir=-1），到起点（progress==0）→ `despawn`（保留 state 3）；
  - plan_duck 出生即写 progress=1.0、**state 3**。0 与"自由游"语义为推断（标注）。
- **TrajectoryPlan 布局（0x30 槽）**：`+0x00 u64` 分配指针、`+0x08 u64` 点数据指针（move_ducks/plan 热键 B 均读）、
  `+0x10 u64` 点数（stride 0xc）、`+0x18 f32 ×2` 随机圆偏移、`+0x20/+0x24 u32` 记账（`*(+0x24)==0` 表示"未执行可 spawn"）、`+0x28 u32` 尾。
  资源侧 `Vec<TrajectoryPlan>`：cap@+0x10、data@+0x18、len@+0x20。
- **CritterLocomotion 枚举（部分确认，标注 unknown）**：move_critters 反汇编值 2/3/4 与掩码 `99U>>(v&31)&1`（选中 {0,1,5,6}）：
  `==2` → terrain_rock_mask 采样；`==4` → 跳过水面双采样；`==3` → 墙体查询跳过 PublicWalls::get；`∈{0,1,5,6}` → 纯 WallSpatialHash 墙体避让（无 spherecast）。
  各值的物种归属（鸭/羊/蝴蝶/萤火…）未钉。

---

## 7. 剩余 unknown（不伪造）

- Butterfly 状态枚举与两个跳表（0x142f68008 / 0x142f68018）各 handler 具体语义（跳表块未逐块精读）。
- fish frame_update call-site 谓词 `is_corrupted()` 与 (min,max) 档位的设计意图；fish `fVar51` 设置值 switch 的真实来源资源。
- wren_boids 停靠点/FreeSpot/HashMap 落盘细节与 `_DEBUG_HAVEN_BIRD_COUNT` 调试计数。
- sim_wren_boids 后半段（>0x142328250）个别常量归属（0x3f8147ae=1.01 出生爆发偏移已见，其余未逐窗核对）。
- CritterLocomotion 物种映射、DuckState 0。
