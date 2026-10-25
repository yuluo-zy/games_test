# REBUILD4 — walls 批 4（26 项重做）+ 批 3 剩余 24（restorer-walls4，2026-10-17）

> 协议：`planning/2026-10-16-logic-verify-protocol.md`（分歧检查表 8 条）、
> `planning/2026-10-17-recovery-sop.md`。方法：**只以本轮 fresh 反编译/反汇编为准**。
> fresh 证据落盘 `work/restorer-walls4/decomp/`：
> - `batch1.result.json` + 逐函数 `.c`（9 个主函数，fields=signature,decompiled_code,callees）；
> - `disasm1.result.json`（has_roof / roof_component / get_shape_params 反汇编）；
> - `createrect.disasm.json`（create_rectangle 849 条指令）、`uicircle.disasm.json`、
>   `uirect.disasm.json`、`sig.disasm.result.json`（circle/rectangle_signifier）、
>   `fmtimpl.result.json` + `fmtimpl.disasm.json`（Display impl）、
>   `curves.disasm.json`（曲线闭包）、`box.result.json`（trigger volume 反汇编）、
>   `resample.result.json`、`fold.result.json`/`fold.disasm.json`、`vtable.result.json`、
>   `strings.result.json`/`fmtnames.result.json`/`meshnames.result.json`（常量与字符串）、
>   `batch2.result.json`（17 个闭包）；
> - 常量经 `read_memory` 核实：0.1 @0x142925984、0.5 @0x142925870、
>   0.2 @0x1429cd068、vtable 0x142c82240 / 0x142c6e340。

## 0. 概览

| 类别 | 数量 |
|---|---|
| 本轮 closed（重建/补齐，逐语句或结构判等） | 9 |
| 曲线闭包落地 | 3 |
| 登记（跨域/缺类型/硬 seam） | 17 |
| 批 3 剩余 24 复核 | 0 可落地（类型仍缺） |

`cargo check -p country-core --lib` exit 0；`cargo test -p country-core --lib wall`
**142 passed / 0 failed**；全量 lib **1968 passed / 4 failed**（4 项均在 paths 域，
他组并发 WIP，非 walls；详见 §6）。

## 1. closed（本轮重建/补齐）

| VA | 函数 | Rust 位置 | verdict / 本轮闭合点 |
|---|---|---|---|
| 0x140a92100 | PublicWalls::has_roof | public_walls.rs | **判等**：双表探测。① `failures`（+0x60/+0x68/+0x78）命中 → `XOR ESI,ESI`；② `roofs`（+0x30/+0x38/+0x48）命中 → `CMP dword[entry+0xc],0; SETNZ AL`（Entity generation 半字，即 `Option::is_some` niche）；`AND SIL,AL`。实现 `!failures.contains_key && roofs.get.is_some_and(gen!=0)`，原 `contains_key` 单表 + 无 gen 校验已删除 |
| 0x140a92530 | PublicWalls::get_shape_params | public_walls.rs | **判等**：内联 `get`（`count<2 || !(length>0)`，UCOMISS `JBE` 含 NaN → `panic`）→ 记录 `+0x260` 的 28 B ShapeParams；`tag<2` 拷贝、`tag≥2` 写规范 None（tag=2）。实现 `self.get(wall)?.shape()` + `is_premade`。原无 body |
| 0x140a923a0 | PublicWalls::roof_component | public_walls.rs | **判等-注意（seam）**：roofs 探针 → entity generation≠0 → InnerWalls 密集屋顶记录链（0x14 稀疏记录 generation/位集/0x48 chunk/`~slot*0x30`/0x58 组件）→ `Option<&Roof>`，全失败返回 0。crate 无密集表，第三参以 `resolve: FnOnce(Entity)->Option<&Roof>` seam 承接（`Query<&Roof>` 做同一 index/generation 校验）。原无 body |
| 0x140a93790 | create_rectangle | shapes/rectangle/create_rectangle.rs | **判等**：补 ① `previous_wall`（事件 +8）进入 `get_building_and_roof_colors`；② 旧屋顶继承：判别 1 + previous + roofs 命中 + gen≠0 + dense 链命中 → `style=comp+0x3d`、`pitch=comp+0x30`，`UCOMISS 0.1,pitch; JC skip` ⇒ 仅有序 `pitch<=0.1` 覆盖 height_01；③ 调用链 `new_wall_w_params→update_elevation→get_building_and_roof_colors→update_style→update_wall_state_enclosure`；④ `ev_new_wall` + **History::add #1** `EditType::Roof`(tag 8)/`RoofHistoryEdit` create(tag 0)，presence 逐字节：`[0]=roof色,[0x18]=0.5,[0x1c]=height,[0x20]=ridge,[0x24]=0.2,[0x28]=dir,[0x29]=style`；`[0x30]=wall id`（0x140a944d1..0x94535） |
| 0x140a94880 | get_building_and_roof_colors | recovered_building/wall_authoring.rs（旧代理） | **判等修正（panic-vs-Err）**：`validate_public_wall_read` 失败改为 `panic!("assertion failed: w.simplified_curve_2d.is_valid()")`，删除 `?`→Err；roof_appearance 复制改**无条件** `[0x40..0x4c]`（flag 是外观[0]）。规范实现另在 `recovered_building/building_colors.rs`（已存在，未重复） |
| 0x14165cd40 | circle_signifier | shapes/circle/ui_create_circle.rs | **判等**：动态 `format!("cursor_circle_{}{}", Circle2d Display, bool Display)`；40 B 记录 `{name_len,name_ptr,name_len,&env,vtable}` 以 `SignifierFactory` 写流；env = `{Arc 头 1/1, Circle2d@0x10, flag@0x20}` 0x28 B。Display 0x14165bd40 = `debug_struct_field3_finish`（center/radius/rotation，Vec2 Debug） |
| 0x14155e460 | rectangle_signifier | shapes/rectangle/ui_create_rectangle.rs | **判等**：`format!("cursor_rectangle_{}{}", Rectangle2d Display, bool)`；env 0x30 B `{1,1,Rectangle2d@0x10,flag@0x28}`；Display 0x141559ce0 = `debug_struct_field4_finish`，字段序 orientation/width/length/center |
| 0x140ad9f10 | skewed_box_trigger_vol | shapes/rectangle/ui_edit_rectangle_orientation.rs | **判等**：两输入各 4×Vec3（0x30 B）→ 8 顶点（arg3 先、arg2 后）+ 固定 12 三角（36×u32）索引，`base+n` 以 32-bit wrapping。原无 body |
| 0x140ada0e0 | box_trigger_volume | 同上 | **判等**：`indices.chunks(3)` → `Triangle::new`，闭包 0x1414784a0 = 顶点 `<24` 边界 panic + 四行 `Vec4` 仿射（`x*r0+y*r1+z*r2+r3` = `Mat4::transform_point3`）。原无 body |

## 2. 曲线闭包落地（3）

| VA | 闭包 | 实现 | 依据 |
|---|---|---|---|
| 0x14165b330（同址折叠 0x1409a3620） | `call_once<circle_signifier::closure_env$0,()>` | `circle_signifier_curve = Circle2d::as_curve3(0.2, 0.0)` | XMM2=0x1429cd068=0.2、XMM3=0 |
| 0x141559a10 | `call_once<rectangle_signifier::closure_env$0,()>` | `rectangle_signifier_curve = as_curve2().crashy_resample_remap_per_segment(0.5, xvy(p,0))` | XMM2=0x142925870=0.5；callee 0x140c9cde0=xvy |
| （同上，返回类型） | `Curve<Vec3,CurveCoordSemantic3D>` | 用 `utils::curve::Curve3` | 符号 0x14146b5f0 `<Vec2,2D,Vec3,3D>` |

## 3. 登记（20 项：17 闭包中余 14 + create_rectangle 第二 History + 签名 vtable + 批 3）

| 项 | 阻塞 / 解锁条件 |
|---|---|
| 14 个 signifier/曲线 build 闭包：0x14165cf80（circle build）、0x14155e6b0（rectangle build）、0x140a65510（edit_line）、0x140b35650（edit_arrow）、0x1409ae3d0（ui_rotate_circle）、0x1408de7e0（signifier_edit_dims）、0x1408df6c0（shape_placement c1，Vec3 拷贝+build）、0x1408dfa00（c2）、0x14155f620（edit_dims c0：`flat_arrow`/x0y/rotation_from_normal+build）、0x14155f9b0（c1）、0x14083df50（edit_orientation）、0x141660600（ui_edit_circle c2）、0x141660b30（show_full_circle c0）、0x140957390（adjust_wall_height_curve c0，Vec3 拷贝+build） | 均为 `SignifierEntityBuilder::build` + `DrawTopology`/`DashedLineLength`/`MeshFollowTerrain` 等 ECS 命令入队（render-host）。解锁：`SignifierEntityBuilder::build` 落地事件/组件（DrawTopology、DashedLineLength、MeshFollowTerrain、UiPass 等）后按各闭包常量接线。circle build 的 mesh 选择已取证：flag=0 → `"vertex_color_shader"`(19B)，flag=1 → `"dashed_line"`(11B)（0x142c822a0/+0x13） |
| circle/rectangle signifier vtable（0x142c82240 size=0x14 / 0x142c6e340 size=0x1c，call_once@+0x28） | env 结构已按 40/48 B 布局定义（Arc 头 + Circle2d/Rectangle2d + flag），记录中的 `&env` 槽在本 crate 为 **layout-only**（流只拷贝 5 qword；consumer 带 vtable 调用属未落地 seam），`marker` 暂 0、不引入帧内泄漏。解锁：boxed `FnOnce` 的 call_once ABI + 上述 build 闭包落地后写入真实 vtable 并由 consumer 接管 env |
| create_rectangle **History::add #2**（0x140a94617，`EditType::Color` tag 9） | gate = `settings.gates != 0`（GladeColorCustomization +0x38）；payload `{+0x08=4, +0x10=wall id, +0x18=0, +0x1c=settings.gates}`（48 B 分配 = 16 Arc 头 + 32 payload）。crate 无 Color history 事件所有者；解锁：history 域提供 `RecordColorHistory`/ColorEdit in-memory 载荷映射 |
| create_rectangle elevation seam | 原生 update_elevation 读事件 +0x2c（0x140a93c39 区）；`CreateRectangleCmd` 无该字段，暂用 SHAPE_ELEVATION=4.0 |
| 批 3 剩余 24（REBUILD3 §2） | 复核后仍缺类型：`PreferredNodeAlignment`（stride 0x20）仍无（stairs 侧已新增 `ObservedAlignKind`，仅半解锁）；collision `(OffsetColliderTransform,RaycastMesh)` 仍无；`AutoClutterObject` 无；`CameraMatrices::aspect_ratio` 无；`UiState80` widget/实体链仅本地代理；`history_finish_recording` 属 history 域。均按 REBUILD3 解锁条件保持登记，未冒顶 |

## 4. 对 LOGIC4 的 fresh 校正/补充

1. **0x140ad9f10 索引不是「每项 0x10B 双索引」**：disasm 为 out+0x60..0xec 的 36 个 dword（12 三角×3），每项 4 B。
2. **create_rectangle History::add ×2 = Roof(tag8) + 条件 Color(tag9)**；Wall edit 在 `ev_new_wall` 0x140a83300 内部再调一次 `History::add`（callees 含 0x140a84ec0），故原生共 3 次 add，本实现只发 Wall/Roof 两个已建模 owner。
3. **RoofHistory presence[0] = roof 色**（0x140a944d5 从 `[RBP+0x1f8]` 取，即颜色输出 +0x04），不是事件字段。
4. **get_building_and_roof_colors 的 roof_appearance 为无条件复制**（flag 字节是外观[0]），旧代理的 `if roof[0x40]!=0` 门删除。

## 5. 改动文件（全部 walls 域及其色彩策略）

`src/resources/walls/public_walls.rs`、
`src/systems/wall/shapes/rectangle/{create_rectangle,ui_create_rectangle,ui_edit_rectangle_orientation}.rs`、
`src/systems/wall/shapes/circle/ui_create_circle.rs`、
`src/systems/wall/shapes/mod.rs`（CreateRectangleCmd +previous_wall）、
`src/recovered_building/{building_colors,wall_authoring}.rs`（色彩策略）。

只读 `--worker shared`，未改数据库；未触碰他域业务。工作期间其它 restorer 并发修改
`create_rectangle.rs`/`ui_create_circle.rs` 等，已保留其色彩链与 `update_style` 成果并叠加本轮差异。

## 6. 测试

新增 11 条（walls 域）：
- `has_roof` 双表门（失败表压制、gen 判据）（1）；
- `get_shape_params` 28 B 记录/自由形状 None/缺 id None（1）；
- `roof_component` seam 三路径（1）；
- create_rectangle 屋顶命令 + `RoofHistoryEdit` tag0/wall id/roof 色（1，并入既有链路测试）；
- circle/rectangle signifier 原生格式化串与曲线（4）；
- `skewed_box_trigger_vol` 顶点序/索引/32-bit wrap（1）、`box_trigger_volume` 变换/越界 panic（2）。

`cargo check -p country-core --lib` exit 0；`cargo test -p country-core --lib wall` **142/142**；
全量 lib **1968 passed / 4 failed**——4 个失败全在
`systems::paths::world_raster_tests`（他组 restorer 当轮并发改动，SOP §6 归属记录，
不代改）；本轮改动落盘后的第一次全量运行（paths 该批改动之前）为 **1966/1966**。
