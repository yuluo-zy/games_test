# 组C re-analyst RE-ANALYSIS — system-roof

日期：2026-10-16。agent：re-analyst-C（dbid 会话 `tg_xc1`）。
范围：`skeleton/B/crates/systems/roof`（26 文件 / 33 业务）。
证据落盘：本目录 `decomp/`（33 文件 = 13 fresh + 17 building-* 既有 .c 拷贝 + 3 evidence/native 旧 DB .c）。

## 0. 盘点结论（33 业务 VA 全覆盖）

| 类别 | 数量 | 说明 |
|---|---|---|
| O（结构/常量已证） | 19 | create_roof、delete_roofs、process_invalid_roof_notifications、mark_roof_for_deletion、replay_roof×2、update_roof_spatial、calculate_roof_trigger_volumes、wall_height_dt、roof_animation、switch_roof_type、place_roof_spires、simulate_roof_tile_for_audio、roof_record_history、assemble_roof_tiles、ui_edit_roof×2、add_roof_systems 等 |
| I（结构级，细节待补） | 12 | edit_roof、assemble_rectangular/circular 内部网格、ui_edit_roof_ridge_dir/length/focused 各 do_*、switch_roof_ridge_direction 等 |
| 仍 U | 2 | highlight_roof（PDB 标 inline）、ui_edit_roof_tip.rs（源图无业务条目） |

decomp 落盘：32/33 文件（create_roof/update_roof_spatial/generate_roof_stone_floor 为 evidence/native 命名 .c，其余 13 fresh + 17 building-* 既有）。

## 1. create_roof（0x1421aaea0，O）

- 旧 DB decomp（evidence/native 拷贝）+ 串/常量核对。参数面：param_1 = &mut 队列资源（§wall-constructor RE-ANALYSIS 9.1，双 0x40 步长数组 + progress@*res + total@+0x40）；param_2 = Query；param_3/4/5 = 三个 ResMut（tick@+0x1c，写回各自 [2]）。
- 主循环：队列 merge-walk（0x40 条目：+8=wall entity、+0x10/+0x14/+0x1c/+0x20/+0x28/+0x30/+0x38 形状数据）→ 每条目 `PublicWalls::get`×2（0x140a91ef0，源 create_roof.rs）→ **`if (wall+600 段计数 <= 1) → 继续`**（>1 才可造屋顶）→ `PublicWallState::max_y`(0x140b2a620)。
- 产出三事件 Vec（cap-first，grow 族 0x140562630/0x140562ab0/0x140562c30）：A stride 0x10 {u64,u64}；B stride 0x28（40B）；C stride 0x20 {entity, u64, entity, u8 tag(0)}。
- **panic**：`Was requested to create a roof on a wall that is not a shape: IsEnclosed::`（0x142ff74a8）+ payload `FUN_1421aae00`（IsEnclosed Debug 打印），panic_fmt 0x1428d9390；源 create_roof.rs。
- 常量区 0x142ff76dc：{1.0,1.0,1.0,0.0}（Vec3(1,1,1)+0）。

## 2. delete_roofs 族（O）

- **delete_roofs（0x14062f8c0，fresh 全解）**：param_1=&mut 资源对（lVar4=*[0]、lVar5=[1]）。lVar4+0xf8=roof id u32 数组 ptr、+0x100=end、+0x108=模式字节（==0 → 走 0xa0 步长 wall 条目路径；否则走 0x48 步长 roof 条目路径）、+0x118/+0x120=双游标。lVar5+0x100=0xa0 步长 wall 条目基、lVar5+0x1a8=0x48 步长 roof 条目基（与 update_roof_spatial 同资源）。
  - 对每 roof id：wall 条目 +0x94 = roof 索引 → 0x48 条目（+0x38=实体数组 ptr、+0x40=len、+0x18=某 0x30 步长数组基）→ `~id*0x30` 反向索引（§9.2）→ **`fVar2 == 0.0 && !NaN`（进度浮点=0）→ 发 `FUN_140d14770(&{entity,uVar6..9}, &'delete_roof.rs'@0x142a0c7a0)` 删除事件**（EventWriter 推 0x18 步长？不 — param_2=Events，事件负载 {entity,4×u64}）。
- **process_invalid_roof_notifications（0x14062eb80，既有 .c）**：无效屋顶通知处理。
- **mark_roof_for_deletion（0x14062f3d0，既有 .c）**：标记删除。

## 3. edit_roof（0x1421ab7c0，I→O 级）

- 既有 .c（19.6KB，旧 DB 未命名 FUN_）。参数面：两 ResMut（tick+0x1c）+ 事件读取（bevy Events 双游标模式：cur-prev 差 → 有效剩余 → 游标更新 = total-(r1+r2)，同 ALGORITHMS.md update_rt_world §2.1 模式）。读 0x142ff73xx 源串区（create_roof.rs 同区）。逐事件分发 edit 指令（tip/profile/height/ridge/eave 修改路径），写回 Roof 组件字段（+0x24/+0x28/+0x30/+0x34/+0x38 等）。**细节待 reviewer 用 fresh DB 复核参数名。**

## 4. replay_roof 族（O 状态机）

- **replay_roof（0x141a37ad0，既有 .c 6.9KB）**：replay 动作分发。
- **impl$0::process（0x1421b48e0，fresh 全解）**：param_1=&mut ReplayAction{+0x00: int 模式、+0x04: f32 时长、+0x08: &Roof 快照、+0x10: f32 进度、+0x14: u8 标志、+0x15: u8 完成标志}；param_2=f32 dt。**switch(roof_snapshot.tag@+0x10)**：
  - case 0/1（两种屋顶型）：按 param_1[0x14] 标志向 param_3（ResMut，0x40 步长 push {entity@0, +0x40→8, +0x14→0x10, +0x18→0x14, +0x20→0x1c, +0x24→0x20, +0x2c→0x28, +0x30→0x2c, +0x38→0x34, tag 0/1@0x3c}）或 param_4（0x18 步长 {entity, data, tag=1}）发创建指令。
  - case 2（动画屋顶）：`t = clamp01((progress+dt)/duration)`；≥1 → 完成标志；**屋顶参数 lerp**（fVar2*fVar24 + fVar23*fVar25 双快照插值）→ 0x40 步长条目推 param_5。
  - case 3/4（两事件型）：progress==0 时向 param_6/param_7 推 0x18 步长事件 {entity, +0x18 数据, 标志字节（case4 三标志）}；case4 收尾 `if (*param_1==1)` 进度推进。
  - 全部 grow 族：FUN_1405627b0（0x40）、FUN_140562db0（0x18）。

## 5. update_roof_spatial（0x1421a7710，O）

- evidence/native .c + 串。**`PublicWalls::roof_entity`**（0x140a922b0）取屋顶 → packed loc 校验（0x14 行 + 掩码位集）→ 0x48 roof 条目 + `~idx*0x30` 反向数组 → `PublicWalls::get`（0x140a91ef0，panic 源 update_roof_spatial.rs）→ `PublicWallState::max_y` 写 `[0x58 条目 +0x4c]` → **屋顶 +0x299 字节（WallStyle::is_halftimbered 0x1408c5670）变化检测 → 推 0x20 事件 {entity, packed, roof_entity, tag=2}**（grow 0x140562c30）→ `Roof::ty`（0x1408e3250）==0 → `Roof::pivot_3d`（0x1408e3220）→ 写 0x30 条目 +0x10/+0x14(+0.010? fVar20=_DAT_142a7c878)/+0x18。
- panic：`update_roof_height received a wall change event for `（0x142ff6d98）+ payload PublicWalls impl$11 partial_cmp（0x140b34ec0）。
- 屋顶几何比较用 `== 且 !NaN`（两 Vec2/浮点对相等门控，0x58 条目 +8/+0xc..+0x20）。

## 6. calculate_roof_trigger_volumes（0x1421a3220，O）

- building-roof-trigger/B-producer 既有 .c（71KB，PDB 命名）。调用：**get_roof_expanded_top_and_bottom（roof_visual_shape）×2、profile_curve_ws（roof_shape）、calc_roof_tip_center_ls（roof_ridge）、apply_roof_noise（roof_noise）** → 生成 RoofTriggerVolume{triangles stride 0x24、normals 0x24、uvs 0x18}（布局见 roofs-rt DIGEST）。
- 既有验收链：building-roof-trigger 全套（B-trigger-* 验证），本组不再重挖。

## 7. visual/assemble 族

- **assemble_roof_tiles（0x141a36700，fresh 1172 行）**：puffin scope（0x142eba60b/7e0 = 模块名串）→ 共享队列 walk（0x48 条目）→ `Roof::is_flat`（0x1408e3380）分支 → SSBO 注册表 `FUN_141209c40`（键 0x142c1a878 族）+ `(**(code**)(entry[1]+0x18))(entry)` → 稀疏实例缓冲追加（SIMD 瓦片变换，unaff_XMM6 字节族）→ 尾部 Arc/condvar 握手（FUN_140c81c70/140c81e10 + `Arc counter overflow`@0x142eab990）。调用 FUN_141a36210×2、FUN_141a3c350（瓦片网格助手）。
- **assemble_rectangular_roof（0x1421aeef0，71KB fresh）/ assemble_circular_roof（0x1421b3120，20KB fresh）**：矩形/圆形屋顶装配（closure_env$0<assemble_roof_tiles>），细节在落盘 decomp。
- **generate_roof_stone_floor_and_roof_bottom（0x1421a6d40，evidence/native .c，O）**：队列 walk（0x20 条目）→ 掩码位集判"应生成" → **随机数 `(float)func_0x000142923f50(0)` clamp [1.0, 5.0] → 整数瓦数**（1.0@0x142925904、5.0@0x142925988、守卫 4294967040.0@0x1429a07d0）→ `0x140833250(uVar12)`（HistoryChainIndex::inner 哈希内联）→ 0x38 步长命令项推入（{FUN_140dceea0, 16B 数据, tag=9, 0, entity, FUN_140dcfa80}，FUN_1428dacd0 reserve）—— **Commands/延迟命令队列（双 fn 指针命令）**。
- **place_roof_spires（0x142196e70，既有 .c 6.6KB）**：尖顶放置；**roof_snow（0x1421a2da0 既有）/ wall_height_dt（0x1421a2b60 既有）**：降雪/墙高差系统。
- **simulate_roof_tile_for_audio（0x1421a7090，fresh）**：`Roof::is_flat`+`Roof::pivot_3d` 门控；常量 0.1/15.0（0x1429cd06c）/-0.01（0x142a82410）/1.0。

## 8. roof_shape 未入重建 14 业务 —— 接口契约（reviewer 优先项，机器码级）

reconstruction/country-core/src/resources/roofs/roof_shape.rs 现为 Gap 占位（ty/is_flat 等返回常量），以下为实测契约（补 dp 差距）：

| 函数 | VA | 机器码契约 |
|---|---|---|
| `Roof::ty`（v3 名，行为=高度上界） | 0x1408e3250 | `movss xmm0,c; ucomiss xmm0,[rcx+0x30]; setb al` → **ty = c < height_01（含无序）**；c=0.1（roof_state.rs 恢复链证实） |
| `Roof::is_flat` | 0x1408e3380 | 同上 `setae` → **is_flat = c >= height_01（有序）** = !ty |
| `Roof::is_flat_w_no_rim` | 0x1408e3390 | `setae dl; cmp byte [rcx+0x3d],0; setne al; and al,dl` → **is_flat && rim_flag@+0x3d != 0**（标志语义待 roof_state 裁决：0x3d=1 为 no_rim 标记则结果=平顶无檐） |
| `Roof::is_gable`（0x1408e33b0，construct_walls 半木构后门控） | — | `movss xmm0,[rcx+0x30]; movss xmm1,[rcx+0x34]; cmpeqps(ridge==c2); cmpnleps(height>c1); andps; al&1` → **is_gable = height_01 > c1 && ridge_length_01 == c2**（c2=1.0 域） |
| `gable_sides`（旧 DB 名 "tiles"） | 0x1408e33e0 | `if (ridge@+0x34 == 1.0 && height@+0x30 > _DAT_142925984(0.1)): out={1, rim@+0x3c, rim\|2}` else {0}；rim 字节 +0x3c（注意：is_flat_w_no_rim 用 +0x3d） |
| `shape_dims`（0x1408e3820） | — | 高度>0.1 时：circle `t=(r-0.9)/7.1` 基 8.0；rect `t=(max(d1,d2)-2.25)/12.75` 基 9.0；`s=smoothstep(clamp01(t/0.4))`；**shape_h = s*12.0+(1-s)*base**；`h = height_01*12.0+(1-height_01)*0.4`；`out=min(h,shape_h)`；floor@+0x4c≠y 时：`y<floor ‖ shape_h+floor<=y → out=2`；else 分段 `(1-profile@+0x2c)*0.5 + profile*1.5 → 1/(…)`；`fVar9<0 → panic`（0x140d8b480，roof_shape.rs 0x142aa5190） |
| `rotation_yaw_radians`（0x1408e3b10） | — | circle：**atan2(-sin@+0x10, cos@+0xc)**（xor -0.0@0x1429258f0）；rect：**-π/2(0x142a10ff0) - angle(0x1416f3210(dims@+0x18))】** |
| `ridge_length_01_from_rectangle_ratio`（0x1408e3b60） | — | r=max(x/y,y/x)；t=clamp01((max(x,y)-2.25)/5.25)；s=smoothstep(t)；**结果 = 1.5*s + r**（截尾见落盘 decomp） |
| `roof_height_ws_from_wall_shape`（0x1408e3c70） | — | circle：t=curve((r-0.9)/7.1)，**h=lerp(4,10,t)/12**；rect：t=curve((max-2.25)/12.75)，**h=lerp(3,6,t)/12**；curve=FUN_140c95ef0（roof_shape.rs） |
| `get_height_01_unclamped_from_ws`（0x1408e4790） | — | 见 roofs-rt decomp（0x1429345a8=10.0、0x142934694=4.0 等） |

### 8.1 纠错登记（reviewer-C 实测确认）
- **roofs-rt/ALGORITHMS.md §5.3/§8 标 `_UNK_142934604 = 16.0` 错误**。字节直读（RNE 原版 = B patched 一致）：0x142934604 = `00 00 00 41` → LE 0x41000000 = **8.0f**（非 16.0 亦非 8.0 之外值）。实现侧 surface.rs/ridge.rs 圆基 8.0 与章节正文正确；shape_dims 中 `fVar9 = _UNK_142934604`（circle 高度基）佐证 **8.0**。同表其他：0x142934600=24.0f、0x142934628=45.0f、0x142934618=10.0(f64)、0x142934620=84.0(f64)、0x142934638=90.0(f64)。
- **0.1 阈值表**：0x142925984=0.1（is_flat/gable_sides/ty 族共用）；0x142925970=0.4；0x142925904=1.0；0x1429258d0=1.5；0x1429258d8=0.3；0x142925908=0.6。

## 9. 未覆盖登记

- highlight_roof：PDB 标 inline（无独立 VA）—— 以 ui_edit_roof/update_roof_spatial 内联实例为准，未单列。
- ui_edit_roof_tip.rs：sourcemap 无业务条目（内联/胶水），未深挖。
- edit_roof 参数名、assemble_rectangular/circular 内部网格坐标：I 级，fresh decomp 已落盘待 reviewer。

## 10. 跨域线索

- 共享队列资源（§wall-constructor 9.1）+ `~id*0x30` 反向索引贯穿 roof/wall 全部系统 —— 中央索引建议合并为统一契约。
- SSBO 注册表（FUN_141209c40 + {hash,ptr,len} 键 + 虚调取缓冲）贯穿 place_pillars/assemble_roof_tiles/half_timbering —— 对应 country-core asset_libraries/ssbo_library.rs + render/sparse_instance_buffer.rs。
- `Roof::is_flat/ty/gable_sides/pivot_3d` 为 country-core roof_shape.rs 缺口（§8 契约可直接落码）。
