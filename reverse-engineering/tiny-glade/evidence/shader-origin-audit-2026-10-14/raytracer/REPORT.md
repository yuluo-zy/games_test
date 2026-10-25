# GPU-Raytracer 原始资料核查（2026-10-14）

结论：Tiny Glade 原作明确声明使用 Jan van Bergen 的 GPU-Raytracer 1.3.0；原版四个 SPIR-V 模块进一步与该上游的 compressed-wide BVH8 节点解码公式、掩码和 80-byte 节点布局吻合。可以强判断游戏的软件光追 traversal 部分是对此开源代码的移植或改写。现有材料不支持把整个 Tiny Glade renderer 或整个 shader 目录归为此项目的 fork。

本报告没有使用逆向恢复的 README、HLSL 注释或恢复源码来证明来源；依据仅为发行包原始 license HTML、原版 compiled-shaders 内的 SPIR-V 指令与原始 source dependency 元数据，以及上游精确 tag 源码。

## 原作公开的来源

- 发行材料：`D:/game/ljxsj_92385/Tiny Glade/third-party-licenses.html`，第 20846–20851 行。其 GPU-Raytracer 链接为 `https://github.com/jan-van-bergen/GPU-Raytracer/`，列版本 `1.3.0`，许可 MIT，Copyright (c) 2021-2022 Jan van Bergen。
- 上游 tag `v1.3.0` 对应 commit `3525cd4f96fba01c8976c7f1a8465dac7bd3d60f`（本次由 GitHub API 获取）。上游 README 描述 C++/CUDA pathtracer，提供 BVH2/BVH4/BVH8、两级加速结构、SVGF/TAA 等。
- [精确版本 BVH8 源码](https://github.com/jan-van-bergen/GPU-Raytracer/blob/3525cd4f96fba01c8976c7f1a8465dac7bd3d60f/Src/CUDA/Raytracing/BVH8.h)。上游原字节和 SHA-256 见 `upstream_manifest.json` 与 `upstream-v1.3.0/`。
- [精确版本许可](https://github.com/jan-van-bergen/GPU-Raytracer/blob/3525cd4f96fba01c8976c7f1a8465dac7bd3d60f/LICENSE.txt)。版本声明不等于游戏未经改动地使用该 tag 的全部文件，也不证明某个公开 fork commit。

## 原版机器指令中的具体吻合

从原版 `.bin` 读取 SPIR-V（不是重新编译恢复源码），以下四个模块出现 `PackedBlBvhNode` 类型，其 runtime array 的 `ArrayStride` 为 80，资源名包括 `rt_bl_bvh`、`rt_triangles`：

1. `_gi_trace_gi_candidates.cs.hlsl_cs_main_0.bin`
2. `_primary_rt_debug.cs.hlsl_cs_main_0.bin`
3. `_water_deferred_rt.cs.hlsl_cs_main_0.bin`
4. `_water_ice_deferred_rt.cs.hlsl_cs_main_0.bin`

上游 `Src/CUDA/Raytracing/BVH8.h` 的 BVH8Node 为 5 个 float4，恰为 80 bytes。更关键的是，四个原版模块同时含 `0x10101010`、`0x1f1f1f1f`、`0x07070707`、`0xff000000`、`0x00ffffff`、`0x01010101` 这一组 BVH8 布局掩码；原指令的数据流也吻合上游，而不只是常数碰巧出现。

以 `primary_rt_debug` 为例（偏移是原始 .bin 内字节偏移，SPIR-V 模块始于 23）：

| 原版指令数据流 | 原版偏移 | 上游对应位置 |
|---|---:|---|
| `(meta & (meta << 1)) & 0x10101010` | 31627–31699 | BVH8.h 内 `is_inner4` |
| `(meta ^ (octant & mask)) & 0x1f1f1f1f` | 31807–31879 | BVH8.h 内 `bit_index4` |
| `(meta >> 5) & 0x07070707` | 31915–31951 | BVH8.h 内 `child_bits4` |
| PackedBlBvhNode runtime array stride 80 | 20759 | BVH8Node 的 5×float4 布局 |

`raw_spirv_fingerprints.json` 保留所有匹配模块的原始文件/module SHA-256、常数定义偏移、直接消费者指令、输入定义和数据流树，可由 `scan_raw_spirv.py` 复现。扫描同时发现屋梁 shader 也有 80-byte stride，但没有上述 BVH8 指纹，未据此归为 GPU-Raytracer；单一 stride 不构成来源证据。

这组指纹也属于公开 compressed-wide BVH 的算法家族，单凭公式可证明同算法结构、不能排除独立实现；但结合原作明确的项目/版本声明，移植或改写 GPU-Raytracer BVH8 是有直接来源声明和机器指令互相支持的强判断。

## 来源边界与包含范围

父调查按原始容器解码的 `../original-source-dependencies.json` 中，`shaders/inc/rt.hlsl` 被 18 个 pipeline 文件列为 source dependency。四个完整 BVH8 指纹模块都在这 18 个文件内。其余消费者包括 deferred sun/sky lighting、GI update、水面、waterfall 和 ice reflection march；这些文件列出 dependency 不能证明每个最终阶段都发射了 BVH8 traversal，更不能把它们的全部逻辑归为上游。

`shaders/inc/ray_cone.hlsl` 被 307 个 pipeline 文件列为依赖，说明它处于广泛使用的 include 链；依赖计数并不是来自该开源项目的实际代码量。尚未逐指令核对 ray cone 实现，故本报告不对其精确源码归属下结论。

原版 shader 名称与资源绑定是游戏自己的 HLSL/Vulkan 工作流，而 GPU-Raytracer 上游是 CUDA/C++ 的交互式 pathtracer。现有证据具体落在软件 BVH8 traversal 和节点格式接入，不能外推上游的波前调度、材质系统、SVGF/TAA、场景加载器等全部被游戏采用。

未解决：原游戏 `rt.hlsl` 的全文已从 SPIR-V OpSource 移除，不能做文本逐字 diff；CPU 侧 BVH 构建/转换器是否移植了上游 `BVH8Converter.cpp` 本轮未审计；游戏使用的最终私有修改 commit 未知。没有可据此计算的全目录开源行数百分比。
