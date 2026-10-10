# Tiny Glade 原作 Rust 还原工程

最终目标是完整复现给定发行构建的游戏行为和画面。当前工程只包含已由原作证据恢复的模块，不能称为完整游戏。

当前工作优先逆向源码。已核查发行目录/PDB嵌入源，目前未取得作者原始Rust文本；这里是按原作机器码恢复并验证的可编译实现源码。GPU与场景回放后置。[源码恢复状态](../SOURCE_RECOVERY.md)列出每个原函数族、对应源文件、已恢复内容和边界。

研究输入为 `D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne`，身份见 `src/lib.rs`。Rust 工程与发行目录分离；没有读取或改动用户之前的 demo。

```powershell
cargo build --release --locked
cargo run --release --locked -- identity
cargo run --release --locked -- aspect 1920 1080
cargo run --release --locked -- roof-state 0.5 1.0
cargo run --release --locked -- roof-profile 0.5 6.0 0.1 5.04
```

当前源码工作流程：真实过程定位 → 数据/调用链证据 → Rust类型与完整函数实现 → 原作机器码校验 → 源码模块联接。之后再做场景与GPU。普通 Cargo 构建只能证明工程可编译，不能代替机器码差分或最终游戏集成。

`roof_pipeline::assemble_roof_tiles_observed`组合完整矩形/圆形CPU瓦片生成。矩形依次调用四边条瓦、脊瓦、面瓦/补片，保留同一RNG/seed序列。CLI `roof-tiles <roof88hex> <rectangle24hex或-> <mode0/1> [输出文件]`可以独立生成64字节瓦片记录；无需原游戏进程或渲染环境。

机器码差分脚本位于 `../scripts`，报告位于 `../evidence`。准确的里程碑、覆盖范围与下一步见 [项目进度](../PROJECT_PROGRESS.md)及 [机器可读追踪](../tracking/project.json)。
