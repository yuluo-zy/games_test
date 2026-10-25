# Nani 顶层真实业务合同（B）

## 取证范围与身份

发行原 RNE SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`，配套 B PDB SHA256 `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。主函数 `country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists`，PDB module 521 / record 57552，VA `140987dd0`，完整范围 11539 B（含独立 EH funclet）。完整原指令、21 个实际下游实例和原跳表已保存，没有访问损坏的共享 Ghidra 数据库。旧反编译只用来导航；未采用旧报告的“全部 O”判断。

文件族同时有 C，但 C source checksum 和 code 均不同。本次恢复是 B 消费者在当前 Rust API 上的接线，不声明 C 等价。

## 原主算法

1. World 必须存在 GpuWorld。`140987e80..e86` 在第一个图分配之前写其真实 changed tick；当前 `scene_compute::generate_nani_draw_lists` 对实际 Bevy Mut 执行同一时点。
2. 第三参数是 `Option<HZB tuple>`，不是任意 round 整数。无 HZB 时处理 `0..4`，有 HZB 时处理 `0..1`（`140988304..3ed`）。首轮清 NextVisibility，补绘轮清 PrevVisibility；cull 补绘轮交换读写角色。
3. 创建 sorting histogram `0x6c000 B` 与 filtered meta `params.total_cmd_offset * 4 B`。清 draw-list length/indirection allocator/filtered allocator，然后清 histogram，再清本轮目标 visibility。
4. 非细粒度子列表先写每 atlas part 的空 draw，8 B push `{kind, command_offset}`，每条 dispatch 64 线程。不是所有 shader 共用 20 B PushConstants。
5. 通用 cull 收集 19 类 owned 子集；缺 owned 是原 panic，count=0 不 dispatch。取 `ShadowedBuffer<GpuInstance>` 的真实 BDA，20 B push `{address_lo,address_hi,kind,count,kind*4}`，线程数为 count。四类 custom（Plaster/Smoke/CurvedHalfTimber/StairsFloor）绑定其已导入实例 data，payload 后两项次序变为 `{kind*4,count}`。
6. 四类 external：RoofTile/WoodenPlank 从实际 SparseInstanceBuffer active ranges 逐段 dispatch，16 B push `{kind,first,count,kind*4}`。Flower/Bush 先读取真实 DrawIndexedIndirectBuffer 的槽 11/10，用 8 B push `{draw_index,64}`生成一条 16 B dispatch args，再用该 GPU 参数间接 dispatch；其 cull push 为 8 B `{kind,kind*4}`。旧叶代码误写为 WoodenPlank/RoofTile 并把 64 误当 4，已纠正。
7. histogram prefix 后创建 `Nani dispatch args` 0x1b00 B 和 `Nani dispatch thread count` 0x1b0 B。原 108 线程 producer 按每子列表产生四个 uint4 参数平面，每平面 `108 * 16 = 0x6c0 B`。不是只分配当前 active 子集的参数。
8. fill unsorted / sorted instance list 按原 RenderSublistsIter 过滤 fine 状态；非 fine 用 12 B `{list_index,instance_offset,command_offset}`，fine 用 8 B 前两项，均 plane 0。filtered 绑定原来实际是 NaniInstanceIndirectionSsbo 的临时用途；没有名为 NaniFilteredInstanceSsbo 的另一库类型。
9. 再次清 draw counters，结束同一 indirection buffer 的临时用途。创建 draw-count histogram `align_up(max(total_fine_grained,1),64)*4` 和 subtotals `max(total_batches,1)*4`，各按原 shader 清理。创建 unsorted-atlas-prefix 0x6c00 B，非 fine 子列表以 8 B `{list_index,command_offset}`、64 线程写 prefix。
10. 原四阶段不能省略：allocate_draw_lists（fine，8 B `{list_index,histogram_offset}`，plane1）；group histogram prefix（fine，20 B `{hist_start,hist_end,subtotal_start,subtotal_end,list_index}`，plane2）；subtotal prefix（fine，8 B `{subtotal_start,subtotal_end}`，plane3）；generate draws（fine，20 B `{kind,list_index,command_offset,histogram_offset,subtotal_offset}`，plane1）。最后非 fine 的 fill-unsorted-instance-indirection 用 4 B list_index、plane0。

原 type/函数名的 `RenderSublistsIter::next` 在这里承载 fine 分类、四 lane 和四组偏移，不是可丢弃的普通迭代胶水。当前实现复用 GpuWorld 的唯一 params，没有另造分配器。

## 现架构上的缺陷修复

- 现 `DispatchMode::IndirectWith` 错给普通 ComputeDispatchApi；该接口没有间接命令，旧多个叶闭包为空。原 PDB 实例 `14160bdf0` / callback `1415fbc50` 明确给 ComputeDispatchIndirectApi。现在模式4绑定声明过的同一个图 buffer，经 ImmediateIndirectDispatch 消费 byte offset，复用现 ComputeExec 和当前 rhapsody managed graph。
- OwnedGpuWorldSubset 的 positions 原先只有 Vec + dirty，未有可消费的 GPU cull buffer。原 specialized `TypedStorageBuffer<GpuInstance>@141616f30` 明确 size=count*16、usage0x20022、memory1、alignment4、name GpuInstance，初值来自真实影子数据；`ShadowedBuffer::new@141691810` 与 flush 实例共同证实 owner。现在补实际 ShadowedBuffer 并复用当前 dirty/upload 算法，不改变普通 TypedStorageBuffer::new 的 usage。
- 原 shader 的 BDA 来自同 BufferHost 的真实 Buffer owner；ManagedHandle 只作为当前源宿主 token，不当 Vulkan 句柄或裸地址。owned data 仍由既有 main prelude 按原先 import/export 生命周期管理。

## 原 producer 与尚未宣称闭合的边界

- visibility 两库不是 init_populate72 中的零大小条目。原 `startup_nani@140a38610` 的真实 calls `140a395ab -> add_new_with_data<Prev>@1412bb570`、`140a395c0 -> Next@1412badf0` 已独立扫描原 .text 并保存。完整 startup_nani atlas/subset/visibility 安装仍需接回；缺失保持失败。
- OS 同行 fresh 发现 WoodenPlank 当前 32 B / 0x800 chunk 候选错：原构造是 64 B / 0x1000 chunk。其 payload/CPU 生产者正独立恢复；本 master 不以空 range 或假 None 绕过实际资源。
- Nani cull 和 generate draws 有真实原子读改写；当前 legacy access code 37 正确不再凭同步 mask 推测内容必读。需要实际 GPU 图核实先 clear 的 producer 是否保留，再按确证 shader 内容意图传 preserve；不能通过全局禁用检查或额外假 clear 解决。
- 完整 scene FramePasses 尚未安装。production compile_world_graph 仍明确报缺，不运行这部分就发布 gameplay-ready。本模块新增入口是可调用实际业务闭包，不能据文件/方法数量宣称整游戏恢复。

## 验证边界

新增 CPU 合同回归覆盖：同声明 buffer 的逐项间接 offset / barrier / push、真实0x20022 cull-buffer dirty上传、108 子列表与 fine 两类及四平面字节偏移。源码 rustfmt 已通过；Cargo 与真实 GPU 由根代理统一执行，本文不预填 PASS。
