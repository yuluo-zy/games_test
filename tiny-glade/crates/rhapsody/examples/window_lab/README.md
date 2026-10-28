# Window Lab

Window Lab 是 rhapsody 的窗口与真实 GPU 验收示例。它通过生产 `FrameLoop` 路径执行素材或 GPU 场景，逐帧做离屏读回与 CPU 参考比较，并写出 JSON 报告。它是验收工具，不是游戏渲染器，也不是框架性能基准。

验证状态见 [VALIDATION.md](VALIDATION.md)。

## 范围与平台

- 入口为 `examples/window_lab.rs`，模块位于本目录。窗口与运行逻辑只在 `windows` 与 `target_os = "macos"` 下编译；其他平台的程序打印错误后以退出码 2 退出。
- Windows 使用 Win32 窗口；macOS 使用 AppKit 与 CAMetalLayer，经 MoltenVK 运行。两者共用素材、`FrameLoop`、读回与报告。
- 需要 rhapsody 所需的 Vulkan 能力。无设备或能力不足时明确失败，失败时仍写出报告。
- 呈现策略为 `SwapchainPolicy::Strict`（源码中的默认值）。该策略不要求 `VK_EXT_swapchain_maintenance1`：有该扩展时用 present fence 证明呈现完成，没有时用带超时的 `QueueIdle` 证明。交换链细节以 [交换链 README](../../src/present/swapchain/README.md) 为准。

## 构建

```sh
cargo build -p rhapsody --example window_lab --offline
```

## 运行

以下路径均相对于 Cargo workspace 根目录 `tiny-glade/`。

默认场景为 `--scene showcase`，不传 `--frames` 时持续播放：

```sh
cargo run -p rhapsody --example window_lab --offline -- --scene showcase --queues prefer --require-validation
```

有限帧素材播放，含窗口缩放、最小化与恢复脚本：

```sh
cargo run -p rhapsody --example window_lab --offline -- --scene fixture --frames 360 --script --report target/window-lab/fixture/report.json --capture target/window-lab/fixture/readback.ppm
```

macOS 上使用 Homebrew 安装的 MoltenVK 时，按实际安装目录设置环境变量：

```sh
export DYLD_LIBRARY_PATH=/opt/homebrew/lib
export VK_ICD_FILENAMES=/opt/homebrew/etc/vulkan/icd.d/MoltenVK_icd.json
export VK_LAYER_PATH=/opt/homebrew/share/vulkan/explicit_layer.d
```

键盘：Space 暂停或继续素材时间轴，N 切换到下一段素材，Right 单步前进一帧，Esc 退出。关闭窗口同样正常退出。

## 场景

| `--scene` | 内容 | 有限运行默认帧数与约束 |
| --- | --- | --- |
| `showcase` (默认) | 四路 GPU 工作加最终合成：细分网格与 compute 后处理、历史预览、多队列工作、建筑/文字/效果版本更新、有界上传与读回 | 持续播放；`--queues` 默认 `prefer` |
| `fixture` | 素材播放器（`bars`、`checker`、`motion`、`ppm`） | 持续播放 |
| `raster` | 颜色附件上的三角形光栅与 storage-image compute 后处理 | 持续播放 |
| `temporal` | 命名 temporal 乒乓对与历史 reset | `--scene-script` 默认 360 帧，至少 180 |
| `transfer` | 有界上传与延迟读回（16 MiB buffer 分块传输） | 默认 360，至少 64 |
| `transient` | 瞬态附件与 aliasing 对照 | 默认 360，至少 360 |
| `result` | 版本化结果缓存与精确受影响的 producer 执行 | 默认 360，至少 360 |
| `multi-queue` | graphics/compute/transfer 依赖链 | 持续播放；`--sharing` 仅此场景有效 |

## 常用参数

所有参数均经 `app.rs` 的 `Options::parse_from` 解析。下表只列当前存在的参数。

| 参数 | 说明 |
| --- | --- |
| `--scene NAME` | 见上表。 |
| `--frames N` | 运行的已呈现帧数。不传则持续播放。 |
| `--min-run-seconds N` | 要求同时满足帧数与最短运行时间；必须与显式 `--frames` 同用。 |
| `--fps 1..240` | 素材固定步长的目标帧率，默认 60。实际呈现受 vsync 与验证开销影响。 |
| `--vsync on\|off` | 默认 `on`。`off` 请求 MAILBOX 或 IMMEDIATE，不支持时回退 FIFO。 |
| `--script` | 窗口脚本：第 30 帧缩小、第 60 帧最小化并恢复、第 120 帧放大。至少 180 帧。 |
| `--wsi-stress` | 窗口压力脚本，与 `--script` 互斥。目标为 100 次实际 resize 与 20 次完整最小化/恢复。非 `showcase` 默认 360 帧，显式帧数至少 360；`showcase` 未给帧数时持续运行。 |
| `--repeat-script` | 只能与 `--shader-script` 或 `--wsi-stress` 同用，连续执行脚本批次。 |
| `--scene-script` | 只能用于 `temporal`。在第 60 帧 reset 历史，第 90 帧注入一次录制失败并验证恢复。 |
| `--shader-script` | 只能用于 `raster`，至少 120 帧。第 30 个已呈现帧提交错误编辑，要求被拒绝且旧画面保留；第 60 帧提交合法替换，并在帧边界发布后验证新像素。`--repeat-script` 时每 300 帧为一轮。 |
| `--rendering auto\|dynamic` | 仅用于 `raster`、`temporal`、`multi-queue`、`showcase`。`dynamic` 要求设备支持，否则返回 Unsupported。 |
| `--queues off\|prefer\|require` | 物理队列选择。`showcase` 默认 `prefer`，其他场景默认 `off`。`require` 缺少队列时失败。 |
| `--sharing exclusive\|concurrent` | 中间图像共享方式，仅用于 `multi-queue`。 |
| `--transient-aliasing on\|off` | `transient` 场景的 aliasing 开关，默认 `off`。 |
| `--transient-attachments 3..64` | 附件数，默认 8。 |
| `--transient-scale 1..8` | 相对场景尺寸的倍数，默认 2。 |
| `--playlist FILE` | 自定义素材播放列表，默认为 `assets/window_lab/playlist.json`。 |
| `--pipeline-cache FILE` | 管线磁盘缓存。运行结束时保存，默认 `target/window-lab/pipeline.cache`。 |
| `--allocation suballocated\|dedicated` | 分配策略。 |
| `--gpu-timing on\|off` | 记录 GPU 命令时间戳。 |
| `--gpu-pass-timing on\|off` | 记录逐 pass 时间戳，需要 GPU 计时：单独指定会隐式开启 `--gpu-timing`；与显式的 `--gpu-timing off` 同时给出则报错，与参数先后顺序无关。 |
| `--plan-cache`、`--subgraph-cache`、`--result-cache on\|off` | 缓存开关，用于对照测量。 |
| `--performance` | 测量模式：`showcase`，预热 300 帧、采样 3000 帧、fps 240，并开启 GPU 时间戳。不能与 `--require-validation`、`--script`、`--wsi-stress`、`--shader-script` 同用。 |
| `--warmup N`、`--samples 1..10000` | 测量模式的预热与采样帧数，仅与 `--performance` 同用。 |
| `--report FILE` | 报告路径，默认 `target/window-lab/report.json`。 |
| `--capture FILE.ppm` | 运行结束时保存最后一帧的离屏读回图（PPM）。 |
| `--require-validation` | 要求 Khronos 验证层启用。层不可用时非零退出。 |

## 素材播放列表

```json
{
  "version": 1,
  "clips": [
    { "name": "Motion", "kind": "motion", "frames": 120 },
    { "name": "Image 01", "kind": "ppm", "path": "frame01.ppm", "frames": 60 }
  ]
}
```

- `kind` 支持 `bars`、`checker`、`motion`、`ppm`。
- `ppm` 的 `path` 相对于播放列表文件所在目录。
- 图片仅支持 P3 与 P6、maxval 255、每边 1 到 4096 像素。没有 PNG、JPEG 或视频解码器。
- 每份列表最多 128 段，图片总量不超过 64 MiB。
- 图片以最近邻缩放到 320×180 显示。

## 报告

报告在运行结束时写入 `--report` 指定的文件，失败时也会写入。JSON 字段较多，下表只列常用字段。

| 字段 | 含义 |
| --- | --- |
| `status` | `pass` 或 `fail`。只表示本次请求的自动检查通过。 |
| `failure` | 失败原因，仅在失败时存在。 |
| `requested_frames`、`presented_frames`、`pixel_verified_frames` | 请求帧数、已呈现帧数、已逐像素验证帧数。`--performance` 模式下不计入验证帧。 |
| `validation_enabled`、`validation_error_count`、`validation_warning_count` | 验证层是否启用及计数。文本各只保留前 32 条。 |
| `platform`、`window_backend`、`device`、`driver_version_raw` | 宿主、窗口后端、设备与驱动信息。 |
| `api_version_raw`、`instance_api_version_raw`、`physical_api_version_raw` | 历史查询值、实例请求的 API 版本、物理设备实际返回的版本。三者不能互相替代。 |
| `enabled_device_extensions` | 实际启用的设备扩展。 |
| `vsync_requested`、`present_mode` | 请求的 vsync 设置与实际选择的呈现模式。 |
| `fixture_hash`、`source_hash`、`readback_hash` | 素材、源数据与读回结果的哈希。 |
| `script_completed` | `--script` 是否完整执行。 |
| `wsi_stress` | 压力脚本的请求数、完成数、完整循环数、批次数与最多 140 条确认事件。 |
| `presentation_shutdown_complete` | 退出前呈现完成证明是否全部满足。 |
| `retained_wsi_generations` | 关闭失败时为避免提前销毁而保留的交换链代际数。非零会使运行失败。 |
| `scene` | 当前 GPU 场景的统计，如 dynamic rendering、bindless、history 与 pipeline 计数。 |
| `showcase`、`transfer`、`transient`、`result_cache_scene`、`multi_queue_scene` | 对应场景的详细报告，仅在该场景运行时存在。 |
| `cpu_frame_ms_p50/p95/p99`、`elapsed_seconds`、`play_seconds`、`initialization_seconds`、`first_present_seconds` | CPU 帧耗时与时间统计。耗时包含参考计算、读回等待与驱动呈现，不是框架性能基准。 |
| `gpu_time_ms`、`gpu_timing`、`gpu_pass_timings` | GPU 时间戳，需要 `--gpu-timing on`。各跨度可能重叠，不能直接相加。 |
| `memory_budget_usage`、`stability_samples`、`teardown_memory` | 应用账本内的内存统计。不包含驱动与操作系统内存。 |
| `performance` | `--performance` 的测量统计。 |
| `window_visual_check` | 固定为 `pending; offscreen readback is verified separately`。窗口显示需人工检查。 |
| `limitations` | 运行时写入的限制说明。 |
| `git_revision_at_launch`、`git_dirty_at_launch` | 启动时的 Git 状态。仅说明运行环境，不等于编译产物的源码哈希。 |

## 验证脚本

`tools/window_lab_checks.py` 是批量验收脚本，位于 `crates/rhapsody/tools/`。它把被测二进制、脚本和相关源码指纹复制到输出目录，并逐项记录结果。本页只核对了其参数定义，尚未运行。

| 参数 | 说明 |
| --- | --- |
| `--suite functional\|performance\|long-run` | 必填。选择功能、性能或长跑套件。 |
| `--output DIR` | 必填。输出目录。 |
| `--build` | 用 `cargo build -p rhapsody --all-features --example window_lab --offline --locked` 构建并保存快照。 |
| `--binary FILE` | 复用已保存的二进制，必须与 `--build-manifest` 同用。 |
| `--build-manifest FILE` | 与 `--binary` 同用，用于校验源码指纹与二进制 SHA256。 |
| `--reference-binary FILE`、`--reference-identity FILE` | 历史对照二进制及其身份文件，须成对使用。 |
| `--performance-pair` | 选择一组性能对照，或 `all`。 |
| `--long-run-case stable\|effects\|resize\|all` | 长跑负载。 |
| `--long-run-seconds N` | 长跑最短时长，默认 1800。 |

示例：

```sh
python3 crates/rhapsody/tools/window_lab_checks.py --build --suite functional --output target/window-lab/checks
```

`--suite functional` 会依次运行多组 GPU 场景，耗时较长。该脚本的 `performance` 套件依赖 `--reference-binary` 与若干历史开关（如 `--rendering legacy`、`--bindless`）。当前 `Options::parse_from` 不接受这些取值，需要另有对应旧版二进制才能运行。脚本中 `DYLD_FALLBACK_LIBRARY_PATH` 固定为 `/opt/homebrew/lib`，只适用于 macOS 的 Homebrew 安装。

## 测试

```sh
cargo test -p rhapsody --example window_lab --offline
```

窗口模块被 `cfg(any(windows, target_os = "macos"))` 限定，因此只能在 Windows 或 macOS 上运行。在其他平台上此命令运行 0 个测试。在 Windows 或 macOS 上，源码中共有 25 个 `#[test]`，覆盖：

- PPM 解析的截断、越界与空白字节处理；播放列表与素材确定性（`fixture.rs`）。
- 窗口压力的真实 extent 确认、最小化恢复顺序、批次计数与诊断上限（`app.rs`）。
- 光栅、后处理与 temporal 故障脚本的参考比较与错误处理（`scene.rs`）。
- 传输分块参考（`transfer_scene.rs`）与结果缓存覆盖（`result_scene.rs`）。
- 性能统计的预热截断与逐帧分布（`metrics.rs`）。

## 当前限制

- 只支持 Windows 与 macOS。其他平台不运行窗口程序。
- 素材与场景的像素比对只证明读回结果与 CPU 参考一致。它不能证明最终屏幕颜色、缩放与方向正确，窗口显示需人工检查。
- 运行中的 CPU 生成与 GPU 读回是正确性检查，不是渲染性能基准。
- `--performance` 的数字只在同一台机器、同一构建、同一参数下对照有意义，不应单独作为性能结论。
- 没有 PNG、JPEG 或视频解码。
- 内存统计只覆盖应用登记的资源与预算，驱动内部与整机内存不可见。
- 无 `VK_EXT_swapchain_maintenance1` 的 `QueueIdle` 呈现证明路径，其覆盖情况见交换链 README 与 [VALIDATION.md](VALIDATION.md)。

## 相关文档

- [VALIDATION.md](VALIDATION.md)：当前验证状态与未完成项。
- [交换链 README](../../src/present/swapchain/README.md)：`SwapchainPolicy`、呈现证明与退出语义。
- [rhapsody README](../../README.md)：crate 总入口。
