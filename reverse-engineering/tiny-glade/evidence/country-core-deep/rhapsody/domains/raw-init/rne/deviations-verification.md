# 审查轮五处事实偏差逐项验证（审查轮闭合证据 P1-6）

- 日期：2026-10-15。逐字节验证（rne-read）+ 数据文件核对。全部 O。

## 1. NotFound 计数 9→10（O）

`domains/raw-init/summary.json`（46 条）error=true 实为 **10**：
0x140be6b40 / 0x14067dd80 / 0x140c0e610 / 0x140c0e630 / 0x140c0e640 / 0x140c0e600 /
0x140a9fa70 / 0x140c001e0 / 0x140c001f0 / 0x140c001c0。
ALGORITHMS.md 第 4 行与 MODULE-DEEP-DIG 登记行写 9 → 偏差确认。

## 2. dispose_cb_resource 实例 6→7（O）

`domains/raw-init.json` 中 `CommandBufferDisposalBuckets::dispose_cb_resource<…>` 共 **7** 条
（ManagedBuffer / ManagedPersistentDescriptorSet / ManagedFramebuffer / ManagedRenderPass /
ManagedSampler / ManagedSwapchain / ManagedTexture）。ALGORITHMS.md §2.7"6 实例"、§3.7"6 族" → 偏差确认。

## 3. 滞回钳位 max(1, 旧/8)（O）

decomp-0x140c01620 16 处 `uVar = (old>>3) + (old>>3==0)`（=max(1,old/8)）。
旧值 0..7 时钳到 [旧-1, 旧+1]（REVIEW §3.4/§4 偏差 #3 成立）。

## 4. dispose_in_order 断言串地址次序（O）

rne-read：
- 0x142d06f88 = `crates/rhapsody/src/raw/disposal.rs`（文件路径，34B，至 0x142d06faa）
- 0x142d06fa2 起 = `assertion failed: seq > self.last_retired`（断言文本紧随路径之后，约 0x142d06faa）
→ ALGORITHMS.md §2.7 行 228"0x142d06f88 后接路径"次序相反 → 偏差确认：**路径@0x142d06f88，断言文本紧随其后**。

## 5. init_logging 断言串地址 0x142b12f50（O，逐字节）

- 0x142b12f50 = `assertion failed: prev.is_none()`（逐字节命中）→ 断言文本确认在 **0x142b12f50**。
- 0x142b12f70 非断言文本（空）；`--fat` 读 0x142b12f70 = fat → **`crates/rhapsody/src/leek.rs`**。
  → REVIEW §4#5 猜测"0x142b12f70 指向 update_pending_submission_timestamp（submit.rs）"**不成立**；
  dump_log 的 panic Location 指向 leek.rs（Location 复用自 leek 区）。doc §2.10 行 264/266 的地址归属以本证为准。
