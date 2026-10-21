# floor_tri_count_to_atlas_idx 原内联完整合同

- 源限定stairs_assemble.rs已存在同名恒0壳，不新增生产文件。原游戏inline位于gen_stair_floors1407dc170完整9864B中的1407dd7f8..dd83c与zero分支1407dd930..944，helpers-native-pack已有全原证。
- input是triangle_count:u32（原先TrimeshChunk.index_count:u16/3得到，完整triangle count最多64）。不是原index_count，不能重复除3。
- `n = triangle_count.saturating_sub(1)`；若n=0→0；否则 `floor(log2(n)).saturating_sub(1)`。无float/logf/ceil误差。
- 原ASM dd7ff..dd80c用magic u32 indexcount/3后减1，dd814 cmovb0；dd81f BSR n=k，dd822 XOR31→31-k，dd827 EDX30，dd82c 30-(31-k)=k-1，dd82e borrow保ECX0否则赋k-1。n=0分支dd930 EAX32使30-32borrow，仍返回0。
- 原140833250共享ICF只返回ECX（MeshAtlasPart::new/DecoratorRank等名字只是alias）；14082bd60把桶写NaniInstanceMeshDesc::AtlasPart，主实际GPU subset=StairsFloor(26)。因此不能将此结果当DecoratorRank实体rank，也不能维持恒0令>4tri块进错bucket。
- 完整域映射：0..4→0；5..8→1；9..16→2；17..32→3；33..64→4。现代u32输入保相同整数公式，u32max→30（native实际u16/3域不要求该输入但安全自然延伸）。
- 测试覆盖全部0..64以原5区间表、边界65/129、u32max；不以函数数/当前GPU程序可编译声称主mesh完成。
