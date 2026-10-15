# resources/walls/wall_stone_columns.rs

> 文件作用：石柱碰撞存储（StoneColumnCollisionStorage）：墙体石柱的碰撞数据存储（v2 新反编译 0x140a78880）。

函数总计 3：业务 1 / 内联无独立符号 0 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::walls::wall_stone_columns::StoneColumnCollisionStorage::clear_and_get_mut` | 0x140a78880 | 25 | StoneColumnCollisionStorage（11.4KB） | O | wall\decomp\decomp-0x140a78880.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 2 项
## 实现方式（算法级）

### StairsPillarProposals::add 0x140901270（7KB，O；属 stairs_walls 域，本章引用）

```
签: (self map, 键 param_2, 值 param_3)
if (map len@+0x18 == 0) → 空处理
FUN_1405b0d70(+0x20) 哈希键 → SIMD 探针（0x… 步长条目，键比较）
  命中冲突 → FUN_140b5ff30(lVar3+lVar16-0x18, &UNK_142aa9d98)（重复柱位 panic，fmt）
未中 → FUN_1402c9720(&out, self, hash, &key) 插入（0x2cB 元素：{u32 键, 值 0x28B}）
尾部 FUN_1428d9000(4, 0x2c)（分配/断言）
```

### StoneColumnCollisionStorage::clear_and_get_mut 0x140a78880（11.4KB，O）

```
签: (self, param_2) → undefined1*（清空并取可变引用）
FUN_140562870(self, &UNK_142ad6c70)（Vec 扩容，panic 串）
FUN_1405b0d70(param_1+7 /*+0x38*/) 哈希 → SIMD 探针（0x… 步长柱记录 map，键比较）
if (param_1[5]/*len@+0x28*/ == 0) → FUN_1428a9ca0(param_1+3, 1, param_1+7, …) 扩容
写/清柱记录，返回 &mut 存储
```

### 跨模块调用

FUN_140562870（Vec 扩容）、FUN_1428a9ca0（hashbrown 扩容）、FUN_1402c9720（map 插入）、FUN_140b5ff30（fmt panic）、FUN_1428d9000（断言）。StairsPillarProposals 供楼梯/石柱构造；StoneColumnCollisionStorage 供墙石柱碰撞。

### 遗留 U

- StairsPillarProposals 条目完整字段（0x2cB 值区）U。
- StoneColumnCollisionStorage map 头精确偏移（+0x28 len/+0x38 hasher 已证，其余 U）。
