# 复验 harness

country-core 他组在途错误阻断 workspace `cargo check` 时，用本目录独立核验
system-onboarding 编译与单测（依赖树同 workspace；country-core/utils 以最小 stub 替代，
src 逐文件与 `crates/systems/onboarding/src/` 一致）。

运行：

```sh
cd verify/sys-onboarding
cargo test   # 期望 0 错误；8/8 测试通过（组合子/filter_to_mode/cleanup/quantize/HintType）
```

stub 只覆盖本 crate 消费的 country-core/utils 表面；workspace 闭合后以
`cargo check -p system-onboarding` + `cargo test -p system-onboarding` 复核为准。
