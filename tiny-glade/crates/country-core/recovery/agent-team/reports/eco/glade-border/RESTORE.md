# restorer-eco-gladeborder — systems/glade-border RESTORE 报告

日期：2026-10-17
状态：in_review（FIX 5 条已按 reviewer-eco-gladeborder REVIEW.md 处理；判等 1 / 未实现-需重做 3 / 胶水 1）
涉及文件：`crates/systems/glade-border/src/{lib.rs, display_glade_border.rs, feedback_input_out_of_border.rs}`

## 证据（fresh 反编译，`--worker shared`，本会话重取）

- `get_functions` 0x141be0ad0 / 0x141be1080 / 0x140650ab0 / 0x141be2280（fields=signature,decompiled_code,locals,callees）
- `disassemble_function` 0x141be0ad0（246 条）、0x14089d8b0（SignifierFactory::new）
- `get_functions` SignifierFactory/UiSignifierStream/SignifierEntityBuilder/DebugLine 系（0x14089d8b0/0x140b2cf90/0x14089d9b0/0x14089dad0/0x14089dc90/0x14089db00/0x14089dd50/0x140b04e70/eb0/ee0/f40/f70）
- `get_xrefs_to` 0x141be1080（closure$1 调用方 `call_once<closure_env$1,(ref<AssetMeshLibrary>,ref<AssetShaderLibrary>,ref_mut<Commands>)>` @0x141be0a53）
- `read_memory`：格式串 0x142efa130="glade_border"、0.7@0x1429cd030、line_width 4.0@0x142934694、DEMO 全局 *0x14362c098→0x143634d90=0、0x142efa150 closure vtable{0x14014b850 drop, 32 size, 8 align, 0x141be0a30 call_once, 0x141be1080 body}

逐份 decomp/disasm 存于 `reports/eco/glade-border/evidence/`。

## 函数表

| VA | 函数 | Rust 位置 | verdict | 分歧点 |
|---|---|---|---|---|
| 0x141be0ad0 | `display_glade_border` | display_glade_border.rs:52 | **未实现-需重做** | ① DebugLine::show()（0x140b04f70）country-core 未实现（已登记为 country-core 任务）；② perimeter signifier 的 SignifierFactory/closure$1 写路径未还原（不再投影冒充，分支标未实现） |
| —（inline） | `display_glade_border::closure$0` | display_glade_border.rs:66-74（内联） | **判等** | 无 |
| 0x141be1080 | `display_glade_border::closure$1` | （未落地） | **未实现** | 依赖 country-core SignifierEntityBuilder/Commands/asset-lib 机制缺失（逻辑已解，见 §3） |
| —（inline，宿主 0x141be2280） | `feedback_input_out_of_border` | feedback_input_out_of_border.rs:38 | **未实现-需重做** | `PlayUiSound` 变体 #4（判别值 4、0x20B 事件）经共享单元结构不可表达（不再 ZST 冒充，写分支标未实现） |
| 0x140650ab0 | `GladeBorderPlugin::build` | lib.rs | **glue-legitimate** | bevy 插件胶水；已对齐：只注册 2 事件、两系统顺序链 |

## §1 display_glade_border（0x141be0ad0）— 未实现-需重做

**签名（O）**：8 参，PDB 候选接口 + 机器码寄存器映射（RCX=AppMode、RDX=GladeBorder、
R8=UiSignifierStream、R9=TerrainHeightsData、[RBP+0x130]=GameTime、[RBP+0x138]=EventReader、
[RBP+0x140]=EventWriter、[RBP+0x148]=GameConfig）。修正前实现的自造 `GladeBorderDebugLines` /
`MainCamera` 参数（原函数无此二项）。

**控制流（O，逐段）**：
1. **模式门**（disasm 0x141be0b00 `CMP dword[RAX],0x13; JZ`）：`AppMode::Photomode`（native_tag 19=0x13）早退。
2. **折线构造**（0x141be0b19 `CALL as_curve2` → 0x141be0b4b `CALL from_iter<Vec3,Map<..,closure_env$0>>`）：`ShapeParameters::as_curve2`（0x1411ff2b0）→ 2D 轮廓，逐点 `sample_at_world_xz`（0x141246fb0）+ `utils::swizzlers::xvy`（0x140c9cde0，`Vec3::new(x,h,y)`）抬升 3D 折线。Rust 以 `border.0`（BorderShape）match + `xvy(p, heights.sample_at_world_xz(p.x, p.y))` 落地。
3. **事件门**（0x141be0b95..0x141be0bde）：bevy EventReader 记账（`sat(last - events_a/b)`，`(a.len-a_idx)||(b.len-b_idx)` 非空）→ `display.read().last().is_some()`。
4. **绘制分支**（0x141be0c80..0x141be0ce9）：`DebugLine::new(折线.clone()).color(1=Red).line_width(4.0@0x142934694).set_ui_pass(0).show()`（0x140b04e70/eb0/f40/ee0/f70）。**分歧①（未实现）**：`.show()`（0x140b04f70）把折线（label/颜色跳表/width/ui_pass/dashed + 折线总长 f32）打包压入 append-only 调试流（`append_only_vec::push<UiAppendSignifier>` 族）；country-core `DebugLine` 无 show（source 已核，doc 注明 show 系列留组件/ECS 包）——Rust 构建链后不落 show，效果丢失。已明确登记为 **country-core 任务（debug_viz show 系列，含 0x140b04f70 语义）**，不以替代实现冒充。
5. **demo 覆盖**（0x141be0cee `CMP byte[*0x14362c098],1`）：`DEV_OVERRIDE_IS_DEMO_MODE`；B 构建 0x143634d90=0（read_memory），`overlay.write(ShowDemoBorderOverlay)` 死路径但逻辑保留。
6. **perimeter 门**（0x141be0d4f `CMP byte[*GameConfig+0x121],0`）：`!config.border_signifier_enabled` 时：
   - `perimeter`（0x1411ff640）；
   - `format!`（格式串 "glade_border"@0x142efa130，**12 字节无空格**，1 个 Display 参数值 = GameTime+0x2c = `frame_count`，格式化器 0x140d8d560）；Rust `format!("glade_border{}", game_time.frame_count)`（**无空格，reviewer 勘误已修**）；
   - 构造 **48B** 记录 `{tag=1, kind=1, 折线 Vec{ptr,len,cap}, perimeter f32@+0x28}`（`alloc 0x30`，disasm 0x141be0e3a..0x141be0e55；reviewer 勘误 44B→48B 已修）；
   - `SignifierFactory::new`（0x14089d8b0：拷贝 size 字节 → {size,ptr,size,record,&0x142efa150}）+ `UiSignifierStream::write`（0x140b2cf90：5×qword 槽）。**分歧②（未实现）**：country-core 无 SignifierFactory / closure 槽位，折线/label/closure$1 载荷不可承载 —— **不再以 UiAppendSignifier 投影冒充**；Rust 分支保留真实可表达部分（perimeter + label），写路径标未实现并登记 country-core gap（SignifierFactory + closure 槽位 + closure$1 唤醒）。

## §2 closure$0（内联，行 46）— 判等

PDB 内联进 `from_iter` 宿主（0x141be6110）；捕获 `&TerrainHeightsData`（宿主 asm R9 入 Map env）。
对每 `Vec2`：`TerrainHeightsData::sample_at_world_xz` + `xvy` → `Vec3(p.x, height, p.y)`。
Rust 内联于 display 的 `.map(|p| xvy(p, heights.sample_at_world_xz(p.x, p.y)))`，逐语句一致。**判等**。

## §3 closure$1（0x141be1080）— 未实现

**逻辑（O，decomp 全解）**：捕获 env = {折线 `Vec<Vec3>`@+0x8/+0x10, f32@+0x18}。
1. `alloc` 复制折线（12×len）；
2. `SignifierEntityBuilder::new`（0x14089d9b0）→ `set_ui_pass(_,_,0)`（0x14089dad0，`+0x8a = r8b^1`）→ `animation_duration(_,_,0.0)`（0x14089dc90）→ `custom_shader(_,_,"dashed_line")`（0x14089db00；串 @0x142efa180，len 11）→ `build`（0x14089dd50）；
3. `Commands` 队列 2 条 `insert`：
   - `insert(DrawTopology)`：0x28B 命令，值 `0x4000000000000001`（tag 1 = `DrawTopology::Alt`，载荷 f32 2.0）；
   - `insert(DashedLineLength)`：0x20B 命令，值 = **捕获 f32 × 0.7**（0x3f333333@0x1429cd030）。
调用方：`FnOnce::call_once<closure_env$1,(&AssetMeshLibrary,&AssetShaderLibrary,&mut Commands)>`
（0x141be0a53，经 0x142efa150 vtable 由 signifier 消费端唤醒）。
**未落地原因**：country-core 无 `SignifierEntityBuilder` 构建链、无对应 Commands 组件装配面；
closure env（折线+f32）由 SignifierFactory 消费端在 country-core 构造。本模块无法在现有 API
下逐语句落地，逻辑与常量已解（dashed_line / ×0.7 / DrawTopology::Alt(2.0)）——**未实现（跨模块 gap）**。

## §4 feedback_input_out_of_border（内联，宿主 0x141be2280）— 未实现-需重做

内联体（O，disasm）：
```
has_feedback = feedback.read().last().is_some();
if has_feedback {
    if state[+0x158] == 0 {   // 上一帧无反馈（上升沿）
        Events<PlayUiSound> push 0x20B 事件：判别值 4 @+0
    }
    state[+0x158] = 1;
} else {
    state[+0x158] = 0;
}
state[+0x130] = change_tick;   // bevy last_run
```
`state[+0x158]` = `Local<InputOfOutBorderCache>`（1 字节）。Rust 落地：`cache.had_feedback_last_run`
bool，控制流逐语句判等（`read().last()` + 上升沿去抖）。事件双缓冲 panic 串（
"Events<FeedbackInputOutOfBorder>Resource requested by  does not exist:" 等）为 bevy 胶水。
**分歧（未实现）**：原写入事件判别值 **4** = `enum2<PlayUiSound>` 变体 #4（0x20B 步长）；
country-core `widget_params::PlayUiSound` 为单元结构（source 已核，变体不可表达）——
**不再以 ZST `sounds.write(PlayUiSound)` 冒充**，音效写分支标未实现并登记 country-core gap
（widget_params 变体裁决，`PlayUiSound` 需还原为带变体 #4 的枚举）。

## §5 GladeBorderPlugin::build（0x140650ab0）— glue-legitimate

decomp：`chain_ignore_deferred`（0x140ea2e70）→ `intern<dyn SystemSet>`（0x140ea3260）→
`in_set_inner`（0x140ea4120）→ `add_systems<Update>`（0x140ea41b0）→ 按 TypeId 存在性检查
`register_event<DisplayOutOfBorder>`（0x140ea4960，TypeId 低 64=0x94d3a5e9912b82a6）、
`register_event<FeedbackInputOutOfBorder>`（0x140ea4330，TypeId 低 64=0xbe7a01660379830b）。
bevy 插件注册胶水（glue-legitimate），不重建。本 crate 以 `(world, schedule)` 落地：
`build` 已对齐原面——**只注册 2 事件**（删除自造的 ShowDemoBorderOverlay/PlayUiSound init；
这两事件由 add_ui 等插件注册，原 build 不注册）+ 两系统 `chain_ignore_deferred` 顺序链加入
Update。命名集名字 U（胶水，不影响语义）。

## 验收

- `cargo check -p system-glade-border`：**通过**（workspace 根，exit 0；仅 country-core 既有 warning）。
- reviewer FIX 5 条处理：
  1. display 分歧① `DebugLine::show()` → 未实现，**登记为 country-core 任务**（debug_viz show 系列，含 0x140b04f70 语义）；
  2. display 分歧② perimeter signifier 写路径 → 未实现，不再 UiAppendSignifier 投影冒充（折线/label/closure$1 载荷登记为 country-core 任务：SignifierFactory + closure 槽位 + closure$1 唤醒）；
  3. 格式串勘误：`format!("glade_border{}", frame_count)`（无空格，@0x142efa130=12B）；
  4. closure$1 确认未实现（SignifierEntityBuilder 链 + 2×Commands insert，逻辑/常量已解：dashed_line / ×0.7 / DrawTopology::Alt(2.0) / animation 0.0 / ui_pass 0）；
  5. feedback 分歧 `PlayUiSound` 变体 #4 → 未实现，不再 ZST 冒充，登记 country-core 任务（widget_params 变体裁决）。
- 跨模块 gap（country-core 侧待接线，本模块已登记）：
  1. `DebugLine::show`（0x140b04f70）——debug_viz show 系列；
  2. `SignifierFactory`/`SignifierEntityBuilder`/closure 槽位 + closure$1 唤醒；
  3. `PlayUiSound` 变体 #4（widget_params 枚举还原）。
