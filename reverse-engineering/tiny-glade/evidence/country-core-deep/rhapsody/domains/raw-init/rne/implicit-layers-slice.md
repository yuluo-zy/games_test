# implicit_layers 返回原始 0x208B 切片语义（审查轮闭合证据 P1-4）

- 日期：2026-10-15。方法：decomp-0x140bda7b0.txt（712B 全量）+ decomp-0x140bd7dd0.txt（消费方）。

## 1. 定案（O）

`GraphicsInstanceLoader::implicit_layers`（FUN_140bda7b0）返回**原始 vk::LayerProperties 切片**
（元素 0x208B），不是过滤后的 implicit 清单、也不转换 20B GraphicsInstanceLayer：
```
FUN_1418b09f0(&lStack_20)                      # 枚举 instance layer properties
if (成功):
  *param_1 = lVar2          # 返回结构 {ptr@0, ptr@8, len(count)@0x10, end@0x18}
  param_1[1] = lVar2
  param_1[2] = lStack_20    # count
  param_1[3] = lStack_10*0x208 + lVar2   # end = ptr + count*0x208
else:
  panic "Failed to enumerate instance layer properties"（@0x142b1178e, len 0x2d）
```
即 Rust `&[vk::LayerProperties]`（{ptr,len}），元素 0x208B。

## 2. 消费方（O）

`GraphicsInstance::new`（0x140bd7dd0, decomp line 317-350）遍历该切片，逐 0x208B 条目转换：
- 读层名 fat 串（&UNK_142b11170 区），构造 GraphicsInstanceLayer；
- uStack_b0/uStack_a0 = FUN_140bd7b50/FUN_140bd7ae0（转换/析构函数指针）；
- 完成后释放临时串（func_140613c20）。
→ "消费者把 0x208B 枚举条目转成 20B GraphicsInstanceLayer"确认（REVIEW §3.7 方向正确）。

## 3. rebuild 对照

rebuild `Option<&[GraphicsInstanceLayer]>`（恒 None）在类型与语义上都不对位（doc 半判定成立）。
应改返回 `&[vk::LayerProperties]`（或等价物）；调用方负责 0x208B→20B 转换。`implicit_layers` 与"过滤到
~implicit~ 前缀层"无关——函数名误导，实际返回全部枚举结果。
