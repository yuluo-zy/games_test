# systems/critters 模块内部实现分析（B 基线）

> 由批次 2 分析 agent 产出，父级代为落盘。

- 模块：`crates/systems/critters`（package `system-critters` 0.1.0）
- 二进制：B patched EXE（sha256 cf048e27…6323），程序 `tiny-glade.exe`；机器码 oracle 为发行 RNE（f507a6ce…d03）。
- 证据来源：PDB 骨架 `skeleton/B/crates/systems/critters/src/*.rs`；既有反编译 `evidence/parallel-critters/B-decompile/*.c`（VA 命名）+ report.json；精读 `evidence/multi-baseline/critters-behavior-analysis.md`、`critters-extra-analysis.md`（reva worker critter-owner/critter-extra-owner + radare2 RNE 常量 hexdump）；候选实现 `reconstruction/critters/`。
- 标注：O=观察，I=推断，U=未知。仅模块内部实现；依赖（std/glam/fastrand/perchance/dolly/ron/itertools/lerp）直接复用不逆向。

## 横切事实
- dt：bevy `Time` delta_seconds 一律读 `*Res<Time>+0x28`（f32）。(O)
- RNG 二分：目标系（duck/fish/butterfly/firefly/sheep goal）用 `perchance::PerchanceContext::uniform_f32`(0x141625ae0)；boids 转向与出生位置用 `fastrand::Rng::f32`。(O)
- 游戏自有 utils：uniform_sample_sphere(0x140c907d0,内部 cosf/sinf)、uniform_sample_circle(0x140c90770)、uniform_sample_disk(0x140c90700)、golden_spiral(0x140c90940)、uniform_sample_cone、swizzlers::x0y/x0z/xvy、float_grid::bilinear_sample_at_world_xz<RasterBufferRef<512>>(0x140a14df0)、time_continuous_lerp(_vec3)、easing_functions::EasingFunction::apply。(O)
- 共享组件布局（多系统一致，硬证据）：Transform stride0x30(rot Quat@+0, translation x@+0x10/y@+0x14/z@+0x18)；Velocity 12B(x@+0,y@+4,z@+8)；CritterGoal 0x10(active u32@+0 bit0, pos x@+4/y@+8/z@+0xc)；CritterVelocityTarget 0x10(dir Vec3@+0..8, retention_per_second f32@+0xc)；CritterVelocityMultiplier 4B@+0（各系统写“查询环境”Query+0x14，语义未钉）；CritterLocomotion 8B(u32@+0，值 2/3/4/6/7；位测 99U>>(v&31)&1 选中 {0,1,5,6})。(O)

---

# src/boids.rs

## Boid::random_vel_change
- 地址 0x141dc2170（RVA 0x1dc2170），源码行 389。
- 内部逻辑 (O)：`timer = *(f32*)(self+0x30) - dt`；`timer<=0` → 2×`fastrand::Rng::f32()` + `utils::uniform_sample_cone()` 取新锥向写 self+0x24(8B dir.x/y)/+0x2c(dir.z)；`fastrand::Rng::f32()`+`utils::lerp()` 重设 timer(self+0x30)。随后 `utils::time_continuous_lerp_vec3()` 插值方向，`glam::Quat::from_rotation_arc()` 生成旋转，把速度(self+0xc 的 Vec3)用四元数旋转写回（手写 m128）。
- 算法：随机转向 + 球面锥采样；四元数 arc 旋转；时间连续 lerp。
- 数据结构/常量：Boid 0x18{pos@0..8, vel@+0xc..0x14, timer@+0x30, dir 打包@+0x24(8B)/+0x2c}；dt 由 frame_update 传入。
- 未知 (U)：cone 采样参数（角/朝向）来自调用点；time_continuous_lerp_vec3 时长参数来源。

## Boid::frame_update<fish closure_env$0> / <wren closure_env$3>（boids 核心，非清单但为 fish/wren 提供算法）
- 地址 fish 0x141dc2740（RVA 0x1dc2740）/ wren 0x141dc3510（RVA 0x1dc3510），源码行 329–387。
- 内部逻辑 (O)：先调 random_vel_change()；邻居循环（6 f32/条=pos+vel）：`dist²<16.0` 且 `self.vel·nvel>=0`（同向门）才计入；分离 `sep -= normalize(n-self)*smoothstep(1-dist/sep_r)`（smooth=(4-2t)t²）；对齐 `vel=mean_nvel*align_w+vel*(1-align_w)`；凝聚 `vel += normalize(mean_pos-self)*|vel|*0.0056`；合并 `vel += sep*1.7`；加速度 delta clamp[-max,max/2]（max=1.0，有目标 1.5）；`|vel|` clamp[min,max]；NaN→归零；`pos+=vel*dt`。fish 对水格 bilinear>=0.5 判水，非水 `pos-=vel*dt; vel=-vel*0.9`；wren 用 closure_3(RAY_DIRS 32 向 raycast) 判可行点，不可行 from_rotation_arc 转向（含 -5.0）。
- 算法：Reynolds Boids 三规则 + smoothstep 分离核 + 速度约束 + 地面反射/射线可行性。
- 常量：邻居半径 4.0(0x41800000 dist²16)、heading 门 -0.5(0xbf000000)、分离 1.7(0x3fd9999a)、凝聚 0.0056(0x3ba3d70a)、加速度 3.0(0x40400000)、6.0/-5.0(0x40c00000/0xc0a00000)、有目标 1.5(0x3fc00000)。call-site：fish sep_r=0.8(0x3f4ccccd)、align_w=0.0584(0x3d6f9db2)、min/max=0.9/4.0 或 3.0/8.0（is_corrupted 谓词选）；wren sep_r=1.3(0x3fa66666)、align_w=0.1169(0x3def9db2)、min/max=7.0/14.0。RAY_DIRS=0x200B 32×Vec3，DAT_14363aad0 futex Once 惰性初始化。
- 未知 (U)：fish 速度档为何由 is_corrupted 选；CritterLocomotion 与两实例对应。

## BoidFlock::remove<fish closure_env$1> / <wren closure_env$6>
- 地址 fish 0x141dc2340（RVA 0x1dc2340）/ wren 0x141dc2540（RVA 0x1dc2540），源码行 114–124。
- 内部逻辑 (O)：对索引 Vec<u32> **倒序**遍历：校验索引<len，对 boid 0x18 槽 swap_remove，并对 0x38 副槽同样 swap_remove（6×8B 字段）；`Entities::contains()` 存活则 `EntityCommands::despawn()`。
- 算法：swap_remove O(1) 删除 + 倒序批处理。
- 数据结构/常量：boid stride 0x18；副槽 stride 0x38；索引 Vec<u32>。BoidFlock{+0x8 boid data,+0x10 len,+0x20 aux data,+0x28 aux len}。
- 未知 (U)：0x38 副槽完整字段语义。

---

# src/shared.rs

## critters_avoid_collisions
- 地址 0x1423179a0（RVA 0x23179a0），源码行 27–196。
- 内部逻辑 (O)：puffin；水格双缓冲([RAX+0x298] 字节选)→PathRastersResources::as_float_grid；SSBO FrozenMap::get+memcmp。逐实体（普通 QueryIter，两形状由 query+0x108 分支）仅处理 `CritterGoal.active==1`；`dir=normalize(goal-pos)`(x/z 2D)；Circle2d::new_rotated + LineInterval::line_segment 构移动段；bilinear_sample_at_world_xz + terrain_rock_mask::sample_at_world_xz。墙体：WallSpatialHash::query→PublicWalls::get→PublicWallState::max_y_gable(max_y_gable<pos.y 跳过)→Circle2d::new_rotated+LineInterval::relate 判交；水 8×bilinear 求和 clamp。RaycastWorld::legacy_spherecast：命中或球距>=1.0→保持目标；球距<0.1 且命中标志→EventWriter::write<DespawnDuckCmd>。收尾 *goal.active=0。
- 算法：线段相交(LineInterval::relate)、圆-线段关系、spherecast、栅格双线性、空间哈希。
- 常量：1.0(0x3f800000)、0.1(0x3ecccccd)；墙体记录 stride0x38；4B 记录（写查询环境 u32）；8B 命中码 2/3/7；可选 0x68 记录 presence 门控。
- 未知 (U)：CritterLocomotion 各值（2=岩石采样、4=跳过水面、3=跳过 PublicWalls::get、{0,1,5,6}=纯墙体避让）与物种映射；8B 命中码语义；0x68 记录语义。

## smooth_critter_velocities
- 地址 0x142318c10（RVA 0x2318c10），源码行 199–210。
- 内部逻辑 (O)：查询(CritterVelocityTarget,Velocity,CritterVelocityMultiplier)。dt=*Res<Time>+0x28；`retention=powf(target.retention_per_second,dt)`(powf@0x142923f70)；`velocity=velocity*retention+target.dir*(1-retention)`（3 轴，8B 打包 x/y + z，原顺序非 lerp 重排）；写 Multiplier=查询环境 u32。
- 算法：指数衰减速度平滑（一阶低通/EMA，per-second 保留率幂）；与 firefly.rs 草稿 smooth_velocity 逐字段一致。
- 常量：VelocityTarget{dir@+0..8,retention@+0xc}；Velocity 12B。
- 未知 (U)：Multiplier 写入值(Query+0x14)语义。
- **既有 firefly 差分唯一失败项即此函数**（B-firefly-verification.json case 1578, kind=smooth）：原码 `ffc00000 ffc00000 7f800000`(负 NaN) vs Rust `7fc00000 7fc00000 7f800000`(正 NaN)——NaN 符号位差异。RNG 直方图确认 goal 更新 0 或 3 次 uniform_f32。(O)

## move_critters
- 地址 0x142318fa0（RVA 0x2318fa0），源码行 214–374。
- 内部逻辑 (O)：puffin；水格双缓冲+as_float_grid；SSBO 校验。查询(&mut Transform,&CritterLocomotion,&Velocity,Option<&CritterVelocityMultiplier>,Option<&Duck>,Option<&Sheep>,Option<&mut BirdSpots>)。dt=*Res<Time>+0x28；`decay=powf(0.01,dt)`(0x3c23d70a)，weight=1-decay。每实体：translation.y=max(y,0)；位移=velocity*dt*multiplier（缺省 1.0）；避让门控（bilinear_sample+terrain_rock_mask、legacy_spherecast 球距<1.0 跳过、水 2×bilinear、WallSpatialHash::query+PublicWalls::get+max_y_gable+LineInterval::relate）；应用位移；朝向 Quat::from_rotation_arc(dir,up) 与旧旋转按 weight 混合（nlerp+归一化，SIMD divps/sqrtps）；BirdSpots(stride0x18) 写入停靠点变换。
- 算法：速度积分（dt 欧拉步进）、四元数 nlerp 朝向平滑、球投/墙几何避让、停靠点矩阵更新。
- 常量：0.01(0x3c23d70a)、1.0(0x3f800000)；Duck/Sheep Option 分别写 *local_1a8/*local_1d8；BirdSpots stride0x18。
- 未知 (U)：CritterLocomotion 各值分支；Multiplier 语义；BirdSpots 内部布局。

---

# src/duck.rs

## plan_duck_trajectories
- 地址 0x14230ff00（RVA 0x230ff00），源码行 89–366。
- 内部逻辑 (O)：受 _DEV_OVERRIDE_IS_DEMO_MODE 门控。资源写 `**(u32**)(arg2+0x10)=*(u32*)(arg2+0x1c)`。热键 A(KeyCode 0x1f)：CursorTerrainRaycast::position 取光标世界位；循环 3 次 uniform_f32+golden_spiral+x0y→3 候选 Vec3；in_place_collect::from_iter<Trajectory,Map<IntoIter<Vec3>,spawn_ducks::closure_env$1>>→一条 Trajectory；uniform_f32+uniform_sample_circle 随机圆偏移；push 资源 Vec<Trajectory>。热键 B(KeyCode 0x27)：遍历轨迹逐点 lerp；RaycastWorld::legacy_raycast 两点射线，受阻/距离判断后 uniform_f32+uniform_sample_circle 生成落点；AssetMeshLibrary::get_handle_by_name+AssetShaderLibrary::get_raster_handle_by_name 取 "duck_shader"；Commands::spawn<(Duck,DuckState,DrawableMeshBundle,CritterLocomotion,CritterGoal,CritterVelocityTarget,ObjectGrassPush,Velocity,CritterVelocityMultiplier,PrevTransform)> 批量生成鸭子；retain<TrajectoryPlan,closure_env$7> 修剪已执行计划。
- 算法：黄金螺旋散点、射线寻路、轨迹点集构建、计划生命周期修剪。
- 数据结构/常量：Trajectory 0x30 槽(Vec<Vec3> 点 ptr/len + 4 f32 随机偏移@+0x10)；TrajectoryPlan(drop_in_place@0x141d82af0)；资源 Vec<Trajectory>{+0x10 指针,+0x18 数据,+0x20 len}；KeyCode 0x1f/0x27=dev 热键。
- 未知 (U)：热键语义；Trajectory/TrajectoryPlan 完整布局；_DEV_OVERRIDE_IS_DEMO_MODE 发行值。

## move_ducks_along_trajectories
- 地址 0x142312200（RVA 0x2312200），源码行 369–576。
- 内部逻辑 (O)：查询(Entity,&mut Duck,&mut DuckState,&mut Transform,&mut Velocity,&mut CritterVelocityMultiplier,&mut CritterGoal,&mut ObjectGrassPush)。到达末端 EntityCommands::despawn()（demo 门控）。utils::curve::CurveInternalCoord::new_for_point_count<CurveCoordSemantic3D>+truncf→段索引。进度推进 `progress += (dir*12.0*dt)/segment_len`(12.0=0x41400000；dir=±1 依 lVar26∈{0,2})。Frenet 帧沿曲线算姿态；utils::inverse_lerp。音频：start_if_not_playing/set_position/set_volume/AudioLoop::current/set_parameter/stop，水面判定用 bilinear_sample_at_world_xz<WaterAudioGrid>。状态迁移：写 DuckState 2/3、Duck+0xe0=1、Duck+0xe4=CONCAT44(2.0,4.0)。
- 算法：曲线参数化（段索引+Frenet 帧姿态）、弧长进度推进、逆 lerp。
- 常量：12.0(0x41400000)；Duck+0x98 Vec<Vec3>(stride0xc)、+0xa0 点数、+0xec 进度、+0xe0 状态字节、+0xe4 8B(2.0,4.0)；DuckState 1/2/3。
- 未知 (U)：DuckState 0 语义；ObjectGrassPush 字段；CurveCoordSemantic3D 参数化。

## update_duck_goals
- 地址 0x142313b30（RVA 0x2313b30），源码行 581–695。
- 内部逻辑 (O)：水格双缓冲+as_float_grid；dt=*Res<Time>+0x28。Duck+0xe0 分支：e0==0：+0xe4.lo-=dt；<=0→2×uniform_f32→+0xe4=(u1*3+1,u2*3+1) 区间[1,4]，置 e0=1,e1=0。e0!=0：+0xe4 两半-=dt；两半<=0→uniform_f32→+0xe4=(u*0.5+0.5,…)，置 e0=0；否则 CritterGoal.active bit0 且 dist²>=0.16→保留；否则选新目标：置 Duck+0xe8=0.1；2×uniform_f32+uniform_sample_disk+x0y+x0z→盘面采样；新位=当前位+采样*0.8(active 原非 0)组合；校验 game_config::is_within_glade_shape()+距原点 r(new)<=r(pos)（向内偏好）+bilinear_sample_at_world_xz<=0.5→无效；否则 4×bilinear 求和 clamp[0,1]>=0.1→无效；否则写 CritterGoal={active=1,pos=new}。结尾 Duck+0xe1=0。
- 算法：计时器驱动“停留-换目标”状态机；圆盘均匀采样；目标有效性校验（glade 形状+半径向内偏好+水格采样）。
- 常量：0.16(0x3e23d70b，即距离 0.4 平方)、0.8(0x3f4ccccd)、0.1(0x3dcccccd)、3.0(0x40400000)、1.0、0.5(0x3f000000)；Duck+0xe0/e1 字节、+0xe4 两 f32、+0xe8 f32。
- 未知 (U)：换目标时 +0xe8=0.1 语义；两半耗尽分支低半寄存器残留。

## set_duck_velocities_towards_goals
- 地址 0x142314470（RVA 0x2314470），源码行 698–728。
- 内部逻辑 (O)：查询(Transform,&Duck,&DuckState,&CritterGoal,&mut CritterVelocityTarget,&mut CritterVelocityMultiplier)。仅 DuckState==2。CritterGoal.active bit0 且 Duck+0xe0==1→dir=normalize(goal-pos)；否则方向零。写 VelocityTarget.dir 三轴+Multiplier。Duck+0xfc==1(冻结)→VelocityTarget 清零。
- 算法：目标方向归一化+冻结门。
- 常量：1.0(0x142925904)；DuckState 过滤值 2；Duck+0xfc 冻结字节。
- 未知 (U)：Multiplier 语义；DuckState==2 物种语义(I：自由游水)。

## update_duck_ripples
- 地址 0x142314b70（RVA 0x2314b70），源码行 735–796。
- 内部逻辑 (O)：dt=*Res<Time>+0x28；2×expf→retention1=1-exp(-2.0*dt)(0xc0200000)、retention2=1-exp(-5.0*dt)(0xc0a00000)（波纹衰减）。from_iter<array<u32,4>,Take<FilterMap<QueryIter<(Entity,Transform,DuckState)>,closure_env$0>>> 收集最多 4 只水面鸭。FrozenMap::get<SsboId,Box<BufferState>>+memcmp；组 0x30 stride 记录→rhapsody::raw::device::GraphicsDevice::update_buffer_bytes() 上传 SSBO(0x210B)。
- 算法：指数衰减(ripple decay)；GPU SSBO 上传。
- 常量：-2.0/-5.0(0xc0200000/0xc0a00000)；最多 4 鸭；buffer 0x210B；SSBO 名 anon_…_0(len0x30=48)。
- 未知 (U)：SSBO 顶点布局(0x30 stride 字段)；FilterMap closure 筛选细节。

---

# src/spawn_ducks.rs

## DuckSpawnLocations::new
- 地址 0x140f8ee90（RVA 0xf8ee90），源码行 54–75。
- 内部逻辑 (O)：双层循环构建 10×10=100 候选点(0x4b0/0xc)。x=(i/24)*52-(1-i/24)*52(52.0=0x42500000,24.0=0x41c00000)；每行 z 取 10 档：-52.0(0xc2500000)、-40.44(0xc221c71c)、-28.89(0xc1e71c73)、-17.33(0xc18aaaa9)、-5.78(0xc0b8e390)、5.78(0x40b8e390)、17.33(0x418aaaac)、28.89(0x41e71c73)、40.44(0x4221c71c)、52.0(0x42500000)。每点 12B：flag u8@0 + pad + x f32@+4 + z f32@+8。尾部写资源：in_RCX[2]=len,[0]=0,[1]=4(cap)，*(u32*)(in_RCX+3)=0x41c80000(=25.0)。
- 算法：均匀网格候选(10×10)。
- 未知 (U)：flag 字节含义；尾部 25.0 语义(I：水面检测半径)。

## spawn_ducks
- 地址 0x140f8f210（RVA 0xf8f210），源码行 80–197。
- 内部逻辑 (O)：门控：非 demo 且 BackupSaveFileKind::is_corrupted()==false。计时资源+0x18+=dt；活鸭数<15(0xf)时 3×速率累加；timer>=30.0(0x41f00000)才生，生后归 0。候选 sorted_by<IterMut<(Vec2,bool)>,closure_env$0>（已用标志）；逐点 bilinear 水采样<=0.99(0x3f7d70a4)跳过、TerrainHeightsData::sample_at_world_xz 高度<=0 跳过（须水面）；取中→uniform_f32+golden_spiral+x0y 构 3 点轨迹→in_place_collect::from_iter<Trajectory,…spawn_ducks::closure_env$1>；uniform_f32+uniform_sample_circle 随机圆偏移→Vec::push<TrajectoryPlan>(0x30 槽)；标记候选已用。
- 算法：冷却计时(30s)+上限(15)的批量生成；候选排序筛选；黄金螺旋轨迹构建。
- 未知 (U)：3×速率累加触发细节；Vec2,bool 候选排序键。

## despawn_ducks
- 地址 0x140f8f8b0（RVA 0xf8f8b0），源码行 206–278。
- 内部逻辑 (O)：逐 Duck：DuckState==2 且水面采样(首点+4 采样求和)判断——深/浅水越界→生成离场 TrajectoryPlan(push 资源)；DuckState<2 且带轨迹(Duck+0xa0!=0)同理(4+1 采样，0.86<水||采样<0.1→离场)；DuckState==3 不生成。离场计划由 plan_duck 热键 B 路径执行。
- 算法：水面采样越界检测→生成离场轨迹计划。
- 常量：0.86、0.1(0x3dcccccd)、1e-5(0x3727c5ac)、0.9(0x3f666666)、0.99(0x3f7d70a4)；Duck+0xa0/+0x98。
- 未知 (U)：离场阈值组合完整条件；DuckState 0/3 语义。

---

# src/duck_splash_particle.rs

## duck_splash_particle
- 地址 0x142322650（RVA 0x2322650），源码行 22。
- 内部逻辑 (O)：响应 SpawnSplashParticle 事件（插件 build 注册）：对触发鸭位置 AssetMeshLibrary::get_handle_by_name+AssetShaderLibrary::get_raster_handle_by_name 取网格/着色器句柄，Commands::spawn<(DrawableMeshBundle,SplashParticle,DontCastShadow)> 生成水花粒子；旧粒子 EntityCommands::despawn()。
- 算法：事件驱动粒子生成/销毁。
- 常量：2.5(0x40200000)、1.0；组件 SplashParticle/DontCastShadow。
- 未知 (U)：粒子网格/着色器名与生命周期参数。

---

# src/ducks_react_to_being_inside_buildings.rs

## ducks_react_to_being_inside_buildings
- 地址 0x14231b140（RVA 0x231b140），源码行 13。
- 内部逻辑 (O)：逐鸭 PublicWalls::get+PublicWallState::get_wall_shape 判是否在建筑内；PublicWallState::max_y+PublicWalls::roof_entity+Roof::height 取屋顶高；GladeBorder::is_pos_inside；触发 AudioInstance::start_if_not_playing（反应音效）。
- 算法：点在墙/屋顶包络内判定（复用 core 查询）+音效触发。
- 常量：1.5(0x3fc00000)。
- 未知 (U)：建筑占用查询返回结构与反应行为细节。

---

# src/fish.rs

## FishSpawnLocations::new
- 地址 0x140f909f0（RVA 0xf909f0），源码行 63–81。
- 内部逻辑 (O)：25×25=625 候选点：x=(i/24)*52-(1-i/24)*52，z=(j/24)*52-(1-j/24)*52(52.0=0x42500000,24.0=0x41c00000)；每点 12B(flag@0+x@+4+z@+8)。尾部 in_RCX[2]=len,[0]=0,[1]=4。
- 算法：均匀网格候选(25×25)。
- 未知 (U)：flag 字节含义。

## despawn_fish
- 地址 0x140f90c10（RVA 0xf90c10），源码行 84。
- 内部逻辑 (O)：释放并重置 FishBoids Vec(dealloc+置空 cap=4)，再调 FishSpawnLocations::new()；随后恢复保存的 FishSpawnLocations Vec。
- 算法：资源清理/重置。
- 未知 (U)：无。

## fish
- 地址 0x140f90cd0（RVA 0xf90cd0），源码行 92–334。
- 内部逻辑 (O)：出生：Boid 数<200 时逐帧补，每帧最多 20(0x14)次；候选点 hash 采样 FishSpawnLocations 网格；bilinear_sample_at_world_xz>0.5 才生（水面）；Commands::spawn<(Fish,NaniBundle<SolidVertexColorInstanceData>)>+写 Boid{pos,vel}。初始 vel：x/y=fastrand::f32+(-0.5)、y 另一档(-1.0)；z=fastrand::f32-1.0。每帧 Boid 更新：Boid::frame_update<fish closure_env$0>；随后 4×水面采样 clamp[0,1]；<0.1→WallSpatialHash::query 逐墙段，投影距离<1.12(0x3f8f5c29)→time_continuous_lerp_vec3 推开。Fish 实体转向：byte==2 记录，2D 距离<2.0(0x40000000)→速度朝目标 lerp(×5.0/×8.0 两档)。光标/相机逃避：距光标<fVar51（设置值 uVar5∈0..15 查 switch：case1/8/0xb/0xd→4.0；case2/3→*res+2.3；case4/5/10/0xc→3.97；case0→0）反向 lerp 逃开；相机 mode==3 且距 eye<2.0→逃开。
- 算法：boids 群体+出生/清理+水面/墙体避让+光标/相机逃避。
- 常量：Boid 0x18；上限 200、每帧 20；水面阈值 0.5、墙距 1.12；逃避半径 switch 表。
- 未知 (U)：fVar51 switch 真实来源资源；is_corrupted 谓词与速度档设计意图。

---

# src/butterfly.rs

## update_butterfly_states
- 地址 0x141d84d40（RVA 0x1d84d40），源码行 43–253。
- 内部逻辑 (O)：查询(Transform,&Velocity,&mut Butterfly,&mut ButterflyAudio,&CritterGoal)；同时收集 Sheep pos(Vec2)避让。dt=*Res<Time>+0x28。距离门：相机 mode==3→距 eye 阈值 4.0(0x40400000)；否则距光标 8.0(0x40800000)。远距才推进状态机。状态机跳表：DAT_142f68008 按 Butterfly.state(u32@+0) 索引，handler 收(fVar25,fVar30,flag&8.0)。Butterfly 记录：[3] dwell f32、[4] 相位 f32、[5] 相位参考 f32。计时：dwell<=0→重置=uniform_f32*0.3+0.2(0x3e99999a/0x3e4ccccd)；相位 |0.5-[5]| 在[0.2,0.9)外→跳表分支。
- 算法：有限状态机（跳表分派）+距离门控+相位计时。
- 常量：4.0/8.0 距离门；0.3/0.2 dwell 区间；Butterfly 记录 stride0x18；跳表 DAT_142f68008。
- 未知 (U)：状态枚举与跳表各 handler 语义。

## update_butterfly_goals
- 地址 0x141d86060（RVA 0x1d86060），源码行 256–331。
- 内部逻辑 (O)：timer@+8-=dt；timer<=0||goal.inactive||dist²<0.16→新 timer=uniform*0.5+0.1；2×uniform_f32+uniform_sample_sphere+x0z→方向；按状态选竖向偏置：state==1→(-0.2835,0.5)(0xbe91745d/0x3f000000)，否则→(0.8529,1.0)(0x3f5a2e8c/0x3f800000)；dir=normalize(加权采样)；proposed=pos+dir*(timer*1.77)(0x3fe147ae)；y clamp<=8.0(0x41000000)；校验 sqrt(z²+x²)<65.0||<原半径(0x42820000)→接受，否则清 goal。
- 算法：球面采样目标生成+状态相关竖向偏置+半径向内偏好校验。
- 常量：1.77、0.5/0.1 dwell、8.0 y 上限、65.0 半径、0.16 到达阈值。
- 未知 (U)：偏置常量对 64 位打包 select 的逐分量配对。

## set_butterfly_velocities_towards_goals
- 地址 0x141d867b0（RVA 0x1d867b0），源码行 334–378。
- 内部逻辑 (O)：查询(Transform,Sheep,CritterGoal,&mut CritterVelocityTarget,&mut CritterVelocityMultiplier)（反编译显示含 Sheep，与 butterfly 语义不符，见 unknown）。dir=normalize(goal-pos)*1.77 写 VelocityTarget；multiplier+=1.0（每帧累加）；随后按 Butterfly.state 跳 DAT_142f68018（动画/姿态 handler，收(…,0.5)）。
- 算法：目标方向归一化×标度+状态跳表动画分派。
- 常量：1.77(0x3fe147ae)、1.0、0.5(0x3f000000)。
- 未知 (U)：查询为何含 Sheep（I：组件共享/反编译误解析）；Multiplier 每帧+=1.0 用途。

## update_butterfly_audio
- 地址 0x141d86eb0（RVA 0x1d86eb0），源码行 381–385。
- 内部逻辑 (O)：逐蝴蝶 ButterflyAudio::set_position()。
- 未知 (U)：无。

## butterflies_send_to_gpu
- 地址 0x141d87140（RVA 0x1d87140），源码行 388。
- 内部逻辑 (O)：from_iter<Vec<ButterflySsbo>,Map<QueryIter<(Transform,Butterfly)>,closure_env$0>> 构建实例缓冲；FrozenMap::get<SsboId,Box<BufferState>>(名 anon_…_2，len0x46=70)+memcmp；render::ssbo::impl$6::cpu_write<ButterflySsbo>() 写 SSBO。
- 算法：CPU→GPU 实例缓冲上传。
- 未知 (U)：ButterflySsbo 字段布局；closure_env$0 打包逻辑。

---

# src/firefly.rs

## fireflies_send_to_gpu
- 地址 0x142316760（RVA 0x2316760），源码行 20。
- 内部逻辑 (O)：from_iter<Vec<FireflySsbo>,Map<QueryIter<(Transform,Firefly)>,closure_env$0>>；FrozenMap::get<SsboId,Box<BufferState>>(名 len0x44=68)+memcmp；render::ssbo::impl$6::cpu_write<FireflySsbo>()。
- 未知 (U)：FireflySsbo 字段布局。

## update_firefly_goals
- 地址 0x142316940（RVA 0x2316940），源码行 46–112。
- 内部逻辑 (O)：入口门 *(char*)(*in_R8+4)!=1→return（CritterControl 门）。每萤：timer-=dt；刷新条件同 duck/butterfly(timer<=0||goal.inactive||dist²<0.16)。**惯性 bias**：*(f32*)(&0xbf000000_3f000000+(goal.active==0)*4)→无目标 **-0.5**(0xbf000000)、有目标 +0.5(0x3f000000)。〔注：批次2复核**驳回** extra-analysis 的「-1.0 校正」——0xbf000000 作为 f32 即 -0.5；raw memory@0x142ccfc58=00 00 00 3f 00 00 00 bf，SETZ 选偏移4。原草稿的 -0.5 正确。〕新 timer=uniform*0.15+0.05(0x3e19999a/0x3d4ccccd)；2×uniform_f32+uniform_sample_sphere+x0z；方向=normalize(sample*(1+bias)，z 额外+0.5)；proposed=pos+dir*timer；校验 sqrt(z²+x²)<65.0||<原半径(0x42820000)。
- 算法：球面采样+惯性偏置目标生成。
- 常量：0.15/0.05 timer、65.0 半径、0.16 到达阈值、bias -1.0/+0.5。
- 未知 (U)：控制门 *in_R8+4 资源身份(I：CritterControl)；z 额外+0.5 几何意图。

## set_firefly_velocities_towards_goals
- 地址 0x142317040（RVA 0x2317040），源码行 115–136。
- 内部逻辑 (O)：查询(Transform,Sheep,CritterGoal,VelocityTarget,Multiplier)（同含 Sheep）。dir=normalize(goal-pos)（无目标取 pos）；写 VelocityTarget.dir；multiplier=1.0(0x3f800000)；海拔下限：pos.y<1.0 时 dir.y=max(dir.y,0.25)(0x3e800000)。
- 算法：目标方向归一化+海拔下限修正。
- 未知 (U)：查询含 Sheep 原因。

---

# src/wren.rs

## update_decorator_bird_spots
- 地址 0x14231c7d0（RVA 0x231c7d0），源码行 81–132。
- 内部逻辑 (O)：puffin；遍历装饰器(DecoratorId)，管理 HashMap<DecoratorId,Entity> 栖点：删除失效项(Entities::contains→EntityCommands::despawn，hashbrown::HashMap::remove_entry)；为新装饰器 BirdSpotManager::gen_id()+create_spots()，Commands::spawn_empty()+queue(insert<BirdSpots>)，HashMap::insert；收尾 HashMap::retain<…,closure_env$0> 清理。
- 算法：HashMap 管理的栖点生命周期；稀疏实例缓冲(ChunkId grow)。
- 未知 (U)：create_spots 返回布局；栖点数量/分布规则。

## update_roof_bird_spots
- 地址 0x14231d6c0（RVA 0x231d6c0），源码行 144–250。
- 内部逻辑 (O)：BirdSpotManager::recycle<closure_env$3>；逐屋顶：PublicWalls::roof_entity、Roof::is_flat、RoofRidgeDims::from、calc_roof_tip_center_ls、Rectangle2d::orientation_mat2/new_axis_aligned/get_edge_pts、Roof::is_gable、ClutterVersion::is_initial；utils::rng::init_rng+utils::random_splits+fastrand::Rng::f32+Roof::height+swizzlers::xvy+roof_noise::apply_roof_noise；BirdSpotManager::gen_id/create_spots；Commands::insert<BirdSpots>。
- 算法：屋顶几何(脊线/山墙/平顶)取样+随机分割(random_splits)布置栖点+屋顶噪声偏移。
- 常量：0.1(0x3dcccccd)、1.0、0.5。
- 未知 (U)：栖点具体布置参数；random_splits 语义。

## update_wrens
- 地址 0x14231e390（RVA 0x231e390），源码行 273。
- 内部逻辑 (O)：PlacementDataSsbo::to_affine3a 取放置矩阵；跳表(按状态)分派行为；DebugLine::new/show、DebugSphere::new/radius/color/show(调试可视化，DEBUG_SHOW_BIRD_* 门控)；audio_resources::impl$10::set_position(音频位置)。
- 常量：_UNK_14304b39c、0.5(0x3f000000)。
- 未知 (U)：跳表各 handler 语义；Wren 状态枚举。

---

# src/wren_boids.rs

## sim_wren_boids
- 地址 0x142326a80（RVA 0x2326a80），源码行 56–533(19140B)。
- 内部逻辑 (O)：顶层 *(char*)(*in_R8+5) 门：非 0 执行主模拟；==0 逐 Wren despawn（发行“wren 开关”）。出生(L385–515)：上限 80(0x4f<len→break)；位置 fastrand::f32：x=u*130-65(0x43020000/0xc2820000)、y=u*9+1(0x41100000/0x3f800000)、z 同理；Commands::spawn<(Wren,NaniBundle<WrenInstanceData>,PrevTransform)>+音效+写 Boid(0x18)+0x38 槽。FreeSpot 收集(from_iter<FreeSpot,FlatMap<…>>)；BirdHaven 树查询(multiunzip<…Without<TreeMarkedForDeletion>…>→Vec<Sphere>,Vec<Entity>)。主循环：每 Wren sqrt(x²+y²)<=91.9(0x42b7d907)且 y<=30(0x41f00000)才保留，否则 BoidFlock::remove<wren>。摇树出生：>10.0(0x41200000)且计时<=0→log "Tree has been shaken!"；burst=rng*5+1 只；受 80 上限。每帧 boid Boid::frame_update<wren closure_env$3>。
- 算法：boids 群体+出生/清理+树摇爆发+停靠点/庇护点分配。
- 常量：上限 80、出生 x∈[-65,65]/y∈[1,10]、保留半径 91.9/y 上限 30、摇树阈值 10.0、burst=rng*5+1；Boid 0x18；sim_wren_boids::closure_3(0x141dc42f0) RAY_DIRS 谓词。
- 未知 (U)：FreeSpot/BirdHaven/HashMap 落盘细节；后半段个别常量(0x3f8147ae=1.01 出生爆发偏移)。

---

# src/sheep/sheep_behavior.rs

## update_sheep_goals
- 地址 0x142315050（RVA 0x2315050），源码行 26–221。
- 内部逻辑 (O)：查询(Transform stride0x30, Sheep stride0x68, CritterGoal stride0x10, 2×4B)。Sheep+8 行为状态(==1 跟随)：MainCamera::mode()；mode==3→eye_position+eye_forward→目标=eye+forward*2；否则 CursorTerrainRaycast::position。写 CritterGoal={active=1,pos}；timer@+0xc-=dt；距目标<=fVar13→timer-=3*dt；timer>=0→2×uniform_f32；timer<0→state=0+uniform_f32。共享目标刷新：Sheep+0x30 字节→+0x34 两半-=dt；全>0→CritterGoal.active 且 dist²>=0.16→保留；否则 2×uniform_f32+uniform_sample_disk+x0y/x0z 新目标；校验 bilinear_sample_at_world_xz 水面。demo 模式额外半径门(>56.0=0x420c0000)。
- 算法：跟随(相机/光标)状态机+圆盘采样漫游目标+计时器。
- 常量：0.2(0x3e4ccccd,fVar13)、1.0、3.0(0x40400000)、0.16(0x3e23d70b)、56.0(0x420c0000)、14.0(0x40e00000)、12.0(0x41400000)、4.0(0x40800000)。Sheep{+8 state u32,+0xc timer f32,+0x30 字节,+0x34 两 f32}。
- 未知 (U)：Sheep+0x30 字节语义；fVar13(0.2) 与 3*dt 快进完整条件。

## set_sheep_velocities_towards_goals
- 地址 0x142315e40（RVA 0x2315e40），源码行 224–245。
- 内部逻辑 (O)：查询(Transform,&Sheep,&CritterGoal,&mut CritterVelocityTarget,&mut CritterVelocityMultiplier)。Sheep::being_petted() 为真→跳过；否则 dir=normalize(goal-pos) 写 VelocityTarget+Multiplier=1.0。
- 常量：1.0(0x3f800000)。
- 未知 (U)：Multiplier 语义。

## update_sheep_audio
- 地址 0x142316490（RVA 0x2316490），源码行 248–253。
- 内部逻辑 (O)：逐羊 audio_resources::impl$10::set_position()。
- 未知 (U)：无。

---

# src/sheep/sheep_terrain.rs

## sheep_walk_on_terrain
- 地址 0x14231bd80（RVA 0x231bd80），源码行 15–80。
- 内部逻辑 (O)：dt=*Res<Time>+0x28；smooth=1-exp(-3.0*dt)(0x142ad58e8=-3.0f)。每实体 TerrainHeightsData::sample_at_world_xz<TerrainHeightSsbo>(0x141246fb0)→h。switch(state@+0)：2(地面)→pos.y=h；3→panic；4(过渡)→time_continuous_lerp→pos.y=lerp，水格 bilinear<=0→state=2；default(行走)→dy=(h+4.0)-pos.y；地形低：dy clamp[-2,2]，v+=(dy*10-v)*smooth；地形高：pos.y=h，dy clamp[-5,5]，v+=(dy*10-v)*smooth*3；写 v@+4；6(浮空)→dy clamp[-0.3,0.3]，v+=(dy*20-v)*smooth，dy>-0.05→state=2,v=0。
- 算法：竖向弹簧-阻尼跟随+状态机(地面/过渡/行走/浮空)+双线性地形/水格采样。
- 常量：-3.0(0x142ad58e8)、1.0(0x142925904)、4.0 身位偏移、行走增益 10.0、浮空增益 20.0、×3 下陷、clamp[-2,2]/[-5,5]/[-0.3,0.3]、阈值-0.05、130.0(0x142a7b658)；地形状态 8B{state u32@+0,f32@+4}；竖向速度 12B f32@+4。
- 未知 (U)：+4 f32 语义；130.0 用途；水格双缓冲来源。

---

# src/sheep/sheep_unstuck.rs

## sheep_unstuck
- 地址 0x1423246b0（RVA 0x23246b0），源码行 0。
- 内部逻辑 (O)：puffin；SSBO/资源 FrozenMap::get+memcmp。逐羊：RaycastWorld::legacy_spherecast、terrain_rock_mask::sample_at_world_xz、bilinear_sample_at_world_xz<RasterBufferRef<512>>、BackupSaveFileKind::is_corrupted()、TerrainHeightsData::sample_at_world_xz<TerrainHeightSsbo>；据命中/掩码判定卡住，1e-6(0x3727c5ac)等阈值；10.0(0x41200000)、0.5(0x3f000000)。
- 算法：卡住检测(射线/岩石/水格采样)+解卡位移。
- 未知 (U)：解卡位移方向/幅度公式；触发条件完整组合。

---

# src/sheep/sheep_react_to_being_inside_buildings.rs

## react_to_being_inside_buildings
- 地址 0x14231f610（RVA 0x231f610），源码行 20。
- 内部逻辑 (O)：逐羊 PublicWalls::get+PublicWallState::get_wall_shape 判建筑内；PublicWallState::max_y+PublicWalls::roof_entity+Roof::height 取屋顶高；GladeBorder::is_pos_inside；utils::rng::init_rng 取随机 emoji 索引((idx&0xffffffff)*7，查 DAT_14304b8c8 表)；Commands::spawn<(Ui2dExtent,Ui2dImage,SheepReactionEmoji)> 生成表情。
- 常量：1.5(0x3fc00000)；emoji 索引表 DAT_14304b8c8/DAT_14304b8e8。
- 未知 (U)：建筑占用查询返回结构；emoji 表内容。

## sheep_reaction_emoji
- 地址 0x14231ff70（RVA 0x231ff70），源码行 114。
- 内部逻辑 (O)：逐表情实体 utils::easing_functions::EasingFunction::apply() 做弹出/淡出；WindowSize::as_vec2()+world_space_to_screen_space() 定位；动画结束 EntityCommands::despawn()。
- 算法：缓动动画+世界→屏幕空间投影。
- 常量：0.2(0x3e4ccccd)、0.4(0x3ecccccd)、1.0。
- 未知 (U)：缓动函数具体类型/时长；UI 布局细节。

---

# src/sheep/sheep_floatie.rs

## sheep_floaties
- 地址 0x142320cf0（RVA 0x2320cf0），源码行 15。
- 内部逻辑 (O)：逐羊：水面判定后 Commands::spawn_empty()+insert<DrawableMeshBundle>+insert<SheepAccessory> 生成漂浮物；AudioCore::instantiate_audio+set_position+start_if_not_playing(入水音效)。
- 常量：-2.0(0xc0000000)、2.0(0x40000000)、1.0；组件 DrawableMeshBundle/SheepAccessory。
- 未知 (U)：漂浮物网格/材质资源；水面判定阈值。

---

# src/sheep/sheep_umbrella.rs

## sheep_umbrellas
- 地址 0x142321980（RVA 0x2321980），源码行 0。
- 内部逻辑 (O)：逐羊：雨天判定后 Commands::spawn_empty()+insert<DrawableMeshBundle>+insert<SheepAccessory> 生成雨伞；AudioCore::instantiate_audio+set_position+start_if_not_playing；utils::easing_functions::EasingFunction::apply(开合动画)。
- 常量：-2.0、2.0、1.0；组件同上。
- 未知 (U)：雨伞网格/材质；天气判定来源。

---

# src/sheep/sheep_walk_animation.rs

## sheep_walk_animation
- 地址 0x14231a630（RVA 0x231a630），源码行 0。
- 内部逻辑 (O)：逐羊 Sheep::being_petted() 门控（被抚摸时跳过）；跳表 DAT_14304b0f8 按 *(uint*)(Sheep 记录+0x10) 索引分派动画状态；fVar9=0.2(0x3e4ccccd)。
- 算法：动画状态机(跳表)。
- 未知 (U)：跳表 handler 语义；动画参数。

---

# src/sheep/sheep_pet_animation.rs

## sheep_pet_animation
- 地址 0x14231ac20（RVA 0x231ac20），源码行 15。
- 内部逻辑 (O)：from_iter<Vec<usize>,Map<EventIterator<PlaySheepPetAnimation>,closure_env$0>> 收集事件目标；跳表 DAT_14304b178 按 *(uint*)(记录+0x18)(phase) 索引分派，收(0.3,1.0,0.2)常量。对应候选实现 reconstruction/critters/sheep_pet.rs 五相位状态机(phase 0..4：idle→启动→保持→淡出→完成；progress=delta/0.3 或 delta/0.2；amplitude@+0x58、控制标志@+0x60、phase@+0x18、progress@+0x1c、target@+0x20)。
- 算法：五相位宠物动画状态机(跳表分派)。
- 常量：0.3(0x3e99999a)、1.0、0.2(0x3e4ccccd)；记录 104B(0x68)；跳表 DAT_14304b178。
- 未知 (U)：跳表各相位 handler 精确写入；事件 PlaySheepPetAnimation 消费链。

---

# src/sheep/sheep_input_record_replay.rs

## sheep_input_record_replay
- 地址 0x14231b720（RVA 0x231b720），源码行 0。
- 内部逻辑 (O)：两条 from_iter：Vec<Vec3>(Map<QueryIter<Transform,With<Sheep>>,closure_env$0>)与 Vec<f32>(closure_env$1)；InputStream::last_frame_mut()+impl$2::index() 读写输入帧；glam::EulerRot::from_euler_angles。
- 算法：输入录制/回放（帧缓冲读写下，变换序列化）。
- 未知 (U)：回放格式；录制触发条件。

---

# src/sheep/ui_pet_sheep.rs

## ui_pet_sheep
- 地址 0x142324f80（RVA 0x2324f80），源码行 29（closure$0 0x2326170、closure$1 0x2326340）。
- 内部逻辑 (O)：MainCamera::mode() 门(==3 跳过)；format!("pet_sheep")；WidgetParams::new()+UiSounds::empty()+sounds()+no_history_recording()；UiState::add<String>()；UiWidgetContext::intersecting()/is_active()；ws_to_cs(世界→裁剪空间)；常量 0.5(0x3f4ccccd)、1.0、0x1e3ce508、0.3(0x3d99999a)；eye_position 距离门(>5.0=0x40a00000)。
- 算法：UI 交互命中(世界→屏幕投影+相交检测)。
- 未知 (U)：UI 布局/触发后行为（发 pet 事件？）。

---

# src/ui_pet_ducks.rs

## ui_pet_ducks
- 地址 0x142322e30（RVA 0x2322e30），源码行 29（closure$0 0x2324290、closure$1 0x2324460）。
- 内部逻辑 (O)：与 ui_pet_sheep 同构：MainCamera::mode() 门；format!("pet_duck")；WidgetParams::new/UiSounds::empty/sounds/no_history_recording；UiState::add<String>；UiWidgetContext::intersecting/is_active；常量 0.4(0x3ecccccd)、1.0、0.5、6.0(0x40c00000)、_UNK_142c2910c/08/04、___xmm_b33bbd2e…。
- 算法：UI 交互命中。
- 未知 (U)：UI 布局/触发后行为；几个 _UNK_ 常量语义。

---

# src/lib.rs（插件注册）

## system_critters::impl$0::build
- 地址 0x14067d440（RVA 0x67d440），源码行 69。
- 内部逻辑 (O)：资源注册 insert_resource：CritterRng、WrenBoids、CritterControl、DuckTrajectoryPlanner、DuckSpawnLocations、BirdSpotManager、FishBoids、FishSpawnLocations；country_core::resources::critters::impl$0::default()；调用 DuckSpawnLocations::new、FishSpawnLocations::new。事件注册 register_event<SpawnSplashParticle>、register_event<DespawnDuckCmd>。系统注册(add_systems+in_set<GameSystemSet>+chain_ignore_deferred)：蝴蝶/萤火虫 SSBO 上传成组；OnExit<AppLoadingState> 上注册 wren/fish 清理系统(ResMut<WrenBoids> / ResMut<FishBoids>+ResMut<FishSpawnLocations>)。
- 算法：Bevy 插件装配(资源/事件/系统调度)。
- 未知 (U)：各 add_systems 系统确切顺序/集合归属。

# src/sheep/mod.rs（插件注册）

## system_critters::sheep::impl$0::build
- 地址 0x14067d810（RVA 0x67d810），源码行 31。
- 内部逻辑 (O)：事件注册 register_event<PlaySheepPetAnimation>、register_event<CmdCheckForSheepInsideBuilding>、register_event<CmdUnfreezeSheep>；add_systems+chain_ignore_deferred+run_if<fn(Res<GameTime>,Res<State<AppLoadingState>>)->bool>。
- 算法：Bevy 插件装配(羊系统)。
- 未知 (U)：add_systems 系统清单与 run_if 条件函数名。

---

# 逻辑总览
- 共享运动底座(boids.rs/shared.rs)：Boid::frame_update(Boids 三规则+速度约束+地面反射/射线可行性)供 fish/wren 两实例；random_vel_change(锥采样随机转向+四元数 arc 旋转+时间连续 lerp)；BoidFlock::remove(swap_remove 批清理)。shared 层 smooth_critter_velocities(指数衰减速度平滑)、move_critters(dt 位移积分+nlerp 朝向+球投/墙/水避让+停靠点)、critters_avoid_collisions(目标方向+线段/圆几何+墙空间哈希+spherecast+DespawnDuckCmd)。
- 物种目标系(duck/butterfly/firefly/sheep)：统一模式——timer-=dt；timer<=0||!goal.active||dist²<到达阈值→球/盘/圆采样+状态偏置生成新目标→半径向内偏好校验→写 CritterGoal；set_*_velocities_towards_goals 归一化目标方向→CritterVelocityTarget，再经 smooth_critter_velocities 平滑。RNG 统一 perchance。
- 鸭子专属链：DuckSpawnLocations(10×10)+spawn_ducks(30s/上限15)+plan_duck_trajectories(黄金螺旋轨迹+热键 spawn)+move_ducks_along_trajectories(曲线 Frenet 推进+音频)+update_duck_goals(停留状态机)+update_duck_ripples(波纹 SSBO)+despawn_ducks(离场轨迹)+duck_splash_particle/ducks_react_to_being_inside_buildings。
- 羊专属链：sheep_walk_on_terrain(弹簧-阻尼地形跟随)、sheep_unstuck(卡住检测解卡)、sheep_behavior(跟随状态机+速度+音频)、sheep_react_to_being_inside_buildings/sheep_reaction_emoji(emoji 反应)、sheep_floaties/sheep_umbrellas(附件)、sheep_walk_animation/sheep_pet_animation(动画跳表)、sheep_input_record_replay(录制回放)。
- wren 链：sim_wren_boids(出生/上限80/树摇爆发/boids)、update_wrens(状态跳表+放置矩阵)、update_decorator_bird_spots/update_roof_bird_spots(栖点管理)。
- GPU/UI 胶水：butterflies_send_to_gpu/fireflies_send_to_gpu(SSBO cpu_write)、ui_pet_sheep/ui_pet_ducks(UI 命中)、插件 build 两处。

# 算法清单
1. Boids 群集(Reynolds)：分离/对齐/凝聚+smoothstep 分离核+heading 同向门+速度约束+NaN 归零(Boid::frame_update，fish/wren 两实例)。
2. 随机转向：锥体均匀采样(uniform_sample_cone)+四元数 arc 旋转+时间连续 lerp(random_vel_change)。
3. 指数衰减速度平滑(EMA)：powf(retention_per_second,dt) 加权(smooth_critter_velocities)。
4. 四元数朝向平滑：from_rotation_arc+nlerp+归一化(move_critters)。
5. 球体投射/射线投射：RaycastWorld::legacy_spherecast/legacy_raycast(避让/寻路/解卡)。
6. 几何线段相交：LineInterval::line_segment/relate+Circle2d::new_rotated(避让)。
7. 空间哈希：WallSpatialHash::query(墙体邻域查询)。
8. 双线性栅格采样：bilinear_sample_at_world_xz<RasterBufferRef<512>>(水/地形)。
9. 黄金螺旋/圆盘/圆/球面均匀采样：golden_spiral、uniform_sample_disk/circle/sphere(目标/出生点)。
10. 计时器状态机：停留-换目标(duck/butterfly/firefly/sheep)。
11. 弹簧-阻尼跟随：竖向地形跟随(sheep_walk_on_terrain)。
12. 曲线参数化+Frenet 帧：轨迹推进姿态(move_ducks_along_trajectories)。
13. 有限状态机跳表：butterfly states、sheep walk/pet 动画、wren 行为。
14. 缓动动画：EasingFunction::apply(emoji/伞)。
15. 世界→屏幕空间投影：world_space_to_screen_space/ws_to_cs(UI 命中/emoji 定位)。
16. GPU 实例缓冲上传：render::ssbo::cpu_write / GraphicsDevice::update_buffer_bytes(SSBO)。
17. RNG：perchance uniform_f32(目标系)+fastrand f32(boids/spawn)。
18. 屋顶几何：RoofRidgeDims/calc_roof_tip_center_ls/Rectangle2d+random_splits+roof_noise(栖点)。

# 未知清单（汇总）
1. CritterLocomotion 枚举各值(2/3/4/6/7 与位掩码{0,1,5,6})的物种映射。
2. CritterVelocityMultiplier 写入值(Query+0x14)来源语义。
3. DuckState 值 0 语义；值 1/2/3 为轨迹正向/自由游/反向(I)。
4. Trajectory/TrajectoryPlan 完整字段布局(0x30 槽仅部分确认)。
5. butterfly 两跳表 DAT_142f68008/DAT_142f68018 各 handler 语义；状态枚举。
6. set_butterfly_velocities / set_firefly_velocities 查询为何含 Sheep(疑似组件共享/反编译误解析)。
7. firefly 控制门 *in_R8+4 资源身份(I：CritterControl)；z 额外+0.5 几何意图。
8. BoidFlock::remove 的 0x38 副槽完整字段语义。
9. critters_avoid_collisions 8B 命中码 2/3/7 语义；0x68 记录语义。
10. update_duck_goals 换目标时 Duck+0xe8=0.1 语义；两半耗尽分支低半寄存器残留。
11. update_duck_ripples SSBO 顶点布局(0x30 stride 字段)。
12. DuckSpawnLocations 尾部常量 25.0 语义(I：水面检测半径)；候选 flag 字节含义。
13. spawn_ducks 3×速率累加触发细节；despawn_ducks 离场阈值组合。
14. fish 速度档由 is_corrupted() 选择的设计意图；fVar51 逃避半径 switch 来源资源。
15. wren sim_wren_boids 的 FreeSpot/BirdHaven/HashMap 落盘细节；后半段个别常量。
16. sheep_walk_on_terrain 记录 +4 f32 语义、130.0 采样参数。
17. sheep_unstuck 解卡位移公式与触发条件完整组合。
18. sheep_react/ducks_react 建筑占用查询返回结构；emoji 表内容。
19. sheep_floatie/sheep_umbrella 网格/材质资源；天气/水面判定来源。
20. sheep_walk_animation/sheep_pet_animation 跳表各 handler 语义；动画参数。
21. sheep_input_record_replay 回放格式与录制触发条件。
22. ui_pet_sheep/ui_pet_ducks 布局与触发后行为；_UNK_ 常量。
23. 两处 impl Plugin::build 的 add_systems 系统完整清单与顺序。
24. **既有 firefly 差分唯一失败项**：smooth_critter_velocities NaN 符号位(原码 ffc00000 负 NaN vs Rust 7fc00000 正 NaN)，根因未定(I：powf/0*inf 求值顺序)。
25. 出生/停靠点/UI 中 _UNK_/anon_… 全局符号的精确值。

---

# 附：最小 Rust 实现草案（仅逻辑完全确定者，不写入工程）

1. random_vel_change 核心（结构示意见下；cone 参数、lerp 区间、time_continuous_lerp 时长为调用点常量→unknown）：
```rust
pub fn random_vel_change(boid: &mut Boid, dt: f32, rng: &mut fastrand::Rng) {
    boid.timer -= dt;
    if boid.timer <= 0.0 {
        boid.dir = utils::uniform_sample_cone(rng);      // 2×rng.f32()
        boid.timer = utils::lerp(0.0, 1.0, rng.f32());   // 区间来自调用点
    }
    let dir = utils::time_continuous_lerp_vec3(boid.dir, boid.dir, dt);
    let q = glam::Quat::from_rotation_arc(Vec3::Y, dir.normalize_or_zero());
    boid.vel = q * boid.vel;
}
```
2. smooth_critter_velocities（逐字段确定，与 reconstruction/critters/src/shared.rs 及 firefly.rs 草稿一致；NaN 符号位差异为已知失败项）：
```rust
pub fn smooth_velocity(v: &mut Vec3, t: &VelocityTarget, dt: f32) {
    let retention = t.retention_per_second.powf(dt);
    let w = 1.0 - retention;
    *v = Vec3::new(v.x*retention+t.direction.x*w, v.y*retention+t.direction.y*w, v.z*retention+t.direction.z*w);
}
```
3. BoidFlock::remove（逐字段确定）：对索引 Vec<u32> 倒序，校验索引后对 0x18 与 0x38 两槽 swap_remove，Entities::contains 存活则 despawn。
4. sheep_walk_on_terrain 竖向核心：reconstruction/critters/src/sheep_terrain.rs 的 apply_terrain_follow 逐字段与反汇编一致，可复用。
5. duck/butterfly/firefly/sheep 目标刷新核心：reconstruction/critters/src/{duck,butterfly}.rs 与 firefly.rs 已落 bevy-free 语义核心；firefly 惯性 bias 维持 -0.5（批次2复核确认「-1.0 校正」是错的）。
