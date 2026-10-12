# 待验收义务

| 函数 | 原证据与Rust位置 | 消费者/版本 | 验收状态与缺口 |
|---|---|---|---|
| `country_core::resources::walls::WallId::get_inner` | mapping与中文注释行6；原asm具体VA | C目标，ABC同码参照；caller见mapping | implemented_unverified；字段/ABI/依赖兼容及真实producer→consumer→history→removal待补/待验 |

待运行反例：动态网格宽≠64及signed回绕；NaN/无穷/边界/负半径；LoopRange负值；三维矩形上下边同时平移；key互异高位输入；holes origin过滤及未过滤别名；空Vec复用。此列表未执行，不构成验证。
