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
| `country_core::resources::walls::decorator_subtype::_::impl$0::validate_param` | 0x14026c6a0 | 336 | 谓词/校验 | U |  |
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
