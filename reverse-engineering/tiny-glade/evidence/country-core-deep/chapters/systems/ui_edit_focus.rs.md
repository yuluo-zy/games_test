# systems/ui_edit_focus.rs

> 文件作用：编辑焦点系统：按模式 id 分发（mode<6 注册 'edit_focus_canvas' widget（tag5/f32::MAX）；mode 1/2 跟随路径过 FUN_140a1c930 谓词；0x2a8 反向步长遍历 wall 数组并断言 simplified_curve_2d 有效性；mode==10 特判；4 张旋转跳表 = 24 变体 GeoSource 匹配；结尾推 {tag0, 计数} 0x18B 事件）。geo_src_to_edit_subject(_w_deco_clutter) 做 GeoSource→EditSubject 映射。

函数总计 5：业务 3 / 内联无独立符号 0 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ui_edit_focus::ui_edit_focus` | 0x1409771f0 | 0 | 系统:周期性处理（ui_edit_focus） | O：完整结构（DIGEST §2.5）；两条 panic 串：'assertion failed: w.simplified_curve_2d.is_valid()'（0x142ada193）+'internal error: entered unreachable code: ui_edit_focus shouldn't include '（0x142ab6a70） | ui-input\decomp\decomp-0x1409771f0.txt |
| `country_core::systems::ui_edit_focus::geo_src_to_edit_subject` | 0x140977f50 | 312 | GeoSource→EditSubject 映射 | O：172KB（含内联）；EditSubject 变体 serde 名全套（WallId/wall_id/Floor/Roof/flat/gable/StairGraphEdge/DecoratorCollision/DecoratorInteraction/StairInteraction/ColumnStairs/Fence/Arches/Tree/Terrain/AutoClutter/entity 等，串 @0x142ab7268 起） | ui-input\decomp\decomp-0x140977f50.txt |
| `country_core::systems::ui_edit_focus::geo_src_to_edit_subject_w_deco_clutter` | 0x140978000 | 317 | GeoSource→EditSubject（含 deco/clutter） | O：_w_deco_clutter 变体额外考虑 deco-clutter 选择（同 0x140977f50 族，172KB 反编译） | ui-input\decomp\decomp-0x140977f50.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 2 项
