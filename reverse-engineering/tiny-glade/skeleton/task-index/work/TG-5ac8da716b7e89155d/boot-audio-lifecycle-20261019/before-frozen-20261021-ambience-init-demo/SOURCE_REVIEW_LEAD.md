# 水声初始化演示门独立复核

直接复核原初始化3955B中的 `1409ebf74` 比较与 `1409ebf7b` 跳转，以及已绑定的 demo 实体/IAT 指针身份。当前源确实在任意模式都会创建 loop/grid；候选在所有分配与 Commands 插入前借用已注入的 DemoModeEnabled，语义与现 water_audio_init/ambience consumer 保持同一 owner。

候选只添加可选只读参数与开头早退，保留已整合的 canonical sampler 和原有算法。三个真实 ECS 测试用实际 DisabledStudio 分支覆盖不创建、正常100格100环、早退不替换现有 owner；没有用默认 WaterAudioGrid 掩盖缺初始化。允许 root 整合并执行测试。

before=`7a26c15b1076680f59acaac3a1335e6070c95c3cc3966933da2827f994a353ba`，after=`b950abf8938ed1d484fa8b117dabe8d09caaf80b34ef74834f09cfcb0f24103a`。复核者 core_business_lead；未编译或执行，原 C 与实际 FMOD 验收未声明完成。
