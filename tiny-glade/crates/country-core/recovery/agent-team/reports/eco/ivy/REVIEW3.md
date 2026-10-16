# REVIEW3 — resources/ivy + systems/ivy 三轮复核（reviewer-eco，2026-10-15）

复核对象：restorer 三轮（RESTORE.md 第三版，send_to_gpu 2 文件、spawner 路径 B 按 RE-ANALYSIS2
重写、grower 10 相状态机结构重构）。证据：re-analyst 章节（ivy_send_to_gpu_leaf/segments、
ivy_spawner）、ALGORITHMS §4.1、RNE/rne-read、实测 `cargo check` 通过 + `cargo test --lib
systems::ivy` 20/20 过（测试未覆盖 grower 相状态机）。

## 逐项复核

### 1. send_to_gpu_leaf/segments（库内缺失 GAP 闭合）— ✅ 业务体实现到位，提交接线=库外

- 元素 48B（IvyLeafGpu/IvyFlowerGpu/IvySegmentGpu，`repr(C)` 3×Vec3+3×f32=48，测试断言 size=48）✓。
- SsboId 常量 rne-read 定案（0x9e3cfa7f2993f406 / 0x1d6de5e69ac50d2a / 0x2002d2fbdff3d80f）✓，
  与章节 O 逐一对上。
- `upload_ivy_leaves/upload_ivy_segments` 纯函数：collect（all_leaves / iter→段）→ pack 48B →
  `gpu_write` → instance_count，结构对章节（收集→缓冲查找→计数×0x30→打包→写→计数）✓。
- 建模 gap 判定（协调者问）：
  - **库内业务已闭**：collect+pack+gpu_write+instance_count 系统体属库内业务，已实现为纯函数。
  - **GPU 提交/缓冲注册 = 库外 render 域接线**：需 GraphicsDevice/RenderGraph（render 世界提供），
    属 render 域，不阻塞本批库内业务验收。
  - **SsboId::of 哈希 NAME ≠ 原 SsboId = 库内共享机制缺口（非本模块引入）**：`AssetSsboLibrary::
    get_mut::<T>()` 用 `SsboId::of::<T>()`（哈希 `T::NAME` 全路径串），与原机算 qword 键不符；
    与 GroundedBuildingsSsbo 同源，属 ssbo_library 层对账项。其后果是 render 域按原键注册的缓冲
    无法被本模块 `get_mut` 命中——端到端需 ssbo_library 修 SsboId 后再接。**登记为共享机制缺口，
    非 ivy 业务缺失**。
  - 48B↔104B 精确字段映射 = 待取证 blocker（pack_leaf/pack_ivy_segments 为文档化子集猜测），诚实登记。

### 2. spawner 路径 B（RE-ANALYSIS2 重写）— 大项对齐，侧向偏移量需对账

| 项 | RE-ANALYSIS2 | 现实现 | 判定 |
|---|---|---|---|
| 随机墙 | `usize_less_than(墙数)` @0x14094d2e7 | `wall_rows[usize_less_than(len)]` | **OK** |
| 随机 u | `uniform_f32` @0x14094d394 | `uniform_f32()` | **OK** |
| FastNoise 密度闸 | `noise>=threshold`（阈值 res+0x78 待取证） | `density < noise.threshold → continue`（IvySpawnNoise.threshold，来源待取证） | **OK（诚实待取证）** |
| 侧向 | `get_bool` 定侧（true→−0.68/false→+0.68 @0x14094d6b0） | `get_bool(){−1:+1} × ROOT_OFFSET` | **PARTIAL**：代码应用 ±0.34，RE-ANALYSIS2 写 ±0.68——需对账 0.68 是否×0.5（章节 path A 是 0.68×0.5=0.34；path B 是否直用 0.68 待 RE-ANALYSIS2 原文定案） |
| 2.56 距校验 | 0x142ab3214 | `root_too_close(2.56)` | **OK**（二轮 GAP 已闭） |
| 垂直 | `(uniform×1.5−0.75)/curve_length + u_on_curve` 钳 [0.000928482,1.0] | `vertical_u(curve.length, approx_u_from_pos(...))` | **OK** |
| 投影 | `get_approx_u_from_pos` FUN_141469a20 | approx_u_from_pos 走折线+points_u 插值 | **OK（近似）** |
| 尝试 | 2+usize_less_than(3)∈{2,3,4} | `2 + usize_less_than(3)` | **OK** |
| 自造掷硬币 | —（二轮 flag） | 已删，改 get_bool | **OK** |

### 3. grower 10 相状态机结构 — 结构对应，但守卫位闩死致主生长不可达（**功能缺陷**）

- `GrowerState Local{phase@+0x1c, guard@+0x48, timer@+0x11c}` + match 分派 0/2/5/6-9，与
  ALGORITHMS §4.1 相表（0=守卫翻转、1/3/4=主、2=计时、5=二级表、6-9=panic）结构对应 ✓。
- **缺陷（FIX）**：`phase==0` 只把 `guard=!guard` 后置 `phase=1`；相值此后永不回 0 → **guard 一旦
  置 true 永不清除**。帧 1：phase0→guard=true→run_guard_body；帧 2+：phase=1→guard=true→
  run_guard_body。**主生长体（`advance_growth(ivy, time.delta,...)`）首帧后永不执行**；
  `run_guard_body` 以 `delta=0.0` 推进 → **growth_progress 永不前进、IvySegmentDoneGrowingMsg 永不发**，
  叶生成链（spawn→grow→leaf_spawner）断裂。二轮版本的每帧生长被本轮回归。
  - 修法建议：让相 0 在守卫/主体后把相写回 0（相循环 0↔1 交替，守卫位逐帧翻转），或守卫体执行后清
    guard；具体相循环以原生主闭包 0x14094ac20/0x94c0c0 逐相语义为准（待取证）。
  - 测试缺口：现有 20 个 ivy 测试只测 advance_growth/start_growth_segment 等纯函数，**未跑
    add_ivy_systems 的相状态机**，故未捕获。
- 主闭包逐相语义（空间哈希遍历、接受阈值）、二级表 0x142ab3bf4 目标、守卫谓词（除墙跳重锚）仍
  待取证——RESTORE.md 诚实登记 ✓。

### 4. 噪声闸门阈值来源（res+0x78 / IvySpawnNoise 构造）— ✅ 未伪造

`noise.threshold`（现 -0.02）来源标待取证；IvySpawnNoise FastNoise 构造参数（seeded45/oct3/
gain2.0/freq0.3/seed123）在首轮已对 insert_ivy_resources 核过。诚实登记，OK。

## 结论与 DISPATCH 建议

- **ivy = fix_pending**：
  - ✅ 已闭：FIX-1、spawner 路径 B 大部（随机墙/u/密度闸/侧向 get_bool/2.56/垂直/尝试）、
    send_to_gpu 业务体（8 函数）、编译+测试。
  - **FIX 必改**：grower 相状态机守卫位闩死（主生长永不执行，叶链断裂）——需相循环回 0 或守卫
    清除；并补一个覆盖 add_ivy_systems 相机的测试。
  - **对账**：spawner 路径 B 侧向偏移 ±0.68 vs ±0.34（RE-ANALYSIS2 原文定案后再改）。
  - **待取证（诚实）**：主闭包逐相语义/二级表目标、空间哈希格采样、max_ivy_grow_height、
    GPU 48B↔104B 映射；GPU 提交=render 域接线；SsboId::of 建模=ssbo_library 共享机制缺口。
- **garden = review_passed（限定）**、**sheep = review_passed（限定）** 维持（三轮无新增改动）。
