# geometry 重构后复审 · 2026-10-18

审查当前 geometry、稀疏缓冲、SSBO 资产库、宿主缓冲表以及直接绘制入口。生产代码未修改。

## R1 · P1：稀疏存储销毁时没有释放宿主 GPU 缓冲

位置：[runtime.rs](../../crates/country-core/src/geometry/runtime.rs#L63) 以及 [SparseInstanceBuffer::try_new](../../crates/country-core/src/render/sparse_instance_buffer.rs#L172)。

新运行期通过真实设备创建 192 MiB 砖缓冲并注册进 AssetSsboLibrary，但保存的 TypedStorageBuffer 只是 Copy 的数值句柄。SparseInstanceBuffer、BufferState 和 AssetSsboLibrary 均没有调用设备 release_buffer 的清理路径，运行期也没有卸载该缓冲的钩子。

[BufferHost](../../crates/tiny-glade/src/render_host.rs#L62) 以槽位表持有 ManagedBuffer。丢弃 CPU 存储或 World 不会清空仍存活宿主的槽，必须显式 release；原 GPU 回归末尾恰好手动调用 host.release，因而没有覆盖拥有者销毁。删除墙只归还 chunk，与释放这 192 MiB 物理缓冲不同。

触发条件是**同一 BufferHost 仍存活时销毁/移除持有砖存储的资产库，再重建资产库并创建砖**；每轮都新增缓冲，旧槽及其物理内存保留。整个宿主一并销毁时最终会释放，不应误述为进程退出后仍泄漏。

建议让该存储或资产库持有来自创建设备的拥有租约，在拥有者销毁时复用现有 release_buffer 接口归还宿主槽；GPU 在途寿命继续交给 ManagedBuffer 和现有延迟回收机制。补充不手动 release 的库/World 销毁测试，以及同宿主多次重建测试。

## 其余结论

原 G1 的普通实例视图问题已通过独立 lookup_buffer 修复；CPU/GPU 砖格式也已统一。待提交状态、容量预检、整批替换和失败后保留 dirty 的实现未发现新的确认缺陷。未将完整 draw-list/culling 和最终场景接线作为已完成内容；这些仍是上轮已列明的边界。

复审开始时 geometry 源码指纹与上轮报告一致；其他模块存在后续修改，详见 [baseline.json](baseline.json)。不能把上轮整个 workspace 的历史通过结果说成本轮重新通过。

## 验证

只读设备查询成功，Apple M3 的 maxStorageBufferRange 为 4,294,967,295 字节，大于 201,326,592 字节的砖缓冲；本机未发现描述符范围不足的问题。查询属性原行见 [device-info.log](device-info.log)（已压缩，仅保留相关属性）。

[最小 GPU 诊断](gpu_probe.rs) 在创建生产稀疏存储后将其 drop，检查真实宿主槽是否仍存活；诊断最后主动释放资源。这个断言复现缺陷，不是回归修复验收。首次编译被保存模块的系统配置 E0277 阻塞，见 [gpu-probe.log](gpu-probe.log)；该模块更新后已重试，但编译因磁盘空间耗尽（写 full.rmeta 时 os error 28）失败，见 [gpu-probe-retry.log](gpu-probe-retry.log)。两次均退出 101，诊断未执行，不计通过；本轮未重跑完整测试或升级 GPU 验收。临时测试入口已移除，仅保留复现源码。结论 R1 为生产源码拥有权审查确认，运行复现仍待可用构建环境。最终状态见 [verification.json](verification.json)。
