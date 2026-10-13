# shader 构建流水线闭环

现工程入口 src/shader_compiler/build.rs：source_root + logical_root + source_path + defines + stage请求进入，返回prepared source、实际依赖、freshSPIR-V、全typed ShaderReflection与可解码Speedy容器。生产者不接受原stored reflection。

预处理直接复用原shader-prepper0.3.0-pre.3：它做include扫描/pragma once和source chunks，保留#define/#if给DXC，并非全C预处理器。游戏侧provider用原relative-path1.9.0处理文件路径；当前相对文件优先、根目录回退为候选适配，因为原provider私有实现未在发行中定位。宏文本按输入顺序写入source；#line将chunk.line_offset+1与逻辑文件名映射回源。依赖集来自真实get_include读取，不从原bin注入。

variants原0x1417388a0克隆整个描述，再分别追加SHADER_VARIANT_DEPTH_ONLY=0/1。0x1420cc210是公共Vec clone，故不臆造已有同名宏过滤/覆盖；当前raster_variants按此clone+append保留base defines与其它字段。egui实际源修复由compiler owner完成。

反射直接调用原spirq公开fork4f22e6f：ref_all_rscs(true)、combine_img_samplers(false)、gen_unique_names(false)。自有policy为去descriptor的__alias_前缀、vertex输入in.var.前缀、跨stage资源合并、$Globals UBO成员字节偏移/尺寸与namehash、local size及push size。读public实际类型生成原wire Type，不读原metadata作为生产参数。输出map顺序为了新打包确定排序；原HashMap迭代顺序不作为业务目标。

pack保留OpString文件路径、OpLine、OpModuleProcessed命令；OpSource/OpSourceContinued正文只保留LF。原全638模块14492Source记录LF-only见根original-debug-payloads.json。header调用原rspirv0.11 ModuleHeader::new(bound)并保留SPIR-V version，得到公共库默认generator0x000f0000。loader本身保留fresh DXC generator14，所以这一步是复用公共header构造的调用策略，不是编译器版本清零或重写assembler。bound与version由fresh模块产生，未声称旧原所有instruction IDs/codegen一样。

实际首轮cargo test --locked --test shader_build_pipeline 3/3通过：404bin/638原stage payload独立再反射，对原stored完整canonical字段零差异；原include库fixture（pragma once、宏条件、依赖和缺include）；fresh fill、egui color/depth实际prepared→DXC→后处理→反射→Speedy→decode完全无原metadata注入，且其接口metadata与对应原bin一致。实际fresh-fill.bin/fresh-egui-color.bin/fresh-egui-depth.bin已写，供独立GPUconsumer测试。日志first-tests.log。

完整HLSL语义尚不能由接口metadata一致证明：egui VS先前仍有整数操作差异；其它shader重建源、私有include/helper和变体宏覆盖由全404/638 audit逐项保留，不以三个fresh样本或完整meta再生产声称全游戏等价。全量audit由compiler owner接现build API运行，结果再登记。

错误/副作用缺口：原compile pipeline函数未在B发行PDB定位，当前String错误、宏输入校验、跨stage冲突/名称碰撞、temp工作目录保留和cache流程都是限定候选；不把合理错误策略说成原作事实。公共shader-prepper/spirq/rspirv/speedy/DefaultHasher实现直接复用，只有自有调用policy恢复。shader-compiler/shader-pipeline整体公开来源仍未知；不能从local lock分类推断原作者全自研。

全量首轮 corpus-final-v2真实尝试404/638，387新包615模块已生成；17错误不计完成，其中8个是重建owned-custom helper的3个相对include缺上一级目录，已按真实nani源树修复，provider不做任意祖先搜索。4个是fresh SPIR-V binding14撞资源，独立raw名称/Decorate证据另存，禁止为通过pack覆盖资源；2个重定义、3个DXC内部异常由compiler owner跟进。170compute中38个原定义hash非零但宏尚未知，当前empty仅baseline。最后重跑统计另见validation.json，v2数据不代表全部源码已经恢复。

独立consumer已在Intel UHD630消费fresh fill：64word均为3221344269，两个fresh egui变体创建真实layout并验证失败注册保留旧layout。仅fill执行计算，未进行raster draw；证据../fresh-consumer-gpu.log与../provenance/pipeline-producer-review.json。

路径adapter具体边界：resolve_path先trim起始/再按父目录相对路径优先；若parent/inc和root/inc均存在，/inc不会强制根目录。这个行为未获原provider证据，不能宣称完整根路径/找不到文件语义等价。

独立wire-order audit量化404原包再生成的metadata：27包wire exact，377仅map存储顺序不同，typed业务metadata与长度一致；新生产者排序不等于原HashMap的随机迭代。这符合用户业务合同目标，不把canonical equality伪称全bin字节一致。证据../provenance/wire-order-audit.json。

最终fresh构建artifact闭环：v2success387、v3sourcefixsuccess6、v4officialDXC1.10success11，独立读取全部404新包共638stage，SPIR-V magic和generator15全合法。393包1.9.2609，11包1.10.2605.37；新编译metadata210/404与原一致，stage公开接口302/638、capabilities617/638、非debug opcode histogram100/638一致，其余差异保留（即使hist相同也不证明完整语义）。根current-tests.log本轮4/4真实通过。见fresh-package-independent-audit.json和validation.json。
