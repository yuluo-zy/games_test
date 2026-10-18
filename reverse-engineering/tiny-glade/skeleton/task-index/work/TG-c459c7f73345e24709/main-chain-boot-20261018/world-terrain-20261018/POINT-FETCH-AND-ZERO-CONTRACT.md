# W3 point16B物理读取 / padding / 零笔画精确结论

## 物理地址合法与数值已定义是两件事

当前候选为每笔真正分配 `min((len+1)&!1,1024)*8` 字节，复制前min(len,1024)个世界点的 **XZ**，保留完整实际DynamicConstantsChunk范围。原分配点 `1408c75ee..766b` 与复制 `1408c76dd/76e3`，原CPU代码未写odd尾部额外8B。本候选不替原值猜0，也不把point_count增加为round-even。

因此明确结论是：尾部的**物理范围、owner与同步是有效且已声明的**，但未使用的8B具体位值仍未规定，不能说整个16B都是已初始化的业务数据。Rust端只创建有意义XZ的Vec<u8>，write_bytes只读取这段已初始化CPU Vec，不把未初始化的Rust对象/Vec内容交给驱动。额外padding位于实际GPU Buffer页中，不存在CPU读取未初始化Rust值的行为。

## 原发行SPIR-V具体使用

本目录original-terrain-stroke.spv来自原发行.bin内完整102120B SPIR-V（不是本地corpus重编译），其反汇编是original-terrain-stroke.spvasm：

- `%412`@8293：count1分支仍`OpLoad %v4uint ... Aligned8`读16B，但随后`%414=VectorShuffle ...0,1`，只让第一点XZ进入距离运算。
- `%447`@8362：parallel粗距离每lane在`idx+1<n`门内读两个有效点；每lane+=64。
- `%497`@8476：fine单点同样只取0,1。
- `%522`@8528：fine多点初始读points0、1，n>=2保证都有效。
- `%570/%592`@8617/8662：fine4点loop的`i+4<=n`门保证两次pair都有效。
- `%714`@8904：fine2点loop的`i+2<=n`门保证pair有效。
- `%790`@9056：fine尾loop的`i+1<=n`允许odd最后一项读lastpoint+padding；后面`%791`取**前一pair**的zw（合法的倒数第二点），`%792`只取新pair的xy（合法末点）。新pair的zw只进入回边phi `%762`，下次条件已false、到`%770`退出，不参与距离/法线结果。单点与最后odd尾的padding不影响结果。

本目录audit-point-fetch-ranges.py逐一核上述原opcode anchor，并穷举n=1..1024的实际索引门，证明每次16B物理读取均在round-even分配范围内；结果point-fetch-range-audit.json。该脚本仅原索引/数据流的有界审核，未执行完整原shader，不冒称GPU验证。

## 当前host是否必须新填padding

目前没有该必要性的源证：当前rhapsody `rg/resource_contracts.rs`对ImportUnowned Buffer建立Whole可读外部内容状态；`resource/range_state.rs`只管物理同步/owner，不记录逐byteCPU初始化值。新metadata精确range不会把尾部解释成另一个点，也不建立Rust引用读取尾部；整个范围进入同一真实Buffer owner和访问同步。此时为静态合同新添zero write没有必要。

若主任务真实GPU验证得到与该padding直接相关的host拒绝，再单列current-host适配并审查，不能先把未证padding值写成原游戏业务。当前结论不声称padding数值已定义，也不据此扩大图初始化保证。

## 零笔画仍执行四个pass

- 同owner rasterized初值false，事件两个buffer都空只在rasterized==true时早退；第一次空世界不早退。
- infos原精确allocate(count*16)=allocate0仍有真实page/address/owner，不替换address0/max16；没有point chunk。
- 原shader外层 `%325=ULessThan(0,stroke_count)`直接到`%326`，infos `%348`以及所有point loads均在`%327`body内，故count0完全不解引用BDA。候选不附零长度read metadata。
- count0仍创建“Rasterize Terrain Stroke”→“Blur Terrain Stroke”(float4,rgba32f,horizontal)→“Blur Terrain Stroke”(float,r16,vertical)→“Terrain Derivative”四个真实pass，声明后写同owner rasterized=true。原shader在外层loop后仍执行地形基底和最终ImageWrite（12875/12878/12883/12886），不能把count0优化成整个W3无操作。
- 候选及原marker时点不等价GPU完成。真正图编译/资源producer/提交成功仍由主任务验证。
