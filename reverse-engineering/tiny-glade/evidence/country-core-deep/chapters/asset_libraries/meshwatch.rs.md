# asset_libraries/meshwatch.rs

> 文件作用：**网格文件热重载监视器**——`MeshWatch { watched: Arc<Mutex<HashSet<PathBuf>>>, watch_lock: Mutex<()>,
> hotwatch: Option<GenericHotwatch> }`：按 **Mesh.source_file**（`&Mesh` 内 +0x18）注册 hotwatch；Create/Write 事件把
> **被监视路径**（不是事件路径）插入共享集合，由 `systems::backend::meshwatch`（0x14085b4b0）消费重载。证据：
> `evidence/country-core-audit/A3_mesh.md` §meshwatch（O）。

函数总计 5：业务 2 / 内联无独立符号 1 / 派生胶水 2

## 布局表（O，A3_mesh，总 0x40）

| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x00 | `Arc<Mutex<HashSet<PathBuf>>>`（Arc 块 0x48） | new 0x1408ba490 |
| +0x08 | `Mutex<()>`（futex 字节 @8、poison @9；PDB `MutexGuard<bool>` 为 ICF 合并名，I：ZST） | new 0x1408ba490 |
| +0x10 | `Option<GenericHotwatch>`（判别字 3 = None） | new 0x1408ba490 |

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `MeshWatch::new` | 0x1408ba490 | 17 | 猜测：构造；证实：按 use_watch 建 hotwatch（100 ms） | **O**（578 B）：`use_watch` 真 → `GenericHotwatch::new_with_custom_delay(Duration{0, 0x5f5e100})` = **100 ms**（旧版 50 ms 错）并 `.expect("hotwatch failed to initialize")`（行 20 列 18）；否则 None。共享集合为空 HashSet（ArcInner 0x48） | A3_mesh.md |
| `MeshWatch::watch` | 0x1408ba6e0 | 29 | 猜测：注册监视路径；证实：**入参是 `&Mesh`**（读 +0x18 source_file） | **O**（1098 B）：source_file None → 立即返回；Some → clone Arc 与路径，取 watch_lock（中毒 unwrap panic，行 39 列 43），watcher 存在则 `hotwatch.watch(&path, closure{Arc, 路径克隆})` + `.expect("failed to watch file!")`（行 52 列 14）。旧版 `watch(path: PathBuf)` + fs::read 入队字节均错 | A3_mesh.md |

## 内联/无独立符号函数

- `MeshWatch::watch_path`（行 35）— **I**：`watch` 的内联实现（无独立符号；PDB 名存在）。

## 派生胶水（2 项：closure）

- `impl$0::watch_path::closure$0` 0x14091e7a0（O，903 B）：参数 `DebouncedEvent`（niche 判别 `tag ^ i64::MIN`）；**2（Create）或 3（Write）** → 锁集合（中毒 unwrap panic）、插入**闭包捕获的被监视路径**（不是事件路径）；其它事件仅 drop。I：notify `DebouncedEvent` 变体编号按 notify 4.x 声明顺序推断。
- `impl$0::new::closure$0`：无独立符号（I）。

## 跨领域线索 / 纠错

- 消费者 `systems::backend::meshwatch` 0x14085b4b0：锁 `[res+0]` 集合 → 逐路径 `reload_mesh_src` → 清空集合 → 即 `changed_paths()` 返回 `&Arc<Mutex<HashSet<PathBuf>>>`（A3_mesh O）。
- 旧版 `watch(path: PathBuf)` + 回调读文件字节——原版 `watch(&Mesh)` 且回调不读文件。

## 算法要点（本批深挖 2026-10-15，decomp: asset-libraries/decomp/ + backend-misc/decomp/）

证据：tg_m1 反编译 `decomp-0x1408ba6e0`（watch 1098 B）、`decomp-0x14091e7a0`（closure 903 B）+ 消费端 `backend-misc/decomp/decomp-0x14085b4b0`（meshwatch 系统，11432 B）。

### 热重载全算法（O）

1. **注册**：`watch(&Mesh)`（读 `mesh+0x18` source_file：None → 立即返回；Some → 克隆 Arc 与路径）→ 取 `watch_lock`（Mutex<()>，ZST 锁；中毒 unwrap panic 行 39 列 43）→ watcher 存在则 `hotwatch.watch(&path, closure{Arc, 路径克隆})` + `.expect("failed to watch file!")`（行 52 列 14）。**以 Mesh 的 source_file 为监视对象；mesh 变体触发后需重新 watch 新路径**（watch_lock 串行化）。
2. **回调闭包**（0x14091e7a0）：`DebouncedEvent` 判别 = `tag ^ 0x8000000000000000`；**== 2（Create）或 == 3（Write）** → 锁共享集合（poison → unwrap panic）→ `insert(闭包捕获的被监视路径)`；其它事件仅 drop。**写入的是注册路径而非事件路径**。
3. **消费端**（systems::backend::meshwatch 0x14085b4b0，bevy 系统 `(Res<GraphicsDeviceResource>, Res<AssetMeshLibrary>, Res<MeshWatch>, Res<AssetShaderLibrary>, Res<ShaderWatch>, Commands)`，O 经 component_access ICF 名）：锁 Arc<Mutex<HashSet<PathBuf>>>（ArcInner：强/弱@0/8、futex@+0x10、poison@+0x11、HashSet@+0x18：ctrl@0x18、mask@0x20、items@+0x30）→ items != 0 且级别 > 2 → debug/info 日志 → **hashbrown SSE 组遍历每个占用桶 → 取 PathBuf → `reload_mesh_src(&mut mesh_lib, path)`（FUN_14092d8d0）**（同时写 ResMut tick）→ 遍历后第二遍释放全部键 PathBuf + `memset(ctrl,0xff)` + 清 items = **drain/clear：一次消费全部清空，事件不累积**。

### 关键常量

| 常量 | 值 | 含义 |
|---|---|---|
| 监视延迟 | 100 ms（0x5f5e100 ns） | new_with_custom_delay（旧版 50 ms 错） |
| 事件变体 | Create=2 / Write=3 | notify 4.x（i64::MIN 基址编码，`tag ^ 0x8000000000000000`） |
| 布局 | 0x40 B：Arc<Mutex<HashSet>>@0、Mutex<()>@8、Option<GenericHotwatch>@0x10（判别 3 = None） | MeshWatch |
| watch_lock | 中毒 → unwrap panic | watch_path 串行化（MutexGuard<bool> 为 ICF 合并名） |

### 模块间调用

- `MeshWatch::watch(&Mesh)` ← mesh 加载方（source_file Some 的网格）；→ `AssetMeshLibrary::reload_mesh_src`（消费端直调）；→ `vao_update::rebuild_vaos`（0x140864bf0）链式重建 VAO（mesh_lib.modified 经 get_mut 压入）。
