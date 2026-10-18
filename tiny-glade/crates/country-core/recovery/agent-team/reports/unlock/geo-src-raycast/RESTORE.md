# RESTORE — unlock/geo-src-raycast（unlock-geo-src-src-raycast，2026-10-17）

> 角色：unlock-geo-src-raycast。目标：解依赖线 #7 —— 在 country-core 输入/几何侧
> 还原 `geo_src_to_edit_subject` / `geo_src_to_edit_subject_w_deco_clutter`，并核对
> `RaycastWorld::legacy_raycast` 的命中码语义，解锁 onboarding `hint_focused_edit_hint`
> （0x14227dd60）/ `hint_clutter_focused_edit_hint`（0x141c3db80）与 tree-tool
> `ui_place_tree`（0x142269170）。
>
> **三态：完成。实现 3（2 新函数 + 1 修正）/ 合同 0 / 登记 4（I/OS，不阻塞）。**
> 证据全部本轮 fresh；未采信此前报告结论。

## 0. 交付位置

| 项 | 位置 |
|---|---|
| 新增模块 | `crates/country-core/src/systems/ui_edit_focus.rs` |
| 模块注册 | `crates/country-core/src/lib.rs`（`systems` 内 `pub mod ui_edit_focus;`） |
| legacy_raycast 修正 | `crates/country-core/src/systems/collision/world.rs:1346` |
| 证据 | 本目录 `evidence/`（完整 MCP 结果 JSON，可重放） |

身份：B patched EXE `tiny-glade.exe`，sha256 `cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323`
（`get_metadata`，base 0x140000000，131517 函数）。只读后端：ghidra `--worker shared`、
radare2 `--worker unlock-geo-src-raycast`。

## 1. 证据方法与一处旧伪影纠正

PDB 给出的函数长度：`geo_src_to_edit_subject` = 0xA2（0x140977f50..0x140977ff1），
`_w_deco_clutter` = 0x1CB（0x140978000..0x1409781ca），`legacy_raycast` = 0xE3。

Ghidra 反编译这两个函数时未跟随跳表，函数体越界吞并后续函数，产生 120KB/172KB
“伪反编译”（警告 `Control flow encountered bad instruction data`）。本轮以
**radare2 原始指令 + 跳表字节**重建真实控制流；`check_evidence`：
`evidence/r2.result.json`（0x140977f50 / 0x140978000 全指令）、
`evidence/r2b.result.json`（deco 分支与 4 个调用点）、
`evidence/r2c.result.json`（跳表第 17 项）、`evidence/r2d.result.json`
（legacy_raycast + panic 串 + Rapier::raycast 入口）、
`evidence/ghidra-rapier.result.json`（Rapier::raycast 逐语句）、
`evidence/ghidra-helpers.result.json`（DecoratorStorage::get / PublicWalls::get/roof_entity）。

两个调用点（`hint_focused_edit_hint` 0x14227e052/0x14227e086、`ui_place_tree`
0x1422692b5/0x1422692e4、`hint_roof_shape` 0x1422850ee/0x14228512e、
`hint_clutter_focused_edit_hint` 0x141c3dcb1）都先把 `RaycastWorldResult.handle`
的 24 字节原样拷到栈，再把指针传给 `geo_src*`。

## 2. 输入 GeoSource（24 B）与跳表

指令（两函数同形）：

```
movzx r8d, byte ptr [rdx]        ; tag = src[0]
xor eax, eax
lea r9, [rip + table]            ; plain 0x142ab747c / _w_deco 0x142ab74c0
movsxd r8, dword ptr [r9+r8*4]   ; int32 相对偏移（高 2B 恒 0xfdec）
add r8, r9
jmp r8
```

跳表 **17 项**（tag 0..=16），逐项解码见 `evidence/jump-tables-decoded.txt`：

| tag | plain 目标 | `_w_deco` 目标 |
|---|---|---|
| 0,1,8,9 | 0x140977f6b | 0x14097801e |
| 2 | 0x140977f87 | 0x14097811c |
| 3,12,15,16 | 0x140977f9e（unreachable） | 0x140978177（unreachable） |
| 4,5 | 0x140977f94（None） | 0x140978030（deco 分支） |
| 6,7,10,11 | 0x140977f94（None） | 0x14097812e（None） |
| 13,14 | 0x140977f7a | 0x14097810f |

目标处写面（输出 `EditSubject`，16 B：判别 @+0 u32、载荷 union）：

- 0x140977f6b：`mov rax,[rdx+8]; mov [rcx+8],rax; eax=1` → `Wall(u64 @+8)`；
- 0x140977f87：同拷贝，`eax=2` → `Roof(u64 @+8)`；
- 0x140977f7a：`mov eax,[rdx+4]; mov [rcx+4],eax; eax=4` → `StairPoint{decorator_id:u32 @+4}`；
- 0x140977f94：`mov [rcx],eax`（eax 保持入口 0）→ `None`；
- 0x140977f9e / 0x140978177：构 `unreachable` panic（消息在 0x142ab6a10）。

`_w_deco` 的 Wall/Roof/StairPoint 写面与 plain 完全同形，仅 tag 4/5 改道。

## 3. `_w_deco_clutter` 的 tag 4/5 decorator 分支（0x140978030..0x140978175）

签名第三参数 `r8 = &DecoratorStorage`（调用点 `hint_clutter_focused_edit_hint`
0x141c3dca9 传 `[resource]`）。逐语句：

1. `test r8,r8; je → None`（空指针门）；
2. `mov edi,[rdx+4]`（decorator_id）、`mov rax,[rdx+8]`、`r9=[rdx+0x10]`；
   `call 0x140a1c2c0 = DecoratorStorage::get(self, {disc=[+8], payload=[+0x10]})`
   （Ghidra 命名为 `DecoratorStorage::get`，已核实）；
3. `test rax,rax; je → None`；
4. `cmp qword [rax+0x48],0; je → None` = 内嵌 decorators map items == 0；
5. hashbrown 查表（条目 stride **0x50**，键 = decorator_id u32 @+0；命中项
   `cmp dword [entry], edi`），未命中 → None；
6. 命中项 `movzx eax, byte [entry+0x4E]` = `DecoratorInfo.ty`（记录 +0x46）；
   `add al, 0xC8`（回绕）后：
   - `al==7`（ty=0x3F=Stairs）→ `esi=4` → `StairPoint{decorator_id}`（0x14097816d）；
   - `al==6`（ty=0x3E）或 `al>=8`（ty ≤0x37 或 ≥0x40）→ `esi=3` →
     `DecoClutter{decorator_id}`（0x140978163）；
   - 其余（ty 0x38..=0x3D：CottageWindow/GothicWindow/ArrowSlit/Chimney/Lantern/Flag）
     → `esi=0` → `None`。

## 4. legacy_raycast 命中码（0x1408cc680，本轮 fresh 复核并修正）

```
push rsi; sub rsp,0xc0; mov rsi,rcx          ; rcx = sret out
mov dword [rsp+0x30], 0xfb                    ; mask = 0xFB（17 槽排除 bit2）
mov dword [rsp+0x28], 0x7f7fffff              ; max_toi = f32::MAX
lea rcx,[rsp+0x80]; xor r9d,r9d
call 0x1408cdc60                              ; Rapier::raycast
cmp dword [rsp+0xb4], 2                       ; Rapier 判别 @+0x34 == 2 → miss
jne hit
mov dword [rsi+0x30], 2                       ; miss：只写 +0x30 = u32 2
hit: 0x40 B 全量拷贝（xmmword [rsi] / … / xmmword [rsi+0x30]）
```

命中码语义：
- `+0x34`（`RaycastWorldResult.discriminant`）：0 = 命中，2 = 未命中（Rapier::raycast 写）；
- `+0x30`（legacy 覆写槽）：miss 时写 **u32 2 的位型**，调用方一律
  `cmp dword [result+0x30], 2` 判 miss（hint 0x14227e057、ui_place_tree 0x1422692ba、
  hint_roof_shape 0x1422850f3）。
- terrain 命中由 `terrain_result` 写 `+0x00` 首字节 `0xB`，判别仍为 0（命中）。

**修正**（world.rs `legacy_raycast`）：
1. `Rapier::raycast` 的 mask 由 `u32::MAX` 改为 **0xFB**（原版固定掩码）；
2. miss 写 `+0x30` 由 `2.0f32`（位型 `0x40000000`，调用方 `==2` 判假）改为
   `f32::from_bits(RAYCAST_WORLD_RESULT_MISS)`（位型 `0x00000002`，逐位等价）。

## 5. 实现与验证

`crates/country-core/src/systems/ui_edit_focus.rs`：

- `pub struct GeoSource([u8;24])`（`from_hit_handle` / `tag` / `payload_u32` /
  `payload_u64` / `payload_u64_hi` / `storage_addr`）；
- `pub fn geo_src_to_edit_subject(src) -> EditSubject` —— 跳表逐项；
- `pub fn geo_src_to_edit_subject_w_deco_clutter(src, Option<&DecoratorStorage>)`
  —— 复用 `resources::walls::decorator_storage::{DecoratorStorage, Decorators,
  DecoratorStorageAddr}` 与 `Decorators::get`，tag 4/5 走 §3；
- `EditSubject` 直接复用 `systems::mode_manager::EditSubject`（判别 0..=5 与原版写面一致：
  `None=0 / Wall(u64)=1 / Roof(u64)=2 / DecoClutter{decorator_id}=3 /
  StairPoint{decorator_id}=4`；Terrain=5 本两函数不产出）。
- 5 个单测覆盖 17 tag 全表、deco 分支类型映射（含 0x3E）、空指针/空桶/缺 id/错桶门。

命令结果（workspace 根 `/Users/yuluo/Desktop/game/games_test/tiny-glade`）：

| 命令 | 结果 |
|---|---|
| `cargo check -p country-core` | **0 error**（既有 14 warning，与本模块无关） |
| `cargo check -p country-core --features native-physics` | **0 error**（10 warning，既有） |
| `cargo test -p country-core --lib ui_edit_focus` | **5 passed / 0 failed** |

> 并发说明：22:30 左右一次 `cargo check` 曾因他组在途文件
> （`resources/history/color_edit.rs`、`systems/collision/visualize_colliders.rs`、
> `systems/save/{snapshot,mod}.rs`）报错 4 条；随后重跑即 0 error。与本模块无关，
> 未代改他模块。最终两次 check 均 `Finished`（exit 0）。

## 6. 登记（I/OS，不阻塞）

1. **OS**：原版 tag 只有 0..=16，越界字节是跳表越界读（UB）；Rust 侧
   `unreachable!`。
2. **I**：tag 3/12/15/16 的 panic 含一个 `Display` 实参；串在 0x142ab6a10，
   静态文本为 `internal error: entered unreachable code: ui_edit_focus shouldn't
   include  into their raycasts`（`include` 与 `into` 间两个空格 = 空 Display）。
   本实现复刻静态文本，Display 变体名未恢复。
3. **I**：`test r8,r8` 空指针门用 `Option<&DecoratorStorage>` 表达。
4. **I**：legacy_raycast miss 时原版只写 sret 的 `+0x30`，其余 0x3C 字节保持调用方
   栈内容（未初始化）；Rust 返回完整 `RaycastWorldResult`（miss 为零值骨架）。
   调用方在 `+0x30==2` 处短路，不读其余字段，无可观察分歧。

## 7. 解锁状态（核心侧已完成，消费侧接线不在本任务范围）

- `hint_focused_edit_hint` 可直接调用 `geo_src_to_edit_subject`：subject 1 = Wall、
  2 = Roof、3/4/5 无 hint、0 = 无 hint，与其跳表 0x1430386a0 逐项吻合。
- `hint_clutter_focused_edit_hint` 调 `_w_deco_clutter`：返回 3 时建
  `DisplayHintRequest(0x100, id=3)`（原 0x141c3dcb6 `cmp dword [out],3`）。
- `ui_place_tree`：`cmp dword [out],0`（0x1422692e9）——`EditSubject::None` 才继续
  放置路径，其余早退；现可直接消费。
- `hint_roof_shape`：`geo_src==2` 门（0x142285133）可直接消费。
