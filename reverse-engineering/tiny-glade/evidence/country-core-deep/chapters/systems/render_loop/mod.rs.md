# systems/render_loop/mod.rs

> 文件作用：render_loop 域公共工具：`RenderContext::finish`（渲染上下文终结/资源释放）、`clear_buffer`/`clear_rg_buffer`/`clear_rg_color_image`（向 GPU 命令队列压入 ClearBuffer/ClearImage 命令包）、`rg_buffer_copy_from_slice<u32>`（CPU 切片→GPU 缓冲的 BufferMemcpy 命令）。被 gpu_generate_draw_lists、read_terrain_data 等系统复用。

函数总计 8：业务 4 / 内联无独立符号 1 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::RenderContext::finish` | 0x1409dda50 | 52 | 渲染上下文终结：把 +0x168 状态字节置 2，拷贝 0x110B 头，释放 +0x150/+0x160（两个 0x18B 元素 Vec）+0x170 字段；`(cap&0xfffffffe)==2` 判满即释放两个指针表 | O | render-loop\decomp\decomp-0x1409dda50.txt |
| `country_core::systems::render_loop::clear_rg_buffer` | 0x1409ddbb0 | 70 | 向命令队列压入"Clear buffer"（UNK_142ac3b88，0xc 字符）命令包：读 slice 的 +0x10 处 u32 清除值，经 FUN_1409dca90 建 0x34 命令，0x40B 包 push 到 param_1+0x90/+0xa0 Vec | O | render-loop\decomp\decomp-0x1409ddbb0.txt |
| `country_core::systems::render_loop::rg_buffer_copy_from_slice<u32>` | 0x14090d9a0 | 99 | CPU slice→GPU 缓冲拷贝：断言 `data.len()<=buffer.desc().size`（`*param_3 < param_5<<2` panic），World 查资源（hash 0x47c94cb78283a72e）取 dest 缓冲句柄，`Buffer memcpy`（UNK_142aab248，0xd 字符）命令 + 加锁写 `RgBuffer`（LOCK/UNLOCK 写占用字节 0/1），memcpy `param_5<<2` 字节 | O | render-loop\decomp\decomp-0x14090d9a0.txt |
| `country_core::systems::render_loop::clear_rg_color_image` | 0x1409dde90 | 137 | 同 clear_rg_buffer 但命令名"Clear image"（UNK_142ac3b94，0xb 字符），清除值直接取 `*param_2`（u32） | O | render-loop\decomp\decomp-0x1409dde90.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::render_loop::clear_buffer`（行 59）— clear_rg_buffer 的转发封装（删除/清理命令入口）

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 3 项

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### 1. `RenderContext::finish`（0x1409dda50）—— 终结
+0x168 状态字节置 2（终结态）→ 拷贝 0x110B 头 → 释放 +0x150/+0x160（两个 0x18B 元素 Vec）+0x170 字段；`(cap&0xfffffffe)==2` 判满（cap-first Vec"已有 2 元素"）即释放两个指针表。compile 闭包尾部调用（CALL-CHAIN §2.2 0x1a147）。

### 2. 命令包 0x40B（clear_rg_buffer / clear_rg_color_image / rg_buffer_copy_from_slice）
- **格式**：`{cmd_tag, name_fat_ptr, len, args, value}` 0x40B；命令队列 = cap-first Vec（+0x90 cap/+0x98 ptr/+0xa0 len，扩容 FUN_1405627b0）。经 FUN_1409dca90 建 0x34B 命令 → 0x40B 包 push。
- **clear_rg_buffer**（0x1409ddbb0）：命令名 "Clear buffer"（0x142ac3b88，0xc 字符）；清除值 = slice 的 +0x10 处 u32。pass 名 "Clear buffer"、访问 0x34、组 0。
- **clear_rg_color_image**（0x1409dde90）：命令名 "Clear image"（0x142ac3b94，0xb 字符）；清除值直接 `*param_2`（u32）。
- **rg_buffer_copy_from_slice<u32>**（0x14090d9a0）：断言 `data.len() <= buffer.desc().size`（`*param_3 < param_5<<2` panic，文案 "assertion failed: data.len() <= buffer.desc().size"@0x142aab1e0 50B）→ World 查资源（TypeId `0x47c94cb78283a72e` = Mutex<DynamicConstants>）取 dest 缓冲句柄 → "Buffer memcpy"（0x142aab248，0xd 字符）命令 → **加锁写 RgBuffer**（LOCK/UNLOCK 写占用字节 0/1）→ memcpy `param_5<<2` 字节。
- 消费方：`gpu_generate_draw_lists`（clear buffer 路径）、`read_terrain_data` 回读（rg_buffer_copy_from_slice）、水/冰 surface（clear_rg_buffer/clear_rg_color_image）。

## 关键事实

- 命令包格式：`{cmd_tag, name_fat_ptr, len, args, value}` 0x40B；命令队列 = cap-first Vec（param_1+0x90=cap/+0x98=ptr/+0xa0=len，扩容 FUN_1405627b0）。
- `RenderContext::finish` 的 `(x&0xfffffffe)==2` 是 cap-first 表"已有 2 元素"判定（PLAYBOOK 事实 1 变体）。
- 跨域：gpu_generate_draw_lists（0x140987dd0）调 `clear_rg_buffer`；`rg_buffer_copy_from_slice` 被 read_terrain_data 等回读路径复用。
