# render/nani/draw_ext.rs

> 文件作用：NaniDrawExt：DrawBuilder 的 nani GPU-driven 间接绘制扩展（nani_draw 0x140904ae0，1428B：subset 读锁 → list_index=kind*4|变体 → 三个间接 SSBO 绑定 → count/raw 分派）。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::render::nani::draw_ext::impl$0::nani_draw` | 0x140904ae0 | 19 | 9 步（O）：取 GpuWorld（type-hash 0x662978458699a5ef；缺→unwrap）→ subset 读锁（槽 None→unwrap）→ draw_count（Owned/External 变体 u64）→ list_index=kind*4|PASS_VARIANT_TABLE[pass_variant(+0xf0)]（0x302010000→[0,0,1,2,3]）→ list_index_to_cmd_offset（越界 panic_bounds_check）→ get_rg_buffer<NaniDrawListLength/NaniDrawList> + read_storage_buffer<NaniInstanceIndirectionSsbo> +（句柄有则）read_rg_storage_buffer(0x2106412d631790c6) → 偏移 cmd_slot*20 → count_path=FORCE_FINE(0x143634d9c)||(kind<=1&&!FORCE_NO_FINE(0x143634d9d)) 选 draw_indexed_indirect_count（max=MAX_DRAWS_PER_LIST[kind]，表 0x142aab288）或 _raw → 释放读锁 | O | B_n_sys draw_ext §1（0x140904ae0）；B_i_nani §2；wp3/contracts/draw_ext.md |

## 证据注记（tg_rn1）
- **PASS_VARIANT_TABLE**（O）：立即数 0x302010000 右移 8*pass_variant 取低字节 → [0,0,1,2,3]。
- **槽名/哈希**（O）：`read_storage_buffer<NaniInstanceIndirectionSsbo>` 槽名 "NaniInstanceIndirectionSsbo"（0x142c8151d）、binding hash 0xcdf9c36a6a9c8c80；`read_rg_storage_buffer(0x2106412d631790c6, handle.resource)`；句柄 = `owned.data().rg_handle()`（External → None，draw_count 仍 U=0）。
- **count/raw 分派**（O）：count 路径偏移 +0x50/+0x58/+0x60/+0x68（cmd_slot*20 / 0x14 / list_index*4 / MAX_DRAWS_PER_LIST[kind]）；raw 路径 r8=cmd_slot*20、r9=0x14、[rsp+0x20]=draw_count；`max_draw_count` 表 0x142aab288 27×u64 实读。
- **A4 缺口**（B_i_nani §2）：`PassHost` 没有 World 资源访问器，`GpuWorld`/`AssetSsboLibrary`/`RenderGraph` 仍以显式参数传入；`draw_*` 现为终结操作所以 `nani_draw` 返回 `()`。
- U：External subset 的 draw_count 来源；Owned 的实例间接缓冲 RG 句柄（原 `OwnedIndirectionHandle` 占位已删）。
