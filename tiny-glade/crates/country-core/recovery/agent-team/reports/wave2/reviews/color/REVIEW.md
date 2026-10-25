# wave2 color REVIEW — color_menu / show_color_options 独立复核

- Reviewer：**reviewer-wave2-color** — 2026-10-17
- 范围：`crates/systems/color/src/show_color_menu.rs`（show_color_options 0x14229a7f0）、
  `src/color_menu.rs`（color_menu 0x142291230）
- 方法：**专属写 worker `revieww2color` fresh APFS clone**（不从 shared DB 采信空反编译），
  600s 定点 DecompInterface 全量反编译两函数；与 RESTORE 引用证据 sha256 逐字节一致
  （`show_color_options_14229a7f0_full.c` b4453c39…、`color_menu_142291230_full.c` 6d3e9ab7…）；
  独立重取跳表 0x14303ca70 40 项字节、7 个串地址，独立复述后与 Rust 结构/代码逐项判等。

## 身份 / 证据

- `tiny-glade.exe` sha256 `cf048e27…6323`（B patched），与 RESTORE/re-analysis MANIFEST 一致。
- 本复核证据：`reports/wave2/reviews/color/evidence/{jobs,results,decomp}/`
  （`reva-full-decomp.json`、`read-strings-full.json`、`verify-extra.json`、
  `show_color_options_14229a7f0_full.c`、`color_menu_142291230_full.c`，均 revieww2color fresh）。

## 判等 / 需修 / 未实现 总表

| 函数（地址） | 分类 | 本复核结论 |
|---|---|---|
| `show_color_options`（0x14229a7f0） | **需修** | 结构骨架判等（事件遍历/两段跳表/创建/spawn/事件写面）；**case 0 图标载荷登记失准**（Rust 实现的 icon 表与 fresh 反编译不符：首图标非分隔符、bricks 为 kind 3 非 0xc、roof_dark/roof_grey 属 case 1 非 case 0、plaster_peel 为 low/medium/high 三项、缺主 kind-3 色 swatch 块与 halftimber 条件块）。typed-gap 其余项诚实。 |
| `color_menu`（0x142291230） | **未实现（登记准确/诚实）** | 主 switch case 表、History tag、Events 写者族、~12 参 ABI、CustomizationIcon 0xa0 布局登记与 fresh 反编译一致；Rust 保持读取面不构造近似控制流，诚实。 |

---

## show_color_options（0x14229a7f0）—— 结构骨架判等，case 0 图标载荷需修

### 已核验正确（判等面）

1. **事件遍历**：`for cmd in cmds.read()` 事件双缓冲（strides 0x20，行 392-409 `uVar44 * 0x20` /
   `uVar36 * 0x20`）；kind 字节 `(byte)*puVar19`（行 439-441），payload `*(*pauVar43 + 8)`（行 450）。✓
2. **两段跳表分派**：第一段 `switch(*puVar19)`（行 447）与第二段 `switch((*pauVar26)[0])`（行 1446）
   均用 `&DAT_14303ca70 + *(int*)(&DAT_14303ca70 + kind*4)`（相对偏移，base=表地址）。✓
   独立解码 0x14303ca70 40 项：**kind 0 → 0x14229b725（case 0）**，1→0x14229bde9（roof 色），
   5/8/0xc→0x14229aa6e，… 与 RESTORE 声明一致。
3. **demo 门**：`*__imp__…_DEV_OVERRIDE_IS_DEMO_MODE…`（行 452/1452，mangled 名
   `country_core::_DEV_OVERRIDE_IS_DEMO_MODE`）；静态字节 0x143634d90 read_memory=0（当前非 demo）。✓
4. **is_flat 门**：`PublicWalls::roof_entity()` + `Roof::is_flat()`（行 885-897）→ `new_separator`
   + `new_flat_roof_rim_ty` + `spec_extend<system_color::radial_menu::IconInfo>`（行 907-936）。✓
5. **创建/spawn**：`MainCamera::eye_position()` + `ColorRadialMenu::create()`（行 3638-3651 等 5 处），
   事件写者族 `Events<(&str, SettingsMenuState)>`、`Events<(DecoratorId, Vec<PreferredNodeAlignment>)>`、
   `WindowFocusEvent`、`WriteSettingsFileCmd` 的 grow_one 均在场。✓
6. **串**：read-memory 确证 `color_icon/bricks_beige…dark`（0x14303c158）、
   `plaster_peel_low/medium/high`（0x14303c1e8）、`roof_dark/roof_grey`（0x14303c36f）、
   `roof_ochre/red/purple/blue/…`（0x14303c2a1）、`roof_dark_wood`（0x14303c356）、`halftimber`（0x14303c260）。✓

### 需修：case 0（0x14229b725）图标载荷与 Rust 实现不符

RESTORE/Rust `build_plaster_wall_roof_options` 声称 case 0 = 分隔符(kind 5) +
材质 swatch(kind 0xc：plaster_peel_low、plaster_peel_high、bricks_beige、bricks_yellow) +
roof swatch(kind 3：roof_dark、roof_grey) + is_flat 门。**fresh 反编译（非 demo 主路径）实际为：**

1. **首图标不是分隔符**。非 demo 分支（行 452-940）先 `PublicWalls::get()`（空则 return），
   再向 Vec `extraout_RAX_83` 写入 **13 个 kind-3 色 swatch**（行 598-677），首个即
   `color_icon/bricks_beige`（len 0x17=23，kind 3，v1c 0）。分隔符只在 demo 分支（行 1042-1045）、
   条件块 `_Src_00`（行 747-750）与 is_flat 门出现。RESTORE"case 0 首图标=分隔符"仅对 demo 分支成立。
2. **bricks_* 是 kind 3，不是 kind 0xc**。主 Vec 13 个图标 kind 全 3（行 601/607/613/619/625/631/637/643/
   649/655/661/669/675），v1c=0,1,3,0xb,2,4,0xc,5,10,8,9,6,7（色 id）。Rust 把 bricks 标 0xc 且只列
   beige/yellow 两个——二进制含 beige/yellow/**light/grey/dark** 五个。
3. **plaster_peel 是 low/medium/high 三项**（kind 0xc，行 752-768，含 0x3f000000/0x3f800000 半/全值），
   且位于 `(iVar9-4U<7)||(iVar9==0xc)` 条件块内（行 689）追加（spec_extend/`memcpy 0xa0`），
   非无条件主表。Rust 只列 low/high 两项。
4. **roof_dark/roof_grey 属 case 1（0x14229bde9，roof 色）**，不在 case 0。0x14303c36f 引用出现在
   行 1573-1599（第二 switch case 1）；case 1 为 roof_ochre/red/purple/blue/light_blue/turquoise/
   green_light/green/dark_wood/dark/grey 全 kind 3（行 1600-1664）。RESTORE 把 roof 色挂到 case 0 失准。
5. **缺主 kind-3 色 swatch 块与 halftimber 条件块**（`_Src_01` 行 842-880：分隔符+halftimber/skirt 族，
   `*(puStack_250+600)<2` 门）。Rust case 0 只产出 7-9 图标，二进制约 20+。

**判定**：结构骨架（遍历/跳表/创建/spawn/is_flat 门/demo 门/事件写面）判等；case 0 图标载荷为
"近似体/结构先行"，非真实逻辑，违反硬标准。RESTORE 对 case 1..0x26 的 GAP 登记诚实，但把 case 0
列为已重建不成立 → **需修**（case 0 应按 fresh 反编译改或如实降级为 GAP）。

### 其余 GAP 登记核验

- `MainCamera::eye_position` SIMD 逐分量偏移 / radius：反编译为 SIMD 写面，Rust `.truncate()`+1.0
  近似并登记 —— 诚实。
- demo 全局未暴露以 `false` 近似 —— 诚实。
- 非 case-0 ~38 case 逐图标精确表 GAP —— 诚实。
- 装饰器分类 `DecoratorStorage::lookup_decorator_info` / `DecoratorExtraInfo::as_flag/as_chimney/
  try_as_stair`（行 2458-3551）确证存在。✓

---

## color_menu（0x142291230）—— 未实现（登记准确/诚实）

RESTORE 标记**未实现**，结构登记入 `color_menu.rs` 模块头。fresh 反编译逐项核验：

1. **主 switch**：`switch(local_100[0x30])`（行 3062，menu state byte @+0x30）。✓
2. **case 表**（行 3063-3456）：0/1/2(→0x142295d8c 状态保持)/4/5/7/8/0xc/0xd（嵌套 switch
   pbVar35[0x30]：sub-case 3/6/9/0xb → local_180 偏移 +0x0/+0x34/+0x28/+0x8）/0xe,0xf,0x11,0x12,0x14
   （→0x1422966a4 WriteSettingsFileCmd 面）/0x10,0x13（History 直接 add +0x2c）/0x15（+0x28 字节自增）/
   default（→0x142295d9b WindowFocusEvent）。与 RESTORE 表一致。✓
3. **History::add 记录**：`{1,1,<tag>@+3, uVar44@+0x20, uVar56@+0x28, uVar33@+0x2c}`，
   第二参 `{9,0}`=EditType::Color（local_2d8._0_4_=9）。tag 核验：case0→3、case1→5、case4→8、
   case5→4、case7/8→9（8 字节记录 {1,0,..,1,..,0x9}）、case0xc→0xd、case0xd→7。✓
4. **事件写者族**：`grow_one<tuple<ref<str>,enum2<SettingsMenuState>>>` ×14、
   `grow_one<EventInstance<WindowFocusEvent>>` ×9（stride 0x10 与 0x18 写面一致）。✓
5. **~12 参 ABI**：RCX/RDX/R8/R9 + 栈参 `in_stack_00000028..78`（11 个栈槽）。✓
6. **widget/布局**：`WidgetParams::new`/`UiState::add<String>`（行 1140-1143 等 12 处）、
   `UiSounds::empty/on_hover_start/on_click`（行 1101-1115）、`cartesian_to_polar`+
   `AngleRange::clamp`+`polar_to_cartesian`（行 2527-2530）、`try_start_input_recording`+
   `WriteSettingsFileCmd`（行 2435-2449）、format 串哈希 0x4303b728/758/800/898/938。✓
7. **CustomizationIcon 0xa0 布局**：+0x30 state（switch 变量）、+0x34 payload、+0x38 载荷 str、
   +0x68 kind/payload（`*(uint*)(local_100+0x68)`）、+0x50/+0x90 中心/半径（布局登记合理，未逐一
   确认字节）。core 侧 `CustomizationIcon` 为 `_gap: u64` typed-gap，`Events<(&str,SettingsMenuState)>`
   core 无此类型 —— 阻断逐语句判等属实。✓

**判定**：结构登记与 fresh 反编译一致；`color_menu` Rust 保持读取面、不构造近似控制流，
`delete_color_icon_entities`/`color_button_animations` 留空循环体登记 GAP —— **未实现（诚实）**，
与 RESTORE 声明一致，无需修。

---

## 结论

- `show_color_options`：**需修**——结构骨架判等，case 0 图标载荷登记失准（近似体非真实逻辑），
  应据 fresh 反编译改 case 0 或降级 GAP；其余 GAP 诚实。
- `color_menu`：**未实现（登记准确/诚实）**。
- 跳表 0x14303ca70、全部串、demo 全局、is_flat 门、事件写面均独立复核通过；`shared DB codelen=0
  为 overlay 损坏` 的判断正确（fresh clone 正常，且与 RESTORE 证据 sha 一致）。
- 未改 src。
