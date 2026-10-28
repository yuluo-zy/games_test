# 渲染 work-package 归宿（当前状态）

## 0. 说明

- 原审计对象 `reverse-engineering/tiny-glade/reconstruction/render/work-packages`（21 个子目录）在当前工作区不存在（工作区根与 `tiny-glade/` 下均无此路径），因此原表的逐目录归宿**无法复核**，旧表已删除。
- 本文件只保留仍能在当前代码中按符号定位到的归宿，供后续迁移参考；它不是完成度证明。
- 任务与缺口以 [IMPROVEMENT_TASKS.md](IMPROVEMENT_TASKS.md) 为准。

## 1. 当前可定位的归宿（2026-10-28，symbol 级核对）

| 旧概念 | 当前位置 | 状态 |
| --- | --- | --- |
| 着色器接口反射 `from_reflection` | `crates/rhapsody/src/pipeline/shader/interface.rs` | 存在（旧类型名 `ShaderInterfaces` 已不存在） |
| 临时资源 `get_or_create_temporal` / `export_temporal` | `crates/rhapsody/src/rg/temporal/mod.rs` | 存在 |
| temporal 重置判定 `should_reset_temporal` | `crates/country-core/src/render/frame_constants.rs` | 存在（消费者侧） |
| CPU arena 池 `CpuArenaPool` | `crates/rhapsody/src/command/cpu_arena/mod.rs`；裁剪逻辑在 `crates/rhapsody/src/collections/cpu_pool.rs` | 存在 |
| 批次编译 `BatchCompiler` | `crates/rhapsody/src/rg/compiled.rs` | 存在 |
| pass 容量检查 `check_pass_capacity` | `crates/rhapsody/src/rg/graph/declare.rs` | 存在 |
| 并行分块 `balanced_chunks` / `plan_chunks` | `crates/rhapsody/src/rg/parallel/mod.rs` | 存在 |
| 资源获取回执 `RecordedAcquires` | `crates/rhapsody/src/transfer/submit.rs` | 存在 |
| 单资源 descriptor 绑定 `bind_persistent_descriptor_set_single` | `crates/rhapsody/src/command/binding.rs` | 存在但零引用，见 IMP-44 |
| 哈希 token 表 `TokenMap` | `crates/rhapsody/src/collections/token_map/mod.rs` | 存在（路径已变） |
| 旧 API `persistent_pool_plan`、`with_create_info`、`queue_family_index_for` | 已删除 | 零引用，无归宿 |
| 图执行器 `GraphExecutor` / `ExecutedRenderGraph` | 未找到同名类型；`crates/rhapsody/src/rg/executor/` 目录存在 | **未核实**对应关系 |
| 临时编译状态 `PersistentCompileState` | 未找到同名类型 | **未核实** |
| 提交引擎 / 计数器 / 序列运行时 `SubmissionEngine`、`DeviceCounters`、`SequenceRuntime` | 未找到同名类型 | **未核实**（可能已由 `command/`、`device/`、`sync/` 中的实现替代） |
| 事件 oracle `EventOracle` | 未找到 | 旧验证工具，不在库内 |

## 2. 未决

- 标注"未核实"的 5 行需要在有源目录时复核；在源目录缺失的情况下，不能据此宣称渲染 work-package 已全部迁入。
- 具体缺口清单（候选排序、epoch graveyard、同步 setup、原多队列策略）仍以 `crates/rhapsody/README.md` 为准，本轮未核对其内容。
