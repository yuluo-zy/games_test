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
