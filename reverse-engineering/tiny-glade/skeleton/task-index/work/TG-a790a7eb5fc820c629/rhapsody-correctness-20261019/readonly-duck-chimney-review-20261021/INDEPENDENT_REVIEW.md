# Chimney 三项失败：原门序独立复核

本复核者不是 derive_decorator_subtype.rs 的候选作者。按中央 agent 分工仅只读复核现生产、实际失败日志与既存原完整4576字节；未运行 Cargo、未修改生产或作者候选。

结论：本轮三项失败均在共同 fixture `chimney_world` 创建 freehand wall 后立即调用 `max_y`，还未调用 derive 系统。支持仅在该 fixture 以四个真实空 path 栅格调用 `prepare_paths`，不调整生产逻辑、断言或共享 public_wall helper。

## 原门与实值输入

原包绑定 B RNE f507a6ce…、PDB2d9686b8…，完整函数140812b20/4576B。复核直接读取原 assembly，不以候选作者报告作为唯一事实。

- 1408131a7调用 wall-storage iterator；第二遍重新借每个 wall bucket，140813367选择ty0x3b、37a/386仅接受dst<=1。邻居来自同一桶；这不是全局距离扫描，也不依赖 d2s 反查。
- 1408133b6读取对应 PublicWall，3be/3c1缺失失败；**3ca调用 required max_y**；3e2随后调用 required chimney extra；3ee读取原 WallSpace。当前源码顺序一致，不能用零高度或可选高度绕过未初始化。
- 3f8/400先计算 `max_y-y` 后 MAXSS(+0)，405/40e依原顺序加3和extra；417/41b的乘零仍保留，Inf传播的NaN不能省略。43c原u投影；483世界参考点使用max_y；488/48d才计算烟囱竖向中心。当前 span/投影/参考点/中心分离正确。
- 52f..554的destination映射：0/1及大于4的raw值归wall，2归roof，3/4不支持。现match逐支保留此门，没有用destination名字替代raw枚举域。
- wall邻居：604 required subtype →60d按真实rank构造key →629 required collision record，62e/631缺失即失败；637宽度、63d/642 top-bottom、65b乘半；672/676严格有序竖向比较，失败或NaN不算接近；688原WallSpace距离；69b/69f严格水平比较 `distance < width/2 + .425`；**6ab以后才排除同ID**。当前 mesh/key、门序和self顺序一致。
- roof邻居：572 required CachedDecoratorTransforms.position(other_id)；584..5b1按 `sqrt(dz²+(dy²+dx²))`；5bd/5c0有序严格 `<1`，NaN不成立；**5c6才排除同ID**。现源码没有默认cache或先排self。
- 6e2命中写20，邻居穷尽2d1写19；第一遍早在此之前完整写入subtype表。因此清空self mesh时仍应先发布ID1=19，随后required lookup失败；测试的失败期望不能删除。

## Fixture 生命周期与修复边界

现 `public_wall(false)` 仅 consume tag2 freehand墙；`PublicWallState::prepare_paths` 才生成该类型的 freeform_heights。`max_y` 对缺失 cache 的 expect 与原 required门契合。实际日志三项均报同一该expect；共同fixture在自身第924行读取maximum，因而没有触发生产内mesh/cache比较。

用 `PathRasters512::from_encoded` 四个512²零栅格并调用真实 `prepare_paths` 能补上本场景缺少的 producer，既不手填 maximum，也不改变 lazy 测试的未初始化输入。应仅在 chimney_world 初始化；其余 public_wall helper 使用者仍保各自生命周期。

墙测试的 strict阈值与roof .5/1.0边界不需要调整。删除邻居后重新派生回19仍应保留；self requiredmesh及缺cache的两条失败路径也应保留。本复核支持水worker的 fixture窄候选，但本次未执行修复后测试；通过与否由root实际验证。
