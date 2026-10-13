# 方法说明：GPU 侧 pass 盘点如何完成

## 输入
- `compiled-shaders/`（`/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/compiled-shaders/`，404 个 `.bin`）。
- SPIR-V 语法表 `skeleton/_tools/spirv.core.grammar.json`（复用既有 `scripts/render-architecture-shaders.py` 同款 grammar 驱动反射方式）。
- 既有反射 `evidence/render-architecture/shader-reflection.json`（20 个 shader）用于逐字复用与交叉核对。

## 1) 文件名清单
对所有 404 个文件名按 `^(.+)\.(cs|raster)\.hlsl_(.+?)\.bin$` 解析：
- pass 名、阶段（`cs`/`raster`）、入口提示（compute 为 `cs_main`/`cs_pass1`/`sample1` 等，raster 无入口槽，其 16 位 hex 段是 HLSL 源哈希）。
- compute variant：`0`（单变体）或 16 位 hex（多变体）；raster variant：16 位 hex。
- 由 pass 名归类到 35 个分析族（首匹配规则，覆盖全部 404 文件；`unclassified` 兜底最终为空集）。

## 2) SPIR-V 深解析
写了一个仅用 stdlib 的宽容 SPIR-V 字解析器（`scripts/render-passes/parse_shader_catalog.py`）：
- 按 magic `0x07230203` 定位每个 `.bin` 内所有 SPIR-V 模块；每个模块读取版本字后的指令流。
- 提取：`OpEntryPoint`(15) 的执行模型+入口名；`OpExecutionMode`(16) 的 `LocalSize`；`OpCapability`(17)；`OpName`(5)；`OpVariable`(59)+`OpDecorate`(71) 的 `DescriptorSet`(33)/`Binding`(34)；`Block`(2)/`BufferBlock`(3) 装饰。
- 绑定类型推断：沿 `OpTypePointer`→pointee，若为 `OpTypeImage` 按 `Sampled`(2=StorageImage) 分类；`OpTypeSampler`/`OpTypeSampledImage`；struct + `Block`→UBO 或 SSBO（依存储类），`BufferBlock`→旧式 SSBO，PushConstant 单独标记。推断是 best-effort，可能误标不透明载体。
- 每个模块与文件计算 SHA-256；每个模块记 `opcode_histogram`、`debug_names`、`member_names`。

**深解析范围**：全部 404 个文件（638 个模块）都做了上述反射；并额外对每个族抽取代表性 shader 的完整绑定明细（见 `catalog.md`）。

## 3) 与既有反射的交叉复用
- `evidence/render-architecture/shader-reflection.json` 覆盖的 20 个文件（nani cull/bucket ×2、cull roof_tile ×2、indirect-to-dispatch、gi_* ×5 或 6、ssgi_* ×3、taa、tgsr ×4、extract_half_res、calculate_reprojection）的模块数据按其 verbatim 复用，并在 `catalog.json` 的 `cross_reference` 记录逐字文件哈希与模块哈希核对结果（20/20 全部匹配）。

## 4) 输出
- `evidence/render-passes/shaders/catalog.json`（机读，5.6 MB）：per pass → family、stage、variant_count、per-variant file/sha256/size、entry point、localsize、capabilities、bindings（name/storage/set/binding/kind）、debug/member names、opcode histogram。
- `catalog.md`：人类可读，按族分组，标 compute/raster 与每族最强证据。
- `summary.md`：〈500 词总结。

## 限制（不要据此过度声明）
- 纯静态反射：只能证明“声明了什么入口/绑定/能力”，不能证明某变体每帧被绑定、descriptor 实句柄、format/extent、执行顺序或算法。
- 绑定 kind 是由 SPIR-V 类型推断的近似，`UniformConstant(other)`/`Uniform-struct(no Block)` 等载体未完全识别。
- `debug_names`（OpName）是 shader 侧名字，不一定与 CPU 字段一一对应。
- 16 位 hex 变体哈希只代表多份变体编译，不代表多 pass 或多帧必然运行。
- 没有反编译 HLSL/算法；`member_names`/opcode 直方图只作为后续算法恢复的线索。
