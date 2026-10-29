# M01 图钉消费者与矩形相交依赖收口

## 结果和验证边界

已替换 `decoclutter_show_pins` 中的人工 pinned/geometry 表、固定零 delta、固定 `global_hit=false` 和仅发送显示意图事件的占位链。当前消费者读取真实装饰权威存储及派生变换，通过原几何分支和实际 Rapier 遮挡查询创建世界空间 UI 图钉，持续更新同一实体并完成淡出和删除。三个地表画笔直接写入同一个原事件队列。

当前候选的 `native-physics` 图钉测试 7/7，三种实际画笔 writer 各 1/1；原矩形完整函数及其原迭代依赖直接执行 5,111 例，与 Rust 判别/原 panic 结果无差异。矩形模块现有回归共 21/21。以上分别是候选 World 集成和原侧几何差分，**不是原图钉整函数与原游戏 ECS 的端到端差分，也不是整个 toolbar、GPU 视觉或整游戏验收**。

此前完成的常量页真实 GPU 回收修复另见 `m01-constants-page-retirement-2026-10-26.md`，两份验证不互相冒充覆盖范围。

## 原路径与现有重构归属

已检查 B 骨架对应文件；不新建业务目录，不恢复另一套原版工程。

| B 骨架原 owner | 当前 owner |
| --- | --- |
| `country-core/src/systems/ui_clutter/decoclutter_show_pins.rs` | 同路径，增量替换其占位消费者与事件 |
| `country-core/src/systems/garden/draw_garden.rs` | 同路径，写共享事件 |
| `country-core/src/systems/paths/draw_path.rs`、`paths/mod.rs` | 继续保留现有手工重构的 `country-core/src/systems/paths.rs`，不拆目录 |
| `systems/water/src/draw_water.rs`、`lib.rs` | 同路径，写共享事件及幂等注册 |
| `utils/src/geometry/rectangle.rs` | 仅恢复 `Rectangle2d::intersects` 及其独立回归；不覆盖另一 owner 的 `expand_along_edge` 等修改 |

`system-water` 新增的 dev-dependency 仅启用既有 `country-core/test-support`，用于实际 writer 的映射 SSBO CPU 测试；生产物理仍使用既有 `native-physics`。

## 真实链与原版规则

```mermaid
flowchart LR
  A[圆/矩形/自由墙与三画笔] --> B[同一 ShowPinned 事件队列最后一项]
  B --> C[DecoratorStorage 墙/地形/楼梯]
  C --> D[CachedDecoratorTransforms 与真实 ClutterMeshes]
  D --> E[扩大足迹/原几何/原墙发布状态]
  E --> F[MainCamera 与 Rapier mask 3 遮挡]
  F --> G[DecoClutterLock + Ui2dWsExtent + Ui2dImage]
  G --> H[GameTime/BackOut/宽限/淡出/despawn]
```

- **事件**：三个实际分支是 Circle、Rectangle、Wall。圆和矩形各有自己的闭区间与可选排除墙；不能将 28 字节 ShapeParameters 生搬到所有 union 分支。接口为 `for_wall`、`for_circle`、`rectangle`、`for_shape`，三画笔为 `brush_circle`。
- **画笔**：原水画笔 `141ba9555..141ba957d` 明确写 tag0、byte1=1、范围 `[0, cursor.y+1.5]`、无排除墙。花园/路径保留各自原半径与高度计算，直接写同一事件。三个旧公开名称只是同一类型的 `pub use`，没有独立队列；事件维护只能对该 TypeId 每帧更新一次。
- **候选装饰**：遍历 `DecoratorStorage` 的墙、地形和楼梯桶。原类型门为 `d = ty.wrapping_sub(0x38); d > 7 || d == 6`。实际变换来自 `CachedDecoratorTransforms`，缺失仍失败，不从装饰目标字段拼假坐标。
- **足迹**：真实 mesh bounds 扩张 `Vec3(0.5)`，方向来自缓存 quaternion 旋转 X 轴再归一化 XZ，平移使用真实 translation。原调用没有乘 scale，当前亦不乘。扩大、设置方向、平移三个原 helper 已检查，原码与 RNE 一致。
- **范围**：`141762b90` 不是三维矩形相交；原码只有两个闭区间比较 `a0 <= b1 && b0 <= a1`。不排序端点、不把两个标量改作中心/高度、不以 SAT 代替。
- **墙请求**：读取实际 `PublicWalls` 行，按其真实 outline 点逐项匹配。原墙行的 u64 key 占 8 字节，必须先扣除 key 才解释字段：freehand_ty==0 使用 `[point.y, metrics.constrained_extra]`；其余使用 `[0, max_y()]`。原点落在实际足迹内才算命中，缺失墙返回不命中，所需未初始化高度保持失败合同。
- **排除墙**：仅原 `DecoratorInfo.dst_tag()==2` 分支读取当前墙 owner 并应用排除 ID，不将所有属于该桶的装饰一律排除。圆 containment 标志控制额外的中心包含/中心距分支，之后保留原轮廓相交。
- **遮挡**：从真实相机 eye 指向缓存装饰位置；方向先取倒长度再乘。实际 `raycast_simple_with_max_toi` 使用 mask3、距离减 `0.2`，仅 miss 显示。未启用 native-physics 的实际请求会明确失败，不能将默认 miss 当生产成功。
- **ID**：所谓 ChunkId 是这个 consumer 中与装饰 ID 同值的 u32 命中/实体去重键。原调用没有 SparseInstanceBuffer 写口；`grow_one<ChunkId>` 的别名不能推导出 GPU SSBO 写入。
- **实体**：原 `DecoClutterLock` 是 24 字节状态组件，不是 ZST。字段为 ID、progress、base_extent、age、hiding。创建时 Ui2dWsExtent.extent 为零，图片名精确为 `menu/tod/locked`。
- **动画和删除**：age 每帧累加且命中不清零。未命中且 age>0.75 时改用 `-0.2` 淡出，否则用 `0.3` 淡入；保留原浮点 clamp 和 BackOut。仅 extent 更新，旧实体位置不凭猜测跟随重写。没有新事件时仍推进，隐藏且 progress<=0 才 despawn；图钉权威状态就在 ECS 组件，不存平行动画表。

有效 ClutterMeshTy 仍由已有枚举裁定。原门允许的 niche/非法值不被映射成任意默认 mesh；对不能解析的非法类型明确失败，而不伪造资产。

## 调度、卸载与中央集成

原 B `14065226c → 140ee2560` 创建 ClutterPlugin 的 `chain_ignore_deferred`；`140652271` 明确写 GameSystemSet `0x24`，`14065229f` 绑定集合，`140652310` 注册 Update。pins 是 tuple12 的第 10 项，在 autoclutter_animation 后、stitches/merlons 前。现有 ClutterPlugin 已保持这一链；形状 UI producers 的 set41 不应因此被移除，也不能再重复注册第二份 pins consumer。

原 ClutterPlugin OnExit 注册点 `14065238b` 的现有业务只清 InterShapeBrickStitches；未发现独立 pins OnExit 系统，不能编造其原归属。当前 pins 的正常不再命中/装饰删除路径已测试到实际实体删除。整场景退出的实体批量卸载仍随当前游戏 world 生命周期由中央集成验证。

中央已移除 startup_construction_runtime 的旧 Pinned* 假资源和 Toggle 事件初始化，保留同一 ShowPinned 事件。生产资产、相机、PublicWalls、碰撞世界仍由已有 canonical owner 初始化；consumer 只在真请求时检查这些已加载依赖，不生成成功默认资产。

## 原矩形算法完整恢复

`140c9b990` 的 MOVSD 是两个 f32 的搬运；算术全为 PS/SS。旧实现将其改作 f64、提前有限性拒绝并增加平行边修正，改变原语义。现恢复：原长度 guard → 原 f32 基底逆 → 两中心分别变换再相减 → 四条边的原 slab 参数 → 原 minnum/maxnum 和有序比较。第二矩形仍走原严格正面积 guard。

原完整 1002 字节函数及其原 itertools helper 在 x86_64/Rosetta 下直接执行；只桥接 core panic 出口记录 panic，不桥接几何、迭代或正常返回。5,111 个输入含随机旋转、尺度边界、平行/重合/包含、零、负值、NaN、无穷；有 16 个原 panic，Rust 均一致。源函数没有堆分配，当前不建立旁路几何实现。

**原版反直觉边界保留**：轴对齐且完全重合的矩形返回 false，完全包含轮廓也返回 false；部分边界会发生 0/0。新增 owner 测试最初错误地假定完全重合必为 true，已保留失败日志；以实际非平行相交输入验证 owner 排除，未放宽断言或换 SAT。fixture0 保留该原 false，后续不能再以直觉“修正”。

## 验证命令与证据

构建统一使用现有 target 和 `CARGO_INCREMENTAL=0`，无 git 提交、切分支、新 target 或整文件格式化。

| 验证 | 结果 |
| --- | --- |
| `cargo test --offline -p country-core --features native-physics --lib systems::ui_clutter::decoclutter_show_pins::tests` | 7/7，通过真正 Rapier 和 World；含三存储/共享 last、墙发布曲线、圆/矩形 owner 排除及高度、图片组件、持续动画、真实删除 |
| garden_drag_border_resume_commit_records_the_live_raster_once | 1/1，真实画笔/边界恢复/提交，断言 canonical 图钉事件真实字段 |
| path_begin_extend_commit_records_the_path_slot_once | 1/1，实际路径开始/延伸/提交，断言同一事件范围与分支 |
| system-water real_water_drag_writes_the_one_shared_pin_queue_and_original_height_interval | 1/1，实际水工具拖拽，两次读回请求、原圆心/半径/区间/标志 |
| `python3 reports/module-recovery-2026-10-23/probe_m01_intersects_native.py` | 原 RNE 5,111 例重放与已保存原输入/结果一致 |
| `cargo test --offline -p utils complete_original_intersects_and_iterator_match_results_and_guards` | 原函数输入的候选判别/panic 全一致 |
| `cargo test --offline -p utils --lib geometry::rectangle::` | 当前矩形模块 21/21，含其他 owner 既有回归；不计作本轮新增 21 项 |

原 patched B SHA-256：`cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`；实际执行 RNE：`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。pins、矩形相交、区间、三个 bounds helper、水事件写段共 7 段原码逐字节相同，见 `m01-pins-original-code-identity.json`。

完整 fresh C/ASM/函数元数据为 `m01-pins-native-evidence.json`、`m01-pins-fresh-decomp.c` 和本目录三个 ASM。原侧可复放探针为 `probe_m01_intersects_native.py` / `probe_m01_intersects_x86.c`，fixture 在既有 `tests/fixtures/native/m01-rectangle-intersects.json`。日志和相关源码指纹见 `m01-pins-validation.json`，原错误预期日志为 `m01-pins-shape-test-initial-failure.log`。

## 下一轮沿用的经验与未验收项

1. 先用真实 ABI 指令裁定标量/向量和 union 分支，再选 typed API。MOVSD、单态容器名称和 u64 key 前缀都不能直接当业务含义。
2. 不接受“逻辑事件存在”作为输出闭环：追真实 owner、碰撞查询、实体组件、渲染图片、动画、无事件持续帧和删除。
3. 原函数看似空 `void()` 或跳表被截断时，查机器码和已绑定版本；不把 decompiler 边界当业务缺失。
4. 原 PE 不能按 raw 文件偏移直接映射后当 RVA 执行。本轮首版探针因此在 id0 fault，没有产生有效比较；现按 PE sections 建立原 ImageBase 映像后重跑，计数均来自成功重跑。
5. 原数学和人的直觉冲突时，保留反例与原证，不用近似碰撞来过 UI 测试。

尚未声称完成：原 pins 整函数与原 ECS 的完整差分、GPU 图片像素/视觉/OS 输入、全游戏 Reload 批量实体卸载、正式场景性能与长稳。CPU World 测试用固定元数据和真实碰撞，画笔 SSBO 使用既有 CPU 测试设备；不将这些计作 GPU 或资产 loader 验收。原全部存档/历史/生产联合链由中央继续验收。
