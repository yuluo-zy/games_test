# 鸭子建筑反应候选独立复核

复核者：core_business_lead。只审 `candidate/ducks_react_to_being_inside_buildings.rs`，未运行 Cargo、Rust 编译器、GPU 或原版指令。候选仍为 `implemented_unverified`。

已读取合同、B 完整函数尾段及当前候选。与真实调用参数和副作用一致的关键点：

- 消费原 `CmdCheckForSheepInsideBuilding`，先 `PublicWalls::get(...).unwrap()`，再预制形状和标高门；没有事件即不扫描墙，不用 `OnWallChanged` 代替。
- 使用当前 canonical `ShapeParameters` 的圆/矩形包含算法，保留每鸭 `max_y` 的失败顺序、真实屋顶关联与失败时高度零分支。
- 使用原 Duck 持有的 quack 句柄，不增加音频镜像组件；当前句柄需要可变访问，`bypass_change_detection` 仅用于后端音频副作用，未写 Duck 行为字段。
- 有序闭区间高度判断正确排除 NaN。预制门与 elevation>1.5 的顺序保留；未用默认零掩盖缺失的 wall 最大高度。
- 九项同 World 测试覆盖真实事件读取、canonical publication、音频输出和关键失败分支；测试尚待 root 执行，不能用测试文本替代成功结果。

允许 root 在核对 before SHA 后应用此窄候选。before=`a55a0b7e27519a8b70a96f4a733afddcb6e8b1f84648326318b92389157dd50e`；最终 after=`cc5d323eb671170dfd4f3a878328e241d6c7866f57a86f178883c5721afef4c8`。首次 root 编译发现 pinned Bevy0.16 的 `Events` 资源不支持 `.write`；候选同步为 `.send`，仅修测试 API，业务不变；首次失败日志由 root 保留，运行结果另记。

整条业务仍有上游缺口：真实矩形编辑 finish 事件生产、previous/current UI 快照字段、原 finish 注册和 Duck 的 Active 音频构造。这些已有独立合同追踪；本消费候选不代表建筑编辑到原生音频的端到端闭包完成。
