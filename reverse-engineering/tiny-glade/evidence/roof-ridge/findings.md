# 原作脊线与矩形面瓦源码还原

研究输入为原作 `tiny-glade.rne`，SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。没有启动游戏，没有修改发行目录。

## 已落实为 Rust 源码

- [ridge.rs](../../reconstruction/roof/ridge.rs)：脊线尺寸正向/逆向参数、局部尖顶中心、世界尖顶偏移，以及 `RoofRidgeDims::to_world_space` 的点/线段全部数值分支。
- [rectangular_faces.rs](../../reconstruction/roof/rectangular_faces.rs)：原作矩形屋顶的面瓦与底缘补片完整 CPU 数值链。原始 Rectangle/Roof 字节输入自动生成上下轮廓、曲线、行插值、两侧裁剪平面、面瓦位置/尺寸/朝向、随机数状态、序号、模式字段，以及 64 字节实例记录。
- 四边檐口条瓦、脊瓦段由对应模块另行组合；本文件的面瓦入口接受它们消费后剩余的 RNG/ordinal。这是显式阶段状态，不使用机器码结果作为面瓦算法的查表输入。

## 准确入口和验证边界

PDB module 3474 的 `assemble_rectangular_roof` 完整过程位于 `0x1421aeef0`，长 15785 字节，结束地址 `0x1421b2c99`（exclusive）。默认参考运行返回正常，生成 769 条实例：44 条侧缘、5 条脊瓦、648 条面瓦、72 条补片。Writer 调用边界分别为 `0x1421afbbc`、`0x1421b042c`/`0x1421b05f0`、`0x1421b1bf2`、`0x1421b1e8e`。

独立参考执行器 [capture_rectangular.py](../../scripts/roof-ridge/capture_rectangular.py) 执行整个原作过程与原作曲线、矩阵、随机划分、RNG、f16 打包指令。只桥接内存分配/释放、当前系统的 ucrtbase 数学导入与最终 Writer 观察边界，不伪造数值核心。捕获 Writer 参数属于只读观测，没有调用 SSBO/GPU。

| 源码验证项 | 原始机器码 | 差分结果 |
| --- | --- | --- |
| ridge dims/from/inverse/tipcenter/tipoffset/world 全链 | 原作过程及 orientation_mat2/x0y/点分支闭包 | 14776 个案例，0 差异 |
| Prepared face rows → 全面瓦/补片循环 | 整个原作矩形入口内实际面瓦子序列 | 46 个面，5670 个实例，全部 64 字节一致 |
| Rectangle/Roof 原始入口 → 自动曲线/行/平面 → 面瓦/补片 | 整个原作矩形入口内实际面瓦子序列 | 29 个屋顶，11591 个实例、最终 RNG 和 ordinal 全部一致 |

报告：[脊线](verification.json)、[面瓦循环](faces-verification.json)、[入口组合](composed-faces-verification.json)。入口组合包括 16 份原作历史参数快照，文件和 JSON 指针保存在报告里；墙形状/位置/朝向是独立构造的有效测试输入，因为 Roof 编辑记录通过 wall_index 引用其他墙形状数据。

## 实际算法要点

1. 脊线最小轴尺寸为 f32 `0.1`。指定轴的尺寸为 `ridge_length_01 * shape_dimension + (1-ridge_length_01) * 0.1`；另一轴保持 `0.1`。非矩形形状得到两轴 `0.1`。
2. 世界脊线在两轴最大尺寸 `<=0.1` 时返回单点，其余合法矩形返回两个端点。世界点分支保留尖顶偏移；线段分支原作实际乘以零，必须保留它对符号零/NaN 的影响，不能擅自改成预期中的偏移效果。
3. 高度具有依形状尺寸计算的 smoothstep 上限。ASM 为 `t*t` 后乘 `3-(t+t)`，Ghidra 的顺序表达不完全等价。
4. 面瓦根据世界剖面曲线长度使用 `ceil(length/0.4375)`，生成 `0..N-2` 的行坐标，额外重复最后一行以结束相邻行循环。最后一行的瓦长使用前一行间距，避免重复行的零间距。
5. 每行列数为 `max(3,ceil(span+span))`，通过原作 `random_splits` 划分。即使振幅是零，内部 RNG 仍被消费，后续瓦长与 UV 抖动依赖完整流状态。
6. 面瓦长度基于**局部**相邻行中心的三维距离；旋转后的世界坐标只用于实例位置和轴。先旋转再测距在数学上接近等价，f32 舍入却不同。
7. 世界位置采用 `center + (local_z*basis_z + local_x*basis_x)` 的 ASM 运算顺序。侧缝裁剪平面根据列起始 `u>0.5` 选左右平面，不根据屋顶行坐标选择。
8. 四个面按 gable_sides 跳过脊向轴对应的两侧；实例序号只为主瓦递增，底缘补片共享对应主瓦序号。

## 明确剩余范围

这些源码覆盖指定 CPU 数值过程，不等于完整游戏源代码。原始 Rust 结构字段名/类型仍以经过观测的字节接口表达。没有验证内存别名、完整错误展开、浮点异常标志、所有非法尺寸组合、原始 ECS 调度/事件调用者与平屋顶独立路径。三类矩形实例阶段的顶层组合由主任务进行整入口验证，不能仅把本模块的通过当作全部矩形过程验收。

## 复现

使用 `workspace/python/Scripts/python.exe` 运行 `scripts/roof-ridge/verify.py`、`verify_faces.py`、`verify_composed_faces.py`。它们调用本机 rustc 编译候选源码，并只读取原作指令/资源。Ghidra 数据库独立保存在 `workspace/ghidra-ridge`，不与其他分析器共享打开。
