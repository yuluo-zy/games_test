# tabloid:06 库外接线缺口 —— 深入分析（B 原生接线图）

2026-10-13。来源：共享 Ghidra B 后端反编译/xref（evidence/foundation-plt-2026-10-13/ghidra-producers.json、ghidra-tabloid-extra.json）+ country-core 源码（reconstruction/country-core/）。

## 结论一句话

tabloid 库本身已完成并验证；缺口在**真实游戏代码里"谁往通道发数据、谁从通道取数据"的生产→消费链没有接通**——country-core 用的是空操作替身（`reused::tabloid`），数据不流动。

## 原作 B 的真实接线（两类通道）

tabloid 是通用发布/订阅/累积通道。原版 `country_core` 用它承载两类游戏增量状态：

### ① DirtyTiles 通道（脏格子位图，64×64 = 512B）
- 语义：`State::empty` = `DirtyTiles::new_clear`(0x140960ac0，全零)；`StateDelta::apply_to` = 逐 u64 按位 OR(0x140960ef0)
- **生产者**（调 `publish`，实例 0x141cf6500/0x1425db920/0x141453350）：墙/屋顶/地形编辑系统；`read_bush_positions_from_gpu`(0x140eebd08)——植被 GPU 位置回读后标记脏格
- **消费者**（`subscribe`+`take`，实例 0x141247b10/0x141cf68e0/0x141292860/0x141454290）：`autoclutter_subscribe_to_tabloids`(0x1422d6bac)、`stairs_subscribe_lazy`(0x1407cf2ff)、`subscribe<512>`(0x142612c49)——需要知道"哪些格子变了"以增量重建

### ② ColliderDiff 通道（碰撞体增删差异，喂结构图重建）
- 语义：`State::empty` = `ColliderDiff::empty`（delta=None）；`StateDelta::apply_to` = `ColliderDelta{Added,Removed}` 写入（delta 类型是 **ColliderDelta**，State 是 **ColliderDiff**，两类不同）
- **生产者**（publish 实例 0x141453c00）：`upsert_collider_entry`(0x140b40ff2/0x140b412e6)、`on_collider_upserted`(0x1408ce487)、`dispose_collider_mesh_instance`(0x1408d972e)——碰撞体增删时发布
- **消费者**（subscribe 实例 0x141454410/0x1412926e0）：`structural_graph_subscribe`(0x1407d49ac)——结构图重建订阅

## 重建工程为什么"没接上"

- tabloid 在共享 workspace（reconstruction/shared-foundation/crates/tabloid），已完成并逐函数验证。
- country-core 是独立 crate，**Cargo.toml 不依赖 tabloid**；为编译在 `reused.rs` 放了空壳替身：
  ```rust
  pub mod tabloid {
      pub struct Publisher<T>(pub crate::Gap, pub PhantomData<fn() -> T>);
      impl<T> Publisher<T> { pub fn publish(&self, _diff: T) {} }       // 空操作
      pub struct Subscriber<T>(pub crate::Gap, pub PhantomData<fn() -> T>);
      impl<T> Subscriber<T> { pub fn take(&mut self) -> Option<T> { None } }  // 恒 None
      pub fn publish<T>(_p: &Publisher<T>, _d: T) {}                    // 空操作
  }
  ```
- 真实游戏类型语义**正确但没实现 trait、没接通道**：
  - `DirtyTiles`(country-core/src/utils/dirty_tiles.rs)：有 `new_clear`/`extend`(OR) 方法，无 `impl tabloid::State/StateDelta`；`DirtyTilePublisher` 仅存在于 world_raster.rs 文档注释（PDB 命名实体），连定义都没有。
  - `ColliderDiff/ColliderDelta`(country-core/src/systems/collision/world.rs:495-515)：有 `empty`/`apply_to` 方法；唯一引用 tabloid 桩处是 `subscribe_to_changes()`/`mesh_intersect_collider_diff_publisher()`(world.rs:429-437)，但 take 恒 None、publish 空操作。

**结果**：即使 country-core 某系统调 `publish`/`take`，数据也不流动。

## "库外"的含义（方法论）

项目对照方法论区分两类缺口：
- **库内缺失逻辑**：tabloid 自己的函数未实现 → **无**（本轮 fresh 反编译确认）
- **库外接线**：游戏侧把真实类型接到通道、把真实系统接到生产/消费 → **本缺口**

tabloid 库的职责止于通用机制；把 DirtyTiles/ColliderDiff 实现为 State/StateDelta 并让真实系统 publish/subscribe，属 country-core 恢复闭包。

## 闭合需要的具体工作（country-core 侧任务）

1. country-core 增加对真实 `tabloid` 的依赖，删除 `reused::tabloid` 桩。
2. `impl tabloid::State for DirtyTiles`（empty=new_clear）、`impl StateDelta<DirtyTiles> for DirtyTiles`（apply=extend/OR）。
3. `impl State for ColliderDiff`（empty=delta None）、`impl StateDelta<ColliderDiff> for ColliderDelta`（apply=写入 delta）。
4. 建真实 producer：`DirtyTilePublisher` 资源持有 `Publisher<DirtyTiles>`，墙/屋顶/地形/植被系统每帧 publish；碰撞体增删路径 publish<ColliderDiff>。
5. 建真实 consumer：stairs/autoclutter/512 系统 subscribe+take DirtyTiles；结构图系统 subscribe ColliderDiff。

## 与本轮 tabloid 契约测试的关系

本轮新增 `reconstruction/shared-foundation/crates/tabloid/tests/dirty_tiles.rs`（512B 位网格 zero-empty + OR-apply，4 测试）正是第 2 步语义的镜像演示——先在 tabloid 侧验证 `State/StateDelta` 契约闭环，作为 country-core 接线的桥。真实接线本身属 country-core 恢复工作流（后续闭包）。
