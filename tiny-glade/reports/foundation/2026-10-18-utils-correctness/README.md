# utils 第二轮修复验证（2026-10-18）

本目录对应 crates/utils/REVIEW.md 的第二轮修复，采用原生 workspace 清单，离线运行，不调用逆向工具。旧 reports/utils-refactor-2026-10-17 的 131 项记录仅用于历史版本。

新增26项聚合测试：core_properties 6、curve_properties 7、geometry_properties 9、integration_properties 4。每项包含多组确定性输入，案例规模和参考结构见 REVIEW.md；157 是聚合测试总数，不是随机输入数量。

复跑命令（从 tiny-glade workspace 根目录）：

```sh
cargo test --offline -p utils --all-targets
cargo test --offline -p utils --all-targets --release
cargo clippy --offline -p utils --lib --test core_properties --test curve_properties --test geometry_properties --test integration_properties -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc --offline -p utils --no-deps
cargo fmt -p utils -- --check
git diff --check -- crates/utils
cargo check --offline -p country-core --lib
```

验证实际使用 /tmp 下独立 target 目录避开其他任务。source-sha256.json 记录 utils 全部源码、测试/输入数据及 Cargo.toml 的 SHA256。verification.json 汇总最终检查，原始输出分别保存；未忽略失败的历史测试组。

interval-allocations.log 使用上一轮保留的探针直接引用本次源码：

```sh
rustc -O reports/foundation/2026-10-17-utils-refactor/tg-utils-interval-allocations.rs -o /tmp/utils-allocations
/tmp/utils-allocations
```

该探针仅证明预热后固定两段结果的20,000次重复裁剪不分配，不能外推为所有输入零分配。几何采用固定大小数组；RDP 准备省略点时增加误差界复核，可能保留更多顶点，未宣称整体性能提速。未验证 Rust 1.88、其他操作系统、完整游戏/GPU 或原程序等价性。
