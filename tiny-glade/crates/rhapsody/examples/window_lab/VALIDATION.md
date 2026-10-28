# Window Lab 验证状态

本页只记录当前状态。用法见 [README](README.md)，交换链语义见 [交换链 README](../../src/present/swapchain/README.md)。

## 检查项

| 检查 | 命令 | 说明 |
| --- | --- | --- |
| 示例构建 | `cargo build -p rhapsody --example window_lab --offline` | 所有平台可运行。 |
| 单元测试 | `cargo test -p rhapsody --example window_lab --offline` | 仅 Windows 与 macOS 编译运行，其他平台 0 个测试。 |
| 功能套件 | `python3 crates/rhapsody/tools/window_lab_checks.py --build --suite functional --output <dir>` | 依次运行多组 GPU 场景，耗时长。 |
| 性能套件 | `... --suite performance` | 依赖历史对照二进制，见 README 的验证脚本一节。 |
| 长跑套件 | `... --suite long-run` | 默认每组至少 30 分钟。 |
| 窗口显示检查 | 人工 | 报告中的 `window_visual_check` 固定为 `pending`，需人工或桌面截图确认最终显示的颜色、缩放与方向。 |

## 当前已验证

| 平台 | 项目 | 结果 |
| --- | --- | --- |
| Linux | `cargo build -p rhapsody --example window_lab --offline` | 通过（本次运行）。 |
| Linux | `cargo test -p rhapsody --example window_lab --offline` | 编译通过，运行 0 个测试（模块被平台 `cfg` 排除）。 |

## 未验证

- Windows：窗口运行、功能套件、性能与长跑套件均未在当前源码上运行。
- macOS（MoltenVK）：窗口运行与功能套件未在当前源码上重新运行。早期运行的证据目录已删除，无法复核，因此不作为当前结论。
- 单元测试在 Windows 与 macOS 上未在本次运行中执行。
- 窗口显示检查：未进行。
- `tools/window_lab_checks.py`：仅核对了参数定义，未运行。
- 无 `VK_EXT_swapchain_maintenance1` 的 `QueueIdle` 呈现路径：交换链 README 记录过一次临时关闭扩展后的测试运行，该结果不计为 Window Lab 验收，且本次未重跑。
- `--require-validation` 在有验证层的环境下的完整通过路径：未验证。

## 仍待完成

1. 在 Windows 上运行单元测试与主要场景（`showcase`、`fixture --script`、`--wsi-stress`）。
2. 在 macOS/MoltenVK 上于当前源码重跑窗口与功能套件。
3. 运行 `window_lab_checks.py --suite functional` 并保存结果。
4. 完成窗口显示的人工检查。
5. 在无 `maintenance1` 的设备上运行 `QueueIdle` 呈现路径的窗口验证。
6. 性能与长跑套件的对照结果需在当前构建上重新产生。
