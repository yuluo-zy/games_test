# tg-text：差异检查与修正

| 触发输入/易误读点 | 原业务规则 | 当前实现 | 复验 |
|---|---|---|---|
| RON source数字 vs Rust image(u32) | RON仅string；Rust存在From<u32> | 避免无依据untagged二选一；RON拒数字，builder保留index | 原deserialize_str调用静态 +候选解析测试 |
| encode_no_images中的indexed image | 仅named image调用不可达resolver；indexed有效 | 按分支保留，不添加全局拒绝image规则 | 原vtable/branch静态 +候选成功/panic |
| image index超过65535 | 保留低16bits，静默截断 | 相同mask，无额外错误 | 28组原indexed +混合prefix零差异 |
| radius负值/NaN/Inf/小数 | 可观察word是clamp-and-truncate | std f32 clamp+Rust cast | 12组原边界逐word零差异 |
| font scale负值/NaN/Inf/±0 | 两metrics乘scale，不净化 | f32乘法、family借用 | 21组原入口逐bits零差异 |
| Raw::encode仅inline | 在append内写raw words，不是读取words | 与Image::with_scaling一起合并进append；words()是新访问接口 | 原IPIID0x86cf/0x86ce +51组原append |
| 同域schema缺字段 | fonts/family/source/scaling必填；缺scale=1；重复字段错误 | 复用serde derive与原RON fork | 3个原schema宿主静态 +候选缺失/重复测试 |

72组直接对照的原数据没有剩余运行差异。以上描述的是已核实规则的保留与实现选择，不宣称曾经运行过一个失败的旧实现。原serde、unknown字段策略、真实纹理name catalog、分配增长、TextFragment、Renderer和A/C仍有未验收范围；不是零差异。

工具修正：初始seed为一字节placeholder，按原PDB范围与字节SHA在自己的clone修复；assembly查询完成，但checkpoint atomic replace出现WinError5，总报告ok=false保留。后续schema job成功checkpoint；正常stop状态另见worker-stop.json。工具故障不改变已取得的原native数据，也不算游戏业务进度。
