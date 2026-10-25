# shader-compiler 互审
审查者：shader_pipeline，2026-10-13。

审查源码 src/shader_compiler.rs、tests/shader_compiler_contract.rs 与 compiler/CONTRACT.md、原 compile-commands.json；只接受 B 原638模块OpModuleProcessed内嵌命令这一限定证据。

1. 初审入口名校验允许数字开头；已修为首字符ASCII字母或下划线，并增加1wrong反例。
2. 初审从全部LiteralString找请求可能误读OpSource正文；初审建议限制为OpString错误，真实638测试揭示原命令均为opcode330 OpModuleProcessed。实现已修为debug_module_processed + OpModuleProcessed，测试拒绝伪OpSource/OpString及重复330命令。pipeline独立按原发行SPIR-V指令复核638/638均为330，不能保留旧OpString恢复声明。
3. 复审from_compiled_stage使用共享Speedy解码产物，核对serialized stage、entry、SPIR-V入口和profile，避免raster vertex/fragment互换。
4. 请求参数15个token、顺序、Vulkan1.1/HLSL2021/调试标记直接对照原样本；路径通过OS参数传给Command，带空白不经shell切分。
5. compile_prepared是候选执行适配，真实DXC缺失，不能将638条命令验证记为实际源码编译或原shader-compiler完整还原。未知前处理、反射生产、缓存、变体生成、打包/临时文件清理仍登记。

源码和反例复审通过；编译/测试运行结论由根集成报告引用。不存在尚未修复的阻断问题。首字符和来源约束是审查发现；OpString选择错误是首次638真实资产测试发现并修复的具体原作对照差异。
