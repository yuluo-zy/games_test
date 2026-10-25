# shader-compiler：限定 DXC 请求合同恢复

2026-10-13，B 基线。实现接入 src/shader_compiler.rs；原函数名/源路径未知，未补造映射。

原 Cargo.lock 的 shader-compiler0.1.0 是 local/path 包，公开来源待核实。公共 shader-prepper/rspirv/spirq/DXC 实现复用，不逆向。MCP 独立 worker shader-compiler-20261013 查询3步通过，patched B SHA cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323匹配，shader_compiler public-symbol查询0结果仅限定PDB搜索边界，不能据此声称没有运行代码。

正向证据是404发行.bin/638模块的dxc-cl-option OpModuleProcessed，404原资产SHA与既有compile-commands.json逐项一致，见original-bin-hashes.json。原参数顺序：input-temp -E entry -T cs_6_4|vs_6_4|ps_6_4 -spirv -WX -Ges -fspv-target-env=vulkan1.1 -HV 2021 -Zi -Fo output-temp -Qembed_debug。原嵌入参数无-I/-D，临时输入与源路径不同，不能单凭这点证明预处理算法/顺序。generator0不能确认原DXC版本。

EmbeddedDxcRequest::from_module 复用原rspirv0.11.0解码，只从OpModuleProcessed提取已观察15参数语法，复核同模块入口和ExecutionModel；缺失、多条、未知语法、非法SPIR-V、阶段/入口不配显式失败。DxcRequest::prepared_arguments替换路径且保留其余token和原顺序，用OsString传参；source_arguments显式增加-I，仅候选源码适配。compile_prepared为候选进程适配，失败保留exit/stdout/stderr原字节，拒绝已有输出，成功必须读新文件并核对入口；输入/输出由调用者独占，原清理/取消/错误行为尚未知。

互审修复：pipeline owner指出digit首字符及误识别非OpModuleProcessed问题，已限定入口首字符ASCII字母或_、仅扫描Op::ModuleProcessed，并添加1wrong、OpSource/OpString冒名和重复OpModuleProcessed记录反例。旧validation-pipeline.md说SAME command过宽：直接源路径并追加-I是适配差异，本次明确标示，没有把历史报告当当前通过。

tests/shader_compiler_contract.rs验证638原模块逐token roundtrip/entry/model，及非法profile、缺参数、多-I、错阶段、空白路径边界。验证结果见validation.json；初次offline因append-only-vec缓存缺失失败留cargo-test.log。

本机where dxc及D:/tools、D:/game hidden file搜索未找到dxc.exe/dxcompiler.dll，没有下载，没有本轮compute/raster真实重编译通过声明。原预处理、反射提取/绑定规则、变体、打包、完整异常及原函数映射仍需恢复；此实现只恢复有证据的DXC请求合同。

compile_prepared的原错误传播、取消、临时文件清理均未取证；所有错误分支只登记候选实现/测试边界，不标为原作异常一致。


集中编译发现测试构造OpSource时误用rspirv新版字段LiteralBit32；已按原0.11实际API修为LiteralInt32。保留失败记录，最终重跑后才记通过。compiler源码和测试备份为source-copy.rs/tests-copy.rs，防止共享工作区外部同步覆盖。


集中638真实测试纠正初次归属假定：原命令实际opcode330 OpModuleProcessed，而非初稿/初审假定的OpString。embedded-request-opcodes.json逐638模块原record定位证实全部330；实现使用原rspirv.debug_module_processed，并显式拒绝仅在OpSource或OpString里的冒名命令，重复原记录报Ambiguous。原失败MissingEmbeddedRequest保留integration-test-attempt-1.log；修复后须重跑才能登记通过。


最终集中验收：compiler合同3/3通过，638原ModuleProcessed命令/serialized stage/SPIR-V入口三方零差异；runtime3/3和已有shader生产4/4回归也通过。validation.json绑定当前源码/共享codec/Cargo锁/原命令与真实输出hash。此通过范围不包含DXC实际运行或完整原compiler异常行为。


原反射producer新增缺口证据：404发布资产public/stored绑定名2577记录有215项恰去__alias_前缀、2362项原样、其他0（69独特去前缀对）；观察不证明任意输入规则，原处理位置、预处理关系与冲突异常仍待逆向，本轮未新增规范化实现。
