# 焦点与独占全屏库内修复

范围：rhapsody 的生产交换链与 FrameLoop。窗口样式、显示器选择、分辨率切换及 OS 事件分发仍由宿主负责。本报告不代表整个模块或 M2/M9 交付完成。

## 实现

- `FrameLoop` / `Swapchain::set_focused(bool)` 接受焦点状态；失焦及零尺寸时停止新 acquire。已有图像租约仍可完成呈现，全部归还后，下一次 acquire 或 presentation collection 释放独占。恢复焦点时重试请求的独占模式。
- `set_window_mode(WindowMode)` 保存宿主请求，下一次 acquire 使用原代际协议重建；状态查询区分 requested/focused/acquired/fallback。原来只标 stale 的 `on_focus()` 已删除。
- Windows 设备创建仅在设备扩展和实例依赖齐备时自动启用 FSE；surface 必须由 `from_win32` 或显式的 `from_win32_raw` 标注。普通 raw surface 不推断平台。
- FSE 创建策略与 capabilities2/formats2/presentModes2 查询使用一致的 monitor 和 mode；普通呈现显式 DISALLOWED，独占使用 APPLICATION_CONTROLLED。先查询支持，再创建，创建完成后才尝试获取独占。
- 不支持、独占创建/获取拒绝、native acquire/present 报独占丢失时，锁存原因并转为普通呈现代际，避免无条件逐帧重试。显式模式请求或真正的失焦后恢复才重试。OOM、SurfaceLost、DeviceLost 保留错误；释放失败保留独占所有权供重试。
- 独占操作只作用于有效代际；重建失败也会退役 oldSwapchain，后续创建使用空 oldSwapchain。进入 native 创建之前失败的能力查询或零尺寸检查不伪造退役。新链的 ancestry 容量提前预留，仍保留旧代际的 acquire/render/present 完成协议。

## 验证与边界

最终命令、退出状态和四个实现文件的前后 SHA-256 见 [checks.json](checks.json)，日志为 `lib.log`、`clippy.log`、`no-default.log`、`window-build.log`。指纹范围仅为本次功能的四个实现文件，不是整个工作区的冻结验收。

[最终结果](validation.json)：全特性库单测 322/322，严格全目标 Clippy、无默认特性全目标检查及窗口构建通过；四个实现文件前后指纹一致。最终窗口二进制 SHA-256 单独保存。macOS 64/64 帧呈现及像素检查通过，validation error/warning 均为 0，关闭完成、保留 WSI 代际为 0。

CPU 检查覆盖独占 acquire/release 幂等性、已退役句柄禁用、获取拒绝与 OOM/DeviceLost 区分、释放失败后重试、不支持扩展和失焦时不调用扩展、焦点恢复重试锁存状态，以及生产 acquire/present 状态机中的租约排空、零尺寸和 native exclusive-lost 呈现等待消费证明。

macOS 普通 Vulkan 窗口回归见 [macos-window.json](macos-window.json)；这是非 FSE 路径的呈现、像素及关闭回归，不是独占模式验证。

**BLOCKED：Windows 原生独占获取/释放、显示器切换、驱动焦点丢失和独占恢复。** 本机为 macOS/MoltenVK，无 Win32 显示环境。CPU 故障注入与 macOS 普通窗口不能替代这组验证。

## 规范依据

- [独占获取](https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireFullScreenExclusiveModeEXT.html)：成功后持有独占；已独占或已退役的交换链不可再次获取。
- [独占释放](https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseFullScreenExclusiveModeEXT.html)：释放后需要重新获取才可继续呈现，因此库不会直接在已释放的独占代际上走普通呈现。
- [交换链创建](https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainCreateInfoKHR.html)：native 创建调用失败也会退役非空 oldSwapchain；重试不能再次传入已退役句柄。
