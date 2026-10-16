# REVIEW4 — resources/ivy + systems/ivy 四轮复核（reviewer-eco，2026-10-15）

复核对象：restorer 四轮（RESTORE.md 第四版，MUST-FIX 守卫闩死修复 + 相循环回归测试 +
侧向偏移分列）。实测：`cargo check` 通过；`cargo test --lib` **1592/1**（唯一败
geometry/sphere 他组）；ivy 21/21 过（含新相循环测试）。

## 逐项复核

### 1. grower 守卫闩死修复 — ✅ 属实，守卫相/生长相交替

- 现实现：`phase==0` toggle guard（false↔true）后**即运行所选体**（guard 体/主生长体）并置
  `phase=1`；`phase 1|3|4` 按 guard 位运行后**回 phase=0**；`phase 2` 计时门（≥0 主/否则守卫）
  后回 0；`phase 5` 二级表后回 0；`6..=9` panic。
- 相循环（默认 phase0/guard=false）：帧1 phase0→guard=true→守卫体；帧2 phase1→守卫体→回0；
  帧3 phase0→guard=false→**主生长体（time.delta）**；帧4 phase1→主生长体→回0；帧5 phase0→
  guard=true→守卫体……即**守卫相 2 帧、生长相 2 帧相隔交替**，主生长体在生长相以 `time.delta`
  推进。与 ALGORITHMS §4.1 相表结构对应（0=守卫翻转、1/3/4=按位主入口、2=计时、5=二级表、
  6-9=panic 入口）。
- 守卫体 `advance_growth(ivy, 0.0)` 不再阻断主生长（生长相带 delta 推进）；守卫体补了
  `maximum_point_count` 门（三轮"过度生长"缺陷消除）。OK。

### 2. 相循环回归测试 — ✅ 真实覆盖原 bug（非假阳）

`phase_cycle_keeps_advancing_growth_and_fires_completion_events`：60 帧完整 `ivy_grower`
（Local<GrowerState> 持久），delta=0.075，断言 pending 段完成（None）+ 尾点≈2X +
`IvySegmentDoneGrowingMsg` 非空。核对：主生长相首帧即 `progress=0.075/0.075=1.0≥0.99` 完成并发
事件。**旧闩死代码**下主生长体永不执行、progress 恒 0、pending 恒 Some → 该断言必败——测试
确实捕获原 bug。测试通过（21/21）。OK。

### 3. 侧向偏移分列 — ✅ 正确

- 路径 A（光标）：`ROOT_OFFSET=0.34`（=0.68×0.5，0x142ab31e0/0x142ab3200×0.5@0x142925870/8e0/e4）
  直接应用；`MIN_ROOT_DISTANCE_SQ=0.25`。
- 路径 B（随机）：`RANDOM_ROOT_OFFSET=0.68`，由 `get_bool` 选侧（true→−0.68/false→+0.68，
  @0x14094d6b0，`offset=RANDOM_ROOT_OFFSET*±1`）；`RANDOM_ROOT_DISTANCE_SQ=2.56`。
- 三轮"侧向 ±0.68 vs ±0.34 需对账"已闭合。OK。

### 4. 裁决项落账 — ✅ 确认

- **send_to_gpu 库内业务已闭**（upload_ivy_leaves/segments 8 函数：collect+pack+gpu_write+
  instance_count；48B 元素；SsboId 常量 rne-read 定案）。
- **GPU 提交/缓冲注册 = 库外 render 域接线**（需 GraphicsDevice/RenderGraph）。
- **SsboId::of 哈希 NAME≠原键 = ssbo_library 层共享机制缺口**（与 GroundedBuildingsSsbo 同源，
  非本模块引入，端到端命中需 ssbo_library 对账）。
- **待取证诚实**：GPU 48B↔104B 精确映射、噪声阈值 res+0x78 来源、grower 主闭包逐相语义/二级表
  0x142ab3bf4 目标、空间哈希格采样、max_ivy_grow_height 数值——均登记未伪造。

## 结论与 DISPATCH 建议

- **ivy = review_passed（限定）**：MUST-FIX（守卫闩死）已修复并经回归测试证实；侧向分列对账已闭；
  send_to_gpu 库内业务到位；编译+测试 1592/1。限定条件：GPU 提交待 render 域接线、SsboId::of
  建模待 ssbo_library 对账、主闭包逐相语义/48B↔104B/噪声阈值等按 RESTORE.md §四登记为待取证
  （不宣布模块 100%）。
- **garden = review_passed（限定）**、**sheep = review_passed（限定）** 维持。
