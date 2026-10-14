# resources/rt/bvh.rs

> 文件作用：**GPU BVH 构建**——`build_gpu_bvh(triangles, count)` 把三角数组构建成线性 BVH 节点数组（CPU→GPU 前的布局）。
> 输入三角 stride 0x28（40B），输出节点 stride 0x30（48B）；空输入产生 3 节点退化根，单输入跳过主循环。
> 产物供 GI/水的软件 BVH（rt_bl_bvh/rt_triangles/rt_triangle_meta SSBO，见 evidence/render-passes/INDEX.md §1）。

函数总计 5：业务 1 / 内联无独立符号 0 / 派生胶水 4

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::rt::bvh::build_gpu_bvh` | 0x140b0da50 | 48 | 分配 `count*0x30` 输出；count==0 → 写退化根 `{0,4,0,0,4,0,0,4,0}`（3 节点/9 u32）；count>=1 → 每 2 个输入三角（各 0x28B：位置×3+元数据）生成 2 个 0x30B 节点，逐 u32 重排包围盒/子信息 | O | roofs-rt/decomp/decomp-0x140b0da50.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 4 项（build_gpu_bvh::closure$0/1/2 + impl$1::default）

## 布局/结构要点

- 输入三角 40B：3×Vec3 位置 + 附加 u32；输出节点 48B：包围盒 + 子指针/计数（重排复制，两三角一组 2×0x50 进 2×0x60）。
- 交叉域：upload_rt_world（rt/mod.rs）上传 rt_triangles/rt_triangle_meta 供本构建消费；render-passes 确认软件 BVH（无硬件光追）。
