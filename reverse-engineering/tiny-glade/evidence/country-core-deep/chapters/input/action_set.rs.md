# input/action_set.rs

> 文件作用：**输入动作集状态机**（MainActionSet<A=Vec2, D=DigitalActionState> / ActionSetAction /
> DigitalActionState / DigitalActionSetActionType）。46 个命名动作（serde_json 键）的状态/数值集。
> 算法深挖（2026-10-15，详见 `evidence/country-core-deep/input/ALGORITHMS.md` §1）：
> **双形态**——运行时 `MainActionSet<Vec2,DigitalActionState>` 0x170 B（digital 40×8B{disc i32,val f32}
> @0x00..0x140 + analog 6×Vec2 @0x140..0x170），serde 形态 0x58 B（analog 6×Vec2 @0x00..0x30 +
> digital 40×bool @0x30..0x58，**字段序相反**，§6.1 裁决）。机器码双侧证实（O）：deconstruct
> 0x140946b80 首读 `movsd xmm6,[rdx+0x140]`（analog@0x140）；serialize 0x1400ed690 读
> `param_1[0..0xb]`（12 f32=analog@0x00）+ `param_1+0xc..0x57`（40 字节 digital@0x30）。
> **reconstruction `action_set.rs` `MainActionSet{analog,digital}`（analog 在前）只匹配 serde 形态，
> 不匹配运行时 digital-first 0x170B 形态——登记差异，恢复须双形态建模（不改源码）。**
> `deconstruct` 把非默认动作打包为 `Vec<[u8;12]>`（u16 tag = `slot<<8|kind`，kind0=数字区 tags
> 0x0..0x500，kind1=键槽区 tags 0x1..0x2701；analog 槽 {tag@0, Vec2@4}，digital 槽 {tag@0, disc u32@4,
> val f32@8}；**NaN 也打包**、disc==4 空闲不打包）；`reconstruct` 逆向（12B 步距循环、byte@+3 判
> kind 分支写 92×i32 0x170B 结构、空路径置 92 槽=4，case 0x19/0x20 处 Ghidra 坏数据 halt）。
> 序列化字段名即 46 动作名（`input_space_cursor_move`、`camera_*`、`tool_*`、`ui_*` 等，全清单
> + 串地址/长度/槽位映射见 ALGORITHMS.md §1.2）；Compact 逐非默认字段写 `"name":value`（analog
> SIMD 8×f32 一次判非零、digital 逐字节判 0，全默认空 `{}`），Pretty 同构加缩进；visit_str
> 按长度分派 + SIMD 16B 窗对融合串表（UNK_142c91850..）匹配，0x2e=unknown。
> 证据：deconstruct/reconstruct/serialize×2/deserialize 反编译（本轮新采 0x1400ed690/1416f8680/
> 1416f92d0/140947c80 + wp15 b1_input）+ 原字节（deconstruct 机器码验证 12B 元素步距/槽偏移）。

函数总计 26：业务 13 / 内联无独立符号 1 / 派生胶水 12

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `deserialize::visit_str<serde_json>` | 0x1416f92d0 | 17 | 46 动作名→变体索引 | **O** | 本轮 MCP 43KB 反编译：按键长分派（顶层 0x17/0xe/0x10 三支）+ SIMD 16B 窗逐字符比较常量窗 `UNK_142c91870..`（输入偏移 P vs 表偏移 P-7 后缀共享对齐）；命中写槽索引 param_1[1]（0/6/2/4/0xc/…）；0x2e=unknown |
| `serialize<Vec2,bool,Compact>` | 0x1400ed690 | 17 | serde_json 对象 | **O** | 本轮 MCP：栈上 Vec → `{` 0x7b → 46 槽逐非默认 `"name":value`（analog 槽 packssdw/packsswb SIMD 8 判、FUN_1404bd170 写 f32 对；digital 槽逐字节 0 判、FUN_1404bc390 写单字节）→ `}` 0x7d；全默认空对象；46 名字串地址/长度逐项落盘（0x14293d9c1..0x14293dc0d，融合表） |
| `serialize<Vec2,bool,Pretty>` | 0x1416f8680 | 17 | serde_json 对象（缩进） | **O** | 同 46 槽同偏移；Pretty 写 LF 0x0a+缩进循环（func_0x142923c00）；名字串表副本 @0x142c91cd0.. |
| `MainActionSet::just_pressed` | 0x140946620 | 35 | 按动作槽号（u8 入参 in_DL）判该槽 just_pressed | **O** | wp15 反编译：switch 46 槽 → 每槽读 disc（digital 槽 @8k i32）→ 槽专属跳表（UNK_142ab2c78.. 46 张）→ 返回 bool；非「任一动作」合取，入参即槽号 |
| `MainActionSet::deconstruct` | 0x140946b80 | 43 | 打包非默认动作→Vec<[u8;12]> | **O** | wp15 .c + 本轮 capstone 机器码：analog 槽 0x140+8k 读 Vec2，`movlps` 8B 写 @+4，tag 0x0..0x500；digital 槽 8k/8k+4 读 {disc,val}，disc!=4 才打包，tag 0x1..0x2701；12B 步距 `lea rcx,[rbx+rbx*2]; [rax+rcx*4]` 证实；grow_one 12B 元素；输出 cap-first {0,4,len} |
| `MainActionSet::reconstruct` | 0x140947c80 | 64 | 逆向打包 | **O** | 本轮 MCP 22KB 复采：12B 步距循环（puVar6+3 uint、终止 in_R8*0xc）；byte@B+3==0 → analog 分支（case 0..5 写 Vec2{dword@B+4, dword@B+8}），!=0 → digital 分支（case 0..0x28 写 {disc@B+4, val@B+8}）；尾段 92×i32 写 0x170B（80 digital+12 analog）；空路径 92 槽=4；case 0x19/0x20 Ghidra 坏数据 halt 区（机器码为准） |
| `ActionSetAction::just_pressed` | 0x1409487b0 | 102 | tag==1 && 内态 && 子判别跳表 | **O** | 原字节：`[0]==1 && [1]==DL` + 跳表 UNK_142ab30f0 按 u32@+4 判别 |
| `ActionSetAction::pressed` | 0x1409487e0 | 113 | tag==1 && 内态 && 子判别<3 | **O** | 原字节 |
| `ActionSetAction::as_analog` | 0x140948800 | 124 | Analog→Option<Vec2> | **O** | tag==0 && +1==0 → movsd Vec2@+4，disc 0/1 |
| `DigitalActionState::pressed` | 0x14093fa70 | 240 | idx<3 | **O** | 原字节（变体序 U，见 DIGEST §7） |
| `DigitalActionState::released` | 0x14093fa80 | 250 | (idx-3)<2 | **O** | 原字节 |
| `DigitalActionState::just_pressed` | 0x14093fa90 | 253 | 跳表判别 | **O** | 原字节：跳表 UNK_142ab2c3c 按 disc 分发 |
| `DigitalActionState::just_released` | 0x14093fac0 | 263 | idx==3 | **O** | 原字节 |

## 内联/无独立符号函数

- `is_default`（行 4）— **I**（默认态谓词：analog=Vec2 双零、digital=disc 空闲，名称推断；
  serialize 的跳过逻辑即其证据）

## 派生胶水（12 项：default、deserialize、eq、expecting、hash、visit_map、visit_seq、visit_str 等）
