# 两个原算术 NaN 的跨 ISA 符号合同

root确认的有界测试修复；TGb932 live NULL 后正常claim，保同一现有worker；生产算法与原golden数组/JSON均不动。只修 native_interaction_goldens 原24case中 type56/63、physical=false、terrain=false、dy=+0 的 Y 值比较。

根因：原499B SHA93edb351已逐范围对RNE核定，原141278191..1c0执行 (0-origin.y)/dy 后 direction*t+origin。真实fixture origin.y2、dy+0得到 t=-Inf，Y为0*(-Inf)+2。x86 SSE记录默认quiet NaN0xffc00000；aarch64本机运行记录0x7fc00000，只有符号位。Rust官方本地std说明算术NaN符号不保证（路径/hash/短摘录见EVIDENCE.json）；不能用平台不保证的一位冒充业务算法回归。typed canonical迁移没有更改这个生产函数。

本修复对两个确切分支仍断言 expected原值ffc00000、actual.is_nan、完整lower31bits相同；只在非x86_64不要求这一符号位。x86_64全32bits不变；所有finite/Inf/±0及其它case全32bits不变。显式计数必须命中两case；不泛化任意NaN、不扩大数值容差、不改expected或跳断言。

本worker未跑Cargo/rustc/GPU。仅候选rustfmt parse，待render_reviewer独审后root唯一执行原24case与更广现有测试。
