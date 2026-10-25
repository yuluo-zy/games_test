# find_alignment_preferences 独立源码复核

结论：**本 1196B 偏好叶源码通过独立静态复核，可与锁定依赖成组交 root 编译验证**。候选 SHA256 `a1bf04884ae3d6ac2ff573286cb3a7a192699186d86c7b9f00f8f7ffb37b8064`。本复核不改作者候选，不执行 Cargo、原程序或 GPU，不写生产源码。

审阅者 `/root/core_business_lead/tree_commands`；live task TG-f768c2955f5ef10ede 原 owner 为 `core-duck-building-20261021`，保持 claim。完整原 B `14080beb0..14080c35c` 共 1196 字节、265 指令已独立按 PE 节映射逐条核对 RNE 原字节，身份与既有 report 一致；结果见 `NATIVE_BYTES_REVIEW_TREE.json`。无 C 版本验收。

## 原行为与 typed API

输入复用真实 from/to StairPoint、caller 给定 deltaXZ、DecoratorStorage 和 PublicWalls；两个浮点入口合成 Vec2，不重算 position，不借 width 猜业务。候选仅替换此前空壳偏好方法并增加对应测试；增量补丁没有改变已审 node fold、best、walk flags、compute 或完整请求 producer。

原 bee5 None-normal 先行返回空序列；Some 才按原顺序读取 source addr/id 的 exact 桶、要求 source dst_tag<2、读取 source u、要求 target addr/id，再要求 source addr.as_wall。源码 `dst_as_wall`、bucket.get 和 addr.as_wall 的失败契约与此一致，不用全局 ID 扫描替换显式地址。缺 target 的失败发生在 source 地址类型要求之前。目标地址非 Wall、不同 WallId 或目标 dst_tag>=2 均进入三项分支，不把这些情形错当 required 失败。

三项分支保原 allocate-before-float、副作用仅返回值分配；调用 pinned glam 0.29.3 的 normalize_or_zero（有限正 reciprocal 才乘，否则 ZERO），按原 y*x-x*y、turn、dot 顺序计算。正 straight_dot 才取原 bit 常量 3ed413cd，NaN 走 factor1；输出顺序 0/1/1、flag0、byte2=sourceTag^1。第三项直接翻转 turn 与 score 符号，不另求 dot。inactive 字段现代初始化不计作原 padding 业务。

同墙先按 y²+x² 和 bits358637bd 做严格 `<`，NaN 不进入短距离；短距离单项不读取 PublicWalls。其余要求真实 PublicWall，shape.tag!=2 与 original_curve_length 对应原 wall_space 21B，而非 premade(tag<2) 猜测；subtract 参数确为 target.u、source.u。先减法、后分配、再 `-difference.signum()` 与 fromTag0 的第二次符号翻转，保原负 canonical NaN 与 signed-zero 路径。

输出使用 canonical AlignKind20B+score 的 typed tuple，依赖已审完整 params after `2a046da8ee6e909304adc12dc4901a64e1bd4a44cb4aa506b7b4085ce29f5e1c`，不把旧24B伪 padding 当 score。samewall 必须配 canonical WallSpace f32、有序双 `<` 修复 after `c510f0f3128f6f83aa1edfbe0dbc2fc77c101ee849f5b33d169567ff5f9ce804`；不应只用旧工程 f64 half-tie 分支运行后称等价。旧合同 subtract 的 `>=` 摘要没有覆盖 unordered，最终共享候选已经按原双 `<` 修复。

## 测试与实际边界

冻结记录及增量源码实际是 **6 个新增测试**。fixture 复用真实 Storage、CachedTransform、StairsState graph、PublicWalls 发布路径和 collect_stair_points；不是 mock lookup。覆盖 None 绕过全部 owner、短距离绕过 PublicWalls、三项顺序和折扣、两侧法线与第三项 XOR、权威 u 差而非 caller 位移、NaN 的严格比较门、source dst 与 target/addr 失败先后、负 canonical NaN。错误地址与缺 ID 测试是刻意 fault injection，未把非法状态声称为正常 producer。Info.ty0 未被本叶读取；真实楼梯 Info producer 类型仍按作者已列边界保留。

测试语义与原码吻合，但这里未运行。特别是 NaN bit 断言仍需 root 在目标 Rust/CPU 执行；不能以源码复核代替实测。当前生产仅有旧空壳定义，候选的新叶只有本地测试调用；完整双请求、interpolate、cache、62KB mesh 与宿主接线仍未由此完成。

## 补丁重现

`after-walk-flags.patch` 从 b42f8928 独立复现，`candidate.patch` 从 87435b0f 独立复现；两份均 `patch --batch --fuzz=0 -p1`、零 fuzz、零 offset，最终逐字节得到 a1bf0488。见 `SOURCE_REPRODUCTION_TREE.json`。完整补丁包含先前已审叶，本轮没有扩大其通过范围。
