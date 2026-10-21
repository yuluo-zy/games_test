# StairsTopDownMesh 真实 buffer owner 必要闭包合同

状态：只读合同，待独立准入；未写候选/生产。源 stairs_top_down.rs 是现有纯类型文件，tasks/files/path/payload 与B skeleton均无单独任务，不造task/claim。依lead明确授权附TG-ad966现合法work。W7 TG-b4f30a32cb343cc349 live owner boot-terrain-20261018保留，仅必要caller一行待root窄委派；两个现代测试fixture只作ROOT hunk。

## 原真实值与业务
原main true140008c90/62374B的140010c35..e63，false140018040/62822B的1400200f5..323已复用Duck完整原pack。main把visual全部edge.floor_vertices Vec4 flatten，非空byte_size=count*16，BufferDesc::new_gpu_only(size,0x20)，实际Device.create_buffer("stairs top down",原15字节名称1429264f0，完整初始化bytes)。create成功0xb才把40B raw::Buffer move进资源+0，并写count u32到+0x28，Commands.insert_resource14014dd30；命令保存48B资源，并非floor的8B Arc指针。资源原值size48/align8。count取低32位，禁止新增几何/count默认值。

空flatten或原其余map为空分支Commands.remove_resource；create Err只drop错误并保旧资源，未见此段Error日志。原主body门/flatten仍由Duck唯一恢复，不在本叶新造publisher或者把每帧无条件create当原行为。

原ComponentDescriptor::drop_ptr<TopDown>14014b700/5B直跳140be24b0/340B managed::handle::drop：原Buffer内部managed strong ref递减，最后owner才把资源身份加入同回收队列，清内部ptr；不是把u64 ID丢弃即结束，也不是floorArc Drop。依赖算法复用现rhapsody，不在core复刻managed队列。

原W7 1409cb7b0/2743B：Option资源有则rsi=资源首地址，1409cbea3直接把rsi传bind_external_storage_buffer，cf47从rsi+28读count→draw_no_vao。None/dirty bool两分支与shader/两纹理绑定都已有恢复，本轮只迁移真实Buffer所有权读取。

## 当前确认缺口与现代同owner适配
当前ManagedHandle **Clone非Copy**，仅u64无Drop/lease；TopDown资源只有该字段+count。实际BufferHost.slots持ManagedBuffer，get_by_id克隆并临时取raw，release才清slot并增generation。全实际callers审查无TopDown移除时release入口，因此替换/卸载后旧slot仍活直到host整体析构。不能因后端表本身强持便宣称已具原final-drop回收。

复用已审visual262f的唯一StairsMeshBuffer（Buffer40B+Arc<dynGraphicsDevice>，Drop调用创建者release_buffer一次）。拟现有TopDown结构：Resource+Clone+Debug，pub buffer:Arc<StairsMeshBuffer>，pub vertex_count:u32；new(buffer:Arc<StairsMeshBuffer>,vertex_count:u32)。现代size16/align8，是安全所有权适配而非硬抄原48B ABI；同BufferHost无新表/registry、无第二allocation，Arc只包本已创建的一块Buffer。

新方法managed_handle()->ManagedHandle仅把同owner.buffer().id投射到已存在draw API，不另拥有后端租约。W7那一行改&stairs.managed_handle()；本资源Arc在整个pass准备期强持实际id/generation的完整Buffer，失败也不能默认handle0。count与bytes来自真实producer，不在constructor补几何或做无原证的成功校验。只从实际成功buffer构造，不实现Default/Copy；世界仍可没有此资源。

## 实际producer/consumer及在途生命周期
- Duck两个主body未来成功branch创建一次Arc<StairsMeshBuffer>并move到TopDown；现生产两main仍未恢复，不计本包已接producer。失败不构造/替换；空则deferred remove。
- W7 core迁移typed owner读取，bind_external_storage_buffer现先host.get_vk_buffer拿raw并保logical ManagedHandle；不改其通用接口/registry。
- 真实tiny-glade/world_terrain.rs现.get_resource::<TopDown>().cloned()，保持该Arc跨scene_raster::with_draw_context；不需改该root-owned文件。
- scene_raster调用frame.with_raster，RasterPass.render内同步run raster_host准备。在raster_host:1901 import_draw_bindings取同shared.buffers.get_by_id，验证raw、size，然后owner.shared()经declarer.import_buffer进入VulkanGraphSession::import_frame_buffer。本次resource临时clone直到所有import结束才drop，因此无回收slot/复用代数窗口。
- graph frame_import拥有实际rhapsody Buffer Arc，recording另retain_recorded资源并在提交成功注册SyncPoint，图取消/retirement清lease；BufferHost.release只移slot，实际GPU物理延迟销毁仍由rhapsody同步机制。该路径已有实现，本包不扩render/device。静态链闭合不代表本机GPU已验。
- 同unload lifecycle.rs:98的Commands.remove_resource<TopDown> API不变，queue.apply才drop世界资源；若root frame持clone，最后clone drop才release，若graph持physicalArc则按提交完成回收。本包不并写peer生命周期source。

所有actualcallers：core W7、core d_main_tests中的唯一literal、system-decorator/tests/decorator_unload_lifecycle唯一literal、root world_terrain clone、root/peer lifecycle remove。当前不存在其它Default/Copy/size_of调用。两个现代fixture要用同真实CPU recording GraphicsDevice.create_buffer输入显式Vec4 bytes构造owned lease，原count与draw/assert/卸载顺序不改。不得私加生产None shell。

## 最窄source/验收
只现有stairs_top_down.rs schema+new/handle+两个真实World/Commands所有权测试；W7现mapped源单行typed caller；两个现代测试fixture必要ROOT hunk，source/claim保持root唯一生产。复用visual262f after262f5440依赖，不能从未整合生产旧visual刷新before覆盖。

CPU测试必须实际RecordDevice.create_buffer+现StairsMeshBuffer，并World.insert/replace/deferred remove，核旧owner只有最后clone释放一次、None不默认、sameArc/count/原完整Buffer generation保留、创建失败沿actualproducer保旧的条件不得由本constructor伪造。W7既有draw+clear路径fixture改typed lease后保全部assert，unload既有延迟移除断言保留；不为本叶新造Arc图或替GPU单元成功。root须跑core与decorator相关真实tests；native Drop/typed schema属合同与依赖复用不计新业务函数数。
