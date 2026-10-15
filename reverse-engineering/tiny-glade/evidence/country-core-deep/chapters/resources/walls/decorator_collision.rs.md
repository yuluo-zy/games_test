# resources/walls/decorator_collision.rs

> 文件作用：装饰物理碰撞体集合（DecoratorPhysicsColliders）：parry3d 0.17.0 复用边界，无独立业务函数。

函数总计 1：业务 0 / 内联无独立符号 0 / 派生胶水 1

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 1 项
## 实现方式（算法级）

### 模块归属说明（纠错）

decomp 头标注这些方法在 **`decorator_subtype.rs`** 模块（`country_core::resources::walls::decorator_subtype::DecoratorCollision::empty`），章节表沿用 PDB 索引的 decorator_collision.rs 归类——两模块是否合并需集成 owner 裁决。本章记录真实机器码行为。

### DecoratorCollision::empty 0x140b25ca0（O）

构造空碰撞体：Rectangle3d + parry3d cuboid + IsoTransform 全零初始化（0x50+ 字节堆栈结构，逐步写 16B 块），返回 parry3d 碰撞描述符（复用 parry3d 0.17.0，不还原其实现）。

### DecoratorMeshLibrary::insert_collision_and_interaction_as_same 0x140b25ef0（O）

把同一碰撞体克隆后同时登记为"碰撞"与"交互"（两张表），省一次碰撞计算。

### DecoratorMeshLibrary::insert 0x140b262a0（O）

```
签: (self, key u32, value Arc<DerivedDecoratorInfo>)
Arc 克隆 info → 分别插入 4 张表（map1@+0x0 区、map2@+0x30 区、map3@+0x60、map4@+0x90）
```
- `get_outline` 0x140b26690：map3@+0x60 查描边。
- `get_collision_outline` 0x140b267f0（closure$0）：map4@+0x90 查碰撞描边（`if (len@+0xa8 != 0)` → 哈希 FUN_1411f7dc0(+0xb0) → SIMD 探针 ctrl@+0x90/mask@+0x98 → 命中返回）。

### 跨模块调用

FUN_1411f7dc0（u32 哈希）、FUN_1400d9b20（Arc 克隆）、parry3d 0.17.0 类型（复用）。DecoratorMeshLibrary 供装饰物碰撞/交互查询。

### 遗留 U

- 4 张表的键/值类型逐项 U。
- parry3d cuboid 精确参数（半尺寸来源）U。
