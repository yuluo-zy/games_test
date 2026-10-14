# asset_libraries/mesh_library.rs

> 文件作用：**网格资产库**——`AssetMeshLibrary`：`FrozenMap<Handle<Mesh>, Box<Mesh>>` 主表 + 名字表
> `RwLock<HashMap<String, Handle<Mesh>>>` + 源路径表 `RwLock<HashMap<PathBuf, Handle<Mesh>>>` + modified 记录 +
> `Arc<Mutex<Vec<…>>>` transient_meshes。支持名字/路径查询、remove、按路径重载（json）。证据：
> `evidence/country-core-audit/A3_mesh.md`（O 全量）+ wp4 asset_libraries 合同。

函数总计 16：业务 12 / 内联无独立符号 0 / 派生胶水 4

## 布局表（O，A3_mesh，总 0xd0）

| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x00 | `Vec<Handle<Mesh>>`（cap-first；modified 记录，get_mut/remove 压入） | get_mut 0x14092d300 |
| +0x18 | `Arc<Mutex<Vec<_>>>`（transient_meshes；ArcInner 0x30：强/弱=1、std Mutex futex@0x10/poison@0x11、Vec@0x18） | transient_meshes 0x140002450（`lea rax,[rcx+0x18]; ret`） |
| +0x20 | `FrozenMap<Handle<Mesh>, Box<Mesh>>`（0x40 B） | new / add / get |
| +0x60 | `RwLock<HashMap<String, Handle<Mesh>>>`（桶 0x20） | add / try_get_handle_by_name |
| +0x98 | `RwLock<HashMap<PathBuf, Handle<Mesh>>>`（桶 0x28） | add / reload_mesh_src |

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `AssetMeshLibrary::new` | 0x14092c920 | 37 | 猜测：构造；证实：五字段全空 + 三个 RandomState | **O**（824 B）：五个字段全空；三个 `RandomState` 各自取线程局部 `KEYS`（k0 自增）。旧版无 `modified`/`transient_meshes`，用 HashMap 冒充 FrozenMap，多一个不存在的 `disposed` | A3_mesh.md |
| `AssetMeshLibrary::add` | 0x14092cc60 | 47 | 猜测：注册网格；证实：**入参一个 0x90 B 结构 `{ asset: Mesh(0x78)@0, name: Option<String>@0x78 }`** | **O**（1652 B）：`HANDLE_COUNT++`（全局 0x143634cd0）→ source_file Some 时克隆路径 → `Box::new(mesh)`（0x78/8）→ `FrozenMap::insert`，失败 `panic!("assertion failed: self.mesh_entries.insert(mesh_handle, Box::new(asset.asset)).is_ok()")` → name Some：写锁 `names.insert`（同名覆盖）→ 路径 Some：写锁 `paths.insert`。旧版 `add(name: String, mesh: Mesh)` 签名错 | A3_mesh.md |
| `AssetMeshLibrary::get` | 0x14092d2e0 | 70 | 猜测：按 handle 取；证实：FrozenMap::get | **O**（29 B）：`FrozenMap::get(self+0x20, &handle)` | A3_mesh.md |
| `AssetMeshLibrary::get_mut` | 0x14092d300 | 74 | 猜测：可变取；证实：先压 modified 再 FrozenMap 借用检查 | **O**（331 B）：**无条件** `self.modified.push(handle)`（含不存在的 handle），再检查借用字节、表空 → None、哈希查找（RandomState @+0x50）。旧版没有 push | A3_mesh.md |
| `AssetMeshLibrary::get_by_name` | 0x14092d450 | 84 | 猜测：名字取网格；证实：try_get_handle_by_name → FrozenMap::get **原样返回 Option** | **O**（145 B）：读锁名字表 → 命中后 `FrozenMap::get` 结果原样返回。旧版多了 unwrap 与错误文案 `mesh {:?} not found` | A3_mesh.md |
| `AssetMeshLibrary::get_handle_by_name` | 0x14092d4f0 | 89 | 猜测：名字取句柄；证实：try_get + unwrap | **O**（120 B）：缺失 → `panic!("Mesh {:?} does not exist in the library")`（pieces @0x142aafcd8） | A3_mesh.md |
| `AssetMeshLibrary::try_get_handle_by_name` | 0x14092d570 | 95 | 猜测：可选名字查询；证实：读锁哈希查找 | **O**（479 B）：parking_lot 快路径 `cmpxchg [lock], lock+0x10`，items@+0x80 为 0 → None，否则 `hash_one<&str>` + memcmp | A3_mesh.md |
| `AssetMeshLibrary::remove` | 0x14092d750 | 103 | 猜测：删除；证实：FrozenMap remove_entry + names retain | **O**（372 B）：借用字节 → unwrap panic；`remove_entry` 命中则 drop Box<Mesh> + dealloc；写锁 `names.retain(|_, h| *h != handle)`（retain 0x14152d7e0）。**不动路径表**。旧版 `assert!(!disposed)` 不存在 | A3_mesh.md |
| `AssetMeshLibrary::reload_mesh_src` | 0x14092d8d0 | 108 | 猜测：按路径重载；证实：paths 查 + json 校验 + load_json_as_mesh 替换 | **O**（1590 B）：入参 `(&mut self, path: &str)`；读锁 paths 查不到 → `panic!("Asset Mesh library was requested to reload a non-existing mesh path {:?}")`（0x142aafe20，位置 mesh_library.rs）；`assert_eq!(path.extension().and_then(to_str), Some("json"))`（静态 `"json"`@0x142aafd10，行 116 列 9）；`self.get_mut(handle).unwrap()`；`load_json_as_mesh<&str>`（0x140b0eeb0，Result<Mesh,anyhow>，indices.cap==i64::MIN 为 Err niche）：Ok → drop 旧 Mesh 后 memcpy 0x78 B；Err → `log::error!("Asset mesh library couldnt reload a mesh: {}", e)`（target `country_core::asset_libraries::mesh_library`，行 122 列 27），旧网格保留 | A3_mesh.md；decomp: backend-misc/decomp/decomp-0x14092d8d0.txt |
| `transient_meshes` | 0x140002450 | 127 | 猜测：返回瞬时网格集合；证实：`&Arc<Mutex<Vec<_>>>` 访问器 | **O**（5 B）：`lea rax,[rcx+0x18]; ret`。**元素类型 U**（占位用 `Handle<Mesh>`）；0x140002450 为 bevy `component_access` ICF 合并地址，`bevy_ecs::…::component_access<…>` 与 `UiState::active_widget_state` 同址挂名（PDB 别名） | A3_mesh.md |

## 派生胶水（4 项：closure ×3 + bevy component_access 合并项）

- `impl$1::get_handle_by_name::closure$0`、`impl$1::remove::closure$0`、`impl$1::reload_mesh_src::closure$0/$1`：哈希等价/比较闭包（I，无独立符号）。
- `bevy_ecs::…::component_access<FunctionSystem<meshwatch 系统>>`（0x140002450，PDB 泛型名揭示系统参数面：`(Res<GraphicsDeviceResource>, Res<AssetMeshLibrary>, Res<MeshWatch>, Res<AssetShaderLibrary>, Res<ShaderWatch>, Commands)`）——bevy 系统包装胶水，与 `transient_meshes`/`UiState::active_widget_state` ICF 合并；**该签名就是 meshwatch 后端系统**（`systems::backend::meshwatch` 0x14085b4b0）的参数面（O）。

## 纠错/差异（A3_mesh，O）

- 旧版签名 `add(name, mesh)` 错：真实入参为 `{asset: Mesh, name: Option<String>}` 结构。
- 旧版 `reload_mesh_src` 先查扩展名、加载失败静默忽略、不压 modified——全错。
- `Mesh.source_file` 应为 `Option<PathBuf>`（Windows PathBuf = 0x20 B：Vec<u8> + is_known_utf8 字节；cap==i64::MIN 为 None）。
- U：`load_json_as_mesh` 真实实现在 `utils::load_json`（B_x_out 列为遗留，0x140b0eeb0）。
- 消费者：`systems::backend::meshwatch` 0x14085b4b0 与 `vao_update.rs` 0x140864bf0（遍历 `mesh_lib.modified` 重建 VAO，见 vao_library 章）。
