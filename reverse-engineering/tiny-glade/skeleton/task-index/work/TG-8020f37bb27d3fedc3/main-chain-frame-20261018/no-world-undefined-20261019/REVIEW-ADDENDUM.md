# 07 passed_design 后新增不变量落实

原冻结合同8cd6a067保持不变；此附录记录root/07明确要求的反射binding重绑定检查。

- 新增undefined许可只适用于当前唯一的graph texture access；同reflection binding有多条历史access或原生descriptor覆盖时拒绝再次许可。
- push_packet_access，以及raw texture/external buffer覆盖，均清除该binding旧undefined marker。后续绑定默认严格；不会从旧图资源继承许可。
- 宿主独立复核marker有且仅有一条actual access、未被packet.bindings覆盖，并拒绝不存在binding/slot、非texture、write、RMW/preserving及inactive冲突。
- 核心builder测试包含graph→graph/raw/external buffer/rg buffer后恢复strict，并覆盖重新许可stale access拒绝和inactive双调用顺序。

只在合法候选目录写入六源，生产未改；仅rustfmt解析格式，不执行Cargo/rustc/GPU。host混合别名、finalpost flag和blur首pass范围测试一并交中央运行。
