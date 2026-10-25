# reviewer-eco-gladeborder — systems/glade-border REVIEW 报告

日期：2026-10-17
审查对象：`restorer-eco-gladeborder` 对 `crates/systems/glade-border` 的重建（RESTORE.md + evidence/）。
审查方法：对每个符号 **fresh 反编译**（`--worker shared`，本会话独立重取），独立复述原函数完整控制流，与 Rust body 逐语句对照。证据存 `review-evidence/`（fresh-decomp/result-fresh-decomp.json、asc2-step*、job-ascurve*、job-callonce* 等）。

## 结论摘要

| VA | 函数 | Rust 位置 | reviewer verdict | restorer verdict |
|---|---|---|---|---|
| 0x141be0ad0 | `display_glade_border` | display_glade_border.rs:52 | **未实现-需重做**（2 分歧点） | 重建（不符） |
| —（inline） | `closure$0` | display_glade_border.rs:66-74 | **判等** | 判等 |
| 0x141be1080 | `closure$1` | （未落地） | **未实现** | 未实现 |
| —（inline，宿主 0x141be2280） | `feedback_input_out_of_border` | feedback_input_out_of_border.rs:38 | **未实现-需重做**（1 分歧点） | 重建（不符） |
| 0x140650ab0 | `GladeBorderPlugin::build` | lib.rs | **胶水-非业务（glue-legitimate）** | glue-legitimate |

统计：判等 1（closure$0）/ 未实现-需重做 3（display、closure$1、feedback）/ 胶水 1（build）。
**restorer 过度断言**：display_glade_border 与 feedback 标"重建"，但两处可观察载荷/效果丢失，不达判等。

---

## §1 display_glade_border（0x141be0ad0）— 未实现-需重做

**原逻辑（O，本会话 fresh disasm 0x141be0ad0..0x141be0eff + fresh decomp 独立复述）**：

1. **模式门**：`MOV RAX,[RCX]; CMP dword[RAX],0x13; JZ epilogue`。AppMode 判别 == 0x13(19) 早退（Photomode，country-core `native_tag` 19 已核对）。
2. **折线构造**：`as_curve2(GladeBorder)`（0x1411ff2b0，disasm 17 条）——
   - `TEST byte[RAX],0x1; JZ circle`；bit0==1 → `Rectangle2d::as_curve2(self+4)`（0x140c98cf0，4 角点）；
   - bit0==0 → `MOVSS XMM2,[0x1429cd068]`（**= 0.2，本会话 read_memory 实读**）→ `Circle2d::as_curve2(self+4, max_step=0.2)`（0x140c97cf0：`ceil(2πr / step)` 点数）。
   - `from_iter<Vec3,Map<Iter<Vec2>,closure_env$0>>`（0x141be6110）：逐 Vec2 `sample_at_world_xz`（0x141246fb0，双线性 4 采样 + 3 lerp）+ `xvy`（0x140c9cde0：`Vec3(x, v, y)`）→ 3D 折线。
3. **事件门**（bevy Events/EventReader 内联记账）：`(a=Events[0x10]-sat(x-0x18)) || (c=Events[0x30]-sat(x-0x38))` 非零才进入绘制分支（`TEST R8B,DL; JNZ skip`）。
4. **绘制分支**：`alloc 12*len` 复制折线 → `DebugLine::new → color(1) → line_width(4.0@0x142934694) → set_ui_pass(0) → show(&UiSignifierStream)`（0x140b04e70/eb0/f40/ee0/f70）。
5. **demo 覆盖**：`*__imp_DEV_OVERRIDE_IS_DEMO_MODE == 1`（0x14362c098 → 0x143634d90，B 构建全 0）才 push 8 字节 `ShowDemoBorderOverlay`。
6. **perimeter 门**：`CMP byte[GameConfig+0x121],0; JZ 画`。`perimeter()`（0x1411ff640）→ f32；`format_inner`：格式 pieces = **1 段 "glade_border"（@0x142efa130，12 字节，无空格）**，1 个 Display 参数 = `GameTime+0x2c`（= `frame_count`，country-core GameClock 布局已核对）；`alloc 0x30` 48B 记录 `{tag=1, kind=1, 折线 Vec{ptr,len,cap}, perimeter f32@+0x28}`；`SignifierFactory::new(label, record, &0x142efa150)`（0x14089d8b0：拷贝 size 字节 → 40B `{size, label_copy, size, record, vtable}`）+ `UiSignifierStream::write`（0x140b2cf90：5×qword 槽）。0x142efa150 = closure vtable `{0x14014b850 drop, 32 size, 8 align, 0x141be0a30 call_once}`。

**Rust body（display_glade_border.rs:52-115）逐语句对照**：

| 原 | Rust | 判定 |
|---|---|---|
| 模式门 Photomode 0x13 | `if mode.native_tag() == AppMode::Photomode.native_tag() { return }` | 判等 |
| as_curve2（Circle step 0.2 / Rect 无 step） | `match border.0 { Circle => circle.as_curve2(BORDER_CURVE_STEP), Rectangle => rect.as_curve2() }` | 判等（BORDER_CURVE_STEP=0.2 实读核对） |
| closure$0 抬升 | `.map(\|p\| xvy(p, heights.sample_at_world_xz(p.x, p.y)))` | 判等 |
| 事件门 | `display.read().last().is_some()` | 判等 |
| new→color(1)→line_width(4)→set_ui_pass(0)→**show** | `DebugLine::new(points.clone()).color(Red).line_width(4.0).set_ui_pass(false)` 后**未调 show**，`_line` 随即 drop | **分歧①：show 缺失，折线不落流，可视化效果丢失** |
| demo 覆盖 `*0x14362c098==1` | `border_overlay_enabled()`（const false） | 行为等价（B 构建全局=0）；运行时全局→常量建模，演示构建会偏离 |
| perimeter 门 `[+0x121]==0` | `!config.border_signifier_enabled`（语义字段，I） | 判等（语义映射） |
| `format!("glade_border{}", frame_count)`（pieces 1 段无空格） | `format!("glade_border {}", frame_count)` | **分歧②a：空格常量错**（原 label="glade_border42"，Rust="glade_border 42"） |
| label 进 SignifierFactory（业务载荷） | `_label` 绑定后**丢弃** | **分歧②b：label 载荷丢失** |
| 48B 记录 {1,1,折线,perimeter} + SignifierFactory + write | `signifiers.write(UiAppendSignifier{tag:1,kind:1, position:首点, scale:perimeter, name:0})` | **分歧②c：记录载荷丢失**（折线只剩首点、closure vtable 全丢，closure$1 永不触发） |

**分歧点**：
- **① `DebugLine::show()`（0x140b04f70）未调**：country-core `debug_viz::DebugLine` 无 `show`（source 已核，无该方法；doc 注明 show 系列留组件/ECS 包）。原实现把折线（label/颜色跳表/width/ui_pass/dashed + 折线总长 f32）打包压入 append-only 调试流；Rust 构建链后 drop，**效果丢失**。
- **② perimeter signifier 载荷丢失**：label 丢弃、48B 记录（含完整折线 + perimeter）与 closure vtable（closure$1）无法经 country-core `UiAppendSignifier`（40B {tag,kind,Vec3,f32,qword}）承载，投影写 `{1,1,首点,perimeter,0}`。原 signifier 的折线/标签/虚线实体装配（closure$1）全部不产出。

**verdict**：控制流 1-3/7-8 判等，但绘制与 perimeter 两路可观察效果未还原 → **未实现-需重做**（附上述 ①②）。

---

## §2 closure$0（inline，宿主 from_iter 0x141be6110）— 判等

**原逻辑（O，fresh decomp 0x141be6110）**：对每 `Vec2`：`sample_at_world_xz<TerrainHeightSsbo>`（0x141246fb0，双线性 4 采样 + 3 lerp）+ `xvy`（0x140c9cde0：`out.x=self.x; out.y=v; out.z=self.y`）→ `Vec3(x, h, y)`。捕获 `&TerrainHeightsData`（宿主 env `{start,end,heights}`）。

**Rust**：`.map(|p| xvy(p, heights.sample_at_world_xz(p.x, p.y)))` 内联于 display，与 `sample_at_world_xz(f32,f32)` + `xvy(Vec2,f32)` 签名一致（country-core `sample_at_world_xz(&self,x,z)`、utils `xvy(xz,y)` 已核对）。**逐语句一致 → 判等**。

---

## §3 closure$1（0x141be1080）— 未实现

**原逻辑（O，fresh decomp）**：捕获 env `{折线 Vec{ptr@+8,len@+0x10}, f32@+0x18}`：
1. `alloc 12*len` 复制折线；
2. `SignifierEntityBuilder::new`（0x14089d9b0）→ `set_ui_pass(0)`（0x14089dad0）→ `animation_duration(0.0)`（0x14089dc90）→ `custom_shader("dashed_line"@0x142efa180)`（0x14089db00）→ `build`（0x14089dd50）；
3. `Commands` 队列 2 条：`insert(DrawTopology)` 值 `0x4000000000000001`（tag 1=Alt，f32 载荷 2.0）、`insert(DashedLineLength)` 值 = **捕获 f32 × 0.7**（0x1429cd030 实读 = 0.7）；
4. 经 vtable `0x142efa150` 由 `call_once<closure_env$1,(&AssetMeshLibrary,&AssetShaderLibrary,&mut Commands)>`（0x141be0a30，fresh 已核：调 closure$1 后 dealloc env）唤醒。

**Rust**：crate 内**无 closure$1**；且因 display 的 perimeter 投影丢弃 vtable，closure$1 实际不可达，虚线实体永不装配。**未实现（与 restorer 一致）**。常量/逻辑已解（dashed_line / ×0.7 / DrawTopology::Alt(2.0) / animation 0.0 / ui_pass 0），待 country-core `SignifierEntityBuilder` + Commands 装配面。

---

## §4 feedback_input_out_of_border（inline，宿主 0x141be2280）— 未实现-需重做

**原逻辑（O，fresh decomp 0x141be2280，业务体摘出）**：
```
has_feedback = (Events<FeedbackInputOutOfBorder> 有未读事件)   // 同 display 记账
if has_feedback {
    if state[+0x158] == 0 {          // 上一帧无反馈（上升沿）
        Events<PlayUiSound> push 0x20B 事件：判别值 4 @+0，id @+0x18
    }
    state[+0x158] = 1
} else {
    state[+0x158] = 0
}
state[+0x130] = change_tick          // bevy last_run（胶水）
```
`state[+0x158]` = `Local<InputOfOutBorderCache>`（1 字节）。资源缺失 panic 串为 bevy 胶水。

**Rust（feedback_input_out_of_border.rs:38-49）逐语句对照**：

| 原 | Rust | 判定 |
|---|---|---|
| `has_feedback = 有未读事件` | `feedback.read().last().is_some()` | 判等 |
| 上升沿 `[+0x158]==0` 才写音效 | `if !cache.had_feedback_last_run { sounds.write(PlayUiSound) }` | 判等（控制流） |
| 写后置 1 / 无反馈置 0 | `cache=true` / `else cache=false` | 判等 |
| 写入事件：**判别值 4，0x20B**（enum2\<PlayUiSound\> 变体 #4） | `sounds.write(PlayUiSound)`（单元结构 ZST） | **分歧：变体 #4 不可表达，载荷丢失** |

**分歧点**：原事件判别值 4 = `PlayUiSound` 变体 #4（步长 0x20）；country-core `widget_params::PlayUiSound` 为单元结构（source 已核），ZST write 与原名不同、音效选择丢失。

**verdict**：控制流逐语句判等，但音效载荷（变体选择）未还原 → **未实现-需重做**（附分歧点，跨模块 gap：widget_params 变体裁决）。

---

## §5 GladeBorderPlugin::build（0x140650ab0）— glue-legitimate

**原逻辑（O，fresh decomp）**：`chain_ignore_deferred`（0x140ea2e70）→ `intern<dyn SystemSet>`（0x140ea3260，命名集，名字 U）→ `in_set_inner`（0x140ea4120）→ `add_systems<Update>`（0x140ea41b0）→ TypeId 存在性检查注册 `DisplayOutOfBorder`（0x94d3a5e9912b82a6）与 `FeedbackInputOutOfBorder`（0xbe7a01660379830b）。

**Rust（lib.rs）**：`init_resource` 2 事件 + `add_systems((display, feedback).chain_ignore_deferred())`。已对齐关键面：**只注册 2 事件**、两系统 `chain_ignore_deferred` 顺序链加入 Update。bevy 插件胶水，不重建。**glue-legitimate**（与 restorer 一致）。登记：`in_set_inner` 命名集成员资格被省略（集名 U，语义影响待接线裁决）；`register_event` → `init_resource` 为等价胶水落地。

---

## FIX 清单

1. **display_glade_border**：补 `DebugLine::show()`（依赖 country-core debug_viz show 系列，把折线压入 UiSignifierStream）——分歧①。
2. **display_glade_border**：label 改 `format!("glade_border{}", frame_count)`（去掉空格）——分歧②a。
3. **display_glade_border**：perimeter 路改经 `SignifierFactory::new(label, {1,1,折线,perimeter}, vtable)` + write 写完整 signifier（依赖 country-core SignifierFactory + closure 槽位；当前折线/label/closure 载荷丢弃）——分歧②b/c。
4. **closure$1**：落地 `SignifierEntityBuilder` 链（new→set_ui_pass(0)→animation_duration(0)→custom_shader("dashed_line")→build）+ 2×Commands insert（`DrawTopology::Alt(2.0)`、`DashedLineLength = f×0.7`）；依赖 country-core SignifierEntityBuilder + Commands 装配面。
5. **feedback_input_out_of_border**：写 `PlayUiSound` 变体 #4（依赖 country-core widget_params 枚举变体裁决）。

## 过度断言 / 勘误

- restorer 标 display/feedback 为"重建"，但两函数可观察效果/载荷未还原到判等 —— 需降级为未实现-需重做。
- 格式串常量错：原 pieces = `"glade_border"`（@0x142efa130，12B 无空格），restorer 落盘 `format!("glade_border {}")` 多一空格。
- 勘误（轻微）：RESTORE §1 记 44B 记录，机器码为 `alloc 0x30` = **48B**（{tag,kind,折线{ptr,len,cap},f32@+0x28}）。
- `border_overlay_enabled()` 以 const false 建模运行时全局 `*0x14362c098`：B 构建等价，但非同一真源；登记不改判。

---

## FIX 复验（2026-10-17，restorer 已处理 5 条）→ **fix_verified**

复验方式：重读改动后的 3 个源文件 + `cargo check -p system-glade-border`。

1. **FIX1（DebugLine::show）**：display_glade_border.rs 已不再保留任何"画了折线"的等效冒充；`DebugLine::new/color/line_width/set_ui_pass` 链后注释明确标**分歧①未实现**（0x140b04f70），登记 country-core 任务（debug_viz show 系列）。`_line` 仅构建并 drop，无副作用假装。✓
2. **FIX2（格式串空格）**：已改 `format!("glade_border{}", game_time.frame_count)`（无空格，@0x142efa130=12B）。✓ 已修。
3. **FIX3（perimeter 写路径）**：`UiAppendSignifier{tag,kind,position,scale,name}` 伪造投影**已删除**；保留真实可表达部分（`_perimeter`、`_label`），写路径标**分歧②未实现**并登记 country-core gap（SignifierFactory + closure 槽位 + closure$1 唤醒）。`signifiers` 仅 `let _ = &mut *signifiers;` 保活绑定，不再写任何载荷。✓
4. **FIX4（closure$1）**：确认未实现，逻辑/常量完整登记（dashed_line / ×0.7 / DrawTopology::Alt(2.0) / animation 0.0 / ui_pass 0）。✓
5. **FIX5（PlayUiSound 变体 #4）**：feedback 已删除 ZST `sounds.write(PlayUiSound)`；参数改 `_sounds`（未用），写分支标未实现并登记 widget_params 变体裁决。✓

`cargo check -p system-glade-border`：**exit 0**，本 crate 无新增警告（仅 country-core 既有 warning）。

**复验判定**：代码不再含伪造投影/等效载荷冒充；未实现分支与登记地址/常量准确（0x140b04f70、0x14089d8b0、0x140b2cf90、0x142efa150、判别值 4、0x20B 步长均与 reviewer fresh 反编译一致）。**fix_verified**。函数 verdict 不变：display/closure$1/feedback 仍为**未实现-需重做**（效果未还原，属跨模块 gap，由 country-core 侧任务闭合），closure$0 判等、build 胶水。
