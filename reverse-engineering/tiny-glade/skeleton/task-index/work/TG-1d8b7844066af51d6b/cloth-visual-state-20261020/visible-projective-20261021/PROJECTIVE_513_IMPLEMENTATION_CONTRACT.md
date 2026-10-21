# 513B同源顺序增量合同

复用peer GOTHIC_ROTATION_REQUIRED_CONTRACT_V1.md SHA d350d9cff04f3f887a9c9ea837f4359f40fa68f8c50ed10f48e73e900a61145a；原包/proof路径保持原TG73目录，proof SHA e61c6314af63ceaa945ef59221bbf5575361fcbd84b6ccb85a0c01d80ef9b8b1。仅既有utils/lib.rs添加 `pub fn xz_projective_rotation_bad(direction:Vec3)->Quat`，原B140c911d0/513B，不改607、504或其它函数。当前实际before35ddf39a已包含root集成607；不是从8011重盖。

输入只读XZ，原Y无条件忽略；原reciprocal=1/sqrt(z²+x²)，严格正有限非零时forward=(x*r,+0,z*r)，否则forward=(1,+0,+0)。不对原三维输入做finite/length检查，Y可NaN/Inf。复核原bit谓词覆盖正normal与正subnormal倒长度，排除±0、负值及非有限。

right先由原Y×forward乘减获得；原第二次规范化是每分量除sqrt(ry²+(rz²+rx²))（c91290..ad），这里显式按该组计算并除，不用先求倒长度再乘改变算序。矩阵列为(right,Y,forward)。复用pinned glam0.29.3 Quat::from_mat3/from_rotation_axes四支：m22<=0先dif10=1-right.x；其余sum10=1+right.x；与原c912bf..c913c5四块逐分支对齐，保正/负Quat符号选择，无额外normalize/clamp。253B Mat3原helper只作同族basis交叉证，不无证增加独立导出。

测试应覆盖轴/四象限实际基、XZ零/非有限/平方溢出/下溢fallback、Y非有限忽略、原四分支符号。没有原运行oracle；不宣称全NaN payload/跨ISA位等价。生产/Cargo/GPU不改，候选只在本dir。
