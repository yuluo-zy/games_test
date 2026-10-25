# 几何Y关联顺序勘误（优先于COLLIDER_GEOMETRY_CONTRACT.md）

独立Water复核原141fe60a8/60ad、60de/60e2、6111纠正Ghidra导航表达式：
- 第0端base_y=p0.y+(-.375)；第1端base_y=(-.375)+p1.y（原顺序）。
- 两端right_y=dy+base_y，不是(dy+p.y)+(-.375)。left_y=base_y-dy。
- 底部右Y由literal(-.5)+right_y（141fe614f），上右Y由right_y+.5（6174）；左底left_y+(-.5)，左上left_y+.5。
旧合同右Y公式错用了C导航的重关联；本条以原ASM为准，不修改历史合同/hash。
几何向量类型和实际输出/zip/stride逻辑不变。算术NaN的不同ISA payload/sign不得拿来冒原字节一致；这里只固定原业务选择和运算关联，不给未证跨后端bit-equality承诺。
