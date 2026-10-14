# asset_libraries/mod.rs

> 文件作用：**Handle 资产核心类型**——`Asset { name: String }`、`HandleId::new`（全局 `HANDLE_COUNT` 原子 0x143634cd0 取号）、
> `Handle<T> { id: HandleId }`（`#[derive(Component)]`，is_valid）、`From` 转换与 Debug 胶水。所有资产库（mesh/shader/
> texture/vao/ssbo/placement_data）共用同一 handle 计数器。证据：wp4 asset_libraries 合同 §mod + render-gap DIGEST §1 #8。

函数总计 12：业务 2 / 内联无独立符号 6 / 派生胶水 4

## 布局表

| 类型 | 布局 | 结论 | 证据 |
|---|---|---|---|
| `Asset` | `{ name: String }`（cap-first） | **O**（名称；布局 I） | wp4 asset_libraries |
| `Handle<T>` | `{ id: HandleId }`（8 B） | **O**（名称；布局 I） | wp4 asset_libraries |
| `HandleId` | u64（`HandleId::new` = `lock xadd [0x143634cd0], 1` 全局计数） | **O** | wp4 asset_libraries + A3_mesh（add/vao 共用计数器） |

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `impl$21::register_required_components<render::vao::Vao>` | 0x140555a90 | 73 | 猜测：bevy 注册钩子；证实：**derive(Component) 胶水** | **O**（17889 B，render-gap 全量反编译）：callees = `enforce_no_required_components_recursion` / `QueuedRegistration::register` / `HashMap::insert<TypeId, ComponentId>` = bevy `#[derive(Component)]` 生成；内含 `Handle<Vao>` 的 TypeId（0x93c2ea0b5dbce370，O）。**纠错**：RENDER_GAP 原判"需补业务"不成立——`Handle<T>` 已在 mod.rs:57 derive Component | render-gap DIGEST §1 #1 |
| `impl$11::fmt<rhapsody::raw::texture::Texture>` | 0x1412e9e10 | 125 | 猜测：Debug；证实：Handle/HandleId Debug 胶水 | **O**：`impl$11::fmt` 委托 `impl$12::fmt` 写 `"HandleId"`（串 0x142c2f488，位置 `crates/country-core/src/asset_libraries/mod.rs`） | render-gap DIGEST §1 #8 |

## 内联/无独立符号函数

- `Asset::new`（行 31）— **I**：`{ name: String }` 构造（无独立符号）
- `Asset::name`（行 35）— **I**：`&self -> &str` 访问器
- `impl$1::from`（行 42）— **I**：From 转换（无独立符号）
- `HandleId::new`（行 58）— **I**：全局原子计数取号（O 行为见上，A3_mesh/vao_library 调用点证实 `lock xadd [0x143634cd0]`）
- `impl$21::register_required_components`（行 73）— 泛型胶水（上述 O）
- `Handle::is_valid`（行 106）— **I**：句柄有效性谓词（无独立符号）

## 派生胶水（4 项：impl$12::fmt 0x1412ea160、impl$17::eq、impl$5::clone、impl$8::eq）

- `impl$12::fmt`（0x1412ea160）= `HandleId` 的 Debug（与 impl$11 同族，render-gap O）；eq/clone = derive。

## 纠错/差异

- RENDER_GAP 猜测 "bevy Resource/组件注册钩子需补" 不成立（0x140555a90 = derive(Component) 胶水，render-gap §1 纠错）。
- 全局 handle 计数器 0x143634cd0 由 mesh_library/vao_library/shader_library/ssbo_library/placement_data_library 共用（O）。
- 消费方 TypeId 证据（d_cb_a §2）：`asset_libraries::Handle<Mesh>` = 0x877acce28b84e29f 组件、`AssetSsboLibrary` = 0x8226cc786e679355、`AssetMeshLibrary` = 0xd455e4bbfd4f066e 资源（真实类型在场）。
