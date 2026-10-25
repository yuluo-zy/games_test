# REVIEW — wave4 NaniBundle 消费者（nani-decorator / nani-roof-toolbar）

> reviewer：`reviewer-wave4-visual`（2026-10-17）
> 范围：`reports/consume/nani-decorator/RESTORE.md`、`reports/consume/nani-roof-toolbar/RESTORE.md`
> 方法：逐项 **fresh 反编译 + 独立反汇编**（不复用 RESTORE 结论）→ 独立复述 → 与
> `crates/systems/{decorator,roof,toolbar}` Rust 逐值对照 → verdict。重点核验 layout /
> 常量 / 条件位 / 组件载荷。
> 身份：B patched EXE（get_metadata：`tiny-glade.exe`，base `0x140000000`，x86:LE:64，
> 131517 函数；与 `cf048e27…` 会话同一持久库）。
> 工具：ghidra `--worker reviewer-wave4-visual`（只读）+ radare2 唯一 worker
> `reviewer-wave4-visual`（RNE 机器码 oracle）。
> 本轮 fresh 产物：`work/reviewer-wave4-visual/`（`fresh-*.c`、`*.disasm.txt`、
> `jobs/job1..job17-result.json`）。**未改任何 src。**

## 0. 三态回执

| 项 | 数 | 说明 |
|---|---|---|
| **判等** | **23** | decorator 5 组 + roof 5 子项 + toolbar 13 项（8 组件值级 5、零载荷 3、组件序/两条附加 insert/容器流/Style 常量/类型读面 5） |
| **需修** | **6** | 1 处逻辑资源面（容器缓冲未释放）+ 2 处组件大小 + 3 处报告/台账事实更正（详 §4） |
| **未实现** | **2** | `Ui2dImage::new(label)` 载荷；`kind_handler` 命令记录载荷（均已在 RESTORE 登记，未虚报） |

补充登记（非逻辑 bug，已存在 typed-gap）：12 个 decorator 条件组件 stand-in、`NaniSolidTexture`
变体名、`NaniMeshSelector` 判别序、builder setter 载荷 U、`SwitchRoofTypeCmd` 目标字段、
`Ui2dExtent`/`UiDrawOrder` core 字段。

## 1. nani-decorator（`build<Transient>` 0x1412ab270 / `build<Persistent>` 0x1412a9b30）

fresh 两实例 `diff`：仅标签地址不同（同体，3281 B）。判等 5 组：

### 1.1 mesh 名 12 类分派 — 判等

跳表 `0x142c2b058`（31×i32 相对偏移，len 9..0x27）解码 + 分支体 fresh（r2）：

| len | 目标 | fresh 名称/字节比对 | 实例类型 | subset | selector |
|---|---|---|---|---|---|
| 9 | 0x1412ab3fc | `nani_uber` | UberInstanceData | 0x17=23 | Named(tag 0) |
| 10 | 0x1412ab612 | `nani_cloth` → 0x1412abbb4 | ClothInstanceData | 0x14=20 | Named(0) |
| 10 | 同上 | `nani_smoke` → 0x1412ab656 | SmokeInstanceData | 0x15=21 | **AtlasPart(tag 1,value 0)** |
| 17 | 0x1412ab4a3 | `nani_balcony_door` | BalconyDoorInstanceData | 0x16=22 | Named(0) |
| 21 | 0x1412ab89d | `nani_lantern_emissive` | LanternEmissiveInstanceData | 0x12=18 | Named(0) |
| 23 | 0x1412ab953 | `nani_solid_vertex_color` | SolidVertexColorInstanceData | 3 | Named(0) |
| 25 | 0x1412ab55c | `nani_gothic_window_bricks` | GothicWindowBricksInstanceData | 7 | Named(0) |
| 27 | 0x1412ab6ce | `nani_clutter_SolidAlbedoMap` | SolidAlbedoMapInstanceData | 0xf=15 | Named(0)+NaniSolidTexture |
| 30 | 0x1412ab7b9→0x1412abc3c | `nani_solid_vertex_color_window` | SolidVertexColorWindowInstanceData | 4 | Named(0) |
| 30 | 0x1412ab7b9 第二支 | `nani_solid_vertex_color_bricks` | SolidVertexColorBricksInstanceData | 6 | Named(0) |
| 35 | 0x1412abafa | `nani_clutter_SolidAlbedoMapEmissive` | …EmissiveInstanceData | 0x10=16 | Named(0)+NaniSolidTexture |
| 36 | 0x1412aba2c | `nani_solid_vertex_color_window_glass` | …WindowGlassInstanceData | 5 | Named(0) |
| 默认 | 0x1412abb1d | `AssetMeshLibrary::get_handle_by_name`(0x14092d4f0) + `get_raster_handle_by_name`(0x140937aa0).expect | DrawableMeshBundle | — | — |

- `Named` 判别 tag=0 + value=`NameHash::compute(mesh 名)`（0x140c9cd80）逐分支确认
  （如 0x1412ab41b/0x1412ab42e）。
- `Smoke` 的 `impl$1::from` fresh（0x14082bd60）：`mov [rcx+4],edx; mov dword [rcx],1`；
  `DecoratorRank::new`（0x140833250）为 `mov eax,ecx; ret`，调用点 `xor ecx,ecx` ⇒
  AtlasPart(0)。Rust `NaniMeshSelector::AtlasPart(...)` + `NaniMeshSelector::Named(NameHash::compute(...))` 一致。
- 两个 albedo 分支对 builder `+0x150` 做 unwrap：`cmp r12b,9; je unwrap_failed`
  （0x1412ab71f 等）⇒ None=9，Rust `Option<u8>.unwrap()` 语义等价（panic 文本差异）。
- `GpuWorldSubsetKind` 判别值与 core 枚举逐一相符（3/4/5/6/7/9/15/16/18/20/21/22/23）。

### 1.2 transform — 判等

fresh 0x1412ab2ad..0x1412ab3bb：
- rotation = builder `+0xc0..0xd0`（Quat x,y,z,w）；translation = `+0xd0/+0xd4/+0xd8`；
  scale = `+0xdc/+0xe0/+0xe4`。
- `|normal_offset(+0x140)| > 0`（`SQRTSS; UCOMISS 0; JBE`，NaN/0 跳过）→
  `translation += q*offset`（逐分量公式与 glam `Quat*Vec3` 相同）。
- `+0x30 == 1` → scale 取 `+0x34/+0x38/+0x3c`（覆盖）。
- `+0xe8`（builder 对齐 padding，8 B）被原样拷入 bundle transform 尾部
  （Transform 0x30 B，align 16）——Rust 不复制 padding，语义无关（见 §4 FIX-3 备注）。
Rust `compute_transform()` 与之一致；4 个单测通过（本轮复跑）。

### 1.3 条件组件链 14 位（O 顺序 + 条件 + 载荷） — 判等

fresh 0x1412abe15..0x1412ac6xx 逐位复核：

| # | 组件 | 机器码条件 | 载荷（builder 偏移） | Rust |
|---|---|---|---|---|
| 1 | ColorId | `+0xf0 &1` | u32@+0xf4 | `Option<ColorId>`（core `ColorId(pub u32)`） |
| 2 | GothicWindowBrickDeformation | `+0x40 == 1` | 0x3c B @+0x44 | `[u8;0x3c]` |
| 3 | BrickColor | **`+0x114 != 2`**（0x1412abe2c `CMP…,2; JNZ`） | 0x18 B @+0x100（tag 在 +0x114） | `[u8;0x18]` |
| 4 | ClothlineCloth | `+0x80 == 1` | u64@+0x84 + u32@+0x8c | `(u64,u32)` |
| 5 | SemiTransparent | `+0x14e != 0` | ZST | bool |
| 6 | WindowLight | `+0x10 &1` | u64@+0x18 + id | `Option<u64>` + id |
| 7 | BalconyDoor | `+0x14f != 0` | translation + `rotation*Z`（逐分量=标准公式，见下） | 一致 |
| 8 | Smoke | **`+0x14c != 2`** | u64(id) + u32 0 + u8@+0x14c + u8@+0x14d | 一致 |
| 9 | AlbedoMap | `+0x20 == 1` | u64@+0x28 | 一致 |
| 10 | NormalMap | `+0x00 &1` | u64@+0x08 | 一致 |
| 11 | RoofFlagCloth | `+0xf8 == 1` | u32@+0xfc | 一致 |
| 12 | WallFlagCloth | **`+0x13c != 2`**（0x1412ac428 `CMP…,2; JNZ`） | u64@+0x130 + u64@+0x134 + u32@+0x13c | 一致（0x14 B） |
| 13 | LanternLightParams | `+0x118 == 1` | `rotation*dir` 3f32 + 原始 4f32@+0x11c + id + u8 0 | 一致 |
| 14 | SittableForBirds | `+0x90 == 1` | id + u64@+0x94 + u32@+0x9c | 一致 |

- BalconyDoor `rotation*Vec3::Z` 的 fresh 公式（0x1412ab884..910）逐分量等于
  `(2(xz+wy), 2(yz−wx), 1−2(x²+y²))`，即 glam `Quat * Vec3::Z`。
- Lantern 方向旋转公式同为标准 q·v；原始 4 f32 原样复制；id@+0x1c、byte 0@+0x20。
- **RESTORE 的极性更正成立**：BrickColor 插入条件确为 `!=2`（wave3 的 `==2` 作废）。
- 顺序与 Rust 完全一致（1→14），状态落点最后。

### 1.4 状态落点 impl$4 / impl$5 — 判等

- `impl$4` 0x1408119e0：哈希 entry 命中后 `sealed &1 != 0` → `panic_fmt`；否则
  `grow_one<usize>` + `*(u64*)(vec+len*8)=in_R8` ⇒ `HashMap<u32,{seal,Vec<Entity>}>` push。
  与既有 `DecoratorPersistentVisualState::add_decorator_visual_entity` 一致。
- `impl$5` 0x140811c30：末尾 `grow_one<usize>`（**8 B 步长**）+ push `in_R8`；
  调用点 0x1412ac662 `MOV EDX,ESI; MOV R8,RBX; CALL`（RBX=实体，见 0x1412ac410
  `MOV [cmd+0x10],RBX` 的 insert 实体槽）。⇒ `HashMap<u32, Vec<Entity>>`。
  **RESTORE 对 wave3 “autoclutter 表（0x60 步长）”的否证成立**；Ghidra 的
  `Vec<DisplayChildAutoClutterInfo>` 类型应用与机器码矛盾。
- Rust `DecoratorVisualStateSink` 两个 impl 语义一致。

## 2. nani-roof-toolbar · roof `switch_roof_type` 0x1421ac4f0（5/5 判等）

| 子项 | fresh 证据 | verdict |
|---|---|---|
| flat：`entity(roof).insert(NaniBundle<StoneFloorInstanceData>)` | 0x1421ac9c8 区；transform=单位（qword 0 / `0x3f800000` ×3）、`XOR ECX,ECX`→`impl$1::from`(tag 1,value 0)、subset byte **9**（0x1421ac98b） | 判等 |
| pitched：`remove::<DrawableMeshBundle>` + warn | 0x1421ac812 区；`entity_command::remove`，handler=`warn` | 判等 |
| `format!("roof_{:?}", entity)` → `try_get_handle_by_name` | `roof_` pieces + `bevy_ecs::entity::impl$10::fmt`；fmt fresh 0x140ceac00 = `{index}v{gen}#{id}` 三片段（0x1ffffffff→"PLACEHOLDER"） | 判等 |
| `Some(h)` → `deferred_delete_mesh(h)` | 0x140aa8070 fresh：Mutex lock（中毒 `unwrap_failed`）→ `grow_one<…>` + `push(in_RDX)`（8 B）→ unlock/wake | 判等 |
| 恒有 `remove::<Handle<Vao>>` + warn | 0x1421accc7 区 | 判等 |

补充：flat/pitched 两分支的 bundle 值相同（identity/AtlasPart(0)/9）；pitched 分支
`XOR ECX,ECX` 同样确认（0x1421acab0）。Rust 实现逐值一致。

## 3. nani-roof-toolbar · toolbar `add_toolbar_item` 0x141e4dc40

### 3.1 spawn 面（fresh 0x141e4e433..0x141e4e50d + `write_components`）

| 项 | fresh 值 | verdict |
|---|---|---|
| 组件顺序 | tuple 名序 = TaffyNode, UiScaleWithUserSetting, UiInteractionState, UiAnimationState, UiMouseInteractable, Ui2dExtent, Ui2dImage, UiDrawOrder | 判等 |
| TaffyNode | `new_leaf`(0x140fe7600) 的 Node（`R12D/R15D`←`[RBP+0x1b4/0x1b8]`），**非 default** | 判等（值）；大小见 FIX-2 |
| UiScaleWithUserSetting | `MOVSD xmm6,[item+0x30]` → `MULPS [0x142f8f600]`（=82.5,82.5）→ `UNPCKLPD [0x142f8f610]`（=1.0,0.011）→ `{size*82.5, (1.0,0.011)}` | 判等 |
| UiInteractionState | `MOV dword [RBP+0x48],0` = bundle+0x18 全零 4 B | 判等（值）；布局 I |
| UiAnimationState | bundle+0x20 qword 0、+0x28 dword 0、**+0x2c byte 2** ⇒ `{0,0,0,state=2,0}` | 判等 |
| UiMouseInteractable | `MOVUPS [RBP+0x60],XMM0(0)` = bundle+0x30..0x40 16 B 零 | 判等（值）；布局 I |
| Ui2dExtent | 零载荷 | 判等（值）；core 字段 gap |
| Ui2dImage | 分配+`memcpy`(0x142923c00) 克隆 label，写入 **24 B（三 qword：len/ptr/len）** ⇒ String 类；Rust 以 `default()` 落库 | **未实现**（登记） |
| UiDrawOrder | bundle+0x60 qword 0 | 判等 |
| 无条件 `insert(UiShowIfParentIsInFocus)` | 0x141e4e501.. 区；handler=panic | 判等（值） |
| `item+0x38 != 0` → `insert(ShowOnlyInFpvAndFree)` | 0x141e4e586 `CMP byte [item+0x38],0` | 判等（条件）；大小见 FIX-3 |
| 容器分支 | kind 4/5；stride 0x40 递归；循环界 `child+0x38 != 2`；停止位之后尾项 drop `(end-next)>>6` | 判等（控制流）；缓冲释放见 FIX-1 |
| `new_leaf` 的 Style 实参 | 静态模板 0x142bc2680（0x158 B，常量；flex_grow=0.0@0x148、flex_shrink=1.0@0x14c 等）≈ `Style::DEFAULT` | 判等（I：常量形状核对） |
| ToolbarItemKind 读面 | +0x20/+0x28 = Box<str>（alloc+memcpy）；+0x30 两 f32 → Vec2；stride 0x40 | 判等 |
| 签名简化（4 参 vs B 6 参/NodeId 双返回） | RESTORE 已登记重建 | 重建（已登记） |

### 3.2 关键常量核对
`read_memory 0x142f8f5e0..`：`0x142f8f600 = (82.5,82.5)`、`0x142f8f610 = (1.0,0.011)`
（此前一轮的偏移心算已复核，RESTORE 数值正确）。`0x142f8f6c0 = {ptr,0x44}`。

## 4. FIX 清单（需修）

| # | 级别 | 项 | 证据 / 要求 |
|---|---|---|---|
| FIX-1 | 中（资源保真） | `add_toolbar_item` 容器分支未释放子项缓冲 | B 在容器分支末尾 `if (local_98 != 0) __rust_dealloc()`（fresh decompile `local_98 != 0` 块）。Rust 用裸指针读取后不 dealloc → 泄漏。修复需在重建的 `ToolbarItemKind` 所有权模型里补 drop/dealloc（或改为 `Vec<ToolbarItemKind>` 拥有）。 |
| FIX-2 | 中（类型大小） | `TaffyNode` 16 B vs 证据 8 B | spawn 只写 8 B（index/gen@bundle+0x40/+0x44）；UiScaleWithUserSetting 从 +0x48 起 ⇒ TaffyNode 不可能 16 B。且 bundle 总长 0x68 的尺寸闭合：Taffy 8+USS 16+UIS 4+UAS 16+UMI 16+Extent 8+Image 24+DrawOrder 8=100→(align 8)104=0x68（Taffy 16 会得 0x70，不符）。Rust `{node,_gap:u64}` 应改 8 B（`taffy::Node` 单字段）。 |
| FIX-3 | 低（类型大小） | `ShowOnlyInFpvAndFree` 16 B vs B insert 载荷 ≤8 B | 两条 insert 命令：UiShowIfParentIsInFocus 预留 0x28（payload 0x10：u64@+0x18 + byte@+0x20），ShowOnly 预留 0x20（payload 0x8：byte@+0x40 + pad）⇒ Rust 两结构均 `{bool,_gap:u64}`=16 B，ShowOnly 应为 1 B bool（按 8 对齐 padded 到 8）。 |
| FIX-4 | 高（文档/台账事实） | decorator RESTORE §1 调用点 ABI 写错 | `[RSP+0x178]` = **&mut Commands**（arg6），不是实体；`[RSP+0x180]` = &mut 状态表（arg7）；build 内用 `Commands::spawn<NaniBundle<T>>` 自建实体（helper 0x14126dfe0 走 `spawn_empty`），arg5 未被 build 读取。"预分配实体"论证作废；Rust 的 `build(...,entity,...)` + 包装 `spawn_empty()` 行为等价，但理由与字段说明须更正。 |
| FIX-5 | 高（文档/台账事实） | `decorator_visual_state.rs` 注释仍称 impl$5 = autoclutter 表，与 fresh 结论/新实现矛盾；`_AUTOCLUTTER_IMPL5_UNIMPLEMENTED` 应重写为瞬态实体表说明。 | 纯注释/台账（本轮不改 src）。 |
| FIX-6 | 低（报告计数/精度） | toolbar RESTORE 计数自相矛盾：§0 “判等 7 / 重建 3 / 未实现 2”，§6 “判等 12(roof5+toolbar7)…重建3/未实现2”；且 `Ui2dImage` fresh 证据为 24 B String 类载荷（不是 16 B Box<str> 面） | 统一计数：toolbar spawn 面 = 8 组件（判等 5 值 + I 2 + 未实现 1）+ 2 附加 insert + 容器流；补记 Ui2dImage 24 B。 |

补充（非 FIX，已在 RESOTRE 登记且本轮确认属实）：
- `NaniMeshSelector` 判别序（源码 `AtlasPart=0,Named=1` vs O `AtlasPart tag=1/ Named tag=0`）——core owner 需显式判别；否则若消费方按 core 判别解析 raw 字节会错。
- 12 个 stand-in 组件：`Smoke`(14 B vs 可能 16)、`ClothlineCloth`/`WindowLight`(12 vs 16)
  的真实 component size 需在 core 落库时按 PDB 复核（当前仅载荷字节数）。
- roof 分支源用 `roof.is_pitched()`（height 观测）而非事件 target 字节；B 有
  `assert target==Roof::ty()`，仅在 `ty()` 与 `is_pitched()` 完全等价时行为一致（预存风险，非本批 mesh ops 缺陷）。

## 5. 高估检查

- **未发现伪装成判等的逻辑近似体**：decorator 的 12 类分派/14 位链/transform、roof 5 子项、
  toolbar 8 组件取值均以 fresh 机器码逐值复述；`BrickColor !=2` 与 `impl$5` 两处更正成立。
- RESTORE 存在 **2 处事实性高估**（FIX-4/FIX-5）与 **1 处计数高估**（FIX-6），均在文档层；
  实现层另有 2 处类型大小与 1 处资源释放（FIX-1..3）。
- `Ui2dImage` 未被声明为完成（诚实）；`kind_handler` 未实现（诚实）。
- 编译/测试独立复核：`cargo check -p system-decorator -p system-roof -p toolbar`
  → `Finished dev profile`，0 error（仅 country-core/既有 warning）；
  `cargo test -p system-decorator --lib utils::visual_entity_builder` → 4 passed。
  与 RESTORE 声明一致。

## 6. 结论

- nani-decorator：**逻辑判等**（build×2 同体，5 组），需报告事实更正（FIX-4/5）。
- nani-roof-toolbar：
  - roof mesh ops **5/5 判等**，无逻辑需修；
  - toolbar spawn 面 **值级判等**，需修 3 项（TaffyNode/ShowOnly 大小、容器缓冲释放）+
    1 项未实现载荷（Ui2dImage）+ 1 项未实现分派（kind_handler）。
- 建议状态：`nani-decorator = review_passed（附台账更正）`；
  `nani-roof-toolbar = fix_pending（FIX-1/2/3；FIX-6 台账）`。
