# shader-pipeline

rhapsody 运行时与 `shader-compiler` 共用的着色器资产协议包，不含 GPU 或编译器依赖。

- `compiled`：`CompiledShaderPipeline` 与 `CompiledShaderStage`，提供 `load_compiled(path)`、`decode`、`encode`、`is_stale`、`spirv_bytes`；容器使用 `speedy` 编码。
- `pipeline_desc`：`ComputePipelineDesc`、`RasterPipelineDesc`，显式的管线输入描述；`compiled_file_path()` 给出发行文件路径规则。
- `shader_defines`：`ShaderDefines`，宏列表，提供 `artifact_hash()` 与 `duplicate_name()`。
- `reflection`、`reflection_ty`：绑定名哈希（`BindingNameHash`）与反射类型。

依赖：`speedy`、`const-fnv1a-hash`、`spirq`。

未完全恢复的部分：原 pipeline 描述的完整默认值与 Option 语义，完整的 material/pass 选择与资产库状态机，以及部分 zero-key compute 的宏列表来源。`pipeline_desc` 中的便捷构造函数不代表原构造函数的行为。包内 `compiled`、`pipeline_desc`、`shader_defines`、`reflection_ty` 共有 11 处 `#[test]`；本次没有运行它们。
