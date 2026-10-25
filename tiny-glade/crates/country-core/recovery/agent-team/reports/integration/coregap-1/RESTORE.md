# restorer-coregap-1 — country-core core-type-gap / core-api-missing 第一组 RESTORE

日期：2026-10-17
角色：core 集成 restorer（coregap-1）
范围：总账 `planning/2026-10-17-unimplemented-ledger.md` 第一组（SignifierFactory / SignifierEntityBuilder / PlayUiSound 变体#4、MoveTreeCmd 0x20 / InsertTreeCmd 0x48 / ReplayCommand elapsed・delay、PublicWalls 密集表、AutoClutter 布局、WoodenPlankSsbo、HighlightRoofCmd / HighlightColor、GraphicsDeviceResource、utils::calculate_hash 等）。

方法：全部 fresh 反编译（ghidra `--worker shared`，证据落 `reports/integration/coregap-1/evidence/batch*.json` + `decomp/*.c`）。硬标准：不发明字段，逐条登记 O/I/U。

**`cargo check -p country-core`：通过（Finished dev，10 warnings 全非本批文件；rhapsody/steam/onboarding 他组在途错误已随其自行闭合）**。

## 实现清单（10 条）

| # | 类型/API | 位置 | 证据（fresh） | 布局 |
|---|---|---|---|---|
| 1 | `PlayUiSound` 变体 #4 | `systems/ui/resources/widget_params.rs` | 0x141be2280（feedback 宿主） | 0x20B `{kind u32@+0, payload@+8..0x17, id u64@+0x18}`；`variant4(id)`（判别值 4） |
| 2 | `SignifierFactory` | `systems/ui/signifiers/signifier_factory.rs` | 0x14089d8b0 | 40B `{name_len@+0, name_ptr@+8, name_len@+0x10, &inner@+0x18, marker@+0x20}`；`new(payload, inner, marker)` 深拷贝 |
| 3 | `SignifierEntityBuilder` | 同文件 | 0x14089d9b0 / dad0 / dc90 / db00 / dd50 | 0x90B；`new`（拷 48B 网格载荷）/`set_ui_pass`（+0x8a=pass^1）/`animation_duration`（+0x78）/`custom_shader`（{len@0x60,ptr@0x68,cap@0x70}）/`build`（spawn；组件入队 GAP） |
| 4 | `MoveTreeCmd` 0x20 | `resources/tree_storage.rs` | 0x142267430 + 0x14226b1a0 | 0x20B `{id@+0, index u64@+8, position Vec3@+0x10}` |
| 5 | `ReplayBucket`/`ReplayCommand` 时序 | `resources/history/replay_cmd_buffer.rs` | 0x141bc34f0 | 每条命令时序头 `CommandTiming {flags u8@+8, delay f32@+0xc, elapsed f32@+0x18, started@+0x1c}`；`bucket.timing` 平行承载 + `advance_timing(delta)` |
| 6 | `HighlightColor` + `HighlightRoofCmd` | `resources/walls/mod.rs` + `resources/roofs/commands.rs` | 0x140b34f30 + 0x141a31280 | `HighlightColor(u8)` `to_rgb`：`==1→{0.1,0.03,0} else {0.07,0.08,0.07}`；`HighlightRoofCmd {_head@+0, wall u64@+8, color@+0x10}`（stride 0x18） |
| 7 | `GraphicsDeviceResource` | `render/device.rs` | 0x140de50a0（gpu_flush_buffers） | 持 rhapsody 设备 + `Deref`；`+0x188` 管线缓存槽登记 GAP |
| 8 | `WoodenPlankSsbo`（SparseSsbo/SsboType） | `resources/roofs/roof_buffers.rs` | 0x141a41ba0（rect push 面） | 元素 0x20B `{position, size, rotation}`（字段序 I 登记）；`SparseSsbo CHUNK_COUNT=0x800` + `SsboType Storage=SparseInstanceBuffer<Self>` |
| 9 | AutoClutter 布局 | `systems/ui_clutter/autoclutter_state.rs` | 0x140ee3fd0 / 0x140ee48c0 / 0x140ee4bf0 | `AutoClutterRegion` 0x40B（表项 0x48B 含 8B 键）、`AutoClutterSpawn` 0x68、`AutoClutterCandidate` 0x98、`AutoClutterState`（pending Vec<u16>@+0x30..0x48、区域表@+0x60、epoch@+0x90） |
| 10 | `utils::calculate_hash` | `utils/mod.rs`（re-export render::utils） | 0x1405bdda0（Sip13Rounds） | std `DefaultHasher`（SipHash-1-3 零密钥）逐位一致 |

附带实现（timberframe 依赖）：`resources::walls::split_height_into_skirts`（0x140b34f80 fresh：`height≥5.9` → `[4.5, 8.0, 11.5, …]` 步长 3.5、`height > f+1.4` 续）。

## 冲突跳过 / 待证（2 条）

| 条目 | 状态 | 说明 |
|---|---|---|
| **InsertTreeCmd 0x48** | **冲突跳过** | `resources/history/tree_edit.rs` 正被他组（history 组，06:44 在途，同批 stairs_history_edit/replay_raster/systems-history）编辑；未触碰。布局合同已从 fresh `place_tree` 0x142269970 解出：**stride 0x48 = {name String@+0, position@+0x10, scale@+0x20, {variant, rotation}@+0x28, flags@+0x30, id@+0x38, height@+0x40}**——供后续照落。 |
| **PublicWalls 密集表** | **待证** | fresh `set_roof_customization` 0x142299dd0 已解访问面（墙记录表 stride 0x48 @InnerWalls+0x1a8、屋顶位集@+0x28/+0x38、屋顶记录 stride 0x30、色表=记录+0x28 的 u32[]、色值源=Res<PublicWalls>+0x14）——但表都内嵌 `InnerWalls`/`RoofShape` 记录（walls 组在途域），未落库。登记 GAP（对齐 color RESTORE FIX-6）。 |

## 未做（他组域，未触碰）

- `SignifierEntityBuilder::build` 完整组件入队（`DrawableMeshBundle`/`UiPass`/`WireframePass`/`DontCastShadow`/`SignifierMinimumLifespan`/`SignifierParentToVolumeOrigin`/`TransientMesh` 未落库）——build 只 spawn，组件入队登记 GAP。
- `DebugLine::show`、`geo_src_to_edit_subject`、`History::insert_starting_edit`、`GameClock` 布局校正等其余 core-api-missing 条目不在本组（后续组）。

## 协作登记

- `signifier_factory.rs` / `render/device.rs` 曾被他组并发改动（world::Command 误导入已修、Debug derive 已去），当前编译通过。
- 证据：`reports/integration/coregap-1/evidence/batch{1..6}-*.json` + `work/coregap-1/decomp/*.c`（0x14089d8b0/9b0/dad0/dc90/db00/dd50、0x140b2cf90、0x142267430、0x14226b1a0、0x142269970、0x141bc34f0、0x142299dd0、0x141a31280、0x140de50a0、0x1405bdda0、0x140ee3fd0/48c0/4bf0/4a70/4ee0、0x140b34f30/80、0x141a41ba0）。

## reviewer-coregap-1 复核 FIX（已落实 2 条）

| FIX | 处置 |
|---|---|
| 1a SignifierEntityBuilder `new` 未拆 mesh_kind | **已修**：`mesh_payload` 改 `[u64;5]`（+0x18..+0x3f），`new` 拆首 qword → `mesh_kind`（build switch 键）+ 余 5 qword 入载荷（对 0x14089d9b0 `in_RCX[2..7] ← in_RDX[0..5]`）。 |
| 1b `default` 未装默认 shader | **已修**：`Default` 装 `"vertex_color_shader"`（len 0x13 = 19，对 `new` 的字符串构造块）。 |
| 1c `custom_shader` 未释放旧串 | **已修**：`shader_name_replace` 先释放旧 `{ptr,len}`（对 0x14089db00 `+0x60 != 0 → rust_dealloc`）。另补手动 `Clone`（深拷贝 shader，避免裸指针 + Drop 派生 Clone 双重释放）。 |
| 2 `advance_timing` 位极性反转 | **已修**：只对 `flags.bit0 置位` 命令推进 `elapsed += delta`，清零命令待移除不推进（对 0x141bc34f0：`pcVar33[8]&1 == 0` → 移除列表；`!= 0` → 推进 elapsed）。`CommandTiming` 偏移判等保持。 |
