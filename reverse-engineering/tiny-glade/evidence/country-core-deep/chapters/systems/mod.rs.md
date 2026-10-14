# systems/mod.rs

> 文件作用：**合并模块树（wp1-20 系统模块 re-export）+ `GameSystemSet` 枚举（1 字节判别）+ `is_gameplay_active` 谓词。GameSystemSet 的 dyn SystemSet 方法（dyn_clone/as_dyn_eq/dyn_hash）确认判别为 1 字节。**

函数总计 8：业务 4 / 内联无独立符号 0 / 派生胶水 4

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::is_gameplay_active` | 0x140a90e00 | 205 | 谓词：两参数（Bevy Res 对），`[rcx]→byte@+0x30 == 1 && [rdx]→byte@0 == 1`（顺序：先查 rcx 资源 +0x30 状态，失败返回 0；再查 rdx 资源 @0）。资源身份 U（无 xref 证据）。 | O | 机器码 rne-read --hex 0x140a90e00（手解） |
| `country_core::systems::_::impl$0::dyn_clone` | 0x140a90e20 | 86 | `Box<dyn SystemSet>::dyn_clone`：分配 1 字节并拷贝判别 → `GameSystemSet` 判别仅 1 字节。 | O | decomp/decomp-0x140a90e20.txt |
| `country_core::systems::_::impl$0::as_dyn_eq` | 0x140a90e70 | 86 | `as_dyn_eq`：`mov rax,rcx; lea rdx,[rip+static_vtable]; ret` → 返回 `self` + 静态 DynEq vtable 的胖指针（`&dyn DynEq` 转换）。 | O | 机器码 rne-read --hex 0x140a90e70（手解） |
| `country_core::systems::_::impl$0::dyn_hash` | 0x140a90e80 | 86 | `dyn_hash`：先 `state.write_u64(0x3da5601bd502516a)`（类型标签），再 `state.write_u8(判别)`。 | O | decomp/decomp-0x140a90e80.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`eq`、`fmt`、`hash`、`next` 等 4 项（`impl$0::next`=GameSystemSet 变体迭代 next，`fmt`@0x140037760、`hash`、`eq` 为枚举 derive）

## 备注

- 重构 `GameSystemSet`（9 变体，Clone/Copy/Debug/PartialEq/Eq/Hash）与证据一致（1 字节判别）。
- `is_gameplay_active` 的 ECS 参数面（两个 `Res`）与重构 `&mut World` 单参不符（见 DIGEST 差异节）。
