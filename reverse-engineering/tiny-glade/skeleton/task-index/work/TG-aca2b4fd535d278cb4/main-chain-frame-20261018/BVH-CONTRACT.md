# B RT BVH 构建和异步所有权合同

仅 B 范围；C 文件族领取不构成 C 验证。原完整 RNE/PDB 范围、指纹见 rt-bvh-rne-manifest.json 与 rt-bvh-closure-manifest.json；RNE SHA-256 为 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03。fresh patched B 反编译保存在相邻 rt/mod claim；完整函数分别是 build_gpu_bvh 1559 B、worker 2397 B、update 2455 B，不能按 Ghidra 首段认定完整。

## 已观察事实

- 原锁依赖 obvhs 0.1.0，git h3r2tic/obvhs 提交 a3f3b3bfad18f3dce0b0505a927200b3575477d0。GitHub 官方 commit tar 的 pax comment 与提交一致；库源码、Cargo.toml 和许可证已原样保存在 third_party/obvhs-a3f3b3bf，完整文件指纹见旁边 provenance.json。它是依赖复用，不是恢复游戏算法数量。
- build 参数：PlocSearchDistance::High（4）、SortPrecision::U128（0）、pre_split=true、search_depth_threshold=1、reinsertion_batch_ratio=0.2、post_collapse_reinsertion_batch_ratio_multiplier=1、max_prims_per_leaf=3、collapse_traversal_cost=3。原 0x140b0dc21..7b 及常量 0x142ae78c0 支持。
- 输入记录 40 B：三个 XYZ 顶点与 u32 metadata；空输入直接返回三个合法空 Vec。
- 按 CwBvh.primitive_indices 同时重排 GPU 三角与原输入 metadata。pre-split 可以重复原 primitive，输出数不要求等于输入数。
- GPU 三角是 24 B：v0 三个 f32，随后三个 u32，每个低 16 位为 f16(v2-v0)，高 16 位为 f16(v1-v0)。1414767d0 明确此顺序；不能直接使用 fork 的 RtCompressedTriangle（它低位为 e1）。
- BVH 节点直接复用 fork 的 repr(C)/Pod CwBvhNode，80 B、align4，不重新发明节点编码。
- worker 按网格 chunks_exact(3)，应用列式仿射 col0*x+col1*y+col2*z+translation，无透视除法；仅过滤变换后九个不有限 f32，没有面积或退化过滤。
- request tag 是 Option<bool>：2=None，0/1 均是有效请求，两者均重建完整快照。每次取走 request 后 busy=true，解锁做算法，写回 Some(result) 后 busy=false。无请求睡 1ms。
- is_idle@1408bc530 同时检查 requestNone、outputNone、!busy；Some(empty) 仍是待领取结果。此前 pending==0 与 STATE_SHUTDOWN 名称均错误。
- update 对外部 UpdateRtWorldEvent、删除或任一 DirtyCow 变化发请求；删除事件先移除 tag0 的墙句柄。消费快照共享真实 Arc<RtMesh> 并清 dirty，不把容器重建误认为无操作。

## 当前工程适配

保留现有 RtWorld/AssetSsboLibrary/InstanceBuffer 和 rhapsody 架构。旧 4 B GpuRtTriangle、opaque 节点与简化空闲判定在原文件内修正。纯 BVH 和 worker 放回该现有模块的子文件；请求、结果使用拥有型 Option 与 Arc/RwLock，避免复制编译器私有布局。宿主线程的显式停止/join 仅为资源清理适配，不声称原程序有同一 shutdown 协议。上传仍使用已恢复的唯一分段上传业务体。

## 验证与未决

独立 reviewer 正在核验上述原事实，报告另存。后续必须编译并运行压缩字序、变换/有限过滤、异步删除清空、节点与 metadata 对应关系的实际测试。尚未安装完整游戏调度、真实墙/屋顶 RT 几何生产者或完成原进程 oracle；不能按库构建成功宣布整个光追世界恢复。
