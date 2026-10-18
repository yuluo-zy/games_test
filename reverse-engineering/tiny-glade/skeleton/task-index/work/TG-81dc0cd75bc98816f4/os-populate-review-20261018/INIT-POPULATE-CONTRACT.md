# init_populate_ssbo_library 原输入与启动副作用

仅 B。函数 `country_core::resources::init_populate_ssbo_library@140909e80`，完整 PDB 长度 2540B。本次复用 parent 的 `main-chain-frame-20261018/populate-ssbo-original.json`、render owner 的完整 RNE `TG-eaa97322db04d63e0d/render-prelude-indirect-20261018/init-populate-original{.asm,-calls.json}`，并按 SHA 绑定原 RNE 重新保存完整 652 条指令到 Nani owner 的 `TG-492fe68933dd29b2b6/frameinputs-20261018/evidence/140909e80.asm`。不是旧 Ghidra 截断体。

## 真实 ECS 输入

FunctionSystem `1408ab230`/`1408ab530` 的完整泛型 PDB 名称保存在 `original-ecs-input-identities.json`，准确参数为：

```text
Res<GraphicsDeviceResource>
Res<AssetSsboLibrary>
EventWriter<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>
```

原函数保存 R8 事件 writer；RDX 指向 library Res；RCX 指向设备 Res。每次注册传入其设备 owner 内 +0x10 的实际设备。没有 CPU TerrainHeightsData 参数；CPU height 初始化另在 `systems::terrain::insert_terrain_resources@1409457c0`，不能用本函数冒充 CPU terrain producer。

## 注册顺序与数据边界

原始逐 call 表是 72 项：69 次 add_new(None)、3 次 add_new_with_data。完整顺序、canonical 类型身份和源码定位在 `REGISTRATION-MAP.md`。该表最初源码快照早于 Nani 修复，Nani 七项已被 `registration-current-source-map-after-nani.json` 和 Nani 合同替代。`missing_direct_definition_or_macro_only` 只是搜索分类，不是缺失结论，也不能按数量宣称恢复进度。

Nani 固定四项原位于序号 62..65；66..68 是 BushCpu/RippleSource/Sparkle；69..71 才是 Nani 三个真实动态 Vec；72 是 CustomCursor。Nani 元素/存储/NAME/BUFFER_LEN/usage/真实向量来源已按完整原构造恢复生产，见 Nani 合同。其聚合子集 producer 不是完整函数，完整函数不得因为有这个接口就省略其余 65 项或声称原整体顺序闭合。

本函数调用 add_new(None) 不等于给该缓冲填零。None 行为由原具体 SsboType 决定：存在映射存储、固定不上传、真实 default 上传、稀疏存储和带计数的 InstanceBuffer。当前宏定义也是真类型，不能因没有字面 `pub struct Name` 判成缺失。已明确识别的现宏族包括 world_raster_defs 六个 staging（u8×512²）、terrain_height 两个 readback（u32×0x20000）、indirect_params 四个真实参数载荷；它们各自原布局验证归相应 owner。

精确类型接线仍不能直接完成的例子：当前 Ivy 原类型由 IvyLeafGpu/IvyFlowerGpu/IvySegmentGpu 替代，库 NAME 是完整类型路径，需核与原身份对应；BrickDrawList/Prev/NextVisibility、NormalDebug、RoofSupportBeam、StairsCounter 等原 canonical 元素在对应源码族没有可直接注册的定义或只剩 consumer tag，不能用同名 tag 推断已经存在正确存储。该表不把这类源码问题和尚未审核的宏混在一起计数。

## CustomCursor 是完整函数的必要末端副作用

`14090a532` 调 `add_new<draw_passes::custom_cursor_pass::CustomCursorSsbo>`，接着以原 key/name 做实际库 lookup/downcast。`14090a5a9` 调 `Buffer::mapped_slice_mut@140bd4cd0`，检查映射起始按 4 对齐、长度是 16B 的整数倍且恰好一条 16B 记录。

`14090a5e2..5e9` 更新事件 writer 资源变更 tick；`14090a617` 把映射内存指针写入事件 payload，`14090a61b` 写 EventId，队列步长 16B。这是 `CustomCursorSsboCreatedCmd`，不是 Ghidra ICF 别名误标的 WindowFocusEvent。必须保有实际缓冲/设备映射生命周期，不能从临时 CPU 数组取地址或发送 null。

原 byte constructor `1416146c0`，531B，实际缓冲 16B，usage 0x22、memory 2、alignment 4，None 时无初始上传。当前 `custom_cursor_pass.rs` 的 `CustomCursorSsbo(pub typed_gap::Gap)` 只有 graph tag；canonical mouse_capture 还没有此创建事件身份。因此完整启动需要恢复真实元素和存储、实际映射 lease、事件 producer→同一 MouseCapture consumer；用 fake pointer 不能通过。

## 当前交付与未闭合范围

已交付可审核的完整原调用表、准确 ECS 身份、Nani 七项真实生产代码、CustomCursor 后续必需副作用合同。没有新增名为完整 init_populate 的部分函数，也没有修改 rhapsody。其余各注册项必须按真实元素与现库 API 逐项接入；本合同不把 72 项原符号库存当作 72 项已恢复业务。

无 Cargo/GPU 执行；root 统一编译和实际 GPU 验证。共享 Ghidra 本轮已出现 BufferMgr corruption，继续采用已保存完整查询和原 RNE/PDB 范围，工具修复不算业务恢复。
