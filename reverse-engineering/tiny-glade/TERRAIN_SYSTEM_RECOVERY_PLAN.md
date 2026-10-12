# 完整地形系统逆向：本会话指导与执行计划

更新：2026-10-11。仅约束本地形专题；其他会话继续各自工作。不修改两级 AGENTS.md、全局依赖政策或共享项目优先级，不把地形设为全项目唯一重点。

## 目标与当前证据

恢复 Tiny Glade 的游戏自有地形业务完整链：输入/选择 → stroke 新建、绘制、编辑、移动与已确认的删除行为 → 求值/栅格化 → 读回/缓存/脏变化 → 建筑、碰撞及直接地面消费者 → 历史事务 → 保存/加载/版本迁移。

B 为当前实现/验证基线，C v1.16.0 为计划交付目标，A 辅助追溯。已有 sampler 和建筑 dirty 消费者仅是局部恢复；不能把读取高度网格当作已经恢复编辑写入。首批15个B函数/69段原RNE核查见 `evidence/terrain-editing/findings.md`。第二批6函数/12段核查及私有helper取证见 `evidence/terrain-editing/second-batch-findings.md`：Edit replay与resolved Delta各512个有限数案例通过。尚无完整编辑事务、Rust地形模块或端到端系统验收。

## 强制归属与依赖筛除

每个函数进入深入分析或实现前：
1. 查询 `tracking/dependency-exclusions.json`，对照 `evidence/multi-version/{A,B,C}-dependencies.json`、`*-sources.json`、各版 Cargo.lock 与 PDB 定义模块、真实调用点。A 缺锁文件时用其 PDB 来源，不从其他版本填造版本。
2. 已确认的 std/core/alloc、公开 Cargo/git 依赖、Bevy、glam、fastrand、half、serde/rkyv/ron、容器、allocator、公开 noise/math 库直接复用原版 API；标记 `dependency_reuse`，不生成替代源码，不再列为核心恢复待办。
3. 泛型名含 Terrain/Stroke 不构成业务归属。游戏字段/枚举 schema 属于业务格式，serde/rkyv 编解码器实现仍是依赖。内联混合时只恢复游戏参数、阈值、调用顺序与业务通知，不重写库展开。
4. 对比三版的包版本/commit、PDB 路径和源校验和、类型、常量及业务调用。库版本/编译器/内联导致机器码不同，不推翻已确认依赖归属；私有源跨版相同，也不自动排除为库。
5. local/path/private 包不自动代表游戏业务。归属不明记录 `unresolved`，不得靠命名或反编译器标签实现。公开/已确认引擎仍复用，只查影响地形业务的必要分叉差异。
6. 每次实现必须附原作归属证据和调用/字段证据。依赖分类、脚本数量、反编译数量均不是系统完成度。

## 工具与并行边界

- 复用持久数据库，目标查询按 `procedure_info`/caller → `batch_decompile` → 必要指令范围进行，不重分析全程序。
- B 的 Ghidra 数据库绑定 patched EXE，实际代码核查和数值 oracle 使用原 `tiny-glade.rne`。A/C 当前数据库未绑定；精选函数清单漏项不代表版本缺少功能。
- 不修改原二进制、共享数据库注释/类型、MCP/权限配置、共用脚本或其他会话证据。新报告/脚本限定本专题路径；运行工具前检查副作用、锁和默认输出目录。
- 只写 `evidence/terrain-editing/`、`tracking/terrain-tasks.json` 和本文；共享进度文件暂不改动。若这些专题文件也被其他会话更新，先检查现状，不覆盖。
- 不将 compiled-shaders/*.bin 直接当 SPIR-V：工具记录说明其封装魔数不是 SPIR-V，需要先识别实际格式。

## 首批实际取证

B 地址均为静态 VA（基址 0x140000000），是调查锚点而非已确认调用图。

| 顺序 | 入口 | 目的 |
|---|---|---|
| 1 | edit_terrain_stroke `0x14094e000`，PDB 长度4814 | 命令/字段写、clamp/no-op、变化通知、pending history |
| 2 | ui_edit_terrain `0x140aef1f0` | 实际事件 producer、height/falloff、坐标语义 |
| 3 | new_stroke `0x140924a10`、get_last_mut_stroke `0x140924900`、insert_or_morph_last_point `0x140a97880` | stroke 生命周期与点列维护 |
| 4 | history_record_terrain `0x140ac9770`、reapply/revert `0x1408592d0`/`0x140858d30`、replay forward/backward `0x140994f30`/`0x140995040` | deferred commit、字段恢复和正反向消费者 |
| 5 | rasterize `0x1408c7120`、on_terrain_modified `0x140aabdc0`、delayed message `0x140aabdd0`、read_terrain_data `0x14098c680`、copy_from_gpu `0x1409ea020` | 权威 stroke/GPU/CPU 网格 owner 与同步顺序 |

优先判定 512×512 u16 网格是否是 GPU 结果的 CPU 读回副本，不先假定 CPU 写入算法。保存实际调用返回、失败、目标身份与查询参数；仅真实 CALL/数据流/调度注册证据支持的边记 confirmed。若连接不可用，明确阻塞并继续可用的原始 PDB/指令证据，不伪称取得反编译。

## 完整任务与依赖

任务细节见 `tracking/terrain-tasks.json`。

- T00：三版函数归属、依赖筛除、地址映射。
- T01：TerrainEditingData/Stroke/Edit schema、运行时布局、bounds/fix 来源、GPU/grid owner。
- T02：输入/射线/选择/绘制/创建，实际命令与坐标空间。
- T03：编辑/移动/删除及资源清理；New/Edit/Move 来自真实历史，删除另取证。
- T04：deferred history、reapply/revert、undo/redo、cursor/replay；schema 提前，完整 snapshot 后置。
- T05：游戏求值、falloff、重叠次序、编码/栅格化。未发现独立 smooth/flatten 时登记未确认，不虚构功能。
- T06：GPU dispatch/readback、derivative/cache、dirty generation、异步时序。
- T07：地形编辑源 → dirty tile/membership → 建筑重发布、楼梯/碰撞/附着更新。
- T08：直接地形消费者（road/water/rock/vegetation）；独立系统不混算地形完成度。
- T09：保存/加载/snapshot/迁移与缺失引用处理。
- T10：C 迁移、A 对照及完整验收，不能用 B 局部报告代验三版。

T00→其余；T01/T02→T03/T04；T01/T03→T05→T06→T07/T08；T01/T04→T09；上述→T10。反编译可交错推进，但实现和验收按已确认数据依赖推进。

## 复用证据与验证

复用 `reconstruction/building/wall_collection.rs:10-68` 的 sampler、`:192-228` dirty predicate，以及 `wall_spatial_index.rs` 的 membership/清理，不重新实现公开依赖。

复用 `evidence/building-collection/findings.md`、三版 terrain/spatial oracle 报告、`reconstruction/history/findings.md:86` 和真实 history.json 的 New/Edit/Move；index/id 不合并。旧报告是既有局部证据，不声称本轮重跑。

状态分别登记 `located`、`analyzed`、`implemented`、`verified_local`、`system_verified`。缺失信息、失败报告、ABI/harness 和跨架构浮点边界不隐藏。

最终验证：New→Edit/Move/已证删除→实际 terrain 求值/读回→dirty→建筑/碰撞→undo/redo→save/reload。检查空/重复点、缺失对象、同值、极值、NaN/Inf、边界、重叠与旧 cache/membership 清理。不能以手工 SetTerrainGrid 注入代替真实编辑源。

源码/业务状态优先，GPU 视觉与场景最终验收后置；但影响权威状态的 GPU 计算/readback 契约必须提前查明。布局/算法证据充分后再恢复 Rust 业务模块；完整地形系统只有最终验收通过才标完成。
