# tiny-glade A/B — A 清单 24 项独立逐函数复核（reviewer-A-tinyglade）

日期：2026-10-17
角色：`reviewer-A-tinyglade`（只读复核；未改 `crates/tiny-glade/src`、未改库）
范围：`planning/2026-10-17-evidence-audit-interaction.md` §2 的 tiny-glade A 清单 24 项
（tinyglade A 6 + tinyglade B 18；其中 crash_payload 的 6 个 Serialize 单态按 1 项计）。
被复核实现：`crates/tiny-glade/src/**`，`render`/`render_world`/`handle_screenshot_error` 的委托实现
`crates/country-core/src/systems/render_loop/{render_frame,render_world}.rs`。

## 0. 方法与证据

- 工具：Ghidra 持久 B 库 `tiny-glade.exe`（sha256 `cf048e27…6323`），`--worker shared` 只读；
  本轮全部函数**重新 fresh 反编译**（`get_functions`），对关键函数另取 `disassemble_function` 与
  `read_memory`（.rdata 常量/字符串、跳表、字面量逐一读回）。
- 逐函数流程：fresh 反编译 → **独立复述**（按分支/循环/调用/数据流）→ 与 Rust 实现**逐语句/逐常量**对照
  → 三态 verdict（判等 / 需修 / 未实现）。
- 证据目录（本报告同目录 `evidence/`）：
  - `0x<VA>_*.c`：24 项（29 个 VA）fresh 反编译；`disasm_*.txt`：frame/vram/on_message/end_frame/
    send_message/dump_screenshot/coaster/render_world 的 fresh 汇编；
  - `mem/*.json`、`result*.json`、`job*.json`：.rdata 读取、工具返回与可复现 job。
- 工具身份：每次调用均返回 `connected: true, backend: ghidra, worker: shared`，`get_functions`
  `isError:false`；未使用 `--write`。

### 0.1 三态回执

| 判等 | 需修 | 未实现 | 合计 |
|---:|---:|---:|---:|
| **11** | **13** | **0** | **24** |

- 判等升 C（独立复核判等）：`render_world`、`handle_screenshot_error`、`log_time_to_first_frame`、
  `FpsLimiter::frame`、`is_supported_steam_language`、`winit_to_bevy_keycode`、`copy_folder`、
  `calculate_max_resolution_scale`、`dump_log_file`、`dump_gpu_crash_dump_file`、
  `spawn_hang_watchdog_thread`。
- 需修 → 按审计口径仍计 B（有实现，但独立复核发现语句/文案/类型偏差，待修并二次复核）。
- 未实现 0：24 项均有可执行实现；缺的是偏差修正，不是空壳。

### 0.2 逐项判定表

| # | 函数 | VA | verdict | 本轮复核要点 |
|---|---|---|---|---|
| 1 | `render` | 0x1404e8010 | **需修** | 帧编排/分支/常量/日志均对上；OOM panic 的 `Option<(vk::Format,[u32;2])>` **Debug 形状被手写串替代**（FIX-A1）。 |
| 2 | `render_world` | 0x1404ec110 | 判等 | 54 pass 顺序、temporal 名/格式、`FREEZE_OCCLUSION_CULLING`/demo 开关、DoF/TAA 分支、返回值三字段均对上。 |
| 3 | `handle_screenshot_error` | 0x1404ebfb0 | 判等（U） | `Some` + Error 级 → 文案 `Error while capturing a screenshot: `；错误对象为 `anyhow::Error` 并 drop。实现改为返回 `(Level,String)` 由宿主日志（U：生产调用点未接线）。 |
| 4 | `log_time_to_first_frame` | 0x1403db200 | 判等 | `Option<Instant>` niche=1e9、`> Warn` 门、target/文案/Info 级、`elapsed` 与 Duration Debug 全对上。 |
| 5 | `FpsLimiter::frame` | 0x140270850 | 判等 | `0<fps` 门（NaN 跳过）、`used=min(elapsed,2ft)`（NaN→2ft）、clamp±ft、`acc>0` sleep、末尾无条件写 `Some(now)`。 |
| 6 | `is_supported_steam_language` | 0x140323230 内 | 判等 | 15 个语言名按长度 6/7/8/9 逐字节核对全部命中，集合与顺序一致。 |
| 7 | `winit_to_bevy_keycode` | 0x1400fb850 | 判等 | 表 0x14294c58c[256] 实测 `tbl[b^0x80]=b+1 (b<=193) else 1` 无偏差；winit 194 变体与 bevy[1:] 顺序逐名相等；NativeKeyCode 五变体同名同序。 |
| 8 | `fs_util::copy_folder` | 0x140355800 | 判等 | `create_dir_all→read_dir→file_type`；`(attrs&0x10)&&!((0x400)&&symlink)` → 递归，否则 `fs::copy`；两处实例同体。 |
| 9 | `EguiBackend::end_frame` | 0x1400dd910 | **需修** | 原版显式 drop `ViewportOutput` 表与一个按 0x60 步长 `drop_in_place<Shape>` 的 Vec，再 `TexturesDelta::append` 并整体拷回 FullOutput；实现缺该 shapes/viewport 销毁-替换，delta 方向也未闭合（FIX-A2）。 |
| 10 | `calculate_max_resolution_scale` | 0x1404f7f20 | 判等（U） | .rdata 7 常量逐一读回；discrete/unified 分支、host 预算 `/0.0`、`max(min(w,h),1)`、`min(sqrt,10912/axis)`、`(x*10).floor()*0.1`（MULSS 10/0.1 + floorf）全对上。U：workspace 的 budget getter 为 `Result<Option<u64>>`，原版调用点无分支。 |
| 11 | `NvAftermathDelegate::default` | 0x140244460 | **需修** | `tmp/gpu-crash-dump.zip` is_file → unlink 对上；日志等级/文案错：原版 **Error** 级 `Error deleting stale {path}: {error}`，实现为 warn + 另写文案；且逻辑在 `new()`，derive `Default` 不走该逻辑（FIX-A3）。 |
| 12 | `NvAftermathDelegate::shader_debug_info` | 0x140245340 | **需修** | HashMap insert 对上；原版在 `max_level>Debug` 时打 **Trace** 日志 `Got NV shader debug info! Ident: {:?}`（Option<Identifier>），实现完全缺该语句（FIX-A4）。 |
| 13 | `crash_payload` Serialize 6 单态 | 0x14003e250/e7c0/eb80/ec80/e350/eda0 | **需修** | 6 个字段面/顺序/键名全部由 .rdata 键表核对一致（`temp_save_path/session`、`session/folder/backup_folder`、`backtrace/message`、10 字段、`GameNotYetInitialized/Other`、`Panic{payload}/recovered`）。偏差：`CrashRecovered.breadcrumbs` 原类型为 `sentry_types::protocol::v7::Breadcrumb`，实现用自制 struct（wire 形状不同，FIX-A5）。 |
| 14 | `ClientHandle::send_message` | 0x14015ac60 | **需修** | 序列化失败走 `unwrap_failed`（字符串 `called \`Result::unwrap()\` on an \`Err\` value`，43B）→ 原版是 `.unwrap()`；实现 `.expect("…")` 文案不同（FIX-A6）。成功路径转 `send_message_impl` 并返回其结果（U：错误类型映射为自定义 `Error::Connect`）。 |
| 15 | `server::on_message` | 0x1402ae1f0 | **需修** | 原版 `from_slice` 后 **tag==Err → 0x1428d9760 `unwrap_failed`（noreturn）**：坏包 **panic**，不是记录后继续；实现却是 log+Ok（FIX-A7）。 |
| 16 | `dump_log_file` | 0x1403acc70 | 判等 | `tmp/log.txt`→`report/log.txt`、`fs::copy`、context `Copying the log file` 逐字对上。 |
| 17 | `dump_settings_file` | 0x1403ae050 | **需修** | `config/settings.ron`→`report/<file_name>`、copy、context 对上；`file_name()` 原版 `.unwrap()`（`core::option::unwrap_failed`），实现 `.expect("…")`（FIX-A8，P3）。 |
| 18 | `dump_gpu_crash_dump_file` | 0x1403ae570 | 判等 | `tmp/gpu-crash-dump.zip`→`report/`、copy、context `Copying the GPU crash dump file` 对上。 |
| 19 | `dump_rhapsody_log` | 0x1403ae770 | **需修** | 原版 `let data=rhapsody::dump_log(); std::fs::write(report/render.log, data).context("Writing rhapsody log")`；实现 `File::create(...)?`（无 context）+ `dump_log(&mut file)`，错误面与结构不同（FIX-A9）。 |
| 20 | `dump_screenshot` | 0x14058a060 | **需修** | 尺寸取 +0/+4、非零、`w*h<=0x38400`、建父目录、`save_buffer` 对上；但**文件名/格式错**：原版 `screenshot_just_before_the_panic.jpg` + **RGB8 JPEG**（RGBA→RGB，`ExtendedColorType=15`），实现写 `screenshot.png` PNG/RGBA；两条错误文案也缺后缀（FIX-A10）。 |
| 21 | `spawn_hang_watchdog_thread` | 0x140365be0 | 判等 | `Builder::new().name("hang watchdog").spawn(closure)`；返回 `Result<JoinHandle>`，错误分支 drop 后返回。 |
| 22 | `write_image_with_callback_impl` | 0x1403206a0 | **需修** | 长度校验/父目录创建/tempfile persist/回调对上；但原版按扩展名分派（`.jpg`→RGBA→RGB + `JpegEncoder::new_with_quality`；其他→`ImageFormat::from_path`+`write_buffer`），父目录 None 是 **error**（不是回退系统 temp），成功/失败回调载荷是 `Result<[u32;2],String>` 而非 `Result<PathBuf>`（FIX-A11）。 |
| 23 | `CoasterAudioInstance::new` | 0x1403b6880 | **需修** | `0.03>volume→None`（NaN 继续，编译码 JA 证实实现正确）、instantiate→set_volume→逐参数、失败→None；偏差：参数元素步长 **0x18 = `(&str,f32)`**（实现为 `(String,f32)` 32B），错误日志原版为 **Error** 级（实现 warn）（FIX-A12）。 |
| 24 | `confirm_incoming_panic_repro` | 0x1400ed280 | **需修** | 文案常量/消息拼接对上（140B、201B 头、33B 尾、第二对话框 READY 文案逐字节一致）；但控制流是**状态机**：state==2 才弹第一框、show 结果写 `+0x1c` 且 state=3（不因 No 提前返回）；state==1 且 `+0x40` 为 Some 且 `+0x48+1==frame_count` 才弹第二框→state=2；标题含 `😅`；第一框描述还有 227B+69B 包裹（FIX-A13）。 |

## 1. 关键 fresh 证据摘录（逐函数）

> 下列均为本轮自己重跑，不引用 restorer 的结论；数值/字符串均以 `read_memory`/汇编为准。

### 1.1 判等项

- `log_time_to_first_frame`：`MOV RAX,[0x143634b88]`（secs）、`MOV dword [0x14362b0a8],0x3b9aca00`
  （take 写 None niche）、`CMP ECX,0x3b9aca00; JZ`、`CMP MAX_LOG_LEVEL_FILTER,0x2; JBE`、
  `CALL elapsed`、Log 结构 level=3(Info)、target `tiny_glade`(10)、片段 `time to first frame: `。
  Duration 只实现 Debug（无 Display），编译器选用的 formatter 即 Debug，与 `{:?}` 一致。
- `FpsLimiter::frame`：`UCOMISS XMM6,0; JBE`（0<fps，NaN 跳过）；`DIVSS 1.0/fps`；
  `CVTSI2SS` 两段 + `/1e9`（as_secs_f32）；`MINSS/CMPUNORDPS` 组合 = NaN→2ft；
  `SUBSS`(acc+ft-used)；`MAXSS -ft` → `MINSS ft`；`UCOMISS acc,0; JBE` 仅 acc>0 sleep；
  尾部 `[RSI]=now; [RSI+8]=nanos` 无条件。
- `is_supported_steam_language`：switch(len) 的 15 组立即数逐字节核对
  （english/italian/koreana/russian/spanish/swedish/turkish/french/german/polish/
  japanese/schinese/brazilian/norwegian/ukrainian），集合与实现 15 项完全一致。
- `winit_to_bevy_keycode`：表 `0x14294c58c` 256×u32 全量读取，逐项验证
  `tbl[b^0x80] = b+1 (b<=193) else 1`，0 mismatch；winit 0.29 的 194 个变体与
  bevy_input 0.16 `[1..]` 逐名相等；NativeKeyCode 五变体同名同序（汇编为 8B 位拷贝）。
- `copy_folder`：`DirBuilder::_create`→`readdir`；属性条件
  `(attr&0x10)==0 || ((attr&0x400)!=0 && symlink-bit)` → copy，否则递归；
  `fs::copy` 与递归两分支逐句对应；错误上下文两处 closure 与实现一致。
- `calculate_max_resolution_scale`：.rdata 0x1429ecb78..0x1429ecbc8 读回
  `-1365057216 / -47.7399274329577 / 146.1091661136655 / 10912 / -992811760 /
  -372245456 / +47.7399 / +1365057216 / 2^-30`；尾段 `MULSS *10; CALL floorf;
  MULSS *0.1`（Ghidra C 丢失第二次乘，汇编证实）；host 路径 `DIVSD XMM0,XMM11`（XMM11=0）。
- `dump_log_file` / `dump_gpu_crash_dump_file`：路径拼接、`windows::fs::copy`、
  `.rdata` 文案 `Copying the log file` / `Copying the GPU crash dump file` 逐字一致。
- `spawn_hang_watchdog_thread`：立即数 `0x74617720676e6168`+`0x676f646863746177`
  = `"hang wat"+"chdog"`；`Builder::name`→`spawn<fn()>`；Result 错误分支仅 drop。
- `render_world`：fresh 反编译的 pass 序列与 `render_world.rs` 逐项一致：
  prev_depth(format 0x64,usage 0xc,name len 0x17)、6 个 upload slot(1,2,3,4,0x13,0x14)、
  W2–W24、gbuffer 0x65 / velocity 0x61、`[0x14362c0d8]`(FREEZE_OCCLUSION_CULLING) 门、
  W35–W37、history 0x7a、W38–W43、demo 门(`*_DEV_OVERRIDE_IS_DEMO_MODE==0`) 内
  refraction+water/ice(由 0x140ae8320 的结果选，实现 is_winter)、W46–W49、
  `dof_mode==0x13`+settings[0xa9+idx]、mips=8、TAA/TGSR 0xab 分支、W53/W54、
  返回体 depth@0 / dof_prepare@0x24 / taa@0x48。
- `handle_screenshot_error`：`if (err!=0) { if (MAX_LOG_LEVEL_FILTER!=0) log; drop(anyhow::Error) }`；
  文案 0x1429ebb?? `Error while capturing a screenshot: `（target `tiny_glade::render_loop`）。
  实现的分支/文案一致，仅把日志副作用交给宿主（U：无生产 caller）。
- `render`：fresh 主链与实现一致：puffin scope→device unwrap→GameConfig expect→
  DrawLists new/build/sort→swapchain format/extent→mode byte 0?loading:world；
  loading `expect(swapchain)`+compile+unwrap；world compile 失败→读 resolution scale
  (`+0xc f32`)+(-0.1)，`new<=0` 前先打 OOM 日志，再 panic `Out of video memory.
  Swapchain info: `；否则写 changed-tick/scale→ResourcePool::clear→两张 temporal 表重置
  →wait_idle→state 资源写 2→Info log→`compile_main_render_graph` with `[32,32]`
  (0x2000000020)→unwrap→Info log。之后 since_prev(None niche 1e9)、begin_frame、
  drained frames、exec Instant、执行/retire、swapchain state==7 跳过、presentation
  graph 失败 panic 文案、end_frame、present、finish_screenshot_capture、stats、
  refresh budget、GPU pacing(ring min/max, from_secs_f32 panic)、末尾写 last_frame_end。

### 1.2 需修项关键证据

- **FIX-A1（render）**：`core::option::impl$46::fmt<tuple<ash::vk::Format,array<u32,2>>>`
  是 `Option<(vk::Format,[u32;2])>` 的 Debug；ash 0.38 的 `fmt::Debug for Format` 输出变体名
  （`const_debugs.rs:2008`）。实现 `out_of_video_memory` 用 `{}` 打 u32，输出数值。
- **FIX-A2（end_frame）**：fresh 汇编显示 `Context::end_frame` 后：
  `CALL 0x140052830 drop_inner_table<ViewportId,ViewportOutput>`；
  self+0x10 长度、base=self+8、步长 0x60 循环 `CALL 0x1405e75e0 drop_in_place<Shape>`；
  再用保存值覆写该 Vec（`[self+0x10]=len; [self]=…`）；`CALL 0x140c801e0
  epaint::textures::TexturesDelta::append`（self+0x18, 拷贝出的 delta）；最后 15 qword
  +0x70 拷回 sret。实现只做 `take(output.delta)→self.append→output.delta=take(self)`，
  没有 Vec/Viewport 的销毁-替换，也没有证明返回的 delta 是原版的哪一个。
- **FIX-A3（aftermath default）**：错误分支 `MAX_LOG_LEVEL_FILTER != 0` 且日志片
  `Error deleting stale ` + Path Display + `: ` + io::Error Display；符号为
  `aftermath::impl$0::default`（Default trait）。实现放在 `new()`，derive Default 绕过。
- **FIX-A4（shader_debug_info）**：`4 < MAX_LOG_LEVEL_FILTER` 门 + 文案
  `Got NV shader debug info! Ident: `（33B）+ `core::option::impl$46::fmt<GFSDK_…Identifier>`
  （Debug）。实现无该语句。
- **FIX-A5（crash_payload）**：`serialize_entry<…,str_,alloc::vec::Vec<sentry_types::
  protocol::v7::Breadcrumb,…>>`；实现 `Vec<crate::…::Breadcrumb>`（自制 4 字段）。
- **FIX-A6（send_message）**：0x14015ad95 的 `unwrap_failed` 参数 EDX=0x2b、
  字符串 0x14295b9d0=`called \`Result::unwrap()\` on an \`Err\` value`（43B）→ `.unwrap()`。
- **FIX-A7（on_message）**：`CMP R15,-0x7ffffffffffffffc; JZ 0x1402ae959`，
  0x1428d9760 符号 `unwrap_failed`（noreturn）。实现 `Err→log+Ok`。
- **FIX-A8（dump_settings_file）**：`Path::file_name` None → `core::option::unwrap_failed`。
- **FIX-A9（dump_rhapsody_log）**：`rhapsody::dump_log()` 后 `std::fs::write::inner`；
  workspace 已有 `rhapsody::dump_log_vec()`。
- **FIX-A10（dump_screenshot）**：join 字符串 0x1429fd4a9 长 0x24 =
  `screenshot_just_before_the_panic.jpg`；`Chunks(4)`→`[u8;3]` 收集；`save_buffer_impl`
  的 color 字节 `0xf`；image 0.25.1 `ExtendedColorType` 第 15 项 = **Rgb8**。
  文案：`Invalid screenshot data dimensions: ` + ` x ` + `. Dimensions can't be zero`
  或 `. Would be too many bytes.`；保存失败 context=`saving image`；
  资源态 5 的文案=`Graphics resources not available`。
- **FIX-A11（write_image）**：`Path::extension` 分支；`.jpg`→
  `image::impl$50::convert<Rgba→Rgb>`+`JpegEncoder::new_with_quality`；
  否则 `ImageFormat::from_path`+`write_buffer_impl`；第二次 `Path::parent()` None →
  `anyhow!(DisplayError<&str>)`（不复用系统 temp）；持久化/编码错误直接
  `anyhow::Error::from`（无 context）；回调载荷 `{tag=Ok,[u32;2]}` /
  `{Err, "Error writing screenshot image: {error}"}`。
- **FIX-A12（coaster new）**：参数循环 `LEA RAX,[base+count*3]; LEA RBX,[base+RAX*8]`
  → 元素 0x18；元素读 `[+0]=ptr,+8=len,+0x10=f32` → `(&str,f32)`。
  日志门 `MAX_LOG_LEVEL_FILTER!=0`（Error 级）。
- **FIX-A13（confirm）**：0x1400ed280 的 state 机/字段写与两份文案组成（见判定表）；
  标题 .rdata 0x14293d868 = `😅Don't panic, it's only a panic`；
  第一框描述 = 227B 前缀 + 消息 + 69B 后缀；第二框描述逐字节等于实现的
  `READY_DESCRIPTION`。

## 2. FIX 清单（按优先级）

> 本轮不改 src；以下为给 restorer 的逐条修正单，修后需二次独立复核。

| ID | 函数 | 优先级 | 修正 |
|---|---|---|---|
| FIX-A5 | `crash_payload` | P1 | `CrashRecovered.breadcrumbs` 改回 `Vec<sentry_types::protocol::v7::Breadcrumb>`（原依赖 sentry-types 0.31.7）；其余 5 单态保持。 |
| FIX-A7 | `server::on_message` | P1 | `serde_json::from_slice::<CrashPickupMessage>(payload).unwrap()`（坏包 panic，和原版 0x1428d9760 一致），去掉 log-and-continue。 |
| FIX-A10 | `dump_screenshot` | P1 | 目标文件名 `screenshot_just_before_the_panic.jpg`；RGBA→RGB，`save_buffer(path,w,h,Rgb8)`（JPEG）；两条尺寸错误文案补 `. Dimensions can't be zero` / `. Would be too many bytes.`；失败 context `saving image`；资源不可用文案 `Graphics resources not available`。 |
| FIX-A11 | `write_image_with_callback_impl` | P1 | 恢复扩展名分派（jpg：RGBA→RGB + JpegEncoder::new_with_quality；其他：from_path+write_buffer）；parent None → error；去掉实现自加的 context；回调载荷改 `Result<[u32;2],String>`。 |
| FIX-A13 | `confirm_incoming_panic_repro` | P1 | 按状态机重写：state==2 弹第一框并写 show 结果/state=3；state==1 且 `+0x40` Some 且 `+0x48+1==frame_count` 弹第二框→state=2；标题加 `😅`；第一框描述 = 227B 前缀+消息+69B 后缀；返回 void。 |
| FIX-A2 | `EguiBackend::end_frame` | P2 | 复现 `drop_inner_table<ViewportId,ViewportOutput>`、Vec<…>(步长 0x60) 的 drop/替换、`TexturesDelta::append` 的参数方向与返回 delta 来源；明确 self 布局（+0 Vec / +0x18 delta / +0x60 Context）。 |
| FIX-A9 | `dump_rhapsody_log` | P2 | `std::fs::write(report/render.log, rhapsody::dump_log_vec()).context("Writing rhapsody log")`。 |
| FIX-A1 | `render` | P2 | OOM panic 用 `{:?}` 打 `Option<(vk::Format,[u32;2])>`（ash Format Debug 变体名），不要手写 `(u32, [..])`。 |
| FIX-A3 | `NvAftermathDelegate::default` | P2 | 逻辑移入 `impl Default`（或让 `Default::default()` 执行 unlink）；日志改 `log::error!("Error deleting stale {}: {}", path.display(), err)`。 |
| FIX-A4 | `NvAftermathDelegate::shader_debug_info` | P2 | 补 `log::trace!(target:"tiny_glade::aftermath", "Got NV shader debug info! Ident: {:?}", identifier_option)`。 |
| FIX-A12 | `CoasterAudioInstance::new` | P2 | 参数类型改 `&[(&str,f32)]`（0x18 步长）；失败日志改 Error 级。 |
| FIX-A6 | `ClientHandle::send_message` | P3 | `.expect(…)` → `.unwrap()`（43B unwrap 文案）。 |
| FIX-A8 | `dump_settings_file` | P3 | `.expect(…)` → `.unwrap()`。 |

## 3. 非阻断观察（U）

- `calculate_max_resolution_scale`：workspace `rhapsody::PhysicalDevice::{device_local,host}_memory_budget_bytes`
  返回 `Result<Option<u64>>`，原版调用点无分支；实现的 `.ok().flatten().unwrap_or(0)` 是库边界适配，
  需 rhapsody 侧确认原 getter 契约（原版似直接返回 u64）。
- `handle_screenshot_error`：实现返回 `(Level,String)`；生产调用点未接日志（仅测试）。委托宿主需
  以 `tiny_glade::render_loop` target 发 Error 日志。
- `ClientHandle::send_message`：原版直接返回 `minidumper::Client::send_message_impl` 的结果；
  实现映射为自定义 `Error::Connect(String)`（逻辑同“传播错误”，类型面有损）。
- `NvAftermathDelegate::shader_debug_info`：key 原类型 `GFSDK_Aftermath_ShaderDebugInfoIdentifier`
  （SDK 缺失，实现用 `[u8;16]`）。
- `render` 的 `RenderHost` 为设备/SDK 缝；本轮验证的是分支/顺序/常量/文案，宿主方法的真实实现
  （rhapsody 调用）不在本 24 项内。
- `confirm` 第二框标题与按钮集合、第一框 show 结果字段的具体类型仍需 PDB/结构复核。
- `render_world` 的 `is_winter` 上游符号在 PDB 中为 ICF 别名
  `BackupSaveFileKind::is_corrupted`（0x140ae8320）；实现注释已登记，逻辑按返回值选 water/ice。

## 4. 复核边界与复现

- 未修改任何 `src`；Ghidra 只读 shared，无 `--write`。
- 复现：见 `evidence/job*.json` + `result*.json`；`fresh/` 文件已平铺到本目录
  （`0x<VA>_*.c`）。核心命令：
  ```sh
  /Users/yuluo/Desktop/game/tools/mcp-servers/ghidra-mcp/.venv/bin/python \
    /Users/yuluo/.codex/mcp-tools/reverse-operator.py ghidra <job.json> <result.json> --worker shared
  ```
- 本报告只对 A 清单 24 项给 verdict；tiny-glade A 的 44 个未实现与 B 的 26 个未实现未复核。
