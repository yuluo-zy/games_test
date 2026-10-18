# RT 已合源码独立审核

2026-10-18，os_chain；源码只读，未运行Cargo。核对resources/rt、bind_resources、draw_builder、draw_stream、compute_pass_builder及tiny-glade/frame_setup、raster_host。rhapsody内部不在本次修改范围。

## 发现：BVH元素alignment丢失

`resources/rt.rs::RtBvhRecord` 当前仅 `#[repr(C)]`，唯一字段 `[u8;0x50]`，因此Rust类型alignment=1；透明包裹的RtBvhSsbo同为1。原upload末尾 `1408bda5f..a70` 释放BVH Vec，size=cap*80，传入dealloc的R8d明确为4；该分支数据来自同一pending BVH Vec，足以确认原元素alignment=4，不只借GPU BufferDesc推断。

建议core给RtBvhRecord使用 `#[repr(C, align(4))]`，并在现layout test加入align_of断言。80B已整除4，不增加padding。当前byte adapter本来检查ptr%4，alignment1的类型允许合法非4对齐slice，跟原typed元素合同不一致。未自行改源码，已即时反馈root。

## 核对成立的数据流

- RtWorld.pending_upload.take()严格区分None和Some(empty)。None不查询库、不改计数；Some(empty)经过三个typed owner，把count依次写0而不update GPU。
- 整批边界是原严格小于。超界分支只日志/stderr并丢弃pending，发生在任何storage lookup之前；不会被buffer unwrap误拦。constructor单独允许容量相等，并通过现fixed byte-storage API传None创建未初始化GPUbuffer，无主机清零数组。
- tri→meta→BVH每个函数内update成功后立即写count，再进入下个lookup；后续失败不回滚。计数发布时点符合原非事务分支。现测试尚未专门模拟“第二stage后端失败后第一count已更新”，这是验证边界，不是已发现业务错误。
- canonical类型只由resources/rt生产，bind_resources重导出；旧RtTriangleSsbo为同一类型alias，无第二same-NAME carrier；WallColorIdSsbo复用真实owner。
- compute直接读取InstanceCounted，draw通过专用ssbo_instance_count，frame_setup以实际typed owner快照其低32位，raster_host只读该表。新路径不从Buffer.id、generation或rg资源号取计数。
- compute regression明确将实际InstanceBuffer计数设置17/9，并对比各buffer.id不同，然后检查反射uniform offset24/28的u32。它能捕获把count误接回id的回归；draw现test另外使用独立受控count值检查反射uniform。均是CPU验证，尚不能宣称真实GPU执行通过。
- frame_setup每帧先清count表，再仅对存在的实际RT typed owner发布值；缺失依旧None，draw调用保留原unwrap失败，未制造zero-count合法替身。

## 仍独立未闭合的范围

本次source修改没有使BVH异步worker/场景FramePasses或实际producer调度自动完成。原typed payload的内部字段也不能只凭24/4/80尺寸宣称已完全逆向。真实GPU字节/绑定验证由root统一执行。
