# 实际 DXC 编译与全量审计互审

审查人：shader_pipeline。审查对象：shader_compiler.rs、tests/shader_dxc_recompile.rs、examples/shader_recompile_audit.rs 与 dxc/real-tests.log。

真实进程测试2/2通过：fresh输出不存在时运行安装的官方DXC，成功输出经rspirv验证entry/model；非法HLSL测试保留非零退出与stderr；预存在的输出在运行前拒绝并原样保留，故旧.spv不会掩盖失败。现测试只证明候选process合同，不证明原发行的错误类型/临时目录/失败清理与缓存完全等价。原DXC版本未知，当前1.9.2609与原代码生成不能直接等同。

构建driver的producer只接恢复源码/defines/stages与原公共依赖，原metadata只在生成新包之后用于独立比对，没有反射注入。每包写fresh.bin后再typed decode验证；逐stage接口、capability与非debug opcode histogram都保留差异。histogram相等仅为有限观察，不是完整SPIR-V语义或GPU等价。summary明确original_full_shader_semantics_verified=false。

首个全404/638报告 corpus-final-v2 产出387包615模块，17失败；实际DXC含未打包重试623成功。8错误是重建owned-custom helper相对include缺../（已修，不扩大provider搜索）；4是新SPIR-V set0/binding14同时绑定VS StorageBuffer instance_indirection_buffer与PS UniformBuffer color_library，raw OpDecorate/OpName独立核查见cross-stage-binding-conflicts-raw.json，原color_library在binding21。2是重建velocity源码重复Affine3Packed定义。3为实际DXC access violation。不得用忽略冲突、旧storedmeta或旧.spv修饰这些失败。

compute170资产中38个原definehash非零，现empty定义只属baseline；我要求driver逐行variant_config_recovered或baseline_unknown_defines明确标记，compiler owner已确认并调查原调用宏。仅raster SHADER_VARIANT_DEPTH_ONLY=0/1具有原caller与已发行key证据；不少旧恢复HLSL未响应此真宏，仍属source残差。

此前phase的OpString命令假定已在真实638测试中纠正：原命令为OpModuleProcessed opcode330。现parser从debug_module_processed限定该opcode，假OpSource/OpString与重复330反例拒绝，不保留旧OpString归属。

最终复审：v2的387成功包 + v3源码修复后6个包 + v4官方DXC1.10差分成功11个包，去重得到404新容器638stage。独立读取每个新包prefix确认SPIR-V magic与generator15全合法，证据fresh-package-independent-audit.json。393包用1.9.2609，11包用1.10.2605.37；原版本仍未知。canonical metadata210/404一致，公开stage接口302/638、capability617/638、非debug opcode histogram100/638；其余差异原样保留，绝非404算法还原完成。新driver逐行标明未知compute定义、原依赖与实际依赖集合差异；compute零hash仅与empty baseline相容，不能唯一反推原tuple。当前4/4 build test通过，pipeline/current-tests.log。

对最终../dxc/merged-audit.json再次正式复审通过：404unique/638newstage、全部计数与独立v2-v3-v4合并一致。1446项真实prepared/source-SPIRV/fresh.bin全部再次读取，SHA256和length均0差异，证据merged-audit-input-review.json。源依赖集合最终213相同/191不同。170compute tuple未独立证明，38nonzero严格unknown；234raster tuple原证据已知但新HLSL算法仍有差异。privatecompile异常/错误/cleanup/fork原版本没有冒认为原策略，未发现本轮合并误计。

最后冻结后复验：根../tests-current.log共10/10（pipeline4、compiler3、actualDXC2、wireorder1），../clippy-current.log严格all-targets通过，../fresh-consumer-gpu-current.log真实fill64words与两个layout再验通过。validation.json刷新当前source/Cargo/测试日志/新包哈希；无待验项。完整原shader算法、compute宏和原compiler私有异常策略残差仍保留。
