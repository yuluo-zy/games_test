# 三个基础库的独立构建入口

此工作区只引用 `../shared-foundation/crates` 中同一份生产源码；没有复制实现。用于独立开发、复验和发布，避免其他并行任务尚未提交的成员影响三库。

```powershell
cargo test --locked --manifest-path reverse-engineering/tiny-glade/reconstruction/small-libs/Cargo.toml
cargo clippy --locked --manifest-path reverse-engineering/tiny-glade/reconstruction/small-libs/Cargo.toml --all-targets --features gpu -- -D warnings
cargo run --locked --manifest-path reverse-engineering/tiny-glade/reconstruction/small-libs/Cargo.toml -p tg-text --features gpu --example text_gpu -- "D:/game/ljxsj_92385/Tiny Glade" "D:/game/text-gpu-output"
```

字体流水线测试需要本地原作字体；RTL 独立验证使用 Windows Arial。GPU 示例需要原 compiled shader，原字体/shader/发行二进制不随源码发布。当前 Bevy 分叉身份及 rhapsody 已登记的 gpu-allocator 0.27 候选偏差仍保留，不能据此声称原引擎整体等价。
