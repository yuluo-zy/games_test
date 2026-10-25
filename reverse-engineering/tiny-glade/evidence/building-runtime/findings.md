> 后续原始采集已证实并更正早期解释：PublicWall+0x10=raw Curve2点数，+0x30/+0x28c=raw长度；AtlasPart0与subset kind9替代早期rank/drawable推测。本轮raw采集、逐拱段gable、Nani注册与实际World进展见[最新记录](../building-gather/findings.md)。旧段落保留其当时范围，不能视作当前缺口清单。

# 墙缓存与真实 Bevy 建筑链（2026-10-11）

这一批推进到实际 World、事件 reader/writer、组件注册、Commands spawn/insert/despawn 和调度屏障。它是有证据支撑的 headless 建筑适配层，原作完整调度图/引擎分叉及完整建筑系统仍未验收。

## 原码与三版对比

- `three-version-producers.json` 保存 A/B/C 各五个入口的原二进制身份、指令和常量。A/B/C 墙初始化入口分别 `1404fb080/14078b550/14083fe70`，自有 `system_wall_constructor`；共享私有代码保留为业务，未当作依赖排除。
- `three-version-cache-verification.json`：每版 1024 例，共 3072 例。真实原码的 circle/rectangle 分支与 Rust 比较 max-y、两个 cache flag/value、arch 数组及长度。提供已分配 Vec 缓冲，不代写任何游戏数值规则。A/B/C record 为 0x288/0x2a0/0x2a8；额外字段偏移为 0x270/0x284/0x288，不能把 B 投影套到另外两版。
- B `14078b865..14078bc96`：轮廓 Y 以 -2.5 做 maxnum fold，再加发布额外字段、加 f32 -0.46、MAXSS 0.960000038；所有 base arch 高度写同值；cache flags108/110均为1。字段284的原名仍未知，不能当作权威墙 height30。
- B `14078c404..14078c42d`：矩形 gable 的第二最大值等于 base maximum + Roof::height + f32 0.46。`B-runtime-verification.json` 包含256例原码比较；逐拱段 profile 计算在这段之前，不在此验收范围。
- B `1421a2b60` 的 wall_height_dt 在真实 runtime 的 TileSimulation 中接入：旧最大值用于差值，再写新最大值；复用 Bevy change tracking。
- B `Commands::spawn` (`140dcf620`) 的泛型 tuple 确认 Roof/TileSimulation/TriggerVolume/Animation/RoofTiles/RoofSpire/RoofSupportBeams/NaniBundle<StoneFloorInstanceData>。这里只恢复游戏传入状态，不实现 Commands 或 archetype。
- RoofSpire/Support 的 B 注册描述符均 size0/align1；不是此前猜测的 allocation 字段。创建时两处零写属于 transform translation，三个1属于 scale；rotation 为[0,0,0,1]。业务 marker 在真实 Bevy 注册。完整 required-components/分叉注册图仍待核对。
- Trigger setters (`1408e45b0/1408e45f0/1408e4660`) stride36/36/24，每组 normal/UV entry 数等于 triangle 数；Rust 用 Vec 的三顶点位置/法线/UV语义类型。未复制 Vec header；非空触发体生成仍待恢复。
- Drawable rank constructor写 observed tag1/rank0；Nani creation discriminant为9。CPU adapter 仅保留这些确认的创建值，活跃 GPU variants、缓冲分配及 spire/support 几何没有被伪造。

`B-focused`、`B-cache`、`B-trigger`、`B-registration` 来自现有 B Ghidra readOnly/noanalysis 数据库（patched EXE SHA cf048e...）；动态指令差分均使用 tracking 中未修改 RNE SHA f507a6...。Ghidra共享函数地址有大量符号别名，`clone_behavior`/`Plugin::finish`标签不能用来认定创建调用的实际业务用途。搜索 cache writer 仅找正偏移108/110会漏掉真正 producer 的 -198/-190，已保存边界和排除的候选。

## 可运行联接

`reconstruction/crates/tg-building-runtime` 使用上游 Bevy app/ecs0.16.0，复用发布库实现。113个registry条目的name/version/source/checksum均与B Cargo.lock一致；features与原分叉补丁不宣称一致。

当前显式顺序为 Begin → Publish → BaseCache → Create/ApplyDeferred → Associate → Spatial → GableCache → CPUVisual/ApplyDeferred → Mark/ApplyDeferred → Animation → Despawn/ApplyDeferred → Reassociate → FinalCache。这是恢复工程的依赖顺序，尚不是原作整张注册图的逆向验收。

输入为明确的 resampled world outline + terrain samples，Arc/Vec/HashMap使用标准实现。发布替换确认字段并失效缓存，base producer填充，创建consumer通过真实Commands落地完整确认的CPU组件，再接原CPU瓦片组合。墙编辑产生有序kind2通知；重复删除请求每实体只标记/发声一次；已标记有效shape屋顶在淡出期间保持关联，despawn提交后清理并发布presence变化。缺失墙走实际删除消费者。gable/presence事件有真实cache消费者。

`tests/building_world.rs` 五项跨多帧验证：矩形→圆形/风格/高度编辑→删除→再建平顶；墙移除触发清理；不支持shape不造缓存；零dt不错误标记Animation变更；gable第二缓存随创建/删除更新。旧历史/纯算法四项测试也通过，workspace共9项。

`building-run fixtures/building-cycle.json` 输出五帧JSON，实体来自Bevy，记录CPU瓦片数/删除/关联结果，非手写实体模拟。ID采用十进制字符串避免精度损失。fixture是显式合成集成场景，不是原作保存文件重放。

## 当前验收边界与下一步

原状态/回放/创建/更新/生命周期报告全部重跑绑定新的workspace与lock；旧Windows整瓦片报告不自动计为ARM当前构建通过。

扩大原矩形流水线验证得到33组/19024 records：32组逐字节一致；case14的一个filler tile位置Y相差1ULP（首个索引53，be92fa02 vs be92fa03）。数量、seed、RNG及其它case一致。`B-roof-pipeline-host-verification.json`保留ok=false；CRT桥使用当前host math，glam SIMD与host三角函数边界仍待定位，原因尚未证实，不重写glam或修改oracle掩盖差异。当前运行时的CPU瓦片联接仅有功能验收，不声称本机全部输出位精确。

高收益未关闭项：原dirty/gather/Curve resample/terrain producer；freeform墙缓存分支；逐arch gable profile（runtime明确gable_profile_pending）；完整Nani及marker required graph；方向/FlatStyle消费者与屋顶历史事务；墙砖/支撑/开口及门窗附着。C继续为交付目标，当前runtime仍是B业务适配，C registry0.17.3/字段变化不能以B build替代验收。
