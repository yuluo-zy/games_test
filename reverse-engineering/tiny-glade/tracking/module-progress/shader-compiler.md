# shader-compiler

状态：部分接线。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../INTERFACE_PROGRESS.md)。

定义归属：local_path_package; original_source_root_unobserved_in_B_PDB。

B包名已知，原源码目录未在runtime PDB定位；原638请求、include/chunk/宏拼接、实际DXC、精确public反射和后处理/打包策略接入现有工程。

## 接线与验证

接线：现有rhapsody源码build_pipeline与批量audit CLI闭环，无原metadata注入；原资产构建完整manifest仍未知。

404新包/638stage实际构建有效；393官方1.9、11官方preview1.10；新metadata210/404与原一致。最终10测试及严格all-targets Clippy、新fill真实GPU读回通过；不代表原shader全等。

## 已恢复的函数和类型范围

- 638原OpModuleProcessed(330)请求提取与原参数实际调用
- 原shader-prepper include/pragmaonce→#line/defines→真实DXC
- SpirQ精确fork独立reflection/name/UBO/type/stage合并
- rspirv LF-only source/header policy→原Speedy容器
- build_pipeline与observed raster_variants

## 明确缺口

- 原DXC部署版本/fork未确认；11包使用官方preview1.10避免1.9崩溃
- 170compute原宏tuple未独证，其中38非零key未知
- 新编译194包完整metadata不同、336stage公共反射不同与shader算法残差
- 原provider路径/错误/temp/取消/清理/cache和完整asset CLI策略

下一闭包：恢复原compute macro/provider/config，修metadata与算法残差；精确验证所需compiler fork差异

## 证据

- [evidence/shader-build-2026-10-13/dxc/validation.json](../../evidence/shader-build-2026-10-13/dxc/validation.json)
- [evidence/shader-build-2026-10-13/dxc/delivery.json](../../evidence/shader-build-2026-10-13/dxc/delivery.json)
- [evidence/shader-build-2026-10-13/pipeline/validation.json](../../evidence/shader-build-2026-10-13/pipeline/validation.json)
- [evidence/shader-build-2026-10-13/provenance/PROVENANCE.md](../../evidence/shader-build-2026-10-13/provenance/PROVENANCE.md)
- [evidence/shader-build-2026-10-13/integration.json](../../evidence/shader-build-2026-10-13/integration.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/rhapsody-rebuild/src/shader_compiler.rs

[reconstruction/rhapsody-rebuild/src/shader_compiler.rs](../../reconstruction/rhapsody-rebuild/src/shader_compiler.rs)

SHA256：`f8c27d9518fc82d93a8b721308b384761c405bac99a8e9bed8387494a16490bd`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `profile` | 18 | 未由扫描证明；以证据限定范围为准 |
| `execution_model` | 25 | 未由扫描证明；以证据限定范围为准 |
| `fmt` | 61 | 未由扫描证明；以证据限定范围为准 |
| `from` | 80 | 未由扫描证明；以证据限定范围为准 |
| `new` | 86 | 未由扫描证明；以证据限定范围为准 |
| `stage` | 102 | 未由扫描证明；以证据限定范围为准 |
| `entry` | 105 | 未由扫描证明；以证据限定范围为准 |
| `prepared_arguments` | 109 | 未由扫描证明；以证据限定范围为准 |
| `source_arguments` | 129 | 未由扫描证明；以证据限定范围为准 |
| `validate_module` | 139 | 未由扫描证明；以证据限定范围为准 |
| `compile_prepared` | 153 | 未由扫描证明；以证据限定范围为准 |
| `from_compiled_stage` | 180 | 未由扫描证明；以证据限定范围为准 |
| `parse` | 197 | 未由扫描证明；以证据限定范围为准 |
| `from_module` | 237 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/shader_compiler/build.rs

[reconstruction/rhapsody-rebuild/src/shader_compiler/build.rs](../../reconstruction/rhapsody-rebuild/src/shader_compiler/build.rs)

SHA256：`9336953401382281eacb523e0c8ccc00f3db461859428400a67e25f2fd2320bc`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `raster_variants` | 32 | 未由扫描证明；以证据限定范围为准 |
| `build_pipeline` | 42 | 未由扫描证明；以证据限定范围为准 |
| `build_pipeline_with_recipe` | 50 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/shader_compiler/pack.rs

[reconstruction/rhapsody-rebuild/src/shader_compiler/pack.rs](../../reconstruction/rhapsody-rebuild/src/shader_compiler/pack.rs)

SHA256：`4b8f98c73a948668e41765ca535146dd408e154b372036fecb5459816f223e52`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `prepare_spirv_for_package` | 9 | 未由扫描证明；以证据限定范围为准 |
| `package` | 28 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/shader_compiler/preprocess.rs

[reconstruction/rhapsody-rebuild/src/shader_compiler/preprocess.rs](../../reconstruction/rhapsody-rebuild/src/shader_compiler/preprocess.rs)

SHA256：`08104de3204660e1c743d6ee6f7617ddb8c44be40bb1dbe2ed3415f2934b3618`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `relative` | 23 | 未由扫描证明；以证据限定范围为准 |
| `resolve_path` | 30 | 未由扫描证明；以证据限定范围为准 |
| `get_include` | 52 | 未由扫描证明；以证据限定范围为准 |
| `prepare_file` | 63 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/shader_compiler/reflect.rs

[reconstruction/rhapsody-rebuild/src/shader_compiler/reflect.rs](../../reconstruction/rhapsody-rebuild/src/shader_compiler/reflect.rs)

SHA256：`c321bddb024fe880b54baec75e4bfcb0fa572ee9b8e1ac2ed9d95f8fe9ee3bec`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `scalar` | 13 | 未由扫描证明；以证据限定范围为准 |
| `vector` | 22 | 未由扫描证明；以证据限定范围为准 |
| `stored_type` | 28 | 未由扫描证明；以证据限定范围为准 |
| `kind` | 72 | 未由扫描证明；以证据限定范围为准 |
| `canonical_name` | 83 | 未由扫描证明；以证据限定范围为准 |
| `size` | 86 | 未由扫描证明；以证据限定范围为准 |
| `generate_reflection` | 92 | 未由扫描证明；以证据限定范围为准 |
