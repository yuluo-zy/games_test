# asset_libraries/mod.rs

> 文件作用：资产库通用类型：`HandleId`（u64 包装）、`Handle<T>`（资产类型化句柄，`#[derive(Component)]`）。

## 关键事实

- `Handle<T>` 是 bevy `Component`（`asset_libraries/mod.rs:57` 已 derive）；实体携带 `Handle<Mesh>`/`Handle<Vao>` 组件，
  wp12 墙 signifier 系统查询 `&Handle<Mesh>`。
- **`asset_libraries::impl$21::register_required_components` 0x140555a90（1776 B，render-gap §2.1）**：
  反编译 + typeid_resolve 证实为 **`#[derive(Component)]` 的泛型注册胶水**，单态化实例 = `Handle<render::vao::Vao>`
  （内含 TypeId `0x93c2ea0b5dbce370`，O）。callees 全部是 bevy ECS 组件注册机（`enforce_no_required_components_recursion`/
  `QueuedRegistration::register`/`HashMap::insert<TypeId,ComponentId>`）——**非业务函数**，derive(Component) 已覆盖。
  **纠错**：RENDER_GAP §2.1 判"真实逻辑需补（Resource/组件注册钩子）"不成立。
- `HandleId` Debug 写 `HandleId({:#x})`（`impl$11/12::fmt` 0x1412e9e10/0x1412ea160 = 胶水，串 "HandleId"，位置
  `crates/country-core/src/asset_libraries/mod.rs`），已实现。

证据：`render-gap/decomp/decomp-0x140555a90.txt`、`decomp-0x1412e9e10.txt`、`decomp-0x1412ea160.txt`；`render-gap/DIGEST.md §1`。
