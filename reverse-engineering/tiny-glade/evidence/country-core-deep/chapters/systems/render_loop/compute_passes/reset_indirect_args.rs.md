# systems/render_loop/compute_passes/reset_indirect_args.rs

> 文件作用：间接参数重置：`reset_indirect_args`（清空/重置间接绘制参数缓冲）。

函数总计 2：业务 1 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args` | 0x14090a870 | 24 | 重置间接参数：串证据 "Reset indirect args"；4 块暂存区 0x80/0x1e0/0x10/8（初值 8×[0,1,1,0] / **memcpy 自静态表 0x142aaaa78（24 条 DrawIndexedIndirectCommand，仅 index_count 非零）** / [0,1,0,0] / 0x0009000000000000）；"Reset indirect args" pass 4×`add_resource_access(id,0x34,0)`，闭包 4×copy_buffer | O（2026-10-14 反编译） | render-loop\decomp\decomp-0x14090a870.txt；render-gap\decomp\decomp-0x14090a870.txt |

> 纠错（render-gap §3.2）：B_c3 判"0x1e0 B 暂存区无写入（U）"**错误**——机器码 `func_0x000142923c00(staging, &UNK_142aaaa78, 0x1e0)` = memcpy。
> 24 条 index_count：`[0x1b,0x12c,0x60,0x39,0x24,0x6,0x3c,0x6,0x30,0xae,0,0,0x10b,0x126c,0x42,0xb4,0xf,0x384,0x12,0x384,0x12,0x12,0x12,0x12]`。
> reconstruction `reset_indirect_args.rs` 的"不写"断言（0xAB 填充）需改为 `index_init()`。

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

### `reset_indirect_args`（0x14090a870 + 0x14169a830 闭包）—— 4 块暂存区 + 4 copy_buffer
处理链（O，round3 §5 + B_c3 §2）：
1. **4 块暂存区**：0x80 = 8×`[0,1,1,0]`（0x142aaa1e0/e8）；**0x1e0 = memcpy 自静态表 0x142aaaa78**（24 × 20B DrawIndexedIndirectCommand，仅 index_count 非零，round3 纠错"无写入"判据）；0x10 = `[0,1,0,0]`（0x142aaaf74/7c）；8B = `0x0009000000000000`；
2. "Reset indirect args" pass：**4 次 `add_resource_access(id, 0x34, 0)`**（传输写）；
3. 闭包（0x14169a830）4 次 `ResourceRegistry::buffer` + `RenderPassApi::copy_buffer`。
- 用途：nani/植被/水的 DrawIndirect 缓冲清零复用；reconstruction 应改为先 memcpy 表内容（index_init()）。

## 关键事实

- 供 nani/植被/水的 DrawIndirect 缓冲清零复用（wp5 原语/调试族）。
