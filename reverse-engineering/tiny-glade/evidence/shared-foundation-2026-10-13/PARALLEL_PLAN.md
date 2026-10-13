# 共享基础：按原源码体量分配的六个 agent

用户指令（2026-10-13）：六个 agent，先处理源码目录较少的底层模块。当前会话最多同时运行三个子 agent，因此六个独立 owner 分批轮换；任务分配不计作恢复验收。

| 顺序 | Agent | 独占原源码范围 | PDB 文件所在目录数 | PDB 原文件数 | 独立工具 worker | 交付要求 |
|---|---|---|---:|---:|---|---|
| 1 | foundation_paths_language | paths/src、theme-defs/src | 1 + 1 | 1 + 1 | foundation-paths-language | 路径策略、活动事件真实类型、调用者与格式边界 |
| 2 | foundation_utils_tabloid | tabloid/src | 1 | 1 | foundation-utils-tabloid | 发布/订阅/读取策略、真实泛型调用者、锁依赖复用 |
| 3 | foundation_lang_small | lang-lib/src | 1 | 2 | foundation-lang-small | locale 加载、回退、热切换、持久化和开发工具缺口 |
| 4 | foundation_assets_small_first | asset-pipe/src、asset/src | 1 + 3 | 5 + 13 | foundation-assets-small-first | 先 asset-pipe，再资源格式/扫描/继承消费者；继承旧 assets 成果 |
| 5 | foundation_geometry_curves | utils/src/components、curve、geometry | 1 + 1 + 2 | 1 + 4 + 15 | foundation-geometry-curves | 先 Transform/曲线小目录，再几何，复用已核实原码证据 |
| 6 | foundation_utils_base | utils/src 顶层文件（lib.rs 除外） | 1 | 22 | foundation-utils-base | 小型工具逐文件恢复、原依赖调用规则和 WallSpace 接线 |

父 agent 负责 workspace Cargo、utils/src/lib.rs、全局模块进度表及包关系图。第 5、6 个 owner 在前序交接完成后创建，避免同时改写同一文件。第 1 个原来负责 lang-lib，第 2 个原来负责 utils 顶层；两者的已恢复文件保留并交接，不能另起一份重复实现。旧 foundation_assets 因服务容量错误中断，新 owner 接续同一源码目录，旧证据保留。

上述原文件数来自 tracking/module-progress.json 的 A/B/C 联合 PDB 路径库存，是任务排序依据，不代表单版本文件数、完整函数数量或复杂度。本轮持久工具绑定 B：asset-pipe 的 B 原路径只有 lib.rs/texture.rs 两个，asset 的 B 原路径只有九个，其余新增路径属于 C，不能拿 B 的实现/合成glue视作 C 已恢复。[分版本目录/文件排序清单](source-size-by-version.json)保存实际分母。公开库必须直接复用原作 Cargo.lock 版本；只恢复游戏自有算法、包装和调用策略，不凭 local package 标签认定私有。

目录数为 PDB 文件所在的不同父目录数量，不推断完整作者仓库。先少目录，再用文件量/业务边界排序；组合任务内部仍先做小子模块。当前六组均已建立：1、2已交付，3源码及验证已完成并由父登记报告，4、5、6继续并发处理。

每个 owner 独占证据子目录，报告原文件→恢复文件→函数映射、原码/PDB/调用者依据、编译结果、行为验证及尚未恢复项。编译通过与算法等价分开记录；不将静态反编译称作原入口执行验证。未证实 ABI、内联函数和错误语义显式保留缺口。
