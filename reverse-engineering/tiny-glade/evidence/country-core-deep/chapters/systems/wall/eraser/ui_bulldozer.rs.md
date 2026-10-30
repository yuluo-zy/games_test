# systems/wall/eraser/ui_bulldozer.rs

> 文件作用：推土机 UI 入口：wall_eraser_bulldoze_condition = AppMode{tag==9, +4==3}；handle_decorators 排空墙属装饰 + terrain/stair cull；can_reconnect_stairs_if_midpoint_deleted；StairsState::disconnect_edge；命中到 AnimateWallBulldozeCmd 入 buffer（RoofDelete FIRST + WallCompound StateChange+Delete 双组）。

函数总计 14：业务 3 / 内联无独立符号 2 / 派生胶水 8

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::eraser::ui_bulldozer::wall_eraser_bulldoze` | 0x1409de190 | 0 | wall_eraser_bulldoze（装饰排空 + 楼梯断开 + RoofDelete FIRST 双组） | O | ui-input\decomp\decomp-0x1409de190.txt, wall\decomp\decomp-0x1409de190.txt |
| `country_core::systems::wall::eraser::ui_bulldozer::wall_eraser_bulldoze_condition` | 0x1409de170 | 49 | wall_eraser_bulldoze_condition（AppMode{tag==9,+4==3}） | O | ui-input\decomp\decomp-0x1409de170.txt, wall\decomp\decomp-0x1409de170.txt |
| `country_core::systems::wall::eraser::ui_bulldozer::hammer_cursor_icon` | 0x1409e35a0 | 933 | UI 交互 | I | ui-input\decomp\decomp-0x1409e35a0.txt, wall\decomp\decomp-0x1409e35a0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::eraser::ui_bulldozer::can_reconnect_stairs_if_midpoint_deleted`（行 817）— 谓词/校验
- `country_core::systems::wall::eraser::ui_bulldozer::handle_decorators`（行 836）— 系统:事件响应

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 8 项

## 实现方式（算法级）

### wall_eraser_bulldoze_condition 0x1409de170（O，机器码）
- `48 8b 01 83 38 09 0f 94 c1 80 78 04 03 0f 94 c0 20 c8 c3` = `(*app).tag==9 && (*app).byte@+4==3`（AppMode 判别 u32@+0 + 子判别@+4）。✓ DIGEST。

### wall_eraser_bulldoze 0x1409de190（O）
- **光线拾取**：`FUN_1408ce180(&hit_type, 光标资源, &ray, 0)` → 按 `hit_type` 分派：
  - `0x11`（树/装饰）→ `FUN_140d79f90`（spawn `cursor_hammer_` 前缀 signifier，UNK_142ac4080）+ `FUN_14146c880`（写事件，装饰删除，`delete_tree_` 语义）。
  - 墙型（0,1,2,6,7,8,9）→ 扫描 **AnimateWallBulldozeBuffer**（0x98 步长匹配 `+0x70` WallId）防重复。
  - roof/stair（4,0xd,0xe,3,0xa..0xc）→ 解析 roof 实体（walls 数组 0x14 步长 → roof map 0x48 步长，同 roof_entity 链），3D 距离 `fVar20=SQRT((p-hit)²)`（初值 f32 max 0x7f7fffff）。
- 命中判定：`hit 存在 && distance<=best && 非已动画中` → `uStack_100=2`（推土目标）；`FUN_1409236d0(资源, &"bulldoze_canvas", 0xf, ...)` 注册画布 signifier。
- **尾跳表**：`(*(&UNK_142ac4544 + *(int*)(&UNK_142ac4544 + uStack_100*4)))()` —— 按 uStack_100（0..5）分派动作（RoofDelete FIRST / WallCompound StateChange+Delete / 装饰排空 / 楼梯断开，I：具体 handler 在跳表目标）。
- **panic**：实体索引越界 → fmt panic（UNK_142ac2ed0/142ac2eb0，line 0x2b）+ 多处 `w.simplified_curve_2d.is_valid()` 断言（UNK_142ac4100/142ac3c60/142ac3ca8 三个文件指针=ui_bulldozer.rs）。

### hammer_cursor_icon 0x1409e35a0（I）
- 锤子光标图标系统：UI 交互层，4.4KB，未逐行。

## 2026-10-30 M06 bulldozer 原字节绑定与分支复核

原静态 RNE SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`；PE unwind 给出 `wall_eraser_bulldoze` 半开区间 `1409de190..1409e2414`，区间字节 SHA-256 `fbba82097469d339feb74cebc5680c26c57e4bb46f0265505ff2a7d0d2e1f19a`。独立 Ghidra worker 的 patched B 身份是 `cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`，该区间校验与原字节一致；两跳表 override 后反编译成功，但后端 checkpoint 失败，不能声称数据库修改已保存。全输出是工作区 `tmp-merge/m6-bulldozer-corrected.c`，原始指令是 `tmp-merge/m6-bulldozer-original-disasm.txt`。

首表 `142ac4508`：0–2、6–9 墙；4 装饰；5 楼梯点；13 楼梯段；14 支撑；3、10–12 及其余未分派值走无目标；17 为未命中。动作表 `142ac4544`：0 墙 `1409deb49`、1 装饰 `1409def6c`、2 树 `1409df103`、3 楼梯段 `1409dedc4`、4 支撑 `1409df26a`、5 无目标 `1409df72e`。demo 模式跳过树候选扫描；普通模式每棵树均与最初射线距离比较，`1409de91c..983` 不更新该距离。

墙 hover `1409e08f0..a07` 使用 Public `+0x88` 预处理 Curve3 和 `+0xe0` 高度数组。既有同一 owner 的证据为 `m01-height-recovery.md` 第 17、33 行及 `m01-anchor-follow-recovery.md` 第 24、50 行；`get_wall_attachment_height@140b2a890` 也同时读取这两槽。当前 `height_outline()` 指向 `PreparedWallPath.positions`，而 `arch_segment_heights()` 指向同一出版流程的高度缓存。迭代器 `14130abb0` / `14157f730` 按 3 的步长取有效 zip 项，逐项写原点与 `SwizzlerXVY::xvy@140c9cde0` 生成的 `(x,height,z)`，再由 `DebugLine::show@140b04f70` 绘制红色 `(0.8,0,0)`、宽 3 的虚线。回归 `hover_preview_uses_prepared_curve_and_nonuniform_published_heights` 使用非均匀地形和非空路径资源，要求真实 prepare 改变点数，逐项检查点和高度来自同一 owner；该回归尚待中央运行。

墙删除 `1409e197e..19bc` 先归档墙桶，`DecoratorStorage::remove_storage@140a1dba0` 再移除墙桶；屋顶历史先于墙 compound。楼梯中点 `1409e0d8f..e6f` 先移除点、连接邻居，再发 12B `HandleStairAttachedDecoratorsOnStairPointDeletion` 和节点删除事件；前者由 `handle_external_decorator_cmds@14081a380` 第三事件流消费。非中点 `1409dff4a..1409e1818` 选中边、收集原图度数 1 的端点、断边后删除节点。支撑动作写 SupportsDelete 并发 kind 0x11 音效。原 `1409e35a0..3866` 构建 60×60 `hammer_cursor`，bundle 类型字符串分别确认 `UiInactive` 和 `Ui2DFollowMouse`，无 CPU 比例乘算。

当前这些生产路径和夹具的编译/运行、实际 App 调度与渲染均以中央后续验证为准；静态证据不是游戏行为验收。
