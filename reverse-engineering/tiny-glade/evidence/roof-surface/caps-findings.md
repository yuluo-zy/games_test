# 矩形屋脊盖瓦 Rust 源码

[rectangular_caps.rs](../../reconstruction/roof/rectangular_caps.rs) 实现原作 `assemble_rectangular_roof` 的 `0x1421afc02..0x1421b062d` 子循环。入口 `assemble_caps_from_observed(rect24, roof88, special_mode, &mut rng, &mut ordinal)` 从原始矩形和 Roof 对象字节自主计算端点、点/线脊线分支、延长量、分割、姿态、尺寸、位置和 64 字节实例记录。

它不是仅接受提前计算端点的实现。端点依据原作矩形前置 context、原始展开顶部尺寸及高度在 Rust 中独立构建。RNG 和序号是整个铺瓦过程的共享状态，应接在四边檐口生产器之后；父工程再接主体面瓦生产器。

## 原始算法事实

1. `RoofRidgeDims::from/to_world_space` 判断是否存在非点脊线。点脊线跳过盖瓦循环。
2. 从**未经过 gable 维度修正**的展开顶尺寸选择较长轴，乘半长，在顶部局部中心两侧构建端点。顶部矩形 frame 及原始输入矩形变换确定世界 XZ，高度由原 Roof::height 给出。直接使用修正后的 top_rect 尺寸会产生错误端点。
3. 当原 `is_gable` 为真时，沿端点方向延长：`roof[0x54]==1` 时每侧 `0.56`，其他值每侧 `-0.28`。保持原 f32 操作顺序。
4. 盖瓦分割边界数为 `max(round(length/0.625),2)`；随机扰动幅度为 `0.125/length`，调用还原后的原 `random_splits` 和共享 fastrand 状态。
5. 由脊线方向和水平正交方向计算原 Quaternion，保留条件分支和 SIMD lane 顺序。正向分支曾在初版中把两个 lane 写反，原机器码比较发现后已修正。
6. 横向瓦宽受 `profile_01*height_01` 的 smoothstep 影响，为 `0.4+0.15*smooth`；纵向为每段长度乘 `1.4`，厚度为 `0.15`。位置是各分段的中点，再沿 Y 下移尺寸相关偏移。
7. 实例字段使用原 f16 转换；第一个及最后一个盖瓦为模式 2，special_mode 时全部模式 2；其余模式 1。盖瓦记录 `+56` 的标记为 0，与普通面瓦的 1 不同。

## 校验结果

[caps-verification.json](caps-verification.json) 保存 22 个原始 Roof/rectangle 输入、原函数完整执行期间捕获的端点和共享状态，以及 107 个盖瓦记录。

独立 Rust 从输入计算的 22 组端点与原指令的 f32 bits 一致；之后生成的 **107 个完整 64 字节记录、终止 RNG 状态和序号全部一致**，无容差比较。方向、gable 风格、不同脊线长度和来自原作 history 的参数均包含在输入中。原 history 参数配合明确的合成墙体矩形，未进行游戏场景回放。

新鲜原始捕获命令：`workspace/python/Scripts/python.exe scripts/roof-surface/verify-caps.py`。快速再次校验已保存的完整原始机器码捕获：同命令加 `--cached`；该模式仍重新编译和执行 Rust，报告标明未在这一次重采原机器码。

Oracle 调用原矩形函数入口直到 return，数值 helpers 全部使用原 RNE 指令，桥接只有分配/释放、实际 host CRT 数学 API和 Writer 边界捕获。Writer 的 GPU 上传和渲染不属于本源码校验范围。零长度/非有限几何、原分配失败和 panic 日志仍未覆盖。
