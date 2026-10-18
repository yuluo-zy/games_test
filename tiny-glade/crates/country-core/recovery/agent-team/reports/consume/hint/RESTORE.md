# RESTORE — consume/hint（hint_orchestrator + hint_position_update）

> restorer-consume-hint，2026-10-17。依赖线：unlock/hint-displayrequest（DisplayHintRequest
> 0x50B / EventInstance 0x58B 布局已判等）。
> 身份：B patched `tiny-glade.exe`（sha256 `cf048e27…6323`），Ghidra `--worker shared`
> 只读 fresh 反编译 + B 机器码（radare2 独立会话 `--worker consume-hint`，仅读取指令）。
> 证据目录：`crates/country-core/recovery/agent-team/work/consume-hint/`。

## 结论（三态）

- **完成**：
  - `hint_orchestrator` 0x14227fc30 —— **判等**（reviewer-consumers #5 判等）。完整控制流：
    事件收集（一次性耗尽 reader）→ id 稳定排序 → Linger/Toolbar/AppMode/Esc 门控 →
    当前提示实体查询 → 无实体 spawn（6 组件 + 条件 InteractionPrompt/HintProgressBar +
    display）→ 同 id 刷新 → 不同 id 标记删除 → 无请求冷却。三条文本模板（id 0x12 /
    id 2 / 默认）逐字节按 B 机器码还原。
  - `hint_position_update` 0x141c50510 —— **case 6/10 判等 + reviewer FIX-1/2 已修**：
    FIX-1 边距改为 `window.height/1000×{20,30}`（`as_vec2` 返回 XMM0=w/XMM1=h，入口用
    XMM1）；FIX-2 case 8 改为“投影四角取屏幕 **x 最大**角（严格大于，并列保留先者）并
    携带该角的 y”。NaN-aware 夹取与 `time_continuous_lerp(rate=10)` 判等。
    **case 7/8 的放置点 Y 未实现**（FIX-3：core typed preview 资源缺 `+0x20/+0x28`，
    以 0.0 占位）；placement 第二级跳转表未做。
- **FIX 吸收（reviewer-consumers）**：
  - FIX-1：`margin20/margin30` 用 `dims.y`；单测期望 200×100 → cursor `(14,20)`、
    clamp `(-103,-203)`。
  - FIX-2：case 8 取 max-x 角并携带其 y（新增判别性单测 `(197,3)`）。
  - FIX-3：7/8 标未实现（不冒充判等）。
  - FIX-4/5（同 crate 其它 hint 文件）：`hint_focused_edit.rs` 的
    `NotifyHintSystemShapeEditWasPerformed` 与 `hint_clutter_focus_edit.rs` 的
    `NotifyHintSystemClutterWasMoved` reader 改为 `.read().count() > 0`（一次性耗尽）。
    `hint_orchestrator` 的两个 reader 自始即用 `.read().count() > 0`（reviewer #5 已判等）。
- **登记（U）**：`ToolbarUiIsInFocus` country-core 类型未落（本地同名事件承接，注册接线 U）；
  `ClutterMeshes` 类型未落（system param 省略；函数体未读该参数）；7/8 的
  `PreviewCircle+0x20` / `PreviewRectangle+0x28` Y 值；placement 第二级跳转表
  0x142f14844（tag 0..5/7..9/11..）语义；remove 错误处理器（native `warn`）；
  id>18 的 Display 名称表越界。
- **未实现（登记，非本批）**：`hint_draw` 0x141c51a60、`render_hints` 0x141c52700
  （mei/toolbar DSL，依赖未落库）；`hint_position_update` 的 case 7/8 Y 与二级跳转表。
- **阻塞**：无。相机矩阵（`MainCamera::camera_matrices`/`read_matrix`）与
  `world_space_to_screen_space`/`Rectangle2d::as_points3` 均已落库。

## 1. SystemParam 证据（PDB 单态化名）

从 `global-symbols.jsonl.gz` 的 `FunctionSystemState<...>` drop glue 取到精确参数表：

**hint_orchestrator**
```
Res<AppMode>,
EventReader<DisplayHintRequest>,
EventReader<LingerHintRequest>,
Query<(Entity, &HintType,
       Option<&HintDeleteAfterCooldown>,
       Option<&HintMarkedForDeletion>)>,
EventReader<ToolbarUiIsInFocus>,
Res<EscMenuState>,
Commands
```
（query 的第 3/4 元素是**两个可选删除标记**，不是 display；display 只从请求侧读取。）

**hint_position_update**
```
Query<(&HintLayoutMemo, &mut HintSpring, &HintPlacement)>,
Res<CursorPositionSS>, Res<WindowSize>,
Res<PreviewCircle>, Res<PreviewRectangle>, Res<MainCamera>,
Res<PublicWalls>, Query<&Roof>, Res<DecoratorStorage>,
Res<ClutterMeshes>, Res<CachedDecoratorTransforms>,
Res<UiSignifierStream>, Res<GameTime>
```

## 2. hint_orchestrator 逐语句判等

fresh 反编译 `work/consume-hint/decomp-0x14227fc30.c` / 反汇编 `disasm-0.json`。

| 步骤 | 机器码 | Rust |
|---|---|---|
| 收集 | `from_iter<&DisplayHintRequest, EventIterator<..>>` 0x141c58da0 | `requests.read().collect::<Vec<&_>>()` |
| 排序 | `insertion_sort_shift_left` 0x1425e5b70 / `driftsort_main` 0x1425e6040，比较 `+0x49` 字节 | `sort_by(|a,b| a.id.cmp(&b.id))`（稳定） |
| Linger reader | 未读事件 → `*cursor=event_count; bVar12=1`；否则 `bVar12 = (AppMode==0xd) ? 1 : (AppMode==0x13 \| esc.is_open())` | `linger.read().count()>0` / Replay / Photomode |
| Toolbar reader | `toolbar_unread && bVar12==0` → return | `toolbar_present && !suppressed` → return |
| 当前实体 | Query 首匹配（无单例检查），`&HintType` 字节 + 两个 marker presence | `query.iter().next()` |
| 无实体门 | `sorted.is_empty()` → return；`bVar12 && id<0x12` → return | 同 |
| spawn | tuple `(HintLayoutMemo, HintSpring, HintAnimation, HintText, HintType, HintPlacement)` 0x142289260 | `commands.spawn((..., first.placement))` |
| InteractionPrompt | `*(int*)req == 10` | `placement.native_tag()==10` |
| display | `Impl$30::clone` + `insert<BindingHintDisplay>`（无条件） | `insert(first.display.clone())` |
| ProgressBar | `*(char*)(req+0x48) != 0` | `first.flags != 0` |
| 同 id | insert 新 placement；`*(int*)(req+0x18) != 0xc4` 时 insert display；remove 两个 marker（handler=warn） | 同 |
| 不同 id | `insert(HintMarkedForDeletion)` | 同 |
| 无请求 | 已有 marker/cooldown → return；否则 `insert(HintDeleteAfterCooldown(DAT_143038ae8[id]))` | 同（表长 19） |
| suppressed 且 existing_id<0x12 | `insert(HintMarkedForDeletion)` | 同 |

### 2.1 文本模板（新判等，disasm 0x1422800ea/0x142280408/0x142280490）

| id | 机器码 | 结果 |
|---|---|---|
| 0x12 | `lookup("Rollercoaster.Ride")`（0x1430388d8,len 0x12）+ pieces `["", " 🎢"]`（0x1430388f0）+ 1 arg | `format!("{} 🎢", …)`（emoji U+1F3A2） |
| 2 | `lookup("KeyBindings.Undo")`/`lookup("KeyBindings.Redo")`（0x143038890/0x1430388a0,len 16）+ pieces `["","/"]`（0x1430388b8）2 args | `"{}/{}"` |
| 其它 | pieces `["Hint."]`（0x143038918）+ `impl$14::fmt` 0x1422810c0 → `lookup(..)` → `to_uppercase()` | `"HINT.<NAME>"` |

`impl$14::fmt` 实为 `Formatter::write_str`，名称表 = 长度表 0x143038b38 +
相对偏移表 0x143038bd0，19 项：
`Deselect, FocusEdit, Undo, DisableSnapping, TerrainSteepness, TerrainFocus, Terrain,
RoofShape, UnlinkWindows, Clutter, CameraPan, CameraRotate, CameraZoom,
StairsCreateStart, StairsCreateFinish, StairsFocus, StairsContinue,
StairsDetachFromWall, RollercoasterPrompt`（`hint_type_display_name`，并实现
`Display for HintType`）。std `fmt::write` 在 piece 0 后追加 arg 0 → `"Hint."+name`。

### 2.2 门控语义（易读错点）

`bVar12` 是**抑制位**，不是“可显示位”：
- Linger 有事件 → 抑制（并非放宽 normal id；normal id<0x12 仍被 spawn 门拦截）；
- Toolbar 聚焦只在 `!bVar12` 时拦截；
- normal id（<0x12）在抑制时：无实体不 spawn，有实体标记删除；
- id 0x12（Rollercoaster）不受抑制门限制。

## 3. hint_position_update 逐语句重建

证据：`decomp-0x141c50510.c`、`disasm-1.txt`、case 体 `r2-3/4.txt`、
jump table 0x142f14830 / 0x142f14844、常量 read（1000.0/20.0/30.0/0.5/10.0）。

公共块（0x141c515e0）：
```
margin20 = window.height/1000*20 ; margin30 = window.height/1000*30
raw    = switch placement
delta  = memo.size - memo.position
target = raw + delta
upper  = (window - memo.position_2) - margin30
final  = clamp_nan_aware(target, margin30, upper) - delta
spring.pos = final
if !initialized { spring.target = final; initialized = true }
else { target.x = lerp(target.x, final.x, 10, dt); same y }
```
`maxps/minps` 的 NaN 选择与 Rust `f32::max/min`（返回非 NaN 操作数）一致；
`time_continuous_lerp`（0x140c904e0）= `a+(b-a)*(1-exp(-rate*dt))`，rate=10.0，
dt=`GameTime.delta`（+0x28）。

| case | 机器码 | Rust |
|---|---|---|
| 6 Cursor | `cursor + (2*margin20, 0)`（0x141c50acb） | 同 |
| 7 WorldPoint | `p = (circle.center.x, Y, circle.center.y) + eye_right*(radius+0.5)` → `world_space_to_screen_space(p,w,h,+0x80,+0x00)` → `+(margin20,margin20)`（0x141c50dc6） | 算术同；Y 占位 0.0 → **未实现（FIX-3）** |
| 8 Bounds | `Rectangle2d{axis:orientation,center,size:dimensions}.as_points3(Y)` 四角投影 → **屏幕 x 最大角**（`ucomiss/seta` 严格大于，并列保留先者）并携带其 y → `+(margin20,margin20)`（0x141c50af9） | x 选取/夹取同（FIX-2）；Y 占位 0.0 → **未实现（FIX-3）** |
| 10 ScreenCenter | `(width*0.5, height*0.5)`（`window * {0.5,0.5}`，0x141c50ae0） | `dims*0.5` |
| 其它 | tag 11 → `unimplemented!()`；其它走第二级跳转表 0x142f14844 | `continue`（U，不伪造） |

相机接线：`MainCamera::camera_matrices().eye_right()`（inverse_view@+0xc0）与
`MainCamera::read_matrix(0x80)/read_matrix(0x00)`（world_space_to_screen_space 的两级矩阵）；
`Rectangle2d::as_points3`、`world_space_to_screen_space` 直接用已落库共享实现。

## 4. 代码变更

- `crates/systems/onboarding/src/hint_orchestrator.rs`
  - `HintLayoutMemo` 改为 native 0x18B：`position@+0` / `size@+8` / `position_2@+0x10`
    （原 16B `anchor/extent` 与 query 步长 0x18 不符）。
  - 新增 `ToolbarUiIsInFocus`（本地 unit 事件，承接缺失的 country-core 类型）。
  - 新增 `hint_type_display_name` + `Display for HintType`（0x1422810c0 名称表）。
  - 新增 `hint_text(id)`（三条模板）。
  - `hint_orchestrator` 由诚实空体 → 完整实现。
  - 新增 9 个单测（排序/文本/门控/同 id/不同 id/冷却/菜单/Rollercoaster/工具栏）。
- `crates/systems/onboarding/src/hint_display_system.rs`
  - `hint_position_update` 由诚实空体 → case 6/7/8/10 实现 + 5 个单测；
    FIX-1 边距用 `dims.y`；FIX-2 case 8 取屏幕 max-x 角并携带其 y；
    7/8 的 Y 以 0.0 占位并在文档/报告标未实现；新增 `HINT_SPRING_RATE=10.0`。
- `crates/systems/onboarding/src/lib.rs`
  - `configure` 注册本地 `Events<ToolbarUiIsInFocus>`（接线 U 注记）；模块头注记更新。
- `crates/systems/onboarding/src/hint_focused_edit.rs`（reviewer FIX-4）
  - `NotifyHintSystemShapeEditWasPerformed` reader 改 `.read().count() > 0`（一次性耗尽）。
- `crates/systems/onboarding/src/hint_clutter_focus_edit.rs`（reviewer FIX-5，低）
  - `NotifyHintSystemClutterWasMoved` reader 改 `.read().count() > 0`。
- 未改 country-core / utils / 其它模块（红线：shared 只读；FIX-3 的 core 字段未落）。

## 5. 编译/测试（现场）

- `cargo check -p system-onboarding`：**通过**（0 error；仅 country-core 既有 warning）。
- `cargo test -p system-onboarding`：**lib 29/29 + numeric_assertions 2/2 通过**。

## 6. 留给 reviewer / 上游（U 清单）

1. `ToolbarUiIsInFocus`：原生 `country_core::resources::toolbar::ToolbarUiIsInFocus`
   （toolbar plugin 注册）；本模块本地事件仅为可编译承接，生产者接线待 toolbar 落库。
2. `ClutterMeshes`：country-core `resources::clutter_meshes` 未落；该 system param 省略
   （函数体未读取），类型落库后应补回参数。
3. `PreviewCircle+0x20` / `PreviewRectangle+0x28` 放置点 Y：country-core typed preview
   资源无此字段（`height` 是 calc_shape_height 的 +0x40 计算高度）；7/8 以 0.0 占位。
4. placement 第二级跳转表 0x142f14844（tag 0..5/7..9/11..；tag 11 原生
   `unimplemented!()`）：本实现跳过未识别 tag，未伪造几何。
5. remove 错误处理器：native `EntityCommands::remove` 用 `warn`；bevy 0.16 默认
   handler 由 feature 决定，未显式指定。
6. id>18 时 `impl$14` 名称表原生越界读；本实现返回空串（合法写面仅 0..=18）。
