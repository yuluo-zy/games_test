# shader-pipeline

B 版 shader 资产协议包。供 `rhapsody` 运行时和 `shader-compiler` 构建器共同使用，避免将编译工具依赖塞入 GPU 管线类型。

已恢复 `CompiledShaderPipeline`/stage 的 Speedy 读写、SPIR-V word/byte 载体、绑定类型与 uniform 布局、宏列表/hash 与发行文件路径规则、runtime `load_compiled` 的读取/解码和错误文本。`pipeline_desc` 是显式输入描述；其中常见入口便利构造为候选 API，不把其空宏默认值当原构造函数事实。std hash、Speedy 和 SpirQ 复用原依赖。

来源：[`rhapsody-rebuild/src/shader_pipeline`](../../../../../reverse-engineering/tiny-glade/reconstruction/rhapsody-rebuild/src/shader_pipeline)，主要提交 `e4e0fbe2`、`db1ecad7`；文件级路径/提交见上级 [migration-sources.json](../../migration-sources.json)。逆向依据是 B 的 404 发行 `.bin`、638 stage、PDB 和 `evidence/shader-gap-investigation-2026-10-13`。

已完成资产结构可读写、原 key/路径、runtime 解码与编译器消费接线。还缺原 pipeline desc 的完整对象/Option/默认语义、原完整 material/pass 选择与资产库状态机，以及 132 个 zero-key compute 宏 tuple 的原 producer 证明。仅 key 为零不能推出宏列表为空；TGV 源码选择宏也不是原 artifact 宏。

历史 DXC 全量构建 404 包/638 stage 成功，397/404 typed 反射相同，仍有 7 包 schema 残差与公共接口差异；协议 roundtrip/反射一致不等于完整 shader 算法与整帧一致。后续开发需保持这些验证范围明确，勿复制旧 metadata 冒充新反射。
