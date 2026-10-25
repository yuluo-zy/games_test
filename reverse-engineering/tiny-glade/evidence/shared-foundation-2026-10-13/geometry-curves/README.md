# utils 基础几何/曲线交付（B）

恢复的是原游戏业务规则，原依赖继续直接复用。逐文件、函数、规则、实际原码对照、缺口见 [delivery.json](delivery.json)。19 个原 B 文件逐项登记；不是 19 个文件全部完成。`curve_u_circular.rs` 目前只有证据说明，不算业务恢复；C 专有 `geometry/brace.rs` 不套用 B 数据库。

源码在 `reconstruction/shared-foundation/crates/utils/src/{components,curve,geometry}/`，接入现有 utils crate。父 agent 维护全局进度表和 workspace 清单，本组没有修改这些文件。

## 已实际核对的规则

| 规则组 | 原入口输入 | 对照范围 |
|---|---:|---|
| Curve2 构造 | 41 | 弧长和逐点 u 的 f32 位值 |
| Curve2 查询 | 各 328 | coord / position / tangent；近点 coord / u；逐位对照 |
| Curve3 构造 | 80 | XYZ 与 XZ 两类弧长；XZ 度量不等于仅比较 XZ 重复点 |
| 曲线错误 | 7 | 点数不足、零/非有限长度、重复边错误 |
| Transform | 100 + 90 | look_at 保留平移和缩放；abs_diff_eq 容差及 NaN |
| 曲线裁切 | 40 + 20 | 保留内点、准确端点、跨 0、同 u、循环接缝和错误 |
| 射线交叠 | 82 + 94 | 三角形双面/边界/1e-5 阈值；球切线、负根和非单位方向 |
| UV → position | 66 | 第一匹配三角形、包含边界、原权重加法顺序 |
| Rectangle3d | 50 × 4 入口 | 点包含、extent、八点、expand；top 排除 |
| Circle | 80 × 4 入口 | position / normal / tangent / closest_u |
| Rectangle2d | 80 × 2 入口 | perimeter position；包含接触的 SAT overlap |
| Threshold crossing | 30 | **仅分类与 crossing 事件投影**；拦截 RowTrimmerSink 回调，不算完整 span 消费链 |
| AABB3 | 各 70 | ray 分类、segment；平行轴、边界、零方向、盒内起点 |
| Plane / Line / Barycentric | 各 70 | 未归一化法线和 min_t、闭线段关系、重心权重 |
| Triangle–triangle | 257 | 类别严格对照；68 段按原**有序端点**核对，端点误差限 3e-5 |

Triangle 类别分布为 1 Point、68 Segment、3 Coplanar、185 Disjoint。3e-5 是明确保留的数值容差，不声称三角交叠端点逐位一致。Coplanar 是原返回类别，不能当作有限三角形一定重叠。

## 实际差异 → 修复

- Circle tangent 原作是 `(-sin, cos)`，和递增 u 的采样方向相反；原候选两个符号均错误，已修复。
- Circle 无法归一化的位移返回 `0.0`，原入口没有 `Option`；去掉候选额外的 `None`。
- 循环裁切去除尾部接缝后，还要检查新的尾点与右侧首点重复；已按原规则修复。
- Sphere、UV 的 f32 加法顺序与候选不同；原入口差分发现后修复。
- Transform 的 SIMD `cmpnle` 不能简化成 `epsilon < difference`，NaN 应失败；已使用原 glam 比较 API。
- Threshold 对 NaN 样本走 above 分类；用有序 `threshold < next` 会漏 crossing，已修复。
- Triangle 输出顺序需要原异侧顶点重排和 winding 定向。已恢复该规则，比较从无序端点升级为有序端点。
- Transform `r@0,t@16,s@28` 的原实例观察纠正了旧注释；这只是表示证据，不计业务验证。

## 依赖复用与归属

原锁 `utils 0.1.0` 依赖段为 `tracking/original-Cargo.lock:8100`，直接包含 glam 0.29.3，没有 triangle collision 依赖。PDB 指向私有 `utils::geometry::triangle_triangle::*`；机器调用为私有 outer → cross → orient，并委托 `glam::Mat4::determinant`（PDB public 地址 `0x1412b7c50`）。恢复只实现私有分类、剪裁、端点及阈值策略，行列式、矩阵转四元数、normalize/try_normalize、通用向量长度和容器都调用原依赖。

工具使用独立 Windows worker `foundation-geometry-curves`。冻结 B seed 保留原码 SHA；有限函数 body 准备不猜 Rust ABI。80 个 curve 入口的批量准备/反编译约 6.6 秒。查询本身不计业务恢复。

`native_oracle.py` 只在 Unicorn 内运行校验过的原 B 叶子代码，提供独立 heap/stack。公开 allocator、memcpy 和 CRT math 由受控桥接处理；Circle 测试委托本机 `ucrtbase.dll` 的原导入数学函数。没有启动整个游戏。Python harness 与生成的 Rust fixtures 是验证设施，不是游戏源码。

## 剩余范围

优先继续曲线 resample（含每段 remap）、smooth/weighted smooth、状态化 CurveSlice extends；再补 Rectangle closest-edge/closest-u/intersections/circular-slice 和 Circle 曲线转换。Ray 的鼠标屏幕输入生产者、完整 RowTrimmerSink 消费链、更多退化/极端/NaN 几何输入仍未闭合。每项缺口已按原文件写入 JSON，不以目录都有文件推算完成率。

重跑：`py -3 native_oracle.py`、`geometry_oracle.py`、`boundary_oracle.py`、`edge_oracle.py`、`linear_oracle.py`、`triangle_oracle.py`；然后从 D:/game 运行 workspace 的 `cargo test -p utils --locked --offline`。本组 19 个原码 fixture 测试和当前 utils 全 37 测试通过；strict Clippy `--all-targets -D warnings` 通过。
