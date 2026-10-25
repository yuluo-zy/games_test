# B 帧循环恢复（round13，frame13）

本批只读 B 原始发行文件和现有源码；未修改生产文件，未运行 Cargo。可应用补丁为 `candidate.patch`，4 个文件的原始字节 base/candidate SHA256 在 `manifest.json`。2026-10-19 `git apply --check --ignore-space-change` 返回 0。候选含回归，但尚未运行；rustfmt 已成功解析候选，check 因候选/原已有格式差异返回 1，不记测试通过。

## 已恢复的实际差异

1. `systems/time_update.rs` 恢复真实时钟的写入与失败生命周期。原 `time_update@140922c30` 在 replay/live 的 `140922ca0/140922d7f` 先写 GameTime.delta，然后转换 Duration，成功后才写 elapsed、ran、frame_count、total_secs、ticks_60、frac。负数、NaN/过大、累计 Duration 溢出均调用不返回的 panic；原先候选先更新副本、遇错归零三个字段并停止回放，改变了原行为。现直接更新真实 resource，并使用原字符串 panic。原字节取自 RNE，见 `time-update-original.asm`，原字符串及 SHA 在 manifest。
2. Replay 模式缺少当前帧不能退回 live 时间。原 `140922c61..83` 检查当前索引，失败至 `140922f80`，字符串 `Out of bounds access`。候选现显式检查 mode 并要求当前帧存在；正常完整回放末帧仍由已有 set1 advance producer 停止，不改变该成功路径。
3. 删除仅由错误归零分支产生的 `fail_current_frame`/`input_suppressed_this_frame` 状态，避免留下不存在的恢复生命周期。文件加载和命令输入边界现有校验仍保留；本批只纠正已经进入游戏帧的原时钟行为。对应输入链回归改为检查 panic 后 delta 已写入而其它字段未变；Stop 释放光标回归继续保留。
4. `GameLoop::frame` 恢复 Egui 原始输入的条件消费。原 main-loop 的 `1404ddf28/35` 检查 DebugGui `+f0` enabled 和 `+101 bit0` experimental window；任一打开跳至 `1404ded31` 调用实际 `EguiEventIo::take_egui_input@14026dfd0`，其它帧调用 `RawInput::default@140c2c770`。原候选无条件消费 OS Egui 输入。现保留每帧 begin/update/end/render，只按实际 debug 标志选择原始输入或 default。默认构造的指令独立落在 `raw-input-default-original.asm`，依赖 RawInput::default 直接复用 egui。新增 5 个资源/标志组合的回归。

## 已核调用联接与边界

| 链段 | 已读源码和原证 | 本批状态 |
| --- | --- | --- |
| OS/game 请求 → loading/gameplay handoff | GameLoop::frame，GameplayPreparation::poll，main-loop.asm | 保留真实旧光标 lease 撤回、六资源 move、StartupRevealStarted → Vermeer hold disable → Egui owner move。未宣称原 OS 双线程布局已恢复；主线程 winit 是平台适配。 |
| 时间 → App update | time_update、GameClock、set1 → set2 → set5 | 修复本批错误路径；正常 gate/replay/live/前两帧 cap 直接复用已有完整算法。 |
| App update → MEI flush | mei_host::install_mei_frame_systems；loading_graph::flush_mei_frame | set44 的 text/layout/post/flush 在 update 中消费一次。构图重试读取上传后的 output，不调用 consume。未增加 flush 或重新生成 primitives。 |
| Egui begin → update → end → platform output → render | GameFrameDriver::frame；原1404de115/349/3d2/464/5bc | 本批修正原始输入门控，保留现有顺序。加载世界强制安装 EguiContext 和原缺资源分支的完整等价性没有新的独立证据，仍需保留为边界。 |
| render 返回 → temporal frame_index → first-frame log → FPS limiter | render_temporal_state；原1404de63a/640/7da | 现有源码按返回时递增，没有移入 prepare 或构图 retry。 |
| 构图/重试/提交/回收 | frame_setup::with_frame_resources，render_frame，render_bridge | 本批仅读。构图 graph worker 和 GPU worker 分别负责其闭合；不能将本批源码联接检查当完整非空帧/GPU 验收。 |

## 集成 owner 的并行小修建议

`production_renderer::minimal_world_frame_allowed` 的 `diagnostic || config.dev.demo` 没有原证。原 world render 中 demo 只跳过一段折射/水面链，主世界内容仍被构建；原外层 `1404e883b` 按 RenderMode 选择 loading/world，没有因为 demo 缺 producer 而降到 clear+export。root 可独立移除 demo 自动允许和错误文案的 `run demo to allow it`；保留显式 `TG_ALLOW_MINIMAL_WORLD_FRAME=1` 仅为已标明诊断入口。此建议不在本补丁中，避免修改并行 owner 的 production_renderer。

## 工具与证据范围

- 原始 B RNE SHA256：`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`，已重新读取文件核验。
- 原 `time_update` 范围 `140922c30..140922fa6`，实际 0x376 字节，直接 LLVM objdump，不依赖 patched 二进制伪码。
- 已从现有 LLVM PDB public-symbol JSON 定点核实三个调用的真实符号，落在 `symbols.json`：RawInput::default@140c2c770，EguiEventIo::take_egui_input@14026dfd0，country_core::time_update@140922c30。依赖 default 实现不逆向重写。
- 重用 `evidence/country-core-deep/camera-time/decomp/decomp-0x140922c30.txt` 与 `skeleton/task-index/work/TG-40ff43c62ef8791c53/oschain-20261018/main-loop.asm`。观察、源码修复与未运行验收分别登记。
- 唯一 MCP worker `frame13` 启动时 Java LaunchSupport 无法取得 JDK home，backend 自行退出；日志 `tools/reverse-runtime/frame13/backend.log`。没有停止或修改其它 worker，没有把失败调用算原证。转而使用已有 PDB/伪码与原始 RNE 指令。工具修复未占用业务交付。
- T13 按用户要求完全排除。A/C、完整游戏帧运行、PDB 未确认的 Bevy fork/source revision 不作完成声明。
