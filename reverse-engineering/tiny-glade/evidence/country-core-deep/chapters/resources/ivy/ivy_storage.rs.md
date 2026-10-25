# resources/ivy/ivy_storage.rs

> 文件作用：**常春藤折线存储：Ivy(VecDeque<IvyPoint 40B>)+IvyStorage(HashMap+RNG)**

函数总计 20：业务 4 / 内联无独立符号 12 / 派生胶水 4

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::ivy::ivy_storage::Ivy::add_point` | 0x140869460 | 46 | 系统:写入 | O：Ivy::add_point——push_back 40B 点（id=next+1，reserved=0） | collision-eco\decomp\decomp-0x140869460.txt |
| `country_core::resources::ivy::ivy_storage::Ivy::last_point` | 0x1408694f0 | 61 | 查询/计算 | O：Ivy::last_point——末点（空 unwrap 失败） | collision-eco\decomp\decomp-0x1408694f0.txt |
| `country_core::resources::ivy::ivy_storage::Ivy::last_segment_point_ids` | 0x140869540 | 90 | 查询/计算 | O：Ivy::last_segment_point_ids——末两段端点 id（len<2 expect 失败） | collision-eco\decomp\decomp-0x140869540.txt |
| `country_core::resources::ivy::ivy_storage::IvyStorage::spawn_ivy` | 0x1408696f0 | 136 | 系统:写入 | O：IvyStorage::spawn_ivy——RNG 采样根点建新株，next_ivy_id+1 | collision-eco\decomp\decomp-0x1408696f0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::ivy::ivy_storage::Ivy::point_count`（行 57）— O：点数 getter（grower/pruner 内联）
- `country_core::resources::ivy::ivy_storage::Ivy::set_last_point_position`（行 65）— 设置/写入
- `country_core::resources::ivy::ivy_storage::Ivy::remove_last_point`（行 70）— 删除/清理
- `country_core::resources::ivy::ivy_storage::Ivy::remove_root`（行 80）— 删除/清理
- `country_core::resources::ivy::ivy_storage::Ivy::get_point_id`（行 102）— 查询/计算
- `country_core::resources::ivy::ivy_storage::IvyStorage::new`（行 127）— 构造
- `country_core::resources::ivy::ivy_storage::IvyStorage::roots`（行 172）— O：全部根位置收集（spawner 内联）
- `country_core::resources::ivy::ivy_storage::IvyStorage::iter_mut`（行 176）— 迭代/变换
- `country_core::resources::ivy::ivy_storage::IvyStorage::iter`（行 180）— 迭代/变换
- `country_core::resources::ivy::ivy_storage::IvyStorage::remove`（行 184）— 删除/清理
- `country_core::resources::ivy::ivy_storage::IvyStorage::total_segment_count`（行 189）— O：全株段数求和（闭包内联）
- `country_core::resources::ivy::ivy_storage::IvyStorage::get`（行 195）— 查询/计算

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`closure` 等 4 项

## 实现方式（算法级）

- `Ivy::add_point`（0x140869460）：push_back 40B 点到可增长容器（cap-first，元素 0x28B）。**IvyPoint 0x28B 布局（O 机器码逐字段）**：`{id u64@0, pos Vec3(x,y@8, z@0x10), dir Vec3@0x14, u32 flags@0x20=0}`。点 id = `[self+0x58]+1`（point_id 计数 @+0x58 自增）。
- `IvyStorage::spawn_ivy`（0x1408696f0）：perchance RNG（`FUN_141625a50(self+0x30, end-start)` range，断言 `end >= start` @0x142aa5710 perchance-0.4.0；`FUN_141625ae0` f32）采样 u ∈ [start,end) → 建新株 root 点（0x28B，id=0）→ `FUN_141527820(self, next_ivy_id, &root_point)` HashMap<usize,Ivy> 插入 → **next_ivy_id @+0x40 自增**。
- **IvyStorage 布局（O）**：`HashMap<usize,Ivy>` 表体 @+0 + **perchance RNG @+0x30** + **next_ivy_id @+0x40**。非 FrozenMap（hashbrown 标准表，spawn_ivy 插入路径证实）。
- 内联面：`point_count` / `set_last_point_position` / `remove_last_point` / `remove_root` / `get_point_id`（grower/pruner 内联调用）；`roots()` 收集全部根位置（spawner 用）；`total_segment_count` 全株段数求和（send_to_gpu_segments 的 GPU 上传预算）。
- 随机源 = **perchance-0.4.0 crate**（RNG range 断言串路径证实，非 fastrand）。
