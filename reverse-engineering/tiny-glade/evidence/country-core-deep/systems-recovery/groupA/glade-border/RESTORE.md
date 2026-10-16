# RESTORE — system-glade-border（组 A restorer，2026-10-16）

> 实现落点：`tiny-glade/crates/systems/glade-border/`。
> 证据：`groupA/glade-border/RE-ANALYSIS.md`、`groupA/glade-border/decomp/`、BASELINE。

## 实现清单

| 函数 | VA | 状态 | 实现要点 |
|---|---|---|---|
| `display_glade_border` | 0x141be0ad0 | 已实现（语义级） | 8 参按机器码序；Photomode(0x13) 门禁；DisplayOutOfBorder 事件门；边界曲线 → DebugLine（utils::debug_viz：new/color=Red(1)/width=4.0/ui_pass=false，0x140b04e70..f70 链）；曲线点 12B 条目输出（`GladeBorderDebugLines`）；全局==1 → ShowDemoBorderOverlay；config+0x121==0 → perimeter signifier（UiSignifierStream） |
| `feedback_input_out_of_border` | 内联（行 12） | 已实现 | BASELINE 3 参（EventReader + PlayUiSound + Local 缓存）；read().last → 音效 |
| `impl$0::build` | 0x140650ab0 | 已实现（O 面） | add_systems + register_event |

## core 侧接线（delta）

- `resources/glade_border.rs`：新增 `ShowDemoBorderOverlay`（权威位置；render 域 draw_callbacks_a_views 改为 re-export）。
- `game_config.rs`（新）：GameConfig/DevSettings 最小契约（+0x121 边界 signifier 字节、upgrade 0x14090df90 登记、0x14362d6dd 开发者模式常量）+ lib.rs 注册。
- `systems/ui/resources/widget_params.rs`：新增 `PlayUiSound`（add_ui 0x140945c60 注册事件）+ `UiNavState`（同插入资源）。

## 未实现项 / 待 reviewer

1. BASELINE #4/#5（TerrainHeightsData/GameTime）与机器码 param_4/param_5（输出 Vec / signifier 位置源 `*param_5+0x2c`）的对应出入——以机器码为准，候选类型保留签名（reviewer 裁决）。
2. 边界 signifier 记录的位置源（I）。
3. feedback_input_out_of_border 内联体细部（防抖 Local 字段 U）。

## 编译

`cargo check -p system-glade-border`（离线）通过。

## 修复轮 2（reviewer-A + 中央裁决）

- 参数面删自造 `_heights/_game_time` → 机器码 8 参（param_5 = 位置源 `*param_5+0x2c`，以 MainCamera raw +0x2c 假设落地，I 登记）。
- 事件消费改一帧全量（`read().last()`——机器码双缓冲游标推进到底）。
- signifier 记录同步 40B 布局（{1, 1, 位置, perimeter}）。
- 全局 `*0x14362c098 == 1` 精确比较登记（overlay 建模不变）。
