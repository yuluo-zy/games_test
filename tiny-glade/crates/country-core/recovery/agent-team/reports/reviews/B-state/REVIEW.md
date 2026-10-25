# REVIEW — B-state：water/timberframe/roof/color/decorator 24 项独立逐语句复核

- reviewer：`reviewer-B-state`
- 日期：2026-10-18
- 范围：`planning/2026-10-17-evidence-audit-ecstate.md` B 类中
  **water / timberframe / roof / color / decorator** 五模块的全部业务函数。water 在 B 类为 0 项
  （`draw_water`/`update_water_audio_grid`/`record_water_history` 已在 state REVIEW 判等；
  `draw_water::closure$0` 为 D），故实审 **24 项**：timberframe 2、roof 3、color 11、decorator 8。
- 方法：**独立 fresh 反编译/反汇编**，不复用 restorer 结论。
  - Ghidra shared 持久库 `program=tiny-glade.exe`，`get_functions`（signature+decompiled_code+callees）
    24 函数一批 → `evidence/result-all.json`、`evidence/decomp/*.c`。
  - `show_color_options` 0x14229a7f0 Ghidra 反编译仍超时（code=0），改用 fresh
    **radare2 反汇编 + 跳表字节实读**：入口 0x14229a7f0、case0 demo 分支 0x14229b725、
    case0 非 demo 0x14229cc37/0x14229cf29、case1 非 demo 0x14229d754；跳表
    0x14303ca70 40×dword 实读（entry0→0x14229b725、entry1→0x14229bde9）。
  - `apply_decorator_op` 断言面/调用点用 radare2 反汇编补证（0x141276e30 入口 80 指令 +
    4 个 caller 的 edx 面）。
  - 常量/字段以 fresh 反编译立即数与反汇编写面为准；未调用任何写工具，**未改 src**。
- 硬标准：逐函数 fresh 反编译 → 独立复述完整控制流 → 与 Rust body 逐语句对照 → verdict；
  逻辑未到判等一律不因"已登记"放行，登记缺口作为 FIX/阻塞项单列。

## 0. 三态结论

口径：**判等** = 函数体逐语句等价（仅剩 I 级不可观测差或已证等价表示）；
**需修** = 主体控制流已重建且大部分语句判等，但存在具体缺失/分歧语句（含 core typed-gap 阻塞的具体写段）；
**未实现** = 主体逻辑（收集主循环/spawn/密集表/执行体）缺失或运行期不可达。

| # | 模块 | 函数（VA） | verdict | 一句话依据 |
|---|---|---|---|---|
| 1 | timberframe | `construct_timberframe_rectangle` 0x141a41ba0 | **需修** | 相位/常量/计数写回逐语句判等；梁 RenderInstance 逐轴长度/朝向 I，`WoodenPlankSsbo.size` 单标量不可承载 |
| 2 | timberframe | `construct_timberframe_circle` 0x1421b67a0 | **需修** | 同上；主环朝向二进制由主高构造（`local_f4*{0,1,0,0}`），Rust `Quat::IDENTITY` 占位 |
| 3 | roof | `generate_roof_stone_floor_and_roof_bottom` 0x1421a6d40 | **需修** | 事件→roof→`(b[8]&1)==0`→`log2f(r*3)`→queue 判等；NaniBundle insert 执行体为 no-op |
| 4 | roof | `highlight_roof`（iter 单态）0x141a284b0 | **判等** | 清 tag→逐事件写 value/tag/`to_rgb`；8B store 覆盖 pad 为 I |
| 5 | roof | `highlight_roof`（store 单态）0x141a31280 | **判等** | 同上（存储迭代权威；EventReader 已覆盖双缓冲） |
| 6 | color | `init_color_ids_buffer` 0x142290750 | **需修** | 3×insert_resource 顺序判等；4×`add_new<*ColorIdSsbo>` 未落（typed-gap） |
| 7 | color | `set_wall_color_id` 0x142290820 | **需修** | 事件/索引/写色/AABB/石膏分类判等；SSBO `update_buffer_bytes` 段缺；selector/value 面证据 U |
| 8 | color | `set_gate_color_id` 0x142299840 | **需修** | 墙命中门→写色→custom==1 AABB tag7 判等；SSBO 段缺；selector U |
| 9 | color | `set_halftimber_color_id` 0x142299250 | **需修** | 无条件写色 + custom==1&&墙命中 tag0xc 判等；SSBO 段缺 |
| 10 | color | `set_decorator_color_id` 0x1422af0c0 | **需修** | 5 层事件源/写偏移/tag 逐语句判等；L1 非 sub4、L3/L4/L5 记录 value 为二进制栈值 U |
| 11 | color | `set_flag_connection_and_chimney_smoke` 0x142298970 | **需修** | flag 写 +4/tag9 判等；烟 `+4` 写缺失；**新发现**：两者记录 selector 字节 = event+0xc，Rust 写 0 |
| 12 | color | `set_roof_customization` 0x142299dd0 | **未实现** | 外环密集位集/分块索引 + **第二写** `*(色表+idx*4)` 缺失；样式环/History 未建 |
| 13 | color | `show_color_options` 0x14229a7f0 | **需修** | case0 13 swatch（v1c/v24 逐值）/plaster_peel/halftimber+skirt、case1 11 roof fresh 判等；case2..0x26/事件写面/门源 GAP |
| 14 | color | `hide_color_menu` 0x1422adcf0 | **未实现** | 仅事件存在门 + 全量 despawn；二进制逐图标 subtype switch（+0xc 写面 9 处）未建 |
| 15 | color | `autoclose_color_menu` 0x142298d30 | **未实现** | 二进制 kind==0xc 直闭 + 图标世界位置/2D 光标跳过 + 3D>10；Rust 以 menu.center 近似 |
| 16 | color | `ColorRadialMenu::create` 0x1425f27c0 | **未实现** | `AngleRange`/cos/sin 极坐标/`spawn<4 组件>`/tag-3 分派全缺；仅数据组装 |
| 17 | decorator | `maintain_dormer_tiles` 0x1407ce920 | **判等** | 游标预写/每瓦 +1/id 单槽/hash_one+remove_entry/drop 逐语句；null guard 为安全缝 I |
| 18 | decorator | `apply_decorator_op` 0x141276e30 | **需修** | guard/kind 分派/归档先于历史/kind3·4·5+locator 判等；归档叶替换、0x88B event U、48B 历史其余字段未证 |
| 19 | decorator | `create_or_restore_decorator_backup` 0x1408238a0 | **需修** | 状态读清/state!=1&&created→RESTORE/三 map tracked clone/+0x48 门控判等；辅助 0x30B 三 map 反向拷贝缺 |
| 20 | decorator | `decorator_record_history` 0x140817850 | **需修** | +0x48 读清/kind6 break/无条件清 Vec+HashMap 判等；48B `{1,1,kind,f1,f2,f3}` 载荷未写入（返回 default） |
| 21 | decorator | `window_lights` 0x1407b5980 | **未实现** | 阈值/灯态/retain 先于帧末/计时公式判等；窗实体收集主循环与形状变化更新缺失 |
| 22 | decorator | `chimney_smoke_anim_thickness` 0x140818df0 | **未实现** | 窗口/目标/符号/`clamp01(old+sign*dt/target)` 判等；spot 收集+初始插入缺；**新发现**：retain 在更新前，Rust 在更新后 |
| 23 | decorator | `replay_decorator` 0x14080c8d0 | **未实现** | tag7/flag0/`max(d/speed,0.05)`/elapsed/索引反转删除模型判等；core 无 Decorator 变体与桶 cull 表，`decorator_cmd_of` 恒 None |
| 24 | decorator | `calc_gothic_half_dormer_profile_ws` 0x1407eb160 | **未实现** | 仅 `0..2` 两次 for_each 骨架；wall-space A/B/C/D lerp/法线/截断精确流未还原 |

**三态：判等 3 / 需修 13 / 未实现 8（合计 24）。**
water 0 项（B 类空）。判等升级 3 项：`highlight_roof` 两单态、`maintain_dormer_tiles`。
`show_color_options` 原 FIX-B2（plaster swatch `+0x24` 副值）经 fresh 反汇编**确认已正确落库**（见 §13），该 FIX 关闭。

---

## 1. 方法与证据（fresh）

- `evidence/result-all.json`：get_metadata（B patched，function_count 131517）+ 24 函数。
- `evidence/decomp/<name>.c`：Ghidra fresh 反编译正文（逐函数独立复述依据）。
- `evidence/result-r2-show*.json` / `r2-*.txt`：show_color_options 五段反汇编、跳表实读。
- `evidence/result-r2-apply.json`、`r2-apply-entry.txt`、`result-r2-callers.json`：apply_decorator_op guard 与 caller edx 面。
- `evidence/result-r2-wall.json`、`r2-wall-tail.txt`：set_wall_color_id SSBO 上传尾块。
- 限制：`show_color_options` 仍无法反编译（Ghidra 空、历史 ReVa 超时），本轮以 fresh 反汇编逐段核对入口/跳表/case0/case1 载荷；case2..0x26 未逐 case 反汇编（本身未实现）。

---

## 2. timberframe（2 项，均需修）

### 2.1 `construct_timberframe_rectangle` 0x141a41ba0 — 需修

fresh 复述（`decomp/timberframe_rectangle.c`）：

1. 槽空→`FrozenMap::get<WoodenPlankSsbo>` + 名字断言（len 0x3d）+vtable+新建 allocation+drop 旧（L140-201）；二次查询+allocation+0x18 互斥+`writer.clear()`（L202-241）。
2. 事件链 A/B 双缓冲，步长 **0x28**；`do{...} while ((*event & 1)==0)` → **处理 bit0=1**，payload=event+4（L254-322）。
3. 垂直梁：`as_curve3` → 每顶点 `orientation_mat3` → `RenderInstance::from` → `push`；每根 +1（L326-421）。
4. 等距梁：`floorf(√d²/1.51)`；负/NaN→0，>1.8446e19→u64 max；`if 1<n`：`t=i/n` lerp→`orientation_mat3`→0.18→push；count+=n-1（L450-555）。
5. 裙层：`split_height_into_skirts` + 栈 `{0.0, cache+0xc}` 两环；每环相邻点中点、`rotation_from_normal`、高 `h+cache+8`、0.2、push；count+=每环边数（L567-733）。
6. 屋脊：`event[0x1c]==0` 门 → `roof_height_ws_from_wall_shape`、`ridge_length_01...`、`RidgeDims::calc` → `as_curve2`×2 → `min(lenA,lenB)` 根（`rotation_from_normal`、0.2）；`0.5<perimeter()` 再补 1 根（L736-896）。
7. 每事件末 `tick` + `BeamCount+8 = 本事件计数`（L898-899）；耗尽后互斥释放 + run-out log（L278-299）。

对照 Rust（`timberframe_rectangle.rs`）：1-7 的控制流、常量（1.51/0.18/0.2/0.5）、tick/计数写回位置、事件步长与 bit0 方向、per-event 重置全部逐语句一致。
**具体分歧（阻塞）**：二进制 `RenderInstance::from` 的尺寸是逐轴的（截面 × 段长，L384-400/L470-535 的乘 0.2 前含段长/主高因子），Rust `WoodenPlankSsbo::new(pos, size: f32, rot)` 只有单标量截面（`country-core/src/resources/roofs/roof_buffers.rs:68`）→ 梁长度维度不可表达；`rotation_from_normal` 以 `beam_rotation(edge)` 代理（I，已登记）；`ResMut<AssetSsboLibrary>` vs 原 `Res` frozen 图面差（已登记）。**verdict：需修**（控制流判等；写路径物理量不等）。

### 2.2 `construct_timberframe_circle` 0x1421b67a0 — 需修

fresh 复述（`decomp/timberframe_circle.c`）：

1. `FrozenMap::get`+名字断言+vtable；`if (*(u64*)*in_RDX==0) unwrap_failed`（panic）；allocation 空→panic；互斥+资源校验（L~90-150）。
2. 事件步长 0x28；`do{}while((*event&1)!=0)` → **处理 bit0=0**（与 rect 相反，L~160-230）。
3. 主环：`perimeter/1.5`→trunc；`<7→6`；`lVar15=0`；`i/(count-1)`（0..1+NaN panic）→`get_pos_at_u`→`xvy`→高 `cache+0xc`→0.2→push（L239-318）。
4. 裙层：`split_height_into_skirts`+`{0.0,cache+0xc}`；每环 `i/(count-1)`+相邻点 `rotation_from_normal`→高 `h+cache+8`→push（L319-460）。
5. 屋顶环：`event[0x1c]==0`→`roof_height_ws_from_wall_shape`→高 `roof+cache+8+cache+0xc`→push（L476-576）。
6. 每事件末 `BeamCount+8=lVar15`（L584）。

对照 Rust（`timberframe_circle.rs`）：count/`u=i/(count-1)`/三相位顺序/两固定环/per-event 计数写回逐语句一致。
**具体分歧**：主环朝向 fresh 反汇编（L287-299）由 `local_f4`（=params+0xc 主高）构造 `{0, h, 0, 0}` 基/四元数分量，Rust 为 `Quat::IDENTITY` 常值占位（`timberframe_circle.rs:98`）；写路径尺寸同 §2.1 单标量问题。**verdict：需修**。

---

## 3. roof（3 项：判等 2 / 需修 1）

### 3.1 `generate_roof_stone_floor_and_roof_bottom` 0x1421a6d40 — 需修

fresh（`decomp/generate_roof_stone_floor.c` L60-125）：事件 stride 0x20；`PublicWalls` 墙表 stride 0x14（key/idx 校验，失败 unwrap_failed）；roof dense bit（+0x28 位图 >> idx&0x3f）；roof 元素 `0x58` stride；`(roof[8]&1)==0` → `log2f(roof[0x14]*3.0)`；`DecoratorRank::new` + `drawable::from`；Commands 队列 push 0x38B 项（insert fn、16B 数据、tag 9@+0x18、0@+0x20、entity@+0x28、panic@+0x30）。
Rust：事件→`roof_entity`→`roofs.get`→`(observed[8]&1)==0`→`log2f(radius*3)`→`commands.queue(ObservedStoneFloorCommand)`，逐语句对应。
**分歧**：`ObservedStoneFloorCommand::apply` 为 no-op（`generate_roof_stone_floor_and_roof_bottom.rs:64-69`），原 insert 执行体（NaniBundle<StoneFloorInstanceData> 16B 载荷）未落。**verdict：需修**（CPU 控制流判等；命令执行体未实现）。

### 3.2 `highlight_roof` 0x141a284b0 / 0x141a31280 — 判等

fresh（`decomp/highlight_roof_store.c` L40-135）：XADD 取世界 tick；事件 stride 0x18（wall u64@+8、color u8@+0x10）；进循环前 `*(u8*)(highlight+9)=0`（tag 清 0）；逐事件：`*puVar6 = u64@event+8`（**8B store** 覆盖 value+pad）、`*(u8*)(puVar6+1)=event+0x10`、`to_rgb()`→EditingHighlight.color（8B+4B），两资源 +0x5c tick 写；耗尽后 `EditingHighlight+0x160 = last_run`。
Rust（`highlight_roof.rs:26-41`）：`highlight.tag=0`；逐事件 `value=wall as u32`、`tag=color.0`、`editing.color=to_rgb()`；ResMut 自动 tick；EventReader 覆盖双缓冲。
差异仅：8B 写 vs u32+tag 字段（pad 互不可观测，I）；+0x160 last_run 为 Bevy ResMut/系统 tick 的显式写，Rust 由框架承担（等价）。**verdict：判等（I 级 pad）**。两单态同一结论。

---

## 4. color（11 项：需修 7 / 未实现 4）

### 4.1 `init_color_ids_buffer` 0x142290750 — 需修
fresh（`decomp/init_color_ids_buffer.c`）：顺序 = `add_new<WallColorIdSsbo>` → `add_new<RoofColorIdSsbo>` → `insert_resource(WallColorIds::new)` → `add_new<GateColorIdSsbo>` → insert Gate → `add_new<HalftimberColorIdSsbo>` → insert Halftimber。
Rust（`set_wall_color_id.rs:42-51`）：3×insert 顺序与内容一致；**4×add_new 全缺**（core 四类型为 typed-gap，无 SsboType）。**verdict：需修**。

### 4.2 `set_wall_color_id` 0x142290820 — 需修
fresh（`decomp/set_wall_color_id.c`）：FrozenMap get + len 0x32/memcmp 50B 断言；事件 stride 0x18；每事件 `idx`（0x1400885d0）→`0x4fff<idx` panic→bounds check→读旧→写 `WallColorIds[idx]`→tick→`PublicWalls::get`（为 0 则继续下一事件）；命中且 `event+0x14!=0`：`is_halftimbered()`（返回字节进入记录 selector）、折线 minps/maxps AABB→center+max_y+xvy→0x20B 记录（kind 6）；石膏：新∈{4..10,12}→Create，否则旧∈集→Delete；事件耗尽后 `len<<2 <= buf+0x18` 才 `update_buffer_bytes`。
Rust：事件/断言/写色/命中门/AABB/石膏分类（含 unsigned 判别等价）逐语句一致；SSBO 上传段缺失（`_library` 未用）；记录 selector/value 置 0（binary selector 候选=is_halftimbered 返回，value 为未解字节/U）。**verdict：需修**。

### 4.3 `set_gate_color_id` 0x142299840 — 需修
fresh：事件 stride 0x18；**先** `PublicWalls::get` 直到非空→idx 断言（0x4fff）→写 `GateColorIds[idx]`→tick；`event+0x14==1`→AABB→0x20B kind 7；无石膏；尾部 SSBO 上传。
Rust（`set_gate_color_id.rs:25-58`）：`if let Some(wall)` 门控后写色+custom==1→tag7，逐语句一致。SSBO 缺、selector U。**verdict：需修**。

### 4.4 `set_halftimber_color_id` 0x142299250 — 需修
fresh：逐事件**无条件**写色（idx 断言）；`event+0x14==1 && PublicWalls::get!=0`→AABB→kind 0xc；无石膏。
Rust（`set_halftimber_color_id.rs:24-55`）：无条件写 + `custom==1` 且墙命中→tag 0xc，逐语句一致。SSBO 缺。**verdict：需修**。

### 4.5 `set_decorator_color_id` 0x1422af0c0 — 需修
fresh（`decomp/set_decorator_color_id.c`）：5 层顺序事件链（stride 0x18/0x18/0x18/0x10/0x18）。
- L1（`SetDecoratorColorIdCmd`）：lookup **unwrap**→`info+0x28=event+0xc`；`sub=(u8)(ty-0x38)；7<sub→6`；`sub==7`→逐连接节点写 +0x28（节点 lookup unwrap）；custom `event+0x10 != 0`→tag=10；`0xcf>>sub&1==0` 时 sub==4→tag13（value=event+0xc）、否则 tag8（value 为栈 U）；记录 selector 为栈 U。
- L2（stair？实际是 brightness/灯笼，`<`）：`type=='<'`→写 lantern+0；custom==1→tag14，value=event+0xc。
- L3（梯子 `?`，stride 0x18）：节点 `stair+8=event+0xc`；custom==1→tag10，value 栈 U。
- L4（扶手 `?`，stride 0x10）：节点 `stair+0x10=byte(event+0xc)`；custom==1→tag10，value 栈 U。
- L5（魔法 `?`）：节点 `stair+0xc=event+0xc`；custom==1→tag10，value 栈 U。
Rust（`set_decorator_color_id.rs:31-176`）：5 层顺序、lookup 判空策略（L1 unwrap、L2-5 if-let）、写偏移（+0x28/+0/+8/+0x10/+0xc）、sub 钳制、tag 位测试、custom 极性全部逐语句一致。**具体分歧**：L1 非 sub4、L3/L4/L5 的 record value 在二进制是未初始化栈值（U），Rust 取事件值（确定值）；L1 selector 为栈 U，Rust 0。属 U 面行为分歧（不能证等）。**verdict：需修**。

### 4.6 `set_flag_connection_and_chimney_smoke` 0x142298970 — 需修
fresh（`decomp/set_flag_connection_chimney.c`）：两事件源均 stride 0x10；flag 先：lookup unwrap→`flag+0=event+0xc`（byte）→`event+0xd!=0`→tag9；烟后：unwrap→`chimney+4=event+0xc`（byte）→`event+0xd==1`→tag0xf。记录构造为 `CONCAT11(value, kind)`：**+8=kind（9/0xf）、+9=selector=event+0xc**；其余为栈 U。
Rust（`set_flag_connection_n_chimney_smoke.rs:24-66`）：flag 写 + tag9、烟 custom==1 + tag0xf 判等；**具体缺失**：烟 `ChimneyExtra+4` 未写（core 仅 4B）；**新发现**：两记录的 selector 字节二进制取 `event+0xc`，Rust 置 0（`value: 0.0` 亦与二进制栈 U 不同）。**verdict：需修**。

### 4.7 `set_roof_customization` 0x142299dd0 — 未实现
fresh（`decomp/set_roof_customization.c` L325-377）：外环 `roof_entity` 返回 {idx,key}→墙表 stride 0x14 key 校验→roof idx@+4→dense 位图 `+0x28`（`>> (idx&0x3f) &1`）→chunk 表 `+0x1a8` stride 0x48→`roof_elem = base + 0x10 + ~(u64[+0x38+global*8])*0x30`；写入 **两处**：
`*(u32*)(color_table_base + idx*4) = uVar4`（第二写）与 `*(u32*)(roof_elem + 0x50 + idx*0x58) = event+0x10`；custom `event+0x14==1`→`pivot_3d`→tag 0xb。另有样式比较环 + `History::add`（0x50B `{1,1,4,...}`）在 L~160-230。
Rust（`set_roof_customization.rs:29-58`）：仅 `walls.roof_entity`+`roofs.get`+`observed[0x50..0x54]=color`+custom→tag0xb；**第二写、密集索引、样式环/History 全缺**（PublicWalls 为 HashMap 抽象）。**verdict：未实现**。

### 4.8 `show_color_options` 0x14229a7f0 — 需修
fresh 证据：入口（`r2-entry.txt`）事件 stride **0x20**（`shl r12,5`）；两段跳表 0x14303ca70 40×dword，base=表地址，entry0→0x14229b725、entry1→0x14229bde9。
- case0 demo 门：`mov rax,[0x14362c098]; cmp byte[rax],0; je 0x14229cc37` → 非 demo 路径在 0x14229cc37；demo 路径建 bricks_grey/plaster/locked/roof_* 串。
- case0 非 demo（`r2-case0-body.txt` L1-140 逐字段）：13 条 kind=3，逐记录 v1c/v24 =
  `0/0, 1/0, 3/0, 0xb/0, 2/0, 4/0, 0xc/7, 5/1, 10/6, 8/4, 9/5, 6/2, 7/3`
  —— 与 Rust（`show_color_menu.rs:78-91`）**逐值相等**；plaster 门 `lea ecx,[rax-4]; cmp ecx,7; jae`（+0xc 分支）→ separator(kind 5, "white", +0x24=0) + plaster_peel_low/medium/high；Rust `push_case0_plaster_peel` 一致。
- halftimber/skirt 门与 flat-roof 门在 case0 后续段（Rust `halftimber_count<2` / `has_flat_roof` 近似承载，源未证）。
- case1 非 demo 0x14229d754：首项 kind=3、v1c=8、v24=0（roof_ochre），与 Rust `build_roof_color_options` 11 项首值一致。
**结论**：Rust 的 case0/1 载荷（含原 FIX-B2 的 v24 副值）经 fresh 反汇编**确认正确**；仍缺 case2..0x26、尾部 6+ 事件写面、demo 分支、门源精确值（get_instruction/+600/eye offset）。**verdict：需修**。

### 4.9 `hide_color_menu` 0x1422adcf0 — 未实现
fresh（`decomp/hide_color_menu.c`）：`DigitalActionState::just_pressed`（L177）+ `HideColorMenuCmd` 双门；命中后逐图标 subtype switch：多处 `*(u32*)(record+0xc+idx*0x18)=uVar67`（L500/543/559/576/625/643）与 case 0xc；`insert<CustomizationIconMarkedForDeletion>`（L293/325）→`despawn`（L657/672）。
Rust（`show_color_menu.rs:204-226`）：仅按事件存在→despawn 全部菜单/图标并加 Marked；just_pressed 动作名 U、**逐 subtype 写面全缺**。**verdict：未实现**。

### 4.10 `autoclose_color_menu` 0x142298d30 — 未实现
fresh（`decomp/autoclose_color_menu.c`）：两事件源 stride **0x28**，任一 `*event==0xc`→直闭（L225）；图标世界位置（目录 stride 0xa0/0x48）→与光标 2D 距离 `size.y*2+size.x*1.2 < dist` 跳过；与 `eye_position` 3D 距离 `>10.0` 闭合（L154/196/158/200）。
Rust（`autoclose_color_menu.rs:30-50`）：kind 门/事件源布局/图标跳过均缺，以 `menu.center` 近似 3D 距离。**verdict：未实现**。

### 4.11 `ColorRadialMenu::create` 0x1425f27c0 — 未实现
fresh（`decomp/color_radial_menu_create.c`）：非分隔符计数→`AngleRange::from_min_and_len`（L153）→`size*0.75/0.25`（L208-209）→`cosf/sinf` 极坐标（L241-252）→`spawn<Ui2dExtent,Ui2dImage,CustomizationIcon,UiDrawOrder>`（L417）→按 tag 分派 `insert<ColorIcon/WallStyleIcon/.../FlatRoofStyleIcon>`（L439-664）。
Rust（`radial_menu/mod.rs:93-113`）：仅计数与字段组装，spawn/极坐标/分派全缺。**verdict：未实现**。

---

## 5. decorator（8 项：判等 1 / 需修 3 / 未实现 4）

### 5.1 `maintain_dormer_tiles` 0x1407ce920 — 判等
fresh（`decomp/maintain_dormer_tiles.c` L23-76）：`prev=*cursor`；`startA=prev-countA(@+0x18)`/`startB=prev-countB(@+0x38)` 各钳 0；`lenA=listA.len(@+0x10)-startA`（`len<start`→哨兵 8/len 0）；`*cursor=(total@+0x40-lenB)-lenA`；A[startA..]→B[startB..] 链式；每瓦：`*cursor+=1`、`*id_slot=+0x1c`（单槽）、`hash_one<&GlobalId>(elem+8)`+`RawTable::remove_entry`、返回值非空→drop `SparseInstanceBufferAllocation<RoofTileSsbo>`。
Rust（`maintain_dormer_tiles_draw.rs:111-158`）：saturating_sub、空列表分支、循环前预写 total-lenB-lenA、chain、四步逐语句一致。`id_slot` null guard 为 Rust 安全缝（binary 无条件 store，I）。**verdict：判等**（rhapsody 注册表生产者/RoofTileSsbo 宿主接线在函数外）。

### 5.2 `apply_decorator_op` 0x141276e30 — 需修
fresh（`decomp/apply_decorator_op.c` + `r2-apply-entry.txt`）：
- guard：`test rcx,rcx; sete dl; test dl,bl; jne panic` → panic 当 **archivist==None 且 (kind_byte&1)!=0**（不是任意非 0）。
- `lVar1 = (kind-3<2) ? kind-2 : 0`；0→change、1→place、2→remove。
- change：读 `op+0x10`；unwrap；archive；`op+0x10 != 3`→二次 archive；历史 Vec<0x20> push kind=3（**仅写首 qword**）+ `DecoratorLocator::new` + map insert；`apply_decorator_change`。
- place：archive→push kind=4+locator+insert→`apply_decorator_place`。
- remove：archive→push kind=5（无 locator）→`apply_decorator_remove`。
- 尾 `DeferredDecoratorOpEvents::send`。
Rust（`apply_decorator_op.rs:311-378`）：顺序、kind 3/4/5、locator 仅 place/change、二次归档条件、unwrap 镜像、send 全对应。
**具体分歧**：① guard 用 `kind_byte != 0`，二进制是 `kind_byte&1`（caller 实测 edx 来自字节标志，多为 0/1；非 1 偶数会分歧——低风险）；② 二进制历史记录只写 kind qword，f1/f2/f3 为未初始化，Rust 写 `f1=id`（U）；③ `archive_decorator_state` 为自写等价（core archivist 缺口）、0x88B event 载荷多处 U。**verdict：需修**。

### 5.3 `create_or_restore_decorator_backup` 0x1408238a0 — 需修
fresh（`decomp/create_or_restore_backup.c`）：`state=+0x282; +0x282=0`；`state!=0` 时：`==1`→CREATE，否则 `created(+0x281)!=0`→RESTORE；末尾 `+0x280`→clear。
- CREATE：`all_decorators` 逐桶 `get_or_insert_default`+`get`(未中 unwrap)+`tracked_clone_from`，三 map clone_from（+0x250/0x1f0/0x220），历史 Vec `spec_extend`，`+0x281=1`。
- RESTORE：入口读 `history+0x48`(pending) 为门；`all_storages` 逐 `get_mut`+`Decorators::clear`+反向 clone 三 map/Vec；`all_decorators` 逐 bucket `DecoMoveToStorage::new`+`tracked_clone_from`；`StairsState::restore_backup`；`state==1` 项且门控→清 0（L565-567）。
Rust（`backup.rs:91-184`）：状态读清、state 1/2 分派、CREATE tracked 拷贝、RESTORE 反向+stairs.restore_backup+`clear_modified=deferred.pending` 与 +0x48 门控对应。
**具体分歧**：辅助 0x30B 三 map（+0x1d0/+0x200/+0x230）反向拷贝未接线；`all_storages/all_decorators` 以 `iter_walls`/bucket.insert 近似；RESTORE 为任意 `state!=1 && created`（Rust 仅 state==2，实际状态域 {0,1,2} 内等价，I）。**verdict：需修**。

### 5.4 `decorator_record_history` 0x140817850 — 需修
fresh（`decomp/decorator_record_history.c`）：tick；`pending=*(+0x48); +0x48=0`；pending→`len=+0x10; +0x10=0`；逐 0x20 记录，`kind==6` break；`uv=__rust_alloc` 48B：`[0]=1,[1]=1,[2]=kind,[3]=f1(+8),[4]=f2(+0x10),[5]=f3(+0x18)`；`History::add(10)`；之后无条件 `+0x10=0`；`+0x30!=0`→memset(+0x18,0xff,len+0x11)、`+0x30=0`、重算 growth_left。
Rust（`deferred_decorator_history.rs:193-217`）：pending 读清、kind6 break、无条件清 records、`clear()` 等价 hashbrown 清空；**分歧**：`record_to_history_edit` 返回 `DecoratorHistoryEdit::default()`（`deferred_decorator_history.rs:183-185`），48B 的 kind/f1/f2/f3 全部丢弃，未写入 History 载荷。**verdict：需修**（循环判等、载荷未还原）。

### 5.5 `window_lights` 0x1407b5980 — 未实现
fresh（`decomp/window_lights.c`）：`deny_load` false/true→下界 0.51/0.44，上界 0.72；`lights_on=lower<=t && t<0.72`；写控制 +0x30（L212-225）；逐条目 `target!=lights_on`→`timer=2*rng`+target（L247-287）；窗实体收集：墙模式 `+0x108==0` 走 `+0xf8` 数组 +0xa0 段桶，屋顶走 0x48 段桶，`+0x110` 索引反查 0x10 步长子记录，缺失插 `{brightness=0, actual=target=lights_on}`、形状变化（`(A新||B新)&&local_f4==pl_var[1]`）→`brightness=1,target=1`（L291-650）；`retain`（L654）**先于**帧末；帧末逐条目 `timer>0→timer-=delta, <=0→0, ==0&&!NaN→actual=target`；`time_continuous_lerp`→brightness（L712-725）。
Rust（`window_lights.rs:109-154`）：阈值/灯态/随机重计时/retain 先于帧末/计时归零/亮度调用**逐语句对应**；但**窗实体收集主循环与形状变化更新整体缺失**，rng 种子/lerp 实参/retain 谓词/`GlobalWindowLightsControl` 布局为 U/近似。**verdict：未实现**。

### 5.6 `chimney_smoke_anim_thickness` 0x140818df0 — 未实现
fresh（`decomp/chimney_smoke_anim.c`）：初始插入 `(t<0.72||0.9<=t)&&spot+0xd==1 → 1.0 否则 0.0`（L383-384）；`retain`（L471-474）**在更新循环前**；`fVar53 = (t<0.72||0.9<=t) ? 1.0 : -1.0`（L477-480）；逐 spot：`init_rng`+抽签；`+0xd==0`→丢弃一次抽签、`target=r*3+(1-r)`、sign=-1.0；否则 `target=r*4+2(1-r)`、sign=±1；更新 `new = clamp01(old + sign*dt@+0x28 / target)`（target 字节重拼为除数，L633-644）；写表值+进度；尾部再处理。
Rust（`chimney_smoke_anim_thickness.rs:85-124`）：窗口/目标/符号/`old+sign*dt/target`/[0,1] 钳制**与 fresh 完全一致**（含 divisor=target 的关键结论）；但**spot 收集主循环与初始插入缺失**，且 **retain 顺序反了**（Rust 更新后 retain，二进制 L474 更新前）；rng 种子/条目 +0xd 承载为 U。**verdict：未实现**。

### 5.7 `replay_decorator` 0x14080c8d0 — 未实现
fresh（`decomp/replay_decorator.c`）：`sync_point_count`；`front_mut_bucket`（None→return）；桶 `+0x18/+0x20` Arc<[DecoratorId]> cull 表（指针置 0 取出）逐 id 入 HashMap，Arc drop；命令扫描 stride **0x30**，`do{}while(*p != 7)` 只处理 tag==7：`flags&1==0`→立即完成；否则 `required=duration/speed; if <=0.05 →0.05`；`required<elapsed`→完成记索引；未完成 `elapsed+=dt`（完成项也加）；完成索引反转→`Vec::remove`（0x30 memmove）+按 tag drop（0/1/2 Raster、3 Terrain、4 WallReplay、5 Roof、6/7 Decorator、8 Tree、9 CrashMisc、default PhysicalSize）。
Rust（`replay_decorator.rs:86-120` + `DecoratorReplayCmd::advance`）：模型公式/索引反转删除与 fresh 一致；但 `decorator_cmd_of` 恒返回 None（core `ReplayCommand` 无 Decorator 变体）、桶 cull 表字段缺失；运行期不处理任何命令，且 per-command cull insert 与二进制"桶级 Arc 表"语义不同（不可达）。**verdict：未实现**（诚实壳）。

### 5.8 `calc_gothic_half_dormer_profile_ws` 0x1407eb160 — 未实现
fresh（`decomp/calc_gothic_half_dormer_profile_ws.c`）：`__rust_alloc` 后 `0..2` 两次 `for_each(closure_env_0)`，闭包体被 Ghidra 删除（unreachable block 13 处），仅保留 bounds-check/handle_error 尾。
Rust（`gothic_wall_window.rs:77-127`）：`0..2` 两次 + 简式 `Vec3(u, h*max_y, 0)` + 语义截断；wall-space A/B/C/D lerp、法线、`inverse_lerp`、`0..2` 索引语义均未实现。**verdict：未实现**。

---

## 6. FIX 清单（针对 Rust 落盘；未改 src）

优先级：高=具体行为分歧；中=整段载荷/写面缺失；低=U/等价性证据。

| # | 级别 | 函数 | FIX |
|---|---|---|---|
| FIX-BS1 | 高 | `set_flag_connection_and_chimney_smoke` | 记录 selector 字节 = `event+0xc`（`CONCAT11(value,kind)`），Rust 置 0；烟 `ChimneyExtra+4` 写面按 core 类型补齐后落位（`set_flag_connection_n_chimney_smoke.rs:53-64`） |
| FIX-BS2 | 高 | `chimney_smoke_anim_thickness` | **retain 移到更新循环之前**（二进制 L471-474 在 L478+ 更新前）；补 spot 收集主循环与初始插入（`{t 窗口 && +0xd==1 → 1.0 else 0}`） |
| FIX-BS3 | 高 | `set_roof_customization` | 外环补第二写 `*(color_table + roof_index*4) = u32(in_R9+0x14)` 与 dense 位集/分块定位；补样式比较环 + `History::add` 0x50B |
| FIX-BS4 | 高 | `construct_timberframe_circle` | 主环朝向按 fresh L287-299（由 `params.main_height` 构造基/四元数）替换 `Quat::IDENTITY` |
| FIX-BS5 | 高 | `hide_color_menu` | 补逐图标 subtype switch（`+0xc+idx*0x18` 写面与 case 0xc）与 `just_pressed` 动作名 |
| FIX-BS6 | 中 | `apply_decorator_op` | guard 改为 `assert!(archivist.is_some() || kind_byte & 1 == 0)`（镜像 `test dl,bl`）；历史 48B 记录其余字段的初值语义需按二进制 U 登记（勿把 f1=id 当判等） |
| FIX-BS7 | 中 | `decorator_record_history` | 48B `{1,1,kind,f1,f2,f3}` 忠实写入 `History::add`（core 模型需扩展；当前 default edit 丢弃载荷） |
| FIX-BS8 | 中 | `create_or_restore_decorator_backup` | RESTORE 补辅助 0x30B 三 map 反向拷贝；桶模型差异登记 |
| FIX-BS9 | 中 | `window_lights` | 补窗实体收集主循环（缺失插 `{brightness=0,actual=target=lights_on}`）与形状变化 `brightness=1,target=1`；取证 rng/lerp 实参/retain 谓词 |
| FIX-BS10 | 中 | `generate_roof_stone_floor_and_roof_bottom` | 落 NaniBundle insert 执行体（0x38B 命令项 16B 数据） |
| FIX-BS11 | 中 | `show_color_options` | 补 case2..0x26 逐 case 表 + 尾部 6+ 事件写面；门源（get_instruction 索引、`+600` halftimber 计数、eye offset）取证；demo 分支登记 |
| FIX-BS12 | 中 | `set_wall_color_id` | SSBO `update_buffer_bytes` 段接线；selector 面取证（is_halftimbered 返回） |
| FIX-BS13 | 中 | `init_color_ids_buffer` / gate / halftimber | 4×`add_new<*ColorIdSsbo>` 与 SSBO 上传段待 core typed-gap 闭合 |
| FIX-BS14 | 低 | `set_decorator_color_id` | L1 非 sub4、L3/L4/L5 记录 value（及 L1 selector）在二进制为栈 U；Rust 取确定事件值须明确登记为建模假设（不可标判等） |
| FIX-BS15 | 低 | `apply_decorator_op` | build_op_event 0x88B 字段放置 U（消费端未复核） |
| FIX-BS16 | 低 | timberframe rect/circle | `RenderInstance` 逐轴尺寸合同（截面×段长）与 `rotation_from_normal` 落 utils 后替换单标量 `WoodenPlankSsbo.size` 与 `beam_rotation` 代理 |
| FIX-BS17 | 低 | `autoclose_color_menu` / `ColorRadialMenu::create` | 按 fresh 补 kind==0xc 直闭、图标 2D 跳过；`AngleRange/cos/sin/spawn 4 组件/tag-3 分派` |

**关闭项**：原 `FIX-B2`（show_color_options plaster swatch `+0x24` 副值 0,7,1,6,4,5,2,3）经本轮 fresh 反汇编逐记录核对**已正确落库**，关闭。

---

## 7. 分歧检查表 8 条扫描

| # | 检查项 | 结果 |
|---|---|---|
| 1 | panic-vs-return 反 | 命中 1 处：`apply_decorator_op` guard 二进制 `(archivist==0 & kind_byte)!=0`（bit0），Rust `kind_byte!=0`（FIX-BS6）。其余 unwrap/panic 面（timberframe 空槽、color idx/断言、record_history alloc）方向一致 |
| 2 | 门控极性反 | 未命中。show_color_options demo 门 `flag==0 → 非 demo`、plaster `(color-4)<7 || color==0xc`、chimney `t<0.72||0.9<=t`、wall/gate/halftimber 的 `!=0` vs `==1` 均与 Rust 一致 |
| 3 | 载荷平面近似 | 命中：`ColorRadialMenu` 仅数据组装；`generate_roof` 命令执行体 no-op；`decorator_record_history` 48B→default；`apply_decorator_op` 事件 0x88B U；`set_flag`/`set_decorator` 的 selector/value 以 0/事件值近似 |
| 4 | 事件/命令自造 | 未命中（无伪 insert/伪事件）。`ObservedStoneFloorCommand` 是显式 no-op stand-in，未冒充 NaniBundle；`DecoratorReplayCmd` 显式登记 core 缺口 |
| 5 | callee 转发错 | 未命中。`to_rgb`/`Aabb2::center`/`max_y`/`xvy`/`split_height_into_skirts`/`remove_entry`/`drop_in_place` 调用面与 fresh 一致 |
| 6 | 返回结构不符 | 不适用（全部 `void` 系统） |
| 7 | 排序取最近 vs 首个 | 未命中；`decorator_record_history` 取 pending、`replay` 索引反转均为二进制原样 |
| 8 | 偏移表/字段错位 | 命中 1 处已修：show_color_options swatch `v1c@+0x1c/v24@+0x24` 逐记录核对一致；其余 `set_decorator`（ty@+0x46、写 +0x28/+8/+0xc/+0x10）、`set_flag`（selector@+9）、`BeamCount+8`、`maintain`（+0x10/+0x18/+0x38/+0x40）均对上；`set_flag` selector 的**值来源**是 FIX-BS1（偏移本身正确） |

## 8. 证据索引

- `evidence/result-all.json`：Ghidra 24 函数 fresh（含 metadata）。
- `evidence/decomp/*.c`：24 份 fresh 反编译正文。
- `evidence/result-show.json` + `evidence/result-r2-show.json` / `result-r2-show2/3/4.json`
  + `r2-entry.txt` / `r2-case0.txt` / `r2-case1.txt` / `r2-nondemo.txt` / `r2-case0-body.txt` /
  `r2-case1-nondemo.txt`：show_color_options 入口/跳表/case0/case1。
- `evidence/result-r2-apply.json` / `r2-apply-entry.txt` / `result-r2-callers.json`：apply guard/caller edx。
- `evidence/result-r2-wall.json` / `r2-wall-tail.txt`：set_wall SSBO 尾块。
- 跳表 0x14303ca70 40×dword 实读（`result-show.json` read_memory，base+相对偏移解出 case0/1 目标）。

## 9. 残留限制

- `show_color_options` case2..0x26 未逐 case 反汇编（Rust 侧本就未实现，不影响 verdict）；demo 分支未逐指令核。
- `create_or_restore_decorator_backup` 的辅助 0x30B 三 map、`apply_decorator_op` 的归档叶（archivist）与 0x88B 事件消费端仍为 U，本轮只核到写面。
- 未修改任何 src；未对 Ghidra/ReVa 数据库做写操作。
