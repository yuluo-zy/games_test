# 功能完整性矩阵

本矩阵区分已有实现、等待输入与明确边界。等待输入不等于缺少算法；模块存在或组件已注册也不代表所有后端能力已完成。

| 模块链 | 当前能力 | 必需输入或边界 | 可观察结果 |
| --- | --- | --- | --- |
| 墙发布 | 原始曲线/地形采集及已解析轮廓发布 | 原始链需要有效 512² 地形；已解析入口不补造原始曲线元数据 | PublicWalls、版本、高度缓存、faults |
| 路径/墙分段 | 路径栅格准备、初始及平台分段 | 显式 path rasters；仅使用当前墙发布版本 | records、pending、generation |
| 屋顶 | 创建、关联、空间同步、方向/参数/样式编辑、动画删除、CPU 瓦片 | 有效规则形状墙；输入及派生几何必须有限 | RoofBundle、RoofCpuTiles、变化与删除事件 |
| 自由墙缓存 | 基于原始曲线和 prepared segments 的高度 | 单独 PublishWall 不能替代 raw/prepared 数据 | pending 或有效缓存 |
| 过梁/拱门/砖块 | CPU 实例生产及结构高度联接 | 原始权威、当前路径、分段及有效高度；未支持 producer 分支显式 pending | 实例、generation、rebuilt/pending |
| 台阶与平台碰撞 | 装饰行→台阶/洞口→真实 Rapier、脏瓦片及射线 | 显式地形、装饰行、脏瓦片 owner；查询范围为平台后端 | CPU 记录、collider diff、query result |
| 木门 | 拱门命令消费、CPU 网格/支撑、删除与缓存失效 | raw/public/path/terrain/source metadata；底层失败有 pending | 门实体、mesh assets、pending |
| 栅栏 | special flag 对应分支的 CPU 输出 | 原始源和有效路径；GPU SparseWriter 外置 | FenceEntities、CPU 数据及 pending |
| 装饰/洞口 | 碰撞资产解析、legacy/native 绑定、放置、归档恢复、洞口汇总 | 显式资产与记录；身份唯一；可恢复失败和永久坏请求分别处理 | opening authority/state、pending、holes |
| 历史 | Roof owner 与已接入 Wall 操作的事务、撤销重做、串行回放 | 显式 ledger/reset、recording、地形及源元数据；不支持的 owner/Wall 操作在提交前拒绝 | 队列、ledger、replay buckets、诊断 |
| Nani | Roof StoneFloor / SolidVertexColor 的最终 CPU 快照及变化 | Named mesh 需要 atlas；真实激活由外部 backend confirmations 确认 | deferred requests、CPU staging、diagnostics |
| 图形与触发器 | 瓦片 CPU 结果和标记组件可用 | RoofTiles GPU allocation 尚为空；Spire/Support marker 不等于已生成绘制；TriggerVolume 当前未构建有效触发三角形 | 不将空占位资源报告为渲染完成 |
| 声音/颜色 | 产生反馈、播放意图及恢复请求 | 音频设备/完整外观应用由外部消费者处理 | feedback/sound/color 事件 |
| tabloid | 有序发布序号、按消费边界合并、不可变快照、克隆共享消费边界 | State 独立 Clone；empty/clone/apply 不重入且不 panic；无持久化/背压 | PublicationReader、SequenceExhausted |
| package integrity | 格式 checksum、schema/version/entry、文件摘要和完整扫描 | 稳定文件树；版本 1；checksum 不是签名；不提供原子文件系统快照 | 分阶段错误或 missing/corrupt/extra |

屋顶类型切换需要原始墙高度 owner 时，缺少 raw authority 会给出当次 pending 诊断；补齐输入后需要重新发起编辑，不承诺保留并自动重放这条用户操作。

未支持的产品能力与缺失的外部资源分别记录；本轮修复范围内，已经存在消费者的事件遗漏、失败后半提交、状态不一致和冗余工作均按问题处理，不用历史兼容性保留。
