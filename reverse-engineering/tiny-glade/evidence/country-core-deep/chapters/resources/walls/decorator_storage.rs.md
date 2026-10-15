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
| `country_core::resources::walls::decorator_storage::impl$8::remove` | 0x140a1b440 | 366 | **d2s 索引存在性守卫**（SIMD 探针 FUN_140b78140 结果 3=未命中 → fmt panic "d2s remove: no such {id}" fat 0x142ac8fb0；命中直接返回，删除在调用点） | O（候选U收尾闭合） | wall\decomp\decomp-0x140a1b440.txt, decomp-0x140b78140.txt |
| `country_core::resources::walls::decorator_storage::impl$8::insert` | 0x140a1b4e0 | 374 | **d2s 索引插入**（FUN_14152bc60 Arc 克隆插入；结果 3=成功返回，冲突 → fmt panic "d2s insert: already exists; {id}" fat 0x142ac8fe8，Display 闭包 0x140a24330/0x140a24370） | O（候选U收尾闭合） | wall\decomp\decomp-0x140a1b4e0.txt |
| `country_core::resources::walls::decorator_storage::impl$8::update` | 0x140a1b5a0 | 384 | 键值集合 update（hashbrown 探针 + 0x14B 元素，修改追踪集合维护） | I |  |
| `country_core::resources::walls::decorator_storage::DecoratorStorage::verify_integrity` | 0x14003bb80 | 395 | decorator_subtype::queue（空函数 ret，no-op） | O | audio-misc\decomp\decomp-0x14003bb80.txt, camera-time\decomp\decomp-0x14003bb80.txt, collision-eco\decomp\decomp-0x14003bb80.txt |
| `country_core::resources::walls::decorator_storage::impl$9::insert` | 0x14003bb80 | 395 | decorator_subtype::queue（空函数 ret，no-op） | O | audio-misc\decomp\decomp-0x14003bb80.txt, camera-time\decomp\decomp-0x14003bb80.txt, collision-eco\decomp\decomp-0x14003bb80.txt |
| `country_core::resources::walls::decorator_storage::impl$10::remove` | 0x140a1b700 | 405 | **self.map remove 断言**（探针未命中（状态 3）→ panic_str "assertion failed: self.map.remove(&id).is_some()" 0x142ac9068 line 0x30；命中返回） | O（候选U收尾闭合） | wall\decomp\decomp-0x140a1b700.txt |
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
## 实现方式（算法级）

### DecoratorStorage 布局（O，DIGEST §1.8 深化）

| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x00 | **per_wall**：HashMap<WallId, Decorators>，条目 **0x98B** = {key WallId u64@+0, value Decorators 0x90B@+8}；头 {ctrl@+0, mask@+8, len@+0x10, seed@+0x18, hasher@+0x20（FUN_1405b0d70 首参）} | get_or_insert_default 0x140a1cf30 探针 *-0x98、lookup_decorator_info 0x140a1c930 二段探针 |
| +0x30 | removed/目的地注册表（RawTable）：头 {ctrl@+0x30, mask@+0x38, len@+0x40, seed@+0x48, hasher@+0x50（FUN_1412e77b0 首参）}；terrain/stair 分支以**常数键 1（0x142ac9118）/ 2（0x142ac9128）**经 FUN_140b766e0（find_or_insert_slot）探它 | get_or_insert_default 两分支；iter_removed_storages 0x140a1e6e0 读 [+0x30]/[+0x38]/[+0x48] |
| +0x60..+0x90 | **decorator_to_storage** 索引：{bucket-array@+0x60, mask@+0x68, seed@+0x70, ctrl@+0x78, hasher@+0x80（FUN_1405b0ed0 首参）}，条目 **24B** = {u32 DecoratorId@0, 4B pad, u64 判别@8, u64 payload@16} | lookup_decorator_info 探针 *-0x18 + param_1[0xf]=+0x78 |
| +0x90 | **terrain**：内联 Option<Decorators>（懒初始化，首 qword≠0 即存在） | get_or_insert_default tag==1 分支写 0x90B@+0x90..+0x120 |
| +0x120 | **stairs**：内联 Option<Decorators>（同上） | tag==2 分支写 @+0x120..+0x1b0 |
| +0x1b0 | **immutable** u8 标志（param_1[0x36]，0x36×8=0x1b0）；mark_mutable 0x140a1c2b0 清零 | get_or_insert_default / apply_decorator_remove / on_wall_split / take_snapshot 均先检 |

### Decorators（0x90B，单墙装饰集合）布局（O）

| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x00 | removed_ids 集合头 {ctrl@0, mask@8, len@0x10}（首 qword 即"已初始化"门控） | 默认值首 qword 为 Arc 克隆的非空静态；get_or_insert_default 懒初始化判据 |
| +0x30 | **decorators 内嵌 map**：HashMap<u32 DecoratorId, DecoratorInfo 0x48B>，头 {ctrl@+0x30, mask@+0x38, len@+0x40, seed@+0x48, hasher@+0x50}，条目 **0x50B** = {key u32@0, 4B pad, DecoratorInfo 0x48B@8} | lookup_decorator_info 二段 *-0x50 探针 + lVar5+8 返回；apply_decorator_remove 内 FUN_140b77b20 |
| +0x60 | modified_ids 集合头 {ctrl@+0x60, mask@+0x68, len@+0x70/0x78} | iter_modified_ids 0x140a1b030 读 [+0x60]/[+0x68]/[+0x78]；apply_decorator_remove 经 FUN_140b776d0(+0x60) 移除 |
| +0x80 | map@+0x60 的 hasher | apply_decorator_remove FUN_1405b0ed0(lVar2+0x80) |

**默认 Decorators 构造算法（O）**：4 个 FUN_1400d9b20(0) 静态 Arc 克隆（每处 {lVar=*p; lVar2=p[1]; *p+=1} = Arc::clone 模式，源 = 静态空容器 _UNK_142c6bf00..18 与 _UNK_142c6bef0）+ 2 对静态常量 _UNK_142aa9130/38/40/48，共 18 qword = 0x90B 写入值区。

### get_or_insert_default 0x140a1cf30（21KB，O）

```
入口: if (self.immutable(+0x1b0) != 0) panic_str(&0x142ac9098/"assertion failed: !self.immutable", 0x21, file 0x142ac91b0)
dst_tag==0（wall）: key=param_3(WallId)；FUN_1405b0d70(param_1+0x20, &key) 哈希 → SIMD 探针（每槽 16B 组，H2 字节匹配 + -0x98 步长键比较）
  - 命中: return &entry+8（&Decorators）
  - 未中: 若 len==0 先 FUN_1428b36e0 扩容；构造默认 Decorators；写 key@entry+0 与值@entry+8..+0x98；len++ → return &value
dst_tag==1（terrain）/==2（stair）: 目的地**内联懒初始化**而非 map 探针：
  - if (qword@+0x90/+0x120 首字段 != 0) return &self+0x90/+0x120   // 已初始化
  - else: FUN_1412e77b0(+0x50, &const_key) 哈希（terrain 键=常量 1@0x142ac9118，stair 键=常量 2@0x142ac9128）
    → FUN_140b766e0(+0x30 map, hash, &const_key) 得插槽（alStack_148[0]==3=空槽）
  - 构造默认 Decorators 18 qword → 写 @+0x90..+0x120 或 @+0x120..+0x1b0 → return &值
```

### lookup_decorator_info 0x140a1c930（O，双层查表）

1. 若索引 ctrl@+0x78 == 0 → return 0。
2. 哈希 DecoratorId（FUN_1405b0ed0(+0x80)）→ SIMD 探针索引（**24B 条目** {u32 id@-0x18, ptrA@-0x10, ptrB@-8}）。
3. 条目判别 ptrA：**0=Wall**（ptrB=WallId，继查 per_wall map：哈希→0x98 步长探针→&entry+8=Decorators）、**1=Terrain**（内联 +0x90，种子@+0xd8）、**2=Stair**（内联 +0x120，种子@+0x168）。
4. 二段：在 Decorators 内嵌 map@+0x30 探针（0x50B 条目，key u32）；未命中或目标不存在 → return 0；命中 → return entry+8（&DecoratorInfo）。

### apply_decorator_remove 0x140a20670（O）

```
immutable 门控（panic 同 get_or_insert_default，file 0x142ac93c8）
args = param_3: {dst tag u32@0, id u64@8, decorator_id u32@0x10}
lVar2 = get_mut(0x140a1cde0)(self, dst, id)；若 null → goto 错误路径
FUN_1405b0ed0(lVar2+0x80) 哈希 → FUN_140b776d0(lVar2+0x60, hash, key)   // 从 modified_ids 集合移除
FUN_1402c9200(lVar2, key)                                                // 标记 removed
FUN_1405b0ed0(lVar2+0x50) → FUN_140b77b20(&out, lVar2+0x30, hash, key)   // 从内嵌 map 移除
  若未命中（状态字节==2）→ fmt panic "apply_decorator_remove: Trying to remove a decorator that doesn't exist: "（0x142ac9430）+ 拼接 id
命中: 把移除的 DecoratorInfo 0x48B 拷入输出结构（param_1，含 u32×8 区 + u8@+0x47 + 字节@+0x3f）
错误路径: panic_str(&0x142ac9068/"assertion failed: self.map.remove(&id).is_some()", 0x30, file)
```

### apply_decorator_place 0x140a20c70（O，2.1KB）

放置方向：immutable 门控 → 目标 Decorators（get_or_insert_default/get_mut）→ 把 DecoratorInfo 写入内嵌 map@+0x30 + 记入 modified_ids@+0x60；输出结构复刻放置的 DecoratorInfo（与 remove 对称）。

### on_walls_merged 0x140a1ea40（21KB，O）

```
immutable 门控（file 0x142ac92d0）
param_2/param_3 = 两墙 WallId；param_4 = 回调/参数（code*）
对两墙分别: FUN_1405b0d70(+0x20) 哈希 → FUN_140b78400(&stack, self, hash, key) 得 per_wall 条目 → func_0x142923c00 0x90B memmove 拷到栈（×2 墙）
Decorators::merge_with 0x14155b170(&result, &wallA, &wallB)   // 合并两集合（见下）
FUN_1405b0d70 → FUN_140b78400 再探 per_wall（结果写回槽）→ 源墙条目清理
若需新建: 默认 Decorators 构造（Arc 克隆 UNK_142c6bf00..18 + _UNK_142aa9130..48）→ FUN_1428b36e0 扩容 → 写 0x98B 条目
尾部多次 0x88B memmove（merge_with 输出记录 0x88B，与 0x90B 的 Decorators 略异）
```

### on_wall_split 0x140a1f590（46KB，O 主算法）

```
immutable 门控 → panic !self.immutable（line 0x21）
param_2=原墙 WallId，param_4=分裂 u（uStack_1a8）
per_wall 探针取原墙 Decorators（0x90B 拷栈）
遍历装饰记录（0x28B 步长条目）：逐条读 {u32 id@-0x28, 8B×4@-0x24..-0x10, u32×2@-0x8..}
  - 每条读状态字节 b@记录-0x12（(b-2)>2 → lVar17=0，否则 lVar17=b-1，即 b∈0..4 映射子侧 0..3）
  - if (lVar17==0): fVar36 = FUN_141469a20(split_u)（approximate_curve_u）→ clamp [0,1]（0.0 / 1.0f@0x142925904）
    → 在 {low,high} f32 对数组（16B 条目，逐 0x10 步进，两 f32@+8/+0xc）做**区间包含搜索**判定分裂点落哪个 u 区间/侧
  - 子侧构建: FUN_141524c50(...) → 目标子墙 map 探针（0x98 步长）；未中 → 默认 Decorators 构造 + 插入子墙 per_wall + FUN_1402c9200 标 removed
  - 已存在分支: 直接 FUN_1402c9200(entry-3, id) 标 removed；另一子侧分支需 lVar17==1 否则 panic "internal error: entered unreachable code"（0x142ac8d80, line 0x28, file 0x142ac9380）
尾部: 原墙条目删除（memmove 0x10B/0x90B 元素）、目标墙 map 扩容（FUN_1428b36e0）、默认值写入
panic 二: "assertion failed: self.decorator_to_storage.map.remove(&decorator_id).is_some()"（0x142ac9300, line 0x4f, file 0x142ac9350）→ 装饰记录二段内嵌 map 移除未中
```

### take_snapshot / restore_snapshot 0x140a20d70（26KB，O）

```
前置 6 断言（panic_str，file 均 decorator_storage.rs）：
  0x142ac9530 "assertion failed: per_wall.is_empty()"  line 0x25
  0x142ac9570 "assertion failed: terrain.is_none()"     line 0x23
  0x142ac95b0 "assertion failed: stairs.is_none()"      line 0x22
  0x142ac95f0 "assertion failed: removed.is_empty()"    line 0x24
  0x142ac9630 "assertion failed: decorator_to_storage.map.is_empty()" line 0x35
  + 0x142ac9098 "!self.immutable" line 0x21
逐墙: FUN_140a1cf30(param_1, dst_tag, id) 取 Decorators（param_1=self）
装饰状态判别跳表: 0x142ac9bdc / 0x142ac9bfc（按 u8 状态字节 bVar13/bVar30 索引的 int 偏移表）
16B XOR 掩码: _UNK_1429cd080 / _UNK_1429cd090（XMM 两半）
ID 解码: FUN_140b776d0(map, (uVar18<<0x20|uVar18>>0x20) ^ mask, key) —— 快照里 DecoratorId 以**字节旋转 XOR 16B 掩码**编码，恢复时旋转还原后移除
FUN_1402c9200 标 removed；FUN_1411fbfa0 辅助写输出
fmt panic 尾部（FUN_140a24330/FUN_140a24370 两个 Display 闭包 + 0x142c6e228 file）
```

### Decorators::merge_with 0x14155b170（24KB，O）

```
自 map@+0x30（hasher +0x50）与源 Decorators（param_3）内嵌 map 双向遍历（0x50B 条目）
逐 DecoratorId: FUN_1405b0ed0 哈希 → FUN_140b78140(&out, map, hash, key) 探源/探自
  - 源有自无 → Arc 克隆 DecoratorInfo（FUN_14152bc60）插入
  - 冲突 → FUN_1402c9200(param_1, id) 标 removed（去重）
fmt panic: 0x142ac8fb0（消息，file 0x142c6e288 / 0x142c6e2a0）
```

### tracked_clone_from 0x141256e40 / 0x1412573b0（各 20KB，O）

```
0x141256e40（DecoMoveToStorage 实例）: FUN_141262d50(+0x0/…) 逐字段克隆；FUN_141263600(+0x30, +0x30) 克隆内嵌 map；
  遍历 removed_ids/内嵌 map → FUN_1402c9200 标 removed；FUN_140a1b440 / FUN_140a1b4e0（往追踪集合 insert，panic 常量 0x142c21300/0x142c21318）
0x1412573b0（tuple$<> 实例）: 同骨架，第二个泛型实例化（差异仅插入闭包参数类型）
```

### clear_modification_tracking 0x141257900（O）

```
if (modified 集 len@+0x78 != 0): 释放元素区（0xff 填充 +0x11 模式）→ len=0 → 重算 cap（(n+1&~7)-(n+1>>3) 哈希表容量公式）
清 decorators map@+0x30（FUN_1428c42c0(param_1, cap, +0x20, 1) = hashbrown RawTable::clear）
```

### 迭代器族（O）

- `iter_terrain_decorators` 0x140a1e280 = 迭代器 {tag=1, ptr=self+0x90}（Option 迭代）
- `iter_stair_decorators` 0x140a1e2c0 = {tag=1, ptr=self+0x120}
- `iter` 0x140a1e5b0 = per_wall map@+0x0 迭代器；`iter_mut` 0x140a1e630、`iter_walls_mut` 0x140a1e540
- `iter_removed_storages` 0x140a1e6e0 = [+0x30]/[+0x38]/[+0x48]（removed 集合迭代）
- `Decorators::iter_modified_ids` 0x140a1b030 = [+0x60]/[+0x68]/[+0x78]；`iter_modified` 0x140a1b070、`iter_modified_mut` 0x140a1b0b0、`mark_all_modified` 0x140a1b140
- `DecoMoveToStorage::new` 0x140a1b430 = 3 字段 {r8, r9, rdx} 结构构造
- `DecoratorStorageAddr::as_wall` 0x140a1c240 = [+0]==0 → [+8]

### archive_current_decorators_in_storage 0x14123b5c0（84KB，O 结构）

```
参数 7+（self 归档目标 + DecoratorStorage + 键/闭包）
FUN_140a1c2c0(param_4) = DecoratorStorage::get 取源存储
静态默认容器（不同单形化的空表）: _UNK_142fcb240..258 / _UNK_142c19540..58（Arc 克隆）
闭包表: UNK_14003bad0 + UNK_142c1ef28（迭代/回调函数指针）
逐墙: 0x90B Decorators 拷贝（func_0x142923c00）→ FUN_1405b0d70(+0x20) 哈希 → 若未中 FUN_1428b36e0 扩容 → 写 0x98B 条目（key WallId + 值 0x90B @ -0x90 相对寻址）
内嵌 map 二段: FUN_1405b0ed0 哈希 → FUN_1408cdb50 / 0x1408cdbd0（u32 键辅助）
panic: FUN_1428d8fe3(uVar23, param_7, &UNK_142c1ef98)（fmt）；分配失败 FUN_1428d80d0(&UNK_142c194a8)
```

### 新常量表

| 常量 | 地址 | 语义 |
|---|---|---|
| "assertion failed: !self.immutable" | 0x142ac9098 | immutable 门控 |
| "assertion failed: self.map.remove(&id).is_some()" | 0x142ac9068 | apply_decorator_remove 移除未中（line 0x30） |
| "assertion failed: self.decorator_to_storage.map.remove(&decorator_id).is_some()" | 0x142ac9300 | on_wall_split 二段移除（line 0x4f） |
| "internal error: entered unreachable code" | 0x142ac8d80 | on_wall_split 子侧分支（line 0x28） |
| "apply_decorator_remove: Trying to remove a decorator that doesn't exist: " | 0x142ac9430 | fmt panic 前缀 |
| "assertion failed: per_wall.is_empty()" / "terrain.is_none()" / "stairs.is_none()" / "removed.is_empty()" / "decorator_to_storage.map.is_empty()" | 0x142ac9530/70/b0/f0/630 | take_snapshot 前置 |
| 16B XOR 掩码 | 0x1429cd080 / 0x1429cd090 | 快照 DecoratorId 编码 |
| 常数键 1 / 2 | 0x142ac9118 / 0x142ac9128 | terrain/stair 目的地键 |
| 静态空容器 | 0x142c6bf00..0x18 / 0x142c6bef0 / 0x142aa9130..48 / 0x142fcb240..258 / 0x142c19540..58 | 默认 Decorators/空表 Arc 克隆源 |
| file 路径 | 0x142ac91b0 等 | crates/country-core/src/resources/walls/decorator_storage.rs |

### 跨模块调用

FUN_140a1cde0（get_mut）、FUN_14155b170（merge_with）、FUN_1402c9200（removed 标记）、FUN_141469a20（approximate_curve_u，utils::curve 域）、FUN_140b78140/FUN_140b78400/FUN_140b766e0/FUN_140b776d0/FUN_140b77b20（hashbrown 探针/移除族）、FUN_1405b0d70/FUN_1405b0ed0（哈希，seed 随 0x20/0x50/0x80 偏移）、FUN_141529bb0/FUN_1402ca2f0（map 遍历/工厂，inner_walls 共用）、FUN_1400d9b20（静态 Arc 克隆）、FUN_1428d9430（panic_str）、FUN_1428d9390（fmt panic）、FUN_140106160（4B memcpy）、FUN_1411fbfa0、FUN_1408cdb50/0x1408cdbd0（u32 键哈希）。

### 遗留 U

- +0x30 map 与 removed 集合的关系：get_or_insert_default 的 terrain/stair 分支经它做插槽探测（常数键），而 iter_removed_storages 读同一区——两者是否同一结构未定（I：疑为"目的地→槽位"注册表兼 removed 集合，需 on_wall_removed 0x140a1f550 佐证）。
- roof 目的地的 Decorators 所在集合（per_wall 之外的第 4 个目的地）未在本次涉及函数中出现，U。
- on_wall_split 的 0x28B 装饰记录（u 区间/子侧状态字节）完整字段语义 U（部分推断）。
- Decorators 内 +0x0..+0x30 首集合与 +0x18/+0x20/+0x28 字段逐字段名 U。
