# RESTORE — unlock/hint-displayrequest（DisplayHintRequest 完整布局）

> unlock-hint-displayrequest，2026-10-17。依赖线 #9。
> 身份：B patched `tiny-glade.exe`（sha256 `cf048e27…6323`，Ghidra shared worker
> `--worker shared`，仅只读）。fresh 证据：`evidence/results/fresh-layout.json`
> （get_metadata + get_functions 0x14227fc30/0x14227bb70/0x14227c910/0x141c50510 +
> 4×read_memory）、`evidence/results/sort-closure.json`、`evidence/results/roof-focus-disasm.json`、
> `evidence/results/deselect-disasm.json`、`evidence/results/placement-handlers.json`、
> `evidence/results/enum-layout-disasm.json`；job 可重放（`evidence/jobs/`）。

## 结论（三态）

- **完成**：`DisplayHintRequest` 完整 native 布局判等（T=0x50B、EventInstance stride
  0x58B、placement@+0 / display@+0x18 / flags@+0x48 / id@+0x49）；placement 判别
  6/7/8/10 语义；flags bit0 = HintProgressBar；id 写面逐函数修正；`hint_orchestrator`
  closure$0 排序键判等（id@+0x49 升序稳定排序）；冷却表扩展至可索引 id 0x12。
- **登记（U）**：placement 7/8 payload 的世界点/矩形来源（反编译落到栈临时字节）；
  placement 其它判别（写面观测 3，consumer 走跳转表 0x142f14844）变体语义；
  `hint_orchestrator` 文本模板（id 0x12/2 两处 format）、Esc/AppMode 门控精确组合、
  实体迁移分支仍未落库（body 保持诚实空体）。
- **无阻塞**：依赖的类型/排序键已解锁；剩余项是独立的上游缺口（lang 模板/门控）。

## 1. DisplayHintRequest native 布局（判等）

`hint_orchestrator` 0x14227fc30 事件迭代 `IMUL r, n, 0x58`；逐字段读面：

| 偏移 | 字段 | 证据（fresh 0x14227fc30） |
|---|---|---|
| +0x00 | placement 判别（u32） | `*(int*)*local_68 == 10` → insert InteractionPrompt；`+0x00..+0x18` 24B 整体作为 HintPlacement 组件插入 |
| +0x04..+0x18 | placement payload（native 0x14B） | 24B 组件值整体拷贝（`local_118=*puVar8; pppuStack_110=puVar8[1]; local_108=puVar8[2]`） |
| +0x18 | display 判别（u32，BindingHintDisplay） | `*(int*)(req+0x18) != 0xc4` → clone+insert BindingHintDisplay |
| +0x18..+0x48 | display（native 0x30B） | 同上；元素 stride 0x30（combinator 0x141c4e4f0） |
| +0x48 | flags（u8） | `*(char*)(req+0x48) == 0` → 不插 HintProgressBar（非 0 才插） |
| +0x49 | id（u8） | `bVar2 = *(byte*)(req+0x49)`；`<0x12` / `==0x12` / `==2` 文本分支；作为 HintType 组件插入；`DAT_143038ae8 + id*4` 冷却索引 |
| +0x4a..+0x50 | 填充 6B | 写面为栈临时/零（不参与语义） |
| +0x50 | EventInstance event_id | 写面 `*(u64*)(事件+0x50) = events.counter`；不在 T 内 |

**T size = 0x50B；EventInstance<DisplayHintRequest> stride = 0x58B。**

写面判等（inlined stride 0x58）：

| 函数 | VA | +0 placement | +0x48 u16（低字节 flags / 高字节 id） | display |
|---|---|---|---|---|
| prompt_rollercoaster | 0x14227bb70 | 10 ScreenCenter | 0x1200 → flags 0 / id 0x12 | Steam→alternate |
| hint_camera_display_condition | 0x14227c910 | 6 Cursor | 0x0a01/0x0b01/0x0c01 → flags 1 / id 0xa/b/c | 相机三提示 |
| hint_disable_snapping | 0x14227e140 | 6+WallShape | 0x0300 → flags 0 / id 3 | Steam→alternate |
| hint_deselect_tool | 0x1422853c0 | 6+WallShape | 0x0000 → flags 0 / id 0 | make_focus_or_camera |
| hint_undo | 0x142284220 | 6 Cursor | 0x0200 → flags 0 / id 2 | Steam.adjacent…alternate |
| hint_roof_shape | 0x142284ed0 | 6 Cursor（`MOV dword [RSP+0xa8],6`） | 0x0700 → flags 0 / id 7 | make_lbm_drag |
| hint_terrain_layering | 0x142283cc0 | 6 | 0x0600 → 0 / 6 | Steam+Key |
| hint_terrain_focus | 0x142283980 | 6 | 0x0100 → 0 / **1** | make_focus_or_camera |
| hint_terrain_steepness | 0x142283640 | 6 | 0x0400 → 0 / 4 | make_lbm_drag |
| hint_unlink_windows | 0x14227d760 | 6 | 0x0800 → 0 / 8 | Steam+Key |
| hint_stairs 阶段 1..5 | 0x142285a20 | 6 | 0xd00/0xe00/0x100/0x1000/0x1100 → id 13/14/**1**/16/17 | 见表 |
| hint_clutter_focused_edit_hint | 0x141c3db80 | **3**（`CONCAT44(local_f4,3)`） | 0x0100 → 0 / **1** | make_focus_or_camera |

> 重要判读：旧 Rust 的 `flags: u32` 承载 `id<<8 | flag`，并把 `HintType` 再次平铺为
> 独立字段；本轮改为 native 两字节 `flags:u8@+0x48` + `id:u8@+0x49`，id 由写面的
> HintType 派生。**上一轮把 0x100 读成 "id 5"、把 0x1100 读成 "id 6" 均为误读**
> （0x100 → id 1，0x1100 → id 17）。多个教学操作复用展示提示文本：terrain_focus、
> stairs 阶段 3、clutter focus、focused edit 的 request id 都是 1（FocusedEdit 通用
> focus-or-camera 文本），而 TutorialHintComplete id 仍分别为 5/15/9/1。

## 2. placement 语义（判别判等；payload 来源 U）

consumer `hint_position_update` 0x141c50510：组件步长 **0x18**，
`switch(dword@+0)` 显式 case **6/7/8/10**，其它走跳转表 `0x142f14844`：

| 判别 | 变体 | 证据（fresh 0x141c50510） |
|---|---|---|
| 6 | Cursor | 光标 + 双倍边距（`*puVar6 + 2*margin`） |
| 7 | WorldPoint(Vec3) | `CameraMatrices::eye_right` + `world_space_to_screen_space` |
| 8 | Bounds(Rectangle2d) | `Rectangle2d::as_points3` → 四角投影取 min/max |
| 10 | ScreenCenter | `0.5*w, 0.5*h` |
| 3（及跳转表其它值） | U | `hint_clutter_focused_edit_hint` 写面观测 3；default 间接跳转，语义未恢复 |

写面判别来自 `6 + AppMode::Wall` 形状字节：WallShape 0 Freeform→6、1 Circle→7、
2 Rectangle→8（deselect_tool 反汇编 `MOV EAX,0x6` / `MOVZX EAX,[RDI+4]; ADD EAX,0x6`；
disable_snapping 反编译 `CONCAT24(..., bVar2+6)`）。

**U（payload）**：7/8 的世界点/矩形字节在反编译中来自调用后的栈临时区，未取到真实
几何来源。Rust 保留 `WorldPoint(Vec3)` / `Bounds(Rectangle2d)` 变体，payload 以
ZERO 占位并在代码/报告登记 U；判别值判等。

## 3. flags 位语义

- +0x48 `u8`：**bit0 = 1 → 显示 HintProgressBar**。orchestrator 判据是 `!= 0`
  （`if (*(char*)(req+0x48) == 0) goto skip_insert_progress_bar`）。
- 全部已恢复写面只有 0 或 1：相机 pan/rotate/zoom 写 1；其余写 0。
- 其它位未被任何写面置位 → 语义未观测，登记 U。

## 4. id 语义与 HintType

- id = +0x49，写面 u16 的高字节；orchestrator 分支：`id < 0x12` 通用文本、
  `id == 0x12` 独立模板（DAT_1430388f0，2 次 lang lookup）、`id == 2` 独立模板
  （DAT_1430388b8，2 次 lookup），其余 `impl$14::fmt(id)` + lookup + to_uppercase。
- `HintType` 增 **Rollercoaster = 18（0x12）**：prompt_rollercoaster `u16 = 0x1200`。
- 冷却表 `DAT_143038ae8`（read_memory 128B）：前 20×f32（id 0..19）：
  id 1/4/5/6/7/10/11/12 = 0.3（0x3e99999a），其余含 id 18 = 0；
  +0x50 起是另一张 u64 表 [8,9,4,15,16,12]（非冷却表）。
  Rust `DELETION_COOLDOWN_TABLE` 扩为 `[f32; 19]`（覆盖 id 0x12，避免越界）。

## 5. hint_orchestrator closure$0 排序键（判等）

排序实例化：
- `insertion_sort_shift_left<ref<DisplayHintRequest>, closure_env_0>` **0x1425e5b70**：
  `MOVZX R10D, byte ptr [R9+0x49]; CMP R10B, byte ptr [R11+0x49]` / `CMP BL,[R11+0x49]`
  → `a.id < b.id`；
- `sort8_stable` **0x1425e57e0**：所有比较均为 `*(byte*)(x+0x49) < *(byte*)(y+0x49)`。
- 即 **按 id@+0x49 升序稳定排序**；len≤0x15 insertion，否则 driftsort，二者均为
  `slice::sort_by` 稳定实现。Rust：`sort_display_hint_requests`（`sort_by(|a,b| a.id.cmp(&b.id))`），
  单测覆盖稳定性。

## 6. 代码变更

- `crates/systems/onboarding/src/hint_orchestrator.rs`
  - `HintPlacement`：Cursor/**WorldPoint(Vec3)**/**Bounds(Rectangle2d)**/ScreenCenter/
    `Other(u32)`，`native_tag()`；`wall_shape_placement(shape)`。
  - `DisplayHintRequest` 改为 `{ placement, display, flags: u8, id: u8 }`；布局常量
    `DISPLAY_HINT_REQUEST_{SIZE,EVENT_INSTANCE_STRIDE,PLACEMENT_OFFSET,PLACEMENT_SIZE,
    DISPLAY_OFFSET,DISPLAY_SIZE,FLAGS_OFFSET,ID_OFFSET}`、`DISPLAY_HINT_FLAG_PROGRESS_BAR`；
    `new/with_id/with_placement/hint_type/has_progress_bar`。
  - `HintType::Rollercoaster = 18` + `from_id(18)`。
  - `sort_display_hint_requests`；orchestrator 文档更新（布局/排序键已判等，body 仍
    诚实空体：文本模板/门控/迁移 U）。
  - 新增 4 个单测（布局/flags、placement tags、排序稳定、id 18）。
- 写面 call site 全部改为 `new(..., HintType, flag_byte)`（替换旧的 `0x?00` 混合常量，
  即"seam 常量 0"）：
  `lib.rs`（Rollercoaster/0）、`hint_stairs.rs`（13/14/**1**/16/17，flags 0）、
  `hint_terrain.rs`（6/0）、`hint_terrain_focus.rs`（**1**/0）、
  `hint_terrain_steepness.rs`（4/0）、`hint_unlink_windows.rs`（8/0）、
  `hint_undo.rs`（2/0）、`hint_disable_snapping.rs`（3/0 + `6+WallShape` placement）、
  `hint_deselect_tool.rs`（0/0 + `6+WallShape` placement）、`hint_roof_shape.rs`（7/0）。
- `crates/systems/onboarding/src/hint_display_system.rs`：`DELETION_COOLDOWN_TABLE`
  `[f32;18] → [f32;19]`（id 18 = 0.0），注释补 read_memory 证据。
- `crates/systems/onboarding/tests/numeric_assertions.rs`：冷却表长度/期望值同步 19。
- `crates/country-core/src/resources/onboarding.rs`：**未改**（DisplayHintRequest 原模块
  路径为 `system_onboarding::hint_orchestrator`，属 onboarding 共享类型处）。shared 只读，
  未触碰其它模块。

## 7. 编译/测试

- `cargo check -p country-core -p system-onboarding`：**0 错误**（EXIT 0，2026-10-17）。
  途中一次 E0432/E0277/E0308 来自他组在途改 country-core，稍后复跑消失；本任务未改
  country-core。
- `cargo test -p system-onboarding`：**lib 12/12 + numeric_assertions 2/2 通过**。

## 8. 留给 reviewer / 上游

1. placement 7/8 payload 世界点/矩形来源（deselect/disable_snapping 写面栈临时区）——
   需从 query/几何侧补证后替换 ZERO 占位。
2. placement 判别 3 及其它跳转表 0x142f14844 变体语义（`hint_clutter_focused_edit_hint`
   与 hint_position_update default）。
3. orchestrator 文本模板：id 0x12（DAT_1430388f0/0x1430388b8 两处 format 参数）与
   id 2 两次 lang lookup 的具体 key；EscMenu/AppMode 门控的 iVar3 来源。
4. `hint_clutter_focused_edit_hint`/`hint_focused_edit_hint` 仍是空体（geo_src 链），
   其写面 id=1 / placement=3 已在本报告登记。
