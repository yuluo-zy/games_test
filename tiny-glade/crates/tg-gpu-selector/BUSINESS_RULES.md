# tg-gpu-selector：B 版业务规则与恢复边界

本 crate 的游戏自有逻辑是：把 GPU/driver 状态呈现给玩家、采集选择结果、生成驱动建议和图形错误提示，以及编辑 Vulkan allowlist 与六项 workaround。Vulkan 枚举、driver classification、底层 GPU 初始化和 iced 控件实现由原依赖提供；这里恢复调用这些依赖的游戏策略，不重写它们。

## 证据身份

- 基线：Tiny Glade B patched 程序 SHA256 `cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`；发行 RNE oracle SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 已保存的原函数范围、反编译和字节哈希见 `reverse-engineering/tiny-glade/evidence/foundation-unstarted-2026-10-13/ui/`；原 GPU/workaround update 叶分支输出见 `original-message-cases.json`，旧版本曾将其映射到生成式 fixture；当前源码改用业务测试，不再保留要求非法输入成功的 fixture。
- 函数原型记录有 Ghidra `void()` 占位，不将它当作 Rust ABI。本文只认代码体、实际调用点和受控原指令案例支持的行为。

## 已恢复的 B 版规则

### GPU 选择与建议

- `gpu_selection::Controls::update`（原入口 `0x1421bb740`）对 `Select(index)` 保存 `Some(index)`；`ShowMoreInfo` 开启附加信息；URL、日志路径和关闭动作交给 GUI runner。已保存的 RNE 叶案例覆盖所观察的选择、更多信息、关闭和空操作分支，包含选择索引的边界值。它不是完整窗口更新函数的全输入穷举。
- Vendor 到驱动页的 B 映射是 NVIDIA、Intel、AMD、Qualcomm 各一条已观察链接；Other/Unknown 没有链接。legacy Intel 不沿用通用 Intel 页，而使用 Intel 7th–10th Gen 页面。
- `device_driver_advice`（原入口 `0x1421be370`）依据上游传入的 support 分类选择 key、最低/推荐版本和链接：Unsupported 使用 minimum；UnsupportedLegacyIntel 使用 minimum 和旧 Intel 链接；Emulated 清空链接；UnknownVendor 显示小写十六进制 ID 并清空链接；KnownBad 使用 known-good 版本；Supported 不显示版本要求并清空链接。其它状态保留按 vendor 预选的驱动链接。
- 图形错误以原始 tag 映射本地化 key：`0x0b/0x0c` 为 NoCompatibleDevices；`0x10/0x11` 及未知 tag 为 NoDriver；`0x12` 为 UnsupportedWindowingSystem；`0x14` 为 SwapchainError，附 details 并使用 SwapchainErrorTip；`0x15` 为 OutOfVideoMemory，附 details 和专属建议；`0x16` 为 InternalError 并附 details；`0x17` 为字面标题 `Unexpected error`，附 details 且没有建议。其余已映射类别默认显示 PleaseUpdateDrivers。

### Vulkan allowlist 与 workaround

- `Controls::new` 按 layer 首次出现顺序分组；相同 vendor 的 layer 合并到首个组，`None` 也是一个可合并组。原层输入顺序在各组内保留。
- allowlist 条目带有 Layer/Vendor 区分。toggle 会删除所有与被切换的同一变体相等的条目；若一个也没删，则追加该条目。不同变体即使字符串相同也互不影响。Apply 克隆当前完整 configuration（包括目前语义不明的 `policy_word` 和六个开关）；Cancel 请求关闭窗口。
- 六项 workaround 的固定索引/标签顺序：sync before each render graph pass、sync before each draw、sync before each compute shader dispatch、serial command buffer recording、skip loading screen、vendor-specific crash breadcrumbs (NVIDIA Aftermath)。B 原消息叶案例验证每个索引分别异或对应 bool；默认候选值为 false。
- 原 update 对 workaround 索引直接按消息 payload 定位字节，调用方生成的六个 checkbox 只产生 `0..6`。候选实现对越界索引安全忽略；不把无效手造消息当成正常原作输入合同。

## 当前工程化规则（源码审查后）

上述 B 版叶分支是历史证据，不是当前安全 API 对非法消息的承诺。当前选择使用 `DeviceId`，仅接受列表内且无初始化错误的设备；有效选择与 Close 都关闭状态机。Vulkan 使用 `Workaround` 枚举，Apply/Cancel 后停止编辑。两套 Controls 的状态为只读视图，后续消息不能重写结果或重新执行外部 IO。分组使用保序 Vec 和 vendor 到组索引的 HashMap，避免反复线性扫描。原始 driver/error 映射保留；展示数据明确区分最低版本、推荐可用版本和未知 vendor ID。

## 当前 Rust 表示

`gpu_selection.rs` 和 `vulkan_config.rs` 是按业务行为恢复的 Rust API，不是作者原始 Rust 文本。`GraphicsInitError { tag, details }` 保留错误的原始 tag 与文本，但没有冒称与原始跨依赖 enum ABI 相同。Vulkan `policy_word` 也作为不透明 `u32` 原样传递，没有猜字段含义。

## 未闭合范围

- **上游/调用方接线：**实际 rhapsody AvailableGraphicsDevice、driver_support 分类到本 crate 输入 DTO 的生产者，以及 Apply 结果如何送回真实 device 创建流程，仍属于 Rhapsody/游戏启动调用链。该逻辑不在本 crate 里臆造，也不重写 Rhapsody。
- **ABI 与 GUI：**`GraphicsInitError` 和 driver-support 的原始 Rust 类型/ABI 未完整恢复；iced 原 view 的精确元素树、样式和像素也没有与发行程序完整差分。当前 native GUI 是使用原 iced fork 的候选视图实现。
- **版本/动态运行：**只覆盖 B；A/C 的独立行为、真实设备列表到用户选择再到 Vulkan device 创建的整链，以及所有窗口事件交互均未验收。

因此，本 crate 当前候选实现覆盖已观察的游戏自有策略；以上列出的上游接线、原 ABI 和精确 GUI 仍单独保持未验收，不把局部叶案例或可编译性提升为整条启动链等价。
