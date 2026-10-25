# Duck reviewer 独立复核：完整 loader26

2026-10-21；core_business_lead 授权在旧合法 work 新增本记录，live show owner 仍 clutter-startup-20261020，未改 claim、candidate 或生产，未运行 Cargo/rustc/GPU/原程序。

**结论：当前冻结 B 候选的有界静态复核未发现需要阻止整合的业务反例。** 接受范围是完整 B 原证→26分支/52 key modern loader 与真实 box 依赖的源码接线，不是 C 等价、运行通过或游戏主链完成。

## 冻结与原证

- 候选 SHA256：7eacff3ac5c9839abaa99017bb504eee6011be3e0accb406477633ea19b302d8。before ba7acd2da87e49cb34cbb7ea6eb825fcff857357dbcffe4c5bb6980c1cd33cbd。候选变化使本记录失效。
- 原完整 pack `../real-collision-producer-20261020/native-pack/report.json`：loader1407b66e0/17386B及三个closure、bbox五项status=ok，RNE f507a6ce…/PDB2d9686b8…，target_executed=false。复核读取原asm与inline/callee注释，不以作者矩阵为唯一来源。
- paired-loader-box.json要求的三项canonical schema/outline/export依赖仍属于独立owner事实；本review没有把它们的原B/C schema差异重新验收。当前decorator Cargo已显式启用 country-core/native-physics，真实geometry构造方法不是缺feature的空替代。

## 独立复核到的罕见路径与失败顺序

1. 文件阶段1407b6837..6883先长度减5/逐字节精确`.json`，load_json1407b68b8在stem UTF8切片检查1407b6943..695a之前。候选strip_suffix与先load再stem[..7]保持失败先后；不是starts_with替代。已复核无排序、无clear、所有目录项完成才进入枚举。watch→assets.add沿现真实MeshWatch/AssetMeshLibrary，未插loaded旗标。
2. 1407b776b case12先corner indexed，交互1407b77dc/7e3偏偏是flag_wall_interaction，之后corner outline1407b7809/7810；候选没有按名字猜成corner interaction。
3. 1407b7d5b case15 bbox flag_terrain collision→indexed interaction，再1407b7e01/08直接从collision本体重读Lines；不是collision_outline。case17在1407b8a97，两个indexed→.4 Sphere→1407b8b4d/54 lantern_terrain_collision本体Lines，同样正确。
4. case24在1407b76c1，1407b76f8和7707两次独立empty，再Sphere .2和显式None，候选没有same Arc或shape None简化。19/20/21/25原sphere→真实Mesh::try_from→indexed→drop原mesh，path分别chimney/vent_pipe/roof/roof，候选吻合。
5. Demo仅6..9与22/23跳过，且仍先注册全部JSON。52行矩阵与demo34行独立计算一致；7张表的21个signed relative target从raw_hex重新计算全部吻合。rank严格1..max_rank，3值来自现canonical DecoratorType，不由资产文件数量猜测。
6. Lines closure1407baf00：未索引position复制→drop临时mesh→空len panic→append首点，已有闭合也追加；候选保持重复首点、重复两次outline文件加载与各自失败边界。convex closure不允许失败回退bbox，临时positions在triangle/bounds前drop。indexed重用当前拥有复制的new_indexed切片API，未额外造缓存镜像。
7. bbox1407bb524..555严格轴(1,0)，unordered亦跳失败；候选Vec2 assert吻合。width/depth原rect size，height top-bottom，center(top+bottom)*.5，原xz_rotation_bad(-axis.y,axis.x)，box call1407bb602早于cuboid1407bb678，再lookup1407bb747+bounds。候选未把full dimensions再减半，二次lookup/真实bounds顺序正确。

## 有界验证与剩余

只核源码、冻结hash、原指令和分支表。候选6个真实CPU几何/52矩阵测试可供root执行，本review未运行；未将fixture或rustfmt解析视为已验证。真实watch锁毒化失败注入尚未执行，跨线程registry重入/原依赖fork/完整C语义未重验。唯一PreStartup root捕获实际root/config.dev.demo及资源安装必须由root整合；本leaf不能证明世界编辑/卸载/保存完成。
