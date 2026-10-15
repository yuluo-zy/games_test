# input/mod.rs

> 文件作用：**活跃输入模式跟踪**。`ActiveInputModeTracker` = 0x20 B
> `{controller_deadline: Instant {secs u64@0, nanos u32@8}, mouse_kb_deadline: Instant
> {secs u64@0x10, nanos u32@0x18}}`——两半各为「最后使用 + 250ms 冷却截止」时刻；
> `mode()` 比较两半返回 bool（控制器截止更晚 → true=Controller）；`ActiveInputMode`
> 2 变体（Controller=1/MouseKeyboard=0），`is_controller` 直接返回判别字节。
> 算法深挖（2026-10-15，详见 `evidence/country-core-deep/input/ALGORITHMS.md` §3）：
> 构造 = `Instant::now()`（0x140d635e0）+ `now.checked_add(250ms)`（0x140d63a20，r8=0、r9=0xee6b280=
> 2.5e8 ns），默认控制器版把 +250ms 写入控制器半、裸 now 写入鼠标键盘半（鼠标键盘版互换）；
> `mode()` 全字节手解 = `(secs@0 > secs@0x10) || (secs@0==secs@0x10 && nanos@8 > nanos@0x18)`
> （cmp+setne/cmovb 双段 + `cmp r9b,1; sete al` 归 bool）——**无存储判别位，判别由比较派生**。
> 证据：new_default_controller/new_default_mouse_keyboard 反编译 + mode/is_controller 原字节
> （本轮 capstone 全反汇编，0x140958f30/0x1408c5680 在 tg_m5 数据库 NotFound，以原字节为准）。

函数总计 7：业务 4 / 内联无独立符号 2 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `ActiveInputModeTracker::new_default_controller` | 0x140958eb0 | 15 | 默认控制器模式跟踪器 | **O** | 本轮 capstone：now()(0x140d635e0)→rdi；now+250ms(0x140d63a20, r9=0xee6b280) 返回 {secs,nanos}→{@0,@8}（控制器半）；裸 now→@0x10、@0x18=now.nanos（鼠标键盘半） |
| `ActiveInputModeTracker::new_default_mouse_keyboard` | 0x140958ef0 | 23 | 默认鼠标键盘跟踪器 | **O** | 同构互换：{@0,@8}=裸 now，{@0x10,@0x18}=now+250ms |
| `ActiveInputModeTracker::mode` | 0x140958f30 | 32 | 当前模式判别（比较派生） | **O** | 本轮 capstone 全字节：`(secs@0>secs@0x10) || (== && nanos@8>nanos@0x18)` → bool；语义（I）：截止更晚=更近使用=该输入活跃；两半相等/过期 → false（鼠标键盘默认） |
| `ActiveInputMode::is_controller` | 0x1408c5680 | 71 | 是否控制器模式 | **O** | 原字节 `movzx eax, byte [rcx]; ret`——返回判别字节（Controller=1；与 StructureType::is_fence 同址折叠，语义巧合） |

## 内联/无独立符号函数

- `ActiveInputModeTracker::on_controller_used`（行 39）— **I**（重置控制器半 = now+250ms，
  名称/行为推断；wp15 证据内联进 `systems::steam::update_steam_inputs`）
- `ActiveInputModeTracker::on_mouse_keyboard_used`（行 46）— **I**（重置鼠标键盘半 = now+250ms，同上）

## 派生胶水（1 项：clone 等）
