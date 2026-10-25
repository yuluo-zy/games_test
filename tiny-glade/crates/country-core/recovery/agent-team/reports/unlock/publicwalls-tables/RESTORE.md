# RESTORE — unlock/publicwalls-tables（PublicWalls 密集窗/屋顶桶表访问面）

> 角色：unlock-publicwalls-tables（2026-10-17，22:21-22:25）。
> 目标：在 `country-core resources/walls` 补密集窗/屋顶桶表访问面
> （stride `0xa0` / `0x48` / `+0xf8` / `+0x110`），解锁 decorator
> `window_lights` / `chimney_smoke` 收集主循环、color `set_roof_customization`、
> onboarding `hint_highlight_shape` 墙高查询。
>
> **结论：实现 0 / 合同 12 / 登记 5（三态=登记，非阻塞）。**
> 判定依据（红线"若 walls 组在途文件→合同+登记"，CHARTER §3 src 互斥）：
> `resources/walls/{mod.rs,public_walls.rs,inner_walls.rs,inner_wall_state.rs,
> public_wall_state.rs,decorator*.rs}` 与 `resources/roofs/*` 均为**未提交且在途**
> （mtime 2026-10-17 19:17–22:03，当前 22:21；`git status` 显示 M），
> 任意新增模块必须改 `resources/walls/mod.rs`（wall 组在途）→ 本任务不落 src，
> 按合同 + 登记交付。
>
> 证据：本目录 `evidence/`，**本轮 fresh**（ghidra `--worker shared` 只读）：
> - `ghidra-decomp-result.json` + `0x1407b5980_window_lights.c`、
>   `0x140818df0_chimney_smoke_anim_thickness.c`、`0x142299dd0_set_roof_customization.c`、
>   `0x141c50120_hint_highlight_shape.c`、`0x140a923a0_roof_component.c`
>   （`get_functions(fields=signature,decompiled_code,callees)`）
> - `disasm-window_lights.json`（0x1407b5980 全 2852B）、
>   `disasm-0.json`（0x142299dd0）、`disasm-1.json`（0x141c50120）
>   （`disassemble_function`，用于核对步长/因子）
> - `ghidra-mem-result.json`：`0x142a7c88c = 66 66 66 3f = 0.9f`
> - `ghidra-publicwals-result.json`：`PublicWalls::roof_entity/get/roof_component` fresh
>
> 未采信此前报告的结论；逐条以本轮反编译/反汇编重定。发现并纠正 1 处旧读法
> （hint type 集合，见 §4）。

---

## 0. 访问面总览（原生）

两个宿主指针成对出现（调用约定：`*param = A`，`param[1] = B`；原始资源/类型名在
PDB 为 `void()` 占位，**U**，下表只记偏移与语义）：

```
A（索引/模式源）                        B（密集表宿主）
+0x28  presence words ptr (u64*)        +0x08  sparse records ptr (stride 0x14)
+0x38  presence bit_len  (usize)        +0x10  sparse records len
+0xf8  ids ptr          (u32*)          +0x100 wall window records ptr (stride 0xa0)
+0x100 ids len          (usize)         +0x1a8 chunk table ptr        (stride 0x48)
+0x108 mode             (u8; 0=墙,非0=屋顶)
+0x110 primary slot     (usize)
+0x118 secondary slot   (usize; 仅 onboarding 墙高)
```

`window_lights` / `chimney` 只消费 A 的 `+0xf8/+0x100/+0x108/+0x110` 与 B 的
`+0x100/+0x1a8`；`hint_highlight_shape` 再消费 `+0x118`；`set_roof_customization`
消费 A 的 `+0x28/+0x38/+0x110` 与 B 的 `+0x08/+0x10/+0x1a8`（同一对宿主、
不同字段面）。A/B 的实际资源归属（`Roof`/`RoofShape`/`InnerWalls` 命名）**登记 U**，
不影响访问面实现。

---

## 1. 合同 C1 — 索引源 A（`+0xf8` 列表 / 模式 / 双槽）

| 偏移 | 类型 | 语义（fresh 逐语句） |
|---|---|---|
| `+0xf8` | `*const u32` | 当前帧待遍历的 **槽键列表** 基址（元素 u32，步长 4） |
| `+0x100` | `usize` | 列表长度；循环终点 = `ptr + len*4`（`LEA RAX,[R15+RAX*4]`，0x1407b5e19） |
| `+0x108` | `u8` | `0` = 墙模式（键 = 墙记录槽）；非 0 = 屋顶模式（键 = chunk 键） |
| `+0x110` | `usize` | 主槽：chunk 反向表键（见 C5/C6） |
| `+0x118` | `usize` | 次槽：仅 `hint_highlight_shape`，取第二记录数组（墙高，见 C9） |

所有消费者在函数入口一次性读出，帧内不变；四条消费函数均**不写**这些字段。
`+0x110/+0x118` 由宿主查询方生产（生产端不在本组范围，登记 U）。

---

## 2. 合同 C2/C3 — 墙记录表（stride `0xa0`）与窗槽列表（stride `0x10`）

B `+0x100` 指向墙记录数组，元素 **0xa0**（disasm `LEA RCX,[RCX+RCX*4]; SHL RCX,0x5`
= idx*0xa0，0x1407b5f27-2b）：

| 记录偏移 | 类型 | 语义 |
|---|---|---|
| `+0x50` | `*const WindowSlot` | 窗槽列表基址（列表项 **0x10** 步长） |
| `+0x58` | `usize` | 窗槽列表长度；`0` → 跳过该墙（do-while 过滤） |
| `+0x94` | `u32` | chunk 键（索引 B `+0x1a8` 表） |

窗槽项（0x10 步长，disasm `SHL RAX,0x4; [RCX+RAX+8]`，0x1407b5fb3-bb）：

| 项偏移 | 类型 | 语义 |
|---|---|---|
| `+0x08` | `u32` | **实体槽**：索引 chunk 记录数组（C5 的 `+0x10` 数组），元素步长 0x10 |
| `+0x00` | `[u8;8]` | 未被本访问面消费（U） |

墙模式下"一条记录 = 一堵墙及其窗实体槽列表"；列表长度即该墙窗数。

---

## 3. 合同 C4/C5 — chunk 表（stride `0x48`）与 chunk 项（stride `0x30`）

B `+0x1a8` 指向 chunk 数组，元素 **0x48**
（disasm `LEA RCX,[RCX+RCX*8] ... *8` = idx*0x48，0x1407b5e97-9b / 0x1407b5f49-4d）：

| chunk 偏移 | 类型 | 语义 |
|---|---|---|
| `+0x10` | `usize` | 该 chunk 的屋顶实体计数（屋顶模式循环上界与跳过门） |
| `+0x18` | `*const ChunkItem` | chunk 项数组基址（**0x30** 步长） |
| `+0x38` | `*const u64` | 反向索引数组基址（槽 → chunk 项号） |
| `+0x40` | `usize` | 反向索引长度（`primary_slot < len` 门） |

chunk 项寻址（两处 fresh 一致）：
`item = item_base + (~V) * 0x30`（`NOT V` 后 `×3×0x10`；0x1407b5ed3-de /
0x142299faf-b6）。`~V = -V-1`，注意是**反向负向**取项，勿写成 `V*0x30`。

| 项偏移 | 类型 | 语义 |
|---|---|---|
| `+0x10` | `*const T` | 记录数组指针；元素类型/步长由消费方决定（见 C6） |
| `+0x28` | `*const u32` | 颜色数组指针（仅 color 消费；元素 u32） |
| 其余 | `[u8;0x10]` / `[u8;0x8]` | 未消费（U） |

**记录数组元素步长按消费面**（同一 `+0x10` 指针，chunk 项类型由槽 V 选定）：
- 窗实体记录：**0x10**（`u64 GlobalId` @+0，后 8B 未消费）；
- onboarding 提示记录：**0x18**（见 C9）；
- onboarding 墙高记录：**0xc**（f32 @+4，见 C9）；
- 屋顶颜色组件：**0x58**（与既有 `recovered_building::building_colors::RoofColorEntry` 同构）。

---

## 4. 合同 C6/C7 — 稀疏记录表（stride `0x14`）与 presence 位集

B `+0x08`（ptr）/`+0x10`（len）指向稀疏记录数组，元素 **0x14**
（`IMUL`/`LEA idx*0x14`，0x142299f2e 等；`PUBLICWALLS` 颜色链既有证据同）：

| 记录偏移 | 类型 | 语义 |
|---|---|---|
| `+0x00` | `u32` | generation；与实体 bits 高 32 判等 |
| `+0x04` | `u32` | presence 位号；`0xffffffff` = 无 |
| `+0x08` | `u32` | 未消费（U） |
| `+0x0c` | `u32` | chunk 键 |
| `+0x10` | `u32` | 组件记录下标（乘消费面步长，color=0x58） |

presence 位集（A 侧，`set_roof_customization` fresh 0x142299f50-7d）：
`bit = rec+0x04`；门 = `bit != 0xffffffff && bit < A+0x38` 且
`words[bit>>6] >> (bit&0x3f) & 1 != 0`（`BT RCX,RDX`）。
`bit >= bit_len`/位 0 → **miss，不 panic**。

---

## 5. 合同 C8 — 窗实体遍历（window_lights / chimney 共用）

伪代码（fresh 逐语句；0x1407b5e00-0x1407b6017、0x1408191f5-0x1408192xx）：

```text
ids = A.ids[..A.len]; mode = A.mode; slot = A.primary
if mode == 0 {                       // 墙模式
  for key in ids:
    rec = wall_records[key]          // 0xa0，无边界检查（生产者不变式）
    if rec.len58 == 0 { continue }   // 0x58 门
    chunk = chunks[rec.chunk94]      // 0x48
    leaf = if slot < chunk.reverse_len && chunk.reverse[slot] != 0 {
               item_base + (~chunk.reverse[slot])*0x30 + 0x10   // 记录数组
           } else { null }           // 原生随后仍解引用 → 生产者不变式
    for i in 0..rec.len58:
       ent_slot = *(u32*)(rec.ptr50 + 8 + i*0x10)
       entity   = *(u64*)(leaf + ent_slot*0x10)     // GlobalId
       yield entity
} else {                             // 屋顶模式
  for key in ids:
    chunk = chunks[key]              // key 直接是 chunk 键
    n = chunk.count10
    if n == 0 { continue }
    leaf = 同上门（slot / reverse / ~V*0x30 / +0x10）
    for i in 0..n:
       entity = *(u64*)(leaf + i*0x10)              // 顺序，无窗口列表
       yield entity
}
```

- 产出 = 窗/烟囱实体的 `GlobalId`（u64），供 decorator 映射到灯/烟条目。
- 原生对 `leaf==null` **无空判**；实现为安全 Rust 时应按 miss 跳过并登记
  "表示差异（有效数据等价）"：有效生产数据下 leaf 恒非空。
- `chimney_smoke_anim_thickness` 两处循环（0x140818df0 首段 0x1408191f5 与次段
  约 0x14081948x）与本伪代码**逐句一致**（同一 A/B 组合）。

---

## 6. 合同 C9 — onboarding 提示 + 墙高查询（`+0x118`）

`hint_highlight_shape 0x141c50120`：遍历算法同 C8，另取第二条记录数组：

```text
leaf_hint = 槽 A.primary  → item+0x10   （记录步长 0x18）
leaf_hgt  = 槽 A.secondary(+0x118) → item+0x10 （记录步长 0xc）
```

- 提示记录（0x18）：`ty: i32 @0`、`GlobalId @8`、`byte @0x10`、`byte @0x11`；
- 墙高记录（0xc）：`height: f32 @+4`；`@0`/`@+8` 未消费（U）。
- 命中条件（fresh **disasm 定案**，0x141c50389-0x141c503c1）：
  - `0.9f < height`（`MOVSS XMM6,[0x142a7c88c]`，read_memory = `66 66 66 3f`；
    `UCOMISS`+`JBE` 跳过 ⇒ NaN 同样不命中）；
  - `ty ∈ {1,2}`（`DEC EAX; CMP EAX,1; JA` + 前置 `CMP` 复合门对 {1,2} 恒真）；
  - `rec.byte_11 != 0`；
  - 命中 → 对 `PublicWalls::roof_entity(GlobalId)` 有效者发高亮命令（tag 2）。
- **纠正旧读法**：`reports/interaction/onboarding/RESTORE.md:40` 的
  "HintType∈{1,2,6,7,8,9}" 不成立；fresh 复合门在 u32 回绕下只剩 `{1,2}`
  （该处另一行 RESTORE:147 已写 {1,2}，两处以后者/本轮为准）。
- 现有 onboarding 重建的缺口：`WallId(0)` 占位与省略 `+0x11` 门
  （`crates/systems/onboarding/src/hint_display_system.rs:158-163`）
  → 访问面落地后按上表闭合。

---

## 7. 合同 C10 — 屋顶颜色组件查找（set_roof_customization 0x142299dd0）

```text
bits = PublicWalls::roof_entity(wall_id) -> Option<Entity> 的 to_bits()
if bits >> 32 == 0 { miss }
idx = bits & 0xffff_ffff
rec = sparse_records.get(idx)                     // 0x14；越界 miss
if rec.generation != (bits >> 32) { miss }
if rec.bit04 == 0xffff_ffff || rec.bit04 >= presence.bit_len { miss }
if presence.words[bit>>6] >> (bit&0x3f) & 1 == 0 { miss }
chunk = chunks[rec.chunk0c]
V = chunk.reverse.get(A.primary)                 // primary < reverse_len；0 => miss
item = item_base + (~V)*0x30
leaf = *(u64*)(item+0x10)                        // 0x58 组件记录数组
colors = *(u64*)(item+0x28)                      // u32 颜色数组
elem = rec.element10
// 分支 A（墙式事件，外循环 stride 0x18）：
   style_old = *(u8*)(leaf + elem*0x58 + 0x3d)
   colors[elem] = color; *(u8*)(leaf+elem*0x58+0x3d) = event.+0x11
// 分支 B（屋顶外观事件，外循环 stride 0x20）：
   flag = *(u8*)(leaf+elem*0x58+0x40)
   colors[elem] = color
   *(u64*)(leaf+elem*0x58+0x40) = event.+0x10 (8B)
   *(u32*)(leaf+elem*0x58+0x48) = event.+0x18
```

- 全失败路径 **miss（无 panic/无 unwrap）**；成功路径也不做 leaf 数组边界检查
  （生产者不变式）。
- 组件记录 `0x58` 布局复用既有类型：
  `recovered_building::building_colors::RoofColorEntry`
  （`+0x3d style`、`+0x40..0x4c appearance`、`+0x50 color_id`），
  **禁止另造第二份**（见登记 R2）。

---

## 8. 合同 C11 — 失败/不变式

| 场景 | 原生行为 | 恢复实现要求 |
|---|---|---|
| color：generation/bit/chunk 任一失败 | miss，无 panic | `Option::None` |
| color：`primary >= reverse_len` 或 reverse==0 | miss | `None` |
| window/chimney：`rec.len58==0` | 跳过墙 | 跳过 |
| window/chimney：chunk 门失败但列表非空 | leaf=0 → **空指针解引用** | 安全模型按 miss 跳过；登记表示差异 |
| hint：primary/secondary 查询失败 | 第二数组=0 → 空指针解引用 | 同上 |
| ids/记录数组越界 | 原生无边界检查（UB） | 安全模型返回 None；有效数据等价 |
| 模式字节 | `0`=墙，非 0=屋顶 | 按非 0 处理，不枚举未知值 |

---

## 9. 合同 C12 — 建议的 Rust 访问面（供 walls owner 落地）

落点建议：`crates/country-core/src/resources/walls/dense_tables.rs`（新文件）
+ `resources/walls/mod.rs` 一行 `pub mod dense_tables;`；`InnerWalls`/`PublicWalls`
持有稠密表字段（walls 组在途文件中增量）。**不要**在 `recovered_building` 另起第二套。

```rust
pub enum DenseMode { Walls, Roofs }

pub struct DenseIndex<'a> {
    pub ids: &'a [u32],
    pub mode: DenseMode,
    pub primary_slot: u64,
    pub secondary_slot: Option<u64>, // +0x118（onboarding）
}

pub struct WallWindowRecord<'a> {      // stride 0xa0 的安全视图
    pub windows: &'a [WindowSlot],     // +0x50/+0x58
    pub chunk_key: u32,                // +0x94
}
pub struct WindowSlot { pub entity_slot: u32 } // 0x10 列表项 +8

pub struct DenseChunk<'a> {            // stride 0x48
    pub count: u64,                    // +0x10
    pub reverse: &'a [u64],            // +0x38/+0x40
    pub items: &'a [DenseChunkItem],   // +0x18, stride 0x30
}
pub struct DenseChunkItem<'a> {        // stride 0x30
    pub records: &'a [u8],             // +0x10（按消费面解释）
    pub colors: &'a [u32],             // +0x28
}

pub struct DenseWindowTables<'a> {
    pub index: DenseIndex<'a>,
    pub wall_records: &'a [WallWindowRecord<'a>],
    pub chunks: &'a [DenseChunk<'a>],
    pub sparse_records: &'a [SparseRecord],   // 0x14
    pub presence: Option<Presence<'a>>,        // +0x28/+0x38
}

impl DenseWindowTables<'_> {
    /// C8：逐窗/烟囱实体 GlobalId（墙 0xa0 列表 / 屋顶 0x48 直排）
    pub fn for_each_window_entity(&self, f: impl FnMut(u64));
    /// C9：逐提示项 (GlobalId, ty, byte_10, byte_11, height)
    pub fn for_each_highlight(&self, f: impl FnMut(HighlightEntry));
    /// C10：颜色组件（成功返回 style/appearance/color 槽引用）
    pub fn roof_color_component(&self, entity_bits: u64) -> Option<RoofColorComponentRef<'_>>;
}
```

记录类型：`SparseRecord { generation:u32, bit_index:u32, _u:u32, chunk_key:u32,
element_index:u32 }`；`HighlightEntry { global_id:u64, ty:i32, byte_10:u8,
byte_11:u8, height:f32 }`；颜色记录复用 `RoofColorEntry`。
安全视图把裸指针换为切片/索引；**步长语义必须保留在方法内**（0x10/0x18/0xc/0x58），
不得用统一 `Vec<T>` 结构掩盖 0x30 反向寻址。

---

## 10. 登记（5 条，含目标/解锁条件）

| ID | 项 | 目标文件 | 解锁条件 |
|---|---|---|---|
| **REG-PWT-1** | 落地 `dense_tables.rs` + 上述 API（C1-C12） | `resources/walls/dense_tables.rs`、`resources/walls/mod.rs`、`inner_walls.rs`/`public_walls.rs`（字段/host） | walls 组释放 `resources/walls/*` 与 `resources/roofs/roof_shape.rs` src 互斥（当前 22:03 仍在途） |
| **REG-PWT-2** | decorator 收集主循环接线：`window_lights`/`chimney_smoke` 用 `for_each_window_entity` | `crates/systems/decorator/src/window_lights.rs`、`chimney_smoke_anim_thickness.rs` | REG-PWT-1 落库；同时补 rng 种子/retain 谓词（既有登记） |
| **REG-PWT-3** | color 查找接线 + **统一密集表模型**：`set_roof_customization` 用 `roof_color_component`；`RoofColorComponentTables` 迁到/复用 `dense_tables` | `crates/systems/color/src/set_roof_customization.rs`、`recovered_building/building_colors.rs`、`systems/wall/registration.rs` | REG-PWT-1；colors 侧确认不保留双模型 |
| **REG-PWT-4** | onboarding 墙高/提示查询：`hint_highlight_shape` 用 `for_each_highlight`；修 `WallId(0)` 占位、补 `+0x11` 门、`height>0.9`、`ty∈{1,2}` | `crates/systems/onboarding/src/hint_display_system.rs` | REG-PWT-1 + `HighlightRoofCmd/WallCmd` 已落库（已满足） |
| **REG-PWT-5** | 台账同步：walls §密集表 entries；decorator §window_lights/chimney 解锁条件；color `set_roof_customization` 解锁；onboarding U11 | `planning/2026-10-17-unimplemented-ledger.md`（coordinator 维护，本任务未改） | coordinator 汇总时并入 |

原始类型命名（A/B 宿主是 `Roof` / `RoofShape` / `InnerWalls` 的哪一个，及 `+0xf8`
列表的拥有者）**登记 U**：PDB 为 `void()` 占位，本轮仅以偏移/语义定案，不为命名编造归属。

---

## 11. 验证

- `cargo check -p country-core`：**Finished，0 error，14 warnings**（约 22:23，本任务未改 src）。
- 本任务未写任何 src；未触碰 `resources/walls`、`resources/roofs`、`recovered_building`。
- 工具：ghidra `--worker shared` 只读（get_functions / disassemble_function / read_memory）。

## 12. 与既有报告的差异/纠正

1. **hint `ty` 集合**：旧 `interaction/onboarding/RESTORE.md:40` 写
   `{1,2,6,7,8,9}` 与 fresh 不符；disasm 复合门（0x141c50397-0x141c503b6）
   等价 `ty∈{1,2}`。旧同文件 :147 的 `{1,2}` 正确。
2. **阈值**：`0.9f` 由 read_memory `0x142a7c88c=6666663f` 证实；
   `UCOMISS` + `JBE` ⇒ 严格大于且 NaN 不命中（旧文只写 ">0.9"）。
3. **窗实体记录步长**：window_lights/hint 的 `item+0x10` 记录数组按 0x10（实体）/
   0x18（提示）/0xc（墙高）解释，color 按 0x58 解释；不是同一个 `Vec` 元素类型，
   实现时禁止跨消费面共用一个泛型元素类型（C5）。
4. **`A+0x100` 是长度不是末指针**：`window_lights.rs` 头注释写"数组末"，
   fresh disasm `MOV R15,[+0xf8]; MOV RAX,[+0x100]; LEA RAX,[R15+RAX*4]`
   （0x1407b5e0b-19）证明 `+0x100` 为元素数，循环终点=基址+len*4。
