# Tiny Glade shader 原始资料来源调查

日期：2026-10-14。目标：判断恢复出的 shader 是否来自开源组件、开源 renderer 的派生代码，或游戏自有内容。调查依据是用户指定的 B 发行目录，不以恢复 README、恢复注释或恢复 HLSL 的分类标签证明来源。

## 结论

原作明确声明使用 **Kajiya 0.2.0、FidelityFX 1.0.0、XeGTAO 1.0.0、GPU-Raytracer 1.3.0**。其中 Kajiya 是开源实时全局光照 renderer，不能只把 shader 的外部来源看作两三个小数学算法。原作 SPIR-V 与公开上游进一步确认 Kajiya reservoir 辅助库、AMD SPD 降采样、XeGTAO 主算法文件以及 GPU-Raytracer BVH8 traversal 的实际使用或同源移植。

因此应将该树视为**含有开源渲染代码派生部分的游戏 shader 集合**。证据支持复用/移植/适配；不支持把整个 shader 树、整个 Rhapsody renderer、或整款游戏一律宣布为 Kajiya/GPU-Raytracer 的完整 fork。游戏使用的最终私有源码 commit 和全部修改范围未恢复。

此前会话中按几个带组件名称的 HLSL 文件体积估算的“约 1.6%”不构成来源占比，撤回。共享 include 会内联进多种 shader；遗漏 Kajiya 和 BVH traversal 还会造成更根本的分类错误。不能由文件名、include 次数、重建输出字节数换算开源/原创行数。

## 原始材料及完整性

原始根目录：`D:/game/ljxsj_92385/Tiny Glade/`。

| 原始资料 | 作用 |
|---|---|
| `build-info/Cargo.lock` | 随发行包的组件名称和声明版本 |
| `credits-crates.txt` | 随发行包列出的组件 credits |
| `third-party-licenses.html` | 原作者交付的上游 URL 和许可证归属 |
| `build-info/manifest.json` | 原作 BLAKE3 内容清单 |
| `compiled-shaders/*.bin` | 原始 shader 容器、SPIR-V、源码依赖、源行调试信息 |

全部 **404 个 shader 容器 / 638 个阶段**均与原作 manifest 的 BLAKE3 值匹配。Cargo.lock、credits、license HTML 也全部匹配该清单。原始 manifest 本身的 SHA-256 是 `3425e76472672b9b7c3d18ed6f7cdc1f21d2c445782845a82528dd4f50bf0a11`。这是发行材料内部完整性核对，不是外部签名认证。

现有 `evidence/render-passes/shaders/oracle-bin` 的 404 份文件与发行目录逐份 SHA-256 一致，既有 oracle 可继续复用，但本轮从发行目录重新解析。

按容器字段顺序读取 stages、entry point、SPIR-V words、source_dependencies；不是以 ASCII 子串或任意偏移 SPIR-V magic 替代结构解析。共得到 **421 个原始源码依赖路径：260 个 compute/raster 入口路径和 161 个支持路径**。详细字段在 `original-shader-inventory.json` 和 `original-source-dependencies.json`。

所有阶段的 OpSource 正文只有空白/换行占位，**没有原始 HLSL 源码正文**。仍保留 OpString 文件名、OpLine、OpName/OpMemberName、类型、资源布局和实际指令。因此能对照源文件与原指令，但不能做全树原始源码逐字 diff。OpSource 的 LF 数量仅是调试源行布局线索，不能直接称为原作代码 LOC。

## 原作者直接声明的来源

| 组件 | 原作声明位置 | 原作声明版本 / URL | 证据含义 |
|---|---|---|---|
| Kajiya | license HTML:13351；Cargo.lock:3782；credits:248 | 0.2.0 / <https://github.com/EmbarkStudios/kajiya> | 在 Apache-2.0 分组声明使用该开源 renderer 的代码 |
| FidelityFX | license HTML:20057；Cargo.lock:6；credits:169 | 1.0.0 / <https://github.com/GPUOpen-LibrariesAndSDKs/FidelityFX-SDK> | AMD 2023 MIT 代码归属 |
| XeGTAO | license HTML:20117；Cargo.lock:14；credits:547 | 1.0.0 / <https://github.com/GameTechDev/XeGTAO> | Intel 2016–2021 MIT 代码归属 |
| GPU-Raytracer | license HTML:20846；Cargo.lock:10；credits:204 | 1.3.0 / <https://github.com/jan-van-bergen/GPU-Raytracer/> | Jan van Bergen 2021–2022 MIT 代码归属 |

这些 Cargo.lock 条目无 source/checksum/dependencies，可能是为非 Rust 源码组件生成 credits 的占位条目。其存在证明原作声明使用，不等于完整原版 Rust crate 被原样链接，更不能从此推导整个游戏引擎架构。

## 原始指令和上游源码互相支持的范围

### Kajiya

详见 [kajiya/REPORT.md](kajiya/REPORT.md)。公开 Embark 主线 shader 树与原作依赖路径有 35 个后缀归一后的路径对应；其中 6 个支持文件的源行 LF 布局相同。路径和 LF 只作辅助证据。

更强证据在原版 deferred shader 的 SPIR-V：`Reservoir1spp` 成员 `w_sum,payload,M,W` 和 `Reservoir1sppStreamState` 成员 `p_q_sel,M_sum` 与上游一致；原始指令在 `inc/reservoir.hlsl` 的源行上实现 `W = w_sum / max(1e-8, M * p_q_sel)`。结合原作明确的 Kajiya attribution，可确认至少这一共享 shader 源码辅助库同源复用。`FrameConstants` 的特有字段等提供附加支持。

原始 `shaders/inc/reservoir.hlsl` 在 13 个 pipeline 的 dependency 表中出现；这不是 13 份独立上游文件，也不是全部这些 shader 的所有代码都归属 Kajiya。整库 fork 和私有修改 commit 尚未证实。

### FidelityFX SPD

原版 `_generate_hzb.cs.hlsl_cs_main_0.bin` 和 `_generate_mips.cs.hlsl_cs_main_0.bin` 的容器依赖均包含 `shaders/inc/fidelityfx/ffx_a.h`、`shaders/inc/fidelityfx/ffx_spd.h`。

不只是声明 include：两个 SPIR-V 各有 **341 个 OpLine 站点 / 100 个不同源行**指向 `ffx_spd.h`，另有 12 个站点指向 `ffx_a.h`。原作编译代码实际来自对应 header 的源行。

上游 [SPD 源码](https://github.com/GPUOpen-Effects/FidelityFX-SPD/blob/master/ffx-spd/ffx_spd.h)与原模块共有 256 线程、64×64 输入 tile、16×16 LDS、wave quad 归约、跨工作组原子计数等具体流程。原作 HZB 使用 min 归约和深度格式，mip pass 使用平均归约和颜色格式，体现应用自定义的 reduction/load/store/绑定适配。原作声明 SDK 1.0.0；本轮未证明其 header 与哪个上游 commit 完全逐字相同。

### XeGTAO

原版 `_ssgi_ssgi.cs.hlsl_cs_main_0.bin` 的容器依赖包含 `shaders/ssgi/XeGTAO.hlsl.inc` 和 `shaders/ssgi/vaShared.hlsl.inc`。SPIR-V 有 **185 个 OpLine 站点 / 79 个不同源行**指向 XeGTAO 主算法 include，证明对应源文件实际发射代码。

原作明确声明 XeGTAO 来源；公开 [XeGTAO.hlsli](https://github.com/GameTechDev/XeGTAO/blob/master/Source/Rendering/Shaders/XeGTAO.hlsli)中的地平线角度、投影法线及 0.05 长度混合修正等与原 SSGI 算法对应。原作版本声明 1.0.0，不等于当前公开主线或某个确定私有修改 commit。不能以恢复头文件只有两函数推断原作只采用这两函数。

### GPU-Raytracer

详见 [raytracer/REPORT.md](raytracer/REPORT.md) 和可复现原指令 `raw_spirv_fingerprints.json`。上游精确 tag v1.3.0 对应 `3525cd4f96fba01c8976c7f1a8465dac7bd3d60f`。

原版 GI candidate tracing、primary RT debug、水面 deferred RT、冰 deferred RT 的 SPIR-V 同时具有 PackedBlBvhNode 的 80-byte 布局和三条具体元数据解码公式：

- `(meta & (meta << 1)) & 0x10101010`
- `(meta ^ (octant & mask)) & 0x1f1f1f1f`
- `(meta >> 5) & 0x07070707`

对应上游 [BVH8.h](https://github.com/jan-van-bergen/GPU-Raytracer/blob/3525cd4f96fba01c8976c7f1a8465dac7bd3d60f/Src/CUDA/Raytracing/BVH8.h)。结合原作 attribution，软件 BVH8 traversal 的移植/改写证据很强。原 `rt.hlsl` 有 18 个 dependency 消费者，完整指纹确认于四个模块。未审计其 CPU BVH converter 和全部其他上游算法，不外推完整 renderer。

### FXAA / FSR / egui

FXAA 原版确实从 `shaders/fxaa.inc.hlsl` 发射代码：344 个 OpLine 站点 / 134 个不同源行。前轮的算法级 FXAA 对应关系仍可作为线索，但原作 license 未单列 FXAA；不能由算法相似或 NVIDIA 的其他 SDK attribution 自动断言该文件精确来源。

原作确实声明 egui 0.27.2（registry 依赖）；特定 egui.raster shader 的全文是否来自上游 renderer 未证实。FSR 命名的 copy/clear 辅助 shader 不等于 FSR 核心上采样源码。它们应与原库本身、游戏接线代码分别判断。

## 对逆向恢复的影响

对明确来源且实际对应的上游部分优先复用原库或原源码，仅恢复游戏对它的参数、资源格式、绑定、控制流组合和必要改动。Kajiya 原作者/上游代码与游戏新增实现必须逐段区分，不能按 shader 名称或所有本地包一律认作私有业务。

原本 260 个 shader 入口编译成 638 个阶段/变体，共享支持文件又会被内联进不同模块。逆向输出还可能同时保留重建候选和转译版本。恢复树的 MB 数与独立人工编写的原始源码量不是同一个指标，也不足以证明整体 fork。

本轮已回答：发行材料直接声明了哪些项目；原始 shader 与这些项目有哪些实际对应；恢复树体积不能换算来源占比。尚未回答：全目录每行精确来源、最终私有分叉 commit、所有游戏新增/改写边界。未匹配的文件应保留来源待查，不能默认全为游戏独创。

## 复现与数据

- `inspect_original.py`：发行容器结构解码、SPIR-V 元数据、依赖清单、BLAKE3 manifest 与 oracle copy 核对。
- `summary.json`：全量覆盖和原始文件 hash。
- `original-shader-inventory.json`：逐原始容器、阶段、OpSource/OpLine 和源依赖。
- `original-source-dependencies.json`：421 个原始路径及消费者。
- `original-license-links.json`：原版 HTML 定点来源片段和 URL。
- `kajiya/`、`raytracer/`：上游精确下载快照、原始机器指令比对及子报告。

脚本使用 Python 标准库和原版 BLAKE3 实现。此机只在 `D:/game/tmp/shader-origin-blake3` 安装了 `blake3==1.0.11`；复现命令为 `py -3 evidence/shader-origin-audit-2026-10-14/inspect_original.py`（从 `reverse-engineering/tiny-glade` 运行）。调查期间未修改发行文件或恢复源码。
