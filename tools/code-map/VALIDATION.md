# 迁移工具验证（2026-10-30）

本次整理保持原数据库和证据，验证对象是跨路径安装、迁移与查询入口。未将这些检查升级为全软件恢复验收。

- code-map 19 项测试通过：身份错误、损坏缓存、指令、查询边界、只读接口、快照隔离、连接复用和全局过滤时保留业务 ICF 别名/共享范围。
- 新增迁移 8 项测试通过：损坏包在写目标前拒绝、拒绝覆盖、逐字节复制/保留来源、越界和未列文件、符号链接和未发布包、运行中实例离线导出保护、含中文与空格的新路径，以及嵌套目标目录不重复映射。
- 依赖锁从官方 PyPI 生成，并在独立 Python 3.14 venv 中以 `--require-hashes` 安装通过。核心无需 MCP。
- `doctor` 在当前 macOS 实测 Java 21.0.5、LLVM 22.1.2 X86 解码、collector 来源哈希与空间前提。
- 关联扫描器另有 3 项测试，覆盖机械名称排除、new/get/insert/remove 要求 owner 原证，以及保留具名业务函数。

可复跑命令（工作区根）：

```sh
python3 -m unittest discover -s reverse-engineering/tiny-glade/tools/code-map -p test_code_map.py -v
python3 -m unittest discover -s tools/code-map -p test_portability.py -v
python3 -m unittest discover -s tiny-glade/tools -p test_inventory_associations.py -v
python3 tools/code-map/manage.py verify-bundle tools/code-map-transfer-2026-10-30
```

迁移包的 `manifest.json` 记录实际文件数、SHA、字节量与原快照；离线模式含 `dumps/neo4j.dump`。实际目标端 `manage.py check --full` 输出保存在该端 runtime 的 `transfer-validation.json` 和 `verification-full.json`。它同时核真实屋顶碰撞/raycast 链、全部已提取原 B 字节、节点/关系数及已记录的合同样本。

本机独立目录演练使用单独端口；其完整结果放当前 `tools/code-map-runtime/portability-validation.json`。跨物理电脑、Linux/WSL2 和实际业务恢复效率仍须目标端验收，不能从本机路径迁移推断已验证。

## 全图清理实测

- 全部物化快照实际删除 **1,319,915** 个节点及其边；按逻辑入口计，A/B/C 纯机械入口分别为 9,284 / 10,519 / 11,277，370 个混合业务别名入口保留。
- 新快照 `5ac3dab66b4ac8c04a89025c`：**1,989,298 节点 / 3,600,990 条 LINK**。旧当前快照为 2,324,845 / 4,270,550。
- 全库审计 **1,621,000** 个符号/内联实体，机械名称残留 **0**；搜索索引 ONLINE；模块依赖重新投影为 332 个模块、4,383 条依赖。
- 全范围原字节与真实业务链验证通过，failures=[]；新依赖锁环境的实际 MCP 九工具/写拒绝/未发布快照拒绝验证通过。
- 可复核摘要见 [`global-filter-results.json`](global-filter-results.json)。原始提取覆盖不受图层过滤影响，未宣称全软件语义或游戏验收完成。

## 独立恢复演练结果

清理后离线包（6,537 个文件，5,442,709,624 字节）已从校验、空目录恢复、独立 Java/Neo4j 配置到 dump load 完整跑通。新目录、新端口、新凭据下的快照 ID、节点/关系和合同样本与原库一致；全范围原 B 字节与真实 MCP 调用通过，凭据权限为 0600。记录见 [`portability-results.json`](portability-results.json)。本次没有跨物理电脑运行；Linux/WSL2 仍需目标端按指南验收。
