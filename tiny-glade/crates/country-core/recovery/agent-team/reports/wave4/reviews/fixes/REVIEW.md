# REVIEW — wave4 FIX 落实复核（geometry G1-G6 / visual F1-F6 / misc 4 项）

> reviewer：`reviewer-wave4-fixes`（2026-10-17）
> 范围：本轮 wave4 三项 REVIEW 的 FIX 清单是否已按反汇编落实。
> - geometry：`reports/wave4/reviews/geometry/REVIEW.md` → G1..G6
> - visual：`reports/wave4/reviews/visual/REVIEW.md` → F1..F6
> - misc：`reports/wave4/reviews/misc/REVIEW.md` → wren h0 / move Sheep / avoid 4+4 / butterfly flag
>
> 对应 RESTORE FIX 记录：`reports/remaining/wall-constructor/RESTORE.md`、
> `reports/consume/types/RESTORE.md`、`reports/consume/nani-roof-toolbar/RESTORE.md`、
> `reports/consume/nani-decorator/RESTORE.md`、`reports/remaining/misc/RESTORE.md`。
>
> 方法：**逐项 fresh 反汇编/反编译**（Ghidra `disassemble_function` +
> `get_functions(decompiled_code)` + `read_memory`；wren h0 跳表目标用 radare2 `pd`），
> 不复用 restorer/reviewer 结论作为证据，再与 Rust 逐语句对照。**未改 src。**
> 证据落盘：`work/reviewer-wave4-fixes/evidence/`（`fresh-main.json`、`fresh-disasm*.json`、
> `dis-0x*.json`、`fresh-r2-wren.json`、`abi-0x*.c`），jobs 可重放。
> 身份：B patched EXE（`tiny-glade.exe`，base 0x140000000；radare2 用同版本 `tiny-glade.rne`）。

## 0. 三态回执

| 组 | fix_verified | 仍有 FIX | 备注 |
|---|---|---|---|
| geometry G1-G6 | **6** | **0** | 3 处非阻塞 seam/文档残留（§4） |
| visual F1-F6 | **6** | **0** | F6 core typed-gap 仍登记（非本 FIX 范围） |
| misc 4 项 | **4** | **0** | 资源身份 U 维持登记 |
| **合计** | **16** | **0** | |

## 1. Geometry（G1-G6）

### G1 `circle_categorize` MAXSS/MINSS + UCOMISS/SETC — **fix_verified**

fresh `0x14124af60`（197 条）：`MOVSS XMM9,[RCX+8]=radius`；
`XORPS XMM1,XMM1; MAXSS XMM1,XMM0(t)` @0x14124affc；
`MOVSS XMM11,[0x142925904]=1.0; MINSS XMM0(1.0),XMM1` @0x14124b00d。
末比较 `DIVSS XMM10(in_wall),XMM13(inside)` @0x14124b24f +
`UCOMISS XMM10,XMM12(threshold)` + `SETC AL` @0x14124b254/258
（unordered/CF=1 → true）。
Rust `minss(1.0, maxss(0.0, t_raw))` 与 `ratio < threshold || nan`：MAXSS/MINSS
无条件返回第二操作数（NaN→t），`SETC` 对 unordered 置 1，逐条一致。
另核实 `ceil/2^63/2^64` 饱和转换序列（0x14124b05a-b95）：NaN→0、>2^64→u64::MAX，
与 Rust `as usize` 相同。**判等。**

### G2 `replay_tree` Move-undo 降级 — **fix_verified**

fresh decomp `0x141bc34f0`：Move 判别 `*(int*)(edit+0x10)==1`；
`undo==0` 读 `edit+0x20`（8B）与 `+0x28` 为**新位置**；`undo!=0` 读
`edit+0x14`（8B）与 `+0x1c` 为**旧位置**，两路均写 0x20B MoveTreeCmd。
Rust `Move` 的 `else` 分支不发射事件，并显式标注「未实现（wave4 FIX-G2）」；
RESTORE 已将 verdict 由判等降为 **重建**，并写明解锁条件（TreeEditData 补旧位）。
按 REVIEW 允许的处置（降级 + 整条路径登记）落实。**fix_verified。**

### G3 `maintain_dormer_tiles` 单槽 store — **fix_verified**

fresh `0x1407ce920`：`MOV EBP,[RCX+0x1c]`、`MOV R13,[RCX+0x10]` 各一次；
每瓦 `INC qword [RAX]` → `MOV dword [R13],EBP` @0x1407cea12，R13 循环内不变、
无容量检查/无 len 更新。Rust `id_slot: *mut u32` + `*id_slot = value`，不再 push。
null 判断是宿主未接线时的安全 seam（原生假定指针非空）。**fix_verified。**

### G4 `process_tree_cmds` Insert selector — **fix_verified**

fresh `0x142267430`：`0x1412df1 LEA RDI,[R12+0x38]`；名称 24B 比较命中后
`0x142268146 MOV RCX,[RDI]`（qword @元素+0x38）→ `CALL 0x14226a170`；
callee `TEST SIL,0x1`（0x14226a1aa/1cf）只取低位。
`[R12+0x38]` 同时用于 `TreeStorage::insert`（0x142267f34），即共享 `InsertTreeCmd.id`。
Rust `random_tree_prefab((cmd.id & 1) as u8, …)` 与机器一致；T4 的「不可表达」理由已撤销
（`is_corrupted` 资源面仍登记）。**fix_verified。**

### G5 `ui_move_tree` `!DEMO` 门 — **fix_verified（附 seam 备注）**

fresh `0x14226b1a0`：tag=`(mode∈[6,0x14])?mode-5:0`，`tag==0xb || (tag==0&&mode==0)`
后 `0x14226b26c MOV RAX,[0x14362c098]; CMP byte[RAX],0; JZ continue`。
Rust 补 `if DEV_OVERRIDE_IS_DEMO_MODE { return; }`，位于 mode 门之后、逻辑之前，
与机器门序一致。
备注：`DEV_OVERRIDE_IS_DEMO_MODE` 为编译期 `false`（与 `ui_place_tree` 同一既定
host-override seam，注释已写明宿主覆盖），不是对 `*0x14362c098` 的运行时读取；
属已登记的建模 seam，不构成逻辑分歧。**fix_verified。**

### G6 `ui_move_tree` state 钳位 / 删除 `cache.dragging` — **fix_verified**

fresh `0x14226b8c6 ADD AL,0xfe; MOVZX EAX,AL; CMP AL,3; MOV ECX,3; CMOVNC EAX,ECX`
→ `state-2 (mod 256)`，`>=3` 一律钳 3（含 state 0/1 回绕）；跳表
`0x143035538` 四 i32 相对值实读解码为 `{0x14226b493(无动作), 0x14226b430(case1),
0x14226b8e8(case2), 0x14226b9c0(case3)}`；case3 块实读
`target=cursor+Local offset → sample → is_pos_inside → MoveTreeCmd+History`。
Rust `wrapping_sub(2)`、`>2 → 3`、无 `dragging` 门、case3 不清 Local，逐条一致。
**fix_verified。**

## 2. Visual（F1-F6）

### F1 容器缓冲 dealloc — **fix_verified**

fresh `0x141e4dc40`：容器分支 `0x141e4e788 MOV RDX,[RBP+0x200]; TEST RDX,RDX;
JZ skip; 0x141e4e794 SHL RDX,0x6; MOV R8D,0x8; MOV RCX,[RBP+0x1f8];
CALL 0x140613c20`（`__rust_dealloc`）。字段来源实读
`0x141e4dcb5 MOV RAX,[item+0x18]`、`MOVUPS XMM0,[item+0x8]`，随后
`[0x190]=item+0x8, [0x198]=item+0x10, [0x1a0]=item+0x18`：
**Vec 序为 cap@+8 / ptr@+0x10 / len@+0x18**。
Rust 结构已按此排序，并在容器末尾 `dealloc(ptr=children_ptr, size=cap*0x40, align=8)`；
尾项 drop 计数 `(end-stop)>>6` 与 Rust `it.add(1)..base.add(len)` 一致。**fix_verified。**

### F2 `TaffyNode` 20B / align 4 — **fix_verified**

fresh descriptor 立即数 `0x142383a39 MOV qword [RBP+0x1f8],0x4`（align）、
`0x142383a44 MOV qword [RBP+0x200],0x14`（size）；`read_memory 0x142383a20` 字节
`…04 00 00 00…14 00 00 00` 复核。Rust `TaffyNode{node: taffy::Node, _tail:[u8;12]}`
`repr(C)`：`taffy::Node = slotmap::DefaultKey`（`KeyData{u32,NonZeroU32}`，
8B/align 4）⇒ size 0x14 / align 4，含 `assert!(size_of == 0x14)`。
原 reviewer「8B」建议由更强 descriptor 证据改判 20B，成立（不是放水）。
**fix_verified。**

### F3 `ShowOnlyInFpvAndFree` ZST — **fix_verified**

fresh `0x142386ac9 MOV qword [RBP+0x1f8],0x1`（align 1）、
`0x142386ad4 MOV qword [RBP+0x200],0x0`（size 0）；`read_memory 0x142386ab0`
字节复核。Rust 已改为 unit struct（size 0），原 `{bool,_gap:u64}`=16B 作废。
**fix_verified。**

### F4 decorator build 调用点 ABI — **fix_verified**

caller `0x1407e29e7 CALL 0x1412ab270` 前：`RCX=&builder`、`EDX=id`、
`R8=mesh_lib`、`R9=shader_lib`，栈实参 `[RSP+0x20]=arg5`、`[RSP+0x28]=arg6`、
`[RSP+0x30]=arg7`（0x1407e29bb-b6）。
callee prologue `8×PUSH + SUB RSP,0x108` ⇒ callee RSP = caller RSP-0x150，
故 arg5/6/7 = `[RSP+0x170/0x178/0x180]`。
fresh 读取点：`0x1412ab3c0 MOV RBX,[RSP+0x178]`（arg6），RBX 作为
`Commands::spawn` 的 self（`0x1412abb9d LEA RCX,[ret]; MOV RDX,RBX; … CALL 0x14126e1e0`）；
`0x1412ac43e/5e4 MOV RCX,[RSP+0x180]`（arg7），在 `0x1412ac662 MOV EDX,ESI; MOV R8,RBX;
CALL 0x140811c30` 作为 `&mut 状态表`；全函数**无** `[RSP+0x170]` 读取（arg5 未用）。
默认分支先两次 handle 查找（0x14092d4f0 / 0x140937aa0）再 `spawn`，与 Rust 顺序一致。
Rust build 已去掉 entity 入参、内部 `spawn_empty` 建实体并返回 Entity。**fix_verified。**

### F5 `impl$5` 注释/实现 — **fix_verified**

fresh `0x140811c30` 尾部：`MOV RAX,[R14-0x10]`（Vec ptr）、
`MOV [RAX+RDI*0x8],R13`（实体，**8B 步长**）、`INC RDI; MOV [R14-8],RDI`（len）。
即 `HashMap<u32, Vec<Entity>>::entry(id).push(entity)`，与 autoclutter 0x60 stride 无关。
`decorator_visual_state.rs` 注释已改为瞬态实体表，新增
`DecoratorTransientVisualState::add_decorator_visual_entity`，sink 委托。
**fix_verified。**

### F6 `Ui2dImage` 32B + 计数 — **fix_verified**

fresh `0x141e33ed0`：`0x141e34167 MOV qword [RBP+0x1f8],0x8`（align）、
`0x141e34172 MOV qword [RBP+0x200],0x20`（size）⇒ **32B / align 8**。
RESTORE §6 计数口径已统一；`Ui2dImage` 32B（24B String 类写面 + 8B tail）已记录。
core `Ui2dImage` 仍为 `{_gap:u64}` 8B，RESTORE 明确标 typed-gap 并写解锁条件，
**未虚报为完成**。FIX-6 为报告精度/计数修正，已落实。**fix_verified。**

## 3. Misc（4 项）

### wren h0 模式门/阈值 — **fix_verified**

fresh radare2 `0x14231ebf8`（h0 handler）：
`MOVSS XMM0,[R12+0x34]; SUBSS XMM0,dt; MOVSS [R12+0x34],XMM0;
UCOMISS XMM8(0),XMM0; JAE 0x14231e937`（0>=timer → h1）；
`UCOMISS XMM7(threshold),XMM8(0); JBE 0x14231e969`（**threshold<=0 → 跳过查询**）；
`CALL 0x140a12650` → `sqrt((cursor.x-pos.x)^2+(cursor.z-pos.z)^2)` →
`UCOMISS XMM7,XMM0; JA h1`（threshold>dist）。
阈值来源 fresh `0x14231e7d9-803` + `0x14231f5cd-e5`：
`tag=(mode∈[6,20])?mode-5:0`；`tag∈{2,3,4,5}`（mode 7..10）→1.0；
`tag==1`（mode 6）且 `byte[res+4]-1<2`（WallShape Circle/Rectangle）→3.0，否则 0；
`tag==0xa`（mode 15 Water）→1.0；其余→0。
Rust `wren_proximity_threshold` 的 AppMode tag 表（Wall=6/Path=7/Cobble=8/Eraser=9/
Terrain=10/Water=15，WallShape 0/1/2）与上式逐项一致；h0 使用
`proximity>0 && proximity>dist`。资源身份仍登记 U（AppMode 强候选）。**fix_verified。**

### move_critters 双水采样 Sheep — **fix_verified**

fresh decomp 查询 tuple 顺序实读：`…Option<Duck>, Option<Sheep>, Option<BirdSpots>`；
frame `[RBP+0x23c]` 由 `[RDI+0x138]`（第 6 列 = Sheep）装载（0x14231978a/7a3 等）。
water2 块 `0x14231a1bd TEST byte [RBP+0x23c],1; JZ skip; CMP dword [R13],4; JZ skip`，
两次 `0x140a14df0` 采样 `water(pos)` / `water(pos+disp)`，`now<=0 && pred>0 → 阻塞`。
Rust 已改 `sheep.is_some() && locomotion.0 != 4`，公式不变。**fix_verified。**

### critters_avoid_collisions 两位置 4+4 — **fix_verified**

fresh `0x142318410-5ab`：4 次 `0x140a14df0`（pos）→ `ADDSS` 求和 →
`MAXSS 0 / MINSS 1` 钳 → `UCOMISS c0,0; JBE use0.3`，否则
`d=sqrt(goal.x²+goal.y²+goal.z²)`；`px=pos.x+dir.x*d, pz=pos.z+dir.z*d`（+0x128/+0x12c）
→ 再 4 次采样 → 求和钳 → `UCOMISS c1,0; JA clear`。
Rust 两轮各 4 次、`d=(c0>0)?|goal.position|:0.3`、`c1>0` 清目标，逐语句一致
（NaN 路径：MAXSS/MINSS 得 NaN → `>0` false，与钳后比较同结果）。**fix_verified。**

### butterfly dispatch flag 基线 — **fix_verified**

fresh 常量装载 `0x141d85287 XMM12=[0x1429cd078]=3.0`、
`0x141d8529d XMM6=[0x142925904]=1.0`、`0x141d852a5 XMM15=[0x142934694]=4.0`。
派发处 `CMPLTSS mask,phase; mask?XMM15:baseline`：
0x141d85828 基线 **XMM6=1.0**（相位门内 dwell-dt>0）；
0x141d85923 基线 **XMM12=3.0**（dwell<=0 且复位为正）；
0x141d85955 按到达路径基线 3.0（相位门外/vel.y>0）或 1.0（相位门内 dwell 复位）。
Rust `flag_hi=4/3`、`flag_lo=4/1`，六路映射一致；0.0 基线已消除。**fix_verified。**

## 4. 残留（非阻塞，均非逻辑分歧）

1. **G1 源码注释陈旧**：`circle_categorize.rs:3` 仍写「837B / 105 条指令」；
   RESTORE 已更正为 **197 条**。仅注释，建议顺手改。
2. **G5 DEMO seam**：`DEV_OVERRIDE_IS_DEMO_MODE` 为编译期 `false`，非运行时
   `*0x14362c098` 读取；与本 crate `ui_place_tree` 既有 seam 一致，宿主接线时覆盖。
3. **G3 null 安全 seam**：`id_slot` 为空时跳过 store；原生无条件写（假定宿主给槽）。
4. **G2 旧位路径**仍按设计未实现（共享 TreeEditData 无旧位），verdict 已降为重建并登记。
5. **F6 core typed-gap**：`Ui2dImage` 仍 8B；32B 仅为 B 布局证据，未声称 core 已完成。
6. **F4 备注**：caller 的 arg5（`[RSP+0x450]`）在 build 内无读取，Rust 无对应入参，
   不影响语义（机器侧该参数为未用槽）。

## 5. 结论

- geometry G1-G6：**6/6 fix_verified**。
- visual F1-F6：**6/6 fix_verified**（F2/F3 由 reviewer 建议值改判为 20B/ZST，
  经 descriptor 立即数 + 内存字节复核成立）。
- misc 4 项：**4/4 fix_verified**。
- **仍有 FIX：0。** 建议各 RESTORE 的 wave4 计数/verdict 保持现状；
  仅需顺手修 G1 源码注释条数（非逻辑）。
- 本轮只读反编译对照，未运行 cargo（未改 src）。
