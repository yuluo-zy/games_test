# resources/walls/wall_trigger_volume.rs

> 文件作用：墙触发体积：quad 数组 {ptr@+8,len@+0x10}（元素 0x60B）到 get_tris 输出 0x48B 三角形；recompute_quads 重算网格（panic !triangles/!mesh_uvs/!normals.is_empty）；形状编辑拖拽触发体。

函数总计 29：业务 9 / 内联无独立符号 9 / 派生胶水 11

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::recompute_quads` | 0x1408eae40 | 0 | recompute_quads（27KB；panic !mesh_uvs/!normals.is_empty()） | O | wall\decomp\decomp-0x1408eae40.txt |
| `bevy_ecs::system::schedule_system::impl$1::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)> >` | 0x140002370 | 116 | edit_arrow clone_behavior（xor eax,eax 返回 0） | O | audio-misc\decomp\decomp-0x140002370.txt, camera-time\decomp\decomp-0x140002370.txt, collision-eco\decomp\decomp-0x140002370.txt |
| `country_core::resources::walls::wall_trigger_volume::impl$5::clone_behavior` | 0x140002370 | 116 | edit_arrow clone_behavior（xor eax,eax 返回 0） | O | audio-misc\decomp\decomp-0x140002370.txt, camera-time\decomp\decomp-0x140002370.txt, collision-eco\decomp\decomp-0x140002370.txt |
| `country_core::resources::walls::wall_trigger_volume::impl$5::register_required_components` | 0x1408ecd50 | 116 | 构造/注册/生成 | U |  |
| `country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::default_tris` | 0x1408e80e0 | 201 | register_required_components（返回 {+0x20,+0x28} 对） | O | wall\decomp\decomp-0x1408e80e0.txt |
| `country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_tris` | 0x1408e80f0 | 207 | get_tris（quad 0x60B → tri 0x48B 输出；panic !triangles.is_empty()） | O | wall\decomp\decomp-0x1408e80f0.txt |
| `country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_collision_geo` | 0x1408e8d70 | 219 | 访问器/转换 | I | wall\decomp\decomp-0x1408e8d70.txt |
| `country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::recompute` | 0x1408e9ed0 | 310 | 重算 | I | wall\decomp\decomp-0x1408e9ed0.txt |
| `country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_position_from_mesh_uv` | 0x1408ec0a0 | 552 | 访问器/转换 | I | wall\decomp\decomp-0x1408ec0a0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::walls::wall_trigger_volume::Quad::apply_params`（行 30）— 待推断
- `country_core::resources::walls::wall_trigger_volume::Quad::as_triangles_w_normals_n_uvs`（行 78）— 访问器/转换
- `country_core::resources::walls::wall_trigger_volume::WallTriggerVolumeParams::new`（行 145）— 构造
- `country_core::resources::walls::wall_trigger_volume::WallTriggerVolumeParams::edit_shape_default`（行 156）— 待推断
- `country_core::resources::walls::wall_trigger_volume::WallTriggerVolumeParams::offset_from_the_top_ws`（行 176）— 待推断
- `country_core::resources::walls::wall_trigger_volume::WallTriggerVolumeParams::u_range`（行 182）— 待推断
- `country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::is_valid`（行 196）— 谓词/校验
- `country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_collision_geo_callback`（行 242）— 查询/计算
- `country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::recompute_tris`（行 522）— 重算

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default` 等 11 项
## 实现方式（算法级）

### WallTriggerVolume 结构（O，DIGEST §1.11 深化）

- quad 数组 {ptr@+8, len@+0x10}，**元素 0x60=96B**（+0x58 u64、+0x64 u64、[1]/[2]/+0xc/+0x14 u32 字段）。
- tri 输出元素 **0x48=72B**；`register_required_components` 0x1408e80e0 返回 {+0x20, +0x28} 对（组件注册表切片，手解）。
- 选项结构（get_tris 参数）{+0xc bit0 存在位, +0x10 f32 u, +0x14/+0x18 u32}。

### get_tris 0x1408e80f0（21KB，O）

```
签: (self, quad 数据 param_2, options param_3)
if (quad 计数 == 0) → panic_str "assertion failed: !self.quads.is_empty()"（0x142aa64c0, line 0x28, file 0x142aa64e8）
分配输出: lVar17×0x48（72B/三角）
if ((options+0xc & 1) == 0) → 分支（存在位判别）
bStack_f1 = quad@+0x60 字节（几何标志）
逐 quad（0x60B 步进）→ 转 2 三角（0x48B 输出）:
  几何/曲线: FUN_140d8b480（构造辅助）；FUN_141467ce0 域插值
  u 范围断言: if (u < 0.0f@0x142960690 || 1.0f@0x142925904 < u) → panic "assertion failed: !u.is_nan()"（0x142aa53d0, line 0x1d）/ "!u.is_infinite()"（0x142aa5498, line 0x22, file 0x142aa5480）
  分支 uVar35 = (0<fVar31) & bStack_f1（u 正半域判定）
  平移: 1.0-u（0x142925904）分量
尾部 panic "assertion failed: !triangles.is_empty()"（0x142aa6480, line 0x27, file 0x142aa64a8）
```

### recompute_quads 0x1408eae40（28KB，O）

```
签: (self, 曲线 param_2)
前置: FUN_1428d9310(&UNK_142aa6670)（fmt）；panic_str "assertion failed: quad_count > 0"（0x142aa6688, line 0x20, file 0x142aa66a8）→ Vec 清空（FUN_1428a9a60(0)）
逐 quad: FUN_14157f420(&几何,&曲线)（矩阵/三线性）；FUN_140c91fb0 ×2（投影）
变换: 逐顶点矩阵乘（常量 _UNK_1429258d4/0x870/0x8e0..8ec、_UNK_142aa4550..55c = 缩放/基向量矩阵系数）
  例如: fVar28 = col1 * _UNK_1429258d4 * _UNK_142925870 等
u 范围分支: if (0<fVar29 或 0<fVar30 不满足) → 分支
panic 族（FUN_140b5f930 fmt，消息 0x142aa67b0/6720/6738/6750/6768/6780）: 几何非法/越界分支
尾部 FUN_1428d9518(0,0,&UNK_142aa66d8) / (0,0,&UNK_142aa66f0)：uvs/normals 断言
```

### 新常量表

| 常量 | 地址 | 语义 |
|---|---|---|
| 0.0f / 1.0f | 0x142960690 / 0x142925904 | u∈[0,1] 断言界 |
| 矩阵系数 | 0x1429258d4/0x870/0x8e0..8ec、0x142aa4550..55c | recompute_quads 变换 |

### 跨模块调用

FUN_140d8b480、FUN_14157f420、FUN_140c91fb0、FUN_141467ce0（曲线插值）、FUN_140b5f930（fmt panic）、FUN_1428d9310（panic）、FUN_1428d9518（断言）、FUN_140613c10（分配）。get_tris/recompute_quads 供触发体积/碰撞构建消费。

### 遗留 U

- quad 0x60B 记录内全部字段名（+0x58/+0x64 u64、u32 字段）U。
- 选项 {+0xc bit, +0x10 f32, +0x14/+0x18 u32} 精确语义 I。
- recompute_quads panic 族（0x142aa67b0 等）原文未逐条解码。
