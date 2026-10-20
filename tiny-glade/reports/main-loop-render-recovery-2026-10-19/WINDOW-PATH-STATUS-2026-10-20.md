# 窗口渲染路径现状与进度（2026-10-20）

范围：渲染主路径 + 主循环。存档/历史/回放链路按用户决定搁置。

## 已完成并验证
- 整帧执行：480×270 整帧（lavapipe）跑通，校验层 0 VUID；树/草/墙像素正确（见 `pixel-verification/`）。
- 多帧稳态：计划缓存命中、显存不增长（`MULTIFRAME-BASELINE-2026-10-20.md`）。
- Linux 窗口路径：winit 原始句柄（Xlib/Xcb/Wayland）→ ash khr surface → `rhapsody::present::Surface::from_raw`；窗口创建、首帧、loading→游戏流程可走通。
- 默认窗口化（不全屏）；`TG_FULLSCREEN` 才全屏；`TG_WINDOW_SIZE=WxH` 指定窗口大小。
- 软件渲染设备（CPU 类型）GPU 等待上限放宽到 180 s；管线编译队列满时背压重试（180 s 截止）。
- Resized 事件同步 `FrameRenderConfig` 的 extent，再调用 `renderer.resize`。
- 每 30 帧输出 `production frame progress`（completed/gameplay/submissions/presents）。
- 绑定描述符耗尽错误信息带 live/retiring/capacity。

## 本次实测（优化构建，480×270，lavapipe，X11）
- 进入游戏后玩法帧持续提交并排队呈现：第 240 帧时 `gameplay_frames=28, submissions=28, presents_queued=28`。
- 约第 240–420 帧之间崩溃：`容量耗尽：bindless StorageBuffer descriptor table is full`（16384 项）。
  - 未闭合：是租约泄漏（缓存/计划长期持有）还是单帧需求随场景增长，尚未判定。已在错误信息中加入 live/retiring 数，下次复现可直接区分。
  - 单帧 lavapipe 世界帧约 5 s，复现需约 20 分钟。

## 未闭合
- 上述 bindless 存储缓冲描述符耗尽（排查已按用户要求暂停）。
- 羊渲染（过曝 + 几何退化）：agent 进行中，`view_constants.rs`、`raster_host.rs` 中的诊断改动未提交。
- 窗口生命周期：挂起/恢复、焦点、vsync、交换链 out-of-date 重建、DeviceLost/OOM 重试、AdaptiveRenderState 均未实测。
- 窗口内容是否为玩法画面（相对菜单背景）未做对照截图。
- 渲染尾部：loading_screen 进度、Ivy 插件、ICC 自定义 LUT、color_menu、未注册着色器（sun_gizmo/tree_billboard/rain_particles/waterfall/roof_snow）、PhotoModeCmd(4) 生产者。
- 墙体 `todo!` 系统、地形/屋顶宿主系统。
- 无真实 GPU / Mac / Windows 验证；CPU 录制 180–560 ms/帧未在真机度量。

## 本次提交范围
`production_renderer.rs`、`render_host.rs`、`main.rs`、`event_loop.rs`、`rhapsody/src/bindless.rs`、本文档。
不提交：`raster_host.rs`（含 TMPDIAG）、`view_constants.rs`（羊 agent 在改）、两个 CRLF 文件、截图 PNG。
