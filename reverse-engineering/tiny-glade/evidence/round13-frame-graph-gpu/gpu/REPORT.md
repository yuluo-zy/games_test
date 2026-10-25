# GPU 宿主与末端所有权审查

本轮仅 B。T13 按用户要求排除。沿用当前 rhapsody，不重建 legacy；worker 未运行 Cargo、未启动逆向后端、未直接修改生产源码。

## 交付

1. `production-renderer-win32.patch`：Windows 宿主此前虽然声明 `VK_KHR_win32_surface`，但 `PlatformSurface::new` 对所有非 macOS 目标固定报错。候选复用当前 ash 0.38.0 的 `win32_surface`、原锁 winit fork 的 HWND/HINSTANCE 和现 `rhapsody::present::Surface` 析构 owner。resize/resume 验证同一个窗口，交换链尺寸仍交现有逻辑。此项为工程平台接线，不算原 B 算法恢复。
2. `frame-driver-loading-wsi.patch`：现有 `original_loading_present` 的限定加载场景夹具改为 `App`，通过实际 `GameFrameDriver::frame` 执行 `App::update`、资产维护、MEI 刷新、production factory、原 `render` 和实际 WSI 呈现。保留三帧真实 present 与正常 shutdown 验证。此夹具不运行完整 gameplay loader，也不证明非空世界图或完整玩家交互。

两项候选均 `rustfmt --edition 2024 --config skip_children=true --check` exit 0；基础及候选 SHA 在各 manifest。Cargo、实际 Vulkan/WSI 的结果由 root 统一记录，当前不预记通过。

`nonempty-gameplay.patch` 与其 manifest/candidate **已撤回，不集成**：root 已指定 `nonempty_acceptance` 独占这个测试，正式非空分层断言由该 owner 交付。撤回候选保留作历史，无生产修改；其全文件 fmt 被其它 owner 新增尚未格式化 helper 影响，不计格式通过。

## 真实验收入口

在活动 Windows 桌面和现有 Vulkan 驱动中运行（会打开 480×270 加载验证窗口，三帧后退出）：

```powershell
$env:TG_INSTALL_ROOT = 'D:/game/ljxsj_92385/Tiny Glade'
cargo test --manifest-path D:/game/tiny-glade/Cargo.toml -p tiny-glade --test original_loading_present --locked --offline
```

该入口 `harness=false`，GPU 初始化、surface、swapchain、录制、提交、呈现或 shutdown 失败都返回错误，不跳过设备。桌面不可访问属于该项环境条件，不把 headless 成功替代 WSI。

非空原场景入口仍为：

```powershell
$env:TG_INSTALL_ROOT = 'D:/game/ljxsj_92385/Tiny Glade'
$env:RHAPSODY_REQUIRE_VULKAN = '1'
cargo test --manifest-path D:/game/tiny-glade/Cargo.toml -p tiny-glade --test original_gameplay_chain shipped_session_through_actual_app_and_production_world_graph --locked --offline -- --exact --nocapture --test-threads=1
```

现有此入口执行真实 App update 与生产 full-world warmup，尚不是 `GameFrameDriver::frame` 的非空世界呈现验收；不能将 warmup 报告成完整帧循环。

## 已核对的边界

- `CoreRenderBridge::compile_built` 仅 compile/allocate；执行入口是 `execute_compiled -> VulkanGraphSession::execute_prepared`。已有 `compile_before_submit` 测试通过真实物理 copy/readback 检验编译未提交、放弃图仅 retire 一次和下一图写入；本轮没有重跑，不引用为本轮验证通过。
- 生产主图输出通过 receipt 的 logical→physical mapping 定位，并要求 logical export；`take_exported_texture` 将实际 `Arc<Texture>` 交下一消费者，同时释放后端旧 export token，避免每帧泄漏。回调/command 的资源 owners 由当前 rhapsody 持有至真实完成。
- resolve 使用 `import_native_texture_owned(source, 0x22)` 导入同一实际图像，swapchain acquisition 交同一 session 的第一类 present target；当前 session 负责实际跨图 physical dependency、acquire wait 与 present signal。未新加 CPU idle 代替同步。
- `ProductionRenderer::collect` 轮询所有 App session、present tickets、swapchain，再回收设备垃圾；`shutdown` 等待实际提交与 present tickets 后清理附件缓存/图资源和设备。worker 静态审查未发现需要恢复旧 rhapsody 的理由；实际多帧与失败路径仍须 GPU 执行验收。
- 非空原场景旧 v14 在音频资源断点失败，`startup-baseline-v20.log` 则为 decorator event 同时 EventReader/EventWriter 的 Bevy B0002，尚未到世界 GPU。必须保留这些失败，不能归因 shader 或据此跳过系统。
- graph owner 独立发现 effects target 的 sample_count 1/4 错配并准备修复；该部分不在本 worker 文件范围。

API 实证直接读取本机锁定依赖：ash `vk/platform_types.rs` 的 HWND/HINSTANCE 均为 isize，`Win32SurfaceCreateInfoKHR::{hwnd,hinstance}`；winit fork `d03de0a/src/platform_impl/windows/window.rs` 的 rwh_06 从真实 HWND/GWLP_HINSTANCE 生产这两个非零句柄。未新增依赖或自行实现平台库。
