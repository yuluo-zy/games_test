# Shader工具链来源与复用边界

这套链不是一个从零自研的HLSL编译器：底层是公开DXC、SPIR-V操作库、反射库和序列化库，外面是项目内的配置、命名、元数据schema和打包逻辑。后五个local包的公开来源仍未核实；没有找到公开仓库不能证明它们完全自研。

| 包 | 原作身份 | 已证实来源 | 处理 |
|---|---|---|---|
| DXC | 不在Cargo中，以外部命令被调用；原具体版本未知 | [Microsoft公开LLVM/Clang派生编译器](https://github.com/microsoft/DirectXShaderCompiler)；作者另有两个publicpatch，但原构建是否使用未证 | 复用公开编译器；仅针对实际要求的fork行为做适配 |
| shader-prepper | 0.3.0-pre.3；原checksum匹配下载crate | [h3r2tic本人原生公开MIT库](https://github.com/h3r2tic/shader-prepper)，不是fork；原发布vcs845491a0 | 直接复用include crawler；游戏provider、chunk连接/#line/宏参数规则另恢复 |
| spirq | 0.6.2，原锁绑定4f22e6f | [PENGUINLIONG的库及作者公开fork](https://github.com/h3r2tic/spirq-rs)；修OpUndef造成定义扫描提前退出 | 精确复用原commit，不换master |
| rspirv | 0.11.0+1.5.4；原checksum匹配本机crate | [gfx-rs公开库](https://github.com/gfx-rs/rspirv) | 直接复用parser/assembler；游戏后处理策略另查 |
| speedy | 0.8.6；原checksum匹配本机crate | [koute公开库](https://github.com/koute/speedy) | 复用codec；游戏CompiledShaderPipeline/Reflection字段是独立合同 |
| shader-pipeline | 0.1.0 local；B PDB有5个项目源文件与原checksum | 项目内shader格式/schema，公开上游和作者文本未知 | 恢复原schema/keys及消费者，不重写codec |
| shader-compiler | 0.1.0 local；依赖链及638嵌入请求 | 搜索的B runtime PDB未列出作者源码，公开上游未知 | 恢复编排/参数/provider/后处理；不是重写DXC |
| shader-variants | 0.1.0 local；B PDB原lib.rs checksum | 项目内变体定义，公开上游未知 | 恢复实际宏/变体策略 |
| asset-pipe | 0.1.0 local；B PDB lib/texture原checksum | 项目内资产包装，公开上游未知 | 游戏格式和组合策略需恢复，图片/压缩库复用 |
| asset-pipe-bin | 0.1.0 local工具CLI | 原锁已确认依赖，搜索的runtime PDB无作者源路径，公开上游未知 | 按发行构建影响恢复CLI策略，不把它视为通用shader编译器 |

shader-prepper只扫描和展开include，完整条件/宏语义仍由后续DXC处理。其原发布crate SHA256与原锁完全相同；七个src文件与固定git commit源相同，只有CRLF/LF差异，未把归一化相同伪称字节hash一致。详见shader-prepper-published-source-identity.json。

## DXC公开fork与当前官方版本

作者[fd2e4ec9补丁](https://github.com/h3r2tic/DirectXShaderCompiler/commit/fd2e4ec9fb2c1b9b2e58a35de759fd6068419fb1.patch)修unknown-format storageimage capability判断；[16ceb560补丁](https://github.com/h3r2tic/DirectXShaderCompiler/commit/16ceb560954f2238dcdad73a75d5e5842e58c839.patch)增加vk::RawBufferLoad2(uint2)。这证明作者公开修改过DXC，但不能仅凭其GitHub存在就把原游戏具体编译器归为该fork。

已核对本轮官方v1.9.2609：其[intrinsic定义](https://github.com/microsoft/DirectXShaderCompiler/blob/v1.9.2609/utils/hct/gen_intrin_main.txt)仍无RawBufferLoad2，真实probe也拒绝它；因此涉及该API的源码不能直接适配为官方RawBufferLoad(u64)而冒称没有差异。现恢复源码尚无已证RawBufferLoad2调用，不能把此潜在差异当成本轮构建阻碍。

新版[capability逻辑](https://github.com/microsoft/DirectXShaderCompiler/blob/v1.9.2609/tools/clang/lib/SPIRV/CapabilityVisitor.cpp)已改为默认添加并交SPIRV-Tools修剪，采用不同实现。真实probe中sampled image只有Shader capability，unknown storage读写具有55/56，两者符合旧patch目标；不应宣称官方缺此行为。结果见../dxc/official-fork-capability-probes.json。

本轮实际构建使用两个公开官方版本：[v1.9.2609](https://github.com/microsoft/DirectXShaderCompiler/releases/tag/v1.9.2609)负责393个容器；另外11个在此版本真实崩溃的容器，改用[官方preview v1.10.2605.37](https://github.com/microsoft/DirectXShaderCompiler/releases/tag/v1.10.2605.37)，保持相同编译flags后完成构建。两个下载archive的官方SHA与实测SHA一致，安装路径/EXE及DLL身份分别保留在../dxc/toolchain.json和alternate-toolchain.json。前面的RawBufferLoad2/capability源码与probe判断专指v1.9.2609；没有把该结论未经核验套到v1.10。两个版本都不是已确认的原作DXC版本。

## rspirv后处理边界

根agent原字节审计638模块header均为0x000f0000；rspirv原版本ModuleHeader::new正好默认该值，而Loader::consume_header原样保留传入header。因而“外层通过rspirv重建module”是有根据的推断；这不是原DXC版本指纹，更不证明通用rspirv自动把全部源码清成LF。14492条原source字符串只有LF，具体作者处理函数仍未知。见rspirv-public-generator-boundary.json及../original-debug-payloads.json。

## 公开同源候选搜索的界限

逐页读取h3r2tic公开88repo和PounceLight公开列表、查询五个精确localcrate名称/0.1.0，并检查作者公开kajiya/rendertoy源码。kajiya有相似includeprovider和DXC组合，但hassle-rs、Vulkan1.2目标、schema不同；rendertoy用shaderc/spirv-reflect/snoozy，不能直接替代B的DXC/spirq/speedy链。没有任何sourcechecksum证实它们等同原local包。GitHubREST403rate limit报告保留，使用HTML/raw/git获取的成功证据单独标记。

结论和精确锁/PDB/公开源码记录在package-classification.json；公开库直接复用，local上游未知保留未知，研究的是它们组合公开依赖的业务规则。

## 三版边界

A/B PDB均出现上述八个local源路径，原checksum各自保留，不从路径相同推断所有业务等价。C则已迁移到shaderface0.1.0，原锁显式指向git+ssh://git@github.com/PounceLight/shabriri.git#2212a07a7c6aaa58f47283f6fff090a6744dfbed，PDB也有该gitcheckout中shabriri-rt与shaderface源路径。C的rspirv为0.12.0，与本轮B不相同；B三个shaderlocal包不再出现在C lock。该git源身份已知，但匿名HTTP返回404不能区分私有、删除或不可访问，更不能证明B本地包与它同源。详见C-source-transition.json与versioned-pdb-source-boundaries.json。

## 本轮独立新编译消费验证

新fill/egui-color/egui-depth三个容器经真实DXC和新反射producer打包，不注入原stored metadata。独立GPU consumer在Intel UHD630读取新fill结果：64个word均为3221344269；同时创建两种egui实际Vulkan layout（3/1 bindings、push均8字节）并验证错误注册保留旧layout。这不是原EXE运行或raster draw，也没有证明全部原shader算法等价。详见最后重跑日志../fresh-consumer-gpu-current.log与pipeline-producer-review.json。

独立wire审核保持原SPIR-V/stage及依赖不变，只重新生产metadata：404容器中27个字节完全相同，377个由于原HashMap记录顺序和新确定排序不同而改变字节；全部长度相同、descriptor/vertex有效map及名字查询业务一致，新生成容器重读成功。此结果不等于377个原字节已1:1复现，也不等于重新DXC编译404shader。原38个nonzero compute define hash的完整输入策略还未恢复。见wire-order-audit.json。

完整新producer另已使用恢复后的HLSL corpus真编译并打包404容器/638stage：首次387成功，修正include及跨stage绑定冲突后再有6个成功，官方preview处理剩余11个，剩余构建失败为0。输入源、prepared HLSL、DXC输出和packed bytes均由../dxc/merged-audit.json绑定。新旧完整schema一致210/404、stage公共反射一致302/638、capability一致617/638、非debug opcode histogram一致100/638；这些差异如实保留。170个compute原宏tuple均未独立证明，其中38个nonzero define key未恢复；234个raster tuple有原证据。这是恢复源码的功能可构建覆盖，不等于作者原源码、全部原配置或shader计算语义已经复原。

最终10项测试及all-targets Clippy -D warnings通过，最后GPU消费也通过；最终源码、三个新容器和日志SHA256见consumer-validation.json。独立另核验merged-audit绑定的1446份prepared/编译/打包产物共44781632字节，hash/长度不一致为0，见independent-artifact-bindings.json。
