# 已恢复的系统 CPU 子函数

本目录将独立草稿中有明确原码合同的已完成子函数接入正式工程。它们是系统输入/输出的纯 CPU 接口；原模块归属、已恢复范围和调用缺口分别如下。接口不声明原 Rust ABI，也不创建猜测的游戏调度/世界状态。

| 正式文件 | 原功能模块 / 来源恢复文件 | 已恢复 | 还差 |
|---|---|---|---|
| tree.rs | `system-tree/process_tree_cmds.rs`；`reconstruction/standalone/tg-tree/src/despawn.rs` | 已解析 marker/animation 输入的删除条件 `<0.01`，含 NaN 不删除 | 原 TreeAnimation 与 DespawnPrefabCmd 生产消费、树完整生命周期；TreeStorage 插入溢出规则未确认 |
| water.rs | `system-water/water_audio_grid.rs`、`country-core/world_raster`；`standalone/tg-critters-water/src/water/{audio_grid,stroke}.rs` | 帧位公式、双 u16 增减、raster 三闭包与选择、coverage/brush 定点换算 | 栅格/音频资源全 schema、真实查询、zeno 覆盖度尺度/宽度和上传；未搬入 capsule AA 替代 |
| critters.rs | `system-critters`；`standalone/tg-critters-water/src/critters/components.rs` | raw locomotion 的 2/4/3 比较与 99 位掩码 predicate | 种类枚举映射、碰撞/水/墙查询、生成/销毁与移动状态机 |
| audio.rs | `country-core/systems/audio`；`clutter-audio/.../audio/{wall_kinds,music}.rs` | 24 项墙音频 kind 表、MusicRichness 边界排除、NewGladeTime 累积 | 真 FMOD loop/oneshot、listener、MusicPlayer、平滑和系统调度；未搬 `smoothed=target` 占位 |
| clutter.rs | `country-core/resources/history/auto_clutter_edit.rs`；`clutter-audio/.../clutter/auto_clutter_edit.rs` | `{"Delete":{"id":u64}}` 外部标签 serde 合同 | RMB 拾取/历史写入/保护集/真实 replay；未搬空 History::add 或 unimplemented 保护删除 |

来源完整路径、源文件哈希、提交和接口变更见 [small-system-audit.json](../../../../provenance/small-system-audit.json)。公共依赖和 serde 派生直接复用，不作为手写系统算法计数。

逆向证据位于 `reverse-engineering/tiny-glade/evidence/multi-baseline/`：`tree-tool-rest-analysis.md`、`water-extra-analysis.md`、`critters-extra-analysis.md`、`audio-extra-analysis.md`、`clutter-extra-analysis.md`。树/水/小生物独立源码由 Git `78be3e3f` 保存，clutter/audio 草稿由 `e4ce4585` 保存；它们的 whole-module 和演示调度没有在此沿用。

水 coverage API 改用 `u32`，只接受合法非负输入；`apply_write` 的权重必须为 `0..=65536`。`apply_audio_bit` 的 bit 必须来自 `frame_bit` (`0..64`)，present 必须由真实查询解析。brush 与音乐数值子函数当前合同限于有限值；超出原证据输入域不声明原版等价。非法水模式返回 None 是接口投影，原入口相应分支 panic，应由调用者选择错误传播策略。

测试复用原草稿已有的常量、独立 wire 字串、PADDW/PSUBW 溢出和数值边界案例；可执行 `cargo test -p country-core recovered_systems`。这是当前源码编译/合同回归，不能替代完整原程序/GPU/音频/系统验收。
