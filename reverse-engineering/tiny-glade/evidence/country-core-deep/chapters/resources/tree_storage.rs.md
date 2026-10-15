# resources/tree_storage.rs

> 文件作用：**树木放置存储资源**——`TreeStorage` 是一个 16 字节条目的哈希表容器（存 `TreeId→Tree`），
> 附一个自增序号计数器（+0x30）；另有 `InteractableForestPlacements`（可交互森林放置位点）与 `BirdHaven` 组件。
> 消费方：树木系统（spawn/despawn）、critters（BirdHaven）。
> 布局（从 get/insert/remove/next_index/iter 机器码 + 反编译）：哈希表域在 +0x00..+0x18
> （条目基址\*+0x0、bucket mask +0x8、len +0x18），表操作经共享 hash helper
> FUN_1405b0d70（hash/probe）与 FUN_140b76f90（remove）；+0x30 = next_index 计数器。

函数总计 11：业务 7 / 内联无独立符号 2 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::tree_storage::TreeStorage::iter` | 0x140a1b100 | 54 | 迭代器构造：`{cur@+0, next@+8, end@+0x10, occupied_mask@+0x18}`；对每个 16 字节槽取高比特位（pmovmskb + NOT）得到占用位图，cur/next 步进 0x10 | O | roofs-rt/decomp/decomp-0x140a1b100.txt（本域反编译；原 wall\decomp 同名文件实为 PublicWalls::wall_ids 的 NotFound 副本，已弃用） |
| `country_core::resources::tree_storage::TreeStorage::get` | 0x140a38460 | 58 | 哈希查找：FUN_1405b0d70(self+4, &key) 得 hash → bucket 探测（16 字节 key 逐字节比较构建匹配位图）→ 命中返回条目值指针（entry+8），未命中/空表返回 0 | O | roofs-rt/decomp/decomp-0x140a38460.txt |
| `country_core::resources::tree_storage::TreeStorage::insert` | 0x140a38560 | 66 | `self.max@+0x30 = max(max@+0x30, id+1)`；返回由 FUN_140eec970（id 生成器）产出的 u64 句柄，高 32 位为 0 时返回 0（None 哨兵） | O | roofs-rt/decomp/decomp-0x140a38560.txt |
| `country_core::resources::tree_storage::TreeStorage::remove` | 0x140a385a0 | 72 | 哈希删除：FUN_1405b0d70(self+0x20,&key) + FUN_140b76f90 移除；返回被删 Tree（CONCAT44） | O | roofs-rt/decomp/decomp-0x140a385a0.txt |
| `country_core::resources::tree_storage::TreeStorage::next_index` | 0x140a38600 | 77 | 机器码手解：`mov rax,[rcx+0x30]; lea rdx,[rax+1]; mov [rcx+0x30],rdx; ret` —— 自增并返回 +0x30 计数器 | O | 手解（rne-read --hex） |
| `country_core::resources::tree_storage::impl$11::register_required_components` | 0x140a3d320 | 116 | Bevy 注册胶水：FUN_140d40d70 注册组件类型 + FUN_140935f60 构造 required components 追加到 Vec | O（胶水） | roofs-rt/decomp/decomp-0x140a3d320.txt |
| `country_core::resources::tree_storage::impl$12::register_required_components` | 0x140a3da10 | 131 | 同上（另一组件） | O（胶水） | roofs-rt/decomp/decomp-0x140a3da10.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::tree_storage::InteractableForestPlacements::clear`（行 110）— 清空放置列表（Vec clear）
- `country_core::resources::tree_storage::BirdHaven::new`（行 147）— 构造（零化）

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 2 项（TreeStorage::default / InteractableForestPlacements::default，derive Default）

## 关键机器码（iter 0x140a1b100）

```
mov rax,rcx; mov rcx,[rdx]; mov r8,[rdx+8]; add r8,rcx; dec r8
movdqa xmm0,[rcx]; pmovmskb r9d,xmm0; not r9d
mov [rax+0x18],r9w; mov [rax],rcx; add rcx,0x10; mov [rax+8],rcx; mov [rax+0x10],r8
```
16 字节条目同时充当 ctrl/占用位（高比特），符合树存储稀疏表设计。

## 实现方式（算法级）

- `iter`（0x140a1b100）：迭代器 `{cur@+0, next@+8, end@+0x10, occupied@+0x18}`；16B 槽取高位（pmovmskb+NOT）得占用位图；cur/next 步进 0x10。
- `get`（0x140a38460）：FUN_1405b0d70 哈希 → 桶探测（16B key 逐字节比较位图）→ 命中返回 entry+8 值指针；未命中/空表返回 0。
- `insert`（0x140a38560）：`max(+0x30, id+1)` 更新 + FUN_140eec970 句柄（高 32 位 0 → None 哨兵）。
- `remove`（0x140a385a0）：FUN_1405b0d70(self+0x20) + FUN_140b76f90 移除；返回被删 Tree。
- `next_index`（0x140a38600）：`mov rax,[rcx+0x30]; lea rdx,[rax+1]; mov [rcx+0x30],rdx` —— 自增返旧值。
