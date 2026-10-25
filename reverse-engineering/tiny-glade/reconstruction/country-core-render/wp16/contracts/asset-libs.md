# 合同：asset_libraries 非渲染表面（watch / VAO / 放置数据 / 核心 id）

源 SHA256（B skeleton）：`asset_libraries/{mod,meshwatch,configwatch,
texture_watch,vao_library,placement_data_library}.rs`（`audio_trigger_library.rs`
属 WP14）。PDB 实体：`Asset` / `Handle` / `HandleId` / `MeshWatch` /
`ConfigWatch` / `TextureWatch` / `AssetVAOLibrary` / `AssetPlacementDataLibrary`
/ `InstancedDrawFromPlacementData`。
Rust：`src/asset_libraries/…`

标注：**O**=反编译/PDB 直接支持；**I**=推断；**U**=未恢复（typed gap）。

## 核心 id（mod.rs）

- `HandleId`（O：`HandleId::new` 全局 `HANDLE_COUNT` 原子自增；`from_raw`）——
  wp4 已恢复，本包重述非渲染面。
- `Asset`（O：`Asset::new` / `Asset::name`，`impl$21::register_required_components`
  0x140555a90 注册 `Vao` 组件）。
- `Handle<T>`（O：按 id 等值/哈希/排序；`is_valid` 内联于 autoclutter）。

## watch（hotwatch 复用）

- `ConfigWatch` / `MeshWatch` / `TextureWatch` 同构（O）：
  `new(use_watch)`（0x1408ba0f0 / 0x1408ba490 / 0x1408bab30）在 `use_watch` 时建
  `hotwatch::GenericHotwatch::new_with_custom_delay<ReadDirectoryChangesWatcher>`
  （notify 后端）+ `Arc<Mutex<Option<Vec<u8>>>>` 变更字节队列；
  `watch(path)`（0x1408ba290 / 0x1408ba6e0 / 0x1408bad70）注册监听闭包：
  文件变更 → 读字节入队。backend `configwatch`/`meshwatch`/`texture_watch`
  每帧排空。
- `MeshWatch::watch` 的字节拷贝（O：`_Size/_Src` memcpy）证明队列内容为文件
  原始字节。

## AssetVAOLibrary

- `new`（0x140924030，两张 hash 表）/ `add(mesh_handle)`（0x140924180：
  RawTable 查重早退，`HANDLE_COUNT++`，`build_vao` 内联）/ `rebuild_vao`
  （0x1409243f0，绑定差异重建）/ `remove`（0x1409247a0）/ `has_vao`
  （`vao_update::rebuild_vaos` 内联）。
- 容器 O：`HashMap<Handle<Mesh>, Vao>`；`Vao` 本体属 wp2 `render::vao`
  （0x909690 `build_vao` 已由 wp2 恢复）。

## AssetPlacementDataLibrary

- `new`（0x140867700，两张 RandomState 表）/ `load<PathBuf>`（0x1409365d0，
  JSON/文件解析，`add` 内联）/ `clear`（`uninit_clearing` 0x140a7a5b0 内联）。
- 条目 `InstancedDrawFromPlacementData`（U 布局）：装饰 scatter/autoclutter 的
  实例化配置；`DrawBuilder::draw_instanced_from_placement_data` 消费
  （wp1 面）。

## 未知（U）

- watch 队列长度/事件丢失策略；VAO 绑定差异判定；放置数据 JSON schema 与
  `InstancedDrawFromPlacementData` 布局；`Asset::name` 哈希细节。
