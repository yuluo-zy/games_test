# 2026-10-20 独立审查方法增补

这是本轮真实遗漏和修正的记录，不替换已经冻结的审查、失败日志或原证。

| 真实情况 | 本轮处理 | 后续审查动作 |
| --- | --- | --- |
| 原 SPV GPU fixture v1 把 `Texture` 传给要求 `Arc<Texture>` 的导入 API，07 的静态审查遗漏，中央实际编译 E0308。 | 保留 v1 源、静态报告和编译失败；独立 v2 仅修真实 owner 转移及 validation collector，中央重新编译并执行 2/2。 | 除比较算法和声明，还要逐个核新增实际调用的参数/返回 owner 类型、feature 可见性；parser 通过不作为 typecheck。 |
| Roof 限定设计用必需 `&Roof` Query，maintain 只检查有 descriptor 的实体，无法覆盖原 marker+key 却没有 descriptor 的更新域。 | 中央指出遗漏，07 追加 `ROOF_DOMAIN_DESIGN_ADDENDUM_V1.json`，要求保原 marker filter，用 `Option<&Roof>` 并对 None 显式 unsupported。原 v1 不改写。 | 分别列 producer、维护 preflight、更新 Query 的实体集合。验证每个前置校验是否覆盖实际消费者全部输入，不能由有 descriptor 的集合推断 marker/key 集合。 |
| 默认 feature 下 system-decorator 不能调用 core 的 native-only constructor，最终 App 的 feature 统一会掩盖独立包依赖缺失。 | 单列 root-only 本地依赖 delta，显式启用现有 native-physics；不返回假几何或 None 成功。 | 检查实际业务 crate 自己的依赖边、最终 App features 与独立包构建域，准确区分可编译范围。 |
| 原首次 CPU384+4+7 的 target 二进制被后续 Cargo 覆盖。 | 历史日志和已记 SHA 保留，明确原可执行不再存在；后续387包和 GPU executable 复制到独立留存路径。 | 构建后先复制实际 executable，再记 hash；不能用后来同名 target 文件替代原测试物。 |
| 固定包 manifest 曾被加入新进度，导致旧审查引用的哈希漂移。 | 恢复逐字冻结文件并另存稳定路径，新观察、纠错、整合状态单独文件。 | 冻结内容只作为不可变输入；状态 append 引用原 SHA，不覆盖原审查事实。 |

新的证据仍按层级记录：原字节/真实调用事实、设计、冻结候选、静态审查、实际编译、实际 CPU/GPU、完整 App/WSI 各有独立边界。本轮原 SPV 的两个真实 Vulkan 用例不能代替 country-core、ProductionRenderHost 或非空世界多帧验收。Catalog 资源、角色 marker、queued receipt、总帧数及退出码也不能代替实际完成、present 和像素结果。
