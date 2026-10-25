# disposal 锁拓扑裁决（审查轮闭合证据 P1-5）

- 日期：2026-10-15。方法：decomp-0x1418c4100（remove）/0x1418c4260（insert）+ rne-read 原始字节逐指令解码（CAS/偏移/拷贝大小）。

## 1. 结构定案（O）

```
CommandBufferDisposalBuckets = parking_lot::Mutex< hashbrown::HashMap<CommandBufferSeq,
                                                          parking_lot::Mutex<DisposalBucket>> >
  +0x00  parking_lot Mutex state word（外层整表锁）
  +0x08  hashbrown::HashMap（base，remove_entry/insert 均以 map+8 操作）
  +0x28  hashbrown RawTable 内部桶区（find 操作以 map+0x28 传入）
  map value = parking_lot::Mutex<DisposalBucket>，0xf8B = {Mutex state@+0(8B), DisposalBucket@+8(0xf0B)}
```

## 2. 外层锁原始字节（O）

remove 入口：`b9 08 00 00 00 31 c0 f0 48 0f b1 0a` = `mov ecx,8; xor eax,eax; lock cmpxchg [rdx],rcx`
→ 对 **map+0 的 state word** 做 CAS 0→8 抢锁；失败（jne）走慢路径 `mov r8d,0x3b9aca00(1e9ns=1s); call FUN_1428cb5f0`。
释放：`xor ecx,ecx; mov eax,8; lock cmpxchg [rbx],rcx` = CAS 8→0 解锁。
insert 入口同构（`lock cmpxchg [r11],rcx`）。→ **外层整表 parking_lot 锁确认存在**（REVIEW §3.3 收敛）。

## 3. map 偏移 +0x28 vs +0x8 矛盾裁决（O）

不是两个 map，是**同一个 hashbrown HashMap 的两个层级**：
- remove：`lea rdx,[rcx+8]` 传给 FUN_1417e7ad0（remove_entry，map base=+8）；
  同时 `lea rcx,[rbx+0x28]` 传给 FUN_1405b0d70（find，RawTable 桶/ctrl 区=+0x28）。
- insert：`lea rdx,[rcx+8]` 传给 FUN_1418adf90（insert_entry/entry，map base=+8）。
→ map base = **+8**；+0x28 为 RawTable 内部字段（hashbrown HashMap={RandomState@+8(16B), RawTable@+0x18}，
RawTable 的 bucket/ctrl 区在 +0x28 附近）。REVIEW 的"+0x28 vs +0x8 矛盾"由此消解。

## 4. 桶值大小与断言（O）

- remove：remove_entry 命中后 `mov r8d,0xf8; memcpy`（值 0xf8B=Mutex<DisposalBucket>），再 `mov r8d,0xf0; memcpy`
  取 DisposalBucket 0xf0B 回传给调用方；Option tag 为 0（缺桶）→ panic
  `bucket removed prematurely or command buffer begin() never called?`（@0x142d069c1, len 0x42）。
- insert：`alStack_228[0]!=0`（entry 已存在/prev 非 None）→ panic
  `assertion failed: prev.is_none()`（@0x142d06a40, len 0x20）。
- DisposalBucket = **0xf0B**（含多 Vec/容量字段，"capacity overflow" 串 @0x142d069b0 佐证桶内 Vec push）。

## 5. rebuild 对照

rebuild `BTreeMap<u64, Vec<DestroyAction>>`（无锁）+ 允许重复 + remove 返回 Option → **结构/方法语义均偏离确认**。
原：外层整表 Mutex + 每桶 Mutex + 重复 seq panic + 缺桶 panic。rebuild 已有 begin_command_bucket 重复检测候选仍非原 panic 语义。
