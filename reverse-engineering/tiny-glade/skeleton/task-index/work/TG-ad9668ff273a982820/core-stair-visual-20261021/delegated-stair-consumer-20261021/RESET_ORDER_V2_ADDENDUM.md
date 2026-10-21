# V2 reset第6步的精确写入顺序附录

本附录绑定VISUAL_SCHEMA_AND_LIFECYCLE_CONTRACT_V2.md SHA32d634f9c11e9f57063d7853893420500e17652f9bec8db9d318f1c5da35ecf3，不覆盖旧freeze。

原2322B reset在140aa8daf先floor Vec4.len=0，8dba再wall Vec24.len=0；8dc5..8e0c仅在cell set非空时清ctrl/len/恢复growth_left（依赖std/hashbrown原clear复用，不重写算法）；8e1e再pillar Vec44.len=0；8e26将100/101两个byte置0；8e2d最后f8 qword置0。V2第6步是容器清单，不应据其列举顺序把pillar clear放到cell set之前，源码按本附录的原执行顺序。

9个完整原函数已另用独立PE映射+连续instructionbytes复核（VISUAL_NATIVE_BYTES.json），包括完整reset/remove/Magic clear+Drop和3个必要typed consumer。此附录不改变其它scope和准入。
