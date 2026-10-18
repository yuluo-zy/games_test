# 三包专项审查方向

范围：`tg-building-runtime`、`tabloid`、`tg-package-integrity` 当前源码及其调用合同。审查以业务完整性、正确性和高性能为主，优先实际可触发问题；按 P0–P3 记录证据。几何算法依赖只追踪必要调用，不开展逆向，不扩展为整个工作区重构。

## 业务完整性

| 包 | 核对路径 | 验收关注 |
| --- | --- | --- |
| building runtime | authoring → raw source/terrain → publication → path/masks → roof/structures → openings → bricks/collision/Nani | 每个输入是否有消费者；派生结果是否使用同一版本；删除、失败、回放是否清理一致 |
| building runtime | history owner → transaction → undo/redo buckets → authority restore → derived rebuild | 同帧多事件不丢失；起点和终点完整；失败不部分提交；回放不重复记历史 |
| building runtime | missing input / unsupported branch → pending / diagnostic → retry | 区分等待合法输入与缺少实现；补输入后能否恢复；失败时旧几何是否按合同保留或撤回 |
| tabloid | delta → pending batch → independent/shared subscriber → take → reader lifetime | 新订阅只收未来变化；克隆共享消费边界；慢消费者不漏合并；reader不被后续发布修改 |
| integrity | manifest read → raw checksum → schema/version/entry validation → streaming hashes → complete traversal | 每阶段错误能否正确停止；不存在/不可读目录不可假成功；有序报告全部文件差异 |

## 高风险正确性与安全

- 检查公开事件、字节投影和公开资源字段的不变量，特别是 NaN/Inf、无效尺寸、长度错配、重复 ID、已删除实体和无效枚举模式。
- 检查失败前是否已修改权威、元数据、索引、历史或 backend 状态；防止部分提交和缓存继续引用失效输入。
- 检查跨帧资源与事件保留期，调度初始化、deferred commands、标记删除和最终清理的先后关系。
- 检查 Mutex 锁顺序、锁内用户代码/析构、Arc/Weak 所有权及 Send/Sync 合同。
- 检查完整性校验的相对路径、跨平台别名、链接/特殊文件、目录枚举失败，以及稳定文件树合同。格式 checksum 不当作来源认证。
- 检查 unsafe/FFI/Windows API 的真实使用点；没有使用的类别不制造推测性问题。

## 性能验证

| 包 | 重点 | 证据方式 |
| --- | --- | --- |
| building runtime | 无变化帧的大数组 clone、指纹构造、全量重投影、几何/实体重建 | 比较 generation、重建列表、后端请求数量；必要时使用同一 fixture 测时间与工作次数 |
| building runtime | 地形、装饰、屋顶变化后的最小失效范围 | 输入变化与缓存键逐字段核对；证明不漏更新，也不因自身输出反复触发重建 |
| tabloid | 每独立 batch 的 Clone/apply 次数、锁范围、分配及慢消费者状态增长 | 使用现有成本示例与固定/增长状态场景；不把固有累积成本误判为冗余 |
| integrity | 并发上限、每批反复创建线程、小文件开销、大文件峰值内存 | 相同文件树和摘要的前后对照；使用核心数有界的工作者；保持报告顺序和流式读取 |

只优化已证实的冗余工作，记录测试环境与测量限制，不把单次耗时当作普适结论。

## 执行与交付

1. 复用已有业务梳理，建立支持/等待输入/未支持矩阵。
2. 首轮记录并修复确证问题，添加少量能区分错误行为的回归。
3. 交换审查范围，独立检查修复及相邻失败路径，再修复遗漏。
4. 运行包测试、文档示例、严格 Clippy、格式和必要性能对照；记录共享依赖变更导致的阻塞，不能把旧结果冒充当前验证。
5. README 写当前模块合同、功能范围、业务流程、错误与生命周期、性能边界和示例；审查过程及原始日志保存在报告目录。
