# 梯位必要叶合同独立复核

复核人 core_business_lead，2026-10-21。已独立阅读完整598B主函数、603B闭包及WallAnchor两个短叶；合同允许进入候选，不代表完整stairs请求/缓存闭包完成。

原1407d903c/JBE保unordered选择，9041..9089的两次Wall判别与degree==1决定.28；90d7的方向是from.xz-to.xz，平方和先z²后x²，无epsilon；9116..9166两次junction短路与.3保留；917d..91e3的完整Vec3加减不得只改xz。原WallAnchor构造140959e80只传递WallId u64和角度，不得解释为Vec2；try_as_wall仅tag2借用+8。603B闭包要求实际petgraph邻接顺序、原edge存在检查、忽略当前规范化边、首个命中map立即按first/last kind决定，不得any化。

canonical请求+198 map与packed u64 identity必须复用真实owner；SubSegment byte1由并行compute证明为业务flag，不能当padding。smallvec原exact依赖由root统一添加。候选应含合同所列真实图/SmallVec/零水平NaN/WallId高位测试，完整前置失败顺序；无Cargo、target运行或C版本等价结论。
