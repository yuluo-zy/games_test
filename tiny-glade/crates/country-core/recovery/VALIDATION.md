# 基础目录恢复与核验，2026-10-14

历史报告：以下结果不覆盖 2026-10-15 的 GUI 拥有状态、LUT 接线和 uuid 依赖增量。最新实现范围见 contracts.json；按用户要求该增量只作编译检查，不扩充业务验证。当前 inventory.json 为实时清单，不是本报告当时的源码快照。

本次是 core 基础资源、工具和墙高亮单系统的限定交付。完整 core、完整目录、原全局调度与整游戏均未验收。当前代码/源码身份/测试输入哈希见 [inventory.json](inventory.json)，逐文件剩余见 [contracts.json](contracts.json)。

## 身份与范围

- 目标：原始 B `tiny-glade.rne`，SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。每个原指令脚本执行前核验该 SHA。
- B PDB SHA256：`2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911`。共享 Ghidra 仅作只读导航，发行 RNE 执行与 patched 分析库分开。
- Rust：`rustc 1.98.1 (48a229cea 2026-09-01)`，`aarch64-apple-darwin`，Debug 与 Release；Git 基点 `6fab8ea05442675b6cd1fb7aba2dd057f22791a3`，本次为工作区修改。
- `Cargo.lock` 未变，SHA256 `5168487eed7ed7bed42122cafbc803687572a0697ac74cb0a6a21d7dd7a4926b`。所有新增游戏实现均在 country-core；未改上层 runtime。

## 原侧执行

| 文件/闭包 | 原指令范围与输入 | 结论/排除边界 |
|---|---|---|
| `resources/ui_settings.rs` | default `0x140b12f10`、horizontal `0x140b0e070`；JSON `deserialize_struct 0x14142eba0`，128 种字段存在组合 | 7 字段/default/缺字段回退逐字节一致；serde 继续复用；只桥接公共 CRT memcmp；未执行完整 RON 或 JSON 错误入口 |
| `resources/window.rs`、屏幕尺寸 | `0x140949bf0/c00/c10/c20` 和 `0x140ac8420`，10 个尺寸组合 | 构造/set/数值转换/比例/屏幕参数逐位一致；含 0、高 u32、2^24 周边 |
| `resources/gui.rs` | `0x14093fbc0..0x14093fc70`，72 个 tag/payload/flag 组合 | 7 谓词（3 个同码别名）一致；仅有效前缀读取，不证明完整菜单 enum 载荷/析构 |
| `resources/color_icon.rs` | `0x140b06770/0x140b06780` | 三个亮度 tag 和默认 scalar；不证明原变体拼写/图标注册 |
| `resources/editing_highlight.rs` | `0x140976890..0x1409768ac`，3 个 tick/不同原字段字节 | 真 ResMut 入口的有效13字节清零与 tick 写入；尾部字节保留 |
| `utils/dirty_tiles.rs` | `0x140960b20..0x140960c16`，881 个边界/随机位模式 | tag+两个坐标、None 不写 payload；公共 UCRT fmaf 用宿主 C fmaf 桥，Windows CRT 本体不计验收 |
| `systems/wall/wall_highlight.rs` | `0x140981390..0x1409815cc`，96 个双缓冲/已读游标/末条命令/启用状态组合 | 完整私有系统指令、真实原 map 键比较和 stores 执行；仅公共 hash 计算桥为受控 hash0；不证明原调度或分配失败 |

三个可重跑脚本和保留取证分别位于：

- `../reverse-engineering/tiny-glade/skeleton/task-index/work/TG-2f6c8496785e2ca5d2/core-foundation-20261014/verify_native.py`
- `../reverse-engineering/tiny-glade/skeleton/task-index/work/TG-f633d8ae33d6b16434/core-foundation-20261014/verify_native.py`
- `../reverse-engineering/tiny-glade/skeleton/task-index/work/TG-33a358ca2d36b0dcc2/core-foundation-20261014/verify_native.py`

以上路径相对于 workspace 根。解释器为 `/Users/yuluo/Desktop/game/tools/venv/bin/python`；每个脚本接收原 RNE 路径和输出 JSON 路径。输出对应 `tests/fixtures/{core-foundation-b,dirty-tile-cell-b,wall-highlight-b}.json`。脚本没有修改发行文件或共享数据库。

## 当前 Rust 接线

`WallEntities` + `Events<HighlightWallCmd>` -> 真实 Bevy `EventReader/Res/ResMut` -> `wall_highlight` -> `WallHighlight` / `EditingHighlight` -> core draw 消费。单系统用 `SystemState` 和 `Schedule` 运行，覆盖两代事件、已读游标、无事件下一帧、最后命令缺失、保持橡皮擦开关；Events 本身不被 drain。

`utils::drain_events/get_last_event` 是另外的、明确消耗事件的工具，复用 Bevy 0.16.0 两代缓冲。测试覆盖顺序、计数器延续、丢弃非末条载荷及缺资源 panic；Bevy fork 的注册/调度/原 panic 文案仍未声明一致。Transform look_at/abs_diff_eq 转发共享恢复实现，不重复数学。

## 保留的失败与纠错

- 初次窗口 0/0：ARM 返回 `0x7fc00000`，原 x86 为 `0xffc00000`。未放宽逐位断言；当前比例函数统一保留原 x86 负 quiet NaN，屏幕尺寸消费者复用它。正常有限数与无限大规则不变。
- 初次 UI 原入口执行遇未解析 CRT memcmp IAT，原失败 PC `0x31953bc`；增加明确的公共依赖桥后重跑全部128组合。未替换 JSON 字段分支、默认值或结果。
- 旧 `FIX-047` 的 f32 bitcast、`FIX-209` 的默认 true/变体1、`FIX-206` 的 is_open 地址和8B完整布局、`FIX-137` 的逐条事件循环均与原指令不符。逐条新裁决写在 contracts.json，历史文件不覆盖。
- Ghidra UiSettings 相关反编译存在错误越界 CFG 和截断；未用这些异常输出补造合同，改用原字节、调用点及原入口执行。

## 检查命令

```sh
cargo test -p country-core --locked --offline --lib --tests --quiet
cargo test -p country-core --release --test core_foundation --test wall_highlight --locked --offline --quiet
cargo fmt -p country-core -- --check
cargo clippy -p country-core --all-targets --locked --offline --no-deps -- -D warnings -A clippy::chunks_exact_to_as_chunks
```

默认测试：1349 库测试 + 11 基础合同测试 + 7 既有墙权威测试 + 2 高亮测试，均通过，无忽略。Release 重验本次 13 个测试。Clippy 延续此前对既有 `chunks_exact_to_as_chunks` 样式建议的命令行豁免，不放宽新代码 lint；原全依赖严格 Clippy 仍有既有 tg-history 告警。rustdoc JSON 生成有34个既有文档链接/HTML告警，清单提取本身成功。

这批完成的是上述明确合同和消费链。GUI 完整拥有状态、PublicWalls 发布/实体增删同步、原事件生产者、全局系统次序及其余 core 业务继续保持未完成；没有独立人工 reviewer 或原整进程/GPU 验收。
