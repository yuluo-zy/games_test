# REBUILD — roofs+stairs 15 函数（fresh 反编译重建/复核）

> restorer-roofs-stairs，2026-10-17。协议：`planning/2026-10-16-logic-verify-protocol.md`
> （含 8 类分歧检查表）与 `planning/2026-10-17-recovery-sop.md`。
> 方法：Ghidra `--worker shared` fresh `get_functions(decompiled_code,callees)` +
> `disassemble_function` 逐指令 + `read_memory` 常量实测（证据留存
> `recovery/agent-team/work/restorer-roofsstairs-fresh/`）。
> **只以 fresh 反编译为准；与 LOGIC/任务说明冲突处以机器码定案（见 §三）。**
> 只改 roofs/stairs 相关源文件；判等函数未动。

## 一、逐函数 verdict

| # | 函数 | 地址 | verdict | 本轮动作 |
|---|---|---|---|---|
| 1 | roof_noise::noise | 0x140a152c0 | **判等** | fresh 149 条指令逐条复核；独立数值 oracle 命中 |
| 2 | apply_roof_noise | 0x1412348f0 | **判等** | 27 条指令逐条（clamp/0.5/0.3/smoothstep/字段 +0x4） |
| 3 | get_height_01_unclamped_from_ws | 0x1408e4790 | **判等** | 79 条指令；依赖 noise 已判等 |
| 4 | get_roof_shape_from_pos_ws_clamped | 0x1408e4750 | **判等** | 20 条指令（max/min clamp → get_roof_shape t@XMM2） |
| 5 | roof_shape_at_intersection_y | 0x1408e3820 | **重建→判等** | y==floor 改**原始 28B 帧体**变体 `RawSection` |
| 6 | roof_uv_at_ws_pos | 0x1408e4040 | **部分重建 + 登记** | 修正 uv/point 输出；注册表两级表跨域登记 |
| 7 | roof_uv_and_point_at_ws_pos | 0x1408e4090 | **部分重建 + 登记** | 同上（1152B 全管线） |
| 8 | roof_ridge::get_inner | 无 fresh 地址 | **判等（结构）** | 地址错标注释已更正；`Self(self.0)` 首字段拷贝 |
| 9 | structural_graph::clear | 0x140869a30 | **判等（语义）** | fresh 确认地址/8 表；2 张 U 表登记 |
| 10 | disconnect_node_by_collider | 0x14086a4c0 | **重建→判等** | **插入端点改为邻接端点 b** + 断言测试 |
| 11 | MagicSupports::clear | 0x140aa8210 | **胶水-登记** | fresh 确认 = Arc/mesh writer 清空；注释更正 |
| 12 | reset_edge / force_rebuild | 0x140aa8a60 | **存储核 + 渲染登记** | 渲染写面登记；upsert 核保留 |
| 13 | StairsRemovedSupports | — | **判等** | 机器码确认 `HashMap<StairGraphEdge,()>` 全边键 |
| 14 | replay_stairs | 0x140af75b0 | **重建→判等** | **undo 语义源修正** + elapsed/装饰偏移修正 |
| 15 | 常量/下限 | — | **判等** | `0x142930d70 = 0.05`（read_memory） |

## 二、重建明细（fresh 反编译 → 控制流 → 逐语句）

### 1. roof_noise::noise 0x140a152c0 — 判等（未改）
- 机器链（逐指令）：`hash1(z0)=((z0*0x401>>6)^z0*0x401)*9`；
  `combine_partial=(((h>>11)^h)*0x8001 + x0*0x19660d)*0x19660d`；
  SIMD 角 = `finalize(combine_partial + [0x142ac7e10]={0x0ae135d3,0x0ae135d3,0,0})`；
  scalar 角 = `finalize((A+0x19660d)*0x19660d+0x0ae135d3)`，等价 seed 增加 `0x19660d²`。
  掩码实测 `0x9d2c5680/0xefc60000/0x007fffff`（0x142ac7e20/30/40），OR `0x3f800000`、
  ADDPS `−1.0`（0x14292e6b0/0x14292e6d0）。
- 插值：行内 `lerp(partial, scalar, sx)`、行间 `lerp(z0, z0+1, sz)`；
  **机器不递增 x0**，两角由 seed 差异表达——当前 Rust 一致。
- 数值 oracle（独立 Python u32/f32 复算）：`M² mod 2³²=0x17385ca9`、
  `SCALAR_SEED=0x2219927c`、`(0,0) u01=0.5487866401672363`（任务要求 ≈0.5488）✓。
- 测试：`noise_matches_machine_reference` 6 点全过。

### 2/3/4. apply_roof_noise / get_height_01 / clamped — 判等（未改）
- apply（27 指令）：`MAXSS 0 / MINSS 1`、`(3−2s)s²`、`x*0.5,z*0.5`、`n*0.3`、
  `y += …`（Vec3@RCX/+4）。常量实测 0.5/0.3/1.0/3.0。
- height（79 指令）：`noise(x*.5,z*.5)`、`y=pos.y+n*(−0.3)`（0x142925900 实测 −0.3）；
  cap 分支 `0.1<height_01`；circle `(r−0.9)/7.1,base=8.0`、
  rect `(sse_max(dx,dz)−2.25)/12.75,base=9.0`（常量 0x142aa5084/88/8c/90、0x142a7ba84、
  0x142934604 实测）；`t=min(1,max(0,dim/0.4))`、`smooth=(3−2t)t²`、
  `cap_base=smooth*12+(1−smooth)*base`、assert≥0、`cap_h=h*12+(1−h)*0.4`、
  `cap=min(cap_base,max(0,cap_h))`；尾 `inverse_lerp(floor,cap+floor,y)`。
- clamped（20 指令）：`MAXSS 0 / MINSS 1` 后以 XMM2 传 t 调 get_roof_shape（0x1408e4920
  entry `MOVAPS XMM0,XMM2`）——与 Rust `clamp(0,1)` 语义一致（NaN 保持 NaN）。

### 5. roof_shape_at_intersection_y 0x1408e3820 — 重建→判等
- y==floor（机器码 else 分支）：`out[0..8]=roof[8..0x10]`、`out[8..0x10]=roof[0x10..0x18]`、
  `out[0xc..0x14]=roof[0x14..0x1c]`、`out[0x14..0x1c]=roof[0x1c..0x24]` ⇒
  **out[0..0x1c] = roof[8..0x24] 全部 28B 原样**。
- 旧实现解码为 `RoofSection` 会丢 tag word 高 3 字节（roof[9..0xc]）与 circle 尾部
  roof[0x1c..0x24]。**本轮改为 `RoofIntersectionSection::RawSection([u8;28])`**，
  逐字节相等。
- 其余分支判等：`y!=floor`（NaN 也走此支，符合 f32 `==`）；`y<floor || cap+floor<=y`
  → OutOfBounds；否则 `t=(y−floor)/cap` → `surface::section`。cap 与 height 同构（已核）。

### 6/7. roof_uv_at_ws_pos / roof_uv_and_point_at_ws_pos — 部分重建 + 登记
- fresh 0x1408e4090（1152B）结构：
  ① `hash_one<RandomState,&GlobalId>` 探表（arg6+0x30 data / +0x38 mask / +0x48 len）→ 64b 句柄；
  ② 第二表决（arg5+0x3df8 data / +0x3e00 mask，bucket 0x20，槽字节≠0x11）+ arena arg5+0x3e10；
  ③ `ColliderSet::get_mut_internal` + `(index < len && gen==(generation|1))` 否则 panic；
  ④ 形状 cast（方向 `(0,−1,0)`、max_toi=`0x7f7fffff`、solid=1）→
     `rapier_hit_to_raycast_world_result`；
  ⑤ 判别 2 → 失败码 3（`*out=0x301`）；判别&1==0 → 失败码 4（`{1,4}`）；
     判别&1 → 成功：uv=`extra[0..2]`、point=result.position。
- 本轮修正 Rust：旧实现把 `hit.toi` 当 u、未判 `discriminant&1`；现改为
  `extra[0..2]` + `discriminant&1` 门控 + `position` 输出。
- **登记（未建模）**：GlobalId 两级注册表（+0x30/+0x38/+0x48 与 +0x3df8/+0x3e00/+0x3e10）
  与 6 参 ABI（RCX=out、RDX=key、XMM1=pos、arg5/arg6=ctx/world）未在 country-core 落库。
  当前用 `RaycastWorld::raycast(ws_pos, −Y, u32::MAX)` 覆盖 ③④⑤（全槽查询代替按键选
  collider）。**解锁条件**：collision/rt 域补 `GlobalId→handle` 表 + arena 后按键选 collider。

### 8. roof_ridge::get_inner — 判等（结构）
- fresh `find_functions("get_inner")` 无 RoofRidgeDims 实例；旧注释 0x1400885d0 属
  `decorator_storage::get`（错标）。函数语义 = 首字段拷贝，`Self(self.0)` 正确；注释已更正。

### 9. structural_graph::clear 0x140869a30 — 判等（语义）
- fresh 符号与函数体均为 `StairsStructuralGraph::clear`：
  8 张 hashbrown 表（表头 +0x18/+0x60/+0x90/+0xc0/+0xf0/+0x120/+0x150/+0x180）的
  `memset(ctrl,0xff)`+items/growth 归零，另清 +0x08 data/+0x10 len 的 0x30-stride 条目
  Vec（逐条 dealloc）与标量 +0x58。
- Rust 清 petgraph 图 + 4 显式映射 + edge_index，覆盖全部已建模集合。
- **U 登记**：+0x120/+0x180 两张表语义；`edge_index` 的原生落点（fresh 未见 +0x48 表头）。

### 10. disconnect_node_by_collider 0x14086a4c0 — 重建→判等
- 机器码（0x14086a585..a5a8）：`GraphMap::edges(node)` 输出 24B `(a=node@-0x38,
  b@-0x2c, &w@-0x20)`；`CMP byte [w],0` 为 0 跳过；`HashMap::insert` 的键取 `[R13] =
  [RBP-0x2c] = b（邻接端点）`；Vec 收集 `(node,b)` 后逐边 `remove_edge(node,b)`。
- petgraph 0.8.1 `Edges::next` = `(from, b, edge)`（`graphmap.rs:809`）⇒ b 是邻接端点。
- **旧实现插入查询节点 a，已改为 b**；新增 `clear_connections_for_test` 与断言
  （conn 含邻居 9、不含查询 5）。边权重 `StructuralGraphConnection` 未建模（`()` 恒过）登记。

### 11. MagicSupports::clear 0x140aa8210 — 胶水-登记
- fresh：两处懒初始化写锁控制器（alloc `{0,4,0,0}` + `Arc::downgrade`）→ LOCK →
  两次 `SparseInstanceBufferWriter::clear<BrickTransformSsbo>` → +0x10(u64)/+0x18(u32)/
  +0x40(u8) 归零 → unlock。**全属 render/rt 胶水**；Rust 只建计数占位并归零。
- **地址定案**：0x140869a30 是 structural_graph::clear（见 §9），MagicSupports 唯一地址
  0x140aa8210；LOGIC §5/任务说明的对应关系被 fresh 驳回。

### 12. reset_edge 0x140aa8a60 / force_rebuild — 存储核 + 渲染登记
- fresh：`hash_one<StairGraphEdge>` 探 edges（bucket 0x110），未命中 `reserve_rehash<…>`；
  主写面 = 2×`SparseInstanceBufferWriter::clear<BrickTransformSsbo>` + `MagicSupports::clear`
  + 实体 despawn + 以值返回重建的 0x110+B `StairsEdgeVisualState`。
- Rust 保留 upsert 核（`edges.insert(edge, default)`）；**渲染写面登记**。`dirty` 置位在
  remove_edge 有证据（+0x30），本函数 fresh 未见——保留为消费侧标记（U）。
- `force_rebuild` = reset_edge 语义（内联别名）。

### 13. StairsRemovedSupports — 判等
- fresh replay_stairs 类型实参：`HashMap::insert<StairGraphEdge,tuple<>>` /
  `RawTable::remove_entry<tuple<StairGraphEdge,tuple<>>>`；键 = payload+0x14/+0x18 两 u32。
- Rust `HashMap<StairGraphEdge,()>`、`record/erase/contains` 一致；`StairGraphEdge::new`
  排序键与机器存储边一致。

### 14. replay_stairs 0x140af75b0 — 重建→判等
- 命令 0x30B：`tag@0==10`、`flags@+8 bit0`、`duration@+0xc`、`payload@+0x10`、
  `elapsed@+0x18`、`undo@+0x1c`；payload 记录 `{判别字节@+0x10 bit0, edge1@+0x14, edge2@+0x18}`。
- **路径**：flags bit0==0 → 立即处理；==1 → `required=max(duration/speed,0.05)`，
  `elapsed<=required` 时 `elapsed+=dt` 继续，否则 `elapsed+=dt` 后处理（**就绪分支也加 dt**，
  0x140af77a5；旧 Rust 未加，已修）。
- **undo 语义（与 LOGIC/任务说明相反，机器码定案）**：两路径在 0x140af77b7/0x140af7829
  **都读 payload+0x10 bit0**（= StairsHistoryEdit 变体）：
  `0 (SupportsDelete)`：undo==0→insert、undo!=0→remove_entry（0x140af78b0/0x140af7896）；
  `1 (SupportsRecreate)`：undo==0→remove_entry、undo!=0→insert（0x140af7a5c/0x140af7a49）。
  命令 flags bit0 只选即时/动画，**不决定 undo**。
  实现：`StairsReplayCmd` 新增 `recreate`（`expanded` 由枚举变体填），replay 按 `cmd.recreate` 分支。
- 装饰过滤：机器读 `[info+0x41] | 0xf8 > 0xf9`（0x140af77da/784c）——
  Rust 旧读 `dst_payload>>8`（+0x11）为偏移错位，**改为 `DecoratorInfo::extra_style()`**。
- 其余判等：done 索引 Vec **reverse 后**逐条 memmove 移除（Rust `iter().rev()`）；
  tag==10 过滤；`lookup_decorator_info(edge1)`；`CachedDecoratorTransforms` 键 edge1；
  audio 事件载荷映射 U（登记）。

## 三、fresh 反编译驳回的既有说法（登记）

1. **0x140869a30 不是 MagicSupports::clear**。fresh Ghidra 符号与函数体均为
   `StairsStructuralGraph::clear`（8 表+Vectors）；MagicSupports::clear = 0x140aa8210。
2. **replay_stairs 双路径不按命令 flags bit0 反向 undo**。flags bit0 只选即时/动画；
   undo 方向由 payload+0x10 bit0（编辑变体）决定，两路径同一选择器（§二.14）。
3. `roof_shape_at_intersection_y` y==floor 必须是 28B 原样拷贝，不能用 t=0 截面/枚举近似
   （已改 RawSection）。
4. `disconnect_node_by_collider` 插入的是邻接端点 b，不是查询节点 a（§二.10）。
5. replay 装饰过滤字节是 DecoratorInfo+0x41（extra_style），非 +0x11。

## 四、U / 登记（写解锁条件）

| 项 | 地址/证据 | 解锁条件 |
|---|---|---|
| roof_uv 两级注册表 | 0x1408e4090 arg6+0x30/38/48、arg5+0x3df8/e00/e10 | collision/rt 域补 GlobalId→handle 表 + arena；6 参 ABI |
| SparseInstanceBuffer 写面 | reset_edge 0x140aa8a60、MagicSupports 0x140aa8210 | render::sparse_instance_buffer / RtMesh 接线 |
| structural_graph U 表 | +0x120/+0x180；edge_index 原生落点 | 装配/碰撞注册面取证 |
| replay audio 载荷 | 0x140af788d..7a76 | audio 域 PlayStairsAudioCmd 精确字段 |
| reset_edge dirty | +0x30（remove_edge 有据，reset_edge 无） | 消费侧脏字节取证 |
| 边权重 | StructuralGraphConnection（机器 !=0 判别） | 碰撞装配域类型 |

## 五、自检

- `cargo check -p country-core --lib`：**Finished，0 error**（workspace 他组 WIP 期间曾
  瞬态失败，均为 collision/save/ivy 文件；稳态复跑通过）。
- `cargo check -p country-core --lib --features native-physics`：**Finished，0 error**。
- 单测：
  - `cargo test -p country-core --lib roofs::` → **20 passed / 0 failed**
  - `cargo test -p country-core --lib stairs` → **24 passed / 0 failed**
  - `cargo test -p country-core --lib replay_stairs` → **3 passed / 0 failed**
  - `cargo test -p country-core --lib structural_graph` → **4 passed / 0 failed**
- 数值 oracle：`noise(0,0)=0.5487866401672363`（任务要求 ≈0.5488）；
  `0x19660d² mod 2³²=0x17385ca9`、`SCALAR_SEED=0x2219927c` 复算命中。
