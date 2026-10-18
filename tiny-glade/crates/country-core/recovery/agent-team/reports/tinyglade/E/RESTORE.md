# crates/tiny-glade 主循环/窗口设置 — RESTORE（restorer-tinyglade-E）

日期：2026-10-17
角色：`restorer-tinyglade-E`（B 版，package `tiny-glade`）
范围：`game_loop.rs`、`setup.rs`、`run_game.rs`、`event_loop.rs`、
`winit_bevy_glue`（剩余项）、`vram.rs`。
工具：Ghidra 持久 B 库（`tiny-glade.exe`，sha256 `cf048e27…6323`），
`--worker shared` **只读**；`run_game`/`run_game_thread` 的 Ghidra 反编译超时，
改用 ReVa 专属 `--write` worker `restorer-tinyglade-E`（`run-script` +
`DecompInterface.decompileFunction(..., 300s)`，只读脚本，未改数据库）。
证据落盘：`recovery/agent-team/work/restorer-tinyglade-E/`
（`job*.json`、`result*.json`、`0x<VA>_<name>.c`、`disasm_*.txt`）。

## 三态回执

| 类别 | 数量 | 明细 |
|---|---|---|
| **判等** | 6 | `winit_to_bevy_keycode` 0x1400fb850；`vram::calculate_max_resolution_scale` 0x1404f7f20；`vram::calculate_resolution_scale_mem_estimate_text` 0x1404f8240（修 u64 截断后）；`setup::impl$1::from_winit` 0x140175990；`setup::impl$0::from_winit` 0x140175450（公开等价面）；`run_game::compute_on_dedicated_thread` 0x14032d170 |
| **重建** | 5 | `find_best_fullscreen_video_mode` 0x1400453c0；`setup_window` 0x1400436b0；`gather_pending_video_setting_changes` 0x1404e3380；`GameLoop::extract_available_monitors_info` 0x1404e2470；`EguiEventIo::{new,take_egui_input}` 0x14026de50/0x14026dfd0 |
| **胶水-legitimate** | 4 | `run_game::impl$2::build` 0x1405aeea0（InputPlugin 转发）；`run_game::impl$1::build` 0x14003bb80（ICF 空体）；`EguiEventIo::take_egui_input`（锁+puffin+上游转发，已计入重建行，不重复计业务）；`game_loop::impl$3::build` 0x1404e3370（一行注册） |
| **未实现** | 8 | `GameLoop::{new,run,main_loop}` 0x1404d8100/0x1404d8250/0x1404d9580；`impl$3::build` 系统体；`run_game` 0x140593b70；`run_game_thread` 0x14059d450；`make_temp_window` 0x1405ae010；`run_event_loop` 0x14026e4a0 |

口径：判等=分支/循环/调用/数据流在 fresh 反编译中逐条可对应；重建=控制流已按
fresh 反编译落库但存在已登记的类型/平台等价面差异；胶水=注册/转发/空体，
按 AGENTS 不计独立业务；未实现=无 Rust body 且附分歧点/解锁条件。

## 逐函数判定表

| 文件 | 函数 | VA | verdict | 分歧点 / 证据 |
|---|---|---|---|---|
| `setup.rs` | `impl$1::from_winit` | 0x140175990 | 判等 | `size()` 打包 +0；`*(u32*)(+0x18)`→refresh@+8；`*(u16*)(+0x1c)`→bit_depth@+0xc。fresh 22 行逐指令 |
| `setup.rs` | `impl$0::from_winit` | 0x140175450 | 判等 | `get_monitor_info`→`decode_wide`→`to_string_lossy`；rect→pos/size；输出 40 B。fork 公开 `name/position/size` 同源 |
| `setup.rs` | `find_best_fullscreen_video_mode` | 0x1400453c0 | 重建 | 核心算法判等：当前模式匹配（EnumDisplaySettings 当前刷新×1000 + rect）；`min_by_key = |area_desired−area| + |refresh−desired.refresh|/10 + |bit_depth−desired|*1000`（0x14004611c..16a）；`"Chosen video mode: {mode:?}"`。D1 平台内部 `get_monitor_info`→公开等价面；D2 第二次 sort 方向未锚定；D3 "does not support" 第二实参（fmt<i32>）取 refresh |
| `setup.rs` | `setup_window` | 0x1400436b0 | 重建 | 恢复：监视器选择/fallback、`"Desired monitor not found; using primary"`、`"Attempting to place window on monitor: {m:?}"`、`expect("exclusive fullscreen requires a monitor")`、borderless/exclusive 分支、`assets/window_icon.png`→RGBA→`Icon::from_rgba`、标题 `"Tiny Glade"`、RedrawWindow。D4 参数 ABI 未锚定；D5 WindowAttributes 逐字段布尔面未对上；D6 FullscreenConfig 分辨率载荷未见消费点 |
| `game_loop.rs` | `gather_pending_video_setting_changes` | 0x1404e3380 | 重建 | 控制流逐语句：两段队列 `off=start.saturating_sub(seg.start)`、`rem=seg.end−off`、`cursor=(total−rem2)−rem1`、逐项 `cursor+=1`、`Mutex<u8>` poison→unwrap panic、`flags|=item.byte@8`、unlock wake。E1 类型字段名未锚定（在 game_loop 内建型，未改 A 组 app_config） |
| `game_loop.rs` | `GameLoop::extract_available_monitors_info` | 0x1404e2470 | 重建 | `AvailableMonitors::clear`→`available_monitors()`→每监视器 `MonitorMatch::from_winit`+模式枚举（BTreeSet，`dmFields&0x5c0000` 门）→sort→`VideoMode::from_winit`→push。E2 同 D2；失败分支公开 API 不可表达 |
| `game_loop.rs` | `GameLoop::new` | 0x1404d8100 | 未实现 | 纯字段 memcpy（0xa8B/0x658B/0xa8B/0x2b0B/4×u64、`0x10b8=1e9`、`0x1114=1`）；字段类型未锚定 → 不造结构 |
| `game_loop.rs` | `GameLoop::run` | 0x1404d8250 | 未实现 | `main_loop`→pending fullscreen `execute_in_thread(set_fullscreen)`→GraphicsDevice（hash `0x7c31cf58ebf489e1`）→Peak GPU memory 日志→input stream 序列化写盘→`shut_down_everything`（drop App/SubApp/通道/RenderResourceCache/Graphs/Steam/EguiBackend）。依赖 GameLoop 字段面 |
| `game_loop.rs` | `GameLoop::main_loop` | 0x1404d9580 | 未实现 | 2,564 行反编译：winit 事件泵 + Bevy update + egui + swapchain 重建/失焦 + 设备丢失 + WindowSize/AvailableMonitors/WriteSettingsFileCmd |
| `game_loop.rs` | `impl$3::build` | 0x1404e3370 | 胶水（未实现） | 一行 `add_systems(PostUpdate, EventReader<ApplyVideoSettingsCmd>)`；系统体属 app_config 未恢复，注册空体会吞事件 → 不写 |
| `run_game.rs` | `impl$2::build` | 0x1405aeea0 | 胶水 | 5 事件 + 2 资源 + 2 系统；转发 `bevy_input::InputPlugin`（上游额外手势/gamepad，已登记不影响消费面） |
| `run_game.rs` | `impl$1::build` | 0x14003bb80 | 胶水 | Ghidra 解析为 `finish<MainCameraPlugin>`：ICF 单字节 RET 空体（1,581 引用） |
| `run_game.rs` | `compute_on_dedicated_thread` | 0x14032d170 | 判等 | `sync_channel(1)`→`Builder::name(name)`→spawn→spawn 失败 `panic!("Thread failed to spawn: {name}: {err}")`（0x1429beeb1 文案）→drop handle→`recv().unwrap()` |
| `run_game.rs` | `run_game` | 0x140593b70 | 未实现 | ReVa 3,671 行；骨架：EventLoop→GraphicsInstance（allowlist/blocklist、Aftermath）→available_monitors→setup_window→create_swapchain×3→try_set_fullscreen→make_temp_window×2→Steam→CrashHandlerHooks→游戏线程→EventLoop::run |
| `run_game.rs` | `run_game_thread` | 0x14059d450 | 未实现 | ReVa 6,544 行；~50 `add_plugins` + 42 `insert_resource` + 40 `register_event` + 6 Update 系统集 + SubApp 装配 |
| `run_game.rs` | `make_temp_window` | 0x1405ae010 | 未实现 | 隐藏原窗（set_visible false）、`GetClientRect`（失败 panic）、WindowBuilder（`"winit window"`→`"Tiny Glade"`）、主监视器定位、`window::init`、WindowState 标志、RedrawWindow |
| `event_loop.rs` | `EguiEventIo::new` | 0x14026de50 | 重建 | 0x120（`egui_winit::State`）+ `{1,1,0}` 包装标志 + 0x138 堆块；F1 外壳 24 B 语义未锚定 → 取 `Mutex<State>` |
| `event_loop.rs` | `EguiEventIo::take_egui_input` | 0x14026dfd0 | 胶水（判等） | `Mutex::lock` + poison unwrap + puffin scope + `State::take_egui_input` + unlock/wake |
| `event_loop.rs` | `run_event_loop` | 0x14026e4a0 | 未实现 | 1,102 行 winit Windows 回调胶水；GameEvent/OsWindowEvent/GameLoop 字段未落库 |
| `winit_bevy_glue.rs` | `winit_to_bevy_keycode` | 0x1400fb850 | 判等 | `*(u16*)(+0x58)==5`→表 `0x14294c58c[b^0x80]`；否则 tag=0+8 B 载荷拷贝。fresh 22 行一致 |
| `vram.rs` | `calculate_max_resolution_scale` | 0x1404f7f20 | 判等 | `.rdata` 常量、`/0.0` 宿主分支、`min/max` NaN select、`floorf(x*10)*0.1` 逐指令 |
| `vram.rs` | `calculate_resolution_scale_mem_estimate_text` | 0x1404f8240 | 判等（修正） | B 先 `as u64`（饱和）再 `*2^-30`；`SYS` 项 `scaled*0.0+pixel*0.0+reserve`；`format_inner` Placeholder `Count::Is(2)`→`{:.2}`。本批已改 |
| `winit_bevy_glue.rs` | `winit_code_to_bevy_keycode` | — | 非业务 helper | 仅测试使用，已收为 `#[cfg(test)]`；不冒充原函数 |

## 关键 fresh 证据

1. **find_best 评分公式**（0x14004611c–0x14004616a 反汇编逐条）：
   `SUB R11,RDI`（bit_depth 差）→ `IMUL R11,RDI,0x3e8`（×1000）；
   `IMUL RSI,RDX; IMUL R9,R10; SUB`（面积差）；`|refresh diff|` 用
   `0xcccccccd >> 0x23` 做无符号 /10；三者和即 key。`R12+0x30` 为
   `Option<render::VideoMode>` 判别式，`+0x34/+0x38/+0x3c/+0x40` 为其
   resolution/refresh/bit_depth。
2. **gather_pending 布局**：0x1404e3380 反编译 + 段内 `+0x10`(end)/`+0x18`(start)/
   `+0x30`/`+0x38`/`+0x40`(total)，item stride 0x10、标志字节 @+8；全局
   `Mutex<u8>`（state/poison/data 三字节）。
3. **run_game/run_game_thread**：Ghidra `get_functions` 返回
   `Exception while decompiling … process: timeout`；ReVa
   `DecompInterface.decompileFunction(f,300,monitor)` 成功（24.9 s / 147.2 s），
   3,671 / 6,544 行已落盘。
4. **vram 精度**：`format_inner` 的 Placeholder 常量（`0x1404f86d1..0x1404f8751`）
   两处 precision 均为 `Count::Is(2)`；usage 的 f64→u64 饱和序列在
   `0x1404f8240` 分支内显式（`<0→0` / `>1.8446744e19→u64::MAX`）。

## 8 条分歧检查表命中

- 修正：`calculate_resolution_scale_mem_estimate_text` 原实现未做 f64→u64 截断
  （第 3 条“载荷平面近似”同类），已按机器码补 `as u64`。
- 已登记未闭合：D1/D2/D3（平台等价面与未锚定 sort/日志实参）、D4–D6（setup_window
  参数 ABI 与属性逐字段面）、E1/E2（资源类型名/布局、失败分支）、F1（EguiEventIo
  外壳）；均为显式未实现/重建附分歧点，无近似体冒充判等。
- 未发现自行编造事件/命令（第 4 条）；未发现门控极性反向；无 callee 转发错。

## 构建门修复（B 组文件，仅 2 处兼容性）

并发/既有变更使 `parking_lot` 启用 `deadlock_detection` 后：
1. `hang_watchdog.rs`：`check_deadlock()` 返回 `Vec<Vec<_>>`（旧 `Option`）——
   改为 `let deadlocks = …; if !deadlocks.is_empty()`，控制流不变。
2. `panic_reporter/reverse_backtrace.rs`：panic hook 以 `{:?}` 记录该适配器，
   补 `impl Debug` 委托 `Display`（保留行反转语义）。
两处均为构建门必需，未改动 B 组其他逻辑。

## 构建与测试

- **`cargo check -p tiny-glade`：通过**（2026-10-18 00:05 最终复跑，强制 touch
  本组 6 个源文件后 `Finished`；仅 `special_module_name` 警告，其余 3 条警告在
  并发 agent 的 `screenshot.rs`）。2026-10-17 23:49 本组首次全量 check 亦通过。
- **`cargo test -p tiny-glade --lib`：14 passed / 0 failed**（含既有
  `winit_bevy_glue` 映射测试）。
- 复查期间同 crate 有并发 agent 改写 `app_config.rs`/`logging.rs`/`icc.rs`/
  `screenshot.rs`/`egui_backend.rs`/`panic_reporter` 与 `tg-text`；其窗口期内的
  失败均已由该 agent 修复，最终门通过。本组未触碰这些文件（仅前述两处构建门修复）。
- 新增依赖：`egui-winit = "=0.27.2"`（workspace 条目；Cargo.lock 校验和
  `2e3da0cb…`，与原 B 锁一致）。

## 交付文件

`crates/tiny-glade/src/`：
- `setup.rs`（from_winit 判等；find_best 重建；setup_window 重建）
- `game_loop.rs`（gather/extract 重建；new/run/main_loop 未实现登记）
- `run_game.rs`（compute_on_dedicated_thread 判等；插件胶水；三入口未实现）
- `event_loop.rs`（EguiEventIo 重建/胶水；run_event_loop 未实现）
- `vram.rs`（两函数判等；截断修正）
- `winit_bevy_glue.rs`（判等复核；helper 收为 cfg(test)）
- `Cargo.toml`（+`egui-winit`）

## 复现

```sh
cd /Users/yuluo/Desktop/game/games_test/tiny-glade
cargo check -p tiny-glade
```
证据：`crates/country-core/recovery/agent-team/work/restorer-tinyglade-E/`
（`result1..8.json`、`reva-rungame-result2.json`、`0x*.c`、`dis_find_best.txt`）。
