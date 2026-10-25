# systems/wall/wall_state/wall_roof_hashmap.rs

> 文件作用：墙到屋顶映射维护：Query<(Entity,&Roof,Option<...>)> 遍历，维护 PublicWalls.roof_entity 与 WallEntities.wall_entity；预置形状无 roof 到清除。

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::wall_state::wall_roof_hashmap::wall_roof_hashmap` | 0x140a94cd0 | 40 | wall_roof_hashmap（Query 遍历 + 双向映射维护） | O | wall\decomp\decomp-0x140a94cd0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项

## 实现方式（算法级）

### wall_roof_hashmap 0x140a94cd0（7 系统参数，47KB）

**参数面**（O）：param_1 = 模式资源（`piVar6=*param_1`，`*piVar6==0xe` → 删 roof panic，`==0xd` → 删除分支）；param_2 = Res<PublicWalls>（`plStack_b0=*param_2`，0x2a8 map）；param_3 = ResMut 事件存储（行 1043 +0x1c tick 回写，16B 行释放 `func_0x000140613c20(ptr,len*0x10…,0x10)`）；param_4 / param_5 = **两个 Bevy Query 迭代状态资源**（`+0xf8` archetype 索引数组、`+0x100` 计数、`+0x110/+0x118` 行游标；`param_4[1]/param_5[1]` = Tables/Archetypes：`+0x100` 0xa0B archetypes、`+0x1a8` 0x48B tables）；param_6 / param_7 = 另两个 ResMut（+0x1c 回写行 576/296）。

**处理链**：
1. puffin scope `FUN_140a919c0(&UNK_142ada041)`（name = `…wall_roof_hashmap::wall_roof_hashmap`，0x142ada041）+ 路径裁剪（文件串 0x142ada450）。
2. 事件 reader（行 222–232）；hash 种子从静态 `_UNK_142ad9ee0/9ee8/9ef0/9ef8` 装载（两组 64 位种子，行 257–264）。
3. **Query 迭代（第一遍，行 288–537，roof 记录 0x58B 步长）**：
   - 遍历 archetype/table：`puVar33`（+0xf8 数组）逐 archetype 取 `tables_ptr + id*0x48`；表行游标推进；实体列 = archetype+0x50 处 16B/8B 行。
   - 每行：`roof_rec = rows_ptr + uVar28*0x58`（0x58B roof 记录），`wall_id = *roof_rec`，`roof_entity = *puVar27`（实体列）。
   - **PublicWalls 0x2a8 查表**（行 413–466）：命中 → `value+0x10<2 || value+0x30<=0` → panic `w.simplified_curve_2d.is_valid()`；`value+0x258`（形状 tag）>1（**Freehand → 跳过 roof 映射**）→ goto 循环继续。
   - tag<2（预置形状）→ `FUN_140eec970(&query, wall_id, roof_entity)`（行 472/476）= **roof_entity 映射 setter**；若返回指示已有 roof → **panic `Only one roof should be associated with a wall id`**（`FUN_1428d9390(&…,&UNK_142ada5a8)`，行 484）。
   - 删除路径：`*piVar6==0xe` → fmt panic **`Tried deleting roof {entity}`**（`FUN_1428d9390(&…,&UNK_142ada538)`，行 533；同串 0x142ada4f0 行 1089）；`==0xd` → 删除分支。
4. **Query 迭代（第二遍，行 855–1018，wall 记录 0x30B 步长）**：同上结构；每行 `wall_id = *rows_0x30`（`rows_ptr + uVar28*0x30`），实体 = 16B 行高 32 位（`uVar37>>0x20`，为 0 则跳过）；PublicWalls 查表 → 有效性断言（行 980）→ `FUN_140eec970(&query2, wall_id, entity)`（行 983）→ 继续。
5. 收尾：param_3 事件行刷新 + 尾事件提交（行 1043–1076）。

**数据结构**：PublicWalls 条目 {WallId u64 key, value 0x2a0}（0x2a8 步长，`value+0x10` 点计数、`value+0x30` 长度、`value+0x258` 形状 tag 均复用 §1.1 布局）。

**常量**：shape tag 判别 `1 < value+0x258`（= Freehand）；模式值 0xd/0xe。

**跨模块 callee**：FUN_140eec970（wall↔roof 映射 setter，wall 域自身）、FUN_140a92100（PublicWalls 查 roof_entity，wall 域）、FUN_140a919c0（puffin guard）、FUN_140562630（Vec 扩容，bevy events）、FUN_1405b0d70（hash）、FUN_140d4d740/FUN_140d64e00（种子静态/生成，与 new 共用）。

**遗留 U**：FUN_140eec970 内部写入的字段偏移（roof_entity 落点）；param_1 模式资源的完整枚举（0xd/0xe 之外值行为未逐行）；预置形状无 roof 的"清除分支"是否即 tag>1 跳过路径（I，未发现独立清除写）。

### 纠错
- 任务/DIGEST 称 Query 迭代器为 FUN_140eec970/d64e00/d4d740——**d64e00/d4d740 实为种子生成/静态访问器**（与 new 0x140aa5150 相同），Query 迭代是内联的 archetype/table 遍历；仅 FUN_140eec970 是映射维护 callee。
