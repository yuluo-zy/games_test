# groupA RE-ANALYSIS-2 — tree 树记录 0x48B 字段（task ④）

dbid: tg_xa1 · 2026-10-16 · 证据: tree-tool 13 函数反编译交叉（place_tree 写入面 + hide_trees/ui_move_tree/despawn_trees/ui_delete_tree/color_menu 读取面）

## 树存储容器布局（O，多函数交叉一致）

TreeStorage 资源（ResMut 参数 = &mut Mut<T>，T 双字段容器）：

```
T = { A@+0x00, B@+0x08 }
A（位集+哈希范围）：
  +0x28  u64  存在位集 ptr；+0x38  u64 位集 len
  +0x110/+0x118  u64  哈希范围 1（id→组件查找）；+0x120/+0x128  u64 范围 2
  +0x40  u64  总计数（每推入 +1）
B（id 映射+条目+记录）：
  +0x08  u64  id→索引表 ptr；+0x10  u64 表 len；条目 0x14B = {u32 id_hi@0, u32 索引@+4, u32 变体@+0xc, u32 子索引@+0x10}
  +0xf8  u32* 索引列表 start；+0x100 u32* end
  +0x100 u64  0xa0B 条目表 ptr；+0x108 u64 条目数
  +0x1a8 u64  0x48B 记录表 ptr
```

### 0xa0B 树条目（B+0x100，按索引访问）

| 偏移 | 大小 | 含义 | 证据 |
|---|---|---|---|
| +0x00 | u8×8 | 标志区（bit0=活跃/处理中；color_menu `*(int*)pbStack_100 == 1` 判活跃） | color_menu 0x142291230 / hide_trees |
| +0x18 | u64 | 子图标/子列表指针（color_menu pfVar30 遍历；0xa0 条目内二级表） | color_menu |
| +0x30 | u8 | 种类字节（color_menu 判别 {9,4,10}；树侧未用） | color_menu |
| +0x34 | u32 | 值（color_menu kind==9/4 分支 push 的 u32） | color_menu |
| +0x50 | u64 | **0x58-stride 子表指针**（含 prefab 路径 String：子条目 +0x48 = String 数据指针(+0x10 偏移), +0x50 = len(0x19)） | despawn_trees 0x1422686d0 / ui_move_tree |
| +0x80/+0x88 | u64×2 | 条目内二级哈希（ptr/count；color_menu `*(e+0x88)` vs A+0x118 范围） | color_menu |
| +0x86/+0x87 | u8×2 | 标志字节 | color_menu |
| +0x94 | u32 | 0x48B 记录索引 | despawn_trees `*(u32*)(e+0x94)` |
| +0xf8..+0x108 | — | 索引列表/计数（见容器 B） | despawn_trees |

### 0x48B 树记录（B+0x1a8，place_tree 写入面 O）

| 偏移 | 含义 | 证据 |
|---|---|---|
| +0x00 | **Arc<String> 树 prefab 引用**（LOCK/inc-dec；place_tree 存 FUN_14226a170 结果） | place_tree `*(rec+0)=plStack_c8`；hide_trees Arc 模式 |
| +0x08 | **&str 数据指针**（prefab 路径串；place_tree 存 param_5 deref） | place_tree `*(rec+8)=plStack_128` |
| +0x10 | **打包位置 {u32 x, u32 z}**（pack_2x16f_uint 家族） | place_tree `*(rec+0x10)=uStack_198`；recalc/move 读 `puVar12[0]` 处打包值 |
| +0x18 | **组件数据表指针**（0x30-stride 组：`组[~h*0x30]`：+0x10 = 组件数组（0x10-stride，可见性字节 @+0xc），+0x28 = u32 写数组（滴答）） | hide_trees `*(rec+0x18)=lVar6`；`lVar6+0x10+~*0x30` / `lVar6+0x28+~*0x30` |
| +0x20 | 打包 {u32, u32}（缩放/高度对；place_tree CONCAT44(uVar23,uVar22)） | place_tree |
| +0x28 | 打包对 2 / 写表指针 | place_tree CONCAT44(uVar21,uVar24) |
| +0x30 | {u8 变体, u8 标志}（place_tree uVar33=CONCAT71(…,uStack_138)） | place_tree |
| +0x38 | **树 id（TreeStorage::next_index 计数值）** | place_tree `*(rec+0x38)=plStack_b0`；ui_move_tree 0x58 子表索引源 |
| +0x40 | **记录索引/代计数**（= 容器 A 总计数；hide_trees 按此查 A+0x110..0x128 范围定位组件） | place_tree `*(rec+0x40)=uVar26`；hide_trees 范围校验 |

读取面佐证（hide_trees 0x14226a260）：
```
rec = *(B+0x1a8) + idx*0x48
id  @ rec+0x40（对照 A+0x110/0x118/0x120/0x128 范围）
ptrA@ rec+0x18 → 组[~h*0x30]：{u64 组件1 @+0x10.., u32 写数组 @+0x28..}
ptrB@ rec+0x38 → 组[~h*0x30] 索引表（*8 步进）
可见性写：*(u8*)(组件A + 0xc + i*0x10) = 0/1；*(u32*)(写数组 + i*4) = 滴答
```

### 0x58B 子条目（0xa0 条目 +0x50 指向）

| 偏移 | 含义 |
|---|---|
| +0x48 | prefab 路径 String 数据指针（+0x10 头部偏移 → 0x19 字节串；ui_move_tree 与 'clearing/…'(0x1430354e8) memcmp） |
| +0x50 | String 长度（= 0x19 判定可移动树） |
| +8 + i*0x10 | 组件索引序列（despawn_trees 遍历取组件 α） |

## 命令/历史对偶（O）

- 0x58B 历史命令 = {u32 1, u32 1, u32 变体(0放/1移/2删), u64 树 id, u64, u64, u64 打包位置, u64, u32, u32, u8 标志}；edit 类型 0xc；`History::add` 0x140a84ec0（位置串 0x1430351b8/0x143035328/0x143035340/0x143035520/0x142ef6040 分属五系统）。
- color_menu 同容器形（墙/装饰共用同一 0xa0+0x48 存储）—— **通用 entity storage**；其 0x30B 编辑命令 {u32 1, 0, 1, 0, f32 值, u8 sel, u8 sel^1, u8 kind}，edit 类型 0x9。

## U 项

- +0x20/+0x28 打包对精确语义（缩放 vs 高度；place_tree 随机缩放 {0.4922..1.0} 落入 +0x20 对，I）。
- 0xa0 条目 +0x00..+0x17 标志区逐位语义（bit0=活跃 已证，其余 U）。
- 0x58 子条目 +0x00..+0x40 字段（color 值/变体候选）。
