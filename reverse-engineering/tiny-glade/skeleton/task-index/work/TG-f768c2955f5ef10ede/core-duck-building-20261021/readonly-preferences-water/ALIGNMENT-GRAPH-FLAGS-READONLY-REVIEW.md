# alignment graph flags 有界独立只读复核

复核对象为duck的ALIGNMENT_GRAPH_FLAGS_CONTRACT.md，不是其尚未写出的候选。原true/false完整请求pack均复用，同RNE f507a6ce/PDB2d9686b8身份，无新数据库查询/源码/Cargo/claim变更。此结果不替代完整request图生命周期的owner/独审/运行验收。

## 结论：该合同的旗位传播规则与所读原指令相符

- true 14027f271加载XMM10=每lane80000000符号mask、f279加载XMM6=1.0、f280令XMM7=ZERO，f283令R14D=1。f3c6从kind+4读first，f3cb从+0c读second，shuffle/mul/sub得到cross=first.x*second.y-first.y*second.x。
- f3df..3e6对cross保符号位并OR1.0，得到±1；f3e9 cmpord ZERO,cross屏蔽NaN；f3f1比较0<该±1；f3f5再and ordered mask。因此是cross.signum()>0，**+0归positive、-0归nonpositive、任何NaN归nonpositive**。不可以用cross>0、不可以用>=0或total_cmp。负NaN符号如何都被ordered mask清掉，不依赖NaN payload。
- f3fc..40a将positive累积到R13、nonpositive到R15，只置true不清已有值。tag0由f3a0设置has_straight；tag1触发上述分类；其他tag由f3c1/c4跳过。
- f410..424只有has_straight且两类turn至少一个存在才再遍历。f4e9..4f4合成flags=(nonpositive&1)|((positive<<1)&2)。第二遍只收集tag<=1，每条12B(node,neighbor,flags)，顺序跟随真实图node/neighbor次序；不得按decorator ID另排序。
- 收集图的所有更新后才从f5a9执行apply。required边缺失原跳140282810，越界原282821；不静默插边。f68a/68d仅tag2跳过，tag1经f69b/69e→f5dd只写flag1，byte2保留；其余tag经f6a4..b4写flag=flags&1、byte2=(flags>>1)&1。first/second/tag均未改写。

## false实例已额外配对

原false完整pack中140277c51..c60对应符号mask/1.0/ZERO初始化；140277da6..dbb对应相同cross；dc2/dc6保符号构造±1、dc9 ordered、dd1 strictpositive、dd5 and。dde2累积positive到R15，ddea累积nonpositive到R14，寄存器与true不同但规则相同。

false写阶段14027806a/06d跳tag2；07b/07e→140277fbd仅tag1的flag1；084..094同样从临时flags写其余tag的flag/byte2。14027809d开始释放该临时Vec，与true6bd对应。此配对只支持本flags块，不据此推true/false全请求同码；dirty旧集收集仍有已知不同。

## 建议针对性验证

用实际DiGraphMap，tag0+tag1各造first/second使cross为+0或-0（例如(-0,+0)与(+0,1)给-0），不要在fixture预先把signed zero归一化；加NaN/Inf×0的nonpositive路径。tag1 byte2原值应保留，而tag0的两flag应覆盖旧值。没有straight或没有turn应保持图原样。未知tag收集跳过与apply防御分派分开验证。所有更新应先收集后写，保缺失边panic，不在此另造资源map。
