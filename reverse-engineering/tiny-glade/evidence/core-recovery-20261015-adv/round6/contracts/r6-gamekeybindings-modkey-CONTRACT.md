# Round6 CONTRACT — GameKeyBindings 40×12B 布局 + mod_key 键码

> 逆向 agent ① 交付。证据：capstone 0x14089b110（default 构建器逐槽写）+ 现源 translate_input.rs
> （BindingsTable 已逐字节核对）。状态：**可直接落地（现源已对齐）**。

---

## 1. 40 槽 × 12B 固定数组（O，capstone default 构建器 0x14089b110）

- 布局：`[key u32@0, value u64@4]`，槽 stride **0xc（12B）**，40 槽（0..39）。
- default 写槽 0..35 键码 + 槽 36 [0x1b0]=0xc3（未绑定）；槽 37/38/39 = 鼠标绑定。

## 2. default 键码表（O，capstone 逐槽写；现源 translate_input.rs:106-116 已逐字节核对）

| 槽 | 键码 | 槽 | 键码 |
|---|---|---|---|
| 0 | 0x2a | 20 | 0x27 |
| 1 | 0x26 | 21 | 0x2d |
| 2 | 0x14 | 22 | 0x2b |
| 3 | 0x17 | 23 | 0xc3(未绑) |
| 4 | 0x18 | 24 | 0x38 |
| 5 | 0x24 | 25 | 0xab |
| 6-9 | 0xc3×4 | 26 | 0xa5 |
| 10 | 7 | 27 | 0x1b |
| 11 | 8 | 28 | 0x49 |
| 12 | 9 | 29 | 0x3a |
| 13 | 0xa | 30 | 0x51 |
| 14 | 0xb | 31 | 0x52 |
| 15 | 0xc | 32 | 0x50 |
| 16 | 0xd | 33 | 0x53 |
| 17 | 0xe | 34 | 0x4f |
| 18 | 0xf | 35 | 0x4e |
| 19 | 0x23 | 36+ | 0xc3/鼠标 |

- **0xc3 = 未绑定哨兵**（u16 槽码空间，button_input.rs:14 `UNBOUND_KEY`）。
- **② 待办**：GameProfile 直读应按 **40×12B 固定数组**（非 `Vec<(u32,u32)>` 占位）落地；
  现 BindingsTable 已对齐，GameProfile.key_bindings 改直读即可。

## 3. mod_key 键码（表2 动作码）— **待取证（中优先）**

- update_mod_key_state（0x140af9c00）表2 @0x142ae5f64 动作码 `r11-5 ∈ 0..` 的键码枚举
  与 GameKeyBindings 槽映射未逐键解；② 按 40 槽 + Modifiers 谓词落地 producer，键码待证。

## 4. 证据索引
- capstone 0x14089b110（default 逐槽写 +0x00/+0x0c/... 步长 0xc）。
- crate systems/backend/input/translate_input.rs:94-116（BindingsTable 40×12B 已核对）、
  button_input.rs:14（UNBOUND_KEY=0xc3）、game_profile.rs（GameKeyBindings 待改）。
