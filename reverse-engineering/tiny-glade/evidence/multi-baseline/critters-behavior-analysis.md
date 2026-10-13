# critters 行为状态机核心链分析（B 主基线，反汇编级证据）

2026-10-13。B patched EXE（sha256 cf048e27…6323），RNE 发行原件（sha256 f507a6ce…d03）作机器码 oracle。
10 个函数 `original_equals_patched_range = true`（evidence/parallel-critters/B-boundaries.json），
radare2/RNE 抽点反汇编与 reva/B 逐条一致，地址/指令未被 B patch 改动。

## 证据来源（本次精读）

- **reva**（B 持久库，worker `critter-owner`，只读脚本，programPath=/tiny-glade.exe）：10 个目标函数
  逐函数 反编译 C + 全反汇编（含原始字节、call targets）。结果落盘
  `evidence/toolchain/critters/{fn}.{decompile,disasm}.json` + `.annotated.txt`（PDB 行号表合并）。
  3 个大函数（plan_duck/move_ducks/move_critters）按 0x800 窗口分块重取，避免 65536 字节 stdout 截断。
- **radare2**（RNE 原件 oracle，worker `critter-r2`）：10 个函数入口反汇编 + .data 浮点常量 hexdump
  （`evidence/toolchain/critters-radare.json`、`critters-r2-floats.json`）。
- **PDB 行号表**：skeleton/B/crates/systems/critters/src/{duck,shared}.rs.pdb.jsonl.gz、
  sheep/sheep_behavior.rs.pdb.jsonl.gz、sheep/sheep_terrain.rs.pdb.jsonl.gz。offset→源码行（相对函数入口）。

## 时序 / RNG（横切证据）

- **每帧 delta**：bevy `Time` 资源的 delta_seconds 一律在 `*Res<Time> + 0x28` 读 f32。
  证据：update_duck_goals L592 `fVar4=[*in_R8+0x28]`、update_duck_ripples L735 `[ *in_R9+0x28]`、
  smooth_critter L203 `[*in_RDX+0x28]`、move_critters L223 `[*in_RDX+0x28]`、
  sheep_walk_on_terrain L24 `[RAX+0x28]`（RAX=[RBX]）。
- **RNG**：统一 `perchance::PerchanceContext::uniform_f32`（0x141625ae0），无 fastrand 直接调用；
  RNG 由插件 build 时传入的 Perchance state（见 firefly.rs 记录 0xc0df）承载，函数内不重播种。
  游戏自有 utils：`uniform_sample_disk`（0x140c90700）、`uniform_sample_circle`（0x140c90770）、
  `golden_spiral`（0x140c90940）生成位置/方向采样。
- **平滑因子**：多处 `1 - exp(dt * k)`；sheep_walk_on_terrain 用 k=-3.0（0x142ad58e8=-3.0f），
  update_duck_ripples 用 -2.5/-5.0，move_critters 旋转用 `powf(0.01, dt)`。

---

## 1. duck 五段链

### 1.1 plan_duck_trajectories（规划鸭子轨迹；入口 0x14230ff00，PDB size 8958，源码 duck.rs 89–366）

Ghidra 函数体：`[[0x14230ff00,0x1423100a2][0x1423100b0,0x1423102d0][0x1423102e0,0x142310886]
[0x142310890,0x1423108d1][0x1423108e0,0x142310978][0x142310980,0x142310c04]
[0x142310c10,0x142310c34][0x142310c40,0x142310e73][0x142310e80,0x142310f4c]
[0x142310f50,0x1423111ce][0x1423111d0,0x142311508][0x14231150e,0x14231151d]
[0x142311523,0x142311532][0x142311535,0x142311540][0x142311543,0x14231155a]
[0x14231155d,0x142311568][0x14231156b,0x142311579][0x14231157c,0x14231158a]
[0x14231158d,0x1423115a2]]`。PDB 行号表覆盖 89–366。

业务步骤（反汇编/反编译证据）：
- L106：puffin 作用域；`_DEV_OVERRIDE_IS_DEMO_MODE`（__imp_ 全局）分支；`**(u32**)(arg2+0x10)=*(u32*)(arg2+0x1c)`（L108，资源写）。
- **L128 热键 A**：`developer_hotkeys_enabled()`（0x14090e070，@0x780）&& `ButtonInput::just_pressed<KeyCode>(0x1f)`（0x14261ed20，@0x813）：
  - `CursorTerrainRaycast::position()`（0x140a12650，@0x837）取光标世界位置；
  - 循环 3 次（L133–136）：`uniform_f32()`（@0x1045）+ `utils::golden_spiral()`（0x140c90940，@0x1065）+ `x0y`（@0x1086）→ 3 个候选 Vec3（=光标位 + golden_spiral 偏移）；
  - `from_iter<Trajectory, Map<IntoIter<Vec3>, spawn_ducks::closure_env$1>>`（@0x? 0x1428a57d0 附近）→ 3 点构成一条 Trajectory；
  - `uniform_f32()`+`utils::uniform_sample_circle()`（@0x1239/0x123f）→ 随机圆偏移（Vec4 打包 1.0,0,0）；
  - push 进资源侧 `Vec<Trajectory>`（lVar25+0x10/+0x18/+0x20，stride 0x30）。
- **L153 热键 B**：`developer_hotkeys_enabled()`（@0x1380）&& `just_pressed<KeyCode>(0x27)`（@0x1420）：
  - 遍历已存轨迹；逐点 lerp（L564 附近 `t=uVar22/lVar7`）构点；
  - `RaycastWorld::legacy_raycast()`（0x1408cc680，@0x3077）两点间射线；受阻/距离判断后
    `uniform_f32()`+`uniform_sample_circle()`（@0x3279/0x3287）生成随机落点；
  - 收尾：`AssetMeshLibrary::get_handle_by_name`（0x14092d4f0）+`AssetShaderLibrary::get_raster_handle_by_name`（0x140937aa0）取 "duck_shader"；
  - `Commands::spawn<(Duck, DuckState, DrawableMeshBundle, CritterLocomotion, CritterGoal, CritterVelocityTarget, ObjectGrassPush, Velocity, CritterVelocityMultiplier, PrevTransform)>`（0x141dc15a0，@0x4688）批量生成鸭子；
  - `retain<TrajectoryPlan, closure_env$7>`（0x141dbd970）修剪已执行计划。
- 结构（证据）：`Trajectory`（0x30 槽：Vec<Vec3> 点 ptr/len + 4 f32 随机偏移 @+0x10 段）；
  `TrajectoryPlan`（drop_in_place@0x141d82af0，含点 Vec）。落点侧 lVar25（资源）内 `Vec<Trajectory>` 布局 +0x10 指针/+0x18 数据/+0x20 len。
- 调用者/消费：plan 生成的轨迹点最终写入被 spawn 的 Duck 组件（见 1.2 +0x98/+0xa0）；move_ducks 消费。
- 依赖复用：bevy `ButtonInput`/`Commands`、puffin 作用域、perchance RNG 直接复用。

### 1.2 move_ducks_along_trajectories（沿轨迹移动鸭子；入口 0x142312200，PDB size 6447，源码 369–576）

函数体：`[[0x142312200,0x142312392][0x1423123a0,0x1423126b0][0x1423126c0,0x14231298e]
[0x142312990,0x142313813][0x142313816,0x142313828][0x14231382b,0x142313836]
[0x142313839,0x142313850][0x142313853,0x14231385e][0x142313861,0x14231386f]
[0x142313872,0x142313889]]`。

查询签名（硬证据，init@0x14230eb00）：
`Query<(Entity, &mut Duck, &mut DuckState, &mut Transform, &mut Velocity, &mut CritterVelocityMultiplier, &mut CritterGoal, &mut ObjectGrassPush)>`。

Duck 组件字段（证据）：
- `+0x98`：Vec<Vec3> 轨迹点数据指针（stride 0xc）；`+0xa0`：点数 u64（越界 panic_bounds_check）。
- `+0xec`：f32 沿轨迹进度。
- `+0xe0`：u16=1（状态写）；`+0xe4`：8B 两 f32（2.0,4.0）。

业务步骤：
- L392/L419：到达末端分支 `EntityCommands::despawn()`（0x140d14770，@0x1275/@0x2907）；
  受 `_DEV_OVERRIDE_IS_DEMO_MODE` 全局门控（L364 `==0 → break`），正常发行默认不直接 despawn，走事件路径。
- L455：`utils::curve::CurveInternalCoord::new_for_point_count<CurveCoordSemantic3D>`（0x14261ef90，@0x3537）；
  L457 `truncf`（0x142924060，@0x3513）→ 轨迹段索引。
- 进度推进（L431–435 附近）：`progress += (dir * 12.0 * dt) / segment_len`（`__real_41400000`=12.0）；
  dir=±1（L386–389 依 lVar26∈{0,2} 选 1.0/-1.0）。
- 切向/法向 Frenet 帧数学（L464–539）→ 沿曲线姿态。
- L495 `utils::inverse_lerp`（0x140c904a0，@0x4469）。
- **音频**（country_core audio_resources，L503–536）：`start_if_not_playing`（0x140a57300）、
  `set_position`（0x140a57da0）、`set_volume`（0x140a575a0）、`AudioLoop::current`（0x140a593f0）、
  `set_parameter`（0x140a57670）、`stop`（0x140a59430）、`runtime`（0x140d8b480，L448）；
  水面判定用 `bilinear_sample_at_world_xz<WaterAudioGrid>`（0x14261f0c0，@0x4697/@0x4816）——与 1.3 的
  地形 RasterBufferRef<512> 是不同网格。
- 状态迁移：写 DuckState 2/3（`*local_100`），L418 `*puVar36=1`（Velocity？），
  L422 `*(u16*)(lVar27+0xe0)=1`、L423 `*(u64*)(lVar27+0xe4)=CONCAT44(2.0f,4.0f)`。

### 1.3 update_duck_goals（鸭子目标选择；入口 0x142313b30，PDB size 2364，源码 581–695）

函数体：`[[0x142313b30,0x142313ef8][0x142313f00,0x14231446b]]`。

参数：RCX=Query（Duck+Transform+CritterGoal 等，stride 0x100/0x30/0x10），RDX=Res，R8=Time，R9=WaterRaster(Res)。
prologue：`as_inner_mut`（@0x127）+ 双缓冲水格选择（[RAX+0x298] 字节标志，@0x140–0x177，同 tree-cmds WaterRaster 模式）→ `as_float_grid`（@0x196）→ 采样上下文；`fVar4=[*in_R8+0x28]`=dt（@0x201）。

Duck 组件字段（证据）：`+0xe0` 字节（在移动/静止标志）、`+0xe1` 字节、`+0xe4` 8B 两 f32（目标计时）、`+0xe8` f32。
CritterGoal（0x10 槽）：`active u32 @+0`、`pos x@+4 / y@+8 / z@+0xc`。

业务步骤（分支偏移/VA）：
- L604–627（@0x218 起）：`Duck+0xe0` 分支：
  - **e0==0（无活动目标）**（LAB_142313f00 @0x5d0）：`+0xe4.lo -= dt`；`<=0` → 2×`uniform_f32`（@0x631/0x63c，L133/135）
    → `+0xe4=(u1*3.0+1.0, u2*3.0+1.0)`（范围[1,4]），置 e0=1、e1=0（LAB_142314000 @0xa20）。
  - **e0!=0**：`+0xe4` 两半均 `-= dt`；两半 `<=0` → `uniform_f32`（@0x7ce，L685）→ `+0xe4=(u*0.5+0.5, …)`，置 e0=0；否则（LAB_142313f7e @0x8de）：
    - CritterGoal.active bit0 且 `dist2(pos,goal) >= 0.16`（`__real_3e23d70b`）→ 保留目标继续；
    - 否则选新目标（@0x9a8 起）：置 `Duck+0xe8=0.1f`；2×`uniform_f32`+`uniform_sample_disk`（0x140c90700，@0xa5f）+`x0y`（@0xa68）+`x0z`（@0xba6）→ 盘面采样；
      新位 = 当前位 + 采样*0.8（CritterGoal.active 原非 0 时）的 3 轴组合；
      校验链（@0xc80–0xd50）：`is_within_glade_shape()`（0x14090dfe0，@0xc83，L667）+ 距原点半径
      `r(new)<=r(pos)`（向内偏好，L668–669）+ `bilinear_sample_at_world_xz<RasterBufferRef<512>>`（0x140a14df0，@0xc96，L669）采样 `<=0.5` → 无效目标（active=0，@0xd8a）；否则 4×bilinear 采样（@0xcd5/0xce0/0xceb/0xcf6，L672）求和 clamp[0,1] `>=0.1` → 无效；否则写
      `CritterGoal={active=1, pos=new}`（@0xd90–0xdac）。
- 结尾 L695：`*(u8*)(Duck+0xe1)=0`，循环。

调用者：set_duck_velocities_towards_goals 消费 CritterGoal/Duck 状态。
依赖：perchance RNG、utils uniform_sample_disk/swizzler、country_core game_config::is_within_glade_shape。

### 1.4 set_duck_velocities_towards_goals（目标→速度；入口 0x142314470，PDB size 1786，源码 698–728）

函数体：`[[0x142314470,0x14231464e][0x142314650,0x142314822][0x142314830,0x142314b69]]`。

查询签名：`Query<(Transform, &Duck, &DuckState, &CritterGoal, &mut CritterVelocityTarget, &mut CritterVelocityMultiplier)>`（QueryIterationCursor::init@0x14230ecf0 前序，@0x4ca）。
过滤：仅处理 `DuckState == 2`（L708 循环 `!=2` 跳过）。

组件（证据）：Transform 位 @stride0x30+0x10/0x14/0x18；Duck @stride0x100（+0xe0 字节、+0xfc 字节）；
CritterGoal @0x10（active@+0, pos@+4..+0xc）；CritterVelocityTarget @0x10（dir.x@+0, y@+4, z@+8）；
CritterVelocityMultiplier @0x4。

业务步骤（L708–725）：
- `CritterGoal.active bit0` 且 `Duck+0xe0==1`（@0x?? L713 分支）→ 目标方向 = normalize(goal - pos)（用 `clone_behavior` 门控）；
  否则方向取零（保留位）。
- 写 VelocityTarget = 归一化方向 3 分量（@0x14xx）；写 Multiplier 等（`*puVar2=local_d4`、`local_198+…=local_94`、`local_b0+…=1.0`）。
- `Duck+0xfc==1`（@0x?? L721）→ VelocityTarget 清零（冻结）。
- 注意：prologue `MOVSS XMM6,[0x142925904]`=1.0f（@0x273）。

调用者：smooth_critter_velocities（消费 VelocityTarget/Multiplier）。

### 1.5 update_duck_ripples（鸭子水波 SSBO；入口 0x142314b70，PDB size 1235，源码 735–796）

函数体：`[[0x142314b70,0x142314fd2][0x142314fd5,0x142314fe0]]`。

GPU 胶水（与移动链无业务耦合，归组 gpu_glue）：
- L735 `dt=[*in_R9+0x28]`；L746/L747 2×`expf` → `retention1=1-exp(-2.5*dt)`、`retention2=1-exp(-5.0*dt)`（波纹衰减）。
- L749 `from_iter<Vec<u32>, FilterMap<QueryIter<(Entity,Transform,DuckState)>, closure_env_0>>`（0x141dbf8c0）收集水面鸭子（closure 过滤）。
- L784–786 `FrozenMap::get<SsboId,Box<BufferState>>`（0x14230e780）+ memcmp 校验 buffer 状态；
  组 0x30 stride 记录 → `rhapsody::raw::device::GraphicsDevice::update_buffer_bytes()`（@0x…）上传 SSBO（鸭子波纹顶点）。

---

## 2. shared 移动 / 避让

### 2.1 move_critters（通用移动；入口 0x142318fa0，PDB size 5774，源码 shared.rs 214–374）

函数体：`[[0x142318fa0,0x142319132][0x142319140,0x1423195b4][0x1423195c0,0x142319caa]
[0x142319cb0,0x14231a36e][0x14231a371,0x14231a37c][0x14231a37f,0x14231a38a]]`。

查询签名（init@0x14230ecf0，硬证据）：
`Query<(&mut Transform, &CritterLocomotion, &Velocity, Option<&CritterVelocityMultiplier>, Option<&Duck>, Option<&Sheep>, Option<&mut BirdSpots>)>`。

组件（证据）：Transform{rotation Quat@+0, translation Vec3@+0x10}（x@+0x10,y@+0x14,z@+0x18）；Velocity{x@+0,y@+4,z@+8}（12B）；CritterLocomotion 8B 枚举（`*puVar29` 值 2/4/6/7 与 `99U>>(v&31)&1` 位测）；Multiplier 4B Option；Duck Option（写 u32 `*local_1a8=local_13c`）；Sheep Option（写 `*local_1d8=local_194`）；BirdSpots Option（stride 0x18，`local_160`）。

业务步骤：
- L223 `dt=[*in_RDX+0x28]`；`fVar31=powf(0.01, dt)`（@0x??，旋转衰减）。
- L234 起每只：`translation.y = max(y, 0)`（下限夹取，@0x? 前序）；位移 = `velocity * dt * multiplier`（Multiplier 缺省 1.0，L527–532：`local_130=v.z*dt*mult`、`local_138=(v.y*dt*mult, v.x*dt*mult)`）。
- 避让门控（L244–315）：
  - `bilinear_sample_at_world_xz<RasterBufferRef<512>>`（@0x? L238）水面/地形采样 + `terrain_rock_mask::sample_at_world_xz`（0x1409ea800）岩石掩码；
  - `RaycastWorld::legacy_spherecast()`（0x1408ccc90，@0x… L257）→ `local_318!=2` 且球距 `<1.0` → 跳过移动（LAB_142319506）；
  - 水面 2×bilinear 采样（L264，`fc&1` 且 locomotion!=4）→ 判水。
- 应用位移（L276 附近）：`translation += displacement`。
- 朝向（L291 起）：`Quat::from_rotation_arc()`（0x140d4ae30）+ 旋转混合 `rot = rot*fVar31 + target*(1-fVar31)`（归一化 divps/sqrtps）。
- BirdSpots（L318，wren 停靠点）：读取 Transform 后更新停靠点位置（stride 0x18 记录）。
- 边界：Duck/Sheep/BirdSpots 为 Option，本函数不构造；鸭子/羊的专属逻辑见 duck/sheep 模块。

### 2.2 critters_avoid_collisions（避让；入口 0x1423179a0，PDB size 4706，源码 27–196）

函数体：`[[0x1423179a0,0x142317b32][0x142317b40,0x142318942][0x142318945,0x142318950][0x142318953,0x14231895e]]`。

查询（普通 QueryIter，两形状 cVar6=`query+0x108` 分支，@0x…）：逐实体取
Transform（stride0x30, pos@+0x10/+0x14/+0x18）、CritterGoal（stride0x10, active@+0, x@+4,y@+8,z@+0xc）、
4B 记录（`*puVar2=local_100` 查询环境 u32）、8B 记录（`*local_108` 命中码，值 2/3/7）、可选 0x68 记录（presence 门控）。
循环仅处理 `CritterGoal.active == 1`（`while(*piVar17 != 1)`）。

业务步骤：
- L51：资源/SSBO 准备（`FrozenMap::get`、`as_inner_mut` 水格双缓冲、`as_float_grid`）。
- L73–84：dir = normalize(goal - pos)（x/z 2D）；`Circle2d::new_rotated`（0x140c97560）+ `LineInterval::line_segment`（0x14261f450）构移动段；
  `bilinear_sample_at_world_xz`（@0x2610）+ `terrain_rock_mask::sample_at_world_xz`（@0x2652）。
- L84 起避让分支：
  - **墙体查询**（L154–179，建筑查询衔接）：`WallSpatialHash::query`（@0x3462）/`PublicWalls::get`（0x140aa5730，@0x3489）/
    `PublicWallState::max_y_gable`（0x140a91ef0，@0x3534，`max_y_gable < pos.y` 跳过）/`Circle2d::new_rotated`（0x140b2a680，@0x3567）/
    `LineInterval::line_segment+relate`（@0x3614/@0x3650）；墙体记录 stride 0x38。
  - 水面 8×bilinear 采样（L99–103，求和 clamp）判水。
- L132–135：`RaycastWorld::legacy_spherecast()`（0x1408ccc90，@0x3318）+ `x0y`（@0x3172）；
  球距 `<1.0` → 保持当前目标继续避让。
- L179：`EventWriter::write<DespawnDuckCmd>`（@0x3718 附近）——球距 `<0.1` 且命中标志 → 发事件撤销鸭子。
- 结尾（@0x…）：`*puVar2=local_100`、`*piVar17=0`（清 CritterGoal.active）。

结构（证据）：墙体记录 0x38 stride；WallSpatialHash 经 `query` 返回邻近记录游标。
依赖：country_core walls/public_walls、wall_state/wall_spatial_hash、collision/world RaycastWorld、terrain_rock_mask。

### 2.3 smooth_critter_velocities（速度平滑；入口 0x142318c10，PDB size 898，源码 199–210）

函数体：`[[0x142318c10,0x142318dae][0x142318db0,0x142318f91]]`。

查询：(CritterVelocityTarget, Velocity, CritterVelocityMultiplier)。
业务（L206）：
- `retention = powf(target.retention_per_second, dt)`（0x142923f70，@0x266）；
- `velocity = velocity*retention + target.dir*(1-retention)`（3 轴，8B 打包 + z）；`multiplier = query 环境 u32`。
结构（证据）：VelocityTarget{dir.x@+0,y@+4,z@+8, retention@+0xc}；Velocity{x@+0,y@+4,z@+8}。
**与 firefly.rs 草稿 `smooth_velocity` 逐字段一致**（retention@+0xc、retention 幂、加权顺序），草稿可复用。

---

## 3. sheep_terrain::sheep_walk_on_terrain（羊地形行走；入口 0x14231bd80，PDB size 1556，源码 sheep_terrain.rs 15–80）

函数体：`[[0x14231bd80,0x14231bff0][0x14231c000,0x14231c18d][0x14231c30f,0x14231c37a]]`。

参数：RCX=Query，RDX=Res，R8=Time（RBX），R9=WaterRaster（as_inner_mut @0x119 + [RAX+0x298] 双缓冲）。
查询列：Transform（stride0x30, pos@+0x10/+0x14/+0x18）、8B 状态（state u32@+0, f32@+4）、
3×4B 记录（写 `*puVarX = 查询环境 u32`）、12B 记录（竖向速度/差 @+4）。

业务步骤：
- L24 `dt=[*in_R8+0x28]`；`smooth = 1 - exp(-3.0 * dt)`（0x142ad58e8=-3.0f，MULSS@0x193 + expf@0x201）。
- L27（每实体）`TerrainHeightsData::sample_at_world_xz<TerrainHeightSsbo>`（0x141246fb0，@0x989）→ `h`（地形高度，**建筑/地形查询衔接点**）。
- `switch(state@+0)`（@0x1030 起）：
  - **2**（地面）：`pos.y = h`（吸附）。
  - **3**：panic（不可达）。
  - **4**（过渡）：`utils::time_continuous_lerp`（@0x?）→ `pos.y=lerp`；`bilinear_sample_at_world_xz<RasterBufferRef<512>>`（水格）；`采样 <= 0` → state=2。
  - **default**（行走，state 0/1/5）：`dy = (h + 4.0) - pos.y`（+4.0 身位偏移）；
    - 地形低于羊（`h <= pos.y`）：dy clamp[-2,2]；`v += (dy*10 - v)*smooth`（弹簧阻尼，增益 10）。
    - 地形高于羊：`pos.y=h`（瞬移上）；dy clamp[-5,5]；`v += (dy*10 - v)*smooth*3.0`（×3 快速修正）。
    - 写 `v@12B+4 += …`。
  - **6**（浮空/漂浮）：dy = h - pos.y，clamp[-0.3,0.3]；`v += (dy*20 - v)*smooth`（增益 20）；`dy > -0.05` → state=2、v=0。
- 常量（RNE hexdump 证实）：0x142925904=1.0f、0x142ad58e8=-3.0f、0x142a7b658=130.0f（采样参数）、4.0/10.0/20.0/0.3/-0.3/-0.05。
- 结构（证据）：羊地形状态 8B{state u32@+0, f32@+4}；竖向速度 12B 记录 f32@+4（其余未用，unknown）。

调用者/消费：无后续（就地吸附羊高度）；与 building/terrain 查询仅经 TerrainHeightSsbo 采样 + WaterRaster 水格。

---

## 4.（附）update_sheep_goals（羊目标状态机；入口 0x142315050，PDB size 3558，源码 sheep_behavior.rs 26–221）

函数体：`[[0x142315050,0x142315334][0x142315340,0x142315804][0x142315810,0x142315e35]]`。

查询：(Transform(stride0x30), Sheep(stride0x68), CritterGoal(stride0x10), 2×4B 记录)。
Sheep 组件字段（证据）：`+8` u32 行为状态（==1 跟随）；`+0xc` f32 目标计时；`+0x30` 字节；`+0x34` 8B 两 f32 计时。

业务步骤：
- L51/L52 资源 deref（as_inner_mut ×2）。
- **state==1（跟随）**（L60–81）：`MainCamera::mode()`（0x140911230，@0x1454）；mode==3 → `eye_position`（0x140911dd0，@0x1471）+`eye_forward`（0x140911df0，@0x1487）→ 目标=eye+forward*2；否则 `CursorTerrainRaycast::position`（@0x1566/0x1658）。写 `CritterGoal={active=1,pos}`。
  - `timer@+0xc -= dt`（@0x?）；距目标 `<= fVar13` → `timer -= 3*dt`；`timer>=0` → 2×`uniform_f32`（@0x1816/0x1853）→ 随机；`timer<0` → state=0 + 1×`uniform_f32`（@0x1874）。
- **共享目标刷新**（L125–148）：`Sheep+0x30` 字节 → `+0x34` 两半 -= dt；全 >0 → CritterGoal active 且 `dist2>=0.16` → 保留；否则 2×`uniform_f32`+`uniform_sample_disk`（@0x2292）+`x0y`/`x0z` → 新目标；
  校验（L168/177/185）`bilinear_sample_at_world_xz<RasterBufferRef<512>>` 水面 → 无效则回退。
- RNG：perchance uniform_f32 ×多、uniform_sample_disk；`__real_3e23d70b`=0.16 距离阈值。

---

## 5. 状态结构字段布局汇总（证据）

| 组件 | stride | 字段 |
|---|---|---|
| CritterGoal | 0x10 | active u32@+0；pos Vec3 @+4 (x),@+8 (y),@+0xc (z) |
| CritterVelocityTarget | 0x10 | dir Vec3 @+0..+8；retention_per_second f32 @+0xc |
| Velocity | 0xc | x@+0,y@+4,z@+8 |
| CritterVelocityMultiplier | 0x4 | f32/u32 @+0（各系统写查询环境值；语义 unknown） |
| Duck | 0x100 | +0xe0 字节(目标活动标志), +0xe1 字节, +0xe4 两 f32(计时), +0xe8 f32, +0xfc 字节(冻结), +0x98 Vec<Vec3> 轨迹点, +0xa0 点数, +0xec 进度 |
| DuckState | 1B 枚举 | 过滤值 2（move/set_vel 用），写 2/3 |
| CritterLocomotion | 0x8 | 枚举 u32@+0（值 2/3/4/6/7 位测） |
| Transform | 0x30 | rotation Quat@+0x0(16B)；translation @+0x10 (x,y,z) |
| 羊地形状态 | 0x8 | state u32@+0；f32@+4 |
| 羊竖向速度记录 | 0xc | f32@+4（其余未用 unknown） |
| Sheep | 0x68 | +8 行为状态 u32, +0xc 计时, +0x30 字节, +0x34 两 f32 |
| 墙体记录（避让） | 0x38 | 逐字段未读全（unknown） |

## 6. 剩余 unknown（不伪造）

- 查询闭包环境 u32 @Query+0x14（写 Multiplier/4B 记录的值）：常量 1.0 在 sheep_walk 场景确认（0x142925904），
  但 duck/shared 各函数写的是 `Query+0x14` 载入值，来源未钉（疑似插件 build 捕获的 bob/阶段常量）。
- `Trajectory` / `TrajectoryPlan` 完整布局（0x30 槽仅确认部分字段；TrajectoryPlan drop_in_place 存在）。
- plan_duck L170（offset 5396）行号非单调：内联 spawn_ducks 闭包的行号回跳，具体归属未解析。
- 热键 0x1f/0x27 的 KeyCode 语义（dev 门控），`_DEV_OVERRIDE_IS_DEMO_MODE` 在发行构建的实际值。
- critters_avoid_collisions 的 8B 命中码记录与 move_critters 的 CritterLocomotion 是否为同一组件未交叉验证。
- duck_splash_particle / spawn_ducks::DespawnDuckCmd 的完整事件消费链（发事件方已确认，消费方待核）。
- sheep_behavior 其余系统（set_sheep_velocities 224–245、update_sheep_audio 248–253）与 sheep_unstuck/sheep_floatie 未在本轮精读。
