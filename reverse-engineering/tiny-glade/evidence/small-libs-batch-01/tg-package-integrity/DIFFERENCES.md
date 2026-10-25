# 差异与修复记录

| 输入/触发 | 原规则 | 早期解释或候选风险 | 最终处理与验证 |
|---|---|---|---|
| 任意清单 raw bytes | BLAKE3 digest 再 Rust array Hash / seeded FNV，最终 low16==0 | 把入口看成 raw FNV，或把 f52dad...当作最终期望值 | 修正为原依赖 Hash/FnvHasher::with_key；真实 B 样本通过，追加空格失败 |
| 数组长度前缀 | 先 Hash<[u8;32]> 的 8-byte 长度前缀，再32byte digest | 只 Hasher::write(digest) 会漏前缀 | 使用原 Rust Hash trait；helper 原机器码以 (state ^ 0x20) * FNV_PRIME^8 开始；原样本验证 |
| manifest.json 自条目 | 精确路径是 Valid，不进行逐文件 hash | 通用 package 验证会产生自引用/不匹配 | self 字节路径特判；假 self hash 的实际 disk 完整流程测试通过 |
| tmp、file.tmp、sub/tmp | 只有根相对 tmp component path 忽略 | 按 .tmp 后缀或任意名 tmp 子目录过滤 | 复用 relative-path join/eq；三个输入磁盘测试区分 |
| 文件读取错误 | 一律 missing/unreadable | 只 NotFound 缺失或返回立即 IO error | 原错误聚合策略；缺失/目录错误静态核对，候选 missing 实测 |
| hash 字符串大写/无效文本 | 不解析期望 hex，精确字符串比较，所以成功read归incorrect | 通用hex parser可能接受大写或提前格式错误 | blake3 to_hex lowercase vs String比较；uppercase实际测试归corrupt |
| 清单重复条目 | 每条检查并保留失败次数 | 用 map 提前去重会丢条目 | 仍Vec逐项检查，known-set仅extra检索；duplicate missing实际保留2项 |
| 正常磁盘包 / 1792项原目录 | 所有条目后聚合三类错误，extra扫描继续 | 逐个首错返回会丢原报告 | 全聚合，当前目录incorrect2/extra288实际记录 |
| 原一次建立1792个并行job | 使用 easy-parallel，稳定文件系统聚合按输入 | 候选照建1792线程可造成不必要资源峰值 | 明确选64项分批同依赖；非1:1时序差异保留，动态写入文件尚未原对照 |
| root选择 / API名称 | 原当前目录固定文件名；原ABI缺失 | 隐式改CWD污染进程 | 新接口显式root，candidate库及example可调用；语义差异为接口表示选择 |
| serde极端属性和OS reparse/error文本 | 已观察主要schema与控制流，完整罕见输入原对照未做 | 候选测试全绿不等于原全部等价 | Unknown登记，不虚报零差异 |

没有修改发行包使其“通过”。没有以测试断言替代原证据，没有重实现标准库/公开依赖。

父独立review后修复测试cleanup边界：删除前canonicalize并要求目标位于本crate target/test-fixtures严格子树；仅测试工具修复，库业务无变化。受影响测试重新验证。
