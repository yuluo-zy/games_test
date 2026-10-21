# 实际交换链/Metal颜色空间只读诊断 V1

root授权；窗口owner确认当前没有改 raw/swapchain struct/ctor、PlatformSurface 或 ProductionRenderer候选。只在合法ICC work出候选，不生产/Cargo/GPU。

- raw/swapchain 保存实际传给 vkCreateSwapchain 的 selected SurfaceFormatKHR.color_space；初值None，不伪造0。与已有self.format同成功发布点写入；readonly surface_format()提供当前实际元数据，不重查询支持列表/猜选中结果。无创建策略、锁、预算、错误或时序改动。
- ProductionRenderer new 创建真实swapchain后、创建ApplicationGpu前，Mac调用诊断；env TG_COLOR_SPACE_DIAGNOSTIC 存在才读framework。off只缓存一次env查询，稳态不读取layer/创建CF对象/格式化输出，无每帧探针。
- CAMetalLayer.colorspace属性为Get借用；现有StrongPtr保layer。非null按CF Get规则retain→RAII CFType，nil记false，绝不拿borrow按Create规则释放。框架CGColorSpace::create_with_name(kCGColorSpaceSRGB)自己RAII释放，再以CFType的CFEqual比较真实身份，不以指针相等或裸字符串断言sRGB。
- 日志携实际selected format/colorSpace与layer_present/layer_is_color_space/layer_is_srgb。Mac不把非null当作sRGB证据；预期真实WSI应为color_space0且两个identity true。其他值保原日志，不改颜色、不绕过错误。
- 使用当前锁已有core-foundation0.9.3/core-graphics0.23.1/foreign-types0.5.0；只新增直接声明，不升级依赖。现有objc Getter所需unsafe和CF借用封装在平台边界，均附中文生命周期说明。启用诊断时framework/selector错误正常传播；关闭不新增错误。
- 仅创建时样本，不声称所有resize代/所有driver源码提交相同。新元数据随recreate更新，后续验收可再次调用实际诊断；全生命周期由窗口owner负责。
- 原B没有Mac平台分支，本包是明确的工程诊断，不是恢复进度。没有Cargo/OS/GPU执行，无像素正确性结论。
