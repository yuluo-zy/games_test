# Tiny Glade shader：复用源码与游戏自有实现的边界

日期：2026-10-14。当前恢复树为 `D:/game/tiny-glade/shaders`，此前提及的 `tiny-glade-recovered/shaders` 已迁移；本清单按当前文件 SHA-256 固定映射。

结论：这套 shader 是公共渲染代码、上游改写和游戏专属实现的混合体。已有明确来源的部分包括 Kajiya、AMD FidelityFX SPD、Intel XeGTAO、GPU-Raytracer BVH8。墙砖、屋顶、地形编辑、植被与动物等包含原版游戏专属的数据接口和行为。不能将整套树视为一个开源项目的原样 fork，也不能将未查明出处的通用辅助代码默认为游戏原创。

## 直接确认包含公共源码核心

路径均为原版容器记录的 `shaders/` 相对路径；恢复文件的映射见 CSV/JSON。

| 原作文件 | 具体来源 | 原始证据和确认范围 |
|---|---|---|
| `inc/reservoir.hlsl` | [Kajiya reservoir](https://github.com/EmbarkStudios/kajiya/blob/9fdec0f074f9f822d395f9cfd041b58749e32303/assets/shaders/inc/reservoir.hlsl) | 原版的两个 Reservoir 类型及成员、源行对应的 `w_sum/max(1e-8,M*p_q_sel)` 指令组合，与公开源码和原作 attribution 互证。 |
| `blur_downsample.cs.hlsl` | [Kajiya blur](https://github.com/EmbarkStudios/kajiya/blob/9fdec0f074f9f822d395f9cfd041b58749e32303/assets/shaders/blur.hlsl) | 原工作组64线程、共享数组138、128×2 tile、两遍11tap、Exp(-d²×0.16)的特定组合一致。游戏资源绑定与 MSAA 入口另外适配。 |
| `inc/fidelityfx/ffx_a.h`、`ffx_spd.h` | [AMD FidelityFX SPD](https://github.com/GPUOpen-Effects/FidelityFX-SPD/blob/master/ffx-spd/ffx_spd.h) | 原 `generate_hzb`、`generate_mips` 模块实际从这些头文件源行发射代码；256线程、64×64 tile、LDS、wave归约和跨组计数等流程相符。恢复树没有这两个独立 `.h`，其计算已被内联。 |
| `ssgi/XeGTAO.hlsl.inc` | [Intel XeGTAO](https://github.com/GameTechDev/XeGTAO/blob/master/Source/Rendering/Shaders/XeGTAO.hlsli) | 原 SSGI 模块有185个源行站点、79个源行来自主算法头；地平线积分、投影法线等计算与上游对应。恢复文件为 `ssgi/XeGTAO.hlsl`。 |
| `inc/rt.hlsl` 中的 BVH8 区域 | [GPU-Raytracer v1.3.0 BVH8](https://github.com/jan-van-bergen/GPU-Raytracer/blob/3525cd4f96fba01c8976c7f1a8465dac7bd3d60f/Src/CUDA/Raytracing/BVH8.h) | 原 PackedBlBvhNode 80字节布局和三个特有的 packed metadata 位操作组合相符；四个原模块完整指纹已核对。确认 CUDA→HLSL 的同源移植/适配，不将整个 rt.hlsl 判为原样复制。 |

原作 `third-party-licenses.html`、Cargo.lock 和 credits 同时声明上述四个项目。版本与源码比较范围详见[原始资料审计](../REPORT.md)。声明版本不是精确源码 commit；公开比较 commit 也不是游戏私有分叉 commit。

## 有上游血统，游戏进行了改写

| 原作文件 | 对比结果 |
|---|---|
| `inc/frame_constants.hlsl` | 继承 Kajiya 的 FrameConstants/ViewConstants 字段族，加入游戏天空、主题、时间、分辨率与渲染控制参数；确认布局血统，不声称全文只增不改。 |
| `calculate_reprojection_map.cs.hlsl` | 保留32767量化、四点1/2/4/8权值除15、Gather排序和-0.2截断的 Kajiya 设计；游戏修改阈值、资源与历史有效性。 |
| `ssgi/temporal_filter.cs.hlsl` | 保留 Kajiya 的统计/variance clamp、双输出与1/8混合结构；上游5×5邻域变为游戏3×3，并改变历史有效性接口。 |
| `blur_pyramid/blur.cs.hlsl` | 沿用 Kajiya 的共享内存模糊结构；共享数组138→136、11tap→9tap，加入 mip 尺寸和边界处理。 |

`fxaa.inc.hlsl` 很可能派生于 NVIDIA FXAA 3.11 PC Quality（特有的边缘/亚像素/双向搜索常数组合支持），但精确来源版本和改动范围未锁定，保留“疑似派生”。`inc/unjitter_taa.hlsl`、HK include、SSGI upsample 也只保留各自证据强度，不按相同文件名判定全文拷贝。HK 本轮编译产物没有对应执行指令。

## 游戏自己实现或定制的层

这里的“自有”针对数据结构、控制策略与接入实现；普通数学、公开算法和共同 include 另计。正面证据来自原版指令读取的具体字段和资源，不来自“搜索没找到”。

| 范围 | 原版可观察的游戏专属内容 |
|---|---|
| `wall/`、`roof/`、`inc/wall_*`、`inc/roof_*` | 墙砖源墙身份、拱高和变形、删除刷高亮、天窗洞、屋顶轮廓和颜色库。wall 的两个 main 主要为包装，主要逻辑在 brick vert/frag include。 |
| `terrain_*`、`rocky_terrain/`、`garden/`、`paths/` | 地形编辑 stroke 数据、刷宽/选择、建筑/花园/水/道路/植被排除 mask 的相互联接、楼梯放置。通用 SDF、marching-squares 和 isolines 核心仍未确认来源。 |
| `nani/`、`nani_*.raster.hlsl` | 游戏自己的 mesh atlas、实例类型、sorted/unsorted draw list、cull/bucket 与 roof/flower/plank 等对象接入。清空、prefix sum 等通用步骤分别标接入层。 |
| `grass*`、`ivy/`、花朵与旗帜 shader | 草叶分块/生成与计数、显现和季节输入、花梗/风弯曲、旗帜图案与调色板。一般噪声和随机采样没有一律判原创。 |
| `critters/` | 鸭子的起降、翼/嘴形变和抚摸状态，蝴蝶/萤火虫自己的实例与可见性数据。 |
| `water/`、水冰 raster、船/睡莲/芦苇 | 水面、扰动源、天气与遮罩、物体摆放和冰裂缝等游戏接入；软件BVH8、一般折射/波形公式分别保留公共或未知边界。 |
| `painterly/`、`vermeer/` | 笔触捕获数量、逐帧生成和进度/历史状态等自己的绘画流程；卷积、组合、copy 等并不因此判算法原创。 |
| `dof/`、TAA、TGSR | 原B的DOF tile/indirect/multires与双焦点接口不同于公开Kajiya DOF；TAA主策略也不同。TGSR有独立的亮度/色度、辅助历史与锐化流程，并有开发者来源声明。只确认游戏系统和策略，不将后续TGSR2.0神经网络公告套到本B版本。 |

详见[游戏领域原始证据](game/REPORT.md)与[渲染 pass 对比](render/REPORT.md)。

## 混合文件与尚未确认的部分

GI/deferred lighting、SSGI入口、HZB/mip入口、水/冰RT等经常是混合：上游提供公共核心，游戏决定资源格式、采样、材质命中、灯光组合和调度。文件包含公共 header 不代表整个文件来自该公共项目。反过来，游戏专属字段也不能证明文件中每行均由开发者从零编写。

数学、hash/noise、颜色转换、BRDF、packing、一般prefix scan、ray cone等仍有来源未定项。`vaShared` 只见于原依赖表，没有独立发射区域，不能据空恢复正文认定原源码为空。未定项完整保留在清单中。

## 覆盖与读法

- 原版404个容器、638个阶段及lock/credits/license与发行manifest的BLAKE3全部匹配。原始资料没有HLSL正文，但保留文件名、源行、类型、资源和真实指令。
- 421个原依赖字符串规范化为417个路径，消除4个 `eval/../` 别名；当前415个恢复HLSL全部有原路径映射，另2个原AMD `.h` 没有独立恢复文件。
- [FILE_INDEX.md](FILE_INDEX.md)：按类别列出全部原路径，适合浏览。
- [file-index.csv](file-index.csv)：原路径、恢复路径、类别、置信度、范围、消费模块与证据文件，适合筛选。
- [original-file-classification.json](original-file-classification.json)：保留每个文件的原证据、所有分类观察和具体上游边界。
- [recovered-file-classification.json](recovered-file-classification.json)：415个当前恢复文件的SHA-256与分类映射。
- [classification-summary.json](classification-summary.json)：最终覆盖计数；类别计数不是开源/原创代码行数占比。

最终417个原路径：6个“包含已确认上游核心”、4个“上游同源并改写”、4个“疑似派生”、174个“游戏专属数据与规则”、125个“游戏接入或组合层”、104个“通用代码来源未定”。没有尚未审阅或恢复文件未映射项。这些标签评价所说明的范围，不能将174+125当成299份逐行原创证明。

“415个恢复文件”不等于415份全部独立手写源码，更不等于恢复后显示的行数。共享include会内联进各个stage/变体，反编译又可能把表达式拆成大量临时变量与控制流。此规模不能单独证明整个渲染器是开源fork，也不支持“人不可能写出来”的判断。本轮不估算无法由原始材料验证的作者行数百分比。
