# DOWNSTREAM-ANCHOR — walls 下游结构锚定与回接

日期：2026-10-17
恢复 crate：`reconstruction/country-core`（`-p country-core --lib`）
证据：B patched EXE（SHA256 cf048e27…）fresh Ghidra 反编译 + 原字节（radare2 RNE oracle）。
前置报告 `restorer-walls5-REBUILD5.md` / `restorer-walls4-REBUILD4.md` 在本次工作区不存在，
本报告只使用可复现的 fresh 证据，未引用缺失文件。

## 1. 结构锚定结果

| 结构 | 落点 | 证据 | 结果 |
|---|---|---|---|
| `PreferredNodeAlignment` | `src/resources/stairs/stairs_assembly_params.rs` | `find_alignment_preferences` @0x14080beb0（3×24B 连续写）、`find_best_neighbor_alignments` @0x14080c360（`plVar8+3`=24B 步进） | **24B**：`tag u8@0, pad@1, flag bool@2, Vec2@4, Vec2@12, f32@20` |
| `AlignKind` | 同上 | `grow_one<indexmap::Bucket<tuple<u32,u32>,AlignKind>>` @0x140562ab0（stride 0x28=8 hash+8 key+24 value）；`outgoing_dir` @0x140959f50、`position_offset` @0x140959f00 原字节 | **24B**：`tag u8@0, flag_a@1, flag_b@2, Vec2@4, Vec2@12, f32@20`；两个方法按原字节重写 |
| `AutoClutterObject` | `reconstruction/clutter/src/autoclutter/mod.rs`（源树；无 Cargo.toml） | `from_iter<u64,&AutoClutterObject>` @0x142612aa0：`imul rax,r15,0x68`、`mov rdx,[rdi+rax+0x40]` | **0x68B，key u64@0x40**；其余字段 U |
| `UiState` | `src/systems/ui/resources/ui_state.rs` | `insert_resource<UiState>` @0x1415398d0 `mov r8d,0x98` + memcpy；`add_with_id` @0x14092e3e0：`lock cmpxchg [rdi+0x60]`、`lea r15,[rdi+0x68]`；`UiWidgetContext::*` @0x14092e5f0/e620/e640/e680 | **0x98B**：hover@0x18、click@0x19、intersection 7×u64@0x28（widget id@0x38）、RwLock 锁字@0x60、HashMap@0x68 |
| `UiWallHeight` | `src/systems/wall/adjust_wall_height/adjust_wall_ui_mode.rs` | `widget` @0x1409566d0、`signifier` @0x140956cd0 原字节 | 观测 `u64@0, bool@8, bool@9`；尾部 U（载体 16B） |
| `MorphPremadeShape` / `PremadeShape` | `src/resources/history/wall_compound_edit.rs` | starting-builds 历史 JSON（MorphPremadeShape 80+85 条，`wall_index/before/after`；Circle{center,radius}、Rectangle{center,length,width,orientation}） | serde schema 复原；运行时内存布局 U |
| Color history owner | `src/systems/wall/shapes/rectangle/create_rectangle.rs` | create_rectangle @0x140a93790：第一次 `History::add`（ev_new_wall）+ 第二次 gated on `*(int*)(..+0x38)!=0` 的 Color add | owner 锚定 = **create_rectangle**；envelope tag/records U |

## 2. 回接（已落地）

- `edit_rectangle` / `ui_create_rectangle`：`PreferredNodeAlignment`+`AlignKind` 类型已在 stairs 域落地并有布局/方法单测；这两个 rectangle 系统本身不持有该类型（B dependencies 中无该边），**不伪造调用**。
- `wall_roof_hashmap`：fresh 复核 dependencies 仅有 ICF 别名集合 `HashMap::insert<u64, ref<AutoClutterObject>>`（与 `insert<WallId,Entity>` 同址 `alias_set_at_0x140eec970`），**不是真实依赖**；已在文件头登记，AutoClutterObject 布局落在 clutter 源域。
- `UiState`：真实资源结构、锁/map 偏移、`add_with_id`/`widget`/`active_widget_state`/`hovered`/`clicked`/`is_active`/`intersecting`/`clearing_canvas_w_layer_and_sound` 全部按 fresh 证据实现（去掉 todo!）。
- `UiWallHeight`：字段布局落地；`ui_adjust_wall_height` 在 premade-shape 分支写 `raw_id`/`active`（原 `new` 内联点）。
- `finish_edit_rectangle`：构造 typed `MorphPremadeShape`（before/after Rectangle，orientation=wip.angle 方向），组 `ConcurrentEdits` → `WallHistoryCompoundEdit` → `EditType::attach_wall_compound`（既有历史边界）。
- `create_rectangle`：修正既有编译错误（`ridge_length_01_from_rectangle_ratio(x,z)` 两参）；补第二次 Color history 的 owner 接线（`History::record_color_edit` typed 边界）。

## 3. 仍登记（seam / 未落地）

1. `PreferredNodeAlignment` 的 decorator 调用点（`find_alignment_preferences` / `find_best_neighbor_alignments` / `generate_playermade_stair_requests` 图边构造）——恢复工作区无 system-decorator crate，类型已落地但无宿主调用。
2. 6 个 wall-domain UI 系统（`ui_create_rectangle`、`ui_edit_rectangle_dims`、`ui_edit_rectangle_orientation`、`ui_create_circle`、`ui_edit_circle`、`ui_rotate_circle`）仍使用 `crate::ui::ui_state::UiState` facade（独立 typed-gap 层，方法签名不同）；与 `systems::ui::resources::ui_state::UiState` 的统一未做，宿主注册/接线缺失。
3. `finish_edit_rectangle` 的 previous enclosure（墙真实 `Source` rectangle）未在 `InnerWalls` 暴露 → before 暂用 after 占位。
4. Color history 的 envelope tag/records/count 编码 U（`History::record_color_edit` 为空边界）；`get_building_and_roof_colors` 返回值仍为 Gap。
5. `UiWallHeight` +0x0a 之后字节 U；`UiState` +0x00..0x18、+0x1a..0x28 U。
6. `AutoClutterObject` 除 key@0x40 外字段 U（源树不可编译，无 cargo 测试入口）。
7. `MorphPremadeShape` 运行时内存布局 U（仅 serde schema 证据）。

仍登记数：**7 项**（上表 1–7；其中 1 为调用点、2 为 6 个 UI 系统合并登记）。

## 4. 验证

```
cargo check -p country-core --lib        # PASS（仅 warning）
cargo test  -p country-core --lib        # 1223 passed / 1 failed
```

新增/相关单测 **8 个（全通过）**：
- `stairs_assembly_params::layout_tests::{align_kind_layout_is_24b, preferred_node_alignment_layout_is_24b, outgoing_dir_matches_evidence}`
- `ui_state::layout_tests::{ui_state_layout_is_0x98, ui_widget_context_layout_is_16b, intersection_widget_id_is_at_0x38}`
- `adjust_wall_ui_mode::layout_tests::ui_wall_height_observed_offsets`
- `wall_compound_edit::morph_premade_shape_tests::morph_premade_shape_serde_schema_matches_observed_json`

唯一失败 `asset_libraries::placement_data_library::tests::load_registers_buffer_and_path_and_returns_data_with_handle`
（JSON 缺 `position` 字段）与本次结构/接线无关，为既有失败。
为使 `--lib` 测试可编译，顺带修正两处既有测试代码：`IvyPoint` 补 `Default`、`GladeType(true/false)` 改为真实变体。
