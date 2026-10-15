# resources/walls/decorator_archivist.rs

> 文件作用：装饰归档/剪枝资源：墙合并/分裂/擦除前对墙属装饰做快照归档，提供 cull 放行策略（任意/地形/楼梯）与 before/after 差异提取；interactions 集合在 +0x90。

函数总计 43：业务 22 / 内联无独立符号 8 / 派生胶水 13

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::walls::decorator_archivist::DecoratorArchivist::archive_storage_decorators<ref$<str$> >` | 0x1412b2a70 | 0 | archive_storage_decorators 实例化 | O | wall\decomp\decomp-0x1412b2a70.txt |
| `country_core::resources::walls::decorator_archivist::DecoratorArchivist::extract_before_after_impl` | 0x140b2be70 | 0 | archive_storage_decorators | O | wall\decomp\decomp-0x140b2be70.txt |
| `country_core::resources::walls::decorator_archivist::_::impl$0::apply` | 0x14003bb80 | 56 | decorator_subtype::queue（空函数 ret，no-op） | O | audio-misc\decomp\decomp-0x14003bb80.txt, camera-time\decomp\decomp-0x14003bb80.txt, collision-eco\decomp\decomp-0x14003bb80.txt |
| `country_core::resources::walls::decorator_archivist::_::impl$0::init_state` | 0x140b30a60 | 56 | bevy_ecs SystemParam::init_state（依赖 derive 胶水；调 bevy_ecs 参数状态初始化器 FUN_14136ca50/FUN_14134c460） | 胶水排除 | wall\decomp\decomp-0x140b30a60.txt（候选U收尾闭合） |
| `country_core::resources::walls::decorator_archivist::_::impl$0::new_archetype` | 0x14003bb80 | 56 | decorator_subtype::queue（空函数 ret，no-op） | O | audio-misc\decomp\decomp-0x14003bb80.txt, camera-time\decomp\decomp-0x14003bb80.txt, collision-eco\decomp\decomp-0x14003bb80.txt |
| `country_core::resources::walls::decorator_archivist::_::impl$0::queue` | 0x14003bb80 | 56 | decorator_subtype::queue（空函数 ret，no-op） | O | audio-misc\decomp\decomp-0x14003bb80.txt, camera-time\decomp\decomp-0x14003bb80.txt, collision-eco\decomp\decomp-0x14003bb80.txt |
| `country_core::resources::walls::decorator_archivist::_::impl$0::validate_param` | 0x1403f7af0 | 56 | bevy_ecs SystemParam::validate_param（依赖 derive 胶水；校验参数索引表 +0x210/+0x218/+0x1e0，输出 Option\<BevyError\>；串 "Resource does not exist" 0x1429d62a0 / 参数类型名拼接 0x142947223） | 胶水排除 | wall\decomp\decomp-0x1403f7af0.txt（候选U收尾闭合） |
| `country_core::resources::walls::decorator_archivist::DecoratorArchivist::archive_storage_decorators<alloc::string::String>` | 0x1412b3910 | 66 | archive_storage_decorators 实例化 | O | wall\decomp\decomp-0x1412b3910.txt |
| `country_core::resources::walls::decorator_archivist::DecoratorArchivist::archive_current_decorators_in_storage<ref$<str$> >` | 0x14123b5c0 | 207 | archive_current_decorators_* 实例化（84KB） | O | wall\decomp\decomp-0x14123b5c0.txt |
| `country_core::resources::walls::decorator_archivist::DecoratorArchivist::archive_current_decorators_in_storages<ref$<str$> >` | 0x1412b4f10 | 233 | archive_current_decorators_* 实例化（46KB） | O | wall\decomp\decomp-0x1412b4f10.txt |
| `country_core::resources::walls::decorator_archivist::DecoratorArchivist::all_decorators` | 0x140b2bb00 | 251 | validate_param（pmovmskb 零字节位图 → 校验描述符） | O | wall\decomp\decomp-0x140b2bb00.txt |
| `country_core::resources::walls::decorator_archivist::DecoratorArchivist::all_storages` | 0x140b2bb80 | 268 | DecoratorArchivist::all_storages（遍历五集合） | O | wall\decomp\decomp-0x140b2bb80.txt |
| `country_core::resources::walls::decorator_archivist::DecoratorArchivist::wall_decorators` | 0x140a1e190 | 284 | DecoratorArchivist::all_storages 闭包 | O | wall\decomp\decomp-0x140a1e190.txt |
| `country_core::resources::walls::decorator_archivist::DecoratorArchivist::archived_walls` | 0x140a1b100 | 288 | PublicWalls::wall_ids | I | wall\decomp\decomp-0x140a1b100.txt |
| `country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_any` | 0x140b2bbf0 | 294 | decorator_cull_allow_any 状态置位（byte[+0x1e0]=1） | O | wall\decomp\decomp-0x140b2bbf0.txt |
| `country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_wall` | 0x140b2bc00 | 312 | decorator_cull_allow_wall（round3 round2 复查修正：原“cull_allow_any”误标；PDB exact=cull_allow_wall） | O | wall\decomp\decomp-0x140b2bc00.txt |
| `country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_stairs` | 0x140b2bd30 | 340 | decorator_cull_allow_stairs（标志读取） | O | wall\decomp\decomp-0x140b2bd30.txt |
| `country_core::resources::walls::decorator_archivist::DecoratorArchivist::is_decorator_cull_allowed` | 0x140b2bd90 | 359 | 谓词/校验 | I | wall\decomp\decomp-0x140b2bd90.txt |
| `country_core::resources::walls::decorator_archivist::DecoratorArchivist::extract_and_restore_to_storage` | 0x140b2c4b0 | 461 | extract_before_after_all | O | wall\decomp\decomp-0x140b2c4b0.txt |
| `country_core::resources::walls::decorator_archivist::DecoratorArchivist::extract_diff` | 0x140b2cb30 | 513 | DecoratorArchivist::extract_diff（before/after 差异提取） | O | wall\decomp\decomp-0x140b2cb30.txt |
| `country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_colliders` | 0x140005c60 | 583 | extract_diff 闭包（环境 +0x60） | O | wall\decomp\decomp-0x140005c60.txt |
| `country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_interactions` | 0x1403c03c0 | 589 | decorator_interactions（返回 &self+0x90） | O | wall\decomp\decomp-0x1403c03c0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::walls::decorator_archivist::_::impl$0::get_param`（行 56）— 查询/计算
- `country_core::resources::walls::decorator_archivist::DecoratorArchivist::archive_storage_decorators`（行 66）— 待推断
- `country_core::resources::walls::decorator_archivist::DecoratorArchivist::archive_current_decorators_in_storage`（行 207）— 待推断
- `country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow`（行 302）— 待推断
- `country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_terrain`（行 329）— 待推断
- `country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_clear`（行 351）— 待推断
- `country_core::resources::walls::decorator_archivist::DecoratorArchivist::extract_before_after_all`（行 430）— 待推断
- `country_core::resources::walls::decorator_archivist::DecoratorArchivist::extract_archived_decorators`（行 474）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 13 项
## 实现方式（算法级）

### DecoratorArchivist 结构（O 骨架）

- 存储 before/after 两组 Decorators：+0xc0 与 +0x150 区各含 0x90B Decorators（extract_before_after_all 0x140b2c4b0 / extract_diff 0x140b2cb30 均 memmove 0x90 自 param+0x18(+0xc0) 与 param+0x2a(+0x150)）。
- interactions 集合 @+0x90（decorator_interactions 0x1403c03c0 手解：返回 &self+0x90）。
- all_storages 遍历的五集合：archivist 内多张 map（+0xc0、+0x150 及更多；迭代器描述符 tag 由 +0xc0/+0x150 的 len 是否为空决定）。
- cull 状态位 @+0x1e0（decorator_cull_allow_any 0x140b2bbf0 置 byte=1，手解）。

### all_decorators 0x140b2bb00（O，手解）

```
签: all_decorators(&self, out) → 构建迭代器描述符
r8 = input[0] + input[8] + 1（桶数组末端）
movdqa xmm0,[input[0]] → pmovmskb → not → u16 非零字节位图 @ out+0x38
out+0x20 = ctrl ptr；out+0x28 = ctrl+0x10；out+0x30 = r8；out+0x40 = input[0x18]
out+0x48 = input+0xc0 或 0（len==0 时 cmove 0）；out+0x50 = input+0x150 或 0
```
即用 pmovmskb 对 ctrl 块生成"非空槽"位图，构成迭代器/校验描述符。**纠错**：PDB 索引将该地址命名为 all_decorators（非任务稿所称 validate_param）；validate_param 真身在 0x1403f7af0（胶水排除=bevy_ecs SystemParam derive，候选U收尾闭合，见 round3/WALL-CANDIDATE-U-CLOSURE.md）。

### all_storages 0x140b2bb80（O，1.5KB）

```
签: all_storages(&self, map) → out 迭代器描述符
out[0]  = (param_2[0x2a]/*+0x150*/ == 0) + 2        // 变体 2/3（+0x150 集合空否）
out[2]  = (param_2[0x18]/*+0xc0*/ == 0) * 2 + 1     // 变体 1/3（+0xc0 集合空否）
out[4]  = ctrl ptr；out[5] = ctrl+0x10；out[6] = ctrl+mask+1；out[7] = ~非零槽位图；out[8] = len
```
遍历 archivist 的（五）集合 map，编码成带双判别（0x150/0xc0 空态）的迭代器描述符。

### decorator_cull_allow_wall 0x140b2bc00（O；PDB 名 round3 修正 = cull_allow_wall）

```
签: (out param_1, key param_2 16B, map param_3)
if (map len@+0x18 != 0):
  FUN_1405b0d70(+0x20) 哈希 key → SIMD 探针（0x98B 条目，键 16B 比较）
  命中: 建回调环境（param_1+0x30 = {ctrl, mask, len+1}）→ FUN_140b76260(&env, 值@entry+0x50, &out)
    （回调遍历该墙 Decorators 内嵌 map 并置 cull 放行标志）
```

### decorator_cull_allow_any 0x140b2bbf0（O，手解）

单字节置位：`byte[self+0x1e0] = 1`（与 cull_allow_wall 同族，any 语义 = 放行全部）。

### decorator_cull_allow_stairs 0x140b2bd30（O）

标志读取（cull 状态位判别），与 cull_allow_terrain 0x140b2b…（内联族，见章节表 0x140b2bd30 同一区）。

### archive_storage_decorators 0x140b2be70（16KB，O；PDB 名 extract_before_after_impl 实为 archive_storage_decorators 的 extract 路径）

```
签: (out, 存储 map param_2, key param_3, …, param_7 bool 预览判定)
if (param_2[3]/*len@+0x18*/ == 0) → return（空）
FUN_1405b0d70(+0x20) 哈希 key → SIMD 探针（0x98B 条目）
if ((param_2 != 0 & (param_7^1)) == 0) → 预览装饰物路径分支
BeVy Query 访问: FUN_140d4d740()（查询迭代器）+ FUN_140d64e00()（计数/推进）——系统级函数
静态默认块 _UNK_142aeb0f0..108
panic:
  0x142aeb540 "Found a preview decorator while trying to extract history diff. "（fmt，file 0x142aeb5a0）
  0x142aeb620 "Decorator `DecoratorStorage` 'after' state exists for wall "（fmt，file 0x142aeb650）
```

### extract_before_after_all 0x140b2c4b0（10KB，O；PDB 名 extract_and_restore_to_storage）

```
签: (out param_1, self param_2)
FUN_140b79960(param_1+0x60/param_1+0x90)（清除两处 before/after 集合）
0x90B memmove: before←self+0xc0、after←self+0x150
静态默认迭代器态 _UNK_142af4bb0..c8
双集合遍历（0x88B 记录 + 0x98B map 条目）: 逐记录 FUN_140897ea0 处理
get_or_insert_default 0x140a1cf30 → Decorators；FUN_14155a950（合并/写回）
0x50B 步长内嵌条目 + 0x10B memmove 维护
```

### extract_diff 0x140b2cb30（8KB，O）

```
签: (out param_1, self param_2, param_3, closed param_4)
读 self+0xc0/+0x150 两组（before/after），0x90B memmove ×2；静态默认块 _UNK_142af4bb0..c8 / _UNK_142c5f830..848
0x88B 记录迭代 + FUN_141449c20 辅助；逐条 diff
panic 三（file 均 0x142aeb698/6e0/728 = decorator_archivist.rs）：
  0x142aeb668 "assertion failed: wall_decorators.is_empty()"   line 0x2c
  0x142aeb6b0 "assertion failed: terrain_decorators.is_none()" line 0x2e
  0x142aeb6f8 "assertion failed: stair_decorators.is_none()"   line 0x2c
diff 语义: 提取 before→after 差异（新增/移除/改动），输出记录供历史 undo/redo
```

### 新常量表

| 串/常量 | 地址 | 语义 |
|---|---|---|
| "Found a preview decorator while trying to extract history diff. " | 0x142aeb540 | archive_storage_decorators fmt |
| "Decorator `DecoratorStorage` 'after' state exists for wall " | 0x142aeb620 | 同上 |
| "assertion failed: wall_decorators.is_empty()" | 0x142aeb668 | extract_diff |
| "assertion failed: terrain_decorators.is_none()" | 0x142aeb6b0 | extract_diff |
| "assertion failed: stair_decorators.is_none()" | 0x142aeb6f8 | extract_diff |
| cull 状态位 | self+0x1e0 | decorator_cull_allow_any |
| 迭代器静态默认 | 0x142af4bb0..c8 / 0x142c5f830..848 | extract 族 |

### 跨模块调用

FUN_1405b0d70/FUN_140b78400（哈希/探针）、FUN_140b76260（cull 回调）、FUN_140b79960（集合清空）、FUN_140897ea0、FUN_14155a950、FUN_140a1cf30（get_or_insert_default）、FUN_141449c20、FUN_140d4d740/FUN_140d64e00（BeVy Query）、FUN_1428d9390/FUN_1428d9430（panic）。归档数据跨域供 history（undo/redo）与 merge_walls/splitter/eraser 消费。

### 遗留 U

- archivist 五集合的完整偏移图（除 +0x90 interactions、+0xc0/+0x150 before/after、+0x1e0 cull 位）U。
- extract_diff 输出记录格式（0x88B）的字段语义 I。
- validate_param（0x1403f7af0）= bevy_ecs `SystemParam::validate_param` derive 胶水（已闭合，候选U收尾）；与 all_decorators 0x140b2bb00 的索引名字错位为 PDB 编制问题，无业务关系。
