# asset_libraries/audio_trigger_library.rs

> 文件作用：**音频触发盒（AABB）库**——`AudioTriggerLibrary = [AudioTriggerCollection; 1]`（按 kind 索引的 SlotMap 数组），
> 每个 `AudioTriggerCollection` 是 `slotmap::SlotMap<AudioTriggerHandle, Aabb3>`（触发器盒集合）。消费者为
> `trigger_audio_system`（0x1409eafa0，PDB 名 ICF 折叠为 `handplaced_tree_animation`）。证据：`evidence/country-core-audit/A3_color.md` §三（O）。

函数总计 10：业务 2 / 内联无独立符号 5 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `AudioTriggerCollection::boxes` | 0x14157ce40 | 27 | 猜测：返回盒列表；证实：`SlotMap` 迭代器构造 | **O**（67 B）：返回 4 qword `{ptr, ptr+len*0x1c, (len!=0), num_elems}` = `slotmap::basic::Iter`（`slots.iter().enumerate()` 跳过哨兵槽 0，起始下标 1）。槽 stride 0x1c = 24 B Aabb3 + 4 B version。唯一调用方 `trigger_audio_system` 按 `[slot+0x18]&1` 判占用、`DecoratorRank::new(idx)` 取键、前 0x18 B 喂 `Aabb3::intersect_segment` | A3_color.md §三 |
| `AudioTriggerLibrary::get_triggers` | 0x14157ce90 | 84 | 猜测：取触发器集合；证实：`kind` 下标守卫 + 数组元素引用 | **O**（42 B）：`rdx==0` → 返回 `rax=rcx`；`rdx!=0` → `panic!("assertion failed: kind != AudioTriggerKind::COUNT")`（行 85 列 9）。调用点传 `edx=0` → `COUNT==1`，库 = `[Collection; 1]`；`insert_resource<AudioTriggerLibrary>` 搬 0x20 B | A3_color.md §三 |

## 内联/无独立符号函数

- `AudioTriggerCollection::clear`（行 23）— **I**：SlotMap 清空（A3_color 无独立符号，语义按 slotmap 常规推断）
- `AudioTriggerLibrary::add_trigger`（行 55）— **I**：插入新盒（内联于 prefab 系统，无符号；`AudioTriggerHandle ↔ u32` 恒等经 `impl$3::from` 0x140833250 = `mov eax,ecx; ret`，O）
- `AudioTriggerLibrary::update_trigger_shape`（行 63）— **I**：按 handle 改写盒形状（无独立符号）
- `AudioTriggerLibrary::remove_trigger`（行 70）— **I**：按 handle 删除（无独立符号）
- `AudioTriggerLibrary::clear`（行 76）— **I**：全库清空（无独立符号）

## 派生胶水（3 项：impl$2::default、impl$18::eq、impl$22::default）

- 均为 derive(Default/Eq) 展开（行 15/33/42），不补业务。

## 布局表（O，A3_color）

| 结构 | 布局 | 证据 |
|---|---|---|
| `AudioTriggerCollection` | `slotmap::SlotMap<AudioTriggerHandle, Aabb3>`：slots Vec @0（cap-first），num_elems u32 @0x1c；槽 24 B Aabb3 + 4 B version | boxes 0x14157ce40 |
| `AudioTriggerLibrary` | `[AudioTriggerCollection; 1]`，总 0x20 B（insert_resource call_once 搬 0x20） | get_triggers 0x14157ce90 |
| `AudioTriggerHandle` | u32 newtype（`DecoratorRank::new` = 恒等，与 impl$3::from ICF 合并） | 0x140833250 |

## 跨领域线索

- 消费者 `trigger_audio_system`（0x1409eafa0）读本库 + `Aabb3::intersect_segment`（shared-foundation utils）；wp14 audio-trigger 合同保留（audio-misc DIGEST §5）。
- 旧版错误：`AudioTriggerCollectionHandle` 与二级 slotmap 纯属臆造；`get_triggers` 返回 Option 错；28 B 盒的 "flags" 实为 version（A3_color §三）。

## 算法要点（本批深挖 2026-10-15）

### SlotMap 迭代算法（boxes 0x14157ce40，67 B）

`AudioTriggerCollection` = `slotmap::SlotMap<AudioTriggerHandle, Aabb3>`：slots Vec @0（cap-first），num_elems u32 @0x1c；槽 stride 0x1c = **24 B Aabb3 + 4 B version**。`boxes()` 返回 4 qword `{ptr + (len?0x1c:0), ptr + len*0x1c, (len != 0), num_elems}` = `slotmap::basic::Iter`（`slots.iter().enumerate()`：**跳过哨兵槽 0，起始下标 1，剩余数 = num_elems**）。

### 消费端算法（trigger_audio_system 0x1409eafa0，PDB 名 ICF 折叠为 handplaced_tree_animation，O）

逐槽迭代 `boxes()` 返回的 Iter：`[slot+0x18] & 1` 判占用（version 奇偶 = 活跃槽标记，**不是 flags**）→ `DecoratorRank::new(idx)`（`mov eax,ecx` 恒等，与 `impl$3::from` 0x140833250 ICF 合并 = AudioTriggerHandle ↔ u32）取键 → 前 0x18 B 当 `Aabb3` 喂 `Aabb3::intersect_segment`（shared-foundation utils）。

### 关键常量/布局

| 项 | 值 | 含义 |
|---|---|---|
| AudioTriggerLibrary | `[AudioTriggerCollection; 1]` 0x20 B | 按 kind 索引（COUNT==1，get_triggers 断言 `kind != COUNT` 行 85 列 9） |
| 槽 | 0x1c B = 24 B Aabb3 + 4 B version | slotmap 槽（旧版"28 B 盒+flags"全错） |
| 键 | u32 恒等 | AudioTriggerHandle ↔ DecoratorRank |
| 插入/更新/删除 | 内联于 prefab 系统（无独立符号，I） | add_trigger/update_trigger_shape/remove_trigger/clear |

### 模块间调用

- 生产者：prefab 系统（内联 add/update/remove，无符号）；消费者：`trigger_audio_system`（0x1409eafa0）读库 + `Aabb3::intersect_segment`；`insert_resource<AudioTriggerLibrary>` 搬 0x20 B。
