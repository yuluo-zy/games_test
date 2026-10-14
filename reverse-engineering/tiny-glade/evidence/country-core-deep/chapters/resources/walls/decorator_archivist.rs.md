# resources/walls/decorator_archivist.rs

> 文件作用：装饰归档/剪枝资源：墙合并/分裂/擦除前对墙属装饰做快照归档，提供 cull 放行策略（任意/地形/楼梯）与 before/after 差异提取；interactions 集合在 +0x90。

函数总计 43：业务 22 / 内联无独立符号 8 / 派生胶水 13

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::walls::decorator_archivist::DecoratorArchivist::archive_storage_decorators<ref$<str$> >` | 0x1412b2a70 | 0 | archive_storage_decorators 实例化 | O | wall\decomp\decomp-0x1412b2a70.txt |
| `country_core::resources::walls::decorator_archivist::DecoratorArchivist::extract_before_after_impl` | 0x140b2be70 | 0 | archive_storage_decorators | O | wall\decomp\decomp-0x140b2be70.txt |
| `country_core::resources::walls::decorator_archivist::_::impl$0::apply` | 0x14003bb80 | 56 | decorator_subtype::queue（空函数 ret，no-op） | O | audio-misc\decomp\decomp-0x14003bb80.txt, camera-time\decomp\decomp-0x14003bb80.txt, collision-eco\decomp\decomp-0x14003bb80.txt |
| `country_core::resources::walls::decorator_archivist::_::impl$0::init_state` | 0x140b30a60 | 56 | 维护/推进 | U |  |
| `country_core::resources::walls::decorator_archivist::_::impl$0::new_archetype` | 0x14003bb80 | 56 | decorator_subtype::queue（空函数 ret，no-op） | O | audio-misc\decomp\decomp-0x14003bb80.txt, camera-time\decomp\decomp-0x14003bb80.txt, collision-eco\decomp\decomp-0x14003bb80.txt |
| `country_core::resources::walls::decorator_archivist::_::impl$0::queue` | 0x14003bb80 | 56 | decorator_subtype::queue（空函数 ret，no-op） | O | audio-misc\decomp\decomp-0x14003bb80.txt, camera-time\decomp\decomp-0x14003bb80.txt, collision-eco\decomp\decomp-0x14003bb80.txt |
| `country_core::resources::walls::decorator_archivist::_::impl$0::validate_param` | 0x1403f7af0 | 56 | 谓词/校验 | U |  |
| `country_core::resources::walls::decorator_archivist::DecoratorArchivist::archive_storage_decorators<alloc::string::String>` | 0x1412b3910 | 66 | archive_storage_decorators 实例化 | O | wall\decomp\decomp-0x1412b3910.txt |
| `country_core::resources::walls::decorator_archivist::DecoratorArchivist::archive_current_decorators_in_storage<ref$<str$> >` | 0x14123b5c0 | 207 | archive_current_decorators_* 实例化（84KB） | O | wall\decomp\decomp-0x14123b5c0.txt |
| `country_core::resources::walls::decorator_archivist::DecoratorArchivist::archive_current_decorators_in_storages<ref$<str$> >` | 0x1412b4f10 | 233 | archive_current_decorators_* 实例化（46KB） | O | wall\decomp\decomp-0x1412b4f10.txt |
| `country_core::resources::walls::decorator_archivist::DecoratorArchivist::all_decorators` | 0x140b2bb00 | 251 | validate_param（pmovmskb 零字节位图 → 校验描述符） | O | wall\decomp\decomp-0x140b2bb00.txt |
| `country_core::resources::walls::decorator_archivist::DecoratorArchivist::all_storages` | 0x140b2bb80 | 268 | DecoratorArchivist::all_storages（遍历五集合） | O | wall\decomp\decomp-0x140b2bb80.txt |
| `country_core::resources::walls::decorator_archivist::DecoratorArchivist::wall_decorators` | 0x140a1e190 | 284 | DecoratorArchivist::all_storages 闭包 | O | wall\decomp\decomp-0x140a1e190.txt |
| `country_core::resources::walls::decorator_archivist::DecoratorArchivist::archived_walls` | 0x140a1b100 | 288 | PublicWalls::wall_ids | I | wall\decomp\decomp-0x140a1b100.txt |
| `country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_any` | 0x140b2bbf0 | 294 | decorator_cull_allow_any 状态置位（byte[+0x1e0]=1） | O | wall\decomp\decomp-0x140b2bbf0.txt |
| `country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_wall` | 0x140b2bc00 | 312 | decorator_cull_allow_any（0x98 步长 map 命中回调） | O | wall\decomp\decomp-0x140b2bc00.txt |
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
