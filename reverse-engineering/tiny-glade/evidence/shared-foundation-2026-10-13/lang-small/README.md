# lang-lib：B 运行时恢复核对

对应原作只有 `crates/lang-lib/src/lib.rs` 和 `language_library.rs` 两个 PDB 源文件。本轮继承之前的恢复，独立核对 24 个 lang_lib 编译单元、8 个游戏自有独立函数；没有把 std、ArcSwap、HashMap、yaml-rust2 的泛型实例算作游戏函数。

| 原文件 | 原函数 | 当前实现 | 状态 |
|---|---|---|---|
| lib.rs | lang_pack_path | 同名函数 | assets/lang/{locale}.yml，原字节/PDB及候选验证 |
| lib.rs | load_locale | 同名函数 | 写入请求的 locale，加载失败回退仍保存原请求 |
| lib.rs | load_locale_impl | 同名私有函数 | 主库原子替换、默认包回退、环境开关、fallback 保留 |
| lib.rs | load_last_used_locale | 同名函数 | 从标准用户目录读取；缺失使用 en-US；PDB/控制流核对 |
| lib.rs | lookup | 同名函数 | 未初始化返回输入，主库→fallback，缺键共享缓存 |
| lib.rs | game_locale | 同名函数 | 返回当前库 locale，未初始化 en-US |
| language_library.rs | LanguageLibrary::get | 同名方法 | 缺键 `?_{key}`、全局缓存、只在首次缓存时日志 |
| language_library.rs | LanguageLibrary::try_from_yml | 同名方法 | 原 yaml-rust2，首文档平面字符串映射及原错误/unwrap策略 |

`parse_yml` 是候选验证所需的纯入口；`try_get`/`locale` 表达原查找/字段访问行为，不当作另外发现的 PDB 独立函数。字符串保持静态有效，切换语言后旧文本仍可使用。没有重新实现外部依赖。

本轮实修：原 `0x140b9ca67` 调用 PDB 命名的 `std::env::_var`，返回分支 `0x8000000000000001` 跳过 fallback 加载；恢复为 `std::env::var(...).is_err()`。之前 `var_os(...).is_none()` 错误处理非 Unicode 环境值。空字符串与 `0` 都关闭新 fallback 加载，非 Unicode 值与缺失值都加载 fallback；后续关闭不清空已有 fallback。原汇编见前轮 `paths-language/lang-root.asm.txt`，本轮原字节 hash及独立反编译见 [inventory.json](inventory.json)、[native-result.json](native-result.json)。附带 Rust 1.87 编译器 Result 分支探针只用于解释该分支，不能替代原 1.86 ABI。

[validation.json](validation.json) 记录实际原锁 `--locked --offline` 验证：2 项单元测试、所有目标严格 Clippy、5 个独立进程环境/热替换合同、15 种原发行语言资源解析/加载。新增的 `verify_locale_policy.rs` 用自有临时目录，隔离全局状态；没有修改发行资源或标准用户 Saved Games。

消费者用原 RNE 的 LLVM 有界反汇编确认 15 个真实 CALL 边界，涵盖启动语言加载、崩溃语言恢复、游戏设置/按键/UI 文本、toolbar 模板当前语言。完整表位于 validation.json。319 个原字节 E8 命中只是候选，未确认的命中不称为真实引用。

剩余范围：原程序函数动态差分未执行；恢复 ABI/内存布局不是原 ABI 声明；准确错误日志文本/错误上下文尚未逐条验收；`load_last_used_locale` 的标准 OS 用户目录读取未建立可注入的完整候选场景；原完整 GUI 接线和 A/C 版本尚未验收。CSV/difference 只有依赖声明，24 个本 crate 编译单元中没有其引用或游戏自有开发工具入口，不凭依赖名补造源码。若以后发现未发行开发工具源码证据，再另列恢复范围。

复验入口：`audit.py` 生成源文件/函数/字节/原调用候选库存，`native-job.json` 通过 Windows 持久 SDK worker `foundation-lang-small` 执行，`verify.py` 记录候选验证与原 CALL 边界。Ghidra worker 已持久保存，只修改自己的 B clone，历史 Mac overlays 未合并。
