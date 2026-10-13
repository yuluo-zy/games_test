# P4 取证：queue_submit 签名 / 同步来源 / setup command buffer

- 日期：2026-10-13。方法：**只读 PE 直读**（`scripts/render-recovery/pe_readonly.py`，标准库解析节表，`.pdata` 定界，`objdump -M intel` 反汇编）。
- 对象：`tiny-glade.rne`，SHA-256 `f507a6ce…7d03`（原版 B，oracle）。PDB `2d9686b8…8911`。未运行目标，**纯静态**。
- 置信度：**H**=指令直接可见；**M**=由参数/常量推断；**L**=猜测。`queue_submit` 6900 B / 114 调用，**本轮精读约 30%**（序言、参数使用、序列号分配、返回），其余循环体未逐行解码。

## 1. C17 签名矛盾：已解决（H）

`queue_submit`(`0x140c04e20`) 序言保存 `rcx→[rbp+0x238]`、`rdx→[rbp+0x418]`、`r8d→[rbp+0x410]`、`r9d→[rbp+0x414]`；全函数**无栈参数读取**（`[rbp+0x528/0x530]` 零命中）。

| 寄存器 | 角色 | 证据 |
|---|---|---|
| `rcx` | **sret 返回槽**（返回 `rax=rcx`） | 尾段 `0x140c06348: [rcx]=r8d|r9d<<32`、`0x140c0634b: [rcx+8]=seq`，`0x140c0653b` 返回 |
| `rdx` | `&GraphicsDevice` | 取 `+0x730`（主队列）、`+0x7e8/+0x7f0`（辅助队列 Vec 与长度）、`+0x7f8`（时间线计数器） |
| `r8d, r9d` | **`Option<u32>` 队列选择**（Rust ABI 把标量对拆两寄存器） | `test r8b,1`→`Some(r9d)`：取 `queues[r9d]`，步长 `0xb0`、越界跳 panic；否则 `device+0x730` |

⇒ 反编译的"5 参"与调用点的"≤3 参"**都不对**。真实 Rust 签名约为 `fn(&GraphicsDevice, Option<u32>) -> SubmitReceipt{ queue: Option<u32>, sequence: u64 }`，机器层 4 个寄存器。**这印证 ASSESSMENT 里"arg_count 是机器寄存器个数，不是 Rust 参数个数"的警告。**

## 2. A2 同步来源：候选的外部注入是错的方向（H/M）

- `queue_submit` **不接收 waits/signals**。输入只有"设备 + 队列选择"。
- 时间线值**内部分配**：`0x140c057fe..0x140c0580f`：`old=[dev+0x7f8]; dev+0x7f8 += n`（n=本次提交条数），返回回执 `sequence = old+n-1`（`0x140c06337 dec rax`）。**设备级单调 u64 计数器，一次预留连续区间。**（H）
- 提交体来自设备内部待提交列表：循环按步长 `0x240`（命令缓冲记录）与 `0x90`（同步记录）把句柄拷成数组（`0x140c05720..0x140c05794`）。（M：数组用途=组 `VkSubmitInfo` 的 command/semaphore 向量，未逐行核）
- 与队列的"已完成值"闭环：`Queue::update_pending_submissions`(`0x140bf0b90`) 以槽 `+0x478`(`vkGetSemaphoreCounterValue`) 读 `queue+0x88` 的时间线信号量，写入 `queue+0x80`，随后回收。（H，复核 G8 结论）
- 辅助队列提交时校验"命令缓冲所属队列 == 请求队列"，否则 panic（`0x140c05bac..0x140c05bc2` 比较 `[cb+0x138/0x13c]` 与 `Option<u32>`）。（M）

## 3. C18 "20ms 阈值"：判定为反编译失真（H）

`submit_pending_cbs`(`0x140bebdf0`, 257 行)中搜索 `0x1312d00 / 0x4e20 / 0x989680 / 0x2faf080 / 0x3ca3d70a`（20ms 的各种整数/浮点编码）**零命中**。结论：不存在该字面量，**不实现**。

## 4. R5 setup command buffer：流程清楚（H/M）

`get_setup_cb_submit_info`(`0x140bf84e0`, `sret=rcx, device=rdx`)：
1. `lock cmpxchg byte [dev+0x9f0]` —— 设备内的**自旋/互斥锁**（H）。
2. 取出 `dev+0x9f8` 的状态并置为哨兵 `4`（= None）；若原值已是 `4`，直接返回 `0x8000…`（即 `None`）（H）。
3. 把 `dev+0xa00..0xa4f` 的 **80 B Command** 搬出；`+0xa40` 即 Command 内 handle（H，与 SYNTHESIS 第 286 行更正一致）。
4. 在该 CB 上记录一条 **`vkCmdPipelineBarrier`**（经槽 `+0x3e0`）：`src=dst=0x10000`(ALL_COMMANDS)，`VkMemoryBarrier`(sType=46) `srcAccess=dstAccess=0x18000`(MEMORY_READ|WRITE)。（M：槽名由参数形状推断）
5. 经槽 `+0x2c8` 调 `vkEndCommandBuffer`（M）；返回"1 条命令 + 5 个空同步向量"的 submit info（H）。

⇒ **setup CB = 设备级"待提交的上传/初始化命令缓冲"**：纹理/缓冲初始化命令写进它；真正提交前补一个全量内存屏障并 end，然后排在图命令之前。**它本身不带 wait/signal。**

## 5. 对实现的含义

| 项 | 原作 | 候选现状 | 结论 |
|---|---|---|---|
| 提交入口 | `(device, Option<queue>) -> {queue, seq}` | `GraphBatchSync` 外部注入 waits/signals | **偏差，应内部化** |
| 时间线值 | 设备级 fetch-add 预留区间 | 外部给定 | 偏差 |
| setup cb | 设备内独占状态机 + 全量屏障 + end | `Unsupported` | **可实现** |
| 20ms 阈值 | 不存在 | — | 不做 |

## 6. 未闭合（不要当已证实）

- `queue_submit` 余下 70% 的循环体：waits 的具体来源（上一次提交的 sequence？跨队列依赖？）、`[rax+0x88]` 调用的 ash 槽名（`0x140c061eb`）。
- `dev+0x9f8` 其他状态值（仅见 `4`=None）的含义。
- `0x142b183f0` 的 0x51 字节串是 panic 位置信息，非业务。
