# T3 CONTRACT — input update_mod_key_state（0x140af9c00）handler 跳表 + u32@+0 语义

> 逆向 agent ① 交付。证据：capstone 0x140af9c00 全反汇编 + 双跳表手解（RNE）。
> 状态：**跳表结构与 handler 分派定案；逐组键码→标志位映射部分待取证**。

---

## 1. 函数骨架（O）

```
0x140af9c08  r11 = *param_1                  ; 修饰键配置资源（指针）
0x140af9c0b  cmp [r11+0x78], 3; setb [*param_2] ; 状态 <3 → 写字节@0（disable_snapping 位）
0x140af9c15..1c  [param_2+0x1c] → *[param_2+0x10]  ; EventWriter 写回模式
0x140af9c24  esi = [r11+0x78]                ; 状态 u32（= 键组选择器）
0x140af9c28  lea rdi,[rip+0x1fec321]         ; 表1 @0x142ae5f50
0x140af9c2f  movsxd rsi,[rdi+rsi*4]; add rsi,rdi; jmp rsi
  → handler 写 [*param_2 + 1]（字节@1 = just_pressed 沿）
0x140af9c54  r10 = *[r10] ; r11d = [r9]（动作码）; rdi = r11-5
0x140af9c90  lea rbx,[rip+0x1fec2cd]         ; 表2 @0x142ae5f64
  → 6+ handler（动作码 r11-5 ∈ 0..）
```

## 2. 表1（状态 `[config+0x78]` ∈ 0..7，8 条目，基 0x142ae5f50）

| idx | handler | 行为 |
|---|---|---|
| 0 | 0x140af9c4b | `mov r11b,1` → 字节@1=1（**强制 ON**） |
| 1,3,4 | 0x140af9c38 | `xor r11d,r11d` → 字节@1=0（**强制 OFF**） |
| 2 | 0x140af9c3d | `xorps xmm0; ucomiss xmm0,[r11+0x7c]` → setae（**阈值 [config+0x7c] f32 比较**） |
| 5 | 0x140af9ca0 | 读 `[r9+4]` vs `[r10+4]` 键状态字节（**动作/按键组 handler**） |
| 6 | 0x140af9c8c | 动作组 handler |
| 7 | 0x140af9d24 | 动作组 handler（Decor 模式区间 0x38..0x3f 判定并入） |

→ "7 组按键 handler" = 状态 0..7（8 条目，1/3/4 合并 = 7 个逻辑组）。组 0=常开、1/3/4=常关、
2=阈值、5/6/7=按键/动作组。

## 3. Modifiers.flags u32@+0 语义（O 部分 + 现源）

- 现源 `Modifiers{flags:u32@0}`：字节@0 = disable_snapping、字节@1 = just-pressed 沿
  （0x1408c5680/0x140af9bf0 已证谓词）。
- 原生 update_mod_key_state **写字节@0（`setb`，状态<3 门）+ 字节@1（handler 分派结果）**；
  经 param_2（EventWriter 写回模式）落盘。
- **待取证**：u32 高位 2 字节语义（未在谓词读面，不猜）；`[config+0x78]` 状态值与键组
  （ctrl/shift/alt/…）的具体对应；表2 动作码（r11-5）的键码枚举。

## 4. GameProfile.key_bindings 40×12B

- input-DELIVERY 已证：`BindingsTable::default` 40 槽逐字节复核 0x14089b110（40 × 12B 直读面）。
- 原生 GameProfile 直读该表（serde 载入覆盖 default）——**② 按现源 BindingsTable 直读实现即可**；
  40×12B 元素布局（每槽键码×2 + 判别）随 input-SPEC/DELTA 已落地，不需重取证。

## 5. 可实现性
- **可直接落地**：表1 8 条目 + 3 类 handler 行为 + `setb 字节@0` 门（状态<3）。
  ② 若落地 update_mod_key_state producer：按状态选择器 `[config+0x78]` 分发，写 flags 字节@0/@1。
- **待取证**：状态值↔键组映射、表2 键码枚举（不造）。

## 6. 证据索引
- capstone：0x140af9c00 全函数（本契约 §1/§2）；表1 基 0x142ae5f50、表2 基 0x142ae5f64
  （手解 LEA disp：48 8d 3d 21c3fe01 → +0x1fec321）。
- rne-read：0x142ae5f50/0x142ae5f64 表字节（fe.. 符号偏移）。
- 现源 systems/backend/input/mod_key.rs（Modifiers 布局 + 已证谓词）。
