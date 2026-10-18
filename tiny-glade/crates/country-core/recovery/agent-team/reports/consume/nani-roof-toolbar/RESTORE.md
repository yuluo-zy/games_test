# RESTORE — consume/nani-roof-toolbar（restorer-consume-nani-roof-toolbar，2026-10-17）

> 任务：roof `switch_roof_type`（0x1421ac4f0）mesh ops 子项；toolbar `add_toolbar_item`
> （0x141e4dc40）8 组件 spawn 面。方法：每函数 fresh 反编译 + `disassemble_function`
> 逐指令判等（硬标准 / 分歧检查表 8 条）。工具：ghidra `--worker shared`（只读），
> 原始输出在本目录 `evidence/`。身份：B patched EXE（`cf048e27…`，get_metadata 复核）。
> **wave4 复核后已落实 FIX-1/2/3/6（见 §7）**；类型大小采用 `ComponentDescriptor`
> `{align@[rbp+0x1f8], size@[rbp+0x200]}` 立即数（直接证据，见 §3.2）。

## 0. 结论速览

| 函数 / 子项 | 判定 | 交付 |
|---|---|---|
| roof `switch_roof_type` mesh ops（NaniBundle insert / DrawableMeshBundle remove / format+try_get_handle_by_name / deferred_delete_mesh / Handle\<Vao\> remove） | **判等 5/5** | `crates/systems/roof/src/visual/switch_roof_type.rs` |
| toolbar `add_toolbar_item` spawn 面（8 组件 + 2 附加 insert + 组件序/容器流/Style/读面） | **判等 13** | `crates/systems/toolbar/src/resouces_and_components/{toolbar_layout,mod}.rs` |
| toolbar 组件字段语义/签名简化 | **重建 2（I）** | 同上 |
| toolbar `Ui2dImage::new(label)`、`Ui2dExtent` 16B 计算值、`kind_handler` 载荷 | **未实现 2（typed-gap）** | 同上 + 本报告登记 |

`cargo check -p system-roof -p toolbar`：**通过**（`Finished dev profile`，0 error；
仅 country-core/roof 既有 warning，改动文件无新 warning）。

## 1. 证据（本目录 `evidence/`）

- `fresh-0x1421ac4f0.c` / `fresh-0x141e4dc40.c`：两函数 fresh 反编译（与
  unlock/nani job1 逐字节一致）。
- `disasm-0.txt` / `disasm-1.txt`：`disassemble_function` 全量指令（判等主依据）。
- `result-apis.json`：`0x140aa8070 deferred_delete_mesh`、`0x140ceac00
  bevy_ecs::entity::impl$10::fmt`（= Entity **Debug**：`{index}v{gen}#{bits}`）、
  insert/remove call_once。
- `result-helpers.json` + `read_memory 0x140833250`：单位函数 `89 C8 C3`（EAX=ECX）
  ⇒ `From` 输入 0（AtlasPart(0)）。
- `result-consts.json`：`[0x142f8f600]=(82.5,82.5)`、`[0x142f8f610]=(1.0,0.011)`。
- `result-cids.json` / `result-regcomp.json` / `result-sizes.json`：8 个
  `register_component` 单态顺序与 `ComponentDescriptor` 的 align/size 立即数
  （见 §3.2）；`result-wc.json`/`wcdis.txt`：`write_components<tuple<…>>`。
- `result-spawn.json` / `spawn-0x140fe0470.c`：`Commands::spawn<(8 组件)>` 复制
  bundle 13 qword（0x68 B）。
- PDB inline：`skeleton-enriched/B/crates/systems/toolbar/src/resouces_and_components/
  toolbar_layout.rs` — `spawn_drawable`（toolbar_layout.rs:446）内联
  `Ui2dImage::new`（components/mod.rs:229）与 `components::impl$45::default`
  （= `Ui2dExtent::default`，source line 205）。
- 复用：unlock/nani、wave2b、wave3 toolbar RESTORE；`reports/wave4/reviews/visual/REVIEW.md`。

## 2. roof `switch_roof_type`（0x1421ac4f0）· 判等 5/5

事件面（stride 0x20：entity@+0x10、source@+0x18、target@+0x19；RoofSound stride
0x28、WallGableChanged/RecalculateWallBricks stride 0x10；ridge==1.0 的
UCOMISS/JNZ+JP NaN 语义；`MAXSS(1.6,h)/MINSS(14,·)`）沿用原有判等实现。

| mesh 子项 | fresh 证据 | 判定 |
|---|---|---|
| flat：`entity(roof).insert(NaniBundle<StoneFloorInstanceData>)` | 0x1421ac9c8..0x1421aca2a：单位 Transform、desc tag=1/value=0、subset=9；insert 错误处理器 `handler::panic` | 判等 |
| pitched：`remove::<DrawableMeshBundle>` | 0x1421ac812 区；`handler::warn`（bevy 0.16 `remove` 默认 warn） | 判等 |
| pitched：`format!("roof_{:?}", entity)` → `try_get_handle_by_name` | 0x1421ac850..b8；pieces "roof_" + Entity Debug（0x140ceac00，三参） | 判等 |
| pitched：`Some(h)` → `deferred_delete_mesh(h)` | 0x140aa8070：Mutex lock（中毒 `unwrap_failed`）→ `Vec::push` → unlock/wake | 判等 |
| 恒有：`remove::<Handle<Vao>>` | 0x1421accc7 区；warn | 判等 |

实现：`switch_roof_type.rs`（bundle 常量、错误处理器、事件顺序均按机器码）。

## 3. toolbar `add_toolbar_item`（0x141e4dc40）

### 3.1 spawn 面（fresh 0x141e4e433..0x141e4e50d）

bundle 载荷 0x68 B；组件声明序 = tuple 名序（O）。

| 组件 | B 布局（O，ComponentDescriptor） | B 值（O） | 判定 |
|---|---|---|---|
| `TaffyNode` | 20B（size 0x14，align 4） | 本次 `new_leaf` 的 Node（index@bundle+0x40、gen@+0x44） | 判等（值）；size 见 §7 |
| `UiScaleWithUserSetting` | ZST（size 0，align 1） | 无载荷 | 判等 |
| `UiInteractionState` | 3B（align 1） | 全零 | 判等（值）；字段语义 I |
| `UiAnimationState` | 16B `{f32,f32,[u8;4],u8@0xc,[u8;3]}` | `{0,0,0,state=2}`（+0x2c byte 2） | 判等 |
| `UiMouseInteractable` | 4B（align 4） | 全零 | 判等（值）；字段语义 I |
| `Ui2dExtent` | 16B（align 8） | `item.size(+0x30 两 f32) * (82.5,82.5)` + `(1.0,0.011)`（`MULPS`/`UNPCKLPD`，载荷 +0x48..+0x58） | **未实现**（core 8B `{_gap:u64}` typed-gap；值已取证） |
| `Ui2dImage` | **32B**（size 0x20；`String` 类载荷 24B + tail 8B） | 克隆 label（alloc+`memcpy`，`Ui2dImage::new` PDB inline；写 3 qword len/ptr/len + +0x18 u32 0） | **未实现**（core typed-gap；克隆保留登记） |
| `UiDrawOrder` | 8B（align 8） | 0（+0x60 qword） | 判等 |

附加 insert：`UiShowIfParentIsInFocus`（8B `{u64}`，无条件，handler=panic）判等；
`item+0x38 != 0` → `ShowOnlyInFpvAndFree`（ZST，insert 命令 0x20）判等。
容器流（kind 4/5、stride 0x40、界 `+0x38 != 2`、尾项 drop、缓冲 dealloc）判等（FIX-1）。
Style 实参 = 静态模板 0x142bc2680（≈`Style::DEFAULT`）判等（I）。签名简化为 4 参/容器
节点为父（B 6 参 + NodeId 双返回）登记为重建。

### 3.2 类型 size 证据（wave4）

`ComponentDescriptor` 构造点的 `{align@[rbp+0x1f8], size@[rbp+0x200]}` 立即数
（`result-sizes.json`；与 `register_component<T>` 内 `local_98/local_90` 一致）：

| 类型 | align | size | 来源 |
|---|---|---|---|
| TaffyNode | 4 | 0x14 | 0x142383a39/a44 |
| UiScaleWithUserSetting | 1 | 0 | 0x142384129/134 |
| ShowOnlyInFpvAndFree | 1 | 0 | 0x142386ac9/d4 |
| UiInteractionState | 1 | 3 | register_component 0x141e34530 |
| UiMouseInteractable | 4 | 4 | 0x141e35eb0 |
| Ui2dExtent | 8 | 0x10 | 0x141e35850 |
| Ui2dImage | 8 | 0x20 | 0x141e33ed0 |
| UiDrawOrder | 8 | 8 | 0x141e351f0 |

尺寸闭合：20+0+3+16+4+16+32+8 = 99 → align 8 补齐 **104 = 0x68** ✓（与 spawn
复制 13 qword 一致）。因此 **FIX-2 的 8B 与 FIX-3 的 bool 推断作废**：TaffyNode
20B、ShowOnly ZST 才是与 `0x68` 闭合及 descriptor 立即数同时一致的解释
（reviewer 的闭合式假设 UiScale=16/Ui2dImage=24）。并发 pass 已按此落库。

其它类型修正（保留）：`ToolbarItemKind` +0x20/+0x28 = label fat pointer、
+0x30 = 两 f32（Vec2 读面）、+0x38 标志、stride 0x40；容器 Vec 内存序
`{cap@+0x08, ptr@+0x10, len@+0x18}`（O，见 §7 FIX-1）。

## 4. 未实现 / typed-gap 汇总

| 项 | 缺口 | 解锁 |
|---|---|---|
| `Ui2dImage` | core `{_gap:u64}`；B 32B（label String 类 24B + 8B tail），`new(&str)` | core owner 落 32B + `new` |
| `Ui2dExtent` | core 8B `{_gap:u64}`；B 16B（spawn 值 `size*82.5 + (1.0,0.011)`） | core owner 落 16B/Vec2 面 |
| `UiInteractionState`/`UiMouseInteractable` 路径 | B 为 `country_core::components::*`，现暂置 toolbar 本地 | core 迁移后改 re-export |
| `kind_handler` | 0x60B/0x20B 记录、fn@0x140fdf9a0/0x140fdf500、[rax+0x18] vtable、kind2 朝向表 U | 后续 kind 载荷批次 |
| 签名 | 4 参简化 vs B 6 参/NodeId 双返回 | 消费侧 mei 接线 |
| `NaniMeshSelector` 判别序 | core 源码 `AtlasPart=0,Named=1` vs O `1/0` | core owner 显式判别（只读未改） |
| `SwitchRoofTypeCmd` | +0x19 实为 RoofType | core owner 校正 |
| TaffyNode tail / Ui2dImage tail 字段语义 | U（布局 O） | PDB/消费侧 |

## 5. 编译
`cargo check -p system-roof -p toolbar` → `Finished dev profile`，**0 error**。
附带最小 compile-unblock：`roof/ui_edit_roof_ridge_length.rs` 的
`WidgetParams::new()` 旧签名以零值占位（注释登记，非本批业务）。

## 6. 计数（统一口径，FIX-6）
- roof：判等 **5**（mesh 子项），无重建/未实现。
- toolbar：判等 **13**（8 组件中值级 5 + 零载荷/类型读面 + 两条附加 insert +
  组件序/容器流/Style 常量），重建 **2**（字段语义 I + 签名简化），未实现 **2**
  （`Ui2dImage`、`Ui2dExtent` 载荷；`kind_handler` 另计 1 属既有登记）。
- 全报告合计：判等 **18**（roof 5 + toolbar 13），重建 **2**，未实现 **2**
  （另 `kind_handler` 1 项为既有未实现登记，不计入本轮新增）。

## 7. wave4 FIX 落实

| FIX | 处置 |
|---|---|
| FIX-1 容器缓冲未释放 | **已修**：`add_toolbar_item` 容器分支末尾按 0x141e4e794 补 `dealloc(ptr=[+0x10], size=cap([+0x08])<<6, align=8)`；同时按机器码把 Vec 字段序改为 `{cap@+8, ptr@+0x10, len@+0x18}`（原 ptr/cap 顺序相反，属同一资源面缺陷）。 |
| FIX-2 TaffyNode 16→8B | **以更强证据改判为 20B**：`ComponentDescriptor` {align=4,size=0x14} 立即数 + `0x68` 闭合（需 Ui2dImage=32/Ui2dExtent=16）；reviewer 闭合式假设 UiScale=16/Ui2dImage=24。并发 pass 已落 20B。 |
| FIX-3 ShowOnly 16→bool | **以更强证据改判为 ZST**：descriptor {align=1,size=0}（0x142386ac9/d4）且 insert 命令 0x20；bool 为 1B 与 size 0 不符。并发 pass 已落 ZST。 |
| FIX-6 计数矛盾 + Ui2dImage 精度 | **已修**：§6 统一计数；`Ui2dImage` 更正为 **32B**（descriptor size 0x20；24B String 类写面 + 8B tail），并登记 typed-gap。 |
