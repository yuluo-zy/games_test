# order_and_filter_devices 排序比较器（审查轮闭合证据 P0-2）

- 日期：2026-10-15。方法：decomp-0x140bdbc30.txt + raw 字节 call 扫描 + MCP 反编译排序 helper
  FUN_1418b9130/1418bb250/1418b5460/1418b5680 + missing_devices 0x140bdb090。

## 1. 排序骨架（O）

```
FUN_140bdbc30(param_1=devices:&mut Vec, param_2):
  ptr=param_1+8; len=param_1+0x10; 元素 0xa0B（AvailableGraphicsDevice）
  if len<2: 仅 filter
  if len<0x15: 逐元素调 FUN_1418b9130(start, start+i*0xa0)（std 插入排序 helper，比较器内联）
  else: FUN_1418bb250(start, len, &closure)（std sort_unstable/driftsort 包装）
        → 内部 FUN_14213bee0(…, len<0x41, &closure)（driftsort 引擎）
  FUN_1418b5460(param_1,param_2)   # 排序后过滤：available/missing
```

## 2. 比较器判据（O，字段级；权重序未完全恢复→I）

比较器内联在 std 排序 helper 中（FUN_1418b9130 反编译 242KB，坏指令/跳转表恢复失败但关键字段可辨）：
- 设备名 contains 检查：`FUN_140bd7250(name,"Direct3D12",10)` / `(name,"Microsoft Basic Render Driver",29)`
  （FUN_140bd7250 内部调 0x142923bf0 两次 = str::contains 系）→ **emulated 设备分类**。
- `*(dev + 0x88) >> 0x1e`（device_local_memory 高 30 位 → GiB 量级评分）。
- `dev[0x99]`（shared 标志字节）switch 分派不同评分/门槛。
- 另涉 +0x90（host_memory）与 +0x18/+0x1f8（名字/驱动区，Ghidra 误判偏移段）。

→ 判据与 rebuild `sort_by_key((supported, compatible, device_type, budget, heap, index))` 的候选在
"emulated 排除 + 内存量级评分 + 共享/分离分派"层面一致；**精确优先级权重未逐字恢复（I）**。

## 3. available/missing 分区谓词（O）

FUN_1418b5460 对每个排序后元素调 FUN_1418b5680：
```
if dev[0x99]==0（分离）:
    dev[+0x88]>999999999(1.0GB) && dev[+0x90]>399999999(400MB)  → available(return 1)
else（共享）:
    dev[+0x88]+dev[+0x90]>1399999999(1.4GB)  → available(return 1)
否则：构造 0xf0B missing 条目（复制名字/String 等，func_142923c10(…,0xf0)），push 到 missing Vec，return 0
```
→ 与 doc is_compatible 内存门槛逐字对上（O）。

## 4. missing_devices（0x140bdb090）条目（O，更正 doc 表述）

- 排序/过滤内部 missing 构造为 **0xf0B** 条目（FUN_1418b5680）。
- 公开 `missing_devices` 另行物化 **0x28B** 条目（decomp line 278/438/498：`*0x28`、`+0x28`、`*0x28`），
  含 vendor id 匹配 `0x1002`（AMD，line 396）/`0x10de`（NVIDIA，line 472）。
- → doc §2.1"missing 条目 0x28B"对 missing_devices 成立（O），但需注明过滤内部为 0xf0B 双表示。

## 5. U 卡点

比较器精确权重序（emulated 分 vs 内存分 vs 共享分谁先）仍 I；如需 O 建议对 RNE 构造多设备场景行为注入，
或修复 Ghidra 对 FUN_1418b9130 的跳转表恢复后重解。
