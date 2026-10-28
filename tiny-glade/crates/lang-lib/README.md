# lang-lib

与 UI 框架无关的静态文本本地化库。它加载 YAML 语言包，提供当前语言与英文回退查询，原子替换语言快照，并保存或恢复最近语言偏好。`tg-iced` 通过全局入口取得界面文本；本包不创建 UI 组件或注册 Bevy 系统。

## 模块结构

| 模块 | 职责 |
| --- | --- |
| `src/lib.rs` | 公开类型、函数和 `language_library` 模块的导出 |
| `src/language_library.rs` | locale 校验、YAML schema 校验、独立语言包和查表 |
| `src/service.rs` | 独立本地化服务、语言加载、快照发布和查询 |
| `src/global.rs` | 进程级配置、延迟初始化和 `paths` 偏好目录适配 |
| `src/preference.rs` | 最近语言读取、校验、临时文件写入及替换 |
| `src/missing_cache.rs` | 有界缺失 key 标记缓存 |
| `src/diagnostics.rs` | 本地化警告的同线程重入保护 |
| `src/tests.rs`、`tests/global_configuration.rs` | 快照、并发、I/O 故障及全局配置测试 |
| `examples/` | 分配、环境配置及显式资源目录消费探针 |

## 数据模型

`LanguageLibrary` 拥有一个 `Arc<str>` locale、`HashMap<String, Arc<str>>` 翻译表和缺失 key 缓存。`Localization` 拥有固定的绝对资源根目录、固定的 key 回退策略及可替换快照。快照包含当前语言包、可选英文包和独立缺失缓存；当前语言已经是英文时复用同一个包。

查询返回 `Arc<str>`。已取得的文本可在语言切换后继续使用；快照和包在最后一个引用释放后回收。翻译命中只克隆 `Arc`，不复制字符串。缺失标记最多缓存 1024 条；缓存容量外仍会生成结果，但不保留结果或记录首次缺失警告。

## 输入与存储格式

语言包路径为 `<asset_root>/<locale>.yml`。文件必须是 UTF-8，内容为空文件或单个扁平字符串映射，例如：

```yaml
Menu.Close: Close
Version: "123"
Enabled: "true"
```

点号属于完整 key，不表示嵌套。未加引号的数字、布尔值和 null 等 YAML 类型不能用作字符串；嵌套映射、数组、非字符串 key/value 和多个文档会返回错误。重复 key 被拒绝，包括引号或 alias 表达不同、解析后相同的 key。合法字符串 alias 可以使用。

locale 必须非空，且只含 ASCII 字母、数字、`-`、`_`，因此不能包含路径分隔符或 `..`。解析与路径构造入口执行校验；`LanguageLibrary::new` 接受调用方已验证的内存数据，不再校验 locale。

偏好文件为 `<preference_root>/last_used_locale`，正文只包含用户请求的 locale，不添加换行。即使该请求实际由英文替代，保存的仍是请求值，以便以后重试。恢复时会 trim 正文；文件缺失、不可读或标识非法时使用 `en-US`。

## 业务流程

### 加载与查询

1. 创建 `Localization` 时将资源目录固定为绝对路径，不读取语言包。
2. 校验请求 locale 并读取语言包。目标包读取或解析失败时尝试 `en-US`；请求本身为 `en-US` 且失败时直接返回错误。
3. 开启 key 回退且当前包不是英文时，还必须成功读取英文包。全部必需资源就绪后，一次发布完整快照；任一步失败都保留之前的快照。
4. 查询先查当前包，再查可选英文包。开启 key 回退时，两包都缺失返回 `?_<key>`；关闭时缺失直接返回原 key。加载前查询返回原 key，报告的默认 locale 为 `en-US`。

**`fallback_enabled` 只控制缺失 key 回退，不关闭整包加载失败时的英文替代。**

### 偏好持久化

独立服务可选择只加载，或在加载成功后保存偏好。保存先创建偏好目录，再创建同目录独占临时文件，写入并 `sync_all`，关闭句柄后重命名替换目标。错误路径会尝试清理临时文件。

持久化采用 best effort：保存失败会警告，但成功发布的快照保留，加载调用仍返回成功。恢复偏好只读取并加载，不重写偏好文件。全局入口无法定位偏好目录时，仍可加载目标包；全局恢复入口则加载默认包。

### 全局配置

`configure` 必须在首次全局加载或 `lang_pack_path` 调用前成功。未显式配置时，首次这些操作使用当时 cwd 下的 `assets/lang`；`TG_DISABLE_LANG_FALLBACK` 只要存在就关闭 key 回退，空值、`0` 和非 Unicode 值也一样。显式 `configure` 的参数不受此环境变量影响。

配置成功后不能重新配置；资源路径和回退策略不随 cwd 或环境变化。全局 `lookup`、`game_locale` 本身不触发初始化。

## 公开 API

以下 `Result` 均为 `anyhow::Result`。所有全局函数从 crate 根导出；`LanguageLibrary` 同时可从 `language_library` 模块访问。

### 独立语言包：`LanguageLibrary`

| 方法 | 输入与返回值 |
| --- | --- |
| `new(locale: String, entries: HashMap<String, Arc<str>>) -> Self` | 从已验证的内存数据创建包 |
| `try_from_yml(path: impl AsRef<Path>, locale: String) -> Result<Self>` | 读取 UTF-8 文件并解析，错误附文件路径 |
| `parse_yml(text: &str, locale: String) -> Result<Self>` | 校验标识与 YAML，创建拥有数据的包 |
| `locale(&self) -> Arc<str>` | 返回包的标识 |
| `try_get(&self, key: &str) -> Option<Arc<str>>` | 查询翻译，不产生缺失诊断 |
| `get(&self, key: &str) -> Arc<str>` | 查询翻译，缺失返回 `?_<key>`，不执行跨包回退 |

### 独立服务：`Localization`

| 方法 | 语义 |
| --- | --- |
| `new(asset_root: impl AsRef<Path>, fallback_enabled: bool) -> Result<Self>` | 固定资源目录和 key 回退策略 |
| `lang_pack_path(&self, locale: &str) -> Result<PathBuf>` | 校验标识并构造路径，不读取包 |
| `load_locale(&self, locale: &str) -> Result<()>` | 加载并发布，不保存偏好 |
| `load_locale_and_save(&self, locale: &str, root: &Path) -> Result<()>` | 加载后尝试在指定目录保存偏好 |
| `load_last_used_locale(&self, root: &Path) -> Result<()>` | 读取偏好或使用默认值，再加载 |
| `lookup(&self, key: &str) -> Arc<str>` | 按当前快照的 key 回退策略查询 |
| `game_locale(&self) -> Arc<str>` | 实际加载的 locale，可能不同于请求值 |

### 全局服务

| 函数 | 语义 |
| --- | --- |
| `configure(asset_root: impl AsRef<Path>, fallback_enabled: bool) -> Result<()>` | 一次性配置进程服务，不加载包 |
| `lang_pack_path(locale: &str) -> Result<PathBuf>` | 配置或初始化服务后构造包路径 |
| `load_locale(locale: &str) -> Result<()>` | 加载并通过 `paths::preference_root_path()` 尝试保存偏好 |
| `load_last_used_locale() -> Result<()>` | 从上述目录恢复偏好并加载 |
| `lookup(key: &str) -> Arc<str>` | 查询；配置或加载前返回原 key |
| `game_locale() -> Arc<str>` | 查询实际语言；加载前返回 `en-US` |

## 错误与并发语义

文件 I/O、YAML schema、非法 locale、必需英文包失败和重复全局配置通过 `Result` 返回。目标包不可用但英文替代成功时，加载返回成功。偏好读取错误使用默认值，偏好写入错误仅告警。

同一服务用互斥锁串行化加载、发布和偏好写入。查询通过 `ArcSwap` 获取完整快照，不等待加载锁或磁盘 I/O；缺失 key 缓存另用短时互斥锁。锁覆盖发布和该次保存，避免一个加载调用在另一次发布之后再写入旧请求。偏好落盘不是与内存发布共同提交的事务，不带保存的加载或保存失败都可能使两者不同。独立服务或其他进程写同一偏好文件时采用最后替换者生效的规则。

每次查询使用一个快照；分别调用 `lookup` 和 `game_locale` 不构成跨调用事务。警告在内部锁释放后调用应用 logger；同线程嵌套本地化警告被抑制，guard 在 logger panic 展开后恢复。logger 的 panic 会继续向调用方传播。

## 最小用例

例子只写独立临时目录，不配置全局状态或访问默认用户目录。

```rust
use lang_lib::Localization;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> anyhow::Result<()> {
    let suffix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = std::env::temp_dir().join(format!("lang-example-{}-{suffix}", std::process::id()));
    std::fs::create_dir(&root)?;
    let assets = root.join("lang");
    std::fs::create_dir(&assets)?;
    std::fs::write(assets.join("en-US.yml"), "Close: Close\nOnlyEnglish: Fallback\n")?;
    std::fs::write(assets.join("fr.yml"), "Close: Fermer\n")?;

    let service = Localization::new(&assets, true)?;
    service.load_locale_and_save("fr", &root.join("preferences"))?;
    let held = service.lookup("Close");
    assert_eq!(&*held, "Fermer");
    assert_eq!(&*service.lookup("OnlyEnglish"), "Fallback");
    service.load_locale("en-US")?;
    assert_eq!(&*held, "Fermer");

    std::fs::remove_dir_all(root)?;
    Ok(())
}
```

## 开发与验证

在 `games_test/tiny-glade` workspace 根执行；`--offline` 要求依赖已缓存。

```sh
cargo test --offline -p lang-lib --all-targets
cargo clippy --offline -p lang-lib --all-targets --no-deps -- -D warnings
cargo fmt -p lang-lib -- --check
cargo run --offline -p lang-lib --example review_missing_cache
```

环境策略探针需要显式提供其可写临时目录：

```sh
probe_root=$(mktemp -d)
cargo run --offline -p lang-lib --example verify_locale_policy -- "$probe_root"
```

`--all-targets` 编译 examples 的测试目标，不执行其 `main`。`verify_original_assets` 需要显式的游戏资源目录和探针输出目录；它消费语言资源，不执行原版游戏。

## 限制

- 只提供静态字符串查表；无参数插值、复数/性别规则、自动语言协商或文件热重载。
- 语言文件、翻译数量和字符串长度没有大小预算。1024 条限制仅约束缺失缓存条数，不约束总字节数；容量外查询仍会分配标记。
- 临时文件同步不包含父目录同步，不承诺断电后的偏好持久性。跨进程无加载事务协调。
- 平台文件替换和权限行为需在目标操作系统验证；资源消费探针不能证明完整 UI key 接线或原版行为等价。

## 来源与审查记录

源码来源与文件身份见 [foundation-audit.json](../../provenance/foundation-audit.json)，审查记录见 [foundation-rereview](../../reports/foundation/2026-10-17-foundation-rereview/README.md)。
