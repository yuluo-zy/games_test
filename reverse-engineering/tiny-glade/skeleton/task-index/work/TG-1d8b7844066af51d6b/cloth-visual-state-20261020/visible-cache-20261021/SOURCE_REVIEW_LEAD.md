# 607B canonical方向叶源码独审

通过。候选仅增xz_rotation_from_forward与3个有意义测试，删除新增区块后原lib逐字不变，已提交bad/factor/其它modules保护。实现完全对应已批准607B合同：单次输入try_normalize-or-X，right仍单独normalize，列(-right,Y,-forward)后复用完整pinned glam Mat3→Quat，不做bad(-raw)/atan2/输出再规范化。

原完整607 ASM已直接核首normalize有效倒长度门、二次right归一化、两次signmask及四quat分支；输入失败仍先+X再翻轴。测试覆盖四象限/非单位方向/局部-Z与Y基、zero/NaN/Inf/有限平方溢出与极小平方下溢的实际fallback bit一致性，以及原Y/W选定符号，未用宽容差掩盖已有oracle失败。当前仅静态恢复与几何契约，不声称原target或跨ISA bitwise oracle。

实际before8011、candidate35dd、patch91bf均重hash；独立scratch patch -p1 --fuzz=0无offset/fuzz精确复现。未修改生产、未Cargo/rustc/GPU。root可先独立整合测试此必要叶，cache/真实gothic另一513B helper仍按各直接consumer闭合，不把叶测试算全视觉链。
