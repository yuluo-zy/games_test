# REBUILD — walls 域返回轮：eraser finish / create_rectangle ridge / restore_decorators / get_building_and_roof_colors / unleash / id（restorer-walls-return，2026-10-17）

> 协议：`planning/2026-10-16-logic-verify-protocol.md`。方法：**只以 fresh 反编译为准**
> （`reverse-operator.py ghidra --worker shared`，`fields=decompiled_code,callees`）。
> 本轮 fresh 证据落在 `planning/impl-walls-return/decomp/`（batch1-fresh.json→batch1-result.json +
> 单函数 txt、batch1-cr-disasm-result.json、batch1-easing-result.json、batch1-readtable-result.json、
> batch1-unleash-xref-result.json、batch1-xref-result.json），全部由本任务轮实时生成（2026-10-17 14:26-15:15）。
> 复验常量：本次 `read_memory` 0x142aa50dc=**5.25**、0x1429258d0=**1.5**、0x1429df180=**-1.5**、
> 0x142934690=**6.0**、0x1429cd078=**3.0**、0x1429cd2d4=**12.0**、0x142aa5938=**SineInOut tag2**。
>
> 上一轮本任务在检查 CreateRoofCmd ridge 公式映射时被网关 400 瞬态中断；本轮恢复并完成 B/E/F，
> A/C/D 以 fresh 结构完整登记（跨域/类型边界，不做简化冒顶）。

## 改动文件（walls 域）

- `systems/wall/shapes/rectangle/create_rectangle.rs`：ridge_length_01 公式重建（**已重建**）+ 新增
  `rectangle_roof_height_01`（height_01 内联）+ CreateRoofCmd 字段修正。`cargo check` 过、
  create_rectangle 3 测试过、wall 全 82 测试过。
- `resources/walls/decorator.rs`：unleash / DecoratorOpEventKind::id 为**上一轮已重建**（14:52 落盘），
  本轮 fresh 复核判等，未再改动。

## 函数表

| VA | 函数 | verdict | 分歧点 / 重建依据 |
|---|---|---|---|
| 0x140a93790 | create_rectangle | **已重建**（ridge/height 公式 + 屋顶字段） | 见 §B；elevation/style 独立序 + get_building_and_roof_colors 接线待 D（登记） |
| 0x140a94880 | get_building_and_roof_colors | **未实现-需重做**（fresh 结构完整登记） | 见 §D；Rust 无对应 |
| 0x14155ba30 | restore_decorators | **未实现-需重做/跨域**（storage 核心已就绪） | 见 §C；collision RaycastWorld::remove_collider ×3 + send_state_diff_events 跨域 |
| 0x1408337c0 | unleash | **判等**（Rust 已重建） | tag==5 哨兵提前 return；drain 0x88→0x90 记录；见 §E |
| 0x140833570 | DecoratorOpEventKind::id | **判等**（Rust 已重建，fresh disasm 逐字节） | `disc∈{3,4}→R8=disc-2；跳表 {0x68,0x40,0x60}@0x142a92e78`；3→+0x40/4→+0x60/其余→+0x68；见 §F |
| 0x14092f990 | eraser finish（11509B） | **未实现-需重做/跨域** | 见 §A；需 history WallHistoryEdit 0xf8B 真布局 + ConcurrentEdits + 原生 archivist 3-map/ChildWalls |
| 0x14092e6e0 | WallEraserArchivist::new | 判等-注意（结构差异） | 原生 3 map（0x90B）；Rust 2 map 简化模型 |
| 0x14092e8e0 | record_rewiring_event | 判等-注意 | HashMap::insert + 旧值 drop |
| 0x14092eff0 | shrink_wall | **未实现-需重做** | map1 探针 → map3 ChildWalls 查找 → ChildWall::slice 改写 |
| 0x14092f280 | add_split_event | **未实现-需重做** | 同上 + ChildWalls swap_remove + slice + 重插 |
| 0x14092f630 | add_fully_erased_event | **未实现-需重做** | 同上 + 空桶重写 0x8000000000000000 |
| 0x14092ea50 | cache_if_original_wall | **未实现-需重做** | map1 已存在→return；map3 已存在→panic；克隆 curve/u32 → EraserWallState{1.0f64} 插 map2 |

## A. eraser finish（0x14092f990）— fresh 逐分支

`planning/impl-walls-return/decomp/14092f990-finish.txt`（11509B，fresh）。

1. **drain map2（EraserWallState，stride 0xe8）**：RawTable 迭代逐 entry，读 0xd8B 载荷 →
   组装 **0xf8B `WallHistoryEdit`** → 推入桶 `local_5b8`（grow_one<WallHistoryEdit>）。
2. **drain map3（ChildWall，stride 0x90）**，每 entry 两分支：
   - **shrink/split 分支**（`SBORROW8(0, children)` 为真）：0.2/height 断言 + NaN panic →
     `Curve::crashy_slice<Vec2,2D>` → ChildWalls `sort_by`（smallsort 1<len<0x15 / driftsort 大）→
     逐段中点 `fVar66 = cur[+8]·0.5 + prev[+0xc]·0.5`（[0,1] 断言 + NaN panic）→ 逐段
     `crashy_slice` + `inverse_lerp`×2（NaN/INF panic）→ clamp → 装配 WallHistoryEdit
     （桶 `local_570` / `local_558`）。
   - **全擦除分支**：len<2 或 height<=0 → panic；ChildWall Vec 排序；逐段改写记录
     （桶 `local_588` / `local_5a0`）。
3. **Sip13 hash**（`core::hash::sip::impl$5::write` 展开）→ inner walls map 探针（-0x78 stride）→
   命中则装配 0x70B ChildWall 改写记录。
4. **末段**：5 桶 Vec → 逐条 **Arc\<WallHistoryEdit\>** 包装 →
   `from_iter<ConcurrentEdits, Map<IntoIter<Vec<Arc<WallHistoryEdit>>>, new::closure_env_0>>` →
   `resources::history::History::add()`（tag 参数 0x7）→ **RawTable::clear ×2**
   （map1 RewireEnclosedWall + map2 EraserWallState）。

**verdict：未实现-需重做（跨域）**。Rust `finish_wall_eraser`（archivist.rs）为单遍
`record_wall_bulldoze` 转发（affected/ranges 简化模型），与原生 5 桶 WallHistoryEdit→
ConcurrentEdits→History::add 链结构不逐条对应。阻塞：① 原生 archivist 事件结构
（RewireEnclosedWall/EraserWallState/ChildWalls）Rust 无；② history 域 `WallHistoryEdit`
当前为 4 变体近似（`wall_history_edit.rs` niche 0x8000000000000004），非 0xf8B 真布局；
③ ChildWalls 重接线 = 宿主边界（REVIEW-DEEP 已登记）。**不做简化冒顶。**

## B. create_rectangle（0x140a93790）— ridge/height 公式已重建

`planning/impl-walls-return/decomp/140a93790-create_rectangle.txt` + `batch1-cr-disasm-result.json`。

**fresh 内联公式（逐步）**：
- `ratio = max(a/b, b/a)`（fVar67=a/b、fVar74=b/a、`if(b/a<=a/b) fVar75=a/b`）；
- `max_dim = max(a, b)`（`if(b<=a) max=a`）；
- `u = clamp01((max_dim − 2.25)/5.25)`（常量 2.25 立即数、**5.25 @0x142aa50dc**，NaN 分支
  `isnan(a)?b:max`）；
- `e = EasingFunction::SineInOut(u)`（apply 0x140562db0；tag2 @0x142aa5938 实测）；
- `height_01 = clamp01((6e + 3(1−e))/12)`（常量 6.0@0x142934690 / 3.0@0x1429cd078 /
  12.0@0x1429cd2d4）= **(1+e)/4** ∈ [0.25, 0.5]；
- `ridge = smoothstep(clamp01((1.5·smoothstep(u) + ratio − 1.5)/1.5))`（常量 1.5@0x1429258d0 /
  −1.5@0x1429df180）——与 `roof_shape::ridge_length_01_from_rectangle_ratio`（0x1408e3b60，reviewer-C 已判等）同式。

**屋顶请求（CreateRoofCmd 0x40B 记录，push 段）**：`tip_offset={0,0}`@+0x20、`profile=0.5`@+0x28、
`height_01=fVar62`@+0x2c、`ridge_length_01=fVar75`@+0x30、`eave=0.2`@+0x34、
`ridge_dir bool=(a<b)`@+0x38、`style=uVar50`@+0x39（uVar50 来自 roof 颜色查表，宿主接线）。

**已重建**（create_rectangle.rs）：
- `ridge_length_01_from_rectangle_ratio`：原 `(max_dim−2.25)/max_dim` 直接 clamp（错）→
  正确 5.25 分母 + smoothstep(u) + ratio blend + smoothstep(clamp)。
- 新增 `rectangle_roof_height_01`：`clamp01((6·SineInOut(u)+3(1−SineInOut(u)))/12)`。
- CreateRoofCmd：`(id, 0.5, height_01, ridge_length_01, 0.2, 0)`（原 1.0/1.0/ridge/0.0/0）。
- 测试重写为原生边界：正方形 4×4→ridge=0（ratio=1<1.5）；6×4→0.8977；max_dim≤2.25→u=0；
  height 6×4→0.4529、1×1→0.25、极大→0.5。

**登记（不冒顶）**：① 原生 `new_wall_w_params` 后**单独** `_try_update_state<update_elevation>`
（0x140aaaac0）→ `get_building_and_roof_colors`（0x140a94880）→ `_try_update_state<update_style>`
（0x140aaa350）→ `_try_update_state<update_wall_state_enclosure>`（0x140aa9610）的调用序，
Rust 现把 elevation/style 并入 new_wall_w_params 参数——需 D 落地后重排；② ridge_dir bool + style
字节（uVar50）宿主接线；③ `ev_new_wall`/`History::add`×2 + 2 事件写出未逐句重建（history 域）。

## C. restore_decorators（0x14155ba30）— fresh 结构，storage 核心已就绪

`planning/impl-walls-return/decomp/14155ba30-restore_decorators.txt`（4438B，fresh）。

**fresh 结构**：`immutable(+0x1b0)!=0 → panic`；迭代输入 diff map → 每 diff 记录：
`get_or_insert_default`（桶）+ 逐 decorator：
1. `hashbrown::remove_entry<DecoratorId,()>`（d2s 面）+ `HashMap::insert<DecoratorId,()>`
   （removed_ids 面）——**restore 方向先移除 d2s**；
2. `remove_entry<DecoratorId,DecoratorInfo>` + `history::decorator_edit::impl$1::from`
   （`DecoratorBeforeAfterWithStorage` 组装）→ `insert<DecoratorId,DecoratorBeforeAfterWithStorage>`；
3. **collision 域**：`RaycastWorld::remove_collider<DecoratorId>` ×3 + `Arc::drop_slow<RaycastMeshData>`
   （条件分支：before tag==3 / after tag==3 / 两者不等或 address 不同）；
4. 末段 `history::decorator_edit::send_state_diff_events<closure_env_0>`（closure=replay_pop_bucket）。

**verdict：未实现-需重做/跨域**。Rust `restore_decorators<F>`（decorator_storage.rs:1012）为占位
passthrough（`restore(self)`，无调用方）；`apply_storage_diff`/`restore_decorators_diff` 已实现
**DecoratorStorage 域**的 diff 应用（d2s 移除/removed_ids/桶 DecoratorInfo 回填/d2s upsert），与原生
①-② 语义对应。阻塞：③ collision `RaycastWorld::remove_collider` + `RaycastMeshData` Arc + 
`send_state_diff_events` 均属 collision 域类型，Rust walls 侧无；原生逐句写语义（to_info 的
`state=2` 等）已登记 U。**重建需 collision 域补 collider 类型后接入 restore_decorators_diff。**

## D. get_building_and_roof_colors（0x140a94880）— fresh 结构登记

`planning/impl-walls-return/decomp/140a94880-get_building_and_roof_colors.txt`（1101B，fresh）。

**输出（in_RCX，0x50B）**：`{u32 building_color@0, u32 c1@4, u32 roof_color@8, u8 flag@0xc,
u8 flag2@0x10, u64 pair@0x30, u64 pair@0x38}`。

**默认值来源（in_stack_00000040）**：building_color=[+0x1c]、c1=[+0x20]、flag=[+0x42]、roof_color=[+0x24]。

**查询链（in_RDX & 1 门）**：
1. map A（in_stack_00000030[0..1]，stride 0x2a8，key=GlobalId）探针 in_R8：
   - 记录：len@-0x290、height@-0x270、count@-0x48、flag@-7；
   - len<2 或 height<=0 → panic；count>1 → 默认值（uVar11=0）直接写；
   - 否则 `building_color = buildingColors[in_R8]`（in_R9 Vec<u32>）、
     `roof_color = roofColors[in_R8]`（in_stack_00000028 Vec<u32>），均带 bounds panic；
2. 若 map A[9] != 0：map B（in_stack_00000030[6..7]，stride 0x10）探针 in_R8 → value u64；
   high32!=0 → in_stack_00000038 内 0x14B 记录表（[8]+idx·0x14）→ 位图门（0x28+((idx>>6)·8)>>(idx&0x3f)）
   → 深层 0x48B 数组（0x1a8 偏移）→ 读 `c1`@+0x50、flag/flag2@+0x40/+0x41、64-bit pair@+0x41..+0x48；
3. map B miss → 回探 map A → `flag`@-7；miss → unwrap_failed。

**verdict：未实现-需重做**。Rust 无对应函数。依赖资源类型（map A 0x2a8B 记录 / map B /
buildingColors / roofColors / 0x14B 记录表 + 位图 + 0x48B 数组）非 walls 域现有类型，需
recovered_building 颜色库/asset 域布局锚定后重建。**create_rectangle 的 elevation/style 独立序
依赖此函数**（见 §B 登记①）。

## E. unleash（0x1408337c0）— 判等

`planning/impl-walls-return/decomp/1408337c0-unleash.txt`（444B，fresh）+ batch1-unleash-xref
（调用面 = `FunctionSystem<ResMut<DeferredDecoratorOpEvents>, EventWriter<DecoratorOpEvent>>`）。

**fresh**：`*(self+0x10 指向) = *(self+0x1c)`（BeVy Events writer 计数器，结构细节）→
`inner Vec len=0` 清空后逐旧元素：**tag@+0==5 → 提前 return（哨兵）**；其余拷贝为
**0x90B 输出**（tag@+0 + 0x80B 载荷@+8..+0x88 + 计数器@+0x88 = inner+0x40 递增号）。

**Rust（decorator.rs:838，已重建）**：`Mutex<Vec<[u8;0x88]>>` drain、tag==5 return、
0x90 record（`record[..0x88]=event; record[0x88..]=out.len()`）。逐条对应 ✓。
计数器以 `out.len()` 为序（无 BeVy Events 结构，登记 U）——**判等-注意**。

## F. DecoratorOpEventKind::id（0x140833570）— 判等

`batch1-readtable-result.json`（0x142a92e78 = {0x68, 0x40, 0x60}）+ **本次 fresh disasm**：

```
MOV RAX,[RCX]        ; disc (u64 @+0)
LEA RDX,[RAX-3]; ADD RAX,-2; XOR R8D,R8D; CMP RDX,2; CMOVC R8,RAX
                     ; disc∈{3,4} → R8=disc-2∈{1,2}；其余 R8=0
LEA RAX,[0x142a92e78]; MOV RAX,[RAX+R8*8]   ; table[R8]
MOV EAX,[RCX+RAX]; RET                      ; u32 @[RCX+offset]
```
→ disc==3→+0x40、disc==4→+0x60、其余→+0x68。**Rust（decorator.rs:779）match 逐条对应 ✓**。
与 DecoratorExtraInfo 判别纠偏（烟囱=4/窗=6/灯=7，REBUILD-PENDING 批 1 已重建）一致。

## G. ~240 待判等

`planning/impl-walls-pending/decomp/batch1..5/` + `impl-walls-remainder/decomp/batch5/` 已覆盖
decorator 族/inner_walls/public_wall_state/inner_wall_state/shapes-eraser 大部（REBUILD-PENDING 批 1-6）。
本轮起 `impl-walls-return/decomp/batch1/` 新 fresh 12 函数（本表）。**剩余大 UI 系统函数**：
`ui_rotate_circle` 0x1409ac910 / `create_circle` 0x14097e560 / `ui_create_circle` 0x14165bed0 /
`ui_edit_circle` 0x14165d460 / `show_full_circle_signifier` 0x141660880 / create_rectangle 事件循环前半等，
为多轮分批（每批≤20）工作，未在本轮完成，登记。

## 自检

- `cargo check -p country-core` 通过（仅既有 non_snake_case 警告）。
- `cargo test -p country-core --lib create_rectangle`：3 passed（新边界断言）。
- `cargo test -p country-core --lib decorator::tests`：6 passed。
- `cargo test -p country-core --lib wall`：82 passed。
- 未改他模块业务；共用只读 `--worker shared`，本任务无需写库。
