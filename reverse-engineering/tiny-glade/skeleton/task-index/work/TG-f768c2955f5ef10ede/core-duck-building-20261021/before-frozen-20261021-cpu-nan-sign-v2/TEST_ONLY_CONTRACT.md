# 本机实际 CPU NaN 符号差异：仅测试候选 v2

中央 agent/root 显式要求保 x86_64 原位断言；v1无条件mask已撤回，不应应用。当前唯一本包直接基生产767ace74。

实际同wall偏好case本机只出现7fc00000与原ffc00000的符号差异。原1196B函数 `-difference.signum()` 再乘法仍完整保留；本包仅在该case两输出断言新增架构条件。x86_64继续完整断言ffc00000；非x86只屏蔽signbit，仍完整要求lower31bits=7fc00000。其它业务值、strict比较、metadata rawbit比较、算法顺序和阈值未改。

F0精确重放，未Cargo/生产修改。原x86的runtime差分依旧由精确断言约束；非x86本case明确未承诺NaN符号等价，不把本窄测试适配扩大成全部数值容差。
