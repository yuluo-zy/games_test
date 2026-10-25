# 纹理批量区域上传适配

实现 `raw::texture_upload::record_texture_regions`：只向既有录制中的 command 写入一次 transfer-dst transition、一次多区域 copy、一次 shader-read transition；不 submit/wait。command 保活 texture/staging。属于重建适配，不声称恢复原作者该接口。

参数检查：同设备/正在录制、TRANSFER用途、SAMPLED、RGBA8 UNORM/SRGB、2D单采样单mip单层、COLOR子资源、正extent/非负xy/固定z和depth、纹理边界、4byte bufferOffset、rowLength/imageHeight最小值、Vulkan行字节限制、checked暂存最后一个实际读取texel边界。单深度单层时imageHeight不影响读取跨度，但仍检查其最小值；末行padding不要求占用buffer。

unsafe调用者合同：不在renderpass内，真实旧布局正确，队列族拥有所有资源且支持图形录制，无并行GPU/宿主写入竞争；UNDEFINED丢弃旧图像，未初始化区域不能采样。资源保活不能代替提交同步。两次transition均QUEUE_FAMILY_IGNORED。

限定 tests/texture_regions.rs：3/3通过，包含纯参数边界与实际GPU整图初始化→两区域padded-row更新→TRANSFER_SRC正确布局回拷→shader-read恢复→HOST屏障→字节比较；没有skip。实测 Intel UHD Graphics 630，vendor32902/device16024/driver1656925/queue_family0。暂存Arc在submit前释放，command保活保证完成前存活。真实日志test.log。

原setup.rs和旧非法shader-read布局copy测试没有修改。未调用任何原游戏或原binary oracle，也未修改依赖锁。根负责tg-text接线与最终统一门禁。

复制行/高度语义依据[Khronos VkBufferImageCopy](https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferImageCopy.html)与[Copy Commands](https://docs.vulkan.org/spec/latest/chapters/copies.html)。
