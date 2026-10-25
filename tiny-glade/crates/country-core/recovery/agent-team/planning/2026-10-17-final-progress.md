# country-core 模块还原进度终版图(2026-10-17,深收口+依赖簇全部交付后)

> 进度 = (判等 + 重建闭合 + 深收口/依赖簇实现) / 模块业务函数面(排除宏生成胶水)。
> 依据各模块 LOGIC/REBUILD/REVIEW 账本;反假实现专项审计确认判等声明可信(55 函数独立复验 0 新假逻辑)。

```
模块                已还原/面  进度
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
input              28/28   ████████████████████  100%
steam(业务)        22/22   ████████████████████  100%
render_loop       115/115  ████████████████████  100%
world_raster/paths 27/28   ███████████████████▏   97%
music               9/9    ███████████████████▏   96%
ivy                22/23   ██████████████████▊    95%
garden+sheep       16/18   █████████████████▊     90%
session+music+cam   28/30  █████████████████▏     90%
history            53/60   ███████████████▉       88%
save+meta          60/70   ███████████████▉       86%
roofs+stairs       76/90   ███████████████▉       85%
camera/time/ui_mode 49/60  ███████████████▏       82%
terrain+ui_clutter 28/35   ████████████████▏      80%
audio              64/85   █████████████▊         75%
UI域余量           41/56   █████████████▊         73%
walls              202/355 ███████████▍           57%
collision          86/135  ████████████▍          64%
```

## 汇总

| 维度 | 数值 |
|---|---|
| 累计已还原 | ~1100 函数 |
| 已闭合(实现层面)模块 | 16 个(深收口 7 路 + 依赖簇 4 路 + 首批) |
| 高进度(≥80%) | 12 模块 |
| 中进度 | audio 75% / UI 余量 73% |
| 低进度(大模块) | walls 57%(~240 待判等,回派实现中)、collision 64%(hardpoint/设备面剩余) |
| 编译 | lib 绿(双特征 collision);test 目标被 rhapsody 在途暂挡 |
| 反假审计 | 0 新假逻辑,判等声明可信;LOGIC 过度判未实现 1 例已修正 |

## 剩余主项

1. walls ~240 待判等 + eraser finish/create_rectangle(history 差分类型已解锁,impl-walls-return 在岗)
2. collision:update_rapier_colliders(decompile-hardpoint 需更深入定点)、find_mesh_mesh_intersections 跨域、rt 设备面(obvhs/rhapsody)
3. 未实现无 body 依赖前置已登记(collision monolith/rt/particle 写面/prefab spawn/startup 资产绑定)
4. 宿主接线(全局调度/渲染消费桥/域 Query 全链)——完整 core 验收前置
5. 全 crate 绿窗口 + cargo test 基线(rhapsody 收口后重验)

## 台账指针

- 总账: planning/2026-10-17-logic-master-ledger.md
- 协议: planning/2026-10-16-logic-verify-protocol.md(含宏生成分类规则)
- 反假审计: reports/logic-audit/FAKE-IMPLEMENTATION-LEDGER.md
- 各模块: reports/<组>/<模块>/{LOGIC,REBUILD,REVIEW}*.md
