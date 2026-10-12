# 三版本 PDB 实际能力审计

本轮仅原始元数据审计，未修改业务 Rust、共享 skill、AGENTS，未运行游戏或业务验证。此前业务实现已停止。证据为同目录 A/B/C-pdb-capability.json；三个 PDB 实际 SHA-256 均重新计算，与 tracking/baselines.json 完全一致。LLVM 工具 /opt/homebrew/opt/llvm/bin/llvm-pdbutil，版本22.1.2，能直接读取全部三版，不需要 Windows DIA。

## 量化结论

三个 PDB 都有 Types、IDs、DebugInfo，不能称“没有类型/调试”。但 **Rust 游戏编译单元没有参数和局部变量记录，建筑函数类型是 void() 占位**；完整变量调试信息主要存在于第三方 C/C++ 模块。

| 版 | DBI模块全部 / Rust rcgu | TPI记录 | IPI记录 | Rust GPROC / LPROC | Rust INLINESITE | Rust LINES子节 | 校验记录 / 独立源码路径 |
|---|---:|---:|---:|---:|---:|---:|---:|
| A | 4709 / 4560 | 6133 | 119594 | 54420 / 20878 | 2333877 | 75298 | 154584 / 5415 |
| B | 4894 / 4739 | 6125 | 132599 | 61531 / 22168 | 3204897 | 83699 | 170253 / 5951 |
| C | 5142 / 4986 | 6126 | 144737 | 67602 / 24779 | 3966178 | 92381 | 188570 / 6533 |

统计基于全 DBI `--sym-stats`，Rust 集合按 .rcgu.o 编译单元识别，另单独统计 country_core/system_wall_constructor/system_roof/system_decorator 原模块，以防依赖记录混入。Rust INLINESITE_END与INLINESITE数量相同。LINES列是 DEBUG_S_LINES 子节数，不是源码行数；校验记录含每个编译单元重复出现的文件，不是独立文件数。building_game_modules 是定义模块集合，**其中仍含依赖泛型实例和内联库代码，不能全部算游戏业务**。

|记录/数据|A Rust|B Rust|C Rust|解释|
|---|---:|---:|---:|---|
|S_LOCAL|0|0|0|无名称+类型+param标志|
|S_REGREL32|0|0|0|无变量寄存器相对位置|
|S_BPREL32|0|0|0|无变量栈帧相对位置|
|全部 S_DEFRANGE*|0|0|0|无变量有效范围/位置迁移|
|S_LABEL32|0|0|140566|C新标签有利控制流命名，不能当变量|
|DEBUG_S_FILECHKSMS|4560|4738|4985|大量源文件SHA-256与第三方MD5|
|DEBUG_S_INLINEELINES|4457|4612|4903|内联ID→原源码文件/起始行|

全PDB实际 S_LOCAL =102459/101046/101182，S_REGREL32都是37，S_DEFRANGE_REGISTER=58988/57922/58184。这些数与Rust集合的零并存。不能扫描整PDB看到variables就宣布游戏可导入变量，也不能只扫描Rust再宣布全PDB缺变量。

## 真实建筑样本

采样 construct_fences/mod.rs、construct_walls.rs、create_roof.rs，A模块370/371/3196，B398/394/3467，C406/403/3778。真正 process_segment owner另选 A1246/B1454/C1505，完整原函数记录均为：

- `system_wall_constructor::construct_fences::process_segment::process_segment<...construct_fences::closure_env$0>`；S_GPROC32程序与源码行/内联树都有。
- TPI引用全为 `0x1001 (void ())`，debug_start/end均0，opt debuginfo。
- S_FRAMEPROC给出帧大小 A736/B688/C768、保存寄存器56、local_fp_reg/param_fp_reg=RBP。**这些是帧属性，不是命名参数或局部变量，不证明每个栈槽类型。**
- 精确模块分别465/526/559个INLINESITE，各有其他同编译单元函数，且无 LOCAL、REGREL、BPREL、DEFRANGE。

所有采样普通建筑函数亦指向1001。该类型与依赖图实际读取：LF_PROCEDURE返回void、#args0、LF_ARGLIST1000为空、calling_conv=cdecl。不能把这个占位签名作为真实 Rust 调用合同，更不能因此认为构造函数没有输入。

重要采样陷阱：仅选“包含 process_segment.rs 的模块”先命中 A1234/B1413/C1471 的私有 utils::threshold_range 泛型实例，真实 process_segment不在其中。必须同时对齐定义函数程序记录、文件行范围和内联上下文；源码路径出现并不等于该模块定义了函数。JSON分别保留路径样本和真正owner样本。

## Types/IDs 能恢复什么

完整 TPI 三版分别6133/6125/6126条；对 country_core::、system_wall_constructor::、system_roof::、system_decorator::、PublicWallState、RoofShape 的全文命中全部0。记录中有C/C++结构、FIELDLIST、ARGLIST/PROCEDURE，**不能把这些库类型导成游戏Rust字段布局**。这不是证明游戏字段不存在，而是此调试流未提供所查游戏类型定义。

IPI确实非常丰富：LF_FUNC_ID=74867/83429/91416，LF_STRING_ID=39360/43625/47557，LF_BUILDINFO=4660/4838/5056；LF_UDT_SRC_LINE都670，LF_MFUNC_ID都35。实查 LF_STRING_ID含PublicWallState等完整Rust作用域，LF_FUNC_ID含construct_walls/construct_fences函数名，但函数type仍1001。**IPI不是Rustlayout的秘密替代流；它保留命名、作用域、内联身份和构建信息。** 泛型实例名字内嵌的 Bevy FunctionSystem/SystemParam/Query等类型名仍能恢复系统参数类别，但这是名字中类型结构的解析，不能推导值字段offset/size。

阳性对照保证工具没有隐藏 locals：A非Rust模块675 std_type_info_static.obj 实有参数this、type_info* const、flags=param、RBX/RCX defrange和RSP+48。B1647/C1743 lcms2-sys cmsxform.o 实有hTransform参数(void*)、p局部(_cmstransform_struct*)、多寄存器有效范围。相同工具能读出名称、类型和位置，Rust的零不是简单格式输出故障。

## 推荐利用顺序与迁移边界

1. 建统一索引：PDB身份→compiland→procedure(section,offset,size,frame)→源文件checksum/行表→inlinesite(parent,inlinee,code-range,line-delta)→IPI作用域与函数ID。现有PublicName索引要升级为此关系图。保留physical地址与inline logical owner，避免folded aliases误认。
2. 先用整条建筑链的行号/内联范围给二进制代码分区，标依赖frame，再恢复caller ABI与实际数据load/store。将“编译器frame属性”“名字嵌入类型结构”“机器指令字段布局”分别保存，不混为TPI字段。
3. A/B同checksum模块可迁移行身份和业务候选；C可迁移未变源码行范围及泛型类别，但新增木构必须独立。**三版皆无Rustlocals/layout来源，不能从某丰富版自动补齐另版参数/变量。** 类型/局部字段仍由实际binary数据流恢复并注明证据；优化导致变量消失不能补造原命名。
4. 依赖C/C++ TPI可精准导入已有库边界帮助参数解释，仍复用原crate。不得因PDB更详细就重新实现这些包。

具体命令：`dump --sym-stats`流式汇总全module；`dump --modi=N --symbols --files --il -l`只采目标；`dump --types --type-index=0x1001 --dependents`查签名图；`dump --ids --id-index=... --dependents`查IPI身份图；`dump --type-stats`与`dump --id-stats`应分别运行（同调用同时打开时工具只显示一种stats，不应误计）。全types/ids本轮仅在进程内过滤，未整文件dump到对话或持久化大文本。

限制：没有作者源码文本、优化前IR或debuginfo2构建；DEBUG=true不能证明完整调试级别。本轮无源级调试执行，零变量结论限定实际Rust DBI记录及所查游戏TPI名称；不能声称原Rust代码没有变量。类型数据库扩展和decompiler导入应基于真有的条目，保留缺失信息而不插假locals。下游恢复计划需要以这些实际边界设计，不能许诺从现有PDB一键反编译完整Rust工程。
