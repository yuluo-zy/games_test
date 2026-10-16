# Window Lab V0

rhapsody 的真实窗口测试素材播放器，也是 [V0–V9 交付计划](../../DELIVERY_PLAN.md) 的共同验收入口。

## 启动

在 `D:/game/tiny-glade`（Cargo workspace 根目录）运行：

```powershell
cargo run -p rhapsody --example window_lab --offline
```

需要 Windows 桌面及支持当前 rhapsody 必需特性的 Vulkan 驱动。无设备/能力不足时明确失败并写报告，不自动切到软件绘图。其他系统当前明确退出，不以 headless 替代窗口。

有限帧自动验收（360 帧覆盖默认 4 段素材）：

```powershell
cargo run -p rhapsody --example window_lab --offline -- --frames 360 --script --report target/window-lab/baseline/report.json --capture target/window-lab/baseline/readback.ppm
```

严格验证层验收在以上命令末尾添加 `--require-validation`；缺少 Khronos 验证层时非零退出，普通模式也在报告里明确记录层是否启用。

## 播放与素材

- Space：暂停/继续素材时间轴；窗口仍持续呈现同一帧。
- N：下一段素材。
- Right：推进素材一帧（暂停时用于检查）。
- Esc / 关闭窗口：正常退出，写报告；有限帧运行被提前关闭则判失败。
- `--fps 30`：素材固定步长的目标播放率，实际呈现可能受 vsync 和验证开销限制。
- `--playlist PATH`：加载自定义 JSON 播放列表，图片路径相对于列表文件。

默认素材为 RGB 色条/灰阶、移动棋盘、移动圆形渐变及真实 PPM 文件。左上红色、右下蓝色角标用于检测方向。图片统一以最近邻缩放到 320×180 再显示到窗口；V0 不做保持宽高比/色彩管理验收。sRGB 交换链的 blit 转换由 Vulkan 格式处理，CPU 参考比较针对 RGBA8 UNORM 离屏图。

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
  → Win32 surface acquire → rhapsody 呈现命令（blit + PRESENT 布局）
  → submit_and_present → 帧完成 → 下一帧
```

`--script` 在第 30 帧缩小窗口、第 60 帧最小化后恢复、第 120 帧放大窗口；读取真实 client extent 并统计零尺寸跳帧。脚本有限帧模式至少 180 帧。窗口连续 30 秒无呈现进展时，有限帧模式失败。

每帧检查 GPU 离屏读回；窗口呈现调用与人工/桌面截图检查分开登记。读回完全一致不能独自证明最终显示颜色、缩放和方向正确。

## 报告与限制

报告默认 `target/window-lab/report.json`；可指定独立输出目录。记录设备、驱动、验证层、素材指纹、帧数、窗口事件、交互计数、输入/读回哈希和错误。查询启动时 Git revision 与 dirty 状态，仅说明运行环境，不等于编译产物精确源码哈希。诊断只保存前 32 条错误/警告文本，累计数量完整；CPU 耗时最多保留最近 10000 个样本并分批淘汰。

`status=pass` 仅表示该次请求的自动检查通过，`window_visual_check` 默认 pending，最终 M0 还需独立窗口检查记录。缺失指标用 null。未到请求帧数、脚本未执行、像素差异或 validation error 均失败；不把不支持/跳过计入通过。

V0 是功能基线：离屏图与呈现分两次提交、每帧 CPU 生成图像并等待 GPU 读回、展示命令逐帧分配、使用现有交换链保守同步。耗时包含 CPU 图案生成/等待/驱动呈现，不可冒充框架性能基准；未实现新 PresentTicket，不能据此认证 M2。V2 将适配生产 FrameLoop，V3 加 GPU 光栅/temporal，V4–V8 逐步加入压力、缓存、着色和多队列场景。

## 开发验证

```powershell
cargo test -p rhapsody --example window_lab --offline
cargo build -p rhapsody --example window_lab --offline
```

单测覆盖 PPM 截断/越界、二进制像素恰好为空白字符、播放列表及动画确定性。窗口运行结果另记在 [验收记录](VALIDATION.md)。
