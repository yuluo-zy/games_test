# M06 既有建筑实现复用核查（2026-10-28）

用户记忆有依据：此前确实恢复过建筑CPU算法和真实Bevy开发运行时；大量实现已经迁入正式工程，不应根据原函数同名匹配率重复恢复。

## 已核查的来源与当前使用

读取 provenance/building-audit.json，限定 source 含 reconstruction/building/ 的记录，共50条。实际检查旧源和 current_path：50对都存在，1对字节相同、49对内容不同。该计数包括屋顶/碰撞/装饰等共享建筑能力，不是M06全部函数数量。内容不同不表示算法丢失，不能反向覆盖当前实现。

| 旧实现 | 当前归属/实际使用 | 结论 |
|---|---|---|
| building/wall_production.rs | country-core/src/systems/wall/construction/production.rs；construct_wall_bricks.rs 调用 produce_wall | 已迁入并被当前生产系统复用 |
| building/wall_structures.rs | country-core/src/systems/wall/construction/structures.rs；tg-building-runtime/src/wall_structures.rs 调用 construct_lintel_supports | 已迁入；过梁、结构高度和分段叶算法可继续用 |
| building/fences.rs、gates.rs | country-core/src/systems/wall/construction/fences.rs、gates.rs | 迁移记录存在、当前文件存在；完整分支原侧验证未在本次重跑 |
| 建筑发布/历史/洞口等旧实现 | resources/walls、systems/wall/construction、systems/history；开发运行时壁垒见FEATURE_MATRIX | 现有能力，按真实权威和输入边界接入，不另建平行实现 |

当前 system-wall-constructor/src/construct_walls.rs 直接重导出 core::systems::wall::construct_wall_bricks::construct_wall_bricks；主宿主 app_systems.rs 的set19已有墙编辑/发布/分段链，set28已有门、栅栏、洞口和墙砖链，set34历史、set44上传。不能把旧算法只当未接线孤岛。

tg-building-runtime仍是已有CPU/Bevy开发验证入口；主游戏没有搜到直接使用其BuildingPlugin。当前主宿主使用领域系统接线，因此不建议整体再挂开发BuildingPlugin，否则可能出现重复资源和重复更新。

## 旧代码不能直接填掉的缺口

- 正式 construct_floors 函数体直接todo；旧 reconstruction/country-core/src/resources/walls/mod.rs 的 Floor 仍是 pub struct Floor(pub Gap)，不是完整旧楼板实现。
- 圆柱/矩形柱/托架、灰泥等原完整系统与过梁 construct_lintel_supports 不是同一职责；不能以有support命名就替换。检索到当前同名系统仍是显式未实现，未找到旧building族同名完整实现。
- 当前construct_walls重导出说明仍明确排除corner/skirts/signifier附属几何；已有主墙砖不能推定这些完成。
- 当前sample_wall_path_intersections及各规则形状UX仍需单独沿现有源码审计，历史报告仅作定位，不在本次宣称已修复或仍缺。

## 对前次统计的修正

M06的314/900（34.9%）只反映词法候选，不能衡量上述改名/迁移/重导出的实现覆盖。该值不得用于判断M06复原完成度。下一轮逐函数分类应把building-audit.json的迁移映射及当前真实调用链纳入后再计数。

本次只读复核和记录；未修改业务源码，未运行编译、测试、原码差分或GPU验收。历史FEATURE_MATRIX、DETAIL-building仅用于来源和边界定位，本结论以明确读取的当前调用点与函数体为限。
