# asset-lib-render 域 HIGHLIGHTS（中央汇总用，≤10 条）

1. **26 章全部就地填充完毕**（asset_libraries 12 + render 14），613 业务全部给出 O/I/U 结论与证据链；**零新 MCP 反编译**——全部映射既有证据（A3_* 审计 + wp2/wp4 合同 + render-gap/audio-misc），仅 4 处 rne-read 字节复核。
2. **ssbo_library 334 业务 = 12 个模板族单态化**：`SsboId = xxh3_64(StorageElement::NAME)`（短名）；`Ssbo` 虚表四组实现（TSB 0x28 / InstanceBuffer 0x30 / Mapped 0x38 / Sparse 0x90）；`as_any`/`as_any_mut` 同址 ICF（38+21+19 对）；buffer/handle 为 1-2 字节桩（8 条目、3 唯一地址）。
3. **BufferState +0x36 = 5 语义修正**：wp4 记 "exported"，L0_NOTES 证实 = **rg None（未导入）**，`get_rg_buffer` None 时 re-import——以 L0_NOTES 为准（reconstruction 已按此落地，无代码差异）。
4. **render/ssbo 113 业务 = 6 族**（impl$0 ×38 / TSB::new ×47 / gpu_write_ranges ×4 / impl$4 ×21 / cpu_write ×2 / impl$7 ×1）；`BUFFER_LEN>0` 固定大小形态已由 `pass_kit::fixed_len_typed_storage` 接缝覆盖（render-gap §4.2）。
5. **fsr_plugin::build（0x14003bb80）= 1 字节 RET**，与 bevy `Plugin::finish<MainCameraPlugin>` ICF 合并——无独立业务；`frame_constants::should_reset_temporal` 字节复核为 `[+0x80]==0 → bool`（wp4 记"空体"需修正）。
6. **A3 审计与 wp 合同的全部纠错已登记到章**：mesh_library `add` 签名 `{asset, name: Option<String>}`、shader_library `force_new` 参数、texture bindless 上限 0x1fe0、color_library 37 槽名表、watch 三件套 100 ms 且回调写注册名、audio_trigger SlotMap 单 kind——reconstruction 已按 O 落地，无遗留代码差异。
7. **本域无新增 U 项**（全为既有合同的 typed gaps）：slot_meta 角色、Vao f6–f8、Mesh 哨兵 +0x18、DerivedAtmosphere 槽语义、CompiledTexture 头字段、PlacementDataSsbo JSON schema。
8. **跨域接线证据齐备**：全局 HANDLE_COUNT 0x143634cd0 五库共用；SsboId 标签 xxh3 复算（"Boats"/"Garden Stone"/"ColorLibrary" 等）；meshwatch 后端系统参数面经 bevy 包装泛型名证实（含 ShaderWatch/GraphicsDeviceResource）。
9. **reconstruction 26 个源文件均无 todo!/unimplemented!**（grep 证实）；本章只登记证据，未改源码（任务约束）。
10. **最大 U 项**：`TypedStorageBuffer::new_arbitrary` 与 `SparseInstanceBuffer::slot_meta`（0x40 表，u32/槽）——无任何已反编译消费者，语义完全未知，建议留给 nani/cull 域 consumer 驱动恢复。
