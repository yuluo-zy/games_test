# determine_ladder_support：完整167B合同

仅本源既有函数；现代错误四参tuple壳没有任何caller，拟改为真实typed输入，不新生产文件。

## 身份与调用
- 原入口 `0x1407de800`，PDB `437:156912`，完整167B；`helpers-native-pack/fn-7de800.{asm.txt,json}`。
- 主true `0x14000ab5b`：RCX=&fromVec3、RDX=&toVec3，R8=&Params.nodes，R9=&两个连续20B AlignKind。`0x14000ab07..1b`、`0x14000aaf1..ab03`分别复制两个alignment；回写AL在`0x14000ab60`。
- RCX/RDX只读+4高度；nodes stride32，anchor u32tag@0、Terrain/Roof height@4、degree@24；与canonical `[(StairNodeAnchor,u8);2]`同真实owner。原三anchor判别0/1/2，不造第四variant。
- R9元素stride20，仅读低端alignment.tag。它不是入参flag，更不是静态byte。

## 原计算与顺序
1. `b.y > a.y`（有序strict）时lower=0，higher=1，alignment=0；其余（包含相等/任一NaN）lower=1、higher=0、alignment=1。原SETBE在unordered也置1，与本分支一致。
2. lower_valid默认true；lower.degree==1时：Terrain/Roof要求height<=threshold（有序；NaN false）；Wall要求对应低端alignment.tag==0。
3. higher_valid默认true；higher.degree==1且anchor Terrain/Roof时要求height<=threshold；Wall保持true，不读alignment。
4. lower_valid=false时返回 `4-u8::from(higher_valid)`（both false 4，only lower false 3）。
5. 两者true返回0；lower true/higher false时：lower若Terrain/Roof且height<=threshold返回1，其余返回2。**最后这个lower height门不受lower.degree限制**，不得复用“degree非1直接true”的判定。
6. threshold原VA `0x142a877d4`，RNE原4B `b8 1e 05 3e`，bits0x3e051eb8，值0.12999999523162842。使用from_bits保原常量。

## 拟现代API与验收
`determine_ladder_support(from:Vec3,to:Vec3,nodes:&[(StairNodeAnchor,u8);2],alignments:&[AlignKind;2])->u8`。
- 原完整函数无heap/无副作用/无panic；typed两个固定数组保证边界。所用inputs是未来完整main实际owner，不建镜像node。
- 测试覆盖5个返回码、两高度方向、同高、NaN端点高度（else路径）、threshold equality/nextup/NaN、degree非1与最后回查height、低端Wall tag门与高端Wall无门。
- 只构建此已完整锁定的必要leaf，不替代两个62KB主producer/构建梯/当前CPU/GPU验收。root唯一执行Cargo，未运行不能写passed。
