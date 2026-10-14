# 建筑 CPU 功能模块

本模块收纳此前漏掉的已恢复建筑业务：墙权威状态、构造与发布、缓存、空间索引、分段、拱门、洞口、台阶、碰撞策略、装饰附着、屋顶生命周期及历史联接。保留功能文件名，旧 `building_*` 名称只是本模块内的兼容别名。

## 来源

原逆向文件位于 `reverse-engineering/tiny-glade/reconstruction/building/*.rs`、`history/transaction.rs` 和 `history/building_transactions.rs`。原目录 `SOURCE_RECOVERY.md` 与 `reconstruction/README.md` 登记原函数范围、原码案例和未恢复边界。过程包含 Git `156add05`、`c3625e1d`、`531b7d32`、`0f9ded43` 等提交；每文件实际最新提交和哈希见 [building-audit.json](../../../../provenance/building-audit.json)。

## 功能与已完成部分

- `wall_*`：已解析墙状态的采集、缓存、发布、生产、空间脏格更新，历史编辑、撤销、重放和洞口失效。
- `roof_*`：关联、创建、组件参数、空间同步、方向/外观调整、删除动画和失效通知。
- `opening_rules`、`decorator_*`、`gates`、`fences`：已解析输入上的候选和参数策略；未识别输入语义继续由调用者显式提供。
- `nani_solid_vertex_color`：已恢复 CPU 请求布局，与 `render/nani` 中 GPU 资源及绘制管理角色不同。
- 两个 history 模块：事务边界和已解析建筑事件联接。

各文件的证据分别为原码差分、静态分析或调用合同；迁移不把它们统一提高为全域验收，原 `UNVERIFIED` 等注释保留。此处 resolved CPU projection 与渲染恢复的资源类型服务不同证据范围，尚未证明是统一的原完整类型/ABI。

## 还差什么

完整原作 ECS 排序与引擎分叉、源场景与资产消费者、部分 freeform/结构消费者、GPU 上传和原碰撞资产接线，以及超出报告输入域的边界。`wall_platform_rapier_backend` 是实际依赖适配代码，启用 `country-core/native-physics` 才可使用；它属于工程接线，不宣称原查询系统完整等价。

## 验证与开发

现有测试随 `cargo test -p country-core` 运行；旧原码报告保留在原 `evidence/building-*` 等目录。当前整合结果见 [validation.json](../../../../provenance/validation.json)。数值/API 的局部 lint 例外登记在 country-core 的 lint-preservation.json，不改变原求值顺序。
