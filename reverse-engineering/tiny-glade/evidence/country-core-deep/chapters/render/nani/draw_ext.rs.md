# render/nani/draw_ext.rs

> 文件作用：NaniDrawExt：DrawBuilder 的 nani GPU-driven 间接绘制扩展（nani_draw 0x140904ae0，1428B：subset 读锁 → list_index=kind*4|变体 → 三个间接 SSBO 绑定 → count/raw 分派）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::render::nani::draw_ext::impl$0::nani_draw` | 0x140904ae0 | 19 | 9 步（O）：取 GpuWorld（type-hash 0x662978458699a5ef；缺→unwrap）→ subset 读锁（槽 None→unwrap）→ draw_count（Owned/External 变体 u64）→ list_index=kind*4|PASS_VARIANT_TABLE[pass_variant(+0xf0)]（0x302010000→[0,0,1,2,3]）→ list_index_to_cmd_offset（越界 panic_bounds_check）→ get_rg_buffer<NaniDrawListLength/NaniDrawList> + read_storage_buffer<NaniInstanceIndirectionSsbo> +（句柄有则）read_rg_storage_buffer(0x2106412d631790c6) → 偏移 cmd_slot*20 → count_path=FORCE_FINE(0x143634d9c)|  |(kind<=1&&!FORCE_NO_FINE(0x143634d9d)) 选 draw_indexed_indirect_count（max=MAX_DRAWS_PER_LIST[kind]，表 0x142aab288）或 _raw → 释放读锁 | O | render-top/decomp/decomp-0x140904ae0.txt（原 B_n_sys draw_ext §1（0x140904ae0）；B_i_nani §2；wp3/contracts/draw_ext.md） |

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `nani_draw`（0x140904ae0）—— GPU-driven 间接绘制 9 步
处理链（O，B_n_sys draw_ext §1）：
1. 取 `GpuWorld`（TypeId `0x662978458699a5ef`；**缺 → unwrap**）；
2. subset 读锁（槽 None → unwrap）；
3. draw_count（Owned/External 变体 u64；External → 0，U）；
4. **`list_index = kind*4 | PASS_VARIANT_TABLE[pass_variant(+0xf0)]`**（立即数 0x302010000 右移 8×variant 取低字节 → **[0,0,1,2,3]**）；
5. `list_index_to_cmd_offset`（= params 第二个 Vec `max_instance_offsets[list]`，**越界 panic_bounds_check**）；
6. 取两个 RG 缓冲：`get_rg_buffer<NaniDrawListLength/NaniDrawList>` + `read_storage_buffer<NaniInstanceIndirectionSsbo>`（槽名 "NaniInstanceIndirectionSsbo"@0x142c8151d、binding hash 0xcdf9c36a6a9c8c80）+（有句柄则）`read_rg_storage_buffer(0x2106412d631790c6, handle.resource)`；
7. 偏移 = `cmd_slot*20`（步长 0x14）；
8. **count/raw 分派**：`count_path = FORCE_FINE(0x143634d9c) || (kind<=1 && !FORCE_NO_FINE(0x143634d9d))` → count（`max_draw_count = MAX_DRAWS_PER_LIST[kind]`，表 0x142aab288 27×u64；偏移 +0x50/+0x58/+0x60/+0x68 = cmd_slot*20/0x14/list_index*4/max）或 raw（r8=cmd_slot*20、r9=0x14、[rsp+0x20]=draw_count）；
9. 释放读锁。
- 数据流：`draw_lists` 回调（env 闭包）在 draw 终结操作前调 `nani_draw_world(b, kind, &GpuWorld, &AssetSsboLibrary, &mut RenderGraph)` → 此处绑定 3 个 SSBO（DrawListLength/DrawList 经 RG、InstanceIndirection 经槽）→ `draw_indexed_indirect_count/raw` 终结。
- U：External subset 的 draw_count 来源；Owned 实例间接缓冲 RG 句柄（External → None）。

## 证据注记（tg_rn1）
- **PASS_VARIANT_TABLE**（O）：立即数 0x302010000 右移 8*pass_variant 取低字节 → [0,0,1,2,3]。
- **槽名/哈希**（O）：`read_storage_buffer<NaniInstanceIndirectionSsbo>` 槽名 "NaniInstanceIndirectionSsbo"（0x142c8151d）、binding hash 0xcdf9c36a6a9c8c80；`read_rg_storage_buffer(0x2106412d631790c6, handle.resource)`；句柄 = `owned.data().rg_handle()`（External → None，draw_count 仍 U=0）。
- **count/raw 分派**（O）：count 路径偏移 +0x50/+0x58/+0x60/+0x68（cmd_slot*20 / 0x14 / list_index*4 / MAX_DRAWS_PER_LIST[kind]）；raw 路径 r8=cmd_slot*20、r9=0x14、[rsp+0x20]=draw_count；`max_draw_count` 表 0x142aab288 27×u64 实读。
- **A4 缺口**（B_i_nani §2）：`PassHost` 没有 World 资源访问器，`GpuWorld`/`AssetSsboLibrary`/`RenderGraph` 仍以显式参数传入；`draw_*` 现为终结操作所以 `nani_draw` 返回 `()`。
- U：External subset 的 draw_count 来源；Owned 的实例间接缓冲 RG 句柄（原 `OwnedIndirectionHandle` 占位已删）。
