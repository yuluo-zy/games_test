# 实际 DXC、完整构建与全量对照交付

当前冻结输入的测试10/10、严格all-targets Clippy和真实新包GPU消费均通过。通过范围是恢复源码经原公开预处理/反射/编解码依赖生成新包并可消费；不是全部原shader算法或原完整compiler异常策略等价。

## 官方工具与原处理纠正

Windows官方DXC位于D:/game/tools/shader-toolchain/专用目录，不改全局环境。v1.9.2609（运行版本1.9.2609.5）Windows包SHA256与微软release公布值ad31b1fc8443175d204f77a611fdb3ef2ec42759bdc2f1167368de24a4a7e7f1完全匹配。对真实访问异常单独使用官方v1.10.2605.37，SHA256与公布值18da85ccac4d1736209bad960f0a124928a5c5a6046882e331c605d66744a889完全匹配。见toolchain.json/alternate-toolchain.json。来源：https://github.com/microsoft/DirectXShaderCompiler/releases/tag/v1.9.2609 与 https://github.com/microsoft/DirectXShaderCompiler/releases/tag/v1.10.2605.37。

原638模块header generator是0x000f0000（Google rspirv ToolID15），并非历史记录中的0；fresh官方输出0x000e0000，包后用原rspirv header变15。原13994 OpSource+498 OpSourceContinued共14492记录仅LF/空，保留4,230,414换行，证据original-debug-source-corpus.json。不能从这些空正文恢复原HLSL；也不能确认原DXC版本。

## 实际路径与差异修复

src/shader_compiler.rs保留原638命令15参数顺序，读取OpModuleProcessed330并核对profile/entry/stage；真实进程拒绝旧输出，保留退出码和诊断原byteVec。Display显示可读诊断，但原错误类型、临时文件清理/取消/缓存尚未证明。preprocess/reflect/pack/build子模块由pipeline owner恢复：原shader-prepper包含展开、调用者宏和#line→DXC prepared输入→原SPIR-Q fork元数据→源正文LF与rspirv header→Speedy原wire。生成不注入原metadata/依赖，不复制原SPIR-V。

源修复限定在阻塞：egui以原SHADER_VARIANT_DEPTH_ONLY控制无资源/IO/OpKill的depth片元；nani_velocity_only的Affine3Packed与共享12f32声明同一guard；两砖shader的原color_library set0/binding21用显式恢复适配覆写共享默认b14，避免与VS t14冲突。pipeline owner修3helper相对include路径。其它shader算法残差未漫游修改。

初轮v2：387有效包615模块，17资产失败（8include、4真实cross-stage资源冲突、2重复类型、3officialDXC AV）。v3重验失败资产得到6新包12模块，剩11 AV；v4保持参数，换单独官方1.10版本，11/11生成有效包。原失败、旧结果与两个工具身份均保留。

## 完整清单与验收边界

merged-audit.json按每资产最新成功合并，不重复计数：404唯一新包638模块，393包由官方1.9、11包由官方1.10。1446项实际fresh.bin/prepared.hlsl/raw SPIR-V全SHA/length绑定，并由pipeline owner独立逐文件再核0差异（pipeline/merged-audit-input-review.json）。完整原metadata生产者另从原404/638重反射与stored字段对照全相等，不注stored字段来制造通过。

新源码产物对原作：210/404完整业务schema相同；302/638完整公开反射相同；617/638 capability相同；100/638非debug opcode histogram相同。源依赖集合213相同/191不同。计数属于不同范围，不能相加或当算法完成率；histogram相等也不证明完整控制流、数值或GPU等价。

全部170个compute原define tuple未独立证明；38个非零键明确使用空定义baseline，不冒充原variant。132个零键仅与empty tuple相容，XOR hash存在重复相消，不能反演唯一原输入。234个raster tuple已由原函数证实，但不少恢复源未正确响应depth宏，因此继续记录原source/schema残差。

官方DXC当前不支持作者公开fork的RawBufferLoad2；真实probe有明确no template失败。未知格式storage读写cap55/56与sampled-only不加该cap的目标在当前official版probe成立，但实现已换默认添加+SPIRV-Tools修剪，未证明原fork等价。此fork确存在不证明原构建采用何commit。

GPU实测是当前新编译fill_buffer→新反射/新wire→runtime布局→Intel UHD Graphics630派发→64字回读3221344269；egui颜色/深度均创建真实布局并验证失败注册保持旧布局，没有graphics draw或原完整游戏运行。

复验命令：cargo test --locked --test shader_build_pipeline --test shader_compiler_contract --test shader_dxc_recompile --test shader_rebuild_consumer；完整日志为../tests-current.log，Clippy ../clippy-current.log，GPU ../fresh-consumer-gpu-current.log。全量driver examples/shader_recompile_audit.rs第二参数必须fresh目录；第三参数可指定旧失败report或逗号文件名前缀，只重验受影响范围。merge_audits.mjs生成带实际输入hash的最终审计。

后续具体缺口：原compute定义与asset-build/config输入；剩余shader算法/IO/UBO/local-size/capability/依赖残差；原DXC fork使用版本及能力等价；原异常/取消/清理/缓存行为。资产库状态机和整帧不在本次范围。