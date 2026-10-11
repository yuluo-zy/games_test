# 三版同码 52 个工具函数的 Rust 候选

52 个逻辑函数组、29 个原文件组、156 个版本映射，仅保存一套候选。

状态 **implemented_unverified**：按此前验证暂停要求，没有构建、测试、原码执行或独立审查。
短函数的观察语义已写成 Rust；原始签名/ABI 和完整 producer/consumer 仍有待补证项，详见每组合同。

源码保存在各合法 claim 的 work 目录，并保持原 crates/… 路径。
每个函数含中文注释、A/B/C VA 与逻辑组 ID。JSON 保存原函数 ID、字节 SHA、证据和消费者。

| 原模块 / 候选文件 | 函数组数 |
|---|---|
| [crates/utils/src/loop_range.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-71ec411b5d563a05bc/abc-tools-52/crates/utils/src/loop_range.rs) | 6 |
| [crates/country-core/src/utils/debug_viz.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-67c8f0bbd38c78935a/abc-tools-52/crates/country-core/src/utils/debug_viz.rs) | 5 |
| [crates/country-core/src/resources/walls/wall_attachment.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-fe6e3b5ee72432c3d7/abc-tools-52/crates/country-core/src/resources/walls/wall_attachment.rs) | 1 |
| [crates/utils/src/geometry/aabb.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-01726d288b6ab83595/abc-tools-52/crates/utils/src/geometry/aabb.rs) | 2 |
| [crates/country-core/src/resources/session.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-a4422b4c9680d489d4/abc-tools-52/crates/country-core/src/resources/session.rs) | 3 |
| [crates/utils/src/lib.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-1d8b7844066af51d6b/abc-tools-52/crates/utils/src/lib.rs) | 3 |
| [crates/country-core/src/resources/walls/decorator.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-43142338addb95ca2d/abc-tools-52/crates/country-core/src/resources/walls/decorator.rs) | 1 |
| [crates/utils/src/geometry/circle.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-ee62d4bfcf31848c9b/abc-tools-52/crates/utils/src/geometry/circle.rs) | 3 |
| [crates/utils/src/geometry/rectangle.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-6e76d82e8a953a9a45/abc-tools-52/crates/utils/src/geometry/rectangle.rs) | 1 |
| [crates/utils/src/wall_space.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-d9be0bb590da0f9f7d/abc-tools-52/crates/utils/src/wall_space.rs) | 2 |
| [crates/utils/src/rng.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-a5b23ebeaacbd24221/abc-tools-52/crates/utils/src/rng.rs) | 1 |
| [crates/country-core/src/resources/walls/mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-4509462cb49323f18d/abc-tools-52/crates/country-core/src/resources/walls/mod.rs) | 1 |
| [crates/country-core/src/resources/onboarding.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-ab43622efad901bc85/abc-tools-52/crates/country-core/src/resources/onboarding.rs) | 1 |
| [crates/country-core/src/resources/walls/inner_wall_state.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-3188a81d4989b20289/abc-tools-52/crates/country-core/src/resources/walls/inner_wall_state.rs) | 1 |
| [crates/country-core/src/systems/ui/resources/ui_state.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-099d9836aac44a50a4/abc-tools-52/crates/country-core/src/systems/ui/resources/ui_state.rs) | 3 |
| [crates/utils/src/geometry/intersect.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-95525ca2902fab432c/abc-tools-52/crates/utils/src/geometry/intersect.rs) | 2 |
| [crates/country-core/src/resources/history/mod.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-3846e898cf43c29d09/abc-tools-52/crates/country-core/src/resources/history/mod.rs) | 1 |
| [crates/asset/src/prefab.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-a3f23bd62617630cba/abc-tools-52/crates/asset/src/prefab.rs) | 3 |
| [crates/country-core/src/input/panic_repro_state.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-867ec0f6d3ac488689/abc-tools-52/crates/country-core/src/input/panic_repro_state.rs) | 1 |
| [crates/country-core/src/lib.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-ef1d59d5448a097d02/abc-tools-52/crates/country-core/src/lib.rs) | 1 |
| [crates/country-core/src/resources/roofs/roof_ridge.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-e0787bc1be6b98fce2/abc-tools-52/crates/country-core/src/resources/roofs/roof_ridge.rs) | 1 |
| [crates/country-core/src/resources/roofs/roof_shape.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-4df0b3511f93a7ad7a/abc-tools-52/crates/country-core/src/resources/roofs/roof_shape.rs) | 1 |
| [crates/country-core/src/systems/timelapse_replay.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-33dfed54eb285875e2/abc-tools-52/crates/country-core/src/systems/timelapse_replay.rs) | 1 |
| [crates/motomoto/src/lib.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-27c9dd1e5f1a286560/abc-tools-52/crates/motomoto/src/lib.rs) | 1 |
| [crates/systems/toolbar/src/mei/core.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-70a753eeccd8998d55/abc-tools-52/crates/systems/toolbar/src/mei/core.rs) | 1 |
| [crates/tg-text/src/shape_stream.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-aa951cb1d534b93d33/abc-tools-52/crates/tg-text/src/shape_stream.rs) | 1 |
| [crates/utils/src/subtractive_1d_boolean.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-ec0964771246600fce/abc-tools-52/crates/utils/src/subtractive_1d_boolean.rs) | 1 |
| [crates/utils/src/timer.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-22c325535624cbec5e/abc-tools-52/crates/utils/src/timer.rs) | 2 |
| [crates/country-core/src/render/sparse_instance_buffer.rs](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-f4e8ea3c0995e850c0/abc-tools-52/crates/country-core/src/render/sparse_instance_buffer.rs) | 1 |

完整对应：[mapping.json](mapping.json)。原证据：[A](pack-A/report.json)、[B 原 RNE](pack-B/report.json)、[C](pack-C/report.json)。

PrefabName::as_name_ref 与 impl$5::as_ref 各版共享地址，后者委托前者；timer apply/queue 共享 ret，以导出别名复用。
其他地址相同但类型/业务语义不同的访问器保留各自模块，不据短指令合并作者类型。

glam 继续使用原依赖。`init_rng_u64` 只恢复游戏 seed 变换，不复写 fastrand。
CirclePoints2 只恢复构造器；未擅自补 Iterator::next。
get_inner/as_tag/as_id 保留 64 位观察值；is_overhang/is_some 保留 byte 返回投影，尚未证明原 Rust 返回类型。
DebugRectangle 构造器不将未初始化默认颜色 payload 补零；DebugSphere 保留已初始化原记录未知字节。
本批未接共享生产 lib/Cargo；不宣称 52 个完整作者类型或完整游戏系统已经还原。
