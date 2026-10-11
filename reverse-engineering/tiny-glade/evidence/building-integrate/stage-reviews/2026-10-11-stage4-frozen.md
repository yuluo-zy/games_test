# 阶段4冻结 review：墙拱、台阶与真实事务消费者

当前冻结Rust/Cargo的54项工作区测试、release workspace build、67项声明范围的原码/运行链验证均成功。113个registry依赖身份匹配B；原fork/features、C、GPU和全部建筑系统没有据此接受。输入和所有汇总保存在stage4-frozen/summary.json，下一源码变化即取消当前性，不修改旧hash冒认新proof。

## 连续输入到消费者

- 普通/自由拱130/270例、4159/9691条96B，290/363个120B游戏门命令。真实12帧/22墙帧318拱、4 Bevy EventWriter命令、3654墙砖按原prepared/cache/calc/platform/arch/third/fullfromcurve逐字节闭合。EventId8是依赖字段，不属于游戏分配计数。
- 台阶96 renderer案例6484条96B；80B实际装饰源、原steps/materialization、随机分行、碰撞顶点/索引、每段holes、原+a4累计和+90扣长都由原码验证。真实9帧/8墙帧16cells/8holes/928台阶记录/2073墙砖闭合；width100实际手工装饰blocker抑制、撤回恢复、height/Roof/path/delete和80B来源清理均有运行回归。RaycastWorld插入/查询、UI手工producer没有冒称完整恢复。
- Wall/Roof统一历史按原外层bucket和内层同Wall组barrier推进；source/meta/color/40B高度请求/实际原始采集/cache/openings/brick恢复已接真实World。拆除原录制是RoofDelete先于Wall StateChange op1 shrink→Delete；原动画.2先phase推进，history.4先eval后elapsed。whole动画768原RET和producer128业务段证明，不宣称上游hit/range/UI整入口已完成。实际创建、直接删除、bulldoze动效、undo/redo、ID不增及最终砖一致性回归通过。

## 独立审查与修复

独立review文件2026-10-11-stage4.md和inputs.json保留其自身快照。P2：unknown owner Drain丢失败/后续队列，改预验和成功后移除；旧无recording Invalid/creation观察不得回填后续事务；本帧cleanup放consume/edit/bulldoze后，避免旧Remove次帧误清freshReplay metadata；active bulldoze缺metadata保持命令/phase并报错，不能当源码不存在；Mark之前association可能看到刚变shape2，已录Roof owner仅抑制同帧同Wall的历史echo，保Invalid清理。

旧Openings CLI fixture使用立即多owner redo，完整原bucket恢复后首次报active bucket错误。根据原Roof→Roof不兼容且每调用只pop一次的证据补实际分帧消费间隔，15帧全部通过；初始失败保存在stage4-frozen。仅失败任务在完全相同Rust/Cargo输入上实际重执行，summary.retry_history保留前次错误；其余66项仍对应本轮同一冻结输入。

## 仍需攻关

扩大16 stairs consumer研究保留10 InvalidInput失败；冻结诊断已锁定probe漏写矩形axes，与native完整shape不一致，71cells/3331条记录用真实shape均一致。下一修probe并整链重跑，不能据此把未经验证的新范围算完成。原Rect finish/enclosure replay/PendingWallChanged研究已持续取证但Rust未接；Gate120最终mesh/TransientMesh/Nani及实体消费者、Pub29a实际owner、原UI/startup/timber/其他结构与历史/存档/原调度/C/GPU继续。ARM Roof case14 filler Y一ULP失败仍保留。system_acceptances=0。
