# 墙砖缓冲生命周期修复

2026-10-18，关闭复审 R1：销毁墙砖资产库后，仍存活的 BufferHost 不再保留其 192 MiB 缓冲槽。

## 修复

- SparseInstanceBuffer 新增 `try_new_owned`，用释放守卫持有创建设备 Arc 和原始代际句柄。存储销毁时恰好归还一次设备分配，CPU 初始化 unwind 也由守卫清理。
- 墙砖生产调度改用拥有设备的构造，不依赖 ECS runtime 与资产库的析构顺序。宿主已有的 ManagedBuffer 引用和延迟销毁机制继续保护在途资源，没有额外设备等待。
- CPU 测试检查库先销毁、runtime 先移除、重复创建、World 销毁与晚于库销毁的分配回收。GPU 测试删除手动 release，检查真实宿主槽释放、原槽复用、代际变化、旧句柄失效与新缓冲回读。

## 本轮验证

| 检查 | 结果 | 日志 |
|---|---|---|
| 几何模块 | 37 通过，0 失败/忽略 | [cpu-geometry.log](cpu-geometry.log) |
| country-core 完整单测二进制 | 2102 通过，0 失败/忽略 | [core-lib.log](core-lib.log) |
| 严格真实 Vulkan GPU 测试 | 1 通过，0 失败/忽略 | [gpu-geometry.log](gpu-geometry.log) |
| 修改的 Rust 源码格式 | 通过 | [fmt.log](fmt.log) |

全库测试运行上述 cargo 命令刚构建的二进制，二进制指纹、命令和源码指纹见 [verification.json](verification.json)。GPU 使用 `RHAPSODY_REQUIRE_VULKAN=1`，设备初始化失败会失败，不允许静默跳过。覆盖常规上传、静止帧、更新、删除和资源销毁；CPU 测试同时覆盖清空、容量失败与设备失败重试。

先前因磁盘耗尽未完成的诊断保留在 [历史复审报告](../2026-10-18-geometry-rereview/README.md)，未将其改记为通过。本轮以修复后的永久生产 GPU 测试重新编译并执行。仅清理取得独占锁且至少 15 分钟未更新的编译增量会话，明细见 [cache-cleanup.json](cache-cleanup.json)，未删除源码、资产或历史报告。

## 边界

借用设备的旧 `try_new` / `new_storage_buffer` 入口保持兼容，调用方仍负责释放；新的生产墙砖入口使用 `try_new_owned`。本轮未统一迁移其他 SSBO 拥有权，也不将局部真实 GPU 上传和生命周期测试视为完整场景、最终画面或原进程行为差分通过。
