# A3 mesh 组：`mesh_library.rs` / `meshwatch.rs` / `vao_library.rs`

方法：`dis.py` 逐函数反汇编 + `rdva.py` 解 panic/format 文案；标记 O＝机器码直接观察，I＝推断，U＝未知。
测试：隔离测试台副本 `/tmp/h_mesh`（41 个新测试，全部通过；整台 149 个）。

## 过程清单（procs.tsv，PDB 名为 country_core::asset_libraries::…）

| 文件 | 过程 | 地址 | 大小 |
|---|---|---|---|
| mesh_library | `AssetMeshLibrary::new` | 0x14092c920 | 824 |
| | `add` | 0x14092cc60 | 1652 |
| | `get` | 0x14092d2e0 | 29 |
| | `get_mut` | 0x14092d300 | 331 |
| | `get_by_name` | 0x14092d450 | 145 |
| | `get_handle_by_name` | 0x14092d4f0 | 120 |
| | `try_get_handle_by_name` | 0x14092d570 | 479 |
| | `remove` | 0x14092d750 | 372 |
| | `reload_mesh_src` | 0x14092d8d0 | 1590 |
| | `transient_meshes` | 0x140002450 | 5（与 bevy `component_access` ICF 合并） |
| meshwatch | `MeshWatch::new` | 0x1408ba490 | 578 |
| | `MeshWatch::watch` | 0x1408ba6e0 | 1098 |
| | `impl$0::watch_path::closure$0` | 0x14091e7a0 | 903 |
| vao_library | `AssetVAOLibrary::new` | 0x140924030 | 328 |
| | `add` | 0x140924180 | 611 |
| | `rebuild_vao` | 0x1409243f0 | 937 |
| | `remove` | 0x1409247a0 | 263 |

交叉核对用到的外部调用者：`systems::backend::meshwatch::meshwatch` 0x14085b4b0、`vao_update::rebuild_vaos` 0x140864bf0、
`render::vao::build_vao` 0x140909690、`HashMap::retain<String,Handle<Mesh>>` 0x14152d7e0。

## 通用结论

* **全局 handle 计数器** 0x143634cd0（`lock xadd`）：`AssetMeshLibrary::add` 与 `AssetVAOLibrary::add` 共用（O）。VAO 的 `add` 在校验**之前**取号，失败也消耗 id（O）。
* **`FrozenMap`**（源码 `crates/frozen/src/lib.rs`）：`[map+8]` 是“借用中”字节，`get_mut`/`remove` 命中则 `Result::unwrap` panic；表在 +0x10（hashbrown：ctrl+0x10、mask+0x18、items+0x28、`RandomState`+0x30）。（O）
* **Windows `PathBuf` = 0x20 B**（`Vec<u8>` + `is_known_utf8` 字节）。`Mesh.source_file` 因此是 `Option<PathBuf>`（`cap == i64::MIN` 为 None，O）；`mesh.rs` 里 `SourceFile{path:String, stamp:u64}` 大小一致但语义不同（I：stamp 实为 `is_known_utf8`）。
* 格式化函数判别：`core::fmt::impl$74::fmt<str>`(0x1405338a0) 尾调 `impl$19`(900 B) = `str` 的 **Debug**；`impl$20`(22 B) = Display。`impl$74::fmt<OsStr/Path>`(0x14002e8d0) 尾调 `std::path::impl$69::fmt` = Path Debug。`anyhow::error::impl$4::fmt` = `ErrorImpl::display`。（O）

## mesh_library.rs

**布局（O，`new` 的字段写入，总大小 0xd0）**：`+0x00 Vec<Handle<Mesh>>`、`+0x18 Arc<Mutex<Vec<_>>>`（ArcInner 0x30：强/弱=1，std Mutex futex@0x10/poison@0x11，Vec@0x18）、
`+0x20 FrozenMap<Handle<Mesh>, Box<Mesh>>`（0x40 B）、`+0x60 RwLock<HashMap<String,Handle<Mesh>>>`（桶 0x20）、`+0x98 RwLock<HashMap<PathBuf,Handle<Mesh>>>`（桶 0x28）。

### `new` 0x14092c920（O）
五个字段全空；三个 `RandomState` 各自取线程局部 `KEYS`（k0 自增）。旧版：无 `modified`/`transient_meshes`，用 `HashMap` 冒充 FrozenMap，多一个不存在的 `disposed`。

### `add` 0x14092cc60（O）
* 入参只有一个 0x90 B 结构 `{ asset: Mesh(0x78)@0, name: Option<String>@0x78 }`（panic 文案里 `asset.asset` 为证）。旧版 `add(name: String, mesh: Mesh)` 错。
* 顺序：`HANDLE_COUNT++` → 若 `source_file` 为 Some 克隆路径 → `Box::new(mesh)`（alloc 0x78/8）→ `FrozenMap::insert`，失败 `panic!("assertion failed: self.mesh_entries.insert(mesh_handle, Box::new(asset.asset)).is_ok()")`
  → `name` 为 Some：写锁 `names.insert`（旧值丢弃，同名覆盖）→ 路径克隆为 Some：写锁 `paths.insert`。
* 旧版 `mesh_source_file` 恒 `None`，路径表从不填充。

### `get` 0x14092d2e0 / `get_mut` 0x14092d300（O）
`get` = `FrozenMap::get(self+0x20, &handle)`。`get_mut` 先 `self.modified.push(handle)`（无条件，含不存在的 handle），再检查借用字节、表空 → None、哈希查找（`RandomState`@+0x50）。**旧版没有 push**。

### `try_get_handle_by_name` 0x14092d570 / `get_handle_by_name` 0x14092d4f0 / `get_by_name` 0x14092d450（O）
读锁（parking_lot 快路径 `cmpxchg [lock], lock+0x10`），items@+0x80 为 0 → None，否则 `hash_one<&str>` + memcmp。
`get_handle_by_name` 缺失 → `panic!("Mesh {:?} does not exist in the library")`（pieces 0x142aafcd8，`{:?}` 见上）。
`get_by_name`：成功后 `FrozenMap::get` 的结果**原样返回**（`Option<&Mesh>`），旧版多了 unwrap 与错误文案 `mesh {:?} not found`。

### `remove` 0x14092d750（O）
借用字节 → unwrap panic；`remove_entry`，命中则 `drop_in_place<Mesh>` + `dealloc(0x78, 8)`；然后写锁 `names.retain(|_, h| *h != handle)`（retain 函数 0x14152d7e0：`[bucket-8] == handle` 才擦除并释放 String）。**不动路径表**。旧版 `assert!(!disposed)` 不存在。

### `reload_mesh_src` 0x14092d8d0（O）
入参 `(&mut self, path: &str)`。流程：读锁 `paths`（items==0 或 `hashbrown::equivalent<Path,PathBuf>` 查不到）→
`panic!("Asset Mesh library was requested to reload a non-existing mesh path {:?}")`（0x142aafe20，meshwatch… 位置 `mesh_library.rs`）；
`assert_eq!(path.extension().and_then(to_str), Some("json"))`（右值静态 `"json"`@0x142aafd10，位置第 116 行 col 9）；
`self.get_mut(handle).unwrap()`（压 modified；缺失 `Option::unwrap`）；`load_json_as_mesh<&str>`（0x140b0eeb0，返回 Result<Mesh,anyhow>，Mesh 的 `indices.cap == i64::MIN` 为 Err 的 niche）：
Ok → `drop_in_place<Mesh>` 后 memcpy 新值（0x78 B）；Err → `log::error!("Asset mesh library couldnt reload a mesh: {}", e)`（target `country_core::asset_libraries::mesh_library`，len 0x2b；第 122 行 col 27），旧网格保留。
旧版：先检查扩展名、加载失败静默忽略、不压 modified。
U：`load_json_as_mesh` 的真实实现在 `utils::load_json`（todo），目前经 `utils_contract` 的占位加载。

### `transient_meshes` 0x140002450（O 地址与布局 / U 元素类型）
`lea rax,[rcx+0x18]; ret`，返回 `&Arc<Mutex<Vec<_>>>`。元素类型未知（占位用 `Handle<Mesh>`）。

## meshwatch.rs

布局（O，`new`）：`+0x00 Arc<Mutex<HashSet<PathBuf>>>`、`+0x08 Mutex<()>`（futex 字节 @8、poison @9；PDB 里的 `MutexGuard<bool>` 为 ICF 合并名，数据字节未被初始化，I：ZST）、`+0x10 Option<GenericHotwatch>`（判别字 3 = None），总 0x40。

### `MeshWatch::new` 0x1408ba490（O）
`use_watch` 为真 → `GenericHotwatch::new_with_custom_delay(Duration{secs=0, nanos=0x5f5e100})`（**100 ms**，旧版 50 ms）并 `.expect("hotwatch failed to initialize")`（第 20 行 col 18）；否则 None。共享集合为空 `HashSet`（ArcInner 0x48）。

### `MeshWatch::watch` 0x1408ba6e0（O）
入参是 `&Mesh`（读 `mesh+0x18` 的 `source_file`）：None → 立即返回；Some → 克隆 Arc 与路径，取 `watch_lock`（中毒 `unwrap` panic，第 39 行 col 43），watcher 存在则
`hotwatch.watch(&path, closure{Arc, 路径克隆})` 并 `.expect("failed to watch file!")`（第 52 行 col 14）。旧版 `watch(path: PathBuf)` + `fs::read` 入队字节，均错。

### 回调闭包 0x14091e7a0（O）
参数 `DebouncedEvent`（niche 判别 `tag ^ i64::MIN`）：2（Create）或 3（Write）→ 锁集合（中毒 unwrap panic）、插入**闭包捕获的被监视路径**（不是事件路径）；其它事件仅 drop。
I：notify `DebouncedEvent` 变体编号按 notify 4.x 声明顺序推断；`reused::watch::GenericHotwatch` 占位回调只带 `PathBuf`，故 `watch` 里把占位回调当作 Create/Write，真实事件分类入口是 `MeshWatch::on_debounced_event`。

### 消费者（范围外，仅取证）
`systems::backend::meshwatch` 0x14085b4b0：锁 `[res+0]` 里的集合 →（集合非空且 log 级别 ≥3 时 debug 日志）→ 逐路径 `reload_mesh_src` → 清空集合（遍历释放 String、重置控制字节）。故 `changed_paths()` 返回 `&Arc<Mutex<HashSet<PathBuf>>>`。

## vao_library.rs

布局（O，`new`，总 0x60）：`+0x00 HashMap<Handle<Vao>, VaoEntry>`（桶 0x50：键@0、`Vao`@8（bindings@8、index_buffer@0x20）、`usize`@0x48）、`+0x30 HashMap<Handle<Mesh>, Handle<Vao>>`（桶 16 B）。
`VaoEntry` 的字段名为 I；`usize` 取值 O：`add`/`rebuild_vao` 写 `mesh.indices.len()`（`[mesh+0x10]`）。

### `new` 0x140924030（O）两张空表，各取一次线程局部 `KEYS`。

### `add` 0x140924180（O）
`(self, device, mesh_lib, mesh_handle) -> Result<Handle<Vao>, anyhow::Error>`：
1. `mesh_to_vao` 非空且命中 → `Ok(已有 vao handle)`；
2. `HANDLE_COUNT++` 取 VAO handle（先于校验）；
3. `mesh_lib.get` 缺失 → `anyhow!("Mesh is not in the  mesh library")`（0x142aae830，**两个空格**）；
4. `indices.len()==0` → `anyhow!("Mesh is invalid")`（0x142aae800）；
5. `build_vao`（0x140909690）；结果的 `bindings.cap == i64::MIN` 走 Err 透传分支——build_vao 内部全 `unwrap`，I：不可达；
6. `vaos.insert(vao_handle, (vao, len))`，drop 旧值，`mesh_to_vao.insert(mesh, vao_handle)`。
旧版：`add(mesh_handle: u64)` 只查重后塞一个空 Vao，无设备、无校验、无 VAO handle。

### `rebuild_vao` 0x1409243f0（O）
返回 `Option<anyhow::Error>`（rax==0 ok）：
1. `mesh_lib.get(mesh).expect("Mesh is not in the mesh library")`（单空格；`vao_library.rs` 第 84 行 col 14）；
2. `indices` 空 → 返回 Ok（**静默**）；
3. `mesh_to_vao` 空/未命中 → `anyhow!("The requested mesh doesn't exist")`（0x142aae898）；
4. `vaos` 空/未命中 → `anyhow!("VAO handle is invalid")`（0x142aae8c0）；
5. `build_vao`，drop 旧 `bindings`，drop 旧索引缓冲（rhapsody managed handle drop）后写入，`index_count = indices.len()`，返回 Ok。
调用者 `rebuild_vaos` 0x140864bf0：遍历 `mesh_lib.modified`，仅处理 `has_vao` 的 handle（否则 warn 级日志跳过），失败的 handle 收集起来写回 `modified`（并 error 日志）。旧版 `rebuild_vao` 是空函数。

### `remove` 0x1409247a0（O）
`mesh_to_vao.remove_entry(mesh)`；命中则 `vaos.remove_entry(vao_handle)` 并 drop `Vao`（`Vec<VaoBinding>` + 缓冲 handle）。旧版只删单表。

## 影响到的外部文件（已处理 / 待主线程）

* `frozen_map.rs`：新增 `FrozenMap::remove(&mut self, &K) -> Option<Box<V>>`（`AssetMeshLibrary::remove` 需要；纯增量）。
* `render/mesh.rs`：`source_file` 应为 `Option<PathBuf>`（现为 `Option<SourceFile{path:String, stamp:u64}>`，大小一致；本组按 `PathBuf::from(&path)` 兼容）。
* `boundary.rs`：只有文档引用与一个 `AssetMeshLibrary(HashMap<u64,Mesh>)` 占位门面，未被本组真实类型替换，等待整合者换成真实类型；其 `get_mut(u64)` / `has_vao(u64)` 签名与真实 `Handle<Mesh>` 版不同。
* `systems/backend/meshwatch.rs`（0x14085b4b0）与 `vao_update.rs` 是未还原的消费者，接口依据：`MeshWatch::changed_paths()`、`AssetMeshLibrary::modified_meshes`（pub）、`AssetVAOLibrary::{has_vao, rebuild_vao}`。
