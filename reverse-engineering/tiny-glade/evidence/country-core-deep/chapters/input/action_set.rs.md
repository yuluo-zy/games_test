# input/action_set.rs

> 文件作用：**输入动作集状态机**（MainActionSet<A=Vec2, D=DigitalActionState> / ActionSetAction /
> DigitalActionState / DigitalActionSetActionType）。46 个命名动作（serde_json 键）的状态/数值集。
> `deconstruct` 把非默认动作打包为 `Vec<[u8;12]>`（u16 tag = `slot<<8|kind`，kind0=数字区 tags
> 0x0..0x500，kind1=键槽区 tags 0x1..0x2701）；`reconstruct` 逆向（损坏控制流，写 92×i32 0x170B
> 结构）。序列化字段名即 46 动作名（`input_space_cursor_move`、`camera_*`、`tool_*`、`ui_*` 等）。
> 证据：deconstruct/reconstruct/serialize×2/deserialize 反编译 + 原字节。

函数总计 26：业务 13 / 内联无独立符号 1 / 派生胶水 12

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `deserialize::visit_str<serde_json>` | 0x1416f92d0 | 17 | 46 动作名→变体索引 | **O** | SIMD 16B 常量比较 `UNK_142c91850..`；0x2e=unknown |
| `serialize<Vec2,bool,Compact>` | 0x1400ed690 | 17 | serde_json 对象 | **O** | 逐非默认字段写 46 键 |
| `serialize<Vec2,bool,Pretty>` | 0x1416f8680 | 17 | serde_json 对象（缩进） | **O** | 同 + depth/LF |
| `MainActionSet::just_pressed` | 0x140946620 | 35 | 任一动作 just_pressed | **O** | 原字节短函数 |
| `MainActionSet::deconstruct` | 0x140946b80 | 43 | 打包非默认动作→Vec<[u8;12]> | **O** | tag=slot<<8|kind；6 数字态+40 键槽；扩容 FUN_140562cf0 |
| `MainActionSet::reconstruct` | 0x140947c80 | 64 | 逆向打包 | **I** | 0xc 步距 switch；case 0x19/0x20 halt；空路径置 92 槽=4 |
| `ActionSetAction::just_pressed` | 0x1409487b0 | 102 | tag==1 && 内态 && 子判别跳表 | **O** | 原字节 |
| `ActionSetAction::pressed` | 0x1409487e0 | 113 | tag==1 && 内态 && 子判别<3 | **O** | 原字节 |
| `ActionSetAction::as_analog` | 0x140948800 | 124 | Analog→Option<Vec2> | **O** | tag==0 && +1==0 → movsd Vec2@+4，disc 0/1 |
| `DigitalActionState::pressed` | 0x14093fa70 | 240 | idx<3 | **O** | 原字节（变体序 U，见 DIGEST §7） |
| `DigitalActionState::released` | 0x14093fa80 | 250 | (idx-3)<2 | **O** | 原字节 |
| `DigitalActionState::just_pressed` | 0x14093fa90 | 253 | 跳表判别 | **O** | 原字节 |
| `DigitalActionState::just_released` | 0x14093fac0 | 263 | idx==3 | **O** | 原字节 |

## 内联/无独立符号函数

- `is_default`（行 4）— **I**（默认态谓词，名称推断）

## 派生胶水（12 项：default、deserialize、eq、expecting、hash、visit_map、visit_seq 等）
