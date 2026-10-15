# systems/wall/snapping/snapping_audio.rs

> 文件作用：吸附音效系统：mode==2 且声音状态字节(+0x20)!=2，或 mode==3 且 <=1 到写 PlayWallUiSound。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::snapping::snapping_audio::snapping_audio` | 0x140a6a480 | 0 | snapping_audio（mode==2 且 +0x20 状态字节!=2 / mode==3 且 <=1 → PlayWallUiSound） | O | audio-misc\decomp\decomp-0x140a6a480.txt, wall\decomp\decomp-0x140a6a480.txt |

## 实现方式（算法级）

### snapping_audio 0x140a6a480（O，全读）

签名（2 参）：param_1 = &SnappingMode（pcVar2 = *param_1），param_2 = &mut Mut\<EventWriter\<PlayWallUiSound>>（tick 回写 @+0x1c；事件行 **0x20B**，Vec {cap@+0x20, ptr@+0x28, len@+0x30, seq@+0x40}）。

处理链（**音效去重**）：
1. `mode != 2/3` → return。
2. mode==2：`if +0x20 byte == 2` → return（端吸附音本帧已播）；否则 `xvy(pos@+8, height@+4)`（func_0x000140c9cde0 = utils::swizzlers::xvy，输出 {x, height, y}）→ kind=1。
3. mode==3：`if +0x20 > 1` → return（段吸附音已播）；否则同上 xvy → kind=2。
4. 写事件行 0x20B：`{u64 seq@0, u8@+8..+0xf（末字节 kind 1/2）, u64@+0x10 = xvy 低 8B, u32@+0x18 = xvy 高 4B}`。

+0x20 语义（O 推断）：由 snapping_maintain 0x140a6ad00 每帧写入的**上一帧发布 mode**；mode2 发 kind=1、mode3 发 kind=2，`+0x20==当前 kind 则跳过` → 连续同模式帧不重复发声。mode==2 且 +0x20==2 跳过 / mode==3 且 +0x20>1 跳过即此。

### 常量
- kind 映射：mode 2 → 1（端吸附），mode 3 → 2（段吸附）。

### 遗留 U
- PlayWallUiSound 事件消费端（音频域）与 uStack_24/uStack_1c 的精确语义。
