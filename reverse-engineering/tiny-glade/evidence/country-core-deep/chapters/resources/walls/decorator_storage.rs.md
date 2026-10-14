# resources/walls/decorator_storage.rs

> 文件作用：装饰存储：按目的地分桶（+0x0 墙 HashMap<WallId,Decorators 0x90B>、+0x90 地形、+0x120 楼梯、+0x18/+0x48 其余、+0x78 为 0x18B{DecoratorId,ptr} 查找索引、+0x1b0 可写标志）；支持快照(take_snapshot，16B XOR 掩码)、修改追踪、墙合并/分裂时装饰迁移（d2s 同步）。

函数总计 114：业务 60 / 内联无独立符号 13 / 派生胶水 41

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::walls::decorator_storage::DecoratorStorage::get_mut` | 0x140a1cde0 | 0 | DecoratorStorage::get_mut | O | wall\decomp\decomp-0x140a1cde0.txt |
| `country_core::resources::walls::decorator_storage::Decorators::iter_modified_ids` | 0x140a1b030 | 69 | Decorators::iter_modified_ids（[+0x60]/[+0x68]/[+0x78]） | O | wall\decomp\decomp-0x140a1b030.txt |
| `country_core::resources::walls::decorator_storage::Decorators::iter_modified` | 0x140a1b070 | 74 | Decorators::iter_modified | O | wall\decomp\decomp-0x140a1b070.txt |
| `country_core::resources::walls::decorator_storage::Decorators::iter_modified_mut` | 0x140a1b0b0 | 81 | Decorators::iter_modified_mut（闭包） | O | wall\decomp\decomp-0x140a1b0b0.txt |
| `country_core::resources::walls::decorator_storage::DecoratorStorage::iter_walls` | 0x140a1b100 | 89 | 访问器/转换 | I | wall\decomp\decomp-0x140a1b100.txt |
| `country_core::resources::walls::decorator_storage::Decorators::iter_removed_ids` | 0x140a1b100 | 89 | 访问器/转换 | I | wall\decomp\decomp-0x140a1b100.txt |
| `country_core::resources::walls::decorator_storage::Decorators::tracked_clone_from<country_core::resources::walls::decorator_storage::DecoMoveToStorage>` | 0x141256e40 | 104 | Decorators::tracked_clone_from（实例化 1） | O | wall\decomp\decomp-0x141256e40.txt |
| `country_core::resources::walls::decorator_storage::Decorators::tracked_clone_from<tuple$<> >` | 0x1412573b0 | 104 | Decorators::tracked_clone_from（实例化 2） | O | wall\decomp\decomp-0x1412573b0.txt |
| `country_core::resources::walls::decorator_storage::Decorators::clear<country_core::resources::walls::decorator_storage::DecoratorToStorageMap>` | 0x141257900 | 150 | Decorators::clear_modification_tracking | O | wall\decomp\decomp-0x141257900.txt |
| `country_core::resources::walls::decorator_storage::Decorators::insert<country_core::resources::walls::decorator_storage::DecoMoveToStorage>` | 0x14155afe0 | 159 | 累加/追加 | I | wall\decomp\decomp-0x14155afe0.txt |
| `country_core::resources::walls::decorator_storage::Decorators::mark_all_modified` | 0x140a1b140 | 233 | Decorators::mark_all_modified | O | wall\decomp\decomp-0x140a1b140.txt |
| `country_core::resources::walls::decorator_storage::Decorators::merge_with<country_core::resources::walls::decorator_storage::DecoMoveToStorage>` | 0x14155b170 | 237 | Decorators::merge_with（装饰集合合并） | O | wall\decomp\decomp-0x14155b170.txt |
| `country_core::resources::walls::decorator_storage::DecoratorMaybeMut::get` | 0x1400885d0 | 274 | WallId::get_inner（mov rax,[rcx] 返回内值 u64） | O | audio-misc\decomp\decomp-0x1400885d0.txt, wall\decomp\decomp-0x1400885d0.txt |
| `country_core::resources::walls::decorator_storage::DecoratorMaybeMut::get_mut` | 0x140a1b1e0 | 277 | DecoratorMaybeMut::get_mut（helper([+8],[+0x10]) 后返回 [+0] 指针） | O | wall\decomp\decomp-0x140a1b1e0.txt |
| `country_core::resources::walls::decorator_storage::impl$4::len` | 0x140a1b2c0 | 311 | Decorators 集合 len 访问器（mov rax,[rcx+0x20]） | O |  |
| `country_core::resources::walls::decorator_storage::DecoMoveToStorage::new` | 0x140a1b430 | 359 | DecoMoveToStorage::new（3 字段 {r8,r9,rdx}） | O | wall\decomp\decomp-0x140a1b430.txt |
| `country_core::resources::walls::decorator_storage::impl$8::remove` | 0x140a1b440 | 366 | 删除/清理 | U |  |
| `country_core::resources::walls::decorator_storage::impl$8::insert` | 0x140a1b4e0 | 374 | 累加/追加 | U |  |
| `country_core::resources::walls::decorator_storage::impl$8::update` | 0x140a1b5a0 | 384 | 键值集合 update（hashbrown 探针 + 0x14B 元素，修改追踪集合维护） | I |  |
| `country_core::resources::walls::decorator_storage::DecoratorStorage::verify_integrity` | 0x14003bb80 | 395 | decorator_subtype::queue（空函数 ret，no-op） | O | audio-misc\decomp\decomp-0x14003bb80.txt, camera-time\decomp\decomp-0x14003bb80.txt, collision-eco\decomp\decomp-0x14003bb80.txt |
| `country_core::resources::walls::decorator_storage::impl$9::insert` | 0x14003bb80 | 395 | decorator_subtype::queue（空函数 ret，no-op） | O | audio-misc\decomp\decomp-0x14003bb80.txt, camera-time\decomp\decomp-0x14003bb80.txt, collision-eco\decomp\decomp-0x14003bb80.txt |
| `country_core::resources::walls::decorator_storage::impl$10::remove` | 0x140a1b700 | 405 | 删除/清理 | U |  |
| `country_core::resources::walls::decorator_storage::_::impl$0::deserialize<enum2$<serde_json::value::Value> >` | 0x141560920 | 476 | Decorators::clone_from 闭包 | O | wall\decomp\decomp-0x141560920.txt |
| `country_core::resources::walls::decorator_storage::_::impl$0::deserialize<serde::__private::de::missing_field::MissingFieldDeserializer<serde_json::error::Error> >` | 0x1415608f0 | 476 | 序列化 | I | wall\decomp\decomp-0x1415608f0.txt |
| `country_core::resources::walls::decorator_storage::_::impl$0::serialize<ref_mut$<serde_json::ser::Serializer<ref_mut$<alloc::vec::Vec<u8,alloc::alloc::Global> >,serde_json::ser::CompactFormatter> > >` | 0x1400eef80 | 476 | 序列化 | U |  |
| `country_core::resources::walls::decorator_storage::_::impl$0::serialize<serde_json::value::ser::Serializer>` | 0x141560650 | 476 | 序列化 | U |  |
| `country_core::resources::walls::decorator_storage::DecoratorStorageAddr::as_wall` | 0x140a1c240 | 487 | DecoratorStorageAddr::as_wall（[+0]==0 → [+8]） | O | wall\decomp\decomp-0x140a1c240.txt |
| `country_core::resources::walls::decorator_storage::DecoratorStorageAddr::try_as_wall` | 0x140a1c290 | 496 | DecoratorStorage serde CompactFormatter | O | wall\decomp\decomp-0x140a1c290.txt |
| `country_core::resources::walls::decorator_storage::_::impl$0::deserialize::impl$0::visit_bytes<serde_json::error::Error>` | 0x141560c80 | 510 | 序列化 | U |  |
| `country_core::resources::walls::decorator_storage::_::impl$0::deserialize::impl$2::visit_enum<serde::__private::de::content::EnumRefDeserializer<serde_json::error::Error> >` | 0x141560dc0 | 510 | 序列化 | U |  |
| `country_core::resources::walls::decorator_storage::impl$15::from` | 0x1408bc520 | 532 | From 转换（xor eax,eax 零构造） | O |  |
| `country_core::resources::walls::decorator_storage::impl$16::from` | 0x140a1c2a0 | 536 | From 转换（单字段提取） | O |  |
| `country_core::resources::walls::decorator_storage::DecoratorStorage::mark_mutable` | 0x140a1c2b0 | 547 | DecoratorStorage::mark_mutable（清 +0x1b0 标志） | O | wall\decomp\decomp-0x140a1c2b0.txt |
| `country_core::resources::walls::decorator_storage::DecoratorStorage::get` | 0x140a1c2c0 | 550 | DecoratorStorage::get | O | wall\decomp\decomp-0x140a1c2c0.txt |
| `country_core::resources::walls::decorator_storage::DecoratorStorage::lookup_storage` | 0x140a1c3e0 | 558 | DecoratorStorage::lookup_storage（+0x78 的 0x18B {DecoratorId u32, ptr} 索引） | O | wall\decomp\decomp-0x140a1c3e0.txt |
| `country_core::resources::walls::decorator_storage::DecoratorStorage::try_lookup_storage` | 0x140a1c510 | 567 | DecoratorStorage::try_lookup_storage | O | wall\decomp\decomp-0x140a1c510.txt |
| `country_core::resources::walls::decorator_storage::DecoratorStorage::decorator_info` | 0x140a1c5f0 | 571 | DecoratorStorage::decorator_info | O | wall\decomp\decomp-0x140a1c5f0.txt |
| `country_core::resources::walls::decorator_storage::DecoratorStorage::decorator_info_mut` | 0x140a1c800 | 579 | DecoratorStorage::decorator_info_mut | O | wall\decomp\decomp-0x140a1c800.txt |
| `country_core::resources::walls::decorator_storage::DecoratorStorage::lookup_decorator_info` | 0x140a1c930 | 588 | lookup_decorator_info（0x98 步长墙 map 查 Decorators，内嵌 DecoratorId map@+0x30 再查） | O | wall\decomp\decomp-0x140a1c930.txt |
| `country_core::resources::walls::decorator_storage::DecoratorStorage::lookup_decorator_info_mut` | 0x140a1cbf0 | 594 | 查询/计算 | I | wall\decomp\decomp-0x140a1cbf0.txt |
| `country_core::resources::walls::decorator_storage::DecoratorStorage::get_or_insert_default` | 0x140a1cf30 | 623 | get_or_insert_default（21KB；!self.immutable 断言） | O | wall\decomp\decomp-0x140a1cf30.txt |
| `country_core::resources::walls::decorator_storage::DecoratorStorage::remove_storage` | 0x140a1dba0 | 670 | 删除/清理 | I | wall\decomp\decomp-0x140a1dba0.txt |
| `country_core::resources::walls::decorator_storage::DecoratorStorage::wall_decorators` | 0x140a1e190 | 701 | DecoratorArchivist::all_storages 闭包 | O | wall\decomp\decomp-0x140a1e190.txt |
| `country_core::resources::walls::decorator_storage::DecoratorStorage::iter_terrain_decorators` | 0x140a1e280 | 709 | iter_terrain_decorators（迭代器 {tag=1, ptr=self+0x90}） | O | wall\decomp\decomp-0x140a1e280.txt |
| `country_core::resources::walls::decorator_storage::DecoratorStorage::iter_stair_decorators` | 0x140a1e2c0 | 715 | iter_stair_decorators（迭代器 {tag=1, ptr=self+0x120}） | O | wall\decomp\decomp-0x140a1e2c0.txt |
| `country_core::resources::walls::decorator_storage::DecoratorStorage::wall_decorator_ids` | 0x140a1e300 | 721 | wall_decorators（墙集合访问） | O | wall\decomp\decomp-0x140a1e300.txt |
| `country_core::resources::walls::decorator_storage::DecoratorStorage::wall_decorators_mut` | 0x140a1e420 | 728 | DecoratorStorage::wall_decorators_mut | O | wall\decomp\decomp-0x140a1e420.txt |
| `country_core::resources::walls::decorator_storage::DecoratorStorage::iter_walls_mut` | 0x140a1e540 | 737 | iter_walls_mut | O | wall\decomp\decomp-0x140a1e540.txt |
| `country_core::resources::walls::decorator_storage::DecoratorStorage::iter` | 0x140a1e5b0 | 742 | iter（墙集合@+0x0 迭代器） | O | wall\decomp\decomp-0x140a1e5b0.txt |
| `country_core::resources::walls::decorator_storage::DecoratorStorage::iter_mut` | 0x140a1e630 | 759 | iter_mut | O | wall\decomp\decomp-0x140a1e630.txt |
| `country_core::resources::walls::decorator_storage::DecoratorStorage::iter_removed_storages` | 0x140a1e6e0 | 777 | iter_removed_storages（[+0x30]/[+0x38]/[+0x48]） | O | wall\decomp\decomp-0x140a1e6e0.txt |
| `country_core::resources::walls::decorator_storage::DecoratorStorage::clear_modification_tracking` | 0x140a1e720 | 783 | 删除/清理 | I | wall\decomp\decomp-0x140a1e720.txt |
| `country_core::resources::walls::decorator_storage::DecoratorStorage::on_walls_merged` | 0x140a1ea40 | 792 | on_walls_merged（21KB；panic d2s remove: no such） | O | wall\decomp\decomp-0x140a1ea40.txt |
| `country_core::resources::walls::decorator_storage::DecoratorStorage::on_wall_removed` | 0x140a1f550 | 839 | 系统:事件响应 | I | wall\decomp\decomp-0x140a1f550.txt |
| `country_core::resources::walls::decorator_storage::DecoratorStorage::on_wall_split` | 0x140a1f590 | 844 | on_wall_split（46KB；装饰迁移主逻辑） | O | wall\decomp\decomp-0x140a1f590.txt |
| `country_core::resources::walls::decorator_storage::DecoratorStorage::apply_decorator_remove` | 0x140a20670 | 927 | DecoratorStorage::apply_decorator_remove（装饰移除应用） | O | wall\decomp\decomp-0x140a20670.txt |
| `country_core::resources::walls::decorator_storage::DecoratorStorage::apply_decorator_change` | 0x140a20890 | 943 | apply_decorator_remove 闭包 | O | wall\decomp\decomp-0x140a20890.txt |
| `country_core::resources::walls::decorator_storage::DecoratorStorage::apply_decorator_place` | 0x140a20c70 | 1026 | DecoratorStorage::apply_decorator_place（装饰放置应用） | O | wall\decomp\decomp-0x140a20c70.txt |
| `country_core::resources::walls::decorator_storage::DecoratorStorage::restore_snapshot` | 0x140a20d70 | 1077 | take_snapshot（26KB；+0x1b0 门控 + 16B XOR 掩码 0x1429cd080） | O | wall\decomp\decomp-0x140a20d70.txt |
| `country_core::resources::walls::decorator_storage::DecoratorStorage::restore_decorators<country_core::systems::history::replay_pop_bucket::replay_pop_bucket::closure_env$0>` | 0x14155ba30 | 1115 | 记录/历史 | I | wall\decomp\decomp-0x14155ba30.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::walls::decorator_storage::Decorators::into_inner`（行 55）— 访问器/转换
- `country_core::resources::walls::decorator_storage::Decorators::decorators`（行 64）— 待推断
- `country_core::resources::walls::decorator_storage::Decorators::clear_modification_tracking`（行 99）— 删除/清理
- `country_core::resources::walls::decorator_storage::Decorators::insert`（行 159）— 构造/注册/生成
- `country_core::resources::walls::decorator_storage::Decorators::get`（行 181）— 查询/计算
- `country_core::resources::walls::decorator_storage::Decorators::get_mut`（行 186）— 查询/计算
- `country_core::resources::walls::decorator_storage::Decorators::remove`（行 192）— 删除/清理
- `country_core::resources::walls::decorator_storage::Decorators::iter_maybe_mut`（行 217）— 迭代/变换
- `country_core::resources::walls::decorator_storage::Decorators::iter_mut`（行 226）— 迭代/变换
- `country_core::resources::walls::decorator_storage::impl$14::from`（行 517）— 待推断
- `country_core::resources::walls::decorator_storage::DecoratorStorage::mark_immutable`（行 542）— 设置/写入
- `country_core::resources::walls::decorator_storage::DecoratorStorage::take_snapshot`（行 1059）— 待推断
- `country_core::resources::walls::decorator_storage::DecoratorStorageSnapshot::sort`（行 1240）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`clone_from`、`closure`、`default`、`deserialize`、`eq`、`expecting`、`fmt`、`hash`、`next`、`partial_cmp`、`visit_map`、`visit_seq`、`visit_str`、`visit_u64` 等 41 项
