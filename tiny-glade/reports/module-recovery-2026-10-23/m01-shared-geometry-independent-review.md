# M01 共享几何独立原指令复核（2026-10-26）

本轮新增既有模块内的差分测试、原执行夹具和证据文档。`Rectangle2d::lerp`、边界搜索及边界谓词由中央 agent 修改；独立复核发现跨平台 NaN 契约后，经中央明确授权，本 worker 仅修复 `expand_along_edge` 局部的异常浮点路径。没有修改全局数学库或新建源码目录。

## 原件与执行边界

原件为 `B版本内容/tiny-glade.rne`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。先从实时 Ghidra 读取三个函数及 `is_shape_inside`、`Circle2d::lerp`，再由 LLVM 直接反汇编此 RNE。保留 [反编译](m01-shared-geometry-native-contract.json)、[rectangle 汇编](m01-shared-geometry-rectangle.asm)、[search 汇编](m01-shared-geometry-search.asm)。

[probe_m01_shared_geometry_native.py](probe_m01_shared_geometry_native.py) 使用原 RNE 执行完整函数：

| 原函数 | 完整入口 | 业务覆盖 |
|---|---|---|
| `Rectangle2d::expand_along_edge` | `0x140c9c620` | 边选择、上下限 clamp、非法范围/下标 guard、原法向和中心更新 |
| `Rectangle2d::lerp` | `0x140c9c730` | 长度/点积、原 acos 近似、有符号最短角、旋转、尺寸中心插值、finite/单位轴 guard |
| `GladeBorder::search_lerped_shape_that_fits_into_glade` | `0x140ad2ce0` | 原循环、异类 guard、起点处理、最后接受值与输出写回 |
| `GladeBorder::is_shape_inside` | `0x140ad2970` | 原四路组合及原 circle/rectangle 几何调用，未注入边界谓词结果 |
| `Circle2d::lerp` | `0x140c98170` | 原 fmod、最短角旋转、半径和中心插值 |

仅在原 CRT 导入 `sinf@0x142923fd0`、`cosf@0x142923e80`、`fmodf@0x142923f10`、`ceilf@0x142923e60` 使用 Darwin libm。原 acos 多项式和所有业务浮点指令仍在 x86 原指令中执行。此边界可验证同宿主数学库的原算法，不能宣称 Windows UCRT 与 Darwin 数学库在所有输入上逐位相同。

私有 ABI 入口只提供已解析的原记录、边界记录和输出存储。输入 ShapeParameters 只用原有效 tag 0/1；没有把 freehand 的其他记录误当该枚举执行。栈初始化为 0xcd、输出为 0xa5。原 panic/panic_fmt/clamp panic 入口仅用于观测 guard，未替换成功算法。

## 原 search 的确定结论

原循环检查 `t=0/20, 1/20, …, 19/20`，最多 20 次，不采终点。`0x140ad2e13/0x140ad2f73` 将计数转成 f32，再除实际常量 20；末端与 `0x14` 比较。没有二分或“先检查 after 再内插”。

起点检查失败：

- 进入函数时 `0x140ad2d0b..0x140ad2d19` 已把 **before 的全部 24B payload** 保存至栈 `+0x50/+0x60`。
- t=0 生成的 candidate 放在另一块临时存储；rectangle 的 `0x140ad2dc0`、circle 的 `0x140ad2f27` 失败均直接跳 `0x140ad2ff5`。
- 输出 tag 来自最初 before；`0x140ad2ff8..0x140ad3006` 写回最初保存的 24B payload。因此起点失败必须返回 before 的原 **28B 完整记录**，包括圆的尾部哨兵和负零，不能重新生成 t=0 candidate。
- 第一次成功后，原代码才用 candidate 替换 accepted；后续首次失败返回上次成功值。异类 circle/rectangle 到 `0x140ad3029` 并调用原 panic_fmt。

夹具共 **465** 个 search 样本，其中 **79** 个真实执行满 20 次边界检查，**234** 个起点失败输出与 before 的 28B 精确一致。输出字段与每次原 factor 位型一并记录；成功圆结果的尾 8B 没有原初始化写面，测试只比较定义字段，避免把栈残留当业务。

## 数值夹具与候选结果

完整 [夹具](../../tests/fixtures/native/m01-shared-geometry.json) 与 [原执行统计](m01-shared-geometry-native.json) 均记录 SHA。候选是 macOS ARM64 debug；未使用候选算法生成 oracle。

| 函数 | 原执行样本 | 当前候选结果 |
|---|---:|---|
| rectangle lerp | 1427 | 1427/1427 完整定义字段逐位及 panic 一致 |
| expand | 2612 | 2612/2612 完整字段逐位、guard 与 panic 前部分写入一致 |
| search | 465 | 465/465 完整定义字段逐位、异类及原 guard 一致；234条起点失败完整28B返回 before |

lerp 覆盖九组轴向（四象限、±0、.6/.8、一般旋转）、负/零/超过 1 的 factor、NaN/无穷、512 个固定种子随机旋转、原 size/axis guard。原导入 sinf/cosf 的输入/输出位型也存于每行；同 Darwin 数学库边界下没有发现 glam 0.29.3 `angle_to` 运算次序、sin_cos 与原独立 sinf/cosf 结果的差异。

expand 覆盖四条边、负/零/正增量、原最小值和最大值、相等边界、无穷边界、非法范围、NaN bounds、非法下标及 panic 前已经发生的字段写入。delta NaN 的传播和已有 NaN payload 没有被默认值替代。

## 严格失败定位、工具交叉验证与局部修复

最初 1458 例发现 40 个 expand 不匹配，均为无限位移与零法向分量相乘后新生 NaN：原 SSE 为 `0xffc00000`，ARM Rust 为 `0x7fc00000`。首例 index=3：axis=(1,0)、edge=0、delta=-∞、min=-∞、max=+∞；center.y 的原结果为负 indefinite NaN。没有放宽为 `is_nan` 或 ULP 容差。

为核准输入 payload 顺序，扩展至 quiet/signaling NaN、正负 payload、old size、axis、center 的组合。此时独立发现 **Unicorn 的双 NaN 传播不等于直接 x86 指令执行**：它会偏向更大的 payload，不能据此把恢复算法改为“挑最大 NaN”。

[probe_m01_expand_x86.c](probe_m01_expand_x86.c) 是工具 harness，不是算法重写：在 Rosetta x86_64 下按原基址映射同一 RNE 的 PE 内存映像，通过 pefile 按 PE sections/RVA 预展开映像（不能直接 mmap raw RNE 文件），校验原入口字节后，用 Win64 ABI 直接调用原 `0x140c9c620` 全函数，读取原记录输出；原 expand 不需要 CRT，**无数学或几何 helper 桥**。最终 **2592** 条非 panic 样本全部直接执行；**20** 条原 panic/部分写入路径仍用 Unicorn 的原 guard 入口观测。两种原执行在 **96** 个双 NaN 样本上不同；原 Unicorn 输出与直接原执行输出均保存于 [差异文件](m01-expand-rosetta-unicorn-differences.json)，fixture 在这些行额外保留 `unicorn_after`。不使用 Rust 候选输出修正 oracle。

原 x86 直接执行确认了加减乘的 NaN 选择顺序：先左操作数、再右操作数，并 quiet signaling NaN；当双方都不是 NaN 但运算生成 invalid 时使用 `0xffc00000`。修复只在结果实际为 NaN 时进入局部 cold 路径，逐操作重放原 `delta + old`、差值、法向、位移乘法及最后中心加法；保留输入 NaN 的符号/payload 和原操作数顺序。正常有限路径仍使用原向量算术，不分配内存、不调用外部数学库。没有对最终记录做笼统 NaN 归一化。

最终 **2612/2612** 严格通过；[最终差异文件](m01-shared-geometry-expand-mismatches.json) 为空。修复前的完整差异与失败仍保存在 [修复前差异](m01-shared-geometry-expand-mismatches-before.json)、[修复前日志](m01-shared-geometry-expand-before-tests.log)。这也是工具失真被直接原执行纠正的记录，不能把 Unicorn 96 条错误结果当成候选业务缺陷。

## 另一个有限边界差异：平方距离不能替换为 sqrt

`GladeBorder::is_shape_inside` 圆包矩形调用原 `Circle2d::contains_point@0x140c980f0`；原汇编是 `distance² < radius²`，不做 sqrt。候选先前使用 `is_point_inside` 的 `sqrt(distance²) < radius`，会因舍入改变 t=0 判定。

明确区分夹具：圆边界 radius=10；矩形 size=(1,1)、center=(5.5,7.499999523162842)，最远角为 `(6,next_down(8))`。其距离平方为 `99.99999237060547 < 100`，原判定为真；sqrt 的 f32 结果舍入成 10，错误判定为假。after 取矩形中心 (0,0)，原算法能检查满20次，错误谓词在 t=0 返回 before。已交中央修复为真实 contains_point，并列入第465条完整 search 原夹具。

## 命令与日志

```sh
/Users/yuluo/Desktop/game/tools/venv/bin/python tiny-glade/reports/module-recovery-2026-10-23/probe_m01_shared_geometry_native.py
cd tiny-glade
CARGO_INCREMENTAL=0 cargo test --offline -p utils --lib original_rectangle_lerp_complete_function_matches_native_bits_and_panics
CARGO_INCREMENTAL=0 TG_M01_GEOMETRY_DIFF_OUTPUT='/Users/yuluo/Desktop/game/tiny-glade/reports/module-recovery-2026-10-23/m01-shared-geometry-expand-mismatches.json' cargo test --offline -p utils --lib original_expand_along_edge_complete_function_matches_native_bits_and_panics
CARGO_INCREMENTAL=0 cargo test --offline -p country-core --lib original_search_complete_function_preserves_steps_guards_and_before_bytes
```

[lerp 日志](m01-shared-geometry-lerp-tests.log)、[expand 日志](m01-shared-geometry-expand-tests.log)、[search 日志](m01-shared-geometry-search-tests.log)。初次 country-core 测试先后遇到其他 owner 正在整合的 ShowPinnedDecoClutter API、circle 测试闭包类型、move API、rectangle 角点测试字段错误，已按 owner 协调修复，最终实际执行通过，未修改其文件。磁盘一度 ENOSPC，由中央清除已停机的共享 incremental 缓存后继续；未把环境故障当成原业务失败，后续统一 CARGO_INCREMENTAL=0。

## 下一轮复用经验

- 全函数调用比孤立内插公式更能发现“失败返回 before”与完整字节保留策略；候选重建的 t=0 数据不等于原 before，尤其是负零、NaN 和未使用载荷。
- 对原 enum 记录的定义字段与未初始化 padding 必须分开验收；只在原明确保留字节的路径比较完整记录。
- 原函数内使用相同依赖版本还要实际跑指令：acos 近似、叉积符号、clamp 源操作数顺序和 sin/cos 依赖边界都可能影响分支或最后接受步数。
- IEEE NaN 的分类一致不自动等于跨平台位型一致。NaN 位型必须区分输入 payload、操作数优先级与新生 invalid。先保留严格失败，再核验原指令；不能用宽容比较制造“全绿”。
- Unicorn 并非所有浮点边界的最终权威；双 NaN payload 需要独立直接执行原指令交叉验证。本次原 RNE 在 Rosetta 直接执行验证了 96 条模拟器偏差，不能迎合模拟器把业务写错。
- 看似等价的平方距离和 sqrt 比较会在 f32 边界改变搜索分支。要设计能让 t=0 后续输出不同的夹具，避免首步返回同一形状掩盖错误。
- 并行整合测试的编译阻断先按文件 owner 协调，保持自有业务证据推进；不得为跑单个测试而修改其他 agent 的 API 或恢复旧壳。

验证目标是 aarch64-apple-darwin、Rust 1.98.1、debug；源码（含窄函数块）、Cargo锁定、原件/工具/fixture/日志与候选二进制 SHA 记录于 [m01-shared-geometry-verification.json](m01-shared-geometry-verification.json)。不包含 Windows UCRT 全域等价、原游戏完整调度或GPU验收。
