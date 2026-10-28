# shader-compiler

rhapsody 的着色器离线构建器：HLSL 预处理、DXC 调用、SPIR-V 反射、包打包与 bindless ABI 降级。运行时依赖的资产结构由 `shader-pipeline` 提供，本 crate 以 `pub use shader_pipeline` 重新导出它。

- `preprocess::prepare_file`：include、`#pragma once`、`#line` 处理。
- `lib.rs` 的 `DxcStage`、`DxcRequest`、`EmbeddedDxcRequest`、`CompileError`：DXC 参数构造（`prepared_arguments`）、真实编译（`compile_prepared(dxc, input, output)`）与 SPIR-V 模块校验（`validate_module`）。DXC 路径由调用方传入；测试通过 `DXC` 环境变量查找，默认 `~/tools/dxc/bin/dxc`，找不到时跳过。
- `build::{build_pipeline, build_pipeline_with_recipe, raster_variants}`：单个 compute 或 vertex/fragment 管线的完整构建。
- `reflect::generate_reflection`、`pack::{prepare_spirv_for_package, package}`：反射与发行格式打包。
- `bindless::{BindlessAbi, lower_pipeline, lower_pipeline_with_limit}`：把已编译管线降级到共享 heap ABI。

目前未完全恢复的部分：原离线工具的 DXC 版本与 fork 提交、完整的异常与临时目录/缓存策略，以及完整资产状态机。测试 `tests/original_contract.rs` 需要原游戏资产，由 `TG_B_GAME_ROOT` 指定；缺失时显式跳过。本次没有运行这些测试。
