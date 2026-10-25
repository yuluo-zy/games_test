# crates/rhapsody/crates/shader-pipeline/src

状态：部分接线。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../docs/current/INTERFACE_PROGRESS.md)。

定义归属：local_path_source_candidate; public_origin_unconfirmed。

B原compiled容器完整schema、反射/类型、文件键与binding hash已恢复；5个原PDB文件映射接入现有rhapsody。 续轮已接独立反射、后处理与fresh容器producer。

## 接线与验证

接线：恢复源码→实际DXC→独立反射→Speedy→真实compiled consumer；fresh fill在Intel UHD630读回，两egui layout成功。

B404原文件/638模块完整typed decode/encode；独立再反射404完整metadata零差异；新编译404包中210metadata与原一致，191依赖集合不同。wire再生成27exact/377map顺序变化，不计字节全等。

## 已恢复的函数和类型范围

- CompiledShaderPipeline阶段/依赖/完整stored reflection编解码
- BindingNameHash::_dims规则与lookup
- Type/Scalar/Vector/Matrix/Array/Struct wire tags及公共spirq尺寸接口
- compiled_file_path、DefaultHasher每对XOR键

## 明确缺口

- 完整ShaderCompilerArtifacts加载错误上下文/缓存所有权
- 原描述对象optional字段与默认构造规则（新constructor是便利候选）
- 完整原macro manifest、shader算法与AssetShaderLibrary/pass真实联接
- A/C与原异常副作用未验收

下一闭包：补原macro manifest/metadata语义残差与加载缓存错误，接country-core资产/pass

## 证据

- [evidence/shader-toolchain-2026-10-13/pipeline/validation.json](../../evidence/shader-toolchain-2026-10-13/pipeline/validation.json)
- [evidence/shader-toolchain-2026-10-13/pipeline/delivery.json](../../evidence/shader-toolchain-2026-10-13/pipeline/delivery.json)
- [evidence/shader-toolchain-2026-10-13/integration.json](../../evidence/shader-toolchain-2026-10-13/integration.json)
- [evidence/shader-build-2026-10-13/pipeline/validation.json](../../evidence/shader-build-2026-10-13/pipeline/validation.json)
- [evidence/shader-build-2026-10-13/integration.json](../../evidence/shader-build-2026-10-13/integration.json)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/rhapsody/crates/shader-pipeline/src/compiled.rs` | A,B | [reconstruction/rhapsody-rebuild/src/shader_pipeline/compiled.rs](../../reconstruction/rhapsody-rebuild/src/shader_pipeline/compiled.rs)：B_PDB_path_and_original_native_reader_contract | 全集/逐函数映射待核对 | candidate_present；B原schema/键/存储字段合同与404逐字节回写；不代表整个原文件全部函数。构造层明确便利候选。：[evidence/shader-toolchain-2026-10-13/pipeline/delivery.json](../../evidence/shader-toolchain-2026-10-13/pipeline/delivery.json), [evidence/shader-toolchain-2026-10-13/pipeline/validation.json](../../evidence/shader-toolchain-2026-10-13/pipeline/validation.json) |
| `crates/rhapsody/crates/shader-pipeline/src/pipeline_desc.rs` | A,B | [reconstruction/rhapsody-rebuild/src/shader_pipeline/pipeline_desc.rs](../../reconstruction/rhapsody-rebuild/src/shader_pipeline/pipeline_desc.rs)：B_PDB_path_and_original_native_reader_contract | 全集/逐函数映射待核对 | candidate_present；B原schema/键/存储字段合同与404逐字节回写；不代表整个原文件全部函数。构造层明确便利候选。：[evidence/shader-toolchain-2026-10-13/pipeline/delivery.json](../../evidence/shader-toolchain-2026-10-13/pipeline/delivery.json), [evidence/shader-toolchain-2026-10-13/pipeline/validation.json](../../evidence/shader-toolchain-2026-10-13/pipeline/validation.json) |
| `crates/rhapsody/crates/shader-pipeline/src/reflection.rs` | A,B | [reconstruction/rhapsody-rebuild/src/shader_pipeline/reflection.rs](../../reconstruction/rhapsody-rebuild/src/shader_pipeline/reflection.rs)：B_PDB_path_and_original_native_reader_contract | 全集/逐函数映射待核对 | candidate_present；B原schema/键/存储字段合同与404逐字节回写；不代表整个原文件全部函数。构造层明确便利候选。：[evidence/shader-toolchain-2026-10-13/pipeline/delivery.json](../../evidence/shader-toolchain-2026-10-13/pipeline/delivery.json), [evidence/shader-toolchain-2026-10-13/pipeline/validation.json](../../evidence/shader-toolchain-2026-10-13/pipeline/validation.json) |
| `crates/rhapsody/crates/shader-pipeline/src/reflection_ty.rs` | A,B | [reconstruction/rhapsody-rebuild/src/shader_pipeline/reflection_ty.rs](../../reconstruction/rhapsody-rebuild/src/shader_pipeline/reflection_ty.rs)：B_PDB_path_and_original_native_reader_contract | 全集/逐函数映射待核对 | candidate_present；B原schema/键/存储字段合同与404逐字节回写；不代表整个原文件全部函数。构造层明确便利候选。：[evidence/shader-toolchain-2026-10-13/pipeline/delivery.json](../../evidence/shader-toolchain-2026-10-13/pipeline/delivery.json), [evidence/shader-toolchain-2026-10-13/pipeline/validation.json](../../evidence/shader-toolchain-2026-10-13/pipeline/validation.json) |
| `crates/rhapsody/crates/shader-pipeline/src/shader_defines.rs` | A,B | [reconstruction/rhapsody-rebuild/src/shader_pipeline/shader_defines.rs](../../reconstruction/rhapsody-rebuild/src/shader_pipeline/shader_defines.rs)：B_PDB_path_and_original_native_reader_contract | 全集/逐函数映射待核对 | candidate_present；B原schema/键/存储字段合同与404逐字节回写；不代表整个原文件全部函数。构造层明确便利候选。：[evidence/shader-toolchain-2026-10-13/pipeline/delivery.json](../../evidence/shader-toolchain-2026-10-13/pipeline/delivery.json), [evidence/shader-toolchain-2026-10-13/pipeline/validation.json](../../evidence/shader-toolchain-2026-10-13/pipeline/validation.json) |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/rhapsody-rebuild/src/shader_pipeline/compiled.rs

[reconstruction/rhapsody-rebuild/src/shader_pipeline/compiled.rs](../../reconstruction/rhapsody-rebuild/src/shader_pipeline/compiled.rs)

SHA256：`7e791fc8244ca987ce78712c590ccd29949460e3ee5e749eee4076ed1804ef13`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `fmt` | 34 | 未由扫描证明；以证据限定范围为准 |
| `load_compiled` | 42 | 未由扫描证明；以证据限定范围为准 |
| `decode` | 54 | 未由扫描证明；以证据限定范围为准 |
| `encode` | 66 | 未由扫描证明；以证据限定范围为准 |
| `spirv_bytes` | 78 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/shader_pipeline/mod.rs

[reconstruction/rhapsody-rebuild/src/shader_pipeline/mod.rs](../../reconstruction/rhapsody-rebuild/src/shader_pipeline/mod.rs)

SHA256：`45f60348cb6b537cfdfd19368836077e785e608601bfa9aad54d479ee5936bbb`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
### reconstruction/rhapsody-rebuild/src/shader_pipeline/pipeline_desc.rs

[reconstruction/rhapsody-rebuild/src/shader_pipeline/pipeline_desc.rs](../../reconstruction/rhapsody-rebuild/src/shader_pipeline/pipeline_desc.rs)

SHA256：`5dfeb34b124ca41e017faf2354a2f54a6fa89d38750c542b0bf55e9064102e39`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 13 | 未由扫描证明；以证据限定范围为准 |
| `with_entry` | 17 | 未由扫描证明；以证据限定范围为准 |
| `compiled_file_path` | 25 | 未由扫描证明；以证据限定范围为准 |
| `new_vertex_fragment` | 45 | 未由扫描证明；以证据限定范围为准 |
| `compiled_file_path` | 54 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/shader_pipeline/reflection.rs

[reconstruction/rhapsody-rebuild/src/shader_pipeline/reflection.rs](../../reconstruction/rhapsody-rebuild/src/shader_pipeline/reflection.rs)

SHA256：`d14d2baa47c56de1375dd94ce93a4012d50fc827f9fcb1191f404c8c28843986`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `new` | 7 | 未由扫描证明；以证据限定范围为准 |
| `with_suffix_dims` | 14 | 未由扫描证明；以证据限定范围为准 |
| `binding_by_name` | 74 | 未由扫描证明；以证据限定范围为准 |
| `binding_by_hash` | 81 | 未由扫描证明；以证据限定范围为准 |
| `effective_descriptor_sets` | 92 | 未由扫描证明；以证据限定范围为准 |
| `effective_vertex_inputs` | 98 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/shader_pipeline/reflection_ty.rs

[reconstruction/rhapsody-rebuild/src/shader_pipeline/reflection_ty.rs](../../reconstruction/rhapsody-rebuild/src/shader_pipeline/reflection_ty.rs)

SHA256：`5b4be5e91a855b8dbda8e7ed5ceeb7c2573401a1b75749ae7f5ff25b15b095e7`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `to_spirq` | 64 | 未由扫描证明；以证据限定范围为准 |
| `to_spirq` | 76 | 未由扫描证明；以证据限定范围为准 |
| `to_spirq` | 85 | 未由扫描证明；以证据限定范围为准 |
| `convert` | 88 | 未由扫描证明；以证据限定范围为准 |
| `nbyte` | 130 | 未由扫描证明；以证据限定范围为准 |
### reconstruction/rhapsody-rebuild/src/shader_pipeline/shader_defines.rs

[reconstruction/rhapsody-rebuild/src/shader_pipeline/shader_defines.rs](../../reconstruction/rhapsody-rebuild/src/shader_pipeline/shader_defines.rs)

SHA256：`3d004b0811c83239ed3e5f29b9aa6638a24bdcbedd38e2753ba307edb6fbfad1`；缺口注释标记数：0。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `artifact_hash` | 7 | 未由扫描证明；以证据限定范围为准 |
