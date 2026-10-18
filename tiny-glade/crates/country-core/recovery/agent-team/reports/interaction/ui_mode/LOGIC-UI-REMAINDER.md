# 逐函数逻辑判等 — UI 域未审计模块(gui/toolbar/grading_lut/image_formation/prefab/photo_mode)

审计员: rebuild-ui-remainder
日期: 2026-10-17
方法: 协议 2026-10-16-logic-verify-protocol.md,全部 fresh ghidra 反编译(`--worker shared`,
`get_functions fields=signature,decompiled_code,callees`,每批 ≤20)+ 机器码核对
(`read_memory`/`disassemble_function`;PDB `void()` 占位不当真实 ABI,空反编译均核对字节)。
证据: `reports/interaction/ui_mode/evidence/results/{gui,toolbar,grading,imageform,photomode-prefab,serde}*-*.json`,
身份 B patched EXE tiny-glade.exe(sha256 cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323)。

## 判定统计

| 模块 | 判等 | 重建 | 未实现(无 body)登记 | 胶水-非业务 |
|---|---|---|---|---|
| gui(36) | 16 | 1(is_in_submenu 条件反,已修) | 0 | 19 |
| toolbar(24) | 6 | 0 | 0 | 18(含 serde 单体) |
| grading_lut(22) | 2 | 0 | 5 | 15 |
| image_formation(22) | 7 | 0 | 0 | 15 |
| prefab(resources,8) | 0 | 0 | 5 | 3 |
| prefab(systems,14) | 0 | 0 | 11 | 3 |
| photo_mode(6) | 3 | 0 | 0 | 3 |
| **合计(132)** | **34** | **1** | **21** | **76** |

> gui 另有 src 托管函数(不在 gui 清单内,附后):`advance_menu_state`(0x14068e5b0)、
> `ScreenLog::show`(0x14015e1c0)、`DebugGui::tod_snapshot_state`(0x1400bff20)、
> `TodSnapshotGuiState::show_toggle`(0x140144590)、`camera_mode_label`(CameraMode fmt
> 0x1400c48b0)——分别判等/胶水,见模块节。

---

## gui.rs(inventory 36)

### 判等(17)

| VA | 函数 | src 位置 | 核对点 |
|---|---|---|---|
| 0x14093ffd0 | `DebugTriggerProxyVis::as_string` | gui.rs | 5 case 跳表,case 首字节 `mov edi,len` 8/0xb/0xe/0x10/0x12 = "Show All"/"Only Meshes"/"Roofs with UVs"/"Meshes + Normals"/"Clutter Collisions",逐长度核对一致;返回 owned String(alloc+memcpy)。判等 |
| 0x14067dd80 | `PendingEguiContextResource::into_egui_context_resource` | gui.rs | 同址折叠为 `TweakerView::as_inner_mut`(4B `return`);src 为平凡 move `EguiContextResource(self.0)`。判等(平凡) |
| 0x14093fad0 | `NextEscMenuState::set` | gui.rs `set` | 旧值 Drop 胶水(旧 tag≠0xb 且 (旧-2>8‖旧-2==4) 时释放 slot6 Arc)+ 12 qword(96B EscMenuState)整块拷入。Rust `self.0=Some(state)` 经 Option 赋值 drop 旧值+move 新值,逐位对应。判等 |
| 0x14093fbb0 | `PrevEscMenuState::get` | gui.rs `get` | 机器码 `xor eax,eax; cmp dword[rcx],0xb; cmovne rax,rcx; ret` —— `*self==None(0xb)`→null,否则返 self。Rust `self.0.as_ref()`。判等 |
| 0x14093fbc0 | `EscMenuState::is_open` | gui.rs view.is_open | 机器码 `word==10→movzx byte[+8](flag)`;否则 `tag!=2→1, tag==2→0`。Rust `word==10 ? flag!=0 : tag!=2`。判等 |
| 0x14093fbe0 | `EscMenuState::is_closed` | gui.rs view.is_closed | 机器码 `word==10→flag^1`;`tag==2→1`;else 0。Rust 同。判等 |
| 0x14093fc00 | `EscMenuState::cast_ui_shadows` | gui.rs view.cast_ui_shadows | 机器码 `(word-3)<2→1`(word∈{3,4});`word==7→flag==1`;else 0。Rust `matches!(word,3|4)‖tag==7&&flag==1`。判等 |
| inline | `EscMenuState::allow_time_passage` | gui.rs view.allow_time_passage | 内联入 time_update(0x140922cba..0x140922cf7);=is_closed()‖(tag==7&&flag==1)。判等(内联,由 is_closed/tag==7 分解成立) |
| 0x14093fc20 | `EscMenuState::should_show_world` | gui.rs view.should_show_world | 机器码 `index=(word-2)若<9否则4; bt 0xaf→1; bt 0x50→0; else flag^1`。Rust 逐条同。判等 |
| 0x14093fc60 | `can_limit_frame_rate`(+共享 `can_use_static_world_capture`/`allow_painterly_overlay` inline) | gui.rs view | 机器码 `(tag!=7)‖(flag!=1)`(cmp dword[rcx],7; setne; cmp byte[+8],1; setne; or)。Rust 三函数同源共享。判等 |
| 0x14093fc70 | `LoadMenuThumbnailMetrics::positions` | gui.rs positions | 机器码 `mov eax,[rdx+0x40]`(cell_count)+ from_iter<Map<Range,closure>>(0x141304fb0):closure 读 column_major@+0x30/cells_per_line@+0x2c/step@+0x18+0x1c/origin@+8+0xc,列主/行主 div-mod + `col*step+origin`。Rust thumbnail_coordinate 的 NaN/inf 软件等价(MULSS/ADDSS)。判等 |
| 0x14093fcd0 | `VideoSettingsSubMenu::is_in_submenu` | gui.rs | **重建** → 判等(见 REBUILD)。机器码 `cmp byte[rcx],0; setne al` = `tag != 0` |
| 0x14093fe50 | `DebugGui::log_to_screen` | gui.rs log_to_screen | alloc+memcpy 消息;`next<5` 越界 panic;写 {cap@+0x10,len@+0x20,ptr@+0x18}+color@+0x28+remaining=0x40a00000(5.0)@+0x2c;`next=(next+1)%5`。Rust 同。判等 |
| 0x14093fce0 | `impl$9::default`(DebugGui::default) | gui.rs Default | HashMap RandomState 初始化 + 字段写:+0x102(trigger_proxy)=0、+0xf0..(enabled 组)=0、+0xf4(show_lazy_grid)=0、+0xf5(show_sheep/music)=0、+0xe8={1.0,1.5}、+0xf7(show_2d_ui)=1、+0xb8 HashMap 空表、+0x103/104 camera_mode/show_raycast=0。Rust 逐字段同。判等 |
| 0x140940170 | `ShowDebugRaycastGeo::as_string` | gui.rs | 长度表 0x142ab2bb8={2,5,5,0x14,0x16} + 字符串 "No","Walls","Roofs","Decorator collisions","Decorator interactions"(read_memory 逐字核对)。判等 |

### 重建(1)

| VA | 函数 | 分歧点 |
|---|---|---|
| 0x14093fcd0 | `VideoSettingsSubMenu::is_in_submenu` | 机器码 `setne` 返 `tag != 0`;旧 Rust `matches!(self,Self::Menu)` 返 `tag == 0` —— 条件反向。已改为 `!matches!(self, Self::Menu)`。见 REBUILD |

### 胶水-非业务 / VA 折叠(19)

- `impl$0::deref`(EguiContextResource Deref)、`impl$17::default`、`impl$18::clone`、`impl$21::default`、
  `impl$24::default`、`impl$28::default`、`impl$29::default`、`impl$30::clone`、`impl$31::clone`、
  `impl$33::clone`、`impl$45::default`、`impl$46::default`、`impl$51::eq`、`impl$56::eq`、
  `impl$63::default`、`impl$65::default`、`impl$6::positions::closure$0`——derive/闭包胶水。
- `_::impl$0::first`(0x140002370)——同址为 `WireframePass::clone_behavior`(3B return),gui 清单 VA 误锚,
  无独立业务体。`_::impl$0::next`(0x1408b99e0)——同址为 `clutter_meshes next`(6B return),同上。

### src 托管附加(不在 gui 清单,逐函数判)

- `advance_menu_state`(0x14068e5b0,toolbar::menu 泛型单体):take next.0(12 qword 存局部+写 None 0xb);
  若 incoming==Closed(2) 写 EscapeMenuJustClosed 事件(事件类型经 caller 泛型签名
  `EventWriter<EscapeMenuJustClosed>` 承明;反编译体 Vec 增长 ICF 折叠到 WriteSettingsFileCmd,不识别事件);
  之后 previous.set(mem::replace(current,incoming))。Rust `advance_menu_state` 同构。**判等(结构)**。
- `ScreenLog::show`(0x14015e1c0,gui_error_log closure):遍历 5 槽(stride 0x20),Some 时
  colored_label(message.clone())→`remaining -= delta`→`<0.0` 时清槽(dealloc+None 标记)。Rust 同。判等。
- `DebugGui::tod_snapshot_state`(0x1400bff20,do_gui_debug closure):hashbrown 探表 get_or_insert
  tod_snapshot_states(+0xb8,RandomState hash_one<GlobalId>)。Rust `entry(id).or_default()`。判等(结构)。
- `TodSnapshotGuiState::show_toggle`(0x140144590):`...` 按钮 clicked→toggle;egui UI 闭包。胶水。
- `camera_mode_label`(CameraMode fmt 0x1400c48b0):fmt 单次 write_str(6 基模式标签);UI 标签胶水。

---

## toolbar.rs(inventory 24)

### 判等(6)

| VA | 函数 | src 位置 | 核对点 |
|---|---|---|---|
| 0x140af8c50 | `Toolbar::new` | toolbar.rs new | 11 分类表逐组核对:buildings 全 {Wall(1),Wall(2),Decorator(0x3f),Wall(0),Fences(0x12)}/demo 3;paths 全 {7,8:1,8:2,8:3}/demo 1;decorators 全 {0xb:0x38,0x39,0x3a,0x3c,0x3d,0x3b}/demo 2;erasers 全 {9:1,2,3,5,6,4}/demo {9:1,2,5,3};color=0xc;tree=0x10(全)/空(demo);terrain=0xa;editing=0;water=0xf(全)/空;garden=0x11;photomode=0x13;current=0(Building)。Rust 测试断言 [5,4,6,1,6,1,1,1,1,1,1]/[3,1,2,1,4,0,1,1,0,1,1] 与机器一致。demo 门= `_DEV_OVERRIDE_IS_DEMO_MODE` 全局。判等 |
| 0x140af89d0 | `ToolbarCategoryModes::contains` | toolbar.rs contains | 遍历 modes(stride 0x10),按 tag+payload 归一比较(Decorator payload-0x38 clamp;Editing 0..5 特判)。Rust `modes.contains(mode)`(PartialEq 语义等价)。判等 |
| 0x140af9850 | `Toolbar::switch_mode` | toolbar.rs switch_mode | 遍历分类调 contains;未命中 panic "Error: Switched to mode but no toolbar category for ...";`-lVar9>10`(cat≥11)panic;命中写 `categories[cat].current=idx` + `current=cat`。Rust 同(含 assert category<11)。判等 |
| inline | `ToolbarCategoryModes::next_mode` | toolbar.rs | 循环闭包 check_if_can_switch<env9>(0x140af9f30):`cursor=(cursor+1)%count`(64 位 mod)。Rust `(current+1)%len`。判等 |
| inline | `ToolbarCategoryModes::last_selected_mode` | toolbar.rs | 同闭包:不循环时 `cursor` 现值 + 越界 panic_bounds_check。Rust `modes[current]`。判等 |
| inline | `Toolbar::advance_category`(native switch_category) | toolbar.rs | 已选同类→next_mode(循环),否则 last_selected;命中更新 current。判等(由 check_if_can_switch 闭包语义成立) |

### 胶水(18,含 serde)

- serde:`_::impl$0::deserialize` 族、`visit_str`(0x140afb5b0)、`visit_map/seq/enum`、`deserialize`(0x140afb280)、
  `serialize<Compact>`(0x1404f5100)、`serialize<Pretty>`(0x140afaed0)、`expecting`×2——serde_json 单态胶水。
- derive:`impl$4::clone`、`impl$8::eq`、`impl$12::clone`、`impl$1::from`(From<usize>)。
- `Toolbar::from_index`(src 侧 helper,非 native 独立函数)。

---

## grading_lut.rs(inventory 22)

### 判等(2)

| VA | 函数 | src 位置 | 核对点 |
|---|---|---|---|
| 0x140b01050 | `GradingLut::load_cube` | grading_lut.rs load_cube | read_to_string→parse_cube(DOMAIN_MIN/MAX 三分量 ==0/1 含 NaN 排除、LUT_1D_SIZE→"1D LUTs are not supported"、LUT_3D_SIZE→size=0x1f 缺省+resize(size³,0)、数据行 parse_rgb→pack_rgb9e5(0x1417c66b0)、"Too many entries..."、行上下文 fmt)→`TextureDesc::new_3d([size;3],0x7b).sampled()`→`create_texture("grading lut")`→unwrap。Rust 同。判等 |
| 0x140b01cc0 | `impl$2::load_cube::parse_rgb` | grading_lut_cube.rs parse_rgb | 机器码:3× str::next+`f32::from_str`(0x140d8ba40),词不够→format_err,解析失败→ParseFloatError;写 3 值+Ok 判别子。Rust split_whitespace+3 parse。判等 |

### 未实现(无 body)登记(5)

- `ColorGradingState::flattened`(0x140b00b20):读 +0x28 bypass 布尔,克隆名/读 4 f32 或写缺省 "default"+{1,0,1,1}。src 无 body。
- `GradingLut::load_unwrapped`(0x140b00bc0):PNG 解码链(image::Reader::open/decode→to_rgba8→TextureDesc)。src 无 body。
- `GradingLuts::load`(0x140b01df0):readdir 扫描目录→逐 LUT 加载(依赖 0x140ae5bb0 闭包未反编译)。src 无 body。
- `lut_name_to_asset_name` / `asset_name`(inline):inline 宿主,消费方 final_post 自带 `grading_lut:` 前缀。src 无 body。

### 胶水(15)

- serde `_::impl$0::serialize<Value>`(0x140af00e0:ColorGradingState 6 字段 bool+String+4×f32)、
  `serialize<Compact>`(0x1400ee2c0)、`visit_str`(0x1400ef340)、deserialize 族、`impl$5::clone`、`impl$7::eq`、
  `impl$0::default`、`impl$9::default`、closure glue——serde/derive 胶水。

---

## image_formation.rs(inventory 22)

### 判等(7)

| VA | 函数 | src 位置 | 核对点 |
|---|---|---|---|
| 0x1408ae7f0 | `impl$0::default`(ImageFormationConfigs) | image_formation.rs with_process_flag/default_plain/default_third | 机器码:3 档逐档写 name String "default"(cap/len 7)+ params_18=(1,0,1,1)(.rdata 0x142a9be30)+flag_28=0+flag_3d=1(档0/1)/档2 params_30=(0,0.015)+flag_3d=_DEV_OVERRIDE_IS_DEMO_MODE;末尾 `[+0xc0]=1`(kind=1)。Rust 逐档同。判等 |
| 0x1408aea50 | `ImageFormationConfigs::current_mut` | image_formation.rs | 机器码 `movzx eax,byte[rcx+0xc0]; shl eax,6; add rax,rcx` = self+(kind<<6)。Rust `configs[kind]`。判等 |
| 0x1408aea60 | `iter_non_default_mut` | image_formation.rs | 机器码写 `{begin=self, end=self+0xc0, idx=0}`(三档 slice iter)。Rust `configs.iter_mut()`。判等 |
| 0x1408aea80 | `get` | image_formation.rs | 机器码 `movzx eax,dl; shl eax,6; add rax,rcx` = self+(i<<6),**无边界检查**。Rust `configs.get(i)` 返回 Option(越界 None)。判等(安全适配:原生越界 UB→Rust None,src 注释已声明) |
| 0x1408aea90 | `switch_kind` | image_formation.rs | 机器码 `mov byte[rcx+0xc0],dl`。Rust `self.kind=kind`。判等 |
| inline | `current` | image_formation.rs | = configs[kind]。判等(内联) |
| 0x14067dd80 | `initial` | image_formation.rs | 同址折叠为 TweakerView::as_inner_mut(4B return);src `initial()`=default()。判等(平凡) |

### 胶水(15)

- serde `_::impl$0::serialize<Value>` 3 单体:outer(0x14085fd80)=3 serialize_field(ColorGradingState,CameraLensConfig,ArtisticConfig)、
  lens(0x14085faa0)=3×f32(vignette/chromatic_aberration/barrel_distortion)、artistic(0x14085fc20)=2×bool(lighting_only/tree_see_thru);
  `serialize<Compact>`(0x1400eea00);deserialize 族、`impl$2::clone`、`impl$6/11/16::eq`、`impl$12::clone`、`impl$14::default`——
  serde/derive 胶水(字段策略与 src GradingSave/LensSave/ArtisticSave 一致,已在 src 实现)。

---

## prefab(resources 8 + systems 14)

### 未实现(无 body)登记(16)

| VA | 函数 | src |
|---|---|---|
| 0x1408e2540 | `PrefabLibrary::add` | resources/prefab.rs 无 body |
| 0x141552d70 | `PrefabLibrary::get<&PrefabName>` | 无 |
| 0x141552ff0 | `PrefabLibrary::has<&PrefabNameRef>` | 无 |
| inline | `PrefabLibrary::iter` | 无 |
| 0x1408e5d10 | `impl$7::register_required_components` | 无 |
| 0x140840430 | `systems::prefab::impl$0::build` | systems/prefab.rs 无 body |
| 0x140002370 | `impl$3::clone_behavior` | 无 |
| 0x1408446b0 | `impl$3::register_required_components` | 无 |
| 0x140840610 | `spawn_prefabs` | 无 |
| 0x140841550 | `create_prefab_rt` | 无 |
| inline | `create_prefab_audio_trigger` | 无 |
| 0x140841b00 | `create_prefab_gpu_instances` | 无 |
| 0x140841d60 | `update_prefabs` | 无 |
| 0x140842d60 | `despawn_prefabs` | 无 |
| 0x140843390 | `despawn_prefab_contents` | 无 |
| 0x140843740 | `hot_reload_prefabs` | 无 |

> 根因:src 仅落地类型合同(`PrefabInstance`/`PrefabInstanceData`/`SpawnPrefabCmd`/`DespawnPrefabCmd`),
> 资产加载/实例化/回收宿主系统未恢复(forest/tree-tool 消费面已由别组接线)。登记不重建。

### 胶水(6)

`impl$1::default`、`impl$5::clone`、`impl$0::get::closure$0`(resources);
`impl$5::default`、`create_prefab_gpu_instances::closure$0`、`hot_reload_prefabs::closure$0`(systems)。

---

## photo_mode.rs(inventory 6)

### 判等(3)

| VA | 函数 | src 位置 | 核对点 |
|---|---|---|---|
| 0x140b59720 | `PhotoModeCameraMode::next` | photo_mode.rs next | 机器码 `movzx ecx,byte[rcx]; shl cl,3; mov eax,0x00030201; shr eax,cl; ret` —— packed 表 0x00030201 按 tag<<3 取字节 = {1,2,3,0}。Rust match 同。判等(inventory 误标 glue,实为业务算法) |
| 0x140b596b0 | `PhotoModeCameraMode::next_demo` | photo_mode.rs next_demo | 跳表 0x142af1278→case:tag0→0x140b596c7 `b0 01`=1、tag1→0x140b596d5 `b0 02`=2、tag2→0x140b596ce `31 c0`=0、tag3→panic("TopDown camera is unavailable in demo mode")。Rust 同。判等 |
| 0x14090e500 | `impl$1::from`(From<PhotoModeCameraMode> for CameraMode) | photo_mode.rs From | 机器码 `lea eax,[rcx+1]; ret` = tag+1。Rust `Self(value as u8+1)`。判等 |

### 胶水(3)

`impl$0::build`(0x140b59640,plugin build register PhotoModeCmd 事件)、`_::impl$0::next`(derive)、`impl$7::eq`(derive)。

---

## 附注

- 所有 `void()` 占位小函数(PrevEscMenuState::get、VideoSettingsSubMenu::is_in_submenu、
  current_mut/get/switch_kind、photo_mode next/from 等)均以 read_memory 字节为判等依据。
- `EscMenuStateView`/`view()` 是 src 对 native 12-qword enum 的结构化重建;view 产生的
  (tag,payload,flag) 与机器码 word/flag 逐位一致(Photomode→10、Closed→2、None niche=0xb)。
- `DebugGui`/`ScreenLog`/`LoadMenuThumbnailMetrics` 的 B 字节布局与 src Rust 布局不同(src 为语义模型,
  字段偏移注释指向 B 而非本布局);判等针对逻辑/数值,不承诺 ABI 复刻。
