# LOGIC 判等报告 — systems/garden（logic-verifier-eco-b，2026-10-16 fresh 反编译）

> 依据：`2026-10-16-logic-verify-protocol.md`。只以 fresh 反编译为准（本会话
> 0x140ae4620/0x141695e10/0x140b6d290/0x1415f87c0 均重新 get_functions+disassemble_function，
> 未采信任何先前 reviewer/restorer 描述）。
> 审计范围：`src/systems/garden/{mod,draw_garden,record_garden_history}.rs` +
> `src/startup/startup_garden_brush.rs` + `src/resources/garden_mask.rs` 的已实现业务函数。

## 函数表

| VA | 函数 | Rust 位置 | verdict | 分歧点 |
|---|---|---|---|---|
| 0x140ae4620 | `draw_garden` | draw_garden.rs:39 | **判等** | 无 |
| 0x141695e10 | `record_garden_history` | record_garden_history.rs:14 | **判等（本体；分歧注记）** | 见 §2 |
| 0x140b6d290 | `impl$0::build`（garden 插件） | mod.rs:108 | 胶水-非业务 | 等价注册 |
| — | draw_garden 3 闭包（on_draw/on_erase/on_record） | draw_garden.rs:134-151 | 胶水-非业务 | 事件写 |
| 0x1415f87c0 | `startup_garden_brush` | startup_garden_brush.rs:29 | **未实现-需重做（部分）** | 见 §3 |
| — | `GardenMaskSsbo`（garden_mask.rs） | garden_mask.rs:11 | 无业务函数 | storage element 胶水（BUFFER_LEN 0x8000） |

## §1 draw_garden（0x140ae4620）— 判等

机器码逐条核对（disasm 316 条）：

| 维度 | 原生 | Rust | 判定 |
|---|---|---|---|
| 模式分发 | `R9-6<0xf → R9-5`；`==0xc`→Garden(17)、`==4`→Eraser(9)、否则 return；Eraser 层 `[RCX+4]∈{0,5}` | `AppMode::Garden`/`Eraser(All\|Garden)`/`_=>return` | 判等 |
| 橡皮半径 | `inverse_lerp(0.3@0x1429258d8, 2.0@0x14292e6f0, brush)` → `t*4.0@0x142934694 + (1-t@0x142925904)`；`\|\|≥0x7f800000`→panic | `t=(b-0.3)/(2.0-0.3)`；`t*4+(1-t)`；`assert is_finite` | 判等 |
| 花丛半径 | `[brush.garden+8]` 直取；`\|x\|>0x7f7fffff`→panic | `brush.1.garden`；`assert is_finite` | 判等 |
| UI 注册 | `clearing_canvas_w_layer_and_sound`：eraser 串 0x142ae2e80/len6/arg9/layer3；garden 串 0x142ae2ea0/len6/arg5/layer3 | `(name, argument_79, 3)` 5/9 | 判等 |
| 界内 | `is_shape_inside(Circle2d(center, 0.4@0x142925970 * radius * 0.8@0x142a16cb8))`；center=光标 x@+4/z@+0xc（tag&1 校验） | `!draw \|\| is_shape_inside(Circle2d(center,(0.4*radius)*0.8))` | 判等 |
| 出界事件 | !allowed→写 DisplayOutOfBorder；且 `native_tag>=2 && id==widget && native_phase@0x19 != 2`→写 FeedbackInputOutOfBorder | `if !allowed { display_outside.write; if phase != 2 { feedback.write } }`（dragged()=native_phase if tag≥2&&id 匹配 else 2） | 判等 |
| 状态机 | r=min(phase-2,3)（inactive→0）；r∈{1,2}：state∈{1,2}→state=2,cmd=2；state∉{1,2}→state=allowed,cmd=draw；r∈{0,3}：r==0 或 state-1≥2→state=0,return，否则 state=3,cmd=3 | 逐条对应（wrapping 减） | 判等 |
| 引脚圆 | `Circle2d::new(center, radius*0.5@0x142925870)`；height=光标 y@+8+1.5@0x1429258d0（tag&1=0 时 1.5） | `Circle2d::new(center, radius*0.5), height: position.y + 1.5` | 判等 |
| draw_raster | `0x140ab59e0`（GardenRasterStagingSsbo）；cmd=0/1/2/3、strength=0x3f000000=0.5、radius、draw 字节 | `draw_raster::<GardenRasterStagingSsbo,C>(RasterStrokeCmd{..,strength:0.5,radius,draw}, pos, 5, ..)` | 判等 |

slot：record 侧 hardcode 偏移 0x1678 = 槽 5×0x470+0x48，draw 侧与 record 同一槽链闭合。

## §2 record_garden_history（0x141695e10）— 判等（本体）+ 分歧注记

机器码核对：
- 事件读取：EventReader 计算未读计数并推进 cursor（drain）；未读=0 → return。✓ `events.read().last().is_none()` 等价。
- 记录：hardcode 偏移 0x1678..0x169c = 槽 5（resampled Vec @+0x48/+0x50/+0x58、brush_size@+0x60、mode@+0x64、param_a@+0x68、param_b@+0x6c）；clear 语义 = take_history_stroke。✓
- 包装：Arc<Vec<Vec3>>（header strong=weak=1 + memcpy count×12）+ Arc<RasterStroke>（0x30 分配，{strong,weak,flags,strength,points,count,radius,draw}）+ EditType tag=6（=Garden）+ `History::add` 0x140a84ec0。✓
- **分歧注记**：Rust `if let Err → failures.write(HistoryRecordingFailed{edit,error})` 分支在原生**不存在**。原生 History::add 失败走 `panic_fmt`（"Tried writing to History when th..."，无 Err 返回），record_garden_history 调用后直接 epilogue（0x14169601a→601f，无测试 RAX/无写失败事件）。Rust 的 failures 分支为共享 history 记录族的自造防御分支（roof/wall/ui_clutter 同样模式），非原生分支。历史写入失败在原生是 panic，在 Rust 被建模为可恢复 Err+事件。

## §3 startup_garden_brush（0x1415f87c0）— 未实现-需重做（部分）

原生按顺序执行 **2 次**：
1. `load_json_as_mesh<str>`（garden_stone.json / lowpoly_flower.json）
2. `load_mesh_into_library`（0x140b1aca0）
3. `load_shader_into_library<str>`（0x140b3fe40）
4. `spawn_empty`（0x140d146a0）+ push 0x78B 命令：第 1 个 `insert<DrawableMeshBundle>`；第 2 个 `insert<(DrawableMeshBundle, DontCastShadow)>`（bevy command_queue::push::closure_env）

Rust `startup_garden_brush` 仅实现 1-2（网格加载注册），缺 **3-4（2×shader 加载 + 2×spawn/insert 命令）**。RESTORE 已登记"shader 两个 + 2 个 spawn 记录依赖渲染管线，待接线"。严格判等下缺分支 → 未实现-需重做（部分），待渲染管线恢复后续接。

## 遗留/边界
- `systems/garden.rs::impl$0::build`：Bevy App 注册胶水（in_set/GameSystemSet、add_systems、insert_resource<GardenRaster>、register_event×3）。mod.rs build() 等价注册（事件超集 + draw→record 序），不计业务。
- garden_mask.rs 无 inventory 原函数；`GardenMaskSsbo` 是 SSBO storage element 胶水，"更新"属 read_terrain_data 的 write_storage_buffer 消费，不在本文件。
