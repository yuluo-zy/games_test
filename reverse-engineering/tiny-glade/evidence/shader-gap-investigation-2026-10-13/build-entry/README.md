# 原构建入口与可证实缺口

当前已实现runtime shader读入合同和显式source recipe隔离，未取回作者offline工具文本。

## 原发行与PDB边界

原manifest1792条记录，仅tiny-glade.exe与6 DLL、runtime PDB；无独立asset-pipe-bin/DXC，也无.rs/.hlsl/.log作者源码或构建日志。当前目录2080文件，其中288额外不能当原发行记录。原build-info只保留Cargo锁、integrity manifest、Rust工具链信息：Rust1.86/LLVM19.1.7，hostLinux、targetWindowsMSVC。这个host不证明外部DXC实际路径/操作系统。详见release-build-inventory.json。

本轮真实LLVM -files覆盖4894module/169720file records；shader/compiler相关50记录只覆盖runtime shader-pipeline/variants等，shader-compiler与asset-pipe-bin作者源码0。public symbol限定查询两namespace亦0；TPI6125条已有全文没有这两个作者类型。局限是当前runtime PDB，不是所有开发工具的存在性证明。原Cargo却确实保留asset-pipe-bin→shader-compiler→shader-prepper/spirq/rspirv/speedy/tempfile依赖：包存在不能推出已随发行带CLI，tempfile依赖不能推出命名/错误/清理策略。

## 已恢复原runtime load_compiled

Compute0x1409e3c50/Raster0x1409e4160各847B，按原PE exception范围准备并逐body SHA绑定。路径生产→std::fs::read→已恢复Speedy解码，没有这些body内的离线编译/缓存/temp步骤。原两个错误分别Failed opening compiled shader pipeline from与Failed deserializing compiled shader pipeline from，path用Debug、底层stdIO/Speedy错误用Display，anyhow::Error::msg保存展开文本。原字面量指针/调用详见loader-error-format-original.json。

现compiled.rs添加CompiledShaderPipeline::load_compiled(path)，复用std文件读取与现typed decoder。返回String为当前接口适配，原anyhow对象和RustABI未声称相同。新测试公共rspirv builder+原SPIR-Q生成新包，检查missing、成功含尾字节、同一文件改截断再读的分类；不是用旧bin注入元数据。

## artifact配置与source recipe

build_pipeline签名保持不变，委托build_pipeline_with_recipe(empty)。新函数只把显式recipe宏追加到prepared输入；返回artifact_defines与recipe_defines两集合，spec不改，原文件key只按artifact定义。TGV不冒认为原PERM配置。实际DXC fixture把LocalSize1切为8，原spec与hash不变；所有shader_build_pipeline测试输出改系统tmp或TG_SHADER_TEST_EVIDENCE_DIR，历史shader-build包/日志不覆写。

原compute38非零keys已由compiler owner恢复8宏并核原producer与布尔来源；我逐条对38asm文件12750instruction records与RNE字节一致、38原资产SHA一致，gauss值由格式模板两段双引号字符（各1byte）包住Display输入，并非Debug转义。actualShaderDefines/hash/path测试覆盖38；0-key132tuple未在此证明。详compute-peer-review.json。旧38empty baseline报告保持历史状态。

## 同伴互审与源缺口

两个VS输入修复真实DXC probe独立读取OpName/Location/Variable/Type全部等原，保留garden空洞0/2/3，不能靠改metadata或删活跃输入。审69源码补丁发现同名SPIRV_Cross_Input误套PS：34块28source属于frag_main，其中11块有明确FragCoord。已阻断并交runtime修；当前34块均恢复为vertex补丁前的PS输入结构，28source逐块复核通过，历史blocker保留并由runtime-patch-final-peer-review.json登记闭合。

新native-recipes全量385成功/19失败，失败全cross-stage UniformBlock。两例直接用exact原SPI-Q库独立重读original/fresh证明源差异：cloud48B位置类型同但VS _Globals_前缀不同；solid_vertex_color原84B，newVS92B且flags错移80/84、shadows88，newPS原84B offsets但前缀。bool_render_shadows原就有bool_，matrix反射双方RowMajor/stride16；不能凭HLSLcolumn_major字样改语义，也不能放宽producer冲突gate。原19失败包最终38阶段的成员名字、类型、偏移、块size全部经独立公共SPIR-Q复验等原；root仅重排solid_vertex_color三字段，使VS92B恢复原84B，未放宽producer gate。119source/796identifier前缀改名有逐声明原name/set/binding/offset证据。详ubo-final-independent-review.json与globals-prefix-peer-review.json。

公开候选kajiya/rendertoy保存源码与原5个pipeline authorfile checksum（含CRLF变体）无匹配；所查源中未找到__alias_/in.var.规范化来源。只排除这些具体完整文本同一，不证明未借用片段或没有别的公开上游。exact shader-prepper原发布checksum已独立证明，直接复用；候选旧provider不同API与工具链不能当原B。

## worker与验收

worker shader-build-gap-1013，Windows专属project，B patched身份已核；仅prepare2body，未改原型/未同步seed/Mac overlays。SDK checkpoint后正常--stop，worker-stop.log。工具启动一度APPDATA java_home.save拒绝，已有正常成功会话与失败日志分别保留，工具调用不计业务交付。

根publish当前21tests、strictall-targets Clippy、新fresh GPU均passed。4个当前源码root与publish exact。远端main已self-contained，测试改为direct helper验证；已验publish业务体不变的中文注释修订已同步root，独立记录当前hash，不重跑注释。最后sourceHLSL/pack全量修复回验由本轮最终报告登记；不声称全部shader算法或offlinecompiler error/cache/temp已等原。

## 最终本轮验收

corpus-final-native生成404新包/638新SPIR-V，预处理与打包失败0；typed metadata397/404相等，public完整接口313/638相等、capability集合300/638相等、非debugopcode直方图88/638相等。这些统计不证明完整着色算法等价。剩余7个typed schema差异：final_post depth缺原资源块，其余三个family两variant有活跃vertex输入差异，详final-schema-residuals.json；不得删输入或注入原metadata伪造相等。132个compute zero-key原tuple还无producer独证。最终报告与源码hash绑定见validation.json；历史385/19中间报告与以前emptybaseline编译报告保持原记录。
