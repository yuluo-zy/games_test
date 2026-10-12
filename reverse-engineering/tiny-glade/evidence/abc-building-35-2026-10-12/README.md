# 优先批：35组共享Rust候选

用户授权按优先级开始还原，本批先补几何、建筑接口和历史接口。目标C，ABC同码函数仅写一次，21个原源码路径文件组，中文注释与105个版本函数映射齐备。

**状态：implemented_unverified。** 候选已保存并冻结，独立静态审查仅覆盖原指令观察投影；完整作者ABI/依赖适配/真实业务消费者仍待恢复；未构建、未测试、未运行原码，也未接入生产工程。冻结指纹只检查交付未变，不代表行为等价。

## 优先级与用途

| 顺序 | 范围 | 数量 | 用途 |
|---|---|---:|---|
| 1 | 几何与数值 | 14 | 插值、二维转三维、RNG入口、范围长度、包围盒/矩形/圆/三角形 |
| 2 | 建筑接口 | 17 | 墙角附着、墙ID、墙洞访问/过滤、空间网格、装饰物ID/key/碰撞借用、屋顶动画初始化、形状中心 |
| 3 | 历史接口 | 4 | 历史链索引、空编辑序列、重放长度、中断命令编码 |

这些是完整建筑链的叶接口；不能把初始化/getter完成算作编辑、删除、undo/redo链已闭合。

## 导航

- [统一组合入口](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-1d8b7844066af51d6b/abc-building-35/assembly/lib.rs)：导入上批类型与本批方法，旧函数不重复实现；这是候选源码入口，尚无生产Cargo整合。
- [35组ABC映射](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/abc-building-35-2026-10-12/mapping.json)：原符号、VA/RVA、原body SHA、候选SHA、源码行、caller与复用候选。
- [冻结清单](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/abc-building-35-2026-10-12/deliveries.json) / [依赖闭包](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/abc-building-35-2026-10-12/dependency-closure.json)。
- [独立恢复前合同审查](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/abc-building-35-2026-10-12/contract-review.md) / [独立冻结候选审查](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/evidence/abc-building-35-2026-10-12/final-review.md)。

## 原模块候选

| 原文件 | 数量 | 共享候选源码 |
|---|---:|---|
| `crates/country-core/src/resources/walls/wall_attachment.rs` | 2 | [Rust](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-fe6e3b5ee72432c3d7/abc-building-35/crates/country-core/src/resources/walls/wall_attachment.rs) |
| `crates/country-core/src/resources/walls/decorator.rs` | 3 | [Rust](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-43142338addb95ca2d/abc-building-35/crates/country-core/src/resources/walls/decorator.rs) |
| `crates/country-core/src/resources/walls/wall_holes.rs` | 4 | [Rust](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-8516251c5a664dec0c/abc-building-35/crates/country-core/src/resources/walls/wall_holes.rs) |
| `crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs` | 2 | [Rust](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-56afc78f73b7ab7966/abc-building-35/crates/country-core/src/systems/wall/wall_state/wall_spatial_hash.rs) |
| `crates/country-core/src/resources/roofs/roof_animation.rs` | 1 | [Rust](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-3b800c682fa41d3683/abc-building-35/crates/country-core/src/resources/roofs/roof_animation.rs) |
| `crates/country-core/src/resources/walls/decorator_archivist.rs` | 2 | [Rust](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-774ee58bc153366d1d/abc-building-35/crates/country-core/src/resources/walls/decorator_archivist.rs) |
| `crates/country-core/src/systems/history/history_process.rs` | 1 | [Rust](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-c4cdc0aa5e62db9266/abc-building-35/crates/country-core/src/systems/history/history_process.rs) |
| `crates/country-core/src/resources/history/mod.rs` | 1 | [Rust](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-3846e898cf43c29d09/abc-building-35/crates/country-core/src/resources/history/mod.rs) |
| `crates/country-core/src/resources/history/replay_cmd_buffer.rs` | 1 | [Rust](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-d63bc50864713c7344/abc-building-35/crates/country-core/src/resources/history/replay_cmd_buffer.rs) |
| `crates/utils/src/swizzlers.rs` | 1 | [Rust](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-268a3bb2f466c1c2b3/abc-building-35/crates/utils/src/swizzlers.rs) |
| `crates/utils/src/lib.rs` | 1 | [Rust](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-1d8b7844066af51d6b/abc-building-35/crates/utils/src/lib.rs) |
| `crates/country-core/src/resources/walls/decorator_subtype.rs` | 1 | [Rust](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-60901c6c8508b1af76/abc-building-35/crates/country-core/src/resources/walls/decorator_subtype.rs) |
| `crates/utils/src/rng.rs` | 1 | [Rust](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-a5b23ebeaacbd24221/abc-building-35/crates/utils/src/rng.rs) |
| `crates/country-core/src/resources/history/wall_compound_edit.rs` | 1 | [Rust](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-3c7e1129dc1230a273/abc-building-35/crates/country-core/src/resources/history/wall_compound_edit.rs) |
| `crates/utils/src/geometry/aabb.rs` | 3 | [Rust](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-01726d288b6ab83595/abc-building-35/crates/utils/src/geometry/aabb.rs) |
| `crates/utils/src/geometry/rectangle.rs` | 5 | [Rust](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-6e76d82e8a953a9a45/abc-building-35/crates/utils/src/geometry/rectangle.rs) |
| `crates/country-core/src/systems/wall/valid_enclosure/mod.rs` | 1 | [Rust](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-f2c3841f004fb1ad05/abc-building-35/crates/country-core/src/systems/wall/valid_enclosure/mod.rs) |
| `crates/utils/src/geometry/circle.rs` | 1 | [Rust](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-ee62d4bfcf31848c9b/abc-building-35/crates/utils/src/geometry/circle.rs) |
| `crates/country-core/src/resources/walls/mod.rs` | 1 | [Rust](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-4509462cb49323f18d/abc-building-35/crates/country-core/src/resources/walls/mod.rs) |
| `crates/utils/src/geometry/intersect.rs` | 1 | [Rust](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-95525ca2902fab432c/abc-building-35/crates/utils/src/geometry/intersect.rs) |
| `crates/utils/src/loop_range.rs` | 1 | [Rust](/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-71ec411b5d563a05bc/abc-building-35/crates/utils/src/loop_range.rs) |

## 恢复边界

- std/Vec、glam、既有类型直接复用；不重写容器、ECS或库内部。使用方的锁/features和平台ABI在后续整合时核对。
- 原PDB函数边界尚未独立证明；本批短体观察与具体消费者补证是范围依据，不能推出完整作者类型。
- WallSpatialHash宽高是signed32动态字段；乘加32位回绕后符号扩展，不能固定64。
- FinalizedWallHoles过滤来自ABC真实closed消费者的record+17==0规则；未过滤holes别名保留所有记录。
- DecoratorLibKey保留EAX/EDX两个32位返回槽，不凭短体改成完整u8枚举。
- ShapeParameters按合法tag读取payload；结构全集和非法输入行为未恢复。
- 原角点Option无效payload不制造；Archivist getter仅借用；历史interrupt仅构造编码，未清队列或commit。
- RoofAnimation仅两个零通道；完整动画调度/时间/事件链未由构造器恢复。

## 下一推进点

下一步按消费者推进装饰物source/asset/subtype → 放置位置与碰撞/开口 → move/delete及历史传播。优先补本批key、holes和Archivist的真实owner输入，避免继续堆无消费者的getter。随后补材料统计和其他历史owner；GPU/场景继续后置。原保存断点与旧验收报告不因本批改变。
