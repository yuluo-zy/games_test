# paths

应用目录模型与平台路径策略库。它定位 Tiny Glade 应用根目录、构造 Steam 账户数据目录并提供进程级路径入口。它不实现存档格式、备份轮换、账户切换或存档读写事务。

## 模块结构

| 模块 | 职责 |
| --- | --- |
| `src/lib.rs` | `Paths` 数据模型、独立目录创建、`ensure_path` 和公开导出 |
| `src/platform.rs` | 无文件系统副作用的平台默认目录选择 |
| `src/global.rs` | 全局单次初始化、失败重试和 getter |
| `src/tests.rs` | 目录布局、平台选择、失败和并发测试 |
| `tests/global_initialization.rs` | 子进程隔离的全局状态和 cwd 测试 |

## 数据模型与目录布局

`Paths` 拥有全部 `PathBuf`，支持 `Clone`、`Debug`。其 getter 借用对应 `Path`，不查询环境或访问磁盘。全局实例发布后在进程生命周期内保持不变。

`Paths::initialize_at(steam_id, app_root, installed)` 接受**完整应用根目录**，不会为它追加 `Tiny Glade`。布局为：

```text
<app_root>/
└── Steam/
    └── <steam_id 的十进制表示>/
        ├── saves/
        ├── beta-saves/
        ├── tmp/
        ├── deleted-saves/
        └── backup-saves/
```

`config` 是账户目录 `Steam/<steam_id>`；`standard_app_root_path` 是 `<app_root>`，供账户以外的应用偏好使用；`installed` 独立于此布局，只记录传入位置，不创建目录或验证资源。`beta-saves` 是预留的 beta 版本存档目录。

`app_root` 和 `installed` 必须非空。相对输入通过 `std::path::absolute` 固定为调用时的绝对路径；这不是文件系统 canonicalize，不解析符号链接，也不要求路径预先存在。目录名以 `OsString`/`PathBuf` 保留，支持非 Unicode 路径。

## 默认平台目录

`find_standard_app_root_path()` 只选择路径，不创建目录。

| 平台 | 默认应用根目录 |
| --- | --- |
| Windows | Known Folder `SavedGames` 下的 `Tiny Glade`；查询失败使用 `$USERPROFILE/Saved Games/Tiny Glade` |
| macOS | `$HOME/Library/Application Support/Tiny Glade` |
| Linux | 绝对 `$XDG_DATA_HOME/Tiny Glade`；该变量缺失、为空或为相对值时使用 `$HOME/.local/share/Tiny Glade` |

需要的 `HOME`/`USERPROFILE` 缺失、为空或不是绝对路径时返回错误。Linux 有有效绝对 `XDG_DATA_HOME` 时不要求 `HOME`。Windows 有有效 Known Folder 时不读取 `USERPROFILE`。其他平台的默认目录查询返回不支持错误；显式目录构造不依赖此默认策略。

## 业务流程

### 独立路径实例

1. 将应用根目录和安装目录都解析为绝对路径；任一输入解析失败时不创建目录。
2. 构造 `Steam/<steam_id>`。
3. 依次递归创建 `saves`、`beta-saves`、`tmp`、`deleted-saves`、`backup-saves`，最后确保账户根目录存在。
4. 所有创建成功后返回 `Paths`。失败时返回包含路径的错误，先前创建的目录保留。

独立实例不注册到全局状态，可同时持有不同账户或不同根目录的实例。

### 全局初始化

`initialize_paths(steam_id, root_override)` 在串行初始化锁内选择应用根目录：override 是**父目录**，绝对化后追加 `Tiny Glade`；没有 override 时使用平台默认目录。安装目录从 `current_exe()` 的父目录取得，然后按独立实例流程创建目录。

只有创建成功才通过 `OnceLock` 发布。失败不会占用全局槽位，可修复阻塞文件或权限后重试。初始化成功后，重复调用在运行构造逻辑前返回错误，不产生该次构造的文件系统副作用；进程内不能切换全局账户。

`preference_root_path()` 不要求全局初始化：初始化后返回配置的应用根目录，初始化前返回平台默认值。它不会隐式初始化全局实例或创建目录。

## 公开 API

以下 `Result` 均为 `anyhow::Result`；`Path`、`PathBuf` 来自 `std::path`。

### 独立目录模型

| API | 语义 |
| --- | --- |
| `Paths::initialize_at(steam_id: u64, app_root: PathBuf, installed: PathBuf) -> Result<Paths>` | 从完整应用根目录构造并创建账户目录 |
| `Paths::config(&self) -> &Path` | 账户根目录 |
| `Paths::saves(&self) -> &Path` | 普通存档目录 |
| `Paths::beta_saves(&self) -> &Path` | beta 存档目录 |
| `Paths::backup_saves(&self) -> &Path` | 备份存档目录 |
| `Paths::tmp(&self) -> &Path` | 临时目录 |
| `Paths::deleted_saves(&self) -> &Path` | 已删除存档目录 |
| `Paths::installed(&self) -> &Path` | 安装目录 |
| `Paths::standard_app_root_path(&self) -> &Path` | 完整应用根目录 |

### 目录策略与创建工具

| API | 语义 |
| --- | --- |
| `find_standard_app_root_path() -> Result<PathBuf>` | 查询平台默认应用根目录，不创建目录 |
| `ensure_path(path: impl AsRef<Path>) -> Result<PathBuf>` | 递归建目录，返回输入路径的拥有副本；不额外绝对化 |
| `preference_root_path() -> Result<PathBuf>` | 优先返回已初始化根目录，否则查询平台默认值 |

### 全局状态

| API | 语义 |
| --- | --- |
| `initialize_paths(steam_id: u64, root_override: Option<&Path>) -> Result<()>` | 创建并发布一次；override 是应用目录的父目录 |
| `try_paths() -> Result<&'static Paths>` | 读取全局实例；未初始化返回错误 |
| `config() -> &'static Path` | 全局账户根目录 |
| `saves() -> &'static Path` | 全局普通存档目录 |
| `backup_saves() -> &'static Path` | 全局备份存档目录 |
| `tmp() -> &'static Path` | 全局临时目录 |
| `deleted_saves() -> &'static Path` | 全局已删除存档目录 |
| `installed() -> &'static Path` | 全局安装目录 |
| `standard_app_root_path() -> &'static Path` | 全局完整应用根目录 |

**全局便捷 getter 要求 `initialize_paths` 已成功，否则 panic。** 无法保证初始化顺序时，应使用 `try_paths()` 处理错误。没有全局 `beta_saves()` 自由函数，可使用 `try_paths()?.beta_saves()`。

## 错误与并发语义

输入路径解析、平台目录查询、定位当前可执行文件和目录创建失败均通过 `Result` 返回。初始化失败不会发布不完整实例。目录创建不是事务，不回滚先前的目录；错误恢复后可以重试。

全局初始化使用互斥锁串行执行，并发调用最多一个成功。成功发布后的 getter 不获取初始化锁。独立 `Paths` 不共享全局锁；相同路径的并发创建依赖文件系统的 `create_dir_all` 行为。

路径对象只保存位置，不保证后续写权限或目录持续存在。外部进程可以修改、删除或用符号链接替换目录，调用方需要处理实际 I/O 错误。

## 最小用例

使用独立对象和临时注入目录，避免写入默认用户目录或发布全局状态。

```rust
use paths::Paths;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> anyhow::Result<()> {
    let suffix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = std::env::temp_dir().join(format!("paths-example-{}-{suffix}", std::process::id()));
    std::fs::create_dir(&root)?;
    let app_root = root.join("Tiny Glade");
    let installed = root.join("installation");
    let account = Paths::initialize_at(7, app_root.clone(), installed.clone())?;

    assert_eq!(account.config(), app_root.join("Steam/7"));
    assert!(account.saves().is_dir());
    assert!(account.beta_saves().is_dir());
    assert_eq!(account.installed(), installed);
    assert!(!account.installed().exists());

    std::fs::remove_dir_all(root)?;
    Ok(())
}
```

## 开发与验证

在 `games_test/tiny-glade` workspace 根执行；`--offline` 要求依赖已缓存。

```sh
cargo test --offline -p paths --all-targets
cargo clippy --offline -p paths --all-targets --no-deps -- -D warnings
cargo fmt -p paths -- --check
```

测试包含平台选择策略、非 Unicode 路径、部分创建失败、初始化重试、并发单次发布及工作目录变化。全局状态通过子进程隔离，避免跨测试污染。Rust 最低版本按 workspace 的 `rust-version` 声明。

## 限制

- 安装目录不进行存在性或资源完整性检查；存档与备份目录也不验证未来写入权限。
- 初始化成功不代表后续 I/O 一定成功；没有文件系统事务、账户迁移、备份轮换或跨进程初始化锁。
- 默认目录策略限定 Windows、macOS、Linux，实际 Known Folder、权限和文件系统行为需在目标平台验证。
- 全局根目录与账户固定；需要多账户或隔离配置时使用独立 `Paths`。

## 来源与审查记录

源码来源与文件身份见 [foundation-audit.json](../../provenance/foundation-audit.json)，审查记录见 [foundation-rereview](../../reports/foundation-rereview-2026-10-17/README.md)。
