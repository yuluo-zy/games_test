# Kajiya 源码血统核对（2026-10-14）

本报告直接读取原作 `D:/game/ljxsj_92385/Tiny Glade/compiled-shaders/*.bin` 的 SPIR-V 指令，与公开上游源码比较。恢复 HLSL 仅用作寻找候选，以下结论不以恢复者写出的注释或文件体积为证据。原版文件身份/BLAKE3 验证由父任务统一记录。

## 结论

**原作确实复用了 Kajiya 的部分 shader 源码；可以确认同源辅助库，无法据现有证据认定整套 Tiny Glade 渲染器是公开 Kajiya 整库 fork。** 原作自己的发行授权文件明确声明 Kajiya 0.2.0 且指向 EmbarkStudios/kajiya。下面的二进制独特结构和运算证据进一步说明这个声明有对应的实际 shader 实现。

比较的公开仓库是 <https://github.com/EmbarkStudios/kajiya>，取当前 main `9fdec0f074f9f822d395f9cfd041b58749e32303`。上游 `crates/lib/kajiya/Cargo.toml` 也写版本 0.2.0；这并不能锁定游戏使用的 commit。最初获取的 `h3r2tic/kajiya` 是该仓库的公开 fork，仅作候选发现，最终比较以 Embark commit 为准。

## 最强证据：Reservoir1spp 源辅助库

上游 <https://github.com/EmbarkStudios/kajiya/blob/9fdec0f074f9f822d395f9cfd041b58749e32303/assets/shaders/inc/reservoir.hlsl> 定义两组独特类型：

- `Reservoir1spp` 成员按顺序为 `w_sum, payload, M, W`。
- `Reservoir1sppStreamState` 成员按顺序为 `p_q_sel, M_sum`。

原作 canopy/grass/plant/solid 四个 deferred pixel shader 的 OpName 和 OpMemberName 全部保留这两个同名类型和同序成员。见 `unique-fingerprints.json`。尤其 grass 原版 bin 的 SPIR-V module offset 为 21538，原文件路径是 `shaders/inc/reservoir.hlsl`。

原版 grass 的 `inc/reservoir.hlsl:94` 指令链可以直接看到：

1. 从 Reservoir1spp 的第 0 成员加载分子，从第 2 成员加载 M。
2. OpFMul 乘 M 与选中的概率。
3. GLSL.std.450 FMax 将结果与 `0x322bcc77`（float32 的 1e-8）取最大。
4. OpFDiv 将分子除以该结果，写入第 3 成员 W。

这与公开上游 `finish_stream` 的 `W = w_sum / max(1e-8, M * state.p_q_sel)` 一致。初始化、更新概率、stream 累积相关行也留下对应指令。原指令 offset、operands 和递归定义保存在 `reservoir-original-instructions.json`，不需要信任恢复 HLSL。

独特类型/成员命名、成员布局、同一个源码 include 路径、具体运算实现，再加发行授权声明，足以确认该辅助源码的同源复用。通用 ReSTIR 算法思想本身并非唯一证据。原 include 96 LF，而上游当前版本 100 LF，说明尚不能宣称源码逐字相同。

## 辅助证据：共同源码布局及 FrameConstants

原版调试源记录给出 421 个源路径；上游 assets/shaders 本次目录有 182 个文件。仅去掉 Tiny Glade 的 `.cs/.raster/.inc` 后缀进行候选比较，得到 35 个路径相同。完整候选清单、原 source LF 和上游 hash 都保存在 `path-and-identifiers-comparison.json`。

其中 6 个 include 的换行数量与上游当前原文精确一致：

| 原版源码路径 | 两边 LF |
|---|---:|
| shaders/inc/bilinear.hlsl | 43 |
| shaders/inc/color/helmholtz_kohlrausch.hlsl | 133 |
| shaders/inc/color/luv.hlsl | 21 |
| shaders/inc/color/xyz.hlsl | 37 |
| shaders/inc/color/ycbcr.hlsl | 12 |
| shaders/inc/morton.hlsl | 18 |

其中 Helmholtz–Kohlrausch 文件名及 133 LF 的一致布局是较具体的源码家族线索。该原 include 出现在两个 final_post bin 中，但其函数被优化消掉：本次**没有**从原 SPIR-V 找到上游自定义 16 项颜色样本表。故这项只支持共同源布局，不能当成当前运行 shader 正在执行该算法或逐字拷贝的证明。

原版 `FrameConstants/ViewConstants` 还保留上游的一组成员名，如 `hi_px_subpixels`、`render_overrides`、`sample_offset_clip`、`prev_clip_to_prev_view`、`prev_view_to_prev_clip`。它们说明共享源码/作者代码的继承关系，强度次于 Reservoir 运算指纹。源 LF 已从上游 252 增至游戏 439，表明该公共结构经历扩充。

## 范围边界

35 个规范化路径相同不能当成 35 个文件已证实相同；没有出现同名路径的 386 个源文件也不能一律判游戏独创。这是候选范围，不是开源代码占比。大量原版路径是游戏领域专用，如 curved_halftimber、building masks、wall brick instances、roof floor、garden spawning、duck/wren、waterwheels 和冰裂纹等；公开 Kajiya tree 中没有对应完整系统。当前资料支持“游戏渲染器集成 Kajiya 代码并扩充自己的游戏流程”，不支持“6 MB 全由一个公开 renderer 整库 fork 来”。

此外，游戏 `shaders/inc/raymarch.hlsl` 620 LF 与同作者公开的 raymarch gist 当前 571 LF 不相同。该 gist 有 Apache-2.0/MIT 声明，是值得继续调查的作者共享代码；本报告没有证明其精确同源版本，不把它算作 Kajiya 确证。URL：<https://gist.github.com/h3r2tic/9c8356bdaefbe80b1a22ae0aaee192db>。

原作 Cargo.lock 的 kajiya 条目没有 source/dependencies；第三方授权可以代表摘用 shader 源码的 attribution 占位，因此不能据这个条目宣称加载了完整 Kajiya Rust 库。公开 `rtoy-rt` 和 `rendertoy` 本次亦只见小型实验目录，不足以构成整套 shader 来源解释；rtoy-samples 未完成深入内容比较。

## 可复核材料

- `compare.py`：原版全 bin OpName/MemberName/OpConstant 和上游目录比较。
- `fingerprints.py`：原版类型成员布局，尝试搜上游颜色数组（未命中即如实记录）。
- `reservoir-lines.py`：原 grass SPIR-V 直接列出 reservoir 源行运算。
- `kajiya-upstream/`：公开 Embark main commit 的稀疏 shader checkout。

不要把仓库的具体 main SHA 误写成原游戏使用的 fork SHA。
