# B Nani 启动 SSBO 闭包

当前 owner 为 `frameinputs-20261018`，文件族 `TG-492fe68933dd29b2b6`。生产文件是 `country-core/src/render/nani/buffers.rs`；原版 B RNE SHA256 为 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`，匹配 PDB SHA256 为 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。完整 18 段原始指令范围及其哈希在 `evidence/native-manifest.json`，原字节 NAME 另在 `evidence/native-name-values.json`。

## 原始输入、类型与创建策略

`init_populate_ssbo_library@140909e80` 的有界 Nani 部分是下面 7 项。`NAME` 与下表类型短名逐字相同，已从各构造器传给 `create_buffer` 的 RIP 字符串读取，未按 PDB 类型名推测。

| 原调用 site | 类型 | 元素/BUFFER_LEN | 原存储 | usage/memory/alignment | 无初始数据时 |
|---|---|---|---|---|---|
| 14090a287 | NaniDrawListLengthSsbo | u32 / 108 | TypedStorageBuffer 40B | 0x122 / 1 / 4 | 创建 432B，不上传 |
| 14090a297 | NaniDrawListOffsetDefaults | u32[108] / 1 | InstanceBuffer 48B | 0x12 / 1 / 4 | default_value 真正计算 108 个偏移，上传 432B，count=1 |
| 14090a2a7 | NaniInstanceIndirectionAllocatorSsbo | u32 / 108 | TypedStorageBuffer 40B | 0x22 / 1 / 4 | 创建 432B，不上传 |
| 14090a2b7 | NaniFilteredInstanceAllocatorSsbo | u32 / 108 | TypedStorageBuffer 40B | 0x22 / 1 / 4 | 创建 432B，不上传 |
| 14090a435 | NaniDrawListSsbo | u32[5] / 0 | TypedStorageBuffer 40B | 0x122 / 1 / 4 | 缺少输入必须失败 |
| 14090a470 | NaniListDrawCmdOffsets | u32 / 0 | TypedStorageBuffer 40B | 0x22 / 1 / 4 | 缺少输入必须失败 |
| 14090a506 | NaniInstanceIndirectionSsbo | u32 / 0 | TypedStorageBuffer 40B | 0x22 / 1 / 4 | 缺少输入必须失败 |

固定三个计数器的原构造器分别是 `141612870`、`141613500`、`141612650`。`None` 分支只把数据长度置 0，然后把空指针和 0 长度传入真实创建函数；没有临时零数组。Some 输入先检查元素字节整除、指针 4 对齐、元素数不超固定容量。允许恰好容量。

OffsetDefaults 原 `add_new@1412bdff0` 分配 48B；`new_storage_buffer@1416207e0` 的 None 分支调用 `default_value@1408bb650`，复制真实 432B 记录，调用 `TypedStorageBuffer::new@1416154c0`，把实例数 1 写入存储 +0，缓冲写入 +8。因此该项不能退化成不带实例数的 40B 存储。

动态三项的原构造器是 `141613940`、`14160ffc0`、`1416132d0`；大小来自输入字节，不是固定一字节占位。20B draw 元素保存五个 u32，但本次没有证据命名每个 word，源码保留不猜字段名的实际布局。

## 初始数据来自实际参数计算

`14090a2f0` 调 `nani_calculate_draw_list_params@1408baed0`。返回体 0x70B：四个 Vec 在 +0/+0x18/+0x30/+0x48，汇总 u32 在 +0x60/+0x64/+0x68/+0x6c。原 `rbp-0x28` 是返回体基址：

- `14090a2f9` 读取 `rbp+0x3c`，即返回体 +0x64：71936。随后分配 `count*20` 并逐条写零，上传为 draw list。它不是 +0x60。
- `14090a462..470` 从返回体的第二个 Vec 取 ptr/len，上传为 `NaniListDrawCmdOffsets`。它是当前 `max_instance_offsets`，不是 `cmd_offsets`/OffsetDefaults。
- `14090a479` 读取 `rbp+0x38`，即返回体 +0x60：3788800。分配 `count*4` 并写零，上传为 instance indirection。

这两次零初始化是原版真实数据生产，不是用零值规避缺失资源。参数表 27×4 项保留当前已还原算法及原表。release 的两个 sorting flag 在 `.data` 虚拟尾部、超出 raw size，是 PE loader 清零；不能把 `get_offset_from_rva` 返回的下一节原字节误当这两个标志。section 边界证明在 stairs 任务的 `evidence/release-statics-section-proof.json`。

## 当前实现和真实消费者

新增 `populate_nani_ssbo_library(&AssetSsboLibrary, &dyn GraphicsDevice)` 作为明确的 7 项子集生产接口；复用现有 `AssetSsboLibrary`、`SsboType`、`TypedStorageBuffer`、`InstanceBuffer` 与现行 device API。没有修改或平行重建 rhapsody。

该接口不是整个 `init_populate_ssbo_library`。完整原函数有 72 次注册，除这 7 项外还有 65 项，并且原固定四项和动态三项之间插有 BushCpu、RippleSource、Sparkle 三次注册。完整启动恢复应依照全表分别注册，或拆分该子集调用；不能用这个聚合接口宣称全 72 项原顺序已实现。注册同一 NAME 两次仍由现库正常拒绝。

现 `draw_ext` 消费 canonical NaniDrawListLengthSsbo/NaniDrawListSsbo/NaniInstanceIndirectionSsbo；元素身份不复制。旧测试曾 `add_new<NaniDrawListSsbo>` 依赖错误 ZST 默认值，改为实际子集 producer。该测试改动另依法领取 `TG-7903aea7edaafd4f8e`；draw 算法未改。

仍未恢复的 NaniNext/PrevVisibilityBitmaskSsbo 与本启动无注册关系，本文件保留其显式旧占位，不把它们计为本轮已恢复。完整 generate-draw-lists 的所有实际 GPU 写面及整个 world render 还需要主任务整合验证。

## 验证边界

`rustfmt --edition 2021` 通过。按主任务统一构建要求，本 agent 没有运行 Cargo/GPU。候选与 source SHA 见 `applied-source-manifest.json`。

新增两个针对性测试：`native_nani_storage_layouts_replace_zero_sized_carriers`；`native_nani_startup_publishes_real_lengths_offsets_and_constructor_policies`。后者调用真实库注册与上述 producer，以 CPU 记录设备检查名称/初始数据长度/None 区别/用途/实际偏移/实例数，不声称 GPU 执行成功。已有 `draw_ext` 两条 indirect 分支测试改用同一生产接口，等待 root 统一运行。
