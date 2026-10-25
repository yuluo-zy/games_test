# Member14 两构造候选 V1

本包依 root 对两 assembly 源的合法授权及 07 的 35bb7a21 scoped design review 实施；不是完整七/四参数宿主。冻结前只运行 rustfmt（exit0），未运行 Cargo/rustc、tests、benchmark、App 或 GPU。

## 实际候选与接口

两个候选保留已整合 canonical 40/32B Params、原浮点 PartialEq 和未注册的显式 consuming tracker adapter。新增函数只写同一个 canonical `BrickTransformSsbo`，实际生产入口接受 `&mut SparseInstanceBufferWriter<BrickTransformSsbo>`，闭包逐砖调用其 `push`。没有第二 typedbuffer、全墙 registry、替身 writer、固定 max、原子整批替换或几何静默丢弃。

Chimney：`ChimneyBrickSection`、`construct_crossbrick_chimney(params,section,wall,&Rng,writer)->bool`；`write_chimney_instance(params,wall,decorator,&TerrainHeightsData<TerrainHeightSsbo>,writer)->bool`，复用真实 terrain sampler。Door：`construct_door_stairs(params,wall,&Rng,writer)->bool`；`write_door_stairs_instance(params,wall,decorator,writer)->bool`。每个 write 入口只执行一次 `Rng::with_seed(utils::rng::init_rng(decorator.0))`；section 共用同 rng。

`true` 表示至少一次 NoFreeChunks，仍继续所有后续砖。每个 dirty wall 的 lane clear 必须由真实周围宿主执行一次，不能每个实例 clear。构造不处理 audio、不消费 tracker log、不进行 GPU flush，不自行创建 wall owner。原 host 不读取 capacity flag 的事实保留，未发明 UI 或自动重试。

## 原证与计算边界

必须与 main work 的 MIDDLE_CHAIN_CONTRACT_B_V1 及强制 GEOMETRY_ADDENDUM_B_V1 一起读。后者纠正 tag1 −0.4；Door extra_vertical 是 word6，固定 0.56 是 depth。两个完整 constructor 的原 RNE bytes/完整 assembly，以及默认 Brick96B 和 source-id table 由 manifest 绑定。

保原 RNG 抽样顺序、两次 Vec 的 normalized splits→borrowed iter collect、层/边与 row/step/x 顺序、两个 Door push 点等价为 peeled first window+remaining windows、Chimney 全层累计 distance、Door 有向 [a,b]、seed 的 wrapping 和 canonical source encoding。两 temporary Vec 是原数据流，未为性能猜测抹去。只用原业务 min/max/clamp，没有全局固定上限，巨量或非法参数仍可能分配失败。

新增两项 unit checks 仅观察实际 emit 载荷和 RNG 推进：Chimney 8砖/连续距离/seed/default/material、Door 10砖/row seed/有向 intervals/default/material；两者验证首砖回报失败后仍尝试全部砖并保持后续载荷/RNG。它们尚未执行，且不证明 Sparse writer、宿主、设备或 native bit-for-bit 等价。

## 未闭合门

1. 同 `WallEntities`→`InstancedWall` 内 base/chimney/door 三 lane 的实际 owner 初始化需 root 合法外部源委派和审查；这是现架构有界映射，不声称恢复原四个 Brick allocations 全部布局。原仅第三/第四已精确追到本两 consumers。
2. 新 wall 必须在 assembly 前建立两 allocation 并 apply Commands；现 `InstancedWall::new` allocation None 不能被当 PublicWalls miss 消耗 log。既有 base update 保留 deco handles，不凭猜测重置 lane。
3. Shared library 必须实际 `get_mut<BrickTransformSsbo>`；flush 仍由同 PostUpdate 共享维护执行，保持 Drop→waste→回收，highlight 仅 base 排除。W21/W27 compute 不因 raster status gate 自动停下。
4. 原 Chimney 7参真实 PublicWalls/terrain/derived 双ref/audio、Door4参真实 library/UI 身份、on-wall-changed anchor dirty 链、注册顺序待后续真实实现。Removed 的 Chimney subtype/audio 不可空化；Door 的 no-op 仅其已证 explicit 分支。
5. rustfmt 不证明编译。若复用 bounded same-source harness，必须直接包含当前真实 canonical definitions/实际 writer，冻结来源 spans/hashes，不能手写 Brick/terrain/writer 替身。此包尚无获审运行 harness；不得外推前一 tracker8tests 或 sharedutils7tests 为本构造已验证。
