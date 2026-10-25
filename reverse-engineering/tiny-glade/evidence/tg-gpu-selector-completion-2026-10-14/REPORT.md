# tg-gpu-selector B 库内规则收口

## 结论

已把现存候选实现的 B 版库内规则逐项写明，并补上驱动建议、graphics 错误投影、Vulkan 分组/allowlist 和六项 workaround 的测试覆盖。本轮没有发现需要改生产策略的已证实差异；生产 Rust 算法未变，新增内容是可复核的规则说明与回归测试。

该结论只覆盖本 crate 已观察的策略。Rhapsody 实际设备枚举和 driver classification 的生产接线、配置结果进入真实 GPU device 创建、跨依赖原始错误 enum ABI、原 iced 完整 view/像素及 A/C 仍未验收。

## 原指令证据

- 原 oracle 是 B 发行 RNE，SHA256：`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 原 GPU update 叶分支和六个 workaround XOR 分支于本轮重新执行：56 输入案例、92 次原函数调用。输出与保留的 `foundation-unstarted-2026-10-13/ui/original-message-cases.json` 完全一致；本轮输出见 [native-update-cases.json](native-update-cases.json)，执行器见 [native_message_oracle.py](native_message_oracle.py)。
- 这些受控案例执行的是实际原 RNE 函数体和 relocated jump tables，不启动游戏，不调用外部 GUI/驱动代码；它们不是完整 update 函数、完整 GUI 或启动链验收。
- 原函数范围和已有反编译绑定在 [B-SURFACE.md](../closures-2026-10-13/gpu-selector/B-SURFACE.md) 与 [UI manifest](../foundation-unstarted-2026-10-13/ui/manifest.json)。历史报告有部分地址/签名占位问题，恢复判断以当前记录的原代码体/调用行为为准，不把 `void()` 当原 Rust ABI。

## 实现与回归覆盖

- `gpu_selection.rs` 新增测试覆盖 vendor 驱动链接、六类 driver support 的建议字段/链接覆写、GraphicsInitError tag/details/recommendation 映射。
- `vulkan_config.rs` 新增测试覆盖 vendor 首次出现顺序、`None` 分组、tagged allowlist 去重与追加、六项 workaround 标签/索引、Cancel 和非法 checkbox 索引保护。
- `BUSINESS_RULES.md` 登记已观察规则、候选表示和未闭合的外部边界；crate README 链接到规则和本报告。

## 验证

- `cargo test -p tg-gpu-selector --all-targets --offline --locked`：12 passed，0 failed。
- `cargo check -p tg-gpu-selector --features native-gui --all-targets --offline --locked`：通过。
- `cargo clippy -p tg-gpu-selector --all-targets --offline --locked -- -D warnings`：通过。
- `cargo fmt -p tg-gpu-selector -- --check`：通过。
- 测试基于当前恢复 Rust crate；除上面标明的 56 个原 RNE update 叶输入外，driver/error/display 案例是按已有静态原代码证据校验候选结果，不称为原函数动态对照。

## 本轮源码 SHA256

| 文件 | SHA256 |
|---|---|
| `tiny-glade/crates/tg-gpu-selector/src/gpu_selection.rs` | `EBCD2159A87752FA5705DC672DEB51EABC638780CC2D1C1A4063ED5A23FD28BF` |
| `tiny-glade/crates/tg-gpu-selector/src/vulkan_config.rs` | `9BA5DF4B4DCDF2091683E14B54483154B15F1AAC328F747101E070160F5D7623` |
| `tiny-glade/crates/tg-gpu-selector/src/lib.rs` | `E346459BC57EAF9779087FF0586E5B5A2016235151E09C9F781C0C8B02D65E6B` |
| `tiny-glade/crates/tg-gpu-selector/src/fixtures/message_cases.rs` | `77C8B23D45C43D049219CF1D8453DF44CDEF67983C96BCB276DE3CD68664994E` |
| `tiny-glade/crates/tg-gpu-selector/README.md` | `97BF36E323C15BFF8B3649BDE5F8D8AEB773123ABA0C4EF7977C145C08A6C8A4` |
| `tiny-glade/crates/tg-gpu-selector/BUSINESS_RULES.md` | `C05EC9650B71BDD5F7AB1D353BF708F6CB0325108E99E9F516E90698F1D3DAB5` |

The selected `3e0387cd` source import remains the base provenance for the original implementation. The hashes above bind this follow-up's documentation and test changes; they do not rewrite that historical import record.

## Tool limitation

The fresh Ghidra MCP `open_database` request for the B patched executable failed before analysis because its worker subprocess exited during spawn. No new database or overlay was created. This round reused the retained, identity-bound decompilation/surface evidence and independently reran the bounded original RNE leaf oracle above.
