# startup 分析与审查证据

说明入口：[startup/README.md](../../../crates/country-core/src/startup/README.md)。问题与建议：[startup/REVIEW.md](../../../crates/country-core/src/startup/REVIEW.md)。本轮新增文档和审查证据，没有修改生产 Rust、Cargo 清单或锁文件。

| 文件 | 用途 |
|---|---|
| `startup-tests.log` | 标准限定 Cargo 测试退出 101：锁文件与清单不同步，未进入编译 |
| `probes.rs` / `run_probes.py` | 编译当前六个 startup 文件，复用 fingerprint 匹配的缓存支持库；不写生产 tests 目录 |
| `source-probes.log` / `source-probes.json` | 22 passed；源与支持产物 SHA256，编译/运行结果及源不变检查 |
| `source-probes-harness-failed.log` | 首次 harness 缺环境变量及缓存缺最新 ToD 接口；保留真实失败，未用于验收 |
| `original-load-glade-audio.asm` | 原发行 RNE 的 0x1408be360..0x1408bee10 有界静态反汇编 |
| `original-audio-data.json` / `extract_audio_data.py` | PE 常量读取、参数名长度边界、主题浮点表和原文件 SHA256 |
| `reviewed-sources.json` | 审查输入源文件与证据的内容指纹；不是整个并行工作区的冻结交付 |

在 `games_test/tiny-glade` 下运行：

```sh
cargo test -p country-core --lib startup:: --offline --locked
python3 reports/runtime/2026-10-18-startup-review/run_probes.py
python3 reports/runtime/2026-10-18-startup-review/extract_audio_data.py
```

探针直接包含当前 `load_prefabs`、`startup_session`、`run_game`、`load_glade_audio`、`startup_terrain`、`load_textures` 源码；其依赖是记录在案的缓存快照。缓存被删除或接口不匹配时 runner 会失败，不自动选择另一快照，也不证明最新整个 country-core 已构建通过。22 项包含 4 个当前缺口探针，探针通过不代表修复。

原证据仅 B 原发行文件的局部静态核对，默认 Ghidra/ReVa 数据库未读取或修改；未启动 agent、未对原程序执行行为差分。完整加载、渲染、音频与 A/C 版均不在本轮验收范围。
