# M01 墙触发体真实碰撞几何恢复（2026-10-27）

## 原证与实现

原 B `WallTriggerVolume::get_collision_geo` 为 `1408e8d70`，匹配 RNE SHA `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03` / PDB SHA `2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。PDB physical span 4437B，不能把 Ghidra body 3669B 当作完整物理范围。`m01-wall-trigger-get-1408e8d70-full.asm` 为完整指令证据，`m01-wall-trigger-get-native.c` 仅辅助读 CFG。

保留 B 原文件 `crates/country-core/src/resources/walls/wall_trigger_volume.rs`，新增真实 `get_collision_geo` 和拥有型 `WallTriggerCollisionGeo`：三组独立数组分别为三角形、逐角点法线、逐角点 UV。参数32B原逻辑包括 u 范围独立存在位、法线偏移独立存在位、高度下限、顶部偏移、apply；与既有 `get_tris` 共享同一原 callback 几何分支。空 quad、范围非法、全部裁剪为空仍失败；没有 bbox 近似、默认缓存替代或吞失败。

原函数每次只读取 quad 与 special，不读取或修改三组默认缓存。120B quad 的 positions、prepared_u、两端 normal/heights 和 UV 全被消费。u 裁剪只插值位置；输出 UV/normal 按原角点索引复制，不插值或归一化。特殊分支统一读首端高度。MAXPS 后 blend 保留 height NaN 与有符号零选择。保留每个加减乘除次序；有限结果走直接算术，非有限结果才重演原 SSE 的输入 NaN 顺序/payload 与新生 invalid `0xffc00000`，不做最终数组 NaN 总归一化。

原调用 `update_rapier_colliders` 在 `1408d2ba8` 取得三个 Vec，`1408d2c04` 消费为 `RaycastMeshData::new_unindexed(triangles,normals,Some(uvs))`。已向 GPU owner 提供 typed API；生产主系统的注册/真实帧验收由该 owner 负责，本文不把局部 World 测试冒充已打通生产主循环。

## 原指令差分与真实资源验收

- `m01-wall-trigger-collision-native.py` 完整执行原入口至自然 ret，仅桥 allocator/panic；Triangle::new 与 inverse_lerp 仍执行原指令。345 组含288既有有限样本、24非有限端点、双 quad、范围、空输入及 NaN/±0/∞组合，53 原失败。
- `m01-wall-trigger-collision-rosetta.c` 将 PE sections 按 RVA 映射到原基址，由 Rosetta 直接执行完整原函数，同样只桥 allocator/panic。345 组/53 失败，`m01-wall-trigger-collision-rosetta.json` 是当前严格逐位夹具。
- Rosetta 否定 Unicorn 在 case335/338 三角点2/5/11的双 NaN payload：原为 `0x7fc12345` / `0xffc54321`，Unicorn误选第二个 `0x7fc98765`。保留两个 oracle，不改原失败、也不放宽 NaN 比较。
- 真实 `PublicWallState::collect → prepare_paths → WallTriggerVolume::recompute` 生成折线墙多面；组件置于真实 World，经两次折线编辑构造三组输出并逐点消费到 `RaycastMeshData`。实际 Rapier upsert/replacement/removal、旧 mesh Weak 失效、最终 Arc 释放和组件 despawn 均验收；失败调用保留默认缓存。测试使用真实局部资源，未改共享生产 main/map。
- `CARGO_INCREMENTAL=0 cargo test -p country-core --lib --offline --features native-physics wall_trigger_volume::native_edit_trigger_tests -- --test-threads=1`：4/4，通过345严格新样本、原288/24回归及非空World生命周期。最终 log 为 `m01-wall-trigger-collision-tests-final-v2.log`。

保留首轮失败 log：首先确认ARM新生NaN与x86位型差异，再用Rosetta交叉否定2个Unicorn错误。World初稿假定freehand height字段改动必然改几何hash，实际该种类不成立；最终改为真实折线控制点编辑，并验证实际hash/mesh替换。没有放宽任何原数值比较。

## 下一依赖边界

`construct_arch_geo1408d7ed0` 为另一完整1165B leaf，输入104B段；原 RDX 是逐三角法线数组，R8 是三角数组，两者stride36。现 ArchSegment 已有 MaterializedWallSegment，LintelSegment/LintelSegments 仍 Gap，必须先核原 writer 并恢复真实 owner，不能插空容器宣称生产链完成。此阶段尚未修改该 helper、Lintel 或生产主系统。

## 补充参数位型验证

新增 `m01-wall-trigger-collision-extra-inputs.json` / `m01-wall-trigger-collision-extra-rosetta.json` 共250组、17原拒绝，逐一将坐标12分量、法线6分量、height/u、offset/minimum/top置±0、±∞、quiet/signal NaN、subnormal及最大有限值，并组合25组u与裁剪范围。继续运行同一原Rosetta入口，三数组全部严格逐位相同；不使用候选代码作oracle。当前新差分总595组、70原拒绝。最终四项测试以 `m01-wall-trigger-collision-tests-final-v3.log` 为准，旧v2保留。
