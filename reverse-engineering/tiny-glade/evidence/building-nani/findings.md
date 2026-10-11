# Nani 屋顶创建注册与 CPU 更新边界

## B 完整创建组件图

原 `create_roof` tuple 的七个游戏组件为 Roof、RoofTileSimulation、RoofTriggerVolume、RoofAnimation、RoofTiles、RoofSpire、RoofSupportBeams，后接 `NaniBundle<StoneFloorInstanceData>`。PDB tuple drop 名称仅用作 schema 证据，std drop/Bevy Bundle/container 的实现均排除。

B `NaniBundle::component_ids` 0x142588be0 实际依次登记：

| 实际登记入口 | 游戏组件 | 证据 |
|---|---|---|
| 0x142588c90 | NaniInstanceDesc | descriptor 字符串 0x142eacc67 |
| 0x14219bb20 | utils::components::transform::Transform | descriptor 字符串 0x142eacdce |
| 0x1428188c0 | InstanceDataMarker<StoneFloorInstanceData> | descriptor 字符串 0x142eacd11 |

三者是显式 Bundle 字段。NaniInstanceDesc 的原数组 stride 为 24，Transform 为 48；Bundle 按 Transform 对齐含尾 padding，所以复制 80 字节不代表 80 字节未知 GPU 组件。typed marker 为 ZST。Rust 使用真实 Bevy `Component`/`Bundle`/`World::register_component`，不复制 ECS 内存布局。

创建 tuple 的全部10个组件的 B required 图均为空（Roof/TileSimulation/TriggerVolume/Animation/Tiles/Spire/Support + Descriptor/Transform/typed Marker）。B 原派生代码仅 enforce → register Self → push → pop。首次 Self 注册内部再调自身派生回调，因此汇编自调用并不是 require-self 或 require-Nani 边。以匹配的 bevy_ecs_macros 0.16.0 原源码解释这一依赖边界，不能把库自登记重写成游戏规则。证据入口分别 0x140903f40、0x14258d1f0、0x14258d8e0、0x14082cad0、0x140c9d1c0、0x1428188c0；补充四类为Roof 0x1408e6400 / TileSimulation 0x140939470 / Animation 0x1408e06d0 / TriggerVolume 0x1408e6b00，原asm见 B-*-required 与 B-roof-required-extra.json。C 的 RoofTiles/Descriptor/Transform required 方法合并为 0x140003310 的 RET，体现 0.17.3 生成策略变化，仍为空；A 没有同类 required API 符号，不能以符号缺失宣称不含组件。

先前 `DrawableRank` 独立组件及 `StoneFloorDrawable` 单 tag 类型是错误解释：0x140833250 是 MeshAtlasPart::new(0) 的别名，0x14082bd60 写入 NaniInstanceMeshDesc AtlasPart tag=1/index=0；tag=9 位于 NaniInstanceDesc+16，为 GPU subset kind。它们现在以 typed descriptor 与 marker 注册。不得把 Ghidra 任选的 DecoratorRank 标签视作实现证据。

## 描述符、更新与移除

`nani_maintain_instances` 0x14206f300 的初次 atlas 选择：直接 AtlasPart 从 +4 读取；Named 从 +8 取 NameHash，在 atlas entries 中选第一个 subset_kind 与 descriptor+16 相符的条目，缺失失败。已有实例 Changed Named descriptor 则要求该名称只有一条 atlas entry 且 subset 一致，再调用 `GpuWorld::update_instance_mesh`。`resolve_nani_mesh` 恢复游戏选择/断言政策，资源 lookup/哈希/解码由资源持有者提供。

原作 NaniActiveInstance 是 GPU backend 成功创建后插入的组件，不属于 Roof 创建 tuple。RemovedComponents<NaniActiveInstance> 驱动 Local entity→InstanceHandle map 移除，missing handle unwrap 失败，再调用 GpuWorld::remove_instance。没有可证实的游戏 Component on-remove hook 替代这个消费者。

当前无 GraphicsDevice/atlas/GpuWorld，不能填造 InstanceHandle 或 NaniActiveInstance。`DeferredNaniRequests` 与 `NaniCpuStagingState` 明确是 CPU backend 适配器：真实 World 创建/修改/移除产生 Create/Update/Remove intentions，保持已恢复 upload 参数，等待 backend 接收。它们不是已经执行原 GPU 分配/上传的证明。消失实体清理通过实际 Query 结果，没有手写 ECS 实体表。

## StoneFloor CPU payload

`project_roof_stone_floor` 恢复 B 0x142078dc4 到 0x141570690 writer 边界的 Roof 分支，给出 64 字节 upload record 及 bounds。记录含 pivot/标高、WallId 低32位、圆形切向或矩形方向、尺寸、动画第一通道、shape/floor/hidden flags、flat style flags 和两参数。Height > 0.1（unordered 亦走此分支）时尺寸为零；flat 标高为 max_y - 0.69；profile margin 为 profile*0.5 + 0.63，ridge=1 时方向只放大一个矩形轴。

Circle2d 切向依赖保持分开的 fmodf/sinf/cosf 调用边界，phase = positive_angle + (1-u)*TAU。ARM 编译器合并 sin/cos 成 sincosf 曾导致11个1ULP差异；独立 noinline 调用使 host-libm 桥接对照通过，不是重写 trigonometry 库。

原码差分1024例：矩形512例直接原游戏指令；圆形512例原游戏指令但 Windows CRT 的三个数学函数明确桥接 host libm。报告 `B-cpu-verification.json` 记录实际 probe 构建来源。这个范围不含原 Floor 分支、atlas/GPU资源创建、Windows CRT 与 ARM libm 自身的位精确等价或 full renderer。

## 三版本差异与依赖边界

A/B/C 创建 tuple 均命名 StoneFloor NaniBundle；默认 AtlasPart0/identity Transform 同策略。A、B subset tag 为9，C为7，原写入地址分别 0x141988c4b / 0x1421ab2d3 / 0x142826f61。B CPU record64；A writer 0x140f3a5e0 前仅48字节且 Roof height/maxY 偏移为28/40。C updater 0x1425b2c70 新增 PublicWall count/height 验证、wall style 输入、flat margin 及 circle 0.1 修正，不能当 B 原指令等价。

`three-version-registration.json`、A/C 原 asm 与 PDB tuple 摘要保留搜索边界；A/C 的完整 generic typed component-ID 候选尚未全部映射，B 的显式创建图已经闭合。三版 drawable.rs / nani/systems.rs 源校验不同，B/C Transform 与 RoofTiles private source checksum 相同仍属于自有 CPU 状态，不能因此排除。Bevy registry 0.16.0/0.17.3、std/HashMap、half/f32 math 等复用，不实现其内部。
