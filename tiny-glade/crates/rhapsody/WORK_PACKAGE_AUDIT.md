# 渲染 work-package 归宿复核

2026-10-14 对 `reverse-engineering/tiny-glade/reconstruction/render/work-packages` 的 21 个目录逐个核查。检查的是库体/类型/调用规则；`main`、JSON 编解码、snapshot fixture、事件 oracle 与旧验证用例不是新的游戏业务 body。不是仅按包名或同名函数判断完成。

| 原目录 | 正式归宿或未迁入原因 |
| --- | --- |
| `barrier-record-r5` | `raw/barrier.rs`，初始/复用/逐 pass 屏障、先 drain 后失败的规则；旧 runner/事件 fixture 留在证据工程 |
| `compiled-state` | `rg/compiled.rs`，BatchCompiler flush/finish、初始 transition 和 pre-pass 决策；`rg/compiled_graph.rs` 保留上轮叶 API。旧 JSON runner 不作为运行时 |
| `descriptor-binding-r8` | 本次补入 `raw/descriptor_binding.rs`：sparse layout key、资源 tag、图像数组、buffer/dynamic offset、set membership、compute slot tuple、update-before-bind。旧单资源 `raw/binding` 不足以覆盖此闭包，现两者分工明确。layout/pool 复用 `raw/descriptor_set`，没有新造另一份公共依赖 |
| `device-counters-r6` | `raw/device.rs`：原每队列 slot 起始为 1、counter wrapping、primary/auxiliary 选择、global bucket replacement；runner 非生产代码 |
| `execution-chain-r8` | 当前目录只有 Cargo.lock/target 构建产物，无 Rust 源或 Cargo.toml。真正集成库在 `render/execution.rs`，已迁入 `rg/executor.rs`；不从产物目录补造代码 |
| `executor-temporal-r4` | `rg/executor.rs` 的 GraphExecutor/ExecutedRenderGraph/ExportedRenderGraphResources，`rg/recorder.rs`、`rg/vulkan.rs` 和 `rg/temporal.rs`。旧 `BackendEvent` 日志模拟器的 execute_compiled/submit_pending_cbs/timeline 被后续 r7/r8 真实序号/后端调用链替代，不再额外保留一份固定事件状态机 |
| `gpu-lifecycle` | 当前目录无 Rust 源，只有旧产物/目录残留；实际原规则见 `gpu-lifecycle-r3` |
| `gpu-lifecycle-r3` | `raw/managed/tracking.rs`、`handle.rs`、`lifecycle.rs`、`resources.rs`：generation/refcount/max last-use、CPU drop、waste 到命令桶、contiguous completion/late drop。`EventOracle`、event_oracle.rs JSON 与固定事件演示留作原验证工具；当前真实 SequenceRuntime 与 managed store 承接规则 |
| `graph-compiler-r2` | `rg/graph.rs`、`resource_info.rs`、`resource_rules.rs`、`compiled.rs`、`resource_registry.rs`/`resource_pool.rs`：population、访问分析、batch、alias、exports。serde runner parse/to_json 是原验证输入输出适配 |
| `pipeline-descriptors-r7` | `raw/descriptor_set.rs` 的 descriptor_set_layout、LayoutPlan::with_create_info、persistent_pool_plan、with_compute_pipeline_layout、pool/allocate backing array 规则 |
| `pipeline-interfaces-r6` | `raw/shader.rs` 的 `ShaderInterfaces::from_reflection`：descriptor stage union/conflict、location/component/type link；`PipelineLayout::new` 检查 push alignment/overflow。反射复用原 SpirQ。旧 merge_bindings/validate_* 是分立语义投影，规则已进入实际布局生产；reflect-shaders/validate-catalog 等 CLI 留证据目录，不能当新恢复 shader 算法 |
| `queue-callers-r7` | `raw/command_buffer.rs` 和 `raw/device.rs`：begin/reset、queue slot/counter 来源与异常顺序 |
| `queue-submit-r8` | `raw/submit.rs`：pending flush→当前 batch、最后 open 命令结束、注册移动和失败保留；Vulkan 调用由 SubmissionEngine/后端接线 |
| `record-resources-r7` | `rg/recorder.rs`：初始化/创建、resource registry、逐 pass barrier、FnOnce callback、export、取出后失败/消费顺序 |
| `record-submit-r6` | r6 是固定 handle 99/100、receipt 41 的事件语义 fixture，不是原真实设备实现。其 callback failure、end-last、empty wait-only、error/post-submit 顺序进入 `rg/recorder.rs`/`executor.rs` 和 `raw/submit.rs` 后续 r7/r8。旧 fixture 不并入运行时输出 |
| `registry-pool` | `rg/resource_registry.rs`、`resource_pool.rs`：logical→physical 访问、payload move、LIFO exact-key pool、export alias 首次移动、clear 保留 capacity |
| `resource-info` | `rg/resource_info.rs`：creation/export/pass usage、access chains/lifetime；公共/原 private usage 换算在 `raw/resource_usage` 与兼容 `rg/resource_usage` |
| `sequence-domain-r5` | 早期探索模型的 begin_cb 从 0 起、全局 u64 bucket 碰撞与队列 timeline 实验已由 r6/r8 原 slot 起始 1 和实际 SequenceRuntime 替代；`raw/device.rs`、`raw/managed/lifecycle.rs` 显式保留 collision/cross-domain 缺口。不把已被原码纠正的探索起始值重新放回生产 |
| `sequence-runtime-r8` | `raw/managed/lifecycle.rs`：实际 DeviceCounters→Command、tracker、pending pop/validation、GPU完成到资源回收；r8 runner/failure_name/scenario 只是 oracle 显示适配 |
| `temporal-lifecycle-r8` | `rg/state.rs::PersistentCompileState` 的 take/restore（先取图、后验 temporal、失败不回滚/先安装后drop）；原 `should_reset_temporal` 属于 country-core consumer，已在 `country-core::render::frame_constants::FrameConstants::should_reset_temporal`，+0x80 字等于 0。snapshot_fixture 的 opaque 字节不是运行时类型 |
| `temporal-r5` | `rg/temporal.rs::get_or_create_temporal/get_or_create_temporal_pair/export_temporal`：原 raw0/raw1/exported2、access0x37、重复请求、extent替换/fixed断言、buffer不比较desc、pair名字/奇偶、失败保留第一次创建；物理持有/释放由 typed owner 接线。原 reset/eviction/ABI 仍不完整 |

源级迁移和实现边界见 [README](README.md)、[migration-sources.json](migration-sources.json)。旧原指令/GPU报告仍在原证据工程，没有从“旧包没有单独复制”推断其 body 未恢复。候选排序、epoch graveyard、同步 setup、原未证实 ABI、主图 provider 与原多队列策略仍按 README 的具体缺口登记。
