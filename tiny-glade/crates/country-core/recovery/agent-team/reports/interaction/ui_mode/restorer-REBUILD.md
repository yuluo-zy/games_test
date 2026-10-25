# ui_mode 12 函数重建 — restorer

日期: 2026-10-17
范围: ui_mode/widget_params/signifier/brush_preview（只以 fresh 反编译为准）
证据: `reports/interaction/ui_mode/evidence/results/restorer-ui-mode-fresh1..16.json`
（ghidra shared，B patched `tiny-glade.exe`；跳表/缺失块用 B `read_memory` + capstone 解码）。

## 逐函数结果

| # | VA | 函数 | verdict | 重建要点 / 分歧关闭 |
|---|---|---|---|---|
| 1 | 0x1409a8e10 | `WidgetParams::new` | 完成 | 机器码确认：`[RDX..+0x20]`→`+0x88..0xa8`(volume 32B)、`[R8..+0x10]`→`+0x78..0x88`(16B)、默认 u32 组 {0,8,0,1,0,2,3,3}、`word@0xa8=0x101`。Rust 改签名 `new(volume, extra_78)`，occlude=true、flags=0x01；`Default` 传默认值。 |
| 2 | 0x1409a8f10 | `no_history_recording` | 已判等（保留） | `byte@0xa9 &= 0xfe` 后 memcpy 0xb0 返回：与 `mut self -> Self` 一致。 |
| 3 | 0x1409a8f40 | `allow_negative_distance` | 已判等（保留） | `arg<<2 | byte&0xfb` 后返回：与带 bool 参数的 builder 一致。 |
| 4 | 0x1409a9130 | `UiFocus::as_mei` | 完成 | 机器码 `xor eax,eax; cmp [rcx],2; setz al; mov rdx,[rcx+8]; ret` = `Option<u64>`（判别 RAX/载荷 RDX）。UiFocus 改为 16B `{tag@0,pad@4,payload@8}`；`as_mei -> Option<u64>`。is_none/is_toolbar/is_mei 判别逻辑不变（&self）。 |
| 5 | 0x140a9bd40 | `Slider3dState::project_cursor_onto_line` | 完成 | 反汇编：self 前 24B = Ray{origin,direction}（第二向量是方向）；屏幕尺寸 `[u32;2]` 现场构造 ProportionalScreenSpace；`mouse_ss_to_cs`→`project_onto_ray`，range 常量 `[-1e10,1e10]`（0x142ada620 位型）；相机矩阵单指针：second@0、first@0x80。Rust 改签名 `(&self, cursor: Vec2, screen_size: [u32;2], &Slider3dMatrices)`；删除世界空间线段投影/clamp 近似。 |
| 6 | 0x140a69c30 | `signifier_animation` | 完成（GAP 登记） | 机器码确认：组件 0x1cB，门 `byte@8==0`；`scaled=eased@0x18·value@0xc`；`x=byte@9==0?value:scaled`；写 `Transform.scale=(x,scaled,x)`。Query 改为 PDB 签名 `(&SignifierAnimation,&mut Transform,Option<&SignifierId>,Option<&SignifierParentToVolumeOrigin>)` + `Res<UiState>`。GAP：kind switch（`*(*[UiState+0x20]+0x98)`）写 translation（case0/6/1 已证，8..0x10 反编译损坏）——UiState 未落库 signifier 配置表。 |
| 7 | 0x140b2cf90 | `UiSignifierStream::write` | 完成 | 0x140b2cf90 为 5B thunk → AppendOnlyVec::push 0x14146c880：`LOCK xadd` 预留槽(@+0x168)、按桶分配、写 0x28B、自旋等 `committed(@+0x160)==槽`、提交槽+1。**无回绕/无容量丢弃**。Rust 从 8 槽环形缓冲改为 append-only Vec + committed/next，满时自旋不丢；`take_all` 复刻 mem::take(0x170B+memset0)。 |
| 8 | 0x140b2cfa0 | `signifier_create` | 完成（GAP 登记） | PDB 系统签名确认（ResMut<UiState>, AssetMesh/ShaderLibrary, Commands, Query<&SignifierId,Without<Marked>>, ResMut<stream>）。实现：Query 建 HashSet 去重 → `UiState+0x18 != 0` 门 → `take_all`（mem::take）→ 逐条 hash 去重 → spawn+SignifierId。GAP：原生经 UiState 配置表(indexmap stride 0x28)+vtable 工厂及 mesh/shader 库生成实体；crate UiState 未落库该表、AssetShaderLibrary 未 derive Resource。 |
| 9 | 0x140929160 | `signfier_mark_for_delete` | 完成（GAP 登记） | PDB 签名确认（Res<UiState>, Query<(Entity,&SignifierId,Option<&SignifierMinimumLifespan>)>, Commands）。机器码：`UiState+0x18!=0` 时 hash `(WidgetId,Option<&str>)` 建表；实体 hash **不在表**且（无 lifespan 或 `min_lifespan@+4 <= t@+0`）→ insert 标记。新增 `SignifierMinimumLifespan` 组件。GAP：UiState 配置表未落库 → live 集恒空（解锁：暴露 hash 迭代面）。 |
| 10 | 0x140865050 | `brush_preview` | 完成 | PDB 签名确认。机器码：`DebugGui.hide_brush_previews(+0xfa)` → 全部 remove Transform 并返回；`OverwriteBrushPreview`(0x20B: flag@8、pos@0xc、kind@0x18) 按 kind 匹配写/插 translation；逐实体 kind switch 写 scale。Rust 从 Events 化回写实体；`BrushPreview` 改为 1B kind。GAP：原生插入 0x68B bundle，crate 只落 Transform。 |
| 11 | — | `brush_size`（内联） | 完成 | 7 路跳表（0x142a966e0）解码：kind 4=water×0.5、5=garden×0.5×0.75、6=(m×1.68×0.5, min(1,max(0.5,m)), 同x)、7/8=m、9=((inverse_lerp(0.3,3.0,m)×3+1)×0.5, clamp, 同x)、10=m×3×0.5、其它=1.0；m=EraserSizeMultiplier。原 AppMode→固定 7/15/17 映射删除。 |
| 12 | — | （同 10/11 覆盖） | — | 任务清单第 12 项与 brush_size 内联同源，无额外符号。 |

## 已判等未动
sounds / occlude_by_world / UiSounds::empty / clearing_canvas / UiFocus::is_mei/is_toolbar（判别逻辑保持，仅类型改 16B 结构）/ despawn_marked / send_mode_just_changed_event / check_if_can_switch×2 / AppMode 族 / signifier_animation_t（只随组件布局同步字段，不改控制流）/ mode_manager。

## 登记（解锁条件）
1. **UiState signifier 配置表**（原生 `*[UiState+0x20]`，0x28B 条目 `(WidgetId,Option<&str>,…,Arc,vtable)`）未落库 → `signifier_create` 的生成工厂、`signfier_mark_for_delete` 的 live 集合、`signifier_animation` 的 kind switch/translation 三处 GAP。解锁：UiState 暴露该表的只读迭代/hash 面（新文件或 ui_state 域 owner 补）。
2. **AssetShaderLibrary 非 Resource** → 未纳入 signifier_create 系统参数；解锁：asset 域补 `#[derive(Resource)]`。
3. **signifier_animation switch 8..0x10**：Ghidra 反编译损坏（swi/halt_baddata）；case 0/6/1 已证。解锁：ReVa High P-code 或逐字节 jumptable 还原。
4. **brush_preview 插入 bundle**：原生 0x68B（Transform + 其它组件）未拆；解锁：render/组件域补 bundle 类型。
5. **Cursor/矩阵宿主类型**：`Slider3dMatrices` 的 +0x40..0x80 未读区；已按单指针块建模，宿主相机类型未落库。

## 机器验证
- `cargo check -p country-core --lib`：**通过（0 error）**。
- 期间 `src/systems/time_of_day.rs`、`src/startup/run_game.rs`、`src/render/directional_light.rs`
  （19:08 其他 agent 并发改写）曾短暂产生 6 个外部错误，随后由其修复；本域未代改。
- 单测（4 个过滤器，全部通过）：`signifiers` **7 passed**、`brush_preview` **3 passed**、
  `slider3d` **2 passed**、`widget_params` **4 passed**；合计 **16 passed / 0 failed**。
  本域新增测试 12 个（animation 2、signifier_create 1、brush_preview 3、slider3d 2、
  widget_params 4），余为同过滤器内既有测试。
