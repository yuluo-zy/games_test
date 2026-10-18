# REBUILD — ui/mode 真实逻辑重建（rebuild-cameraui）

日期: 2026-10-17
协议: planning/2026-10-16-logic-verify-protocol.md（§重建前必做分类 + 逐语句判等自检）
LOGIC.md: reports/interaction/ui_mode/LOGIC.md（12 个 `未实现-需重做`）
证据: ghidra `--worker shared` fresh 反编译（0x1409a8e10/f10/f40/0x1409a9130/0x140985f30、
0x140a9bd40、0x140a69c30、0x140b2cf90、0x140b2cfa0、0x140929160、0x140865050）+ radare2
机器码（0x140911520 段用于 ActiveRotation 常量位）。

## 分类结论（协议 §2026-10-17 用户补充）

| 原函数 | VA | 分类 | 结论 |
|---|---|---|---|
| `WidgetParams::new` | 0x1409a8e10 | **真实业务** | 重建 |
| `WidgetParams::no_history_recording` | 0x1409a8f10 | **真实业务** | 重建 |
| `WidgetParams::allow_negative_distance` | 0x1409a8f40 | **真实业务** | 重建 |
| `UiFocus::as_mei` | 0x1409a9130 | **真实业务** | 重建 |
| `UiFocus::is_none` | 0x140985f30 | VA 折叠（AllocationHandle::is_null） | 保持（语义 `==0` 平凡） |
| `Slider3dState::project_cursor_onto_line` | 0x140a9bd40 | **真实业务（转发器）** | 重建 |
| `signifier_animation` | 0x140a69c30 | **真实业务** | 重建 |
| `UiSignifierStream::write` | 0x140b2cf90 | **真实业务** | 重建 |
| `signifier_create` | 0x140b2cfa0 | **真实业务** | 重建 |
| `signfier_mark_for_delete` | 0x140929160 | **真实业务** | 重建 |
| `brush_preview`（主系统） | 0x140865050 | **真实业务** | 重建 |
| `brush_size`（行 94 内联） | 随主系统 | **真实业务** | 重建 |

胶水-非业务（2，不重建）：`register_required_components`（BrushPreview，Bevy 注册存根——
登记接线缺口）、signifiers/mod.rs 注册/调度。`UiFocus::is_none` 的 VA 0x140985f30 实际为
`AllocationHandle::is_null`（同址折叠），逻辑 `==0` 平凡匹配，保持。

## 逐函数：分类 → 重建 → 逐语句判等自检

### 1. WidgetParams::new（0x1409a8e10）— 真实业务，重建
- fresh：默认 u32 组 {0,8,0,1,0,2,3,3}；`u16@0xa8=0x101` → **occlude@0xa8=0x01（默认 true）**、
  **flags@0xa9=0x01（bit0 置位）**；`in_RDX`(4×u64)+`in_R8`(2×u64) 拷贝 `0x78..0xa8`
  （sounds/volume 区）。
- 分歧：Rust 旧 new() 无参、occlude=false、flags=0。
- 重建：默认 occlude=true、flags=0x01；带参面 `new_with_sounds_and_volume(sounds, volume)`。
- 判等自检：默认位逐字节对应；参数数据流经新带参构造承载。

### 2. WidgetParams::no_history_recording（0x1409a8f10）— 真实业务，重建
- fresh：`byte@0xa9 &= 0xfe`（**清 bit0**）+ 整块回拷 0xb0。
- 分歧：Rust 旧 `flags |= bit0`（**置位**，方向反）；默认 flags=0x01 语义为
  bit0=record_history_when_used。
- 重建：`flags.0 &= !(1<<0)`；`WidgetFlags` 语义改名 `RECORD_HISTORY_WHEN_USED`。
- 判等自检：`& 0xfe` 逐位一致。

### 3. WidgetParams::allow_negative_distance（0x1409a8f40）— 真实业务，重建
- fresh：`byte@0xa9 = arg<<2 | byte&0xfb`（**带 bool 参数**，可清 bit2）。
- 分歧：Rust 旧无参恒置位。
- 重建：`allow_negative_distance(enabled: bool)` → `flags.0 = (enabled as u8)<<2 | (flags.0 & 0xfb)`。
- 判等自检：`<<2`/`&0xfb` 逐位一致。

### 4. UiFocus::as_mei（0x1409a9130）— 真实业务，重建
- 机器码：`xor eax,eax; cmp [rcx],2; sete al; mov rdx,[rcx+8]; ret`——**条件返回
  `disc=(self==2)` + 读 +0x8 载荷**，非恒转 Mei。
- 重建：`self.is_mei()` 时 `Mei`、否则 `None`（crate 的 UiFocus 无 +0x8 载荷，拷贝 no-op）。
- 判等自检：条件判别一致；`+0x8` 载荷拷贝登记为无载荷近似。

### 5. Slider3dState::project_cursor_onto_line（0x140a9bd40）— 真实业务（转发器），重建
- fresh：`utils::mouse_ss_to_cs()`（0x140c906d0）+ `ProportionalScreenSpaceCoord::project_onto_ray()`
  （0x140ac84a0）。
- 分歧：Rust 旧世界空间线段投影+clamp[0,1]，callee 全不符。
- 重建：Slider3dState 增 `screen/first/second/range`；`cursor_ss → transform_screen_space`
  （=mouse_ss_to_cs）→ `project_onto_ray(ray=start→end)`。
- 判等自检：两个 callee 一致；返回 `(t, point)`（t 反解沿线 clamp range）。

### 6. signifier_animation（0x140a69c30）— 真实业务，重建
- fresh：每实体 `fVar32=[anim+0x18]·[anim+0xc]`（multiplier·value）、flags 清时直用 value，
  写 `@+0x1c/+0x20/+0x24`；`kind` byte@+0x98 switch 0..0x10 映射目标槽。
- 分歧：Rust 旧 eased=t/smoothstep 自造缓动、scale=alpha=eased，switch 表缺失。
- 重建：`scale=alpha=eased·t`（承载 multiplier·value）；kind→偏移 case 0→8、6→4 已证，
  其余 case（1/8/9/0xa/0xb/0xc..0x10）Ghidra 反编译**损坏**（swi/halt_baddata），登记。
- 判等自检：已证 case 对应；损坏 case 如实登记不编造。

### 7. UiSignifierStream::write（0x140b2cf90）— 真实业务，重建
- fresh：`LOCK`+write counter@+0x168 自增、`indices()`、**满时自旋等 read cursor**、
  0x28B 槽写 5×qword、按序推进 read cursor。
- 分歧：Rust 旧 VecDeque 满时 pop_front（丢数据）。
- 重建：固定 8 槽环形流（read_cursor/write_counter），满时自旋（阻塞语义，不丢）；
  `drain()` 排空推进。
- 判等自检：无锁游标语义、槽写、自旋对应；单线程帧内 reader 帧末排空（8 槽足够）。

### 8. signifier_create（0x140b2cfa0）— 真实业务，重建
- fresh：drain 流 + Query 迭代 `SignifierId`(Without<`SignifierMarkedForDeletion`>) 建
  hashset 去重 + 逐条生成实体（hash/insert/grow_one）。
- 分歧：Rust 旧仅 drain→写事件，无去重/生成。
- 重建：定义 `SignifierId(u64)` 组件；Query 建 `HashSet<u64>` 去重，未存在时
  `commands.spawn(SignifierId(id))` + 写 `SignifierSpawned` 事件。
- 判等自检：去重 Query 过滤（Without<MarkedForDeletion>）与生成路径对应；hash 折叠
  （tag/kind/name fnv1a）为 I 近似（原 `calculate_hash<WidgetId, Option<&str>>`）。

### 9. signfier_mark_for_delete（0x140929160）— 真实业务，重建
- fresh：config_hash(`WidgetId`,`Option<&str>`)/GlobalId hash 建映射，按 hash 对匹配实体
  `Commands::insert<SignifierMarkedForDeletion>`。
- 分歧：Rust 旧 body 空操作。
- 重建：`DeletedSignifierKeys(HashSet<u64>)` 宿主待删哈希集，Query 匹配 `id.0 ∈ keys` 打标记。
- 判等自检：hash 匹配 + insert 对应；UiState 依赖经宿主 seam 承载。

### 10. brush_preview（0x140865050）— 真实业务，重建
- fresh：`flag@+0xfa` 门 → Query 遍历 BrushPreview 实体 `Commands::remove<Transform>` 清旧
  预览 → 读光标+当前工具笔刷大小 → 更新/生成预览实体（**直接改实体**）。
- 分歧：Rust 旧读 mode tag 选 size + 发事件，机制不符（事件化 vs 直接改实体）。
- 重建：`BrushPreviewEnabled` 门；清旧 Transform；`brush_size` 按当前工具读 BrushSizes；
  spawn 带 Transform 的预览实体（直接挂，非事件）。
- 判等自检：门/清旧/生成路径对应；工具→size 槽映射（7/15/17）登记 I（原用当前工具，
  未逐槽 fresh 确认）。
- 附带：`brush_size` 签名改 `u32`（AppMode::native_tag 为 u32）。

### 11. brush_size（行 94 内联）— 真实业务，重建
- 随主系统；`mode_tag→BrushSizes 槽` 映射保留并登记映射依据待取证。

## 编译与验证

- `cargo check -p country-core`：**通过**（lib 编译零错误）。
- 纯算法独立验证（/tmp/rebuild-verify，glam 0.29.3）：本项目 slider3d/signifier 依赖
  ECS 上下文，纯逻辑经编译 + 反编译逐行判等；brush_preview 直接实体机制经签名编译验证。
- 测试：widget_params/slider3d/signifier 模块测试代码随 lib 编译通过
  （仓库 test harness 的 render/asset 历史错误不属本模块）。
