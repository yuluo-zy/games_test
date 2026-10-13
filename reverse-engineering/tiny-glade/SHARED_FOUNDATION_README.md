# 七个共享基础库

当前交付范围：`asset`、`asset-pipe`、`paths`、`lang-lib`、`theme-defs`、`tabloid`、`utils`。

源码位于 `reconstruction/shared-foundation/crates/`。按 B 版原作的业务规则恢复，接口采用可审查的 Rust 形式，不宣称作者源码或原 Rust ABI。通用库使用原锁定版本，不重新实现其算法。

在 `reconstruction/shared-foundation` 执行：

```powershell
cargo test --locked --offline -p asset -p asset-pipe -p paths -p lang-lib -p theme-defs -p tabloid -p utils
cargo clippy --locked --offline -p asset -p asset-pipe -p paths -p lang-lib -p theme-defs -p tabloid -p utils --all-targets -- -D warnings
```

`offline` 要求依赖已缓存；首次获取依赖时可省略该选项。两个 `shipped_*` 测试需要本机 B 版发行目录 `D:/game/ljxsj_92385/Tiny Glade`，原作文件不随 Git 发布。已采集的原指令对照 fixture 位于 `evidence/`，属于测试输入，需与源码一起检出。

当前功能与剩余限制以 [业务功能对照](FOUNDATION_BUSINESS_COMPARISON.md) 和 `evidence/foundation-completion-2026-10-13/` 为准。完整游戏调用方接线不属于本轮七库补全范围；纯宏派生、依赖转发和未产生机器码的声明不能冒充已验收业务。

本地共享工作区可能同时包含其他聊天的库。此次 Git 提交的工作区清单仅登记上述七个库；本地其他库的清单和源码保持原状，分别提交。
