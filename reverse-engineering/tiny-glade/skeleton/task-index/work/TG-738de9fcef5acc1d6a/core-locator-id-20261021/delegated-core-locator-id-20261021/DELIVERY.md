# Locator 与独立操作 ID 候选交付

已完成四个唯一 owned 源的候选；`four-source-index.json` 给出当前冻结 before/after 与各最窄 patch。Core Locator 真实含 addr+decorator，现代 Op 显式含 DecoratorId，三历史登记点使用实际 key；change 目标 None 沿源、Some 沿实际目标。legacy storage/event 的 key 输入也改为独立字段，typed 分支仍以 patch.id 为权威，不从可变 Info.word2c 猜寻址。

Peer decorator_chain 的同源 cull V4 已另包补两个真实循环 ID literal，最终 d9bab992；该源码/manifest/增量与完整 patch 已只读核哈希并写索引。没有在此覆盖 peer 的 8143B location validator 候选。

5 新测试覆盖真实 Storage/Archivist/History/Events，验证同墙不同 ID、重复 locator 只合并集合、remove 无新 locator、目标分流与 None 沿源、map key 独立于修改前后的 Info.word2c，以及字段真实失败时已写 locator/未改条目或发事件。测试尚未执行，rustfmt 仅2024语法解析并丢弃 stdout；四 patch 仅作者 scratch 下 -F0 精确自验，等待独立源码审与 root Cargo。

严格保留边界：原 place+54 的完整真实 producer 仍无生产对应入口；chimney 现 lookup local ID 上游仍是已知 dispatcher payload0 缺口；旧事件 codec/裸112B union 和其它复杂字段算法不因本次身份接口修复而计完整。当前证据绑定原 B，不宣称 C 行为等价。
