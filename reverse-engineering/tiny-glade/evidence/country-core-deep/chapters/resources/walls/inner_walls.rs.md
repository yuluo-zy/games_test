# resources/walls/inner_walls.rs

> 文件作用：内部墙集合资源：HashMap<WallId, WallState 0x70>（条目步长 0x78）+ +0xc0 脏代计数器 + +0x30 墙工厂子结构；new_wall/new_wall_with_id/get/get_unwrapped/remove/max_id/reset_max_id 与 _try_update_state 闭包族（6+3 实例，写偏移即 WallState 布局证据）。

函数总计 58：业务 26 / 内联无独立符号 16 / 派生胶水 16

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::walls::inner_walls::InnerWalls::new` | 0x140859f90 | 0 | InnerWalls::new | O | wall\decomp\decomp-0x140859f90.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::new_wall_from_curve` | 0x14085a200 | 47 | InnerWalls::wall_ids（+0xc0 脏代计数递增 + map 遍历） | O | wall\decomp\decomp-0x14085a200.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::new_wall_w_params` | 0x14085a370 | 59 | InnerWalls::new_wall_w_params（参数化建墙） | O | wall\decomp\decomp-0x14085a370.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::new_wall_with_id` | 0x14085a4a0 | 80 | InnerWalls::new_wall_with_id（FUN_141529bb0 收集 + 尾调 FUN_1402ca2f0(self+0x30,id) 工厂） | O | wall\decomp\decomp-0x14085a4a0.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::reset_max_id` | 0x14085a550 | 94 | InnerWalls::reset_max_id | I | wall\decomp\decomp-0x14085a550.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::get` | 0x14085a560 | 107 | InnerWalls::get | O | wall\decomp\decomp-0x14085a560.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::get_unwrapped` | 0x14085a650 | 116 | InnerWalls::get_unwrapped | O | wall\decomp\decomp-0x14085a650.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::remove_all` | 0x14085a730 | 120 | InnerWalls::remove_all（round3 round2 复查修正：原“max_id”误标；PDB 唯一符号=remove_all，行为=清空 map 头 4 qword+128 位占用位图写 0） | O | wall\decomp\decomp-0x14085a730.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::remove` | 0x14085aa60 | 126 | InnerWalls::remove | O | wall\decomp\decomp-0x14085aa60.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::update_arch_height` | 0x14085ad20 | 148 | InnerWalls::update_arch_height（→ _try_update_state<update_arch_height> 0x140aaa710） | O | wall\decomp\decomp-0x14085ad20.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::update_arch_type` | 0x14085ad40 | 154 | InnerWalls::update_arch_type（→ 0x140aa9eb0 写 WallState+0x64） | O | wall\decomp\decomp-0x14085ad40.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_height` | 0x14085ad60 | 202 | InnerWalls::update_wall_state_height（→ 0x140aaaca0 写 WallState+0x54） | O | wall\decomp\decomp-0x14085ad60.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::_try_update_state<country_core::resources::walls::inner_walls::impl$1::_deprecated_update_skirt::closure_env$0>` | 0x140aaa350 | 208 | _try_update_state<update_plaster_peel> 实例化 | I | wall\decomp\decomp-0x140aaa350.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::_try_update_state<country_core::resources::walls::inner_walls::impl$1::mark_as_non_enclosed::closure_env$0>` | 0x140aaa8f0 | 208 | _try_update_state<mark_as_non_enclosed>（写 enclosure tag 2 = Freehand） | O | wall\decomp\decomp-0x140aaa8f0.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::_try_update_state<country_core::resources::walls::inner_walls::impl$1::update_arch_height::closure_env$0>` | 0x140aaa710 | 208 | _try_update_state<update_arch_height>（写偏移待解，候选 arch_height f32） | I | wall\decomp\decomp-0x140aaa710.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::_try_update_state<country_core::resources::walls::inner_walls::impl$1::update_arch_type::closure_env$0>` | 0x140aa9eb0 | 208 | _try_update_state<update_arch_type>（写 WallState+0x64 u8） | O | wall\decomp\decomp-0x140aa9eb0.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::_try_update_state<country_core::resources::walls::inner_walls::impl$1::update_elevation::closure_env$0>` | 0x140aaaac0 | 208 | 更新 | I | wall\decomp\decomp-0x140aaaac0.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::_try_update_state<country_core::resources::walls::inner_walls::impl$1::update_freehand_ty::closure_env$0>` | 0x140aa99e0 | 208 | _try_update_state<update_freehand_ty>（写 WallState+0x60 u8） | O | wall\decomp\decomp-0x140aa99e0.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::_try_update_state<country_core::resources::walls::inner_walls::impl$1::update_plaster_peel::closure_env$0>` | 0x140aaa530 | 208 | _try_update_state<update_style> 实例化 | I | wall\decomp\decomp-0x140aaa530.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::_try_update_state<country_core::resources::walls::inner_walls::impl$1::update_style::closure_env$0>` | 0x140aaa350 | 208 | _try_update_state<update_plaster_peel> 实例化 | I | wall\decomp\decomp-0x140aaa350.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::_try_update_state<country_core::resources::walls::inner_walls::impl$1::update_supports::closure_env$0>` | 0x140aa9800 | 208 | _try_update_state<update_supports>（写 WallState+0x67 removed_supports u8） | O | wall\decomp\decomp-0x140aa9800.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::_try_update_state<country_core::resources::walls::inner_walls::impl$1::update_wall_state::closure_env$0>` | 0x140aa9bc0 | 208 | _try_update_state 实例化（bulldoze 增长路径） | I | wall\decomp\decomp-0x140aa9bc0.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::_try_update_state<country_core::resources::walls::inner_walls::impl$1::update_wall_state_curve::closure_env$0>` | 0x140aaa090 | 208 | _try_update_state<update_wall_state_curve>（drop 旧双 Vec 换入新曲线） | O | wall\decomp\decomp-0x140aaa090.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::_try_update_state<country_core::resources::walls::inner_walls::impl$1::update_wall_state_enclosure::closure_env$0>` | 0x140aa9610 | 208 | 更新 | I | wall\decomp\decomp-0x140aa9610.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::_try_update_state<country_core::resources::walls::inner_walls::impl$1::update_wall_state_height::closure_env$0>` | 0x140aaaca0 | 208 | _try_update_state<update_wall_state_height>（写 WallState+0x54 f32） | O | wall\decomp\decomp-0x140aaaca0.txt |
| `country_core::resources::walls::inner_walls::InnerWalls::load_state` | 0x14085ad80 | 218 | IO/解析 | I | wall\decomp\decomp-0x14085ad80.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::walls::inner_walls::InnerWalls::wall_ids`（行 43）— 待推断
- `country_core::resources::walls::inner_walls::InnerWalls::new_wall`（行 69）— 构造
- `country_core::resources::walls::inner_walls::InnerWalls::max_id`（行 89）— 待推断
- `country_core::resources::walls::inner_walls::InnerWalls::set_max_id`（行 99）— 设置/写入
- `country_core::resources::walls::inner_walls::InnerWalls::all`（行 103）— 待推断
- `country_core::resources::walls::inner_walls::InnerWalls::contains`（行 111）— 谓词/校验
- `country_core::resources::walls::inner_walls::InnerWalls::update_wall_state`（行 136）— 系统:周期性处理
- `country_core::resources::walls::inner_walls::InnerWalls::update_elevation`（行 142）— 系统:周期性处理
- `country_core::resources::walls::inner_walls::InnerWalls::update_style`（行 160）— 系统:周期性处理
- `country_core::resources::walls::inner_walls::InnerWalls::update_supports`（行 166）— 系统:周期性处理
- `country_core::resources::walls::inner_walls::InnerWalls::update_plaster_peel`（行 172）— 系统:周期性处理
- `country_core::resources::walls::inner_walls::InnerWalls::update_freehand_ty`（行 178）— 系统:周期性处理
- `country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_curve`（行 184）— 系统:周期性处理
- `country_core::resources::walls::inner_walls::InnerWalls::mark_as_non_enclosed`（行 190）— 设置/写入
- `country_core::resources::walls::inner_walls::InnerWalls::update_wall_state_enclosure`（行 196）— 系统:周期性处理
- `country_core::resources::walls::inner_walls::InnerWalls::_deprecated_update_skirt`（行 241）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 16 项
## 实现方式（算法级）

### InnerWalls 结构（O，DIGEST §1.12 深化）

- hashbrown map（ctrl@+0、mask@+8、len@+0x18 等）＋ **+0xc0 u64 脏代计数器**（wall_ids/new_wall_w_params 每次 +1）+ **+0x30 工厂子结构**（FUN_1402ca2f0(self+0x30, gen) 尾调）+ +0x20 hasher（FUN_1405b0d70 首参）。
- 条目：{WallId u64 key, WallState 0x70B value}（步长 0x78，DIGEST §1.2）。
- 全局 `cRam0000000143634d90`（静态 bool 调试开关）× 5 影响描述符 tag 构建。
- InnerWalls::new 0x140859f90：经 FUN_140d4d740() 惰性 once（首次置 +0/+8/+0x10，后续 +8+=1 代计数）→ 从静态常量 _UNK_142a94940..58 拷贝默认 map 配置 + Arc 克隆。

### wall_ids 0x14085a200（O）

```
签: (self, out, sorted u8)
if (sorted==0): 过滤器描述符 = 静态常量 _UNK_142a948f0..fc + 0；else: _UNK_142a94900..0c + 0x100
cStack_8f = (cRam_143634d90 == 0) * 5                                  // 调试开关×5
uStack_c0 = 2（描述符 tag）
+0xc0 代计数 += 1 → lVar1
FUN_141529bb0(&out, self, lVar1, &描述符)   // hashbrown 遍历收集 WallId → out Vec（8B 元素）
清理 out 两 Vec（若非空，元素 8B/4B 释放）
FUN_1402ca2f0(self+0x30, lVar1)             // 尾调工厂
return lVar1                                // 新代计数
```

### new_wall_w_params 0x14085a370（O）

```
签: (self, WallParams 0x30B, tag u32, u8×2)
把 WallParams 复制到栈描述符（uStack_f8..c8 = 8 qword），置 uStack_c0=2、uStack_a0=_UNK_142a94910、
  uStack_94=0、uStack_93=param_5、uStack_92=0、uStack_98=0、uStack_a4=param_3(高度/类型)、uStack_90=param_4
+0xc0 代计数 += 1 → FUN_141529bb0(&out, self, gen, &描述符) → 尾调 FUN_1402ca2f0(self+0x30, gen)
return gen
```

### new_wall_with_id 0x14085a4a0（O 结构，DecompilationFailed）

FUN_141529bb0 收集 + 尾调 FUN_1402ca2f0(self+0x30, id) 工厂（id 显式指定；Ghidra 对该函数"Unable to find unique hash for varnode"失败，依角色映射 + 调用点）。

### 访问器（O）

- `get` 0x14085a560：len@+0x18==0 → null；哈希 + SIMD 探针（0x78 步长，WallId 键比较）→ 命中返回 &value。
- `get_unwrapped` 0x14085a650：同 get 但未命中 panic（unwrap 语义）。
- `remove` 0x14085aa60：hashbrown erase（含 +0xc0 代计数触碰，merge_walls 内联调用 ×2）。
- `max_id` 0x14085a730：仅尾调 FUN_1402ca2f0（+0x30 工厂查最大 id）。
- `reset_max_id` 0x14085a550：重置 max_id 状态（I，未逐行）。
- update 转发链（O）：`update_arch_height` 0x14085ad20 → _try_update_state<update_arch_height> 0x140aaa710（写 arch_height f32 偏移待解）；`update_arch_type` 0x14085ad40 → 0x140aa9eb0（写 WallState+0x64 u8）；`update_wall_state_height` 0x14085ad60 → 0x140aaaca0（写 +0x54 f32）。

### _try_update_state 闭包族（O，写 WallState 字段）

| 闭包 | 地址 | 写 |
|---|---|---|
| update_supports | 0x140aa9800 | +0x67 u8（removed_supports） |
| update_freehand_ty | 0x140aa99e0 | +0x60 u8 |
| update_arch_type | 0x140aa9eb0 | +0x64 u8 |
| update_wall_state_curve | 0x140aaa090 | 双 Vec 换入（drop 旧 {cap@+0,ptr@+8,len@+0x10} × 8B 元素 + 第二 Vec@+0x18） |
| mark_as_non_enclosed | 0x140aaa8f0 | enclosure tag 写 2（Freehand） |
| update_wall_state_height | 0x140aaaca0 | +0x54 f32 |
| update_arch_height | 0x140aaa710 | arch_height f32（偏移待解，I） |
| update_style / update_plaster_peel | 0x140aaa530 / 0x140aaa350 | 实例化（I） |
| bulldoze 增长 | 0x140aa9bc0 | 实例化（I） |

均为：查 InnerWalls map（0x78 步长）→ 命中改 WallState 字段 → 标脏（+0xc0 代计数）；更新历史/事件。

### 新常量表

| 常量 | 地址 | 语义 |
|---|---|---|
| cRam0000000143634d90 | 全局静态 bool | 调试开关（×5 影响描述符 tag） |
| 过滤器常量对 | 0x142a948f0..fc / 0x142a94900..0c | wall_ids sorted 分支 |
| 默认 map 配置 | 0x142a94940..58 | InnerWalls::new |

### 跨模块调用

FUN_141529bb0（hashbrown 遍历收集）、FUN_1402ca2f0（+0x30 工厂）、FUN_1405b0d70（哈希）、FUN_140d4d740（惰性 once）、FUN_1400d9b20（Arc 克隆）。InnerWalls 消费于 draw_wall 0x140a7cba0、finish_wall 0x140a800a0、merge_walls 0x140999920、wall_roof_hashmap 0x140a94cd0、publish_public_wall_state 0x140a8b530。

### 遗留 U

- +0x30 工厂子结构（FUN_1402ca2f0）内部布局与 max_id/reset_max_id 完整语义。
- new_wall_w_params 的 param_3（u32 tag）与 u8×2 精确字段名。
- update_arch_height（0x140aaa710）写的 arch_height f32 偏移。
