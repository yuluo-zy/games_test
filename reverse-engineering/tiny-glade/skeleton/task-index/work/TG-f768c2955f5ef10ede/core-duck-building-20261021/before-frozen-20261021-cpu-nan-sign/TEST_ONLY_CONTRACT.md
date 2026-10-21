# 本机实际 CPU NaN 符号差异：仅测试候选

中央 agent 显式授权本任务沿现 source767ace74 调整跨ISA NaN sign-class；不改生产数值或阈值。实际日志 tg-business17-system_decorator-tests-20261021.log 中 samewall_preference_negates_signum_before_nan_payload_multiplication 仅 x 断言得到7fc00000而期待ffc00000。

原1196B偏好函数仍按完整合同保 `-difference.signum()` 再乘法，不改为 `(-difference).signum()`；原 x86 constant 负NaN作为证据保留，跨ISA结果不承诺NaN符号。此测试仅对最终两个 NaN 结果屏蔽符号位，仍逐位要求quiet canonical payload7fc00000，保原tag断言以及上游真实world/storage输入。未改有限值、strict比较、失败门、业务函数或其余测试。

本包只有两条assert及相邻中文原因注释。F0精确重放不等于CPU通过；未Cargo/生产修改，交root实际重跑。
