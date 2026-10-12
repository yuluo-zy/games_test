# R1 图编译器合同

状态：`evidence_contract_started`。这是工作包合同，不是实现或验证完成报告。

- 核心函数地址：28
- 含一层内部依赖的地址：61
- 已有反编译：6
- 已有局部原码验证：4
- 下一批定向查询：20

## 已确认入口

- `0x140161cd0` / 9204 bytes：`rhapsody::rg::state::RenderGraphState::compile::h1404e5612569a6e5`（public-only；已有伪代码）
- `0x1401647b0` / 5284 bytes：`rhapsody::rg::state::RenderGraphState::compile::h1edf2b06c104b210`（public-only）
- `0x140166510` / 3338 bytes：`rhapsody::rg::state::RenderGraphState::compile::hc97c995713b979ef`（public-only）
- `0x1402c1d40` / 4674 bytes：`rhapsody::rg::compiled::CompiledRenderGraph$LT$UserData$GT$::record_batch::hedb91080fd331410`（public-only）
- `0x140530430` / 1416 bytes：`rhapsody::rg::resource_info::ResourceInfo::calculate::h1c626033b539586e`（public-only；已有伪代码）
- `0x1402c3d00` / 1403 bytes：`rhapsody::rg::compiled::CompiledRenderGraph$LT$UserData$GT$::add_pre_pass_barriers::hcb31af73759a11ec`（public-only；已有伪代码）
- `0x140be7ba0` / 5274 bytes：`rhapsody::rg::resource_registry::ResourceRegistry::populate_resources`
- `0x1402c4280` / 385 bytes：`rhapsody::rg::compiled::CompiledRenderGraph$LT$UserData$GT$::new::h6e8718a94436b742`（public-only）
- `0x140be9040` / 557 bytes：`rhapsody::rg::resource_registry::ResourceRegistry::into_exports`
- `0x140be6d70` / 1133 bytes：`rhapsody::rg::resource_pool::ResourcePool::insert_texture`
- `0x140be7320` / 973 bytes：`rhapsody::rg::resource_pool::ResourcePool::insert_buffer`
- `0x140be6b90` / 473 bytes：`rhapsody::rg::resource_pool::ResourcePool::take_texture`
- `0x140be71e0` / 314 bytes：`rhapsody::rg::resource_pool::ResourcePool::take_buffer`
- `0x140be95e0` / 30 bytes：`rhapsody::rg::compiled::BatchCompiler::new`
- `0x140be9600` / 131 bytes：`rhapsody::rg::compiled::BatchCompiler::flush_batch`
- `0x1404f7ca0` / 563 bytes：`rhapsody::rg::temporal::TemporalRenderGraphRecorder$LT$UserData$GT$::export_temporal::h949acac9fc332e8a`（public-only）

## 下一关口

先关闭 `ResourceInfo`、访问记录和 compiled graph 的字段约束，再写整条 CPU 编译路径。
查询只使用 `query-queue.json` 中的有限地址；不得重新扫描全 PDB 或逐函数翻译。
