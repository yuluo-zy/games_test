# RT BVH 原证与实现计划独立复核

Reviewer：`/root/os_chain`，2026-10-18。范围为 parent 拟恢复的 `build_gpu_bvh` / worker / glue / 原 obvhs 依赖。只读审查，无 Cargo、无生产源码修改，不接管 parent 的 rt 文件族。结论：复用精确 obvhs fork、按原输入/输出建立游戏适配层的方向成立；现有源码的 4B GpuRtTriangle、pending==0 idle 和 STATE_SHUTDOWN=2 解释不成立，必须随闭包一起替换。

## 身份与依赖

独立读取原 B RNE，SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。parent 两份 manifest 的 8 个完整 native 范围哈希重新计算相等；另捕获 is_idle、consume wrapper、consume fold 三个完整 PDB 范围。逐项证据见 `independent-byte-review.json`，不依赖已经出现 BufferMgr 错误的共享 Ghidra。

原 Cargo.lock 的 `obvhs 0.1.0` git rev 为 `a3f3b3bfad18f3dce0b0505a927200b3575477d0`。本机原 commit archive SHA 为 `ef2728abfd7e26be4486747169ddf73751509bbcd80f4d668808eadd13863716`，tar PAX comment 精确等于此 commit。24 个 vendor 文件逐字节匹配 archive，并全部匹配 provenance SHA。该步骤只证明依赖身份与文件完整性，不是游戏 BVH runtime 对照。

## 必须保留的游戏输入和输出

原 worker 的 InputTriangle 是 40B：三个变换后 XYZ，共 36B，加 mesh color/metadata u32@+0x24。`build_gpu_bvh@140b0da50` 转换到库 Triangle 时分配 48B/triangle、16 对齐，三个 Vec3A 每个占 16B。

原 worker flatten 的是整张请求 mesh snapshot 中每个 positions 的 `chunks_exact(3)`；不足三个顶点的尾部不产生 primitive。变换表达式为 `((col0*x + col1*y) + col2*z) + translation`，不除 W，不做投影变换，不可按旧注释的 row-major 解释直接转置。

`filter_map` 的 finite closure `141233e50` 检查全部变换后 9 个 f32：逐个取 `bits & 0x7fffffff`，若任意 `>=0x7f800000` 则 None。它接受有限 subnormal/±0，拒绝 NaN/±Inf，也拒绝矩阵乘法溢出后生成的 Inf。该闭包没有面积、共线、重复点、法线或半精度范围筛选；不可顺手添加退化三角形剔除。

## CwBvh primitive reorder 不能省略

原 `build_gpu_bvh@140b0dd82` 读取库返回 `primitive_indices`。`1414767d0@6844` 逐个读取其中的 u32，按 index×48 从转换后的 Triangle 表读取三点，生成 24B GPU 三角形。另 `140b0ddf0..de05` 对同一索引按 index×40 取原 InputTriangle+0x24，生成 metadata u32。两数组顺序和长度必须严格一致。

vendor `cwbvh/bvh2_to_cwbvh.rs::count_primitives` 递归按叶序 append 原 primitive index；构造时还会 order children。因此该向量不是 0..N 的恒等映射。`splits.rs` 在空间 split 成功时向 indices 追加同一个原 index，扩增 AABB，不生成新的原 Triangle。由此返回的 primitive_indices 可包含重复项，输出三角形/metadata 长度可大于输入长度；不能 zip 原输入、去重、按源 mesh 顺序上传或假定长度 N。

## 24B 打包和边序

原 GPU triangle 是 `[v0.x, v0.y, v0.z]` 三个 f32，再三个 packed u32，大小 24B、对齐 4。`1414767d0`：

```text
e1 = v1 - v0
e2 = v2 - v0
packed_x = half(e2.x) | (half(e1.x) << 16)
packed_y = half(e2.y) | (half(e1.y) << 16)
packed_z = half(e2.z) | (half(e1.z) << 16)
```

原 `half::f16::from_f32@1408b9f20` 是已复用依赖转换。native @68ca/68d2/68db 分别左移 e1，再 OR 零扩展 e2；@68ed..6903 按 24B stride 写输出。vendor `RtCompressedTriangle` 的字段顺序恰相反，是低 e1、高 e2，不能直接 bytemuck 上传该类型。应在游戏侧显式打包或交换每对 half，而不是修改 vendor/rhapsody。

## build 参数逐项

原 config 基址 rbp+0x50；native @140b0dc30..59 完整写入：

| 相对偏移 | 原值 | 当前语义字段 |
|---|---|---|
| +0 | usize 1 | search_depth_threshold |
| +8 | f32 0.2，字节 cdcc4c3e | reinsertion_batch_ratio |
| +12 | f32 1 | post_collapse_reinsertion_batch_ratio_multiplier |
| +16 | u32 3 | max_prims_per_leaf |
| +20 | f32 3 | collapse_traversal_cost |
| +24 | u8 4 | PlocSearchDistance::High |
| +25 | u8 0 | SortPrecision::U128 |
| +26 | u8 1 | pre_split=true |

调用的是 `obvhs::cwbvh::builder::build_cwbvh_from_tris@1417c93d0`。复用库内部真实顺序：AABB→可选 pre-split→PLOC→reinsertion→BVH2 转 CWBVH，children reorder=true、exact node AABB=false。post-collapse multiplier 和 collapse traversal cost 是库参数合同的一部分，但这个 CwBvh helper 不使用它们，不能自己额外补一次 BVH2 collapse/reinsertion 冒充原复杂性。

原 `build_gpu_bvh` 对空输入在 @140b0daf7..db29 提前返回三个合法空 Vec，未调用库构建。必须保留；不能指望库对空输入自行处理，也不能返回 None。其结果应作为 Some(empty) 交付，删除最后 mesh 后才能清 GPU 三个实例计数。

## 请求、脏标记与 idle

glue +0xa0 是 Option<bool> niche：2=None、0=Some(false)、1=Some(true)。它不是 shutdown 枚举。worker @140b24dd0/8 取旧请求并写 2；旧值为 2 时解锁、sleep(0s,1000000ns) 后继续。两个 Some 状态都取完整 map snapshot 并全量构建，本体没有按 true/false 分别选择增量算法。host 如需可停止线程，应另设明确宿主退出机制，不能把 2 改为终止含义。

update_rt_world 的 world +0x38 结构脏位在移除事件路径置 1；@1408bcb8e 读取、@bcb92 清 0，并在 @bcc38 作为本次 request bool。即使 world 结构脏为 false，只要任一 DirtyCow 脏也会走同一请求路径，发 Some(false)。因此 bool 不代表是否存在请求。

**额外发现的必要消费步骤**：`update@1408bcbd3` 调 `from_iter@141449790`，后者调用 `RawIterRange::fold_impl@140b760d0`。完整原 fold 227B 本次另存；@140b76160 读取原 entry 的 dirty、@76164 将原 entry dirty 清 0、@76168/6c 取 Arc 并加引用、@76172 把旧 dirty 放入 snapshot entry。这是清原权威 map、保留 snapshot 旧状态；仅 clone HashMap 而不清原 dirty 会每帧反复请求构建。

worker 取请求后 @140b24e7c busy=true，在锁外收集/过滤/构建；@140b25052..9a 以新三 Vec 输出覆盖旧结果并释放旧数据；@250f5 busy=false。待构建请求和已完成未领取输出都是单槽可替换状态，不是 FIFO pending count。

`is_idle@1408bc530` 原条件精确为：request tag==2，output 首 Vec capacity==i64::MIN（Option None niche），且 !busy。`neg rbx; seto` 专门检测 i64::MIN；输出 Some(empty) 的 cap0 不等于 None。因此已构建但未领取结果时必须返回 false。只检查 pending==0、只检查 !busy 或把空 Vec 当作没有结果均错误。

## 审查边界

以上支持 parent 的原依赖复用与真实 producer 修复计划，并新增 DirtyCow consume 必清位、空结果不可吞掉、两态全量构建三项具体要求。尚未审查 parent 后续生产 patch，因为本次查的是证据/计划；当前旧实现的注释不能继续作为事实。无 Cargo、无 native runtime oracle、无 GPU 结论。后续以实际 worker 生命周期→reordered 24B triangle/4B metadata/80B node→既有 InstanceBuffer 上传链作有界验证，保留现行 rhapsody 架构。
