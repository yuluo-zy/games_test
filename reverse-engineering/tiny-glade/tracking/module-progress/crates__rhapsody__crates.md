# crates/rhapsody/crates

状态：功能作用已评估，逐文件恢复未评估。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../INTERFACE_PROGRESS.md)。

定义归属：private_source_candidate; ownership_requires_dependency_filter。

仅有源码清单或候选交付；完整模块行为尚未评估

## 功能作用评估（2026-10-13）

对渲染帧路径的结论：5 个子库中 3 个纯辅助、2 个在渲染链上但不阻塞流程。

- **aftermath-rs**（AMD Aftermath 崩溃转储）：辅助，仅在设备创建/错误路径，不碰帧输出。
- **dxgi-enumerate-gpus**（启动选 GPU）：辅助，一次性 init，不在帧路径。
- **gpu-profiler**（时间戳/调试标记）：纯观测，读回 profiling 数据，不改帧像素。
  - 以上三者不在 `evidence/render-recovery/WORKFLOW_RESET.md` 的任何闭包清单内。
- **vk-sync-fork**：在渲染链上——rg/binding.rs、rg/recorder.rs 有真实机器码调用 `vk_sync_fork::AccessType::is_write` / `get_access_info`（决定 barrier/access 分类）。但它是公开 `vk-sync 0.4.0` 的 fork，基座可复用公开版；唯一未知是分叉补丁。重建工程已在 `reconstruction/rhapsody-rebuild/src/rg/resource_rules.rs:41` 用显式外部分类器处理；WORKFLOW_RESET 明确允许同步作为外部边界。缺口是同步正确性的边角，不是流程功能。
- **shader-pipeline**：在渲染链上（反射→descriptor→pipeline 创建），但已恢复，内联为 `reconstruction/rhapsody-rebuild` 的 `crate::shader_pipeline`（compiled/reflection/shader_defines），本模块进度见 `crates__rhapsody__crates__shader-pipeline__src.md`（部分接线）。

结论：先前未对其逆向分析不是遗漏——从帧输出功能看分类正确：辅助的直接跳过，链上的两个要么可复用公开基座（vk-sync-fork）、要么已恢复（shader-pipeline）。真实挂账是 vk-sync-fork 分叉来源/补丁待核实（同步正确性边界，见 PROJECT_MEMORY.md），不是渲染流程功能性缺口。

## 接线与验证

接线：not_established。

未建立模块/文件级验收；候选交付不等于原作等价

## 已恢复的函数和类型范围

原函数全集尚未核对。下面的Rust定义清单仅登记实现符号。

## 明确缺口

- 逐文件恢复、原函数全集、类型/ABI、生产者/消费者和版本适配尚未核对
- vk-sync-fork 分叉来源/补丁待核实（同步正确性边界；基座可复用公开 vk-sync 0.4.0）
- aftermath-rs / dxgi-enumerate-gpus / gpu-profiler 如需等价接入，先确认原调用策略与消费者链

下一闭包：本模块对渲染帧路径非阻塞（辅助或已恢复/可复用）；不启动逐文件逆向，除非具体消费链要求。

## 证据

尚未登记模块级证明。

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/rhapsody/crates/aftermath-rs/src/lib.rs` | B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/rhapsody/crates/aftermath-rs/src/sys.rs` | B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/rhapsody/crates/dxgi-enumerate-gpus/src/lib.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/rhapsody/crates/gpu-profiler/src/backend/ash.rs` | B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/rhapsody/crates/gpu-profiler/src/shared.rs` | B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/rhapsody/crates/vk-sync-rs/src/cmd.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/rhapsody/crates/vk-sync-rs/src/lib.rs` | A,B | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

