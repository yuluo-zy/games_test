# Mac 与 Windows 显示颜色管理补充 V1

- 原B是Windows发行程序。ICC的Win32来源使用已存在get_monitor_icc_profile；核心生成算法在支持lcms2的主机可运行。
- 当前 PlatformSurface::new/update（production_renderer.rs1530起）只设置layer/view/bounds/scale/drawableSize，没有自行选择CGColorSpace。不能据此认为最终present没有OS颜色管理，因为swapchain创建在其后由MoltenVK设置。
- 官方 MoltenVK v1.4.2 `MVKSwapchain.mm`474..478：imageColorSpace为SRGB_NONLINEAR_KHR时设layer的colorspaceNameMVK为kCGColorSpaceSRGB，并关闭EDR。来源 https://raw.githubusercontent.com/KhronosGroup/MoltenVK/v1.4.2/MoltenVK/MoltenVK/GPUObjects/MVKSwapchain.mm （2026-10-21 web读取）。这是该tag源码证据，不能当已安装dylib确切源码提交的证明。
- Apple文档 https://developer.apple.com/documentation/quartzcore/cametallayer/colorspace 明确colorspace非nil时CoreAnimation对合成内容做需要的颜色空间转换；nil表示不做匹配。因此“GPU先转换sRGB到display ICC、再以sRGB交由合成器转换”会重复管理，是现设计必须避免的路径。
- 合法Mac平台策略：生产swapchain实际选择SRGB_NONLINEAR，且实际layer颜色空间由驱动设置为sRGB时，显示器转换归系统；不用Win32 ICC查询，不伪造profile或发送None事件清除用户状态。应记录实际swapchain格式/颜色空间与layer颜色空间作为WSI验收证据。若实际选择不同颜色空间，先核对应驱动/OS合同，不默认当SRGB。
- 显式注入的ICC事件仍完整执行原CPU/GPU算法，可用于校准工具和独立headless正确性验收；它不等于在系统已管理的正常Mac窗口自动再套显示器profile。
- 本机原crate校验通过。lcms2-sys4.0.1 vendor/include/lcms2.h为LCMS_VERSION2150，本机系统little-cms2为2.19.1；核心依赖启static以保原vendor算法。没有运行Cargo/GPU，没有验证实际layer值。
- 第一小包先交核心producer→完整变换→GPU纹理owner→finalpost消费者；Windows tracker API及窗口owner的真实调用点随后独审接入。该分阶段不称OS来源已完成。
