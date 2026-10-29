# M01 trap door 与楼梯节点画面生成恢复

日期：2026-10-27。只修改原 B 既有的 `systems/decorator/src/decorator_visual/trap_doors.rs` 和 `stairs.rs`，不新增业务目录。共享 Blueprint API 修复与生产注册由中央 agent 负责。此报告不代表整套楼梯工具、楼梯砖装配或整游戏 GPU 验收完成。

## 原版身份与输入面

实际读取了 `reverse-engineering/tiny-glade/skeleton/B/crates/systems/decorator/src/decorator_visual/` 同名两文件。trap SourceSHA 为 `d34b2810667a4f4f37744e041a22778a32d7567dc827292d80d9e57ba78faf59`；stairs 为 `71c82baf8ce016a9c152c1c8cc0e3f2f04ac677cb06fc297deeb9030444488a5`。发行 RNE、匹配 PDB 的 SHA/GUID/Age、逐指令与采样 metadata 已由本目录 `m01-trap-stair-visual-evidence-manifest.json` / report / 三组 procedure 文件绑定。trap 原 VA `1407be730`、PDB 完整 7587B；stairs 原 VA `140803dd0`、完整 9024B，不能以旧 Ghidra 的片段长度代替。

trap 七项参数：DerivedDecoratorInfoResources、Storage、CachedTransforms、PublicWalls、Query<Roof>、Blueprints、UiSignifierStream。已替换旧单一 subtype 及瞬态适配，使用真实 subtype/mesh owner 视图。

stairs 七项参数：Storage、CachedTransforms、PlayermadeStairsAssemblyRequests、Blueprints、PublicWalls、Query<Roof>、UiSignifierStream。当前 Rust 加入第八项既有 `DemoModeEnabled`，投影原 process-global tri-state==1 门；与请求生产器共用相同 canonical 资源，不创建平行默认门。

## trap door 完整输出

沿原墙 bucket、roof 存在且为 flat、subtype 5/10、必需缓存依次检查。保留原矩形负 padding `bfee147a`、圆探针半径 `3f6e147a` 与 intersect Option 的原 polarity。符合条件后才插 blueprint；后续失败保留此前已经发生的写入。

实际写入消费端读取的 visual_entities：cottage/gothic trap mesh、`nani_balcony_door`、完整缓存 Transform/scale、颜色999→0、state 非0半透明。nani 类别不等于 BalconyDoor 条件组件，原 builder 的该条件仍为 false。实际碰撞与交互用 `(1.3,.5,1.3)` 盒，克隆 RaycastMeshData 形成不同外层 Arc、共享内部 shape，保留原世界 pose。

墙洞写入 canonical `DormerHoleSsbo::cottage`，不再构造后丢弃本地影子类型。wall ID 沿原 4B getter 1400885d0 截断，尺寸 `(1.125,.375,1.125)`、缓存位置及完整非单位 quaternion 的 rotated-Z 尾项，固定末字1、-50。旋转运算保留原 SSE 加乘顺序，无归一化。

## 楼梯节点完整输出

原此叶生成交互形状、轮廓与条件墙洞，实体砖装配由独立 stairs_assemble 消费请求。恢复这里时没有额外虚构一个楼梯 Nani 模型。

依序遍历 wall/terrain/stair 真 bucket，只检查 modified IDs 和 ty0x3f；缓存缺失失败先于 insert，insert 写 source/state 后立即固定 interaction_source_flags=1，再取真正20B stair extra。对当前节点仅统计 outgoing alignment，tag2仍计数、incoming不算；原 straight/turn 冲突保留 `assertion failed: !any_normal`，不吞错。由原四类 profile 得到宽度、局部 Z offset、是否开墙洞，保留 width+.1+.1 的两次浮点加法。

outline 用完整 quaternion 转动 `(0,.25,offset)` 后加原位置，保留原 scale/rotation；交互 collider 世界 pose 使用原缓存位置，不误用 offset 后轮廓位置。目标 tag2/3 生成原顺序三子盒 compound，原 hash 输入含 identity pose；tag4 保留原 unreachable，其他目标用 base。只有最后才把 collision slot Disabled；错误路径不提前修改后续字段。

墙洞仅满足真实 wall/publication、roof 存在、flat、premade、目标类型允许时产生。材料 Matching 走真实半木 authority；显式材质读 bit0。原 `max_y - wall_coord.y < .85` 有序比较拒绝 NaN，木/石 rim 原字 `3f428f5d/3f28f5c3`；canonical RawWallHole 中心 `[u,(max-rim)+5]`、尺寸 `[width,10]`、kind4。

## Arc allocation 与 data 指针校正

楼梯 `1408047aa` 调 insert，`1408047af/47b6` 保存/恢复返回值，**`1408047bd` 写 `[returned+129]=1`**，47d6随后取 as_stair。早期口头地址140804719是错误估计，现以完整字节证据更正，不采用该估计。

insert `140825df2` 从 bucket 取 Arc allocation，`140825df6 LEA RAX,[RCX+10]` 返回数据起点；`140825e02/e09` 把传入 Info.state 写 allocation+138，即 data+128。消费者140829850分开读 allocation138(skip双物理upsert)与139(交互source flags)。因此 data129恰是既有 interaction_source_flags，不能另造字段。中央已修 insert_for 第三参→u138、flags默认0；楼梯专门写 flags1，trap保留0。中央也将 set_interaction_mesh 改为 canonical interaction_mesh 消费槽，消除旧 legacy 字段写了却未消费的问题。

正式 state0 与预览 state1均保留相同业务几何。预览u1381不注册两张碰撞表；转正式后楼梯仅交互表1，trap两张表均有真实 collider。不能把 state 误设 flags 导致预览注册碰撞，也不能把 flags1误当skip造成正式楼梯不可交互。

## 差分、失败和生命周期验证

本目录两份 probe 在 Unicorn 执行发行 RNE 原指令的有界区间，无替代 callee。楼梯 profile/shape 8类×6宽度共48行，其中42正常、6触发原冲突panic入口；trap64原SSE旋转向量包含非单位quaternion和±0。Rust逐位比较106正常向量，原panic门另测。区间执行不等于原程序完整启动。

最终命令均用 CARGO_INCREMENTAL=0、既有 target、offline、system-decorator lib test；该依赖启用 country-core/native-physics：

- `canonical_trap_dynamic_tests`：6 passed，0 failed/ignored，441 filtered，0.01s。
- `stair_visual`：6 passed，0 failed/ignored，442 filtered，0.03s。

测试实际运行两个生产 system→Blueprint→fullvisual，并检查非空真实 World、named Nani/Transform/Color、hole accumulator、两张物理表、同帧/跨帧无无谓重建、正式/预览切换、删除despawn、最后一个 mesh Arc释放、缺subtype/cache、错误extra与unreachable的副作用顺序。期望panic被 catch 并断言，日志中的panic不代表测试失败。完整日志和源哈希见 validation.json。

CPU fullvisual 资源描述符使用 TestDevice；本轮没有 OS输入，没有 GPU纹理/窗口/画质验收，不据此声称整套楼梯或M01全完。生产注册、全15叶联合GPU与Reload由中央单独验收。

一次测试fixture把u40=0误当非stair，实际上0是合法stair tag；改为 canonical Window extra 后通过，保留失败日志，未改业务逻辑掩盖错误。

## 工具修复与可复用经验

旧共享 Ghidra 的楼梯CFG跨入其他函数，不能照跨函数反编译补实现。专用worker先核PDB完整边界与原指令，再对原switch140804a9c/表142a8e438写4目标 override（4a9e/4ab9/4acf/4ae8）；修后完整反编译6126B且无跨函数污染。原数据库不在共享上修改。

当前ReVa请求有外层transaction，显式program.save报active transaction；不abort未知事务。采用正常managed stop再只读reopen验证，仍保留4目标override、完整decompile，确认持久化后再次停止自己worker。save失败、reopen成功的原始结果都保留。

下一轮先核pointer实际返回基址再解释offset；先追原失败顺序再放写入；带真实消费者验证canonical字段；用正式/预览双分支识别state/flags混淆；保留Arc生命周期和原浮点顺序，不以测试通过替代整条生产注册证明。
