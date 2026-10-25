# 独审续接：整池统计、维护顺序与装饰验证

截至本次报告，reviewer 只读生产、原 RNE/PDB 和实际日志；没有运行 Cargo/GPU、没有提交 Git，也没有接管其他 claim。生产整合与实际验证唯一 owner 是 root。原 B 发行身份及各候选精确 SHA 由相邻 JSON 绑定；不得拿随后重建的二进制替换已保留运行输入。

## 已审交付

- 原版真实资产加载测试：`REAL_B_DECORATOR_ASSET_TESTS_V1_REVIEW_V1.json`。原 loader/旧测试逐字不变，真实完整安装树四表、网格拓扑、shape 与 Arc 身份；必须显式运行被 ignored 的两项，0 tests 没有证明力。
- 跨 ISA 的两处算术 NaN sign：`OFFSET_GOLDEN_ARITHMETIC_NAN_SIGN_V1_REVIEW_V1.json`。原 499B 与运算分支独核，只有非 x86_64 的两处已知 arithmetic quiet NaN 放开 sign 一位；原数组、31 位载荷/quiet、其它值与生产均不变。
- 整池真实同时高水位：`POOL_SIMULTANEOUS_HIGH_WATER_V1_REVIEW_V1.json`。旧 sum(page historical peak) 是上界，不是整池同时峰；新事件式纯 counter 保旧 heap 历史，预算/多池/失败页/注销/锁序已审。此候选不改变 lease/GC/容量/绘制，不能先许诺长稳门会通过。
- OOM 跨队列回滚：`RETIREMENT_MOVE_OOM_ROLLBACK_V1_REVIEW_V1.json`。queue0=5/queue1=100 的迁移失败必须恢复原 value，避免跨时间线阻塞源队头；不归因为当前世界内存问题。
- headless 维护 V1 已否决且未应用，见 `HEADLESS_MAINTENANCE_V1_ORDER_BLOCKER_REVIEW_V1.json`。V2 通过静审，见 `HEADLESS_COMPILE_THEN_DRAIN_V2_REVIEW_V1.json`。原顺序是 compile→begin/drain→execute；独立原 PE call bytes/PDB 在 `HEADLESS_MAINTENANCE_ORIGINAL_ORDER_NATIVE_ADDENDUM_V1.json`。新 compiled 的 marker/callback/registry/result-cache pin 与 projection 重建已核，仍需 root 实测。
- 真实 GameLoop WSI 入口：`ACTUAL_GAMELOOP_GAMEPLAY_WSI_HARNESS_REVIEW_V1.json`。实际双 App、非空场景、同 World、唯一 receipt、至少 12 次 raw native present、真实关闭及保存线程 owner/注销门；只是待执行验证入口，不是已通过结果。
- 装饰 mute V2、icon leaves、完整蓝图验证 consumer：分别见 `PAIRED_DECORATOR_MUTE_GUARDS_V2_REVIEW_V1.json`、`DECORATOR_CURSOR_ICON_LEAVES_V1_REVIEW_V1.json`、`COMPLETE_BLUEPRINT_VALIDATION_CONSUMER_V1_REVIEW_V1.json`。六完整范围及实际两跳表逐条字节核 B，11 reason/lookup/partial side effects/type62 continue 均审；typed accessor 与 startup leaf 由 core_business_lead 独审。完整宿主 mount 未授权为完成。

## 实际运行事实与失败

- 增量 prefix：`INCREMENTAL_RANGE_PREFIX_GPU_RESULTS_REVIEW_V1.json` 绑定同真实 635 cells/3536 patches/1478 reads 输入，状态准备约 11.19s→0.122s；两个实际 PPM 逐字相同。不是整个游戏完成证明。
- 共享常量池 30 帧：实际执行全部帧，正式失败于旧聚合 peak；`SHARED_CONSTANTS_THIRTY_WORLD_FAILED_RESULT_REVIEW_V1.json`。live 门已执行并过；后续内存/像素断言未执行。保存 PNG 独立离线最后间隔约 2.031/2.889/0.00320，在原像素门内，不替代实际 Rust 断言。
- 真整池 counter 30 帧：实际执行全部帧，正式失败于 live 3042..6022，见 `TRUE_POOL_HIGH_WATER_THIRTY_FAILED_RESULT_REVIEW_V1.json`。peak=6022 固定，index2 含两批真实命令 owner；旧帧不足飞行上限却尚未 finish 的路径有具体源码支持。完整 completion 不能凭单个 cached sequence 证明。禁止改采样/steady 起点/阈值、加 idle 或提早释放真实在途 owner。
- loading WSI 的旧实际 3 SUCCESS/SUBOPTIMAL 与关闭结果仅属 loading path，详见既有 v3 review。尚不能当作 gameplay WSI 的新结果。

## 仍需维持的边界

root 后续必须保原 30 帧全部断言，执行 V2 bridge 与真实 WSI；失败日志保留。真正两帧同时在途时 live 可能合法不同，不能靠删 owner 或统计重命名掩盖。保存线程 join/detach 不证明编辑数据落盘或重启恢复。蓝图验证 consumer 源码可调用，不意味着旧整套 set25、所有 cache/arch/geometry producer、stairs 泛型 caller、legacy type writer、gothic、卸载/删除、编辑恢复闭环已可挂载。

## 本轮可复用经验

1. 调度顺序读 consumer 前后完整区域，并核原 call/PDB；不能由 drain/Startup 等名字推断位置。这里在 source review 阶段拦住 V1 顺序偏差。
2. 运行输入同时锁定 source epoch、实际二进制、原证和完整日志；ignored/filtered 的 0 tests 不能当通过。
3. 峰值必须先明确统计域和时间语义。sum(max(page)) 不等于 max(sum(page))，修统计也不保证业务长稳。
4. 新 compiled 与旧 pending 维护要逐 owner 确认：retire callback marker、result-cache pinned tokens、projection snapshot 和真正 raw command owner 各自独立，不能仅看一处 Arc 数。
5. 跨 ISA 只有已证具体算术差异才可作最窄测试合同修正；原 oracle 与生产算法不动，未知载荷和其它 case 不泛化豁免。
6. 读冻结包先核 SHA 与确切版本。本轮另一 reviewer 的 grep 混入 V1 `.write`，即时读 V2/复核 SHA 后撤回疑点，没有无谓重写 V3。
