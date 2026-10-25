# 逐函数逻辑判等 — ui/mode（input/UI/mode 组，卡E）

审计员: logic-verifier-camera-ui
日期: 2026-10-17
方法: 协议 2026-10-16-logic-verify-protocol.md，全部 fresh ghidra 反编译 + 机器码核对（`--worker shared`；radare2 仅作机器码 oracle）。

## 判定统计

| verdict | 数量 |
|---|---|
| 判等 | 13 |
| 判等（结构，逐字段/全 body 待补） | 3 |
| 未实现-需重做 | 12 |
| 胶水-非业务 | 2 |

## 判等（13）

| VA | 函数 | Rust 位置 | 核对点 |
|---|---|---|---|
| 0x1409a8e80 | `WidgetParams::sounds` | widget_params.rs | memcpy 96B(4×24)@0x18..0x78 + 整块回拷 0xb0。判等 |
| 0x1409a8ee0 | `WidgetParams::occlude_by_world` | widget_params.rs | 写整字节 @+0xa8 + 回拷。判等（Rust 存 bool 等价 0/1） |
| 0x1409a8fb0 | `UiSounds::empty` | widget_params.rs | 清 4 槽首 u32（偏移 0/0x18/0x30/0x48）。判等 |
| 0x1409a9110 | `WidgetVolumeShape::clearing_canvas` | widget_params.rs | tag=4、layer=arg、size=0x43020000_43020000(130×130)。判等 |
| 0x1409a9140 | `UiFocus::is_mei` | widget_params.rs | 机器码 `cmp [rcx],2; sete al;ret`。判等 |
| 0x1409a9150 | `UiFocus::is_toolbar` | widget_params.rs | 机器码 `cmp [rcx],1; sete al;ret`。判等 |
| 0x140929a90 | `despawn_marked` | signifiers/signifier_mark_for_delete.rs | 遍历标记实体调 `despawn`(0x140d14770)。判等 |
| 0x140a875e0 | `send_mode_just_changed_event` | mode_manager.rs | current≠previous→写事件(current,previous)+previous=current。判等 |
| 0x140afa2c0 | `check_if_can_switch<env$3>`（=select_app_mode） | mode_manager.rs `select_app_mode` | guard 后：mode∈{0..5,20}→推进工具栏 Editing 槽→switch_mode；结构匹配（帧 mod 推进 `(cursor+1)%count`）。判等（尾部 switch_mode 分支未全读） |
| 0x140afa9f0 | `check_if_can_switch<env$14>`（=toggle_photomode） | mode_manager.rs `toggle_photomode` | `*mode==0x13(19)`→mode=0+`Toolbar::switch_mode`；否则推进 Photomode 槽。判等 |
| 0x140a86b30 | `AppMode::default` | mode_manager/types.rs | `*in_RCX=0`。判等 |
| 0x140a86b40 | `AppMode::is_tool_selected` | mode_manager/types.rs | 机器码 `lea eax,[rcx-6]; cmp eax,0xf; setb dl; add cl,0xfb; mov eax,0x3cfe; shr eax,cl; and al,dl`。mask 0x3cfe 逐位=Wall..Color+Water..Fences({6..12,15..18})，与 Rust matches! 完全一致。判等 |
| 0x140867080 | `impl$6::clone_behavior` | brush_preview.rs | 机器码 `lea rdx,[0x1412eff70]; mov eax,2; ret`。disc=2。判等 |

## 判等（结构，逐字段/全 body 待补）— 3

| VA | 函数 | Rust | 分歧点 |
|---|---|---|---|
| 0x140a69340 | `signifier_animation_t` | signifiers/animation.rs | 原生 Query 迭代（组件表 stride 0xa0、源索引@+0x94）推进 t 并累加 tick。Rust 同 Query 逐实体 `t=(t+tick/duration).clamp(0,1)`、`tick_acc+=tick`。**结构判等**；原生 body 大部为 ECS 机制噪声，t 推进精确公式/`duration!=0&&!NaN` 守卫为 Rust 自加，未逐字段独立证实 |
| 0x140a86b60 | `mode_manager`（主系统） | mode_manager.rs | fresh 反编译截断(1.7KB)。**esc_menu 早退**命中：`hovered&&widget_id==canvas_id&&hit_tag!=2→return`。各 check_if_can_switch 闭包结构命中（env0 历史命令：mode=0xd(Replay)+`assert(direction<2)`+事件写；env3/env14 见上）。热键槽号表/全分支未因截断全读，按闭包+guard 判等 |
| — | `category_advance` | mode_manager.rs | 框架为各 category 闭包（guard→advance_category→mode 写）；env14 的 Photomode 分支已证。Generic 闭包逐槽未全读 |

## 未实现-需重做（12）

| VA | 函数 | Rust | 分歧点 |
|---|---|---|---|
| 0x1409a8e10 | `WidgetParams::new` | widget_params.rs | 原生默认 u32 组{0,8,0,1,0,2,3,3}命中，但 **u16@0xa8=0x101**→occlude@0xa8=0x01(默认 true)、flags@0xa9=0x01(bit0 置位)；**volume/sounds 区(0x78..0xa8)由参数 in_R8/in_RDX 拷贝**。Rust new() 无参、occlude=false、flags=0——默认值分歧 + 参数数据流缺失 |
| 0x1409a8f10 | `WidgetParams::no_history_recording` | widget_params.rs | 原生 `byte@0xa9 &= 0xfe`（**清 bit0**）。Rust `flags|=bit0`（**置位**）——方向相反；且默认 flags=0x01 语义为 bit0=record_history_when_used，Rust 的 NO_HISTORY_RECORDING 命名/语义均反转 |
| 0x1409a8f40 | `WidgetParams::allow_negative_distance` | widget_params.rs | 原生 `byte@0xa9 = arg<<2 | byte&0xfb`（**带 bool 参数可清 bit2**）。Rust 无参恒置位——参数缺失 |
| 0x1409a9130 | `UiFocus::as_mei` | widget_params.rs | 机器码 `xor eax,eax; cmp [rcx],2; sete al; mov rdx,[rcx+8]; ret`——**条件返回 (==2)+读 +0x8**，非恒转 Mei。Rust 恒返回 Self::Mei，数据流不符 |
| 0x140985f30 | `UiFocus::is_none` | widget_params.rs | VA 实际为 `AllocationHandle::is_null`（Ghidra 命名），非 UiFocus::is_none。逻辑 `==0` 平凡匹配枚举，**VA 映射错位**（RESTORE 已注同址折叠） |
| 0x140a9bd40 | `Slider3dState::project_cursor_onto_line` | slider3d.rs | 原生调用 `mouse_ss_to_cs`(0x140c906d0)+`project_onto_ray`(0x140ac84a0)。Rust 世界空间线段投影+clamp[0,1]——callee 全不符（屏幕→相机空间 + 射线投影 vs 世界空间线段） |
| 0x140a69c30 | `signifier_animation` | signifiers/animation.rs | 原生按 kind byte@+0x98 switch 0..0x10 映射 scale/alpha 目标@+0x1c/+0x20/+0x24（restorer 自注 U）。Rust `eased=t`/smoothstep 自造缓动 + scale=alpha=eased——switch 表缺失，自造近似 |
| 0x140b2cf90 | `UiSignifierStream::write` | signifiers/signifier_create.rs | 原生**无锁环形流**：lock+write counter@+0x168、`indices()`、**满时自旋等 read cursor**、0x28B 槽写 5×qword。Rust VecDeque 满时 pop_front（**丢弃旧记录**）——满语义分歧（原生阻塞自旋 vs Rust 丢数据），无锁机制未还原 |
| 0x140b2cfa0 | `signifier_create` | signifiers/signifier_create.rs | 原生：drain 流 + **Query 迭代 SignifierId(Without<MarkedForDeletion>) 建 hashset 去重** + 逐条生成实体（hash/insert/grow_one）。Rust 仅 drain→写 SignifierSpawned 事件——实体去重/生成机制未实现 |
| 0x140929160 | `signfier_mark_for_delete` | signifiers/signifier_mark_for_delete.rs | 原生：config_hash(WidgetId,Option<str>)/GlobalId hash + insert<WallId> 建映射、按 hash 对匹配实体打标记。Rust body **空操作**（只迭代 Query 不做任何事）——逻辑未实现 |
| 0x140865050 | `brush_preview`（主系统） | brush_preview.rs | 原生：flag@+0xfa 门→Query 遍历 BrushPreview 实体**Commands remove<Transform> 清旧预览**→读光标+当前工具笔刷大小更新/生成预览实体。Rust 读 mode tag 选 size + 发 BrushPreviewUpdate 事件——机制不符（原生直接改实体，Rust 事件化），工具→size 映射未证 |
| — | `brush_preview::brush_size`（行 94 内联） | brush_preview.rs | 原生 BrushSizes 读取随主系统；Rust 的 `{7→path,15→water,17→garden,_→1.0}` 映射未在 fresh 反编译独立确认（原生用当前工具而非固定 7/15/17） |

## 胶水-非业务（2）

| VA | 函数 | 备注 |
|---|---|---|
| 0x140866800 | `register_required_components`（BrushPreview） | 原生为完整 Bevy 注册（ComponentInfo/hash 表/register_required_components 递归）。Rust `fn(_world:&mut World){}` **空存根**——按 AGENTS.md 计注册胶水不独立审计，但存根不注册 = 组件未实际登记（接线缺口，需 host 补） |
| — | signifiers/mod.rs register_signifier_resources/add_signifier_systems | 注册/调度胶水，不审计 |

## 附注

- `AppMode::native_tag`/`DecoratorType`/`ClutterMeshTy` 的 serde 往返测试通过；native_tag 数值与 0x140afc7c0 序列化证据一致的假设成立，未逐函数 fresh 核对枚举判别表。
- `PlayUiSound`/`UiNavState` 为事件/资源声明（glue），不计业务。
- widget_params 的 flags 字节语义以 fresh 反编译为准：**bit0=record_history_when_used（默认置位），no_history_recording 清位**；RESTORE 原记"bit0=no_history_recording"方向有误。
