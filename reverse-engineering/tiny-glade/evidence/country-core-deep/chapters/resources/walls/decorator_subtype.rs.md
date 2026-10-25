# resources/walls/decorator_subtype.rs

> 文件作用：装饰子类型网格库：DecoratorSubtype(u8, 空=0x1a)、DecoratorLibKey{subtype,variant} 复合键、DecoratorMeshLibrary 4 表（collision/interaction/outline/collision_outline）、DerivedDecoratorSubtypes(HashMap<u32,Arc<DerivedDecoratorInfo>>)。

函数总计 49：业务 20 / 内联无独立符号 11 / 派生胶水 18

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::walls::decorator_subtype::_::impl$0::first` | 0x140002370 | 20 | edit_arrow clone_behavior（xor eax,eax 返回 0） | O | audio-misc\decomp\decomp-0x140002370.txt, camera-time\decomp\decomp-0x140002370.txt, collision-eco\decomp\decomp-0x140002370.txt |
| `country_core::resources::walls::decorator_subtype::DecoratorSubtype::can_be_on_corner` | 0x140b25b50 | 65 | DecoratorSubtype::can_be_on_corner（字节标志读取） | O | wall\decomp\decomp-0x140b25b50.txt |
| `country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes::get` | 0x140b25b60 | 84 | DerivedDecoratorSubtypes::get（HashMap<u32,Arc<DerivedDecoratorInfo>> 未命中 panic） | O | wall\decomp\decomp-0x140b25b60.txt |
| `country_core::resources::walls::decorator_subtype::DecoratorLibKey::new` | 0x140b25c90 | 104 | impl$1::get::closure$0（两参交换） | O | wall\decomp\decomp-0x140b25c90.txt |
| `country_core::resources::walls::decorator_subtype::DecoratorCollision::empty` | 0x140b25ca0 | 122 | DecoratorCollision::empty（Rectangle3d+parry3d cuboid+IsoTransform） | O | wall\decomp\decomp-0x140b25ca0.txt |
| `country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::insert_collision_and_interaction_as_same` | 0x140b25ef0 | 143 | DecoratorMeshLibrary::insert_collision_and_interaction_as_same | O | wall\decomp\decomp-0x140b25ef0.txt |
| `country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::insert` | 0x140b262a0 | 156 | DecoratorMeshLibrary::insert（克隆后插 4 表） | O | wall\decomp\decomp-0x140b262a0.txt |
| `country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_outline` | 0x140b26690 | 171 | DecoratorMeshLibrary::get_outline（map3@+0x60） | O | wall\decomp\decomp-0x140b26690.txt |
| `country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_outline` | 0x140b267f0 | 183 | DecoratorMeshLibrary::get_collision_outline（map4@+0x90） | O | wall\decomp\decomp-0x140b267f0.txt |
| `country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype` | 0x140b26970 | 277 | DerivedDecoratorInfo::subtype | O | wall\decomp\decomp-0x140b26970.txt |
| `country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::try_get_subtype` | 0x140b26980 | 281 | DerivedDecoratorInfo::try_get_subtype | O | wall\decomp\decomp-0x140b26980.txt |
| `country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::bbx_ls<country_core::resources::walls::decorator_subtype::DecoratorSubtype>` | 0x141fdc7b0 | 291 | DerivedDecoratorInfo::dims_wall_space（bbx 差值） | O | wall\decomp\decomp-0x141fdc7b0.txt |
| `country_core::resources::walls::decorator_subtype::impl$6::into_decorator_subtype` | 0x140833ba0 | 325 | DerivedDecoratorInfo::collision_mesh（偏移表尾跳转发） | O | wall\decomp\decomp-0x140833ba0.txt |
| `country_core::resources::walls::decorator_subtype::impl$7::into_decorator_subtype` | 0x140833250 | 331 | DecoratorRank::new | I | wall\decomp\decomp-0x140833250.txt |
| `country_core::resources::walls::decorator_subtype::_::impl$0::apply` | 0x14003bb80 | 336 | decorator_subtype::queue（空函数 ret，no-op） | O | audio-misc\decomp\decomp-0x14003bb80.txt, camera-time\decomp\decomp-0x14003bb80.txt, collision-eco\decomp\decomp-0x14003bb80.txt |
| `country_core::resources::walls::decorator_subtype::_::impl$0::init_state` | 0x140b26e40 | 336 | 维护/推进 | I | wall\decomp\decomp-0x140b26e40.txt |
| `country_core::resources::walls::decorator_subtype::_::impl$0::new_archetype` | 0x14003bb80 | 336 | decorator_subtype::queue（空函数 ret，no-op） | O | audio-misc\decomp\decomp-0x14003bb80.txt, camera-time\decomp\decomp-0x14003bb80.txt, collision-eco\decomp\decomp-0x14003bb80.txt |
| `country_core::resources::walls::decorator_subtype::_::impl$0::queue` | 0x14003bb80 | 336 | decorator_subtype::queue（空函数 ret，no-op） | O | audio-misc\decomp\decomp-0x14003bb80.txt, camera-time\decomp\decomp-0x14003bb80.txt, collision-eco\decomp\decomp-0x14003bb80.txt |
| `country_core::resources::walls::decorator_subtype::_::impl$0::validate_param` | 0x14026c6a0 | 336 | bevy_ecs SystemParam::validate_param（依赖 derive 胶水；校验参数索引表 +0x210/+0x218/+0x1e0，输出 Option\<BevyError\>；串 "::subtypes" 0x1429a7502/"::mesh_lib::subtypes" 0x1429a74f8 + "Resource does not exist" 0x1429a6c32 + 参数类型名 0x14294586a） | 胶水排除 | wall\decomp\decomp-0x14026c6a0.txt（候选U收尾闭合） |
| `country_core::resources::walls::decorator_subtype::impl$8::from` | 0x140b26a70 | 346 | From 转换（双字段提取 {+0x0,+0x20}） | O |  |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::walls::decorator_subtype::DecoratorCollision::dims_wall_space`（行 118）— 待推断
- `country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::get_collision_bbx_ls`（行 223）— 查询/计算
- `country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::dims_wall_space`（行 241）— 待推断
- `country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::collision_mesh`（行 249）— 待推断
- `country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::interaction_mesh`（行 258）— 待推断
- `country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::new`（行 273）— 构造
- `country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::dims_wall_space`（行 285）— 待推断
- `country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::bbx_ls`（行 291）— 待推断
- `country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::collision_mesh`（行 298）— 待推断
- `country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::interaction_mesh`（行 309）— 待推断
- `country_core::resources::walls::decorator_subtype::_::impl$0::get_param`（行 336）— 查询/计算

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`clone_from`、`closure`、`default`、`eq`、`fmt`、`hash`、`next` 等 18 项
## 实现方式（算法级）

### 方法面（O）

- `queue` 0x14003bb80 = **`ret` 空函数**（no-op，PDB 索引把 decorator_archivist/_impl$0/apply、new_archetype、queue 与 decorator_subtype::queue 同名同址）。
- `into_decorator_subtype` 0x140833250 = `mov eax,ecx; ret`（字节直通）。
- `DecoratorRank::new` 0x140833250（同址，I）。
- `can_be_on_corner` 0x140b25b50：字节标志读取。
- `DerivedDecoratorSubtypes::get` 0x140b25b60：HashMap<u32, Arc<DerivedDecoratorInfo>> 未命中 panic。
- `DerivedDecoratorInfo::subtype` 0x140b26970 / `try_get_subtype` 0x140b26980：Arc 子类型访问。
- `collision_mesh` 0x140833ba0 = 尾跳转发（mov edx,eax; jmp 偏移表 getter，sret 双参风格）。
- `get::closure$0` 0x140b25c90 = `mov eax,edx; mov edx,ecx; ret`（两参交换，比较器/哈希器）。

### DecoratorCollision::empty 0x140b25ca0（O，decomp 头标注 decorator_subtype 模块）

构造空碰撞体：Rectangle3d + parry3d cuboid + IsoTransform 三连结构全零初始化（0x50+ 字节），返回 parry3d 碰撞描述符。

### DecoratorMeshLibrary::insert 0x140b262a0（O，克隆后插 4 表）

```
签: (self, key u32, value Arc<DerivedDecoratorInfo>)
Arc 克隆 info → 分别插入 4 张表（map1@+0x0 区、map2@+0x30 区、map3@+0x60、map4@+0x90）
```
- `get_outline` 0x140b26690：map3@+0x60 查描边。
- `get_collision_outline` 0x140b267f0（closure$0）：map4@+0x90 查碰撞描边；`if (len@+0xa8 != 0)` → 哈希（FUN_1411f7dc0, hasher@+0xb0）→ SIMD 探针（ctrl@+0x90, mask@+0x98）→ 命中返回轮廓。

### get_outline::closure$0 0x140b267f0（5.6KB，O）

```
签: (self, 参数 u8×2 = 轮廓索引对)
cStack_7c/7b/7a/79 = 4 份参数拷贝（索引对）
if (map4 len@+0xa8 != 0): FUN_1411f7dc0(+0xb0, &key) 哈希 → SIMD 探针（0x… 步长，键 2×u8 比较）→ 命中返回 &outline
```

### dims_wall_space 0x141fdc7b0（13KB，O）

```
签: (out, self, u1 u32, u2 u32) → 装饰在墙空间的尺寸（bbx 差值）
FUN_1411f7dc0(+0x? , &u) 哈希 → 探针（0x50B 条目，u 键）→ 取装饰记录 bbx min/max → 差值出 dims
panic: "The decorator mesh library doesn't contain "（0x142fcaa58，fmt，file 0x142fcaa68 =
  /home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/decorator_subtype.rs）
  + 拼接库内缺失 id
```

### 新常量表

| 常量 | 地址 | 语义 |
|---|---|---|
| "The decorator mesh library doesn't contain " | 0x142fcaa58 | dims_wall_space fmt |
| 源路径泄漏 | 0x142fcaa68 | /home/h3/code/pouncelight/country-slice-private/crates/country-core/… |

### 跨模块调用

FUN_1411f7dc0（u32 哈希）、FUN_140b25c90（比较器/哈希器闭包）、FUN_1428d9390（fmt panic）、Arc 克隆（FUN_1400d9b20）。DecoratorMeshLibrary 4 表供 DecoratorSubtype 查询 / dims_wall_space / outline 消费。

### 遗留 U

- DecoratorMeshLibrary 4 表（map1@+0x0/map2@+0x30/map3@+0x60/map4@+0x90）键/值类型逐项 U。
- dims_wall_space 探针步长与 bbx 记录内字段 U。
