# systems/glade-border 模块分析（B 版）

本报告只覆盖 `crates/systems/glade-border` 的**模块内部实现**，不展开上下层接线、caller/callee 链与 GPU 下游。
证据来源：B patched EXE（sha256 `cf048e27…6323`，程序名 `tiny-glade.exe`）Ghidra 定点反编译 + 原机器码反汇编 + 骨架 PDB 元数据 + 常量内存读取。
标记：O=已由原指令/原字节确证；I=由结构/调用形态推断；U=unknown，不猜。

## 0. 模块清单与地址

| B 源文件 | 函数 | 符号 | RVA / VA | 形态 | 源码行 |
|---|---|---|---|---|---|
| `src/display_glade_border.rs` | `display_glade_border` | `system_glade_border::display_glade_border::display_glade_border` | RVA 0x1be0ad0 / VA 0x141be0ad0（size 1074） | 独立物理函数 | 24 |
| 同上 | `display_glade_border::closure$0` | 同名 `::closure$0` | 无独立 RVA（inline-only，内联宿主 `from_iter` VA 0x141be6110） | inline 闭包 | 46 |
| 同上 | `display_glade_border::closure$1` | 同名 `::closure$1` | RVA 0x1be1080 / VA 0x141be1080（size 700） | 独立物理函数 | 67 |
| `src/feedback_input_out_of_border.rs` | `feedback_input_out_of_border` | `system_glade_border::feedback_input_out_of_border::feedback_input_out_of_border` | 无独立 RVA（inline-only；内联宿主 `run_unsafe` VA 0x141be2280、`run_without_applying_deferred` VA 0x141be5840） | inline 系统体 | 12 |
| `src/lib.rs` | `GladeBorderPlugin::build` | `system_glade_border::impl$0::build` | RVA 0x650ab0 / VA 0x140650ab0（size 366） | 独立物理函数 | 14 |

PDB 只记录 `void()` 占位签名；真实参数/ABI 由 PDB enrichment 的“候选系统接口”+ 原指令恢复（见各节）。

---

## 1. `display_glade_border`（VA 0x141be0ad0）

**输入（O/I）**
`(Res<AppMode>, Res<GladeBorder>, Res<UiSignifierStream>, Res<TerrainHeightsData<TerrainHeightSsbo>>, Res<GameTime>, EventReader<DisplayOutOfBorder>, EventWriter<ShowDemoBorderOverlay>, Res<GameConfig>)`
寄存器映射（O，来自 asm）：RCX=AppMode、RDX=GladeBorder、R8=UiSignifierStream、R9=TerrainHeightsData；栈参 [RBP+0x130]=（数值资源，见下）、[RBP+0x138]=EventReader、[RBP+0x140]=EventWriter、[RBP+0x148]=GameConfig。

**内部逻辑（O，逐段）**

1. **模式门（O）**：`RAX=[RCX]; CMP dword[RAX],0x13; JZ 尾部`。即 AppMode 判别值 == `0x13`(19) 时**直接返回**，不画边界；否则继续。变体身份 = U（PDB 未记录 AppMode 枚举值）。

2. **构造边界折线（O）**：
   - `ShapeParameters::as_curve2(GladeBorder)`（0x1411ff2b0）→ 2D 曲线 `Vec<Vec2>`（元素 8 字节，XZ 平面上的林间空地边界轮廓）。
   - `alloc::vec::spec_from_iter::from_iter<Vec3, Map<Iter<Vec2>, closure_env$0>>`（0x141be6110）→ 用 `closure$0` 把每个 `Vec2` 抬升到地形高度，得 `Vec<Vec3>`（3D 边界折线）。闭包捕获 `&TerrainHeightsData`（O：R9 存入 Map env）。
   - 释放中间缓冲。

3. **“有新事件才画”门（O）**：对 `[RBP+0x138]`（EventReader<DisplayOutOfBorder> 状态）做内联算术（见 §6），比较 reader 游标与 `Events<DisplayOutOfBorder>` 计数，`TEST R8B,DL; JNZ skip`。**仅当 reader 有未读事件时**才绘制：
   - `alloc 12*len` 复制折线；`DebugLine::new → color → line_width → set_ui_pass → show`（0x140b04e70/eb0/f40/ee0/f70）。
     - `line_width` = **4.0**（float `0x40800000` @0x142934694，O）。
     - `color` 参数 = 常量 **1**（枚举判别值，O；颜色枚举语义 = U）。
     - `set_ui_pass` 第三参 = 0（O）。

4. **demo 覆盖事件（O）**：`RAX=[0x14362c098]; CMP byte[RAX],1; JNZ 跳过`。该静态量 = `__imp_…DEV_OVERRIDE_IS_DEMO_MODE`，值 **false(0)**（O：0x143634d90 全 0）。为真才 `EventWriter<ShowDemoBorderOverlay>::write`（push 8 字节事件；Ghidra 因 ICF 标成 `grow_one<EventInstance<WriteSettingsFileCmd>>`，类型名不可信）。B 构建默认不执行。

5. **边界周长 UI signifier（O，条件）**：`RAX=[RBP+0x148]; RAX=[RAX]; CMP byte[RAX+0x121],0; JZ 画周长`。即 `GameConfig` 偏移 **0x121** 的 bool == false 时才：
   - `ShapeParameters::perimeter(GladeBorder)`（0x1411ff640）→ f32；
   - `format_inner` 构造字符串：格式片段常量 = 字符串 **"glade_border"**（@0x142efa130，len 12，O），带 1 个数值参数（Display 格式化器 `0x140d8d560`，参数值 = `(*[RBP+0x130]) + 0x2c` 处数值字段）；
   - `alloc 0x30` 构造 48 字节对象（含 1、1、折线 ptr/len/cap、perimeter f32 @+0x28），`SignifierFactory::new`（0x14089d8b0）+ `UiSignifierStream::write`（0x140b2cf90）→ 把“边界周长”标签写进 UI signifier 流。
   - 之后释放折线与格式化字符串。若 GameConfig[0x121] != 0，则释放折线并直接返回。

**算法**：地形高度抬升（2D 轮廓→3D 折线）；bevy `EventReader`/`Events` 记账（内联）；debug 线段可视化；UI signifier（标签）生成。

**关键数据结构/常量/偏移**
- `GladeBorder`（core 资源）：仅经 `as_curve2`（→`Vec<Vec2>`）与 `perimeter()`（→f32）消费；内部布局 = U。
- `ShapeParameters`（core `systems::wall::valid_enclosure`）：语义在 core，本模块只调用（依赖复用）。
- `GameConfig` 偏移 **0x121**（bool，语义 U）。
- 常量：line_width=4.0、color=1、AppMode 判别 0x13、DEV_OVERRIDE_IS_DEMO_MODE=false、格式串 "glade_border"。
- 栈参 [RBP+0x130] 资源，其 +0x2c 数值字段被格式化（资源身份 U，候选顺序对应 `Res<GameTime>`，语义待定）。

**未知（U）**：AppMode 变体 0x13 名称；DebugLine 颜色 1 含义；[RBP+0x130] 资源身份与 +0x2c 字段语义；signifier 文本最终渲染；ShowDemoBorderOverlay 载荷；GameConfig[0x121] 语义。

---

## 2. `display_glade_border::closure$0`（inline-only，宿主 0x141be6110）

**形态（O）**：`Map<Iter<Vec2>, closure_env$0>` 的映射闭包，源码行 46，内联进 `from_iter`（0x141be6110）。
**捕获（O）**：`&TerrainHeightsData<TerrainHeightSsbo>`（宿主 asm：R9 存入 Map env）。
**内部逻辑（O）**：对每个 `&Vec2`（边界轮廓点 xz）：
1. `TerrainHeightsData::sample_at_world_xz<TerrainHeightSsbo>(point)`（0x141246fb0）→ 采样该 XZ 处地形高度；
2. `utils::swizzlers::impl$1::xvy`（0x140c9cde0）→ 组合成 `Vec3`。原指令确证 `xvy`：`Vec3::new(self.x, v, self.y)`（`*out=self.x; out[1]=v; out[2]=self.y`，O）。
输出：`Vec3(point.x, terrain_height, point.y)` —— 把 2D 边界轮廓抬升到地形表面。

**算法**：地形高度采样 + 分量重排（swizzle）。**常量/结构**：无自有常量；依赖 core 的 `TerrainHeightsData::sample_at_world_xz` 与 utils 的 `xvy`。
**未知**：无（完全确定）。

最小 Rust 草案（逻辑确定，供参考，不写入工程）：
```rust
let h = terrain.sample_at_world_xz(*p);   // f32
Vec3::new(p.x, h, p.y)
```

---

## 3. `display_glade_border::closure$1`（VA 0x141be1080）

**形态（O/I）**：`FnOnce::call_once<closure_env$1, tuple<&AssetMeshLibrary, &AssetShaderLibrary, &mut Commands>>`（PDB enrichment），源码行 67–78。signifier 实体构造闭包。
**捕获（O）**：`Vec<Vec3>`（边界折线，env+0x8=ptr、+0x10=len）与一个 f32（env+0x18）。

**内部逻辑（O）**
1. `alloc 8`；把捕获的折线 `Vec<Vec3>` 复制进新分配（`12*len` 字节）。
2. `SignifierEntityBuilder::new`（0x14089d9b0）→ `set_ui_pass(_,_,0)`（0x14089dad0）→ `animation_duration(_,_,0.0)`（0x14089dc90，XMM2=0）→ `custom_shader(_,_, "dashed_line", len=11)`（0x14089db00；常量 @0x142efa180="dashed_line"，O）→ `build`（0x14089dd50）。
3. 经 `Commands` 队列插入 2 条命令（`EntityCommands::insert` 的 FnOnce 闭包，O）：
   - 命令1（0x28 字节）：`insert(DrawTopology)`（PDB 闭包名 `<enum2<DrawTopology>>`）；
   - 命令2（0x20 字节）：`insert(DashedLineLength)`，值 = **捕获 f32 × 0.7**（`MULSS XMM6,[0x1429cd030]`，`0x3f333333`=0.7，O）。

**算法**：UI signifier 实体装配 + 虚线参数（×0.7）。**常量**：着色器名 "dashed_line"、系数 0.7、animation_duration=0.0、ui_pass=0。
**未知（U）**：捕获 f32（env+0x18）来源/语义；`DrawTopology` 写入值（0x4000000000000001/0）语义；`DashedLineLength` 布局。

---

## 4. `feedback_input_out_of_border`（inline-only，宿主 0x141be2280 / 0x141be5840）

**输入（O/I）**：`(EventReader<FeedbackInputOutOfBorder>, EventWriter<PlayUiSound>, Local<InputOfOutBorderCache>)`。
**形态（O）**：完全内联进 `run_unsafe`/`run_without_applying_deferred`；无独立物理函数。

**内部逻辑（O）**
1. 校验/取两个资源：`Events<FeedbackInputOutOfBorder>` 与 `Events<PlayUiSound>`（宿主 panic 串确证，@0x142efa9a0 / @0x142efaad0）。
2. 对 `Events<FeedbackInputOutOfBorder>` 做内联 reader 记账（同 §6），判定是否有未读事件。
3. **若有未读事件**（O）：
   - 更新 reader 游标；
   - 若系统本帧尚未变更（`[state+0x158]==0`），`EventWriter<PlayUiSound>::write` push 事件：步长 **0x20** 字节，判别值 **4**（`[buf+0]=4`，O）→ `PlayUiSound` 变体 #4；
   - 置变更标志 `[state+0x158]=1`、tick `[state+0x130]=change_tick`。

**算法**：bevy `EventReader::read().last()` + `EventWriter::write`（内联）；`Local` 缓存去抖。
**数据结构/常量/偏移**：`InputOfOutBorderCache`（本 crate 的 `Local` 缓存类型，PDB 仅记名；字段 U）；`Events` 字段 +0x10/+0x18/+0x30/+0x38/+0x40；系统状态 +0x130/+0x140/+0x148/+0x150/+0x158；PlayUiSound 判别 4、步长 0x20。
**未知（U）**：PlayUiSound 变体 #4 音效；`InputOfOutBorderCache` 字段与去抖语义；FeedbackInputOutOfBorder 载荷。

最小 Rust 草案（方向确定，细节待 core/UI 类型）：
```rust
fn feedback_input_out_of_border(
    mut ev: EventReader<FeedbackInputOutOfBorder>,
    mut sfx: EventWriter<PlayUiSound>,
    mut _cache: Local<InputOfOutBorderCache>,
) {
    if ev.read().last().is_some() {          // 有“输入越界”反馈事件
        sfx.write(PlayUiSound::VARIANT_4);   // 播放 UI 音效（变体 #4）
    }
}
```

---

## 5. `GladeBorderPlugin::build`（VA 0x140650ab0，lib.rs）

**输入（O/I）**：`fn build(&self, app: &mut App)`（`GladeBorderPlugin` 为 ZST；RDX=app）。

**内部逻辑（O，来自 asm 0x140650ab0）**
1. `chain_ignore_deferred`（0x140ea2e70）：系统元组构造成“链式（顺序执行）”配置。
2. `Interner::intern<dyn SystemSet>`（0x140ea3260）：为命名系统集取 intern id（名字串未恢复，U）。
3. `ScheduleConfigs::in_set_inner`（0x140ea4120）：把链放进该 set。
4. `SubApp::add_systems<(), Update, …>`（0x140ea41b0）：把链加入 **`Update`** 调度。
5. 注册事件（O，TypeId + 存在性检查）：
   - `DisplayOutOfBorder`：TypeId = `{0x94d3a5e9912b82a6, 0x575605e438d8fae1}`；不存在则 `register_event<DisplayOutOfBorder>`（0x140ea4960）。
   - `FeedbackInputOutOfBorder`：TypeId = `{0xbe7a01660379830b, 0x9d0861613bfb42e8}`；不存在则 `register_event<FeedbackInputOutOfBorder>`（0x140ea4330）。

**算法**：bevy 插件构建胶水（schedule 配置 + event 注册）。**常量**：两事件 TypeId u128；调度 `Update`。
**未知（U）**：系统集名字字符串；两系统的相对顺序虽为 chain，但 set 名与 before/after 约束未定。

---

## 6. 共享内联算法：bevy `EventReader` / `Events` 记账（§1.3、§4.2）

`display_glade_border` 与 `feedback_input_out_of_border` 内联了**同一段**算术（O，两处指令序列一致），对象是 `Events<T>` 资源与 reader 游标：

```
x  = reader.last_event_count
t1 = sat(x - Events[+0x18]); t2 = sat(x - Events[+0x38]);
c  = sat(Events[+0x30] - t2);
a  = sat(Events[+0x10] - t1);
Events[+0x40]' = Events[+0x40] - (a + c)     // 游标回写
if (!(borrow(a) && borrow(c))) { …“有未读事件”分支… }
```

`SETBE`/`CMOVC`（饱和减）+ 位与门 → 判定 reader 是否有未读事件。**归属**：bevy `Events`/`EventReader` 内联记账（依赖复用），非本模块自有业务。字段精确语义 = U。

---

## 7. 逻辑总览

- 模块 = 一个 Bevy 插件 `GladeBorderPlugin`，向 `Update` 注册两个系统（顺序链）并注册事件 `DisplayOutOfBorder`、`FeedbackInputOutOfBorder`。
- `display_glade_border`：非 AppMode(19) 时，取边界 2D 轮廓（`as_curve2`），逐点采样地形高度抬升为 3D 折线（`closure$0`），收到 `DisplayOutOfBorder` 时用 `DebugLine` 画出边界（线宽 4.0）；可选：demo 覆盖下发 `ShowDemoBorderOverlay`；当 `GameConfig[0x121]==false` 时，用 `perimeter()` 经 `closure$1`（虚线 signifier，`DashedLineLength=f×0.7`，着色器 "dashed_line"）生成“边界周长”UI signifier。
- `feedback_input_out_of_border`：收到 `FeedbackInputOutOfBorder`（笔刷移出边界）时播放 `PlayUiSound`（变体 #4）；用 `Local<InputOfOutBorderCache>` 去重/去抖。

## 8. 算法清单

| 算法 | 位置 | 对应已知算法 | 状态 |
|---|---|---|---|
| 2D 轮廓 → 3D 折线（地形高度抬升） | `closure$0` | 地形采样 + swizzle `xvy` | O |
| bevy EventReader / Events 双缓冲记账 | §1.3、§4.2 | bevy `Events`/`EventReader` | I（字段 U） |
| debug 线段可视化 | §1.3 | `utils::debug_viz::DebugLine` | O |
| UI signifier（标签/虚线实体）生成 | §1.5、closure$1 | `SignifierFactory`/`SignifierEntityBuilder` 链 + insert 命令 | O（结构）/ 值 U |
| 虚线参数 | closure$1 | `DashedLineLength = f × 0.7` | O |
| 事件去抖 | feedback | `Local` 缓存 + change tick | I |

## 9. 未知清单（汇总）

1. AppMode 判别值 **0x13** 对应变体名（PDB 未存枚举值）。
2. `ShapeParameters::as_curve2`/`perimeter`/`GladeBorder` 内部布局（core 依赖，本模块只消费）。
3. `GameConfig` 偏移 **0x121** bool 语义。
4. 栈参 [RBP+0x130] 资源身份及其 +0x2c 字段。
5. `DebugLine::color`=1 的颜色枚举含义；`DrawTopology` 写入值语义。
6. `ShowDemoBorderOverlay` 载荷；B 构建 DEV_OVERRIDE_IS_DEMO_MODE=false（默认不触发）。
7. `PlayUiSound` 变体 **#4** 音效；`InputOfOutBorderCache` 字段与去抖语义。
8. `closure$1` 捕获 f32（env+0x18）来源；`DashedLineLength` 布局。
9. 插件系统集名字；`Events` 内联记账字段精确语义。

（本报告不主张模块已实现/验收完成；仅登记内部逻辑、算法、常量与未知。）
