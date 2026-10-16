# RE-SESSION-SWITCH — 会话切换链卡2 阻塞证据

分析师: reanalyst-state
日期: 2026-10-15
范围: ① run_game_thread 3 命令载荷 ② fence 双 tag 关系 ③ load_game_session 完整切换流
④ commander_update case 0 深度。证据 = fresh 反编译 + RNE 原字节（本次新增 FUN_1412e0b90
反编译、0x14091f450 原字节、run_game_thread 命令构造原字节）。

---

## 0. 核心新证：FUN_1412e0b90 = **Commands::insert_resource<NextSession>**

fresh 反编译（evidence/decomp-0x1412e0b90-fresh-shared.txt）：`bevy_ecs::system::commands::
Commands::insert_resource<country_core::resources::session::NextSession>`——把 0xa0B NextSession
经 memcpy 入 Bevy deferred command queue（条目 0xa8B）。**这统一了整条切换链**：

```
commander_update case 0（会话加载发起）：
  param_13 = Commands → FUN_1412e0b90(Commands, &NextSession_0xa0B, …)
    → insert_resource<NextSession>（带 on_load 列表 + fence）
load_game（读档主系统）：
  param_8 = Commands → FUN_1412e0b90(Commands, &新 NextSession_0xa0B, …)
    → insert_resource<NextSession>（把下一会话的 on_load 回填）
```

→ 会话切换 = **通过 insert_resource<NextSession> 资源接力**：commander 发起 → NextSession 资源
触发 load_game → load_game 处理 on_load → 回填新 NextSession → …。restorer 落 Rust 时可直接用
`Commands::insert_resource(NextSession)`。

---

## 1. commander_update case 0 完整状态机（O，补失败分支）

switch `tag^0x8000000000000000 == 0`（MetaCommand tag 0x8000000000000000 = 会话加载命令）。

### 路径 A：+0x18 载荷 == 0（首调 / 发起）
```
AppMode 门控：*(uint*)*param_2 - 6 < 0xf && *(uint*)*param_2 - 0xd < 2
  └─ 不在范围 → 组装 {tag 0x8000000000000000, 0, 0, 0} 重入队（等待下次，不变更状态）
  └─ 在范围 → 主路径：
    1. 新 fence = FUN_14091ef90()（new_pair_blocked，Arc strong=2，标志 +0x10=1）
    2. ResMut(param_9) tick 回写 + `[param_9 首字节] = 2`（写 LoadGameData 态/mode 字节）
    3. 建临时空 Vec{cap0,ptr8,len0} → FUN_141328ac0 压入 fence 条目 {0x8000000000000000, fence}
    4. FUN_1412e0b90(param_13=Commands, &临时 Vec/NextSession 0xa0B, …) → insert_resource<NextSession>
    5. 重入队命令 {tag 0x8000000000000000, fence_arc(+0x18), 载荷…}（FUN_140b5a7a0 push）
    6. drop 旧的 +0x18 Arc（LOCK dec）
```

### 路径 B：+0x18 载荷 != 0（重入队后再次到达，fence 携带在 +0x18）
```
if ([fence + 0x10] != 0)   // 标志字节仍 1 = 仍阻塞
    → 组装同命令重入队（继续等待；AppMode 门控仍生效）
else                        // 已 unblock（=0）
    → Arc drop（LOCK dec 归零释放）→ 命令处理完成
```

### 失败/等待分支
- AppMode 门控失败 → 重入队等待（无 panic，幂等重试）。
- **fence 标志偏移 = +0x10**（原字节 0x14091f450 `movzx eax,[rax+0x10]`，与 new_pair_blocked
  机器码 `c6 40 10 01` 一致）——**旧结论"+2"为 Ghidra 指针算术伪影**，纠正为 +0x10。

---

## 2. fence 双 tag 关系（O，解析）

| 值 | 语义 | 证据 |
|---|---|---|
| on_load 条目 tag `0x8000000000000000` | append_on_load<MetaCommandFence> 写入——**变体 0 会话加载命令，payload1 = fence Arc**（非 fence 终止） | 0x141328ac0 机器码 `mov [rbp-0x30], 0x8000000000000000` |
| load_game Pass1 变体 0（同 tag） | 写 LoadGameManager 状态 + push 24B 事件 {fence, p2, p3}（=PostGameLoadCommand 列表） | decomp-0x14146d030 行 392-405 |
| load_game Pass1 变体 2（tag `0x8000000000000002`） | **unblock + drop Arc**：LOCK `[payload+0x10]=0`（fence 标志清 0）+ LOCK strong--（归零释放） | 行 416-428（+0x10 修正） |
| load_game 终止哨兵 tag `0x8000000000000003` | 两遍回放 break | 行 383/743 |
| 命令 tag `0x8000000000000002` | commander_update 变体 2 = 命令重排 {tag3, countdown}（**与 on_load 变体 2 不同上下文**） | — |

**语义结论**：
- append 的 fence（tag 0）被 load_game 当作**会话加载命令**处理（携带 fence 进入 24B 事件）；
  fence 的**实际释放**走变体 2（tag 0x8000000000000002）条目——由**另一生产者**压入（session
  切换完成路径，非 append_on_load）。变体 2 条目 payload1 = fence Arc。
- **0x8000000000000003 终止哨兵来源**：非 append_on_load（后者只写 tag 0/3）；最可能由
  FUN_1412e0b90(insert_resource<NextSession>) 消费链或 list finalizer 追加——残留待追源
  （见 §5 #3）。
- 命令重排队后 commander_update 变体 2（fence 仍在 +0x18）检查标志 +0x10，阻塞则继续等待。

---

## 3. load_game_session 0x14166e7d0（O，澄清）

- **已 O**（startup_session.rs 章）：78KB 起始会话加载——起始 build 选择
  （assets/starting-builds/{full,demo} + intro 跳过）、会话目录创建、meta.json 读写（theme
  大小写表）、history.json/snapshots/screenshot.jpg 载入、copy_dir_all。
- **澄清**：`load_game_session` 是**应用启动的会话加载**，**不是**游戏内会话切换；
  后者 = commander_update case 0 → insert_resource<NextSession> → load_game（§1/§2）。
  原字节 0x14166e7d0 前 0x140 字节：16 参 prologue（sub rsp,0xed8）、读 SessionLoadAttempt
  Vec{cap@0,ptr@8,len@0x10} 判断、push 8B 元素、memcpy 0xa0（NextSession 拷贝）——
  与启动流一致（`should not have been consumed earlier` 串）。

---

## 4. run_game_thread 3 命令载荷（O 部分 + 残留）

- append_on_load<[MetaCommand;3]> 调用点 0x1405a3440：`lea rcx,[rbx+0x2590]`(NextSession 域) /
  `lea rdx,[rbx+0x440]`(on_load Vec) / `lea r8,[rbx+0xb00]`(3×0x38 数组)。
- **确认 tag**：cmd2（数组 +0xb70）tag = `0x8000000000000008`（变体 8，机器码
  `mov [rbx+0xb70], 0x8000000000000008`）。
- **载荷源**（cmd2 域 +0xb78/+0xb88/+0xb98）：`movups` 从 [rbx+0x2230]/[rbx+0x2240]（各 16B）
  与 rdi；cmd1（+0xb38）载荷写于 +0x10/+0x20/+0x30；另见变体 5 值 `0x8000000000000005`
  存于 [rbx+0x2bf0] 作源。
- **残留**：cmd0/cmd1 的完整 tag 与逐字段载荷需对 run_game_thread 0x1405a33xx 前段做
  ReVa High P-code 或逐指令反汇编（函数 39KB，栈上大结构多分支）——列为卡点（§5 #1）。
  但**结构结论已可交 restorer**：run_game_thread 构建 3 条 MetaCommand（含变体 8），
  经 append_on_load<[MetaCommand;3]> 压入 on_load，作为新会话的初始命令集。

---

## 5. 未决疑问

1. run_game_thread 3 命令逐 cmd tag/载荷（需 ReVa p-code 定点）。
2. 0x8000000000000003 终止哨兵的生产者（insert_resource 消费链或 list finalizer）。
3. 变体 2 on_load 条目（tag 0x8000000000000002）的生产者与触发时机（fence 释放时机）。
4. param_9（ResMut）与 param_13（Commands）在 commander_update 的参数面完整身份
   （13 参 SystemParam 逐位映射）。
