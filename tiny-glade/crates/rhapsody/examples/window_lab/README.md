# Window Lab

rhapsody 的真实窗口测试素材播放器，也是 [V0–V9 交付计划](../../DELIVERY_PLAN.md) 的共同验收入口。

## 默认持续综合场景

默认 `--scene showcase` 每帧运行四组正式 GPU 工作，然后在 GPU 上合成四分屏：旋转的 64 三角形细分网格与 compute 后处理、真实上一帧历史预览、多队列 graphics/compute/transfer、局部建筑/阴影/像素字形版本更新，以及有界大批上传与延迟读回。建筑、光照、文字、材质与效果版本持续循环变化，不在首轮后停止。字形和建筑是确定性几何夹具，不声称完整游戏资产或字体排版。

不传 `--frames` 时持续播放。默认优先独立队列（缺硬件则如实降级）；每 180 帧调整窗口，每 900 帧穿插最小化/恢复；资源与效果更新在播放中执行。密集窗口压力仍通过 `--wsi-stress --repeat-script` 显式选择，普通播放不会每隔几帧缩放。

```sh
cargo run -p rhapsody --example window_lab --offline -- --scene showcase --queues prefer --require-validation
```

四路 producer 和最终合成均作真实像素验证；报告的 `showcase` 保存帧数、历史、效果发布、上传轮数、物理队列及画面变化。此模式为正确性播放器，会等待各路 GPU 校验，不用于吞吐性能宣称。原简单素材播放器使用 `--scene fixture`。

## 性能与构建模式

调试构建包含四路逐帧 GPU 回读与独立 CPU 校验，不能代表渲染吞吐。建议正常观看使用优化构建：

```sh
cargo run --release -p rhapsody --example window_lab --offline
```

[实测定位报告](../../reports/refactor/2026-10-17-v9/showcase-profile/README.md) 记录短程同机结果及限制：优化参考计算后，同 debug 约 7→19.6 FPS；仅播放器代码开启 opt-level=3 后约 38.7 FPS，仍保留完整像素与同步验证。报告增加四路 producer 构图/提交/等待/校验计时；旧 graph_framework 与 GPU command 指标只覆盖最后合成，不能当全部场景成本。release 全框架结果须另测，不以播放器单独优化数字代替。

## 启动

在 `D:/game/tiny-glade`（Cargo workspace 根目录）运行：

```powershell
cargo run -p rhapsody --example window_lab --offline
```

支持 Windows 原生窗口与 macOS AppKit/CAMetalLayer 窗口，需要当前 rhapsody 必需的 Vulkan 能力及 `swapchainMaintenance1`。生产 `FrameLoop` 默认要求可证明最终关闭；退出通过 `FrameLoop::shutdown` 等待真实呈现 fence，失败会写入报告并非零退出。macOS 复用已锁定的 winit 与 MoltenVK，渲染在主线程 RedrawRequested 回调内执行；素材、生产 FrameLoop、GPU 读回和报告均与 Windows 共用。无设备或能力不足时明确失败并写报告。

Apple Silicon + Homebrew Vulkan/MoltenVK 的运行示例（按实际安装目录设置路径）：

```sh
export DYLD_LIBRARY_PATH=/opt/homebrew/lib
export VK_ICD_FILENAMES=/opt/homebrew/etc/vulkan/icd.d/MoltenVK_icd.json
export VK_LAYER_PATH=/opt/homebrew/share/vulkan/explicit_layer.d
cargo run -p rhapsody --example window_lab --offline -- --frames 360 --script --require-validation --report target/window-lab/macos/report.json --capture target/window-lab/macos/readback.ppm
```

macOS 实际窗口运行需要桌面会话及 Metal 访问权限。图像读回、自动窗口事件和独立的可见窗口检查分别登记；截图权限不足时可见检查保持 pending。macOS/MoltenVK 结果单独记录，不替代 Windows 结果。

有限帧自动验收（360 帧覆盖默认 4 段素材）：

```powershell
cargo run -p rhapsody --example window_lab --offline -- --scene fixture --frames 360 --script --report target/window-lab/baseline/report.json --capture target/window-lab/baseline/readback.ppm
```

严格验证层验收在以上命令末尾添加 `--require-validation`；缺少 Khronos 验证层时非零退出，普通模式也在报告里明确记录层是否启用。

## 播放与素材

- Space：暂停/继续素材时间轴；窗口仍持续呈现同一帧。
- N：下一段素材。
- Right：推进素材一帧（暂停时用于检查）。
- Esc / 关闭窗口：正常退出，写报告；有限帧运行被提前关闭则判失败。
- `--fps 30`：素材固定步长的目标播放率，实际呈现可能受 vsync 和验证开销限制。
- `--vsync on|off`：`off` 请求 mailbox/immediate，表面不支持时回退 FIFO；报告的 `vsync_requested` 与 `present_mode` 记录请求值和实际选择。
- `--playlist PATH`：加载自定义 JSON 播放列表，图片路径相对于列表文件。

`--scene fixture` 的素材为 RGB 色条/灰阶、移动棋盘、移动圆形渐变及真实 PPM 文件。左上红色、右下蓝色角标用于检测方向。图片统一以最近邻缩放到 320×180 再显示到窗口；V0 不做保持宽高比/色彩管理验收。sRGB 交换链的 blit 转换由 Vulkan 格式处理，CPU 参考比较针对 RGBA8 UNORM 离屏图。

播放列表示例：

```json
{
  "version": 1,
  "clips": [
    { "name": "Motion", "kind": "motion", "frames": 120 },
    { "name": "Image 01", "kind": "ppm", "path": "frame01.ppm", "frames": 60 },
    { "name": "Image 02", "kind": "ppm", "path": "frame02.ppm", "frames": 60 }
  ]
}
```

支持 `bars`、`checker`、`motion`、`ppm`。PPM 支持 P3/P6、maxval 255、每边 1..4096；每份列表最多 128 段，图片输入文件总量不超过 64 MiB。V0 没有 PNG/JPEG/MP4 解码器；可用多张 PPM 按序播放，禁止把未支持格式当作已测试。

## 实际端到端路径

```text
播放列表 / CPU 确定性图案
  → rhapsody Buffer 上传
  → VulkanGraphSession：上传 pass → 图像读回 pass → HOST_READ 导出
  → GPU 完成后逐字节比较
  → Win32 / Metal surface acquire → rhapsody 呈现命令（blit + PRESENT 布局）
  → FrameLoop.execute：设备时间线依赖 → PresentTicket → 完成回收 → 下一帧
```

`--script` 在第 30 帧缩小窗口、第 60 帧最小化后恢复、第 120 帧放大窗口；读取真实 client extent 并统计零尺寸跳帧。脚本有限帧模式至少 180 帧。窗口连续 30 秒无呈现进展时，有限帧模式失败。

### WSI 压力脚本

`--wsi-stress` 与 `--script` 为互斥的独立模式，默认运行 360 帧，显式 `--frames` 不能少于 360：

```sh
cargo run -p rhapsody --example window_lab --offline -- --frames 360 --wsi-stress --require-validation --report target/window-lab/wsi-stress/report.json --capture target/window-lab/wsi-stress/readback.ppm
```

目标为 **100 次实际 resize 和 20 次完整最小化/恢复**。每组先完成 5 次 resize；只有观测到变化后的非零物理 extent 才确认一次 resize。随后请求最小化，必须观测到零 extent，保持至少 50 ms 后才请求恢复；再观测到非零 extent 后确认这一完整循环。macOS 最小化期间仅运行控制计时，不提交图像工作。

`wsi_stress` 报告分别记录 request 与 completion 数量、完整循环数，以及最多 140 条实际确认事件（前后物理 extent、请求的逻辑尺寸和确认时已呈现帧数）。请求次数不会直接算作完成次数。每个等待状态最多 10 秒；请求帧数耗尽但目标未完成、窗口提前关闭、原生操作失败或验证层错误都会非零退出。压力脚本继续走相同 FrameLoop，未引入测试专用渲染器。

长跑添加 `--repeat-script` 会连续执行批次；`completed_batches` 记录完整的 100 次 resize / 20 次最小化恢复批次，request/completion 为累计真实计数。诊断只保留最新 140 条确认事件，`diagnostic_events_dropped` 记录已移出的条数。最后 16 帧停止新请求；已经发出的操作仍等待原生确认，最小化必须恢复。达到请求帧数后只处理这些确认，不再提交或增加呈现帧；额外等待单列为 `script_drain_seconds`，从长跑的 `elapsed_seconds` 中扣除。没有完整首批次仍然失败。默认不重复，原 360 帧基线保持 100 / 20。

`presentation_shutdown_complete` 记录退出前真实呈现完成证明是否全部满足；`retained_wsi_generations` 记录关闭失败后为避免提前销毁而保留的代际数。后者非零会使运行失败，不能计作资源归零验收。

每帧检查 GPU 离屏读回；窗口呈现调用与人工/桌面截图检查分开登记。读回完全一致不能独自证明最终显示颜色、缩放和方向正确。

## S03 GPU 光栅与后处理

```sh
cargo run -p rhapsody --example window_lab --offline -- --scene raster --frames 360 --script --require-validation --report target/window-lab/s03/report.json --capture target/window-lab/s03/readback.ppm
```

相同 `FrameLoop` 执行图内颜色附件上的旋转三角形光栅 pass，然后执行 storage-image compute 后处理，把 RGB 映射为 BRG，再读回并沿原展示路径放大到窗口。着色器和顶点约定复用三角形绘制测试；CPU 只给出顶点和覆盖参考，不上传场景像素。`scene` 报告记录 GPU 程序指纹、实际光栅/后处理/参考通过帧数和最坏像素差异。RGBA8 的后处理颜色逐像素严格检查；只有距三角形边界 0.75 像素以内允许 Vulkan 光栅化取整差异，内部和外部覆盖必须匹配。

## S04 temporal 与历史重置

```sh
cargo run -p rhapsody --example window_lab --offline -- --scene temporal --scene-script --frames 360 --script --require-validation --report target/window-lab/s04/report.json --capture target/window-lab/s04/readback.ppm
```

在相同图内使用 Session 持有的命名 temporal 乒乓对：本帧 GPU 后处理结果写入 current，previous 独立拷回并与上一帧已验证的 GPU 图像逐字节比较。首帧或 reset 后的无效 previous 先由 GPU 清零，再验证读回确实为零。实际窗口 extent 变化使历史失效；reset 保留已有分配，资源描述变化才替换分配。`--scene-script` 另外在呈现 60 帧后显式 reset，在 90 帧后触发一次真实图录制失败，然后验证下一帧的历史初始化和继续呈现。只接受该故障回调实际运行后返回的录制错误，DeviceLost/OOM 等其他失败仍非零退出。脚本至少运行 180 帧，默认 360 帧；缺少 reset、故障或恢复确认均失败。

`history_verified_frames`、`history_initializations`、`history_reset_count/reasons`、`injected_failed_frames` 和 `failure_recovery_verified_frames` 分别记录实际结果。`--script` 仍负责窗口缩放/最小化/恢复，可与 `--scene-script` 同时使用；`--wsi-stress` 也可在 GPU 场景中复用。

## 动态渲染

```sh
cargo run -p rhapsody --example window_lab --offline -- --scene raster --rendering dynamic --frames 360 --script --require-validation --report target/window-lab/dynamic/report.json
```

动态渲染是设备和光栅管线的正式最低合同，`auto` 与 `dynamic` 均要求设备实际启用此能力。不支持时返回 Unsupported；附件必须使用图提供的显式布局同步，隐式布局转换声明同样拒绝。生产源码不再创建 VkRenderPass/VkFramebuffer。报告保留 `scene.dynamic_rendering_frames` 和缓存视图数量。历史 Legacy A/B 证据保存在 `reports/refactor/2026-10-17-v9/legacy-reference`，CLI 不保留旧渲染分支。

## 后台管线与着色器编辑脚本

```sh
cargo run -p rhapsody --example window_lab --offline -- --scene raster --rendering dynamic --shader-script --frames 360 --script --require-validation --pipeline-cache target/window-lab/pipeline.cache --report target/window-lab/shader/report.json
```

场景启动通过有界 `AsyncPipelineCompiler` 预热图形/计算管线；帧内只 poll。`--shader-script` 在第 30 个已呈现帧后提交一个错误编辑，确认旧管线仍显示并通过像素验证；第 60 帧后提交合法的另一种颜色通道排列，在下一次图构建前的帧边界发布并验证新像素。脚本必须实际完成一次拒绝和一次发布，否则失败；不接受 DeviceLost、OOM 等设备错误作为预期编辑失败。该模式只用于 raster，有限运行至少 120 帧。

配合 `--repeat-script` 时，每 300 个已呈现帧开启下一轮错误编辑和合法替换；合法实现会在两种既有颜色排列之间切换，每帧沿当前已发布版本验证像素。未完成的后台请求不会被下一轮覆盖，尾部不足 120 帧时不启动新轮。`shader_completed_rounds`、错误拒绝数和帧边界发布数累计记录真实推进。`--repeat-script` 只可与 `--shader-script` 或 `--wsi-stress` 一起使用。

`--pipeline-cache FILE` 使用正式磁盘缓存与设备身份/驱动数据校验，退出保存。重复同一命令可对照 missing/loaded；损坏文件由缓存接口报告 rejected 并重建。`scene` 中记录真实编译请求/峰值、缓存加载/保存、错误编辑拒绝、旧画面保留、新版本发布与验证帧数及两版 shader 指纹。附件缓存每表默认最多 256 项；容量压力只淘汰已完成且没有命令持有的旧项，仍被使用时返回 Timeout。

## 正式全局描述符路径

```sh
cargo run -p rhapsody --example window_lab --offline -- --scene raster --rendering dynamic --shader-script --frames 360 --require-validation --report target/window-lab/bindless/report.json
```

S03、S04、S08 的绘制和计算统一使用 `BindlessShaderPackage` 降低原逻辑 shader 到共享 heap；每次 draw/dispatch 只绑定全局描述符集与 4 字节参数块索引。`BindlessParameters` 按逻辑 binding 写入带真实所有权的资源，参数页由有界 `DynamicConstants` 管理，命令保活其 lease，槽位和页只在实际 GPU 完成后复用。没有逐次分配描述符集或手写第二套 bindless shader 的执行分支。缺少实际启用的 descriptor indexing、core 动态数组索引或非一致数组索引能力时返回 Unsupported。shader 编辑必须保持逻辑参数 ABI，失败编辑保留旧画面；报告记录真实 bindless 验证帧和槽位峰值。

## S05 瞬态附件与 S06 结果缓存

```sh
cargo run -p rhapsody --example window_lab --offline -- --scene transient --transient-aliasing off --frames 360 --require-validation --report target/window-lab/s05/off.json
cargo run -p rhapsody --example window_lab --offline -- --scene transient --transient-aliasing on --frames 360 --require-validation --report target/window-lab/s05/on.json
cargo run -p rhapsody --example window_lab --offline -- --scene result --frames 360 --require-validation --report target/window-lab/s06/report.json
```

S05 在真实 GPU 上 clear 首个图像，再通过整条 image-copy 链传递到画面；相邻图像同 pass 使用，隔一代图像可在真实依赖允许时复用 backing。默认 8 个附件、2 倍场景尺寸，可设 `--transient-attachments 3..64` 与 `--transient-scale 1..8`。两种分配模式都逐像素验证，并报告实际内存需求、backing 数/字节、复用字节、依赖边及退出时真实分配/释放配平。

S06 的建筑轮廓、阴影和文字夹具使用正式结果缓存。静止帧跳过 producer；脚本在 60/120/180/240/300/320 帧分别改变几何、光照、文字、实现、视图和材质，验证实际像素及精确受影响的 producer 执行数。两个场景默认且至少运行 360 帧，结果分别记录在 `transient` 与 `result_cache_scene`，均复用同一 FrameLoop 和呈现/关闭协议。

## S07 有界上传与延迟读回

```sh
cargo run -p rhapsody --example window_lab --offline -- --scene transfer --frames 360 --script --require-validation --report target/window-lab/s07/report.json --capture target/window-lab/s07/readback.ppm
```

复用正式 `TransferManager`，用两个 1 MiB 上传/读回槽和 8 MiB staging 实际 backing 上限，分块传输 16 MiB GPU buffer。画面是该 buffer 首块经相同图和呈现路径拷出的 RGBA 数据。每个 readback ticket 至少持有四个呈现帧；两槽被持有时第三次请求必须返回 `Timeout`，释放并验证旧 ticket 后继续真实传输。读回整个 payload，逐字节检查偏移与轮次，至少完整验证一轮；默认 360 帧，有限运行不能少于 64 帧。

`transfer` 报告记录 staging `committed_bytes`、实际 `capacity_stalls`、满容量拒绝与恢复次数、已上传/验证字节、完成 payload 轮次、leased/pending 峰值及实际 `try_read` 的调用/未就绪/完成数量。未就绪次数按实际结果统计，驱动较快时可以为零。退出单独等待每个成功 transfer 提交的完成集合，`transfer.shutdown_complete` 与呈现关闭分别确认。

## 报告与限制

报告默认 `target/window-lab/report.json`；可指定独立输出目录。记录平台与窗口宿主、设备、驱动、验证层、素材指纹、帧数、窗口事件、交互计数、输入/读回哈希和错误。

`api_version_raw` 保留历史查询字段；`instance_api_version_raw` 是创建实例时实际请求的 API，`physical_api_version_raw` 是该实例下 `vkGetPhysicalDeviceProperties` 的实际返回值。MoltenVK 在不同实例请求或配置下可能报告不同版本，不能用另一次 vulkaninfo 输出替换本次查询值。物理设备支持版本与实例请求、实际启用的扩展分别记录，避免把支持能力当成已启用能力。

查询启动时 Git revision 与 dirty 状态，仅说明运行环境，不等于编译产物精确源码哈希。诊断只保存前 32 条错误/警告文本，累计数量完整；CPU 耗时最多保留最近 10000 个样本并分批淘汰。

`memory_budget_usage` 记录已登记资源范围/backing 与受预算管理 CPU 存储的 live/in_flight/cache/retiring 分类快照和采样峰值；已知 CPU live/cache/peak 另列。native 预留块及子分配空隙另报；未登记 CPU 容器、驱动内部和整台 GPU 内存不能由此推算。

`stability_samples` 在第 300 帧、每 1000 帧和关闭前记录实际图资源、命令、池、命名资源与已登记内存，最多保留 128 条；长跑检查最后十次帧完成采样的资源/命令数量是否稳定。`teardown_memory` 在 Lab 与设备所有者释放后检查资源登记和已知 CPU/总预算归零。此处只证明应用账本，驱动和操作系统内存另属不可见范围。

`status=pass` 仅表示该次请求的自动检查通过，`window_visual_check` 默认 pending，最终 M0 还需独立窗口检查记录。缺失指标用 null。未到请求帧数、脚本未执行、像素差异或 validation error 均失败；不把不支持/跳过计入通过。

播放器保留确定性 fixture、GPU 光栅/后处理和 temporal 场景，均通过生产 FrameLoop.execute 管理主图和展示提交、租约及完成回收。耗时包含参考计算、读回等待与驱动呈现，不是框架性能基准。源码接线不能独自认证里程碑；当前验收范围见 REFACTOR_PROGRESS。

## 开发验证

```powershell
cargo test -p rhapsody --example window_lab --offline
cargo build -p rhapsody --example window_lab --offline
```

单测覆盖 PPM 截断/越界、二进制像素恰好为空白字符、播放列表及动画确定性、压力脚本的真实 extent 确认策略、光栅参考拒绝错误图像、后处理 SPIR-V 操作，以及故障脚本拒绝吞掉无关错误。窗口运行结果另记在 [验收记录](VALIDATION.md)。


## S08 真实多队列与测量

```powershell
cargo run -p rhapsody --example window_lab --offline -- --scene multi-queue --queues require --sharing exclusive --gpu-timing on --frames 360 --require-validation --report target/window-lab/s08/report.json
```

`--queues off|prefer|require` 控制真实 graphics/compute/transfer 配置，`--sharing exclusive|concurrent` 改变实际中间图像的共享方式。S08 复用同一光栅、计算和读回闭包及 CPU 像素参考；报告记录实际提交捕获、队列族、时间线值与 waits。单队列 Off 合法退化，Require 缺队列时明确失败。外部展示图像仍经正式 ownership 往返，Concurrent 不表示省略内存依赖。

`--warmup 300 --samples 3000` 选择预热后固定样本，`--plan-cache`、`--subgraph-cache`、`--result-cache` 提供 on/off 对照，`--allocation dedicated|suballocated` 控制正式分配策略。CPU 图构建、编译、录制、提交、完成等待、宿主读回与参考图单列；`framework_including_declarations` 包含构建及图执行的联合分布，缓存查询与键构造成本计入其中。`--gpu-timing on` 记录各物理队列命令跨度的每帧总和，跨度可能重叠，且不含展示提交/WSI，不能冒充 GPU active time 或端到端延迟。

`initialization_seconds` 记录 `Lab::new` 的设备/场景初始化和后台管线预热，`first_present_seconds` 记录本次运行开始到首次成功呈现的时间。`performance.cold_start` 独立保留最初最多 300 个已呈现帧的总帧、图框架和各物理队列 GPU command-span 分布，不与预热后的稳定样本混算。图框架先对同一帧的 compile/record/submit 相加再计算分位数。磁盘缓存主要对照这些启动指标，不能仅凭稳定 FPS 判断收益，也不能将各阶段 p99 相加当作总 p99。

验收脚本 `../../tools/window_lab_checks.py` 使用 `--build` 保存构建快照，或以 `--binary` 和匹配的 `--build-manifest` 复用同一快照；它复制二进制与验收脚本并记录 SHA256、相关源码和素材指纹、命令及逐项结果。performance 预热 300、采样 3000、交替次序重复 3 次，对 CPU 和各物理队列 GPU 分布分别检查 p95/p99。历史对照须同时提供 `--reference-binary` 与 `--reference-identity`；旧二进制缺少构建与执行的联合分布时，只比较共同的执行分布并保留该限制，不能相加两个 p99 伪造总 p99。

`--performance-pair pipeline-disk` 每次先从缺失的应用缓存启动，再把此次保存文件用于下一进程，重复三组；分别要求报告 `missing` / `loaded`，记录初始化、首次呈现及首 300 帧分布。操作系统和驱动私有缓存不受控制，三次启动时间不宣称为启动 p99；稳定 3000 帧的 p95/p99 仍另行记录。

long-run 默认稳定/效果/缩放三种负载各至少 30 分钟且至少 10000 帧，可用 `--long-run-case` 单独选择。它使用显式 `--frames 10000 --min-run-seconds 1800`：两个条件同时满足才停止，普通 `--frames` 仍要求精确帧数。`play_seconds` 从初始化完成后计时，排空原生窗口确认的额外时间不计入其中；重复脚本持续到停止边界，尚未发布的最后一轮 shader 编辑单列丢弃数，已有完整轮及每帧像素验证仍必需。二进制依赖的 rhapsody、shader 或素材在运行期间改变，结果明确无效；不关联 Window Lab 的下游业务源码另由消费者矩阵验收。屏幕截图和交互按独立状态记录。
