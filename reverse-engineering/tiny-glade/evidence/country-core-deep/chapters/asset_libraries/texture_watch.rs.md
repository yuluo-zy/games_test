# asset_libraries/texture_watch.rs

> 文件作用：**纹理文件热重载监视器**——`TextureWatch { hotwatch: Option<GenericHotwatch>, changed:
> Arc<Mutex<HashSet<RelativePathBuf>>> }`：`watch(name: String, path: &Path)` 注册后，Create/Write/Rename 事件把
> **被监视的名字**（RelativePathBuf）插入共享集合；消费端读取并重载（消费过程不在本模块，U）。证据：
> `evidence/country-core-audit/A3_texture.md` §TextureWatch（O）。

函数总计 4：业务 2 / 内联无独立符号 0 / 派生胶水 2

## 布局表（O，A3_texture）

| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x00 | `Option<GenericHotwatch>`（判别 None） | new 0x1408bab30 |
| +0x08 | `changed: Arc<Mutex<HashSet<RelativePathBuf>>>`（Arc 块 0x48 B） | new 0x1408bab30 |

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `TextureWatch::new` | 0x1408bab30 | 17 | 猜测：构造；证实：按 use_watch 建 hotwatch（100 ms） | **O**：`use_watch` → `GenericHotwatch::new_with_custom_delay(secs 0, nanos 0x5f5e100)` = **100 ms**（旧版 50 ms 错），失败 `expect("hotwatch failed to initialize")`；`changed: Arc<Mutex<HashSet<RelativePathBuf>>>` 空（Arc 块 0x48 B） | A3_texture.md |
| `TextureWatch::watch` | 0x1408bad70 | 29 | 猜测：注册监视；证实：**签名 `(&self, name: String, path: &Path)`** | **O**：`rdx=&String, r8:r9=&Path`；先 clone Arc；hotwatch None → drop Arc 与 name 返回；否则 `hotwatch.watch(path, 闭包{name, arc})`，失败 `expect("failed to watch file!")`。旧版把参数当 PathBuf 并在回调里读文件字节——原版不读文件 | A3_texture.md |

## 派生胶水（2 项：closure）

- `impl$0::watch_path::closure$0` 0x14091edb0（O）：`DebouncedEvent`（notify 4.0.17）仅 Create(2)/Write(3)/Rename(6)（掩码 0x4c）处理：Mutex 锁（poison→unwrap），`insert(RelativePathBuf(name.clone()))`；插入的是**监视时的名字**而不是事件路径。变体序号按 notify 4.0.17 源码顺序对应（I，由 `0x4c` 掩码 + 变体数 9 推出）。
- `impl$0::new::closure$0`：无独立符号（I）。

## 遗留（U）

- 消费端（读取并重载 changed 集合）不在三个过程里，U（A3_texture）。
- 与 ConfigWatch/MeshWatch 同族：100 ms 延迟、回调写注册名而非事件路径（O 共性）。
