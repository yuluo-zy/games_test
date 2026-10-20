# macOS Surface native owner 窄补丁

候选 `candidate.patch` 只在现有 Surface 增加 `from_raw_retained` constructor，并在现 macOS PlatformSurface 保存/传递真实 `Arc<winit::Window>`。base 和 candidate SHA 在 manifest，基于 root 已合入 Win32 bounded native owner 后的生产快照。未改生产，未运行 Cargo 或 macOS GPU。

保留当前 CAMetalLayer 创建、StrongPtr、setLayer/setWantsLayer、bounds/contentsScale/drawableSize 和 Vulkan Metal Surface 逻辑。不把 objc::StrongPtr 或原始 NSView pointer 强行标为 Send/Sync；Surface 的同一个有界 `Any + Send + Sync` owner 槽仅保存 Window Arc。

## 证据与所有权

- 原锁 winit fork `d03de0a/src/window.rs` 的公开 Threading 合同明确 Window 为 Send + Sync；同文件 Window::drop 转调平台主线程方法。`platform_impl/macos/window.rs` 中平台 Window 持 `MainThreadBound<Id<WinitWindow>>` 与 delegate，Drop 通过 `get_on_main` 关闭；rwh_06 的 ns_view 来自这个窗口的 contentView。没有另造 NSWindow/NSView owner 或绕过原线程代理。
- 本机锁定 objc 0.2.7 `src/rc/strong.rs` 的 StrongPtr::new 接收已经 +1 的对象，Drop 调 objc_release。这说明原 PlatformSurface 的 StrongPtr 只保证它自身存在时持 layer；当 WSI 安全保留 Surface 而释放 PlatformSurface 时，原 +1 会释放。
- Apple 的 [NSView.layer](https://developer.apple.com/documentation/appkit/nsview/layer) 合同将该属性作为 view 的实际 backing layer。官方 [Layer Hosting 设置说明](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/CoreAnimation_guide/SettingUpLayerObjects/SettingUpLayerObjects.html) 用 autoreleased layer factory 的结果直接 setLayer 并随后显示 view；结合 [CAMetalLayer 的 AppKit 接线](https://developer.apple.com/documentation/quartzcore/cametallayer)，这支持 NSView 持有所赋 layer 的语义。这里的持有链推断为 retained Window → native NSWindow/contentView → 当前 hosted CAMetalLayer；它不是在 Windows 主机上执行过 Objective-C retain-count 探针。
- 当前 host 对这个 view 只安装一次 CAMetalLayer，没有后续 setter 替换它。候选保留这一关联。Surface 的 raw-retained 安全合同要求传入 owner 确实保住底层平台对象；其它 raw/interop 调用者仍承担其原始 unsafe 合同。

正常析构时 Surface::drop 先执行 vkDestroySurfaceKHR，随后单槽 native Window Arc 才释放。失败 WSI teardown 仍沿现 Swapchain::drop 保留同一 Surface，因此 Window/contentView 的 hosted layer 关联也继续存活；PlatformSurface 的独立 StrongPtr 可以正常释放。没有额外 native owner 集合、全局泄漏、线程不安全 wrapper 或 Device 策略变更。

## 验证边界

Rustfmt 已解析两候选文件；rhapsody 文件完整 check 通过。宿主完整 check 的 diff 仅位于 root 同时整合的 world target/screenshot 段，macOS 模块自身另作限定格式检查，不把其它 owner 的未格式化片段改进本补丁。

本 Windows 主机不具备 macOS target/AppKit/Metal 运行验收，不宣称 Mac 编译或 GPU 通过。必要后续为 macOS 主机的 `cargo check -p tiny-glade` 和现 `original_loading_present` 主线程 WSI 验证；本候选只补实际 native owner 生存期，不代表 Surface 全平台/全部异常分支已运行验证。
