# B 目录对照：人工源码复核（2026-10-28）

本文件补充 B-MODULE-INVENTORY.md 的机器定位结果。该初筛不是完成率；当前尚未完成全模块逐函数人工分类。未运行编译、游戏或原版差分。原B源码骨架和发行目录在当前检出缺失，B符号来自保留的 functions.jsonl.gz。当前工作区包含其他任务的未提交修改，JSON中保存所扫描候选源码指纹。

## 已直接读取函数体的发现

| 模块 | B 函数 / 当前文件 | 判定 | 具体缺口 |
|---|---|---|---|
| M01 | toolbar_size_mult_update；toolbar/src/toolbar_update.rs:73 | 有实现，未独立原侧复验 | 调用 toolbar_size_multiplier，实际读配置和窗口并写倍率；不能据此判整个M01完成 |
| M01 | toolbar_button_size_and_padding_update；toolbar/src/toolbar_update.rs:90 | 有实现，未独立原侧复验 | 遍历普通/摄影context、更新scale、root和menu子节点gap；完整调度与输入域待核实 |
| M06 | construct_floors；wall-constructor/src/construct_floors.rs:56 | 未实现 | 函数体直接todo；删除/改变事件、旧实体移除、楼板创建均未执行 |
| M07 | ui_edit_roof；roof/src/ui_edit_roof.rs:46 | 部分骨架 | 仅检查编辑模式；没有触发体拾取、slider3d、highlight和历史派发；机器会将其误列非占位体候选 |
| M07 | do_ui_edit_roof；roof/src/ui_edit_roof.rs:66 | 未实现 | 空体；编辑事件、屋顶/墙高修改、归档及历史构造均未执行 |
| M08 | assemble_railings；decorator/src/decorator_visual/stairs_railings_assemble.rs:20 | 未实现 | 空体；扶手段几何和蓝图装配未执行 |
| M08 | railings_tips；同文件:30 | 未实现 | 固定Vec::new，无端头几何；机器会将其误列非占位体候选 |
| M08 | construct_wooden_railing；同文件:48 | 未实现 | 空体，无木扶手几何 |
| M08 | add_corner_stone_railing_piece；同文件:62 | 未实现 | 空体，无角石扶手件 |
| M09 | decorators_on_glade_unload；tiny-glade/src/decorator_plugin.rs:49 | 未实现 | 直接todo，完整装饰物/碰撞/视觉实体卸载业务缺失 |
| M09 | decorator_plugin::build；同文件:30 | 部分接线 | 已调用资源、系统和UI注册，但源码说明原schedule/set/run_if及生成族尚未逐项还原 |

以上11项：有实现但未独立验证2、部分骨架/接线2、未实现7。该比例仅属于本次选取的11项，选样偏向缺口，严禁推算模块或全游戏完成率。

## 统计尚未闭合的范围

- 初筛6,716符号组保留了impl$和部分自动生成方法，尚需剔除或单列。
- 多候选和未匹配需要按实际owner、迁移映射、宏、合并实现确认；不能计作未实现。
- M02–M05等路径归类仅涵盖显式模块，历史、GPU、资源和共享输入中相应职责尚未分摊，当前分母不是模块全部业务函数。
- 完成比例需要针对剩余每项核对函数语义、真实producer/consumer与现有证据；本报告尚未做到。
