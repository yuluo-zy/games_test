# nani buffers 契约 — buffers.rs（2 procs）

标注：**O**=反编译直接支持；**I**=推断；**U**=未恢复。注意：`nani_calculate_draw_list_params`（0x1408baed0）与 `default_value`（0x1408bb650）反编译**重叠**（跳表错位，Ghidra 两入口解析到同一代码体）；二者本质是同一 `DrawListParams` 构建逻辑的两条调用路径。

## 1. `DrawListParams` 返回结构（0x70B，O）

`nani_calculate_draw_list_params()` 按值返回 0x70B 结构（`local_c0` 末尾写）：
| 偏移 | 字段 |
|---|---|
| +0x00 | `Vec<u32> cmd_offsets`（前缀和；push 每 kind 的累计 cmd_offset） |
| +0x18 | `Vec<u32> max_instances_offsets`（push 每 kind 的累计 max_instances） |
| +0x30 | `Vec<u32> fine_grained_offsets`（push 每 kind 的累计 fine-grained 子列表数） |
| +0x48 | `Vec<u32>`（第 4 个：push `iVar38` 分批汇总） |
| +0x60 | `u32 total_cmd_offset` |
| +0x64 | `u32 total_max_instances`（`local_50`） |
| +0x68 | `u32 total_fine_grained`（`local_4c`） |
| +0x6c | `u32 total_batches`（`uVar36`） |

## 2. `nani_calculate_draw_list_params`（0x1408baed0，行 133；O）

对每个 subset kind（0x1b=27 个，switch 循环）：
1. `max_draws = DAT_142aa14c0[kind]`（u32 常量表，**每 kind 每 draw-list 最大 draw 数**，U 逐值）。
2. push 累计 `cmd_offset`（前缀和）到 Vec#1；`local_50 += max_instances_per_draw`；push 到 Vec#2。
3. `fine = max_draws >> 5`（每批次 32），受 `NANI_FORCE_NO_FINE_GRAINED_SORTING` / kind<2 / `NANI_FORCE_FINE_GRAINED_SORTING` 开关调整（kind≥2 且未强制时 fine=0）；push 累计到 Vec#3；`batch = fine/1024 + (fine%1024!=0)` 累计到 Vec#4。
4. 返回结构（见上）。

常量表 `DAT_142aa14c0`（27×4B）未读内存；`default_value`（0x1408bb650）是同一代码体的构造入口（`memset 0x1b0` + 跳表逐 kind 填表 + 返回 {tag=1, 0,0,0, 数据}），`case 0x1b` 处 `memcpy(in_RCX+4, 表, 0x1b0)`。二者**数值等价性未逐字节验证**（U），但结构与生成逻辑同源。

## 3. SSBO 类型（O，public-symbols + draw_ext/access 证据）

- `NaniDrawListLengthSsbo`：draw-list 长度（每 list 的 draw 数）。
- `NaniDrawListSsbo`：间接 draw 列表本体。
- `NaniInstanceIndirectionSsbo`：实例间接寻址表。
- 关联：`draw_ext::nani_draw` 0x140904ae0 经 `AssetSsboLibrary::get_rg_buffer<...>` 取三者；`pass_resource_access` 证据同。

## 4. 消费者

- `GpuWorld::new`（0x1409917d0）首行调用 `nani_calculate_draw_list_params()`，结果用于 `+0x14a0` 与 27 槽布局（见 gpu_world.md §1/§2）。
- `draw_ext::nani_draw` 读 NaniDrawListLength/NaniDrawList/NaniInstanceIndirection（见 draw_ext.md）。

## 5. Unknown

- `DAT_142aa14c0`（每 kind max_draws_per_list）与 `DAT_142aba790`/`DAT_142c77010`（每 kind max_instances）数值表未读内存（typed gap，需 `read_memory`）。
- 两函数重叠的确切边界（Ghidra 未正确分裂函数）U；本契约按同一构建逻辑处理。
