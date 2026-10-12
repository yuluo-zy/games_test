# Shader runtime scalar 的真实设备来源

当前 `src/raw/shader.rs` 的 `ShaderLibrary::create_compute_pipeline` 与 `prepare_raster_program` 仍由调用方传 `runtime_device_scalar`，GPU 示例传常数4096。pure `ShaderInterfacePlan::from_reflection` 的显式参数适合保留CPU测试；生产入口不能以示例常数替代物理设备属性。

root 可在 `VulkanDevice` 添加以下查询（本 agent 不编辑该共享文件）：

```rust
pub fn max_descriptor_set_update_after_bind_sampled_images(&self) -> u32 {
    let mut indexing = vk::PhysicalDeviceDescriptorIndexingProperties::default();
    let mut properties = vk::PhysicalDeviceProperties2::default().push_next(&mut indexing);
    unsafe {
        self.instance.instance.get_physical_device_properties2(
            self.physical, &mut properties,
        );
    }
    indexing.max_descriptor_set_update_after_bind_sampled_images
}
```

当前同工程 PhysicalDevice::driver_version 已使用同一 properties2 查询链；这是 ash 原版 API，不是新增自造属性布局。原构建API1.2下descriptor-indexing属性已核心化。query返回的原始属性是 **P**，交给 `runtime_descriptor_count(P)` 后再减32/cap8192；不能把已减/已clamp的值重新传入。

生产 ShaderLibrary 两条路径内部从 `self.cache.device` 查P，传给 from_reflection，并用实际P形成现有layout cachekey。移除生产的外供scalar参数或保留单独明确的测试入口；不得静默把生产API参数留成无效常数。descriptor indexing特性是否启用仍由既有DeviceCapabilities验证，属性查询不能替代feature启用。

原字段来源的证据等级依 SYNTHESIS11.5/R4：rhapsody自有 panic字符串坐实 limit名称，`PhysicalDevice wrapper+0x680`精确成员偏移仍属静态推断；上面真实Vulkan查询不声称候选VulkanDevice具有同样原ABI，也不证明任一游戏pass动态绑定。
