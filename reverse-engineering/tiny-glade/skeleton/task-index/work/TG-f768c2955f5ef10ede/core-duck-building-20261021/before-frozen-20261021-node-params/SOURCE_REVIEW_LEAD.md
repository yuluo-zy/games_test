# 楼梯 owner/node 独立源码复核（lead）

结论：本包最窄源码范围通过；完整原合同已先独审，现逐项读完整候选与增量 patch，隔离 `patch -p1 -F0` 从真实生产 before 重放，退出0且字节完全等于冻结候选。未 Cargo/rustc/GPU，未改生产；不能因此称完整楼梯链完成。

原840B fold 必须实际storage lookup→bucket→info→cache→extra→可选墙；候选顺序一致，不造query镜像。48B临时point中的 normal 来自实际发布墙曲线的 tangent 换分量并按dst侧翻，width 从真StairExtra+4读取，position来自唯一cache。

原455B derive 跳表已解：真实有类型 WallId+atan2(-x,-y) 墙锚、Roof/Terrain height bits、dst4 unreachable；图 degree count as u8 保原回绕，不把tag索引当返回值。185B determine 先空heights失败，再空points失败；独立首尾点与首尾高度，不强求等长；raw f32 sqrt 与 strict > 分段保 NaN→kind1，第二返回byte固定1有原证。没有epsilon/finite guard。

八新测试读真实World内storage/cache/StairsState与通过InnerWalls→consume发布的真实PublicWalls，覆盖双侧法线、墙身份、cached位置、degree/回绕、extra/ID缺失顺序、height NaN bits及dst4/分段阈值。它们未运行。现derive/type外部调用搜索无生产caller，旧测试已换为实际业务值测试；随后完整request必须真实消费这同函数，不允许另造简写实现。

生产before：`87435b0f6c78d5695d7f1d89a2f8768585d7a62a34ab26a60d4e175ece0427c6`；候选：`966771700efa9f2058fa60eb0e9b328023d8a0b2f8ae09f4c76588de6c88f54d`；patch：`0f5c106e818fa8e1e78f33188d1a87f7217471bf5f23e67cc3415f63ce041ce8`。
