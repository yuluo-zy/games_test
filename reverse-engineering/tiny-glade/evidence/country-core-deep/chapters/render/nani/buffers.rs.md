# render/nani/buffers.rs

> 文件作用：nani 绘制参数：MAX_DRAWS_PER_LIST 27 项表（0x142aa14c0，与 max_instances 两张表逐值相同）+ NaniDrawListParams（0x70B = 4 Vec<u32> + 4 u32 汇总）+ NaniDrawListOffsetDefaults（0x1b0B = [u32;108]）+ 7 个 SSBO 载体。

函数总计 6：业务 2 / 内联无独立符号 4 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::render::nani::buffers::nani_calculate_draw_list_params` | 0x1408baed0 | 133 | 27 kind × 4 list 前缀和：cmd_offsets += T[k]；draws 钳制（!FORCE_FINE && (k>=2||FORCE_NO_FINE) → 0x40）；fine=draws>>5（掩码 **0x3fe** 非 0x3ff）；batch=(fine>>10)+(fine&0x3fe!=0)；循环体机器码展开 4 份（0x1408bb0bc 系） | O | B_n_world §1/§2.8/§2.9；B_i_nani §2.1（LISTS_PER_KIND=4、108 项、总 3788800/71936/2048/8）；wp3/contracts/buffers.md（0x1408baed0；与 default_value 是**两个独立函数**） |
| `country_core::render::nani::buffers::impl$9::default_value` | 0x1408bb650 | 258 | NaniDrawListOffsetDefaults（[u32;108]，0x1b0B）：offsets[4k+j] = base + j*T[k]，base += 4*T[k]；跳表 0x142aa1618 | O | B_n_world §1/§2.9（0x1408bb650；wp3 合同称两者反编译重叠 = Ghidra 跳表误读，已纠正） |

## 内联/无独立符号函数

- `country_core::render::nani::buffers::GpuInstance::new`（行 23）— GpuInstance（紧凑实例记录）构造 —— I（字段 U：B_n_world U 清单）
- `country_core::render::nani::buffers::GpuInstance::set_mesh_atlas_part`（行 31）— 写 mesh atlas part 字段 —— I（字段 U）
- `country_core::render::nani::buffers::NaniDrawListParams::list_draw_merge_window_range`（行 111）— draw-list 合并窗口范围（内联；语义由名+draw 列表接线推断）—— I
- `country_core::render::nani::buffers::NaniDrawListParams::list_draw_merge_window_subtotal_range`（行 121）— 合并窗口小计范围 —— I

## 证据注记（tg_rn1）
- **MAX_DRAWS_PER_LIST**（O，27 值从 0x142aa14c0 读出）：8192,8192,4096,32768,8192,8192,8192,8192,262144,40960,1024,53120,9600,131072,512,8192,4096,1024,32768,65536,2048,1024,2048,131072,81920,256,32768；同一张表也是 max_instances（0x142aba790/0x142c77010 逐值相同）。
- **108 项互证**：`NaniDrawListInfo` 缓冲 0x1b0B = 108×4B = `NaniDrawListOffsetDefaults`；汇总 (3788800, 71936, 2048, 8)（B_i_nani §2.1）。
- **纠错**：`nani_calculate_draw_list_params` 与 `default_value` 是**两个独立函数**（0x1408baed0+0x77e / 0x1408bb650+0x570 不重叠；wp3 合同称 Ghidra 跳表误读为重叠）。
- 掩码是 **0x3fe** 不是 0x3ff（`test ebx, 0x3fe; setne`，照抄）。
- U：`GpuInstance` 字段；`list_draw_merge_window_range/subtotal_range` 语义（I）；7 个 SSBO 载体（NaniDrawListLength/NaniDrawList/NaniInstanceIndirection/Allocator 等）的库 `SsboType` 注册已由 B_i_nani §2 接线。
