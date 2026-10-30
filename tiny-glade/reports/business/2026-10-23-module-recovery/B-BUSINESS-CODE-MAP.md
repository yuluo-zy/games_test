# 清理后业务函数、源码关联与未闭合链路

图快照：`5ac3dab66b4ac8c04a89025c`；B 原二进制 SHA：`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
本次扫描 1589 个生产 Rust 文件；生成时间（UTC）：2026-10-30T16:18:56.261867+00:00。

## 统计口径

B 为统计主线，A/C 保持辅助证据，不把三版相同业务相加。机械名称及 new/get/insert/remove 按图谱统一规则排除；闭包实例不当独立算法。用户删除项、Daily暂停项、调试辅助另列排除口径。
函数组按原源码路径＋去泛型后的符号合并；物理入口和仅有内联证据的定义分别列出。一个符号组不代表一个独立算法。源码关联只是位置候选，不能作为恢复完成率。没有同名实现不直接判缺失；原版 ret／原 panic 不当待补业务。
M01为台账的toolbar（当前交流常称M0），M02为地形（常称M1）；其余保留台账编号。M17没有独立函数分母。共享函数不强行按调用者重复摊入业务模块。

## 各业务域

| 模块 | 业务定义组 | 物理入口 | 仅内联定义 | 唯一源码候选 | 多候选 | 未定位 | 空/占位待核 |
|---|---:|---:|---:|---:|---:|---:|---:|
| M01 公共交互 / toolbar | 357 | 281 | 134 | 162 | 23 | 172 | 0 |
| M02 地形编辑 | 13 | 12 | 1 | 13 | 0 | 0 | 0 |
| M03 路径 / 铺石 | 20 | 16 | 4 | 10 | 0 | 10 | 0 |
| M04 花园 | 3 | 3 | 0 | 2 | 0 | 1 | 0 |
| M05 水与林地边界 | 7 | 5 | 2 | 5 | 2 | 0 | 0 |
| M06 墙体 / 规则建筑 | 576 | 397 | 179 | 400 | 3 | 173 | 0 |
| M07 屋顶 | 87 | 69 | 18 | 77 | 2 | 8 | 3 |
| M08 楼梯 / 平台 / 扶手 | 110 | 89 | 23 | 96 | 0 | 14 | 8 |
| M09 装饰物 | 192 | 114 | 72 | 155 | 5 | 32 | 0 |
| M10 颜色 / 自定义 | 35 | 24 | 11 | 24 | 0 | 11 | 0 |
| M11 clutter | 70 | 38 | 30 | 42 | 1 | 27 | 0 |
| M12 ivy | 52 | 31 | 21 | 45 | 0 | 7 | 0 |
| M13 树工具 | 16 | 13 | 3 | 14 | 1 | 1 | 0 |
| M14 森林 | 6 | 4 | 2 | 5 | 1 | 0 | 0 |
| M15 小动物 | 59 | 44 | 14 | 46 | 0 | 13 | 0 |
| M16 菜单 / 设置 / 主题 / 引导 | 143 | 99 | 49 | 101 | 0 | 42 | 0 |
| SHARED 共享基础／渲染／宿主 | 2510 | 2320 | 916 | 1476 | 157 | 877 | 3 |
| M17 整游戏联合验收（无独立函数分母） | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

物理入口可由多个定义／模块共享，各行物理数不能直接相加；97个结构模块的父子包含数同样不可相加。

## 有证据的业务缺口与待核对链路

确认缺代码、已观察验收失败、原合同待核和联合验收未覆盖分别记录。失败记录不自动等于缺算法；下列结论不取代运行原版或正式生产验收。

### BIZ-01 · M01, SHARED · MEI 节点执行顺序

**分类：confirmed_semantic_difference**。当前 mei_pre_update 按 GlobalId 排序；独立原指令审查记录原表按 HashMap control 字节桶顺序执行。算法顺序存在差异，事件/声音影响范围尚未闭合。
链路：MEI 节点表 → pre_update/交互遍历 → vtable 输入/声音/事件
下一步：按当前源码SHA核两轮循环与原桶序；验证不同插入/冲突序的可观察副作用，不按同名函数判等。
- 当前源码：`tiny-glade/crates/systems/toolbar/src/mei/systems.rs:73` · `mei_pre_update` · body_candidate
- 依据：`tiny-glade/reports/business/2026-10-23-module-recovery/STATUS.md:126`（存在，已绑定SHA）

### BIZ-02 · M01, SHARED · MEI 键盘/剪贴板生产入口

**分类：source_wiring_gap**。原侧宿主明确调用 on_key_event/on_clipboard_paste；当前源码实查仅方法定义，宿主事件仍走 core/Bevy。直接生产入口缺口已记录，具体控件影响及间接路径仍待核。
链路：OS/宿主键盘与剪贴板 → MeiContext 队列 → MEI 控件
下一步：核真实 game_loop/输入分派到同一 MEI 队列及payload/tag；不要把所有键盘交互概括为失效。
- 当前源码：`tiny-glade/crates/systems/toolbar/src/mei/core.rs:804` · `on_key_event` · body_candidate
- 当前源码：`tiny-glade/crates/systems/toolbar/src/mei/core.rs:811` · `on_clipboard_paste` · body_candidate
- 依据：`tiny-glade/reports/business/2026-10-23-module-recovery/STATUS.md:127`（存在，已绑定SHA）

### BIZ-03 · M02, SHARED · 地形跨进程最终画面

**分类：observed_acceptance_failure**。已有编辑、双高度场、CPU/碰撞、历史及部分GPU证据；跨进程严格最终图像多轮失败，ShadowMaskMean/AppMode等运行态诊断不能当成已确定唯一根因。
链路：真实地形输入 → 同epoch GPU/CPU高度 → 墙/lighting → 保存/新进程 → 最终帧
下一步：用存在的最新完整日志与源码指纹复核最终首差；当前缺失的 Windows 日志只保留报告级观察，不据此定新算法缺陷。
- 当前源码：`tiny-glade/crates/country-core/src/systems/terrain/ui_edit_terrain.rs:180` · `ui_edit_terrain` · body_candidate
- 依据：`tiny-glade/reports/business/2026-10-23-module-recovery/STATUS.md:110`（存在，已绑定SHA）

### BIZ-04 · M03 · 正式路径工具尚未装配

**分类：source_wiring_gap**。draw_path、record_path_history及局部 PathsPlugin 已存在，但最新冻结记录仍明确正式 App 编辑未接入。当前主宿主静态搜索也无这两个安装点。
链路：工具栏路径模式 → 正式 draw/record → 四栅格 → staging/声音/历史
下一步：先核未决命令字段和九个回调，再按原注册安装正式父链；局部 World 测试不替代实际 App。
- 当前源码：`tiny-glade/crates/country-core/src/systems/paths.rs:279` · `draw_path` · body_candidate
- 当前源码：`tiny-glade/crates/country-core/src/systems/paths.rs:531` · `record_path_history` · body_candidate
- 依据：`tiny-glade/reports/business/2026-10-23-module-recovery/STATUS.md:1107`（存在，已绑定SHA）

### BIZ-05 · M03 · 跨变体命令第二字和九个回调

**分类：original_contract_review**。源码仍给跨变体 draw_raster 传三个空回调；已有证据只闭合部分 staging 及共享事件，不能判断全部原回调为空，也不能直接判为三个缺算法。目标/擦除 strength 字段定义仍 OPEN。
链路：路径/铺石目标与跨变体擦除 → widget/active stroke/history第二字 → 间接消费者
下一步：补目标 rsp+0x8c、擦除 rsp+0x9c 的定义及全部实际消费者；保留已闭合的 DrawState 叶函数结果。
- 当前源码：`tiny-glade/crates/country-core/src/systems/paths.rs:279` · `draw_path` · body_candidate
- 依据：`tiny-glade/reports/business/2026-10-23-module-recovery/STATUS.md:1107`（存在，已绑定SHA）

### BIZ-06 · M03, SHARED · 路径 recorder 失败合同

**分类：original_contract_review**。record_path_history 仍有 let _ = record_raster_history；应核共享 helper 当前返回/原 History::add 的失败语义，不能沿用旧报告把花园、水也一并判为吞错误。
链路：笔画提交 → 录制状态 → History::add → 复合事务提交/取消
下一步：核当前 helper 与路径四槽的失败、事件先后和事务边界；只有确认可达差异后才列实现修复。
- 当前源码：`tiny-glade/crates/country-core/src/systems/paths.rs:531` · `record_path_history` · body_candidate
- 依据：`tiny-glade/reports/business/2026-10-23-module-recovery/STATUS.md:540`（存在，已绑定SHA）

### BIZ-07 · M04 · 花园最终 drawable、声音和 Reload

**分类：joint_validation_open**。已有真实笔画→History→新进程及 GPU mask 通过记录，不能再列 draw/record 未装配。新增 draw-list 断言未实跑；实际非零实例/最终像素、同App重载和音频设备仍未闭合。
链路：Garden UI → 栅格/History → 生成实例/石头小花绘制 → save/restart/卸载
下一步：复用已有通过子链，补真正生产 draw owner、最终像素和多次Reload/音频生命周期；先核旧 Windows receipt 是否可取得。
- 当前源码：`tiny-glade/crates/country-core/src/systems/garden/draw_garden.rs:39` · `draw_garden` · body_candidate
- 依据：`tiny-glade/reports/business/2026-10-23-module-recovery/STATUS.md:1108`（存在，已绑定SHA）

### BIZ-08 · M05 · 画水有效字段和边界输入

**分类：original_contract_review**。strength=1.0 的原有效字段来源仍未闭合；界外候选曾被其它 widget 占有，不能把不满足原门的失败归为画水算法错误。
链路：真实 widget/drag 门 → draw_water命令 → WaterRaster/地形读回通知
下一步：按原版有效字段和真实widget所有权设计输入；不把 Max 省略字段与内存字段直接判为丢存档。
- 当前源码：`tiny-glade/crates/systems/water/src/draw_water.rs:312` · `draw_water` · body_candidate
- 依据：`tiny-glade/reports/business/2026-10-23-module-recovery/STATUS.md:1109`（存在，已绑定SHA）

### BIZ-09 · M05 · 水编辑完整持久化及声音/边界生命周期

**分类：joint_validation_open**。现有正式水插件、即时回放、植被GPU读回不是待从零恢复项。最新 App 日志未到最终 test result，后续 undo/redo/保存/新进程和动态声音/边界尚无完整通过。
链路：画水/擦水 → 地形/碰撞/植被/声音/边界 → History → 文件/新进程 → 清理
下一步：完成有最终结果的真实 App 用例，分别核动态GPU水、完整History和FMOD/谷地边界释放。
- 当前源码：`tiny-glade/crates/systems/water/src/draw_water.rs:312` · `draw_water` · body_candidate
- 依据：`tiny-glade/reports/business/2026-10-23-module-recovery/STATUS.md:1109`（存在，已绑定SHA）

### BIZ-10 · M06, M10, SHARED · 保存加载后完整 typed history 丢首项

**分类：observed_acceptance_failure**。当前复验在正式墙色链 child 缺父 Tree::Place(index100143)；后续阶段诊断显示落盘及加载完成时首项存在，缺口缩到加载后的变更。尚未确定生产原因，不能将墙色/保存全部列成没实现。
链路：真实颜色编辑/完整ledger → save worker/磁盘 → load完成 → 后续runtime mutation
下一步：保留已有父链与两端281项阶段证据，沿加载后的首个writer闭合；不跳树、不只比较末项、不放宽断言。
- 依据：`tiny-glade/reports/business/2026-10-23-module-recovery/STATUS.md:22`（存在，已绑定SHA）

### BIZ-11 · M06, M09 · 归档恢复后新ID碰撞与最终删除

**分类：joint_validation_open**。最新恢复父函数已接真实set42，live shape和mesh/source/位姿保留已回归通过；当前仍追新ID碰撞体最终删除消费者与完整App生命周期，不能倒退成恢复函数未实现。
链路：归档Info → 原模式/事件守卫 → 新ID实体/碰撞 → 删除/Reload释放
下一步：核同owner完整创建/UndoRedo/删除/重入及存活/退役基线，继续复用已有shape反例。
- 当前源码：`tiny-glade/crates/systems/decorator/src/restore_archived_decorators.rs:97` · `restore_archived_decorators` · body_candidate
- 依据：`tiny-glade/reports/business/2026-10-23-module-recovery/STATUS.md:20`（存在，已绑定SHA）

### BIZ-12 · M06, SHARED · 多allocation重发布与异常释放

**分类：original_contract_review**。已有五类sidecar分离和+120/+160/+170 owner原证与部分回归；最后lane/退役顺序及缺库异常完整EH回收不能由旧窄范围owner测试外推。
链路：发布成功/局部构造失败 → 独立砖木lanes → 实体/receipt → Drop/EH释放
下一步：逐项核新旧owner、提交前缀和异常退出；按当前源码而非早期75%估计给出结论。
- 依据：`tiny-glade/reports/business/2026-10-23-module-recovery/STATUS.md:58`（存在，已绑定SHA）

### BIZ-13 · M07 · 屋顶形状/高度交互消费

**分类：confirmed_unrecovered_code**。当前交互执行函数为空，原函数记录为实际业务体；已有setter/历史消费者不能替代缺少的触发体拾取、slider动作和事件/历史生产。
链路：屋顶触发体/真实widget → DoUiEdit消费 → Roof字段/geometry事件 → History/墙联动
下一步：复用已恢复setter，补原交互控制流、旧新值/取消/历史载荷和真实生产接线，再验编辑/回放/保存。
- 当前源码：`tiny-glade/crates/systems/roof/src/ui_edit_roof.rs:66` · `do_ui_edit_roof` · empty_body_review
- 依据：`tiny-glade/crates/systems/roof/src/ui_edit_roof.rs:15`（存在，已绑定SHA）
- 原体图证据：`function:B:1a19a90`（最大已验证区间985条指令）

### BIZ-14 · M07 · 屋脊方向交互消费

**分类：confirmed_unrecovered_code**。当前交互执行函数为空，原函数记录为实际业务体；已有setter/历史消费者不能替代缺少的触发体拾取、slider动作和事件/历史生产。
链路：屋顶触发体/真实widget → DoUiEdit消费 → Roof字段/geometry事件 → History/墙联动
下一步：复用已恢复setter，补原交互控制流、旧新值/取消/历史载荷和真实生产接线，再验编辑/回放/保存。
- 当前源码：`tiny-glade/crates/systems/roof/src/ui_edit_roof_ridge_dir.rs:83` · `do_ui_edit_roof_ridge_dir` · empty_body_review
- 依据：`tiny-glade/crates/systems/roof/src/ui_edit_roof_ridge_dir.rs:3`（存在，已绑定SHA）
- 原体图证据：`function:B:1a39490`（最大已验证区间918条指令）

### BIZ-15 · M07 · 屋脊长度交互消费

**分类：confirmed_unrecovered_code**。当前交互执行函数为空，原函数记录为实际业务体；已有setter/历史消费者不能替代缺少的触发体拾取、slider动作和事件/历史生产。
链路：屋顶触发体/真实widget → DoUiEdit消费 → Roof字段/geometry事件 → History/墙联动
下一步：复用已恢复setter，补原交互控制流、旧新值/取消/历史载荷和真实生产接线，再验编辑/回放/保存。
- 当前源码：`tiny-glade/crates/systems/roof/src/ui_edit_roof_ridge_length.rs:77` · `do_ui_edit_roof_ridge_length` · empty_body_review
- 依据：`tiny-glade/crates/systems/roof/src/ui_edit_roof_ridge_length.rs:17`（存在，已绑定SHA）
- 原体图证据：`function:B:21a8c50`（最大已验证区间1006条指令）

### BIZ-16 · M08 · 玩家楼梯主装配循环

**分类：confirmed_unrecovered_code**。playermade_stairs_assemble 主体为空。原两实例已修CFG取得完整C/ASM，工具超时已不再是缺口；已恢复插值/懒订阅/碰撞叶不能替代梯步、楼层、栏杆和两阶段主分支。
链路：真实stairs请求/缓存 → 主装配 → 梯步/楼层/栏杆/碰撞 → 实体与视觉输出
下一步：按原请求完整尾部/reset返回及分支恢复同一父循环；禁止挂空生产阶段。
- 当前源码：`tiny-glade/crates/systems/decorator/src/decorator_visual/stairs_assemble.rs:278` · `playermade_stairs_assemble` · empty_body_review
- 依据：`tiny-glade/reports/module-recovery-2026-10-23/m01-stairs-assembly-recovery-2026-10-27.md`（存在，已绑定SHA）
- 原体图证据：`function:B:8c90`（最大已验证区间11379条指令）, `function:B:18040`（最大已验证区间11458条指令）

### BIZ-17 · M08 · 平台楼梯几何主入口

**分类：confirmed_unrecovered_code**。construct_platform_stairs 当前直接 todo!，尚无真实80B装饰源→blockers→段/格/碰撞→砖/holes/stats 主体。
链路：平台装饰/墙/地形 → blockers与platform分段 → 原96B砖和碰撞/holes/stats
下一步：恢复原父体和同一storage的生产消费；不能用已有draw pass替代几何入口。
- 当前源码：`tiny-glade/crates/systems/wall-constructor/src/construct_platform_stairs/mod.rs:34` · `construct_platform_stairs` · explicit_stub
- 依据：`tiny-glade/crates/systems/wall-constructor/src/construct_platform_stairs/mod.rs:3`（存在，已绑定SHA）
- 原体图证据：`function:B:7744d0`（最大已验证区间4399条指令）

### BIZ-18 · M08 · 玩家楼梯预处理及支撑/柱装配

**分类：confirmed_unrecovered_code**。preprocess_curve、拱/墙主装配、柱proposal处理仍直接todo，brackets/magic_support/pillars_visual有明确未完成分支；它们是一条依赖链，不按单态化数重复算算法。
链路：楼梯曲线 → 两路模式/重采样 → 墙拱/柱提案 → 托架/浮砖/柱视觉
下一步：先恢复完整数据结构和原分支，再串支持请求/提案、生产几何、碰撞、历史和退役。
- 当前源码：`tiny-glade/crates/systems/wall-constructor/src/construct_playermade_stairs/mod.rs:90` · `preprocess_curve` · explicit_stub
- 当前源码：`tiny-glade/crates/systems/wall-constructor/src/construct_playermade_stairs/mod.rs:100` · `construct_playermade_stairs_arches_n_walls` · explicit_stub
- 当前源码：`tiny-glade/crates/systems/wall-constructor/src/construct_playermade_stairs/pillars_process_proposals.rs:128` · `process_playermade_stair_pillars_proposals` · explicit_stub
- 当前源码：`tiny-glade/crates/systems/wall-constructor/src/construct_playermade_stairs/brackets.rs:17` · `construct_playermade_stair_brackets` · partial_stub
- 当前源码：`tiny-glade/crates/systems/wall-constructor/src/construct_playermade_stairs/magic_support.rs:16` · `playermade_stair_floating_bricks` · partial_stub
- 当前源码：`tiny-glade/crates/systems/wall-constructor/src/construct_playermade_stairs/pillars_visual.rs:19` · `construct_playermade_stair_pillars_visuals` · partial_stub
- 依据：`tiny-glade/reports/module-recovery-2026-10-23/m01-stairs-assembly-recovery-2026-10-27.md`（存在，已绑定SHA）
- 原体图证据：`function:B:7834f0`（最大已验证区间2202条指令）, `function:B:77a2f0`（最大已验证区间2653条指令）, `function:B:496800`（最大已验证区间4635条指令）, `function:B:780e20`（最大已验证区间1721条指令）, `function:B:77d520`（最大已验证区间2853条指令）

### BIZ-19 · M09 · 装饰set25真实配置树与全生命周期

**分类：registration_and_lifecycle_review**。已有放置、重定位、完整Info归档和真实Rapier子链；独立审查曾否定把叶函数平铺等价原tuple/chain/flush。最新整树和全卸载不能仅由已过叶级测试证明。
链路：真实请求 → 原注册父树/flush → 位置/mesh/碰撞 → 完整归档/UndoRedo/卸载
下一步：按原配置数组逐槽审查当前DecoratorPlugin，再验非空对象、重复注册位置、完整ID/shape和清理。
- 依据：`tiny-glade/reports/business/2026-10-23-module-recovery/STATUS.md:116`（存在，已绑定SHA）

### BIZ-20 · M10, M06, M09 · 多对象颜色菜单到真实GPU/历史

**分类：joint_validation_open**。颜色setter、13类菜单及部分真实GPU/history已恢复；墙色完整新进程失败属于BIZ-10，装饰/rim/灯/楼梯等仍需逐对象非空交互、回滚与保存证据，不能统称所有菜单未实现。
链路：真实菜单picker → canonical颜色权威 → 同物理owner上传 → 完整History/取消/重载
下一步：逐对象复用已过子链，检查全表GPU、完整ledger、实体/菜单退役；设备丢失另查而非判算法失败。
- 依据：`tiny-glade/reports/business/2026-10-23-module-recovery/STATUS.md:106`（存在，已绑定SHA）

### BIZ-21 · M11 · clutter生成权威与哈希/几何合同

**分类：original_contract_review**。已有真实窗口/屋顶交线→鸟巢/杂物→退役的World通过，不能列为全无producer。原哈希、stitches/merlons细节及正式父链的完整场景合同仍需按当前路径审查，旧近似注释不自动变成新缺陷。
链路：墙/屋顶/树/水权威 → dirty/候选/原hash几何 → Nani/碰撞 → 重建和退役
下一步：区分已弃用近似helper和生产路径，补原具体候选与非空实际App证据。
- 依据：`tiny-glade/reports/business/2026-10-23-module-recovery/STATUS.md:238`（存在，已绑定SHA）

### BIZ-22 · M12 · ivy完整注册、生长修剪与建筑联动

**分类：registration_and_lifecycle_review**。函数有体和原9系统tuple的静态证据不能证明持续生长/避洞/修剪/叶片上传/声音整链；未取得全域当前验收，保留审查边界。
链路：墙/洞/装饰变化 → 原spawner/grower/pruner调度 → 叶片/动画/声音 → 删除重入
下一步：按原tuple和真实FunctionSystem name核当前父配置；用同株生命周期联动验证，不能按无TODO判恢复。
- 依据：`tiny-glade/reports/business/2026-10-23-module-recovery/STATUS.md:686`（存在，已绑定SHA）

### BIZ-23 · M13 · 树工具同一权威的编辑与持久化

**分类：joint_validation_open**。已恢复方法与非空函数体不是完整树工具验收；本轮没有新的原侧差分证据证明放置/弯曲/移动/隐藏/删除和历史存档联合闭合。
链路：树工具输入 → TreeStorage → 几何/碰撞/渲染 → 历史/隐藏恢复/卸载
下一步：按当前原证和真实同一TreeStorage审查，不把同名多个owner当已定位，也不把待验项全部判为缺算法。
- 依据：`tiny-glade/reports/business/2026-10-23-module-recovery/STATUS.md:687`（存在，已绑定SHA）

### BIZ-24 · M14, M13 · 森林预放树与树工具重载联动

**分类：joint_validation_open**。森林转换、prefab与动画方法位置需与同一树权威核对，现无完整地形变化/批量重载/认领与资源释放验收。
链路：预放树/prefab → 批量树权威/动画 → 地形/隐藏 → Reload订阅及GPU释放
下一步：确认当前转换入口与真实资源所有权，检查不重复实例、不残留旧订阅。
- 依据：`tiny-glade/reports/business/2026-10-23-module-recovery/STATUS.md:688`（存在，已绑定SHA）

### BIZ-25 · M15 · 鱼实体写面和状态2目标消费

**分类：original_contract_review**。当前fish维护Boid位置，但生产Fish实体只写state；独立repel_from_fish_targets的原实体查询消费尚未接入。应核完整0x30记录与真正渲染/状态producer，不能把已补四路径采样再列为缺失。
链路：鱼生成 → Boid/Fish实体完整写面 → 状态2近距目标/速度 → 动画/渲染/删除
下一步：绑定原鱼实体记录与当前消费者，恢复确实缺少的生产字段/查询，再验真实鱼群；出生速度档另待原判别。
- 当前源码：`tiny-glade/crates/systems/critters/src/fish.rs:34` · `fish` · body_candidate
- 当前源码：`tiny-glade/crates/systems/critters/src/fish.rs:154` · `repel_from_fish_targets` · body_candidate
- 依据：`tiny-glade/crates/systems/critters/src/fish.rs:152`（存在，已绑定SHA）

### BIZ-26 · M15 · 羊/鸭抚摸交互反馈未齐

**分类：partial_implementation_review**。当前timer、模式门、widget和部分frozen/音频已有；signifier、WindowFocus/decorator反馈及部分音频仍有明确未恢复边界。两处非空函数不能算宠物交互全部闭合。
链路：真实动物命中/拖拽 → 宠物状态 → signifier/窗口/声音/装饰反馈 → 释放
下一步：逐物种核原控制流中遗漏写面与各事件消费者，使用真实UI/音频 owner 验证。
- 当前源码：`tiny-glade/crates/systems/critters/src/ui_pet_ducks.rs:38` · `ui_pet_ducks` · body_candidate
- 当前源码：`tiny-glade/crates/systems/critters/src/sheep/ui_pet_sheep.rs:38` · `ui_pet_sheep` · body_candidate
- 依据：`tiny-glade/crates/systems/critters/src/ui_pet_ducks.rs:21`（存在，已绑定SHA）
- 依据：`tiny-glade/crates/systems/critters/src/sheep/ui_pet_sheep.rs:57`（存在，已绑定SHA）

### BIZ-27 · M15 · 蝴蝶组件写面与群体关联

**分类：original_contract_review**。蝴蝶两个系统仍记录stride-4组件身份/写面未建模；wren群体关联亦有原结构未知。没有足够证据把未知组件翻译成猜测默认值，也不重提已纠正的旧boids近似。
链路：目标/状态 → 原组件写面/群体关联 → 速度/动画/实体消费
下一步：从typed query描述符与原writer读者确认身份，再审所有物种的完整调度、认领释放及混合场景。
- 当前源码：`tiny-glade/crates/systems/critters/src/butterfly.rs:358` · `update_butterfly_goals` · body_candidate
- 当前源码：`tiny-glade/crates/systems/critters/src/butterfly.rs:466` · `set_butterfly_velocities_towards_goals` · body_candidate
- 依据：`tiny-glade/crates/systems/critters/src/butterfly.rs:356`（存在，已绑定SHA）

### BIZ-28 · M16 · 非Daily菜单和调试辅助边界

**分类：scope_and_source_review**。普通新建/设置/窗口已有有界通过，不从早期未进入表回退；Daily暂停、Credits/许可证/报告删除、真实Steam SDK暂排除。调试GUI空壳独立列辅助项，不冒充游戏核心算法缺口。
链路：实际页面命令 → 已存在的游戏消费者 → 配置/主题/提示/错误呈现
下一步：核普通页面所有实际消费者与原调用；Daily和用户删除项不列当前阻断，调试工具另报范围。
- 当前源码：`tiny-glade/crates/tiny-glade/src/gui/gui_debug.rs:51` · `do_gui_debug` · explicit_stub
- 依据：`tiny-glade/reports/business/2026-10-23-module-recovery/STATUS.md:132`（存在，已绑定SHA）

### BIZ-29 · SHARED, M03, M04, M05 · 公共栅格依赖身份与原输出

**分类：dependency_contract_review**。zeno crates.io与原PounceLight fork同版本号不证明行为同源。已有私有基础库/渲染架构继续复用，不能因模块0个业务符号就判断缺整个库或强行回退架构。
链路：原栅格/重采样 → 依赖调用及组合参数 → 四/六栅格权威 → 业务消费者
下一步：先核当前锁和实际使用路径，仅对影响业务的fork差异做有界原输出对照，基础接口未明保留待核。
- 依据：`tiny-glade/reports/business/2026-10-23-module-recovery/STATUS.md:539`（存在，已绑定SHA）

### BIZ-30 · M17 · 全软件跨模块联合验收

**分类：joint_validation_open**。各叶函数、模块源码关联和局部场景通过均不足以证明全游戏完成；本轮只重统计和识别证据边界。
链路：混合编辑 → 墙/屋顶/楼梯/装饰/地形/动物 → 完整History/磁盘 → 新进程/卸载重入
下一步：按当前有效范围逐真实链验收；性能/长稳、真实OS输入按既有暂缓决定单列，不额外塞进本次函数统计分母。
- 依据：`tiny-glade/reports/business/2026-10-23-module-recovery/STATUS.md:535`（存在，已绑定SHA）

## 全部模块的审查边界

| 业务域 | 真实链与本轮未闭合边界 |
|---|---|
| M01 | toolbar/MEI：原桶序和键盘/剪贴板入口有具体差异；普通新建/窗口/设置有局部通过；547项原证不足记录不能当全原版等价。 |
| M02 | 地形：13项非通名定义含5项已跨模块/同文件定位；最终跨进程画面和运行态依赖仍待核，不把原 MouseActivityTimer::new 算入机械清理后的业务分母。 |
| M03 | 路径：正式输入父链缺接线，九回调/命令字段未闭合；DrawState全256³局部差分与7项局部测试已有，不从零重复。 |
| M04 | 花园：真实输入/History/新进程/GPU mask已有旧receipt；最终drawable、像素、音频及Reload/资源基线仍待当前验收。 |
| M05 | 水/谷地边界：插件和动态GPU子链已有；实际App末端无完整通过、有效字段/界外输入/声音/边界生命周期待核。 |
| M06 | 墙建筑：原两个ret和原失败分支已排除假缺口；完整保存后Tree::Place首项变更、ID碰撞删除及异常资源释放仍待闭合。 |
| M07 | 屋顶：形状/方向/长度交互执行端有真实空体；已有setter、几何/碰撞子链不能替代生产入口。 |
| M08 | 楼梯/平台/扶手：主装配、平台、曲线预处理/支撑柱仍明确未恢复；插值/结构图/碰撞叶已有，不用draw存在冒充上游。 |
| M09 | 装饰：真实注册父树/flush、完整Info/碰撞双表及最终删除重入继续审；不把已补恢复和重定位重列为空实现。 |
| M10 | 颜色：部分真实菜单/同GPU owner及History已测，逐对象完整交互/取消/保存仍待证，不能把全部色菜单判为未实现。 |
| M11 | clutter：真实窗口/屋顶交线/鸟巢子链已有；哈希、其它几何族与正式父链完整场景合同需进一步原证。 |
| M12 | ivy：原9系统tuple与已有grow/prune源码需连到真实建筑变化、叶片/音频和删除生命周期；函数有体不算完成。 |
| M13 | 树工具：同一TreeStorage的放置/移动/弯曲/隐藏、历史、渲染及保存卸载联合证据待逐链核对。 |
| M14 | 森林：预放树转换、prefab、动画、地形变化和重载认领/释放需与树工具共同核对，不复用旧覆盖率作验收。 |
| M15 | 小动物：鱼实体写面/状态2消费、羊鸭反馈、蝴蝶未知组件等需原证；逐物种记录，不能把历史已修近似一并重列。 |
| M16 | 菜单/主题/引导：普通菜单/设置现有证据逐消费者复核，Daily暂缓和用户删除项独立排除；调试辅助不充业务算法分母。 |
| SHARED | 全部共享基础/渲染/宿主：按定义归属和原fork合同审；97结构模块中无业务定义者保留归属未决或依赖复用，不能算已恢复。 |
| M17 | 联合验收：无独立函数分母；没有一条统计比例可替代全游戏真实链和跨进程/卸载验收。 |

## 源码关联待核对清单

全部逐项记录见同名 CSV/JSON，含原路径、图符号/物理函数/内联证据、当前类型owner、源码SHA和关联理由。下列为每域前8项入口；完整结果不受此展示上限影响。

### M01 · 195 项定位／函数体待核

- `country_core::systems::backend::input::input_record_replay::normalize_cursor_pos` · `crates/country-core/src/systems/backend/input/input_record_replay.rs` · unresolved_source_association · 无可靠位置候选
- `country_core::systems::backend::input::input_record_replay::unnormalize_cursor_pos` · `crates/country-core/src/systems/backend/input/input_record_replay.rs` · unresolved_source_association · 无可靠位置候选
- `country_core::systems::mode_manager::check_if_can_switch` · `crates/country-core/src/systems/mode_manager.rs` · unresolved_source_association · 无可靠位置候选
- `country_core::systems::mode_manager::switch_app_mode` · `crates/country-core/src/systems/mode_manager.rs` · unresolved_source_association · 无可靠位置候选
- `toolbar::impl$0::build` · `crates/systems/toolbar/src/lib.rs` · unresolved_source_association · 无可靠位置候选
- `toolbar::impl$1::build` · `crates/systems/toolbar/src/lib.rs` · unresolved_source_association · 无可靠位置候选
- `toolbar::menu::report_bug::impl$6::process_input_action` · `crates/systems/toolbar/src/lib.rs` · unresolved_source_association · 无可靠位置候选
- `toolbar::menu::welcome::ui` · `crates/systems/toolbar/src/lib.rs` · unresolved_source_association · 无可靠位置候选

### M03 · 10 项定位／函数体待核

- `country_core::systems::paths::draw_path::DrawState::finish` · `crates/country-core/src/systems/paths/draw_path.rs` · unresolved_source_association · 无可靠位置候选
- `country_core::systems::paths::draw_path::DrawState::start_or_continue` · `crates/country-core/src/systems/paths/draw_path.rs` · unresolved_source_association · 无可靠位置候选
- `country_core::systems::paths::draw_path::draw_path` · `crates/country-core/src/systems/paths/draw_path.rs` · unresolved_source_association · 无可靠位置候选
- `country_core::systems::paths::draw_path::erase_other_path_variants` · `crates/country-core/src/systems/paths/draw_path.rs` · unresolved_source_association · 无可靠位置候选
- `country_core::systems::paths::draw_path::impl$2::bilinear_sample_at_world_xz` · `crates/country-core/src/systems/paths/draw_path.rs` · unresolved_source_association · 无可靠位置候选
- `country_core::systems::paths::impl$0::build` · `crates/country-core/src/systems/paths/mod.rs` · unresolved_source_association · 无可靠位置候选
- `country_core::systems::paths::path_brush_preview_size::path_brush_preview_size` · `crates/country-core/src/systems/paths/path_brush_preview_size.rs` · unresolved_source_association · 无可靠位置候选
- `country_core::systems::paths::record_path_history::record_path_history` · `crates/country-core/src/systems/paths/record_path_history.rs` · unresolved_source_association · 无可靠位置候选

### M04 · 1 项定位／函数体待核

- `country_core::systems::garden::impl$0::build` · `crates/country-core/src/systems/garden.rs` · unresolved_source_association · 无可靠位置候选

### M05 · 2 项定位／函数体待核

- `system_glade_border::impl$0::build` · `crates/systems/glade-border/src/lib.rs` · ambiguous_source_candidates · tiny-glade/crates/systems/glade-border/src/lib.rs:44; tiny-glade/crates/systems/glade-border/src/lib.rs:58
- `system_water::impl$0::build` · `crates/systems/water/src/lib.rs` · ambiguous_source_candidates · tiny-glade/crates/systems/water/src/lib.rs:173; tiny-glade/crates/systems/water/src/lib.rs:199

### M06 · 176 项定位／函数体待核

- `country_core::resources::walls::curved_halftimber::CurvedHalfTimberDrawables::remove_wall_id` · `crates/country-core/src/resources/walls/curved_halftimber.rs` · unresolved_source_association · 无可靠位置候选
- `country_core::resources::walls::decorator::is_default` · `crates/country-core/src/resources/walls/decorator.rs` · ambiguous_source_candidates · tiny-glade/crates/country-core/src/resources/walls/decorator.rs:805; tiny-glade/crates/country-core/src/resources/walls/decorator.rs:1151
- `country_core::resources::walls::decorator_archivist::DecoratorArchivist::archived_walls` · `crates/country-core/src/resources/walls/decorator_archivist.rs` · unresolved_source_association · 无可靠位置候选
- `country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_colliders` · `crates/country-core/src/resources/walls/decorator_archivist.rs` · unresolved_source_association · 无可靠位置候选
- `country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_clear` · `crates/country-core/src/resources/walls/decorator_archivist.rs` · unresolved_source_association · 无可靠位置候选
- `country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_stairs` · `crates/country-core/src/resources/walls/decorator_archivist.rs` · unresolved_source_association · 无可靠位置候选
- `country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_terrain` · `crates/country-core/src/resources/walls/decorator_archivist.rs` · unresolved_source_association · 无可靠位置候选
- `country_core::resources::walls::decorator_archivist::DecoratorArchivist::decorator_interactions` · `crates/country-core/src/resources/walls/decorator_archivist.rs` · unresolved_source_association · 无可靠位置候选

### M07 · 13 项定位／函数体待核

- `country_core::resources::roofs::roof_noise::gpunoise::lerp` · `crates/country-core/src/resources/roofs/roof_noise.rs` · unresolved_source_association · 无可靠位置候选
- `country_core::resources::roofs::roof_shape::Roof::ty` · `crates/country-core/src/resources/roofs/roof_shape.rs` · unresolved_source_association · 无可靠位置候选
- `country_core::resources::roofs::roof_shape::RoofShapeParams::from_wall_shape` · `crates/country-core/src/resources/roofs/roof_shape.rs` · unresolved_source_association · 无可靠位置候选
- `system_roof::impl$0::add_roof_events_and_resources` · `crates/systems/roof/src/lib.rs` · ambiguous_source_candidates · tiny-glade/crates/systems/roof/src/lib.rs:37; tiny-glade/crates/systems/roof/src/lib.rs:144
- `system_roof::impl$0::add_roof_systems` · `crates/systems/roof/src/lib.rs` · ambiguous_source_candidates · tiny-glade/crates/systems/roof/src/lib.rs:49; tiny-glade/crates/systems/roof/src/lib.rs:189
- `system_roof::simulate_roof_tile_for_audio::simulate_roof_tile` · `crates/systems/roof/src/simulate_roof_tile_for_audio.rs` · unresolved_source_association · 无可靠位置候选
- `system_roof::ui_edit_roof::do_ui_edit_roof` · `crates/systems/roof/src/ui_edit_roof.rs` · unique_source_candidate · tiny-glade/crates/systems/roof/src/ui_edit_roof.rs:66
- `system_roof::ui_edit_roof::update_wall_height_if_needed` · `crates/systems/roof/src/ui_edit_roof.rs` · unresolved_source_association · 无可靠位置候选

### M08 · 22 项定位／函数体待核

- `country_core::resources::stairs::stairs_state::StairsState::clear_backup` · `crates/country-core/src/resources/stairs/stairs_state.rs` · unresolved_source_association · 无可靠位置候选
- `country_core::resources::stairs::stairs_state::StairsState::contains_edge_or_stub_impl` · `crates/country-core/src/resources/stairs/stairs_state.rs` · unresolved_source_association · 无可靠位置候选
- `country_core::resources::stairs::stairs_state::StairsState::force_make_backup_no_cleanup` · `crates/country-core/src/resources/stairs/stairs_state.rs` · unresolved_source_association · 无可靠位置候选
- `country_core::resources::stairs::stairs_state::StairsState::take_snapshot_impl` · `crates/country-core/src/resources/stairs/stairs_state.rs` · unresolved_source_association · 无可靠位置候选
- `system_decorator::decorator_visual::stairs::ShapeWalkOutput::invalid` · `crates/systems/decorator/src/decorator_visual/stairs.rs` · unresolved_source_association · 无可靠位置候选
- `system_decorator::decorator_visual::stairs::is_ccw_quad_convex` · `crates/systems/decorator/src/decorator_visual/stairs.rs` · unresolved_source_association · 无可靠位置候选
- `system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble` · `crates/systems/decorator/src/decorator_visual/stairs_assemble.rs` · unique_source_candidate · tiny-glade/crates/systems/decorator/src/decorator_visual/stairs_assemble.rs:278
- `system_decorator::decorator_visual::stairs_assemble::resample_curve` · `crates/systems/decorator/src/decorator_visual/stairs_assemble.rs` · unresolved_source_association · 无可靠位置候选

### M09 · 37 项定位／函数体待核

- `system_decorator::audio::op_preview_audio::move_sound` · `crates/systems/decorator/src/audio/op_preview_audio.rs` · unresolved_source_association · 无可靠位置候选
- `system_decorator::blueprint::DecoratorBlueprint::clear` · `crates/systems/decorator/src/blueprint.rs` · unresolved_source_association · 无可靠位置候选
- `system_decorator::blueprint::DecoratorBlueprint::outline` · `crates/systems/decorator/src/blueprint.rs` · unresolved_source_association · 无可靠位置候选
- `system_decorator::blueprint::DecoratorBlueprintCollider::as_transformed_collider` · `crates/systems/decorator/src/blueprint.rs` · unresolved_source_association · 无可靠位置候选
- `system_decorator::blueprint::DecoratorBlueprints::iter` · `crates/systems/decorator/src/blueprint.rs` · unresolved_source_association · 无可靠位置候选
- `system_decorator::blueprint::DecoratorBlueprints::iter_maybe_modified` · `crates/systems/decorator/src/blueprint.rs` · unresolved_source_association · 无可靠位置候选
- `system_decorator::blueprint::clear_blueprints` · `crates/systems/decorator/src/blueprint.rs` · ambiguous_source_candidates · tiny-glade/crates/systems/decorator/src/blueprint.rs:317; tiny-glade/crates/systems/decorator/src/blueprint.rs:353
- `system_decorator::blueprint::copy_blueprints_to_prev` · `crates/systems/decorator/src/blueprint.rs` · ambiguous_source_candidates · tiny-glade/crates/systems/decorator/src/blueprint.rs:335; tiny-glade/crates/systems/decorator/src/blueprint.rs:376

### M10 · 11 项定位／函数体待核

- `system_color::radial_menu::IconInfo::chimney_smoke_toggle` · `crates/systems/color/src/radial_menu/mod.rs` · unresolved_source_association · 无可靠位置候选
- `system_color::radial_menu::IconInfo::magic_color` · `crates/systems/color/src/radial_menu/mod.rs` · unresolved_source_association · 无可靠位置候选
- `system_color::radial_menu::IconInfo::new_brightness` · `crates/systems/color/src/radial_menu/mod.rs` · unresolved_source_association · 无可靠位置候选
- `system_color::radial_menu::IconInfo::new_color` · `crates/systems/color/src/radial_menu/mod.rs` · unresolved_source_association · 无可靠位置候选
- `system_color::radial_menu::IconInfo::new_color_array` · `crates/systems/color/src/radial_menu/mod.rs` · unresolved_source_association · 无可靠位置候选
- `system_color::radial_menu::IconInfo::new_flag_connection` · `crates/systems/color/src/radial_menu/mod.rs` · unresolved_source_association · 无可靠位置候选
- `system_color::radial_menu::IconInfo::new_flatroof_style` · `crates/systems/color/src/radial_menu/mod.rs` · unresolved_source_association · 无可靠位置候选
- `system_color::radial_menu::IconInfo::new_freehand_ty` · `crates/systems/color/src/radial_menu/mod.rs` · unresolved_source_association · 无可靠位置候选

### M11 · 28 项定位／函数体待核

- `country_core::resources::clutter_meshes::_::impl$0::first` · `crates/country-core/src/resources/clutter_meshes.rs` · unresolved_source_association · 无可靠位置候选
- `country_core::systems::ui_clutter::defered_autoclutter_history::history_record_auto_clutter` · `crates/country-core/src/systems/ui_clutter/defered_autoclutter_history.rs` · unresolved_source_association · 无可靠位置候选
- `country_core::systems::ui_clutter::ui_autoclutter::maintain_autoclutter_parent_hashmap` · `crates/country-core/src/systems/ui_clutter/ui_autoclutter.rs` · unresolved_source_association · 无可靠位置候选
- `country_core::systems::ui_clutter::ui_focus_decoclutter::ui_focus_decoclutter` · `crates/country-core/src/systems/ui_clutter/ui_focus_decoclutter.rs` · unresolved_source_association · 无可靠位置候选
- `system_clutter::autoclutter::manage_autoclutter_entities::update_shutter` · `crates/systems/clutter/src/autoclutter/manage_autoclutter_entities.rs` · unresolved_source_association · 无可靠位置候选
- `system_clutter::autoclutter::AutoClutterRegion::basis_xz` · `crates/systems/clutter/src/autoclutter/mod.rs` · unresolved_source_association · 无可靠位置候选
- `system_clutter::autoclutter::AutoClutterRegion::center` · `crates/systems/clutter/src/autoclutter/mod.rs` · unresolved_source_association · 无可靠位置候选
- `system_clutter::autoclutter::AutoClutterRegion::centered` · `crates/systems/clutter/src/autoclutter/mod.rs` · unresolved_source_association · 无可靠位置候选

### M12 · 7 项定位／函数体待核

- `country_core::resources::walls::public_wall_state::PublicWallState::max_ivy_grow_height` · `crates/country-core/src/resources/ivy/wall_coord.rs` · unresolved_source_association · 无可靠位置候选
- `country_core::systems::ivy::ivy_grower::closest_segment_index` · `crates/country-core/src/systems/ivy/ivy_grower.rs` · unresolved_source_association · 无可靠位置候选
- `country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf` · `crates/country-core/src/systems/ivy/ivy_send_to_gpu_leaf.rs` · unresolved_source_association · 无可靠位置候选
- `country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments` · `crates/country-core/src/systems/ivy/ivy_send_to_gpu_segments.rs` · unresolved_source_association · 无可靠位置候选
- `country_core::systems::ivy::impl$0::add_ivy_events` · `crates/country-core/src/systems/ivy/mod.rs` · unresolved_source_association · 无可靠位置候选
- `country_core::systems::ivy::impl$0::add_ivy_systems` · `crates/country-core/src/systems/ivy/mod.rs` · unresolved_source_association · 无可靠位置候选
- `country_core::systems::ivy::impl$0::insert_ivy_resources` · `crates/country-core/src/systems/ivy/mod.rs` · unresolved_source_association · 无可靠位置候选

### M13 · 2 项定位／函数体待核

- `system_tree::impl$0::build` · `crates/systems/tree-tool/src/lib.rs` · ambiguous_source_candidates · tiny-glade/crates/systems/tree-tool/src/lib.rs:81; tiny-glade/crates/systems/tree-tool/src/lib.rs:107
- `system_tree::ui_move_tree::remap_pos_y_if_in_water` · `crates/systems/tree-tool/src/ui_move_tree.rs` · unresolved_source_association · 无可靠位置候选

### M14 · 1 项定位／函数体待核

- `forest::impl$0::build` · `crates/systems/forest/src/lib.rs` · ambiguous_source_candidates · tiny-glade/crates/systems/forest/src/lib.rs:64; tiny-glade/crates/systems/forest/src/lib.rs:83

### M15 · 13 项定位／函数体待核

- `system_critters::boids::Boid::apply_rules` · `crates/systems/critters/src/boids.rs` · unresolved_source_association · 无可靠位置候选
- `system_critters::boids::Boid::unobstructed_dir` · `crates/systems/critters/src/boids.rs` · unresolved_source_association · 无可靠位置候选
- `system_critters::boids::BoidFlock::clone_state` · `crates/systems/critters/src/boids.rs` · unresolved_source_association · 无可靠位置候选
- `system_critters::boids::BoidFlock::drain` · `crates/systems/critters/src/boids.rs` · unresolved_source_association · 无可靠位置候选
- `system_critters::boids::BoidFlock::iter_mut` · `crates/systems/critters/src/boids.rs` · unresolved_source_association · 无可靠位置候选
- `system_critters::boids::BoidFlock::len` · `crates/systems/critters/src/boids.rs` · unresolved_source_association · 无可靠位置候选
- `system_critters::boids::BoidFlock::push` · `crates/systems/critters/src/boids.rs` · unresolved_source_association · 无可靠位置候选
- `system_critters::duck::PlanPurpose::is_for_entity` · `crates/systems/critters/src/duck.rs` · unresolved_source_association · 无可靠位置候选

### M16 · 42 项定位／函数体待核

- `system_onboarding::hint_camera_progress::CameraHintProgress::from_saved_profile` · `crates/systems/onboarding/src/hint_camera_progress.rs` · unresolved_source_association · 无可靠位置候选
- `system_onboarding::hint_camera_progress::CameraHintProgress::just_completed` · `crates/systems/onboarding/src/hint_camera_progress.rs` · unresolved_source_association · 无可靠位置候选
- `system_onboarding::hint_camera_progress::CameraHintProgress::progress` · `crates/systems/onboarding/src/hint_camera_progress.rs` · unresolved_source_association · 无可靠位置候选
- `system_onboarding::hint_display_system::render_hints::fmtkey` · `crates/systems/onboarding/src/hint_display_system.rs` · unresolved_source_association · 无可靠位置候选
- `system_onboarding::hint_orchestrator::BindingHintDisplay::cleanup_impl` · `crates/systems/onboarding/src/hint_display_system.rs` · unresolved_source_association · 无可靠位置候选
- `system_onboarding::hint_orchestrator::BindingHintDisplay::resolve_steam_impl` · `crates/systems/onboarding/src/hint_display_system.rs` · unresolved_source_association · 无可靠位置候选
- `toolbar::menu::load::ThumbnailScrollView::abandon_focus` · `crates/systems/toolbar/src/menu/load.rs` · unresolved_source_association · 无可靠位置候选
- `toolbar::menu::load::ThumbnailScrollView::clicked_item` · `crates/systems/toolbar/src/menu/load.rs` · unresolved_source_association · 无可靠位置候选

### SHARED · 1037 项定位／函数体待核

- `asset_pipe::texture::BcMode::block_bytes` · `crates/asset-pipe/src/texture.rs` · unresolved_source_association · 无可靠位置候选
- `asset_pipe::texture::apply_transfer_function` · `crates/asset-pipe/src/texture.rs` · unresolved_source_association · 无可靠位置候选
- `asset_pipe::texture::compile_or_load_texture_assets` · `crates/asset-pipe/src/texture.rs` · unresolved_source_association · 无可靠位置候选
- `asset_pipe::texture::compile_texture_asset` · `crates/asset-pipe/src/texture.rs` · unresolved_source_association · 无可靠位置候选
- `asset_pipe::texture::compile_texture_assets` · `crates/asset-pipe/src/texture.rs` · unresolved_source_association · 无可靠位置候选
- `asset_pipe::texture::compress` · `crates/asset-pipe/src/texture.rs` · unresolved_source_association · 无可靠位置候选
- `asset_pipe::texture::pad_image` · `crates/asset-pipe/src/texture.rs` · unresolved_source_association · 无可靠位置候选
- `asset_pipe::texture::process_dds_image` · `crates/asset-pipe/src/texture.rs` · unresolved_source_association · 无可靠位置候选

## B版97个结构模块

| 模块 | 包含定义组 | 独占归属组 | 物理入口 | 唯一候选 | 多候选 | 未定位 |
|---|---:|---:|---:|---:|---:|---:|
| `_references/sources/3fc5d42ccde59c78` | 0 | 0 | 0 | 0 | 0 | 0 |
| `_references/sources/637adda31829802b` | 0 | 0 | 0 | 0 | 0 | 0 |
| `_references/sources/8a79a84fa41db363` | 0 | 0 | 0 | 0 | 0 | 0 |
| `crates/asset` | 37 | 37 | 20 | 16 | 14 | 7 |
| `crates/asset-pipe` | 13 | 13 | 6 | 0 | 0 | 13 |
| `crates/conservative-line-raster` | 5 | 5 | 2 | 0 | 0 | 5 |
| `crates/country-core` | 2419 | 72 | 2224 | 1564 | 111 | 744 |
| `crates/country-core/src/asset_libraries` | 77 | 77 | 288 | 51 | 12 | 14 |
| `crates/country-core/src/components` | 9 | 9 | 5 | 3 | 1 | 5 |
| `crates/country-core/src/geometry` | 6 | 6 | 5 | 5 | 0 | 1 |
| `crates/country-core/src/input` | 28 | 28 | 23 | 18 | 0 | 10 |
| `crates/country-core/src/render` | 345 | 345 | 583 | 177 | 71 | 97 |
| `crates/country-core/src/resources` | 804 | 272 | 519 | 559 | 11 | 234 |
| `crates/country-core/src/resources/critters` | 7 | 7 | 6 | 5 | 0 | 2 |
| `crates/country-core/src/resources/history` | 86 | 86 | 39 | 16 | 0 | 70 |
| `crates/country-core/src/resources/ivy` | 22 | 22 | 7 | 21 | 0 | 1 |
| `crates/country-core/src/resources/render` | 10 | 10 | 12 | 9 | 1 | 0 |
| `crates/country-core/src/resources/roofs` | 51 | 51 | 40 | 48 | 0 | 3 |
| `crates/country-core/src/resources/rt` | 15 | 15 | 9 | 5 | 0 | 10 |
| `crates/country-core/src/resources/stairs` | 46 | 46 | 36 | 42 | 0 | 4 |
| `crates/country-core/src/resources/walls` | 295 | 295 | 207 | 222 | 3 | 70 |
| `crates/country-core/src/startup` | 77 | 77 | 52 | 48 | 0 | 29 |
| `crates/country-core/src/systems` | 949 | 67 | 700 | 633 | 16 | 300 |
| `crates/country-core/src/systems/audio` | 82 | 82 | 65 | 68 | 1 | 13 |
| `crates/country-core/src/systems/backend` | 33 | 33 | 24 | 19 | 0 | 14 |
| `crates/country-core/src/systems/camera_rig` | 59 | 59 | 32 | 19 | 0 | 40 |
| `crates/country-core/src/systems/collision` | 103 | 103 | 70 | 55 | 15 | 33 |
| `crates/country-core/src/systems/debug` | 0 | 0 | 0 | 0 | 0 | 0 |
| `crates/country-core/src/systems/garden` | 3 | 3 | 3 | 2 | 0 | 1 |
| `crates/country-core/src/systems/history` | 39 | 39 | 21 | 11 | 0 | 28 |
| `crates/country-core/src/systems/ivy` | 23 | 23 | 17 | 17 | 0 | 6 |
| `crates/country-core/src/systems/paths` | 10 | 10 | 6 | 2 | 0 | 8 |
| `crates/country-core/src/systems/render_loop` | 187 | 187 | 155 | 155 | 0 | 32 |
| `crates/country-core/src/systems/save` | 72 | 72 | 51 | 53 | 0 | 19 |
| `crates/country-core/src/systems/steam` | 15 | 15 | 15 | 12 | 0 | 3 |
| `crates/country-core/src/systems/terrain` | 13 | 13 | 12 | 13 | 0 | 0 |
| `crates/country-core/src/systems/ui` | 80 | 80 | 66 | 50 | 0 | 30 |
| `crates/country-core/src/systems/ui_clutter` | 8 | 8 | 6 | 5 | 0 | 3 |
| `crates/country-core/src/systems/wall` | 155 | 155 | 107 | 107 | 0 | 48 |
| `crates/country-core/src/utils` | 52 | 52 | 46 | 38 | 0 | 14 |
| `crates/crash-reporter-gui` | 0 | 0 | 0 | 0 | 0 | 0 |
| `crates/feedback-reporter` | 0 | 0 | 0 | 0 | 0 | 0 |
| `crates/frozen` | 0 | 0 | 0 | 0 | 0 | 0 |
| `crates/geforcenow` | 0 | 0 | 0 | 0 | 0 | 0 |
| `crates/gpu-allocator` | 0 | 0 | 0 | 0 | 0 | 0 |
| `crates/lang-lib` | 10 | 10 | 7 | 4 | 0 | 6 |
| `crates/leekspin` | 3 | 3 | 3 | 0 | 0 | 3 |
| `crates/motomoto` | 11 | 11 | 6 | 3 | 0 | 8 |
| `crates/paths` | 10 | 10 | 10 | 8 | 0 | 2 |
| `crates/rhapsody` | 38 | 0 | 12 | 4 | 0 | 34 |
| `crates/rhapsody/crates` | 38 | 0 | 12 | 4 | 0 | 34 |
| `crates/rhapsody/crates/aftermath-rs` | 0 | 0 | 0 | 0 | 0 | 0 |
| `crates/rhapsody/crates/dxgi-enumerate-gpus` | 0 | 0 | 0 | 0 | 0 | 0 |
| `crates/rhapsody/crates/gpu-profiler` | 0 | 0 | 0 | 0 | 0 | 0 |
| `crates/rhapsody/crates/shader-pipeline` | 38 | 38 | 12 | 4 | 0 | 34 |
| `crates/rhapsody/crates/vk-sync-rs` | 0 | 0 | 0 | 0 | 0 | 0 |
| `crates/rhapsody/src/raw` | 0 | 0 | 0 | 0 | 0 | 0 |
| `crates/rhapsody/src/raw/managed` | 0 | 0 | 0 | 0 | 0 | 0 |
| `crates/rhapsody/src/rg` | 0 | 0 | 0 | 0 | 0 | 0 |
| `crates/shader-variants` | 1 | 1 | 1 | 0 | 0 | 1 |
| `crates/systems/clutter` | 57 | 57 | 27 | 33 | 1 | 23 |
| `crates/systems/color` | 35 | 35 | 24 | 24 | 0 | 11 |
| `crates/systems/critters` | 57 | 57 | 43 | 44 | 0 | 13 |
| `crates/systems/decorator` | 235 | 235 | 153 | 194 | 5 | 36 |
| `crates/systems/forest` | 6 | 6 | 4 | 5 | 1 | 0 |
| `crates/systems/glade-border` | 3 | 3 | 2 | 2 | 1 | 0 |
| `crates/systems/onboarding` | 56 | 56 | 49 | 50 | 0 | 6 |
| `crates/systems/roof` | 36 | 36 | 29 | 29 | 2 | 5 |
| `crates/systems/timberframe` | 5 | 5 | 4 | 5 | 0 | 0 |
| `crates/systems/toolbar` | 358 | 358 | 283 | 160 | 23 | 175 |
| `crates/systems/tree-tool` | 16 | 16 | 13 | 14 | 1 | 1 |
| `crates/systems/wall-constructor` | 127 | 31 | 76 | 66 | 0 | 61 |
| `crates/systems/wall-constructor/src/construct_arches` | 4 | 4 | 3 | 3 | 0 | 1 |
| `crates/systems/wall-constructor/src/construct_elevation_supports` | 18 | 18 | 18 | 17 | 0 | 1 |
| `crates/systems/wall-constructor/src/construct_fences` | 3 | 3 | 2 | 1 | 0 | 2 |
| `crates/systems/wall-constructor/src/construct_gates` | 15 | 15 | 4 | 2 | 0 | 13 |
| `crates/systems/wall-constructor/src/construct_lintels` | 5 | 5 | 2 | 3 | 0 | 2 |
| `crates/systems/wall-constructor/src/construct_platform_stairs` | 4 | 4 | 1 | 1 | 0 | 3 |
| `crates/systems/wall-constructor/src/construct_playermade_stairs` | 9 | 9 | 5 | 6 | 0 | 3 |
| `crates/systems/wall-constructor/src/curved_halftimber` | 2 | 2 | 2 | 2 | 0 | 0 |
| `crates/systems/wall-constructor/src/segmentation` | 14 | 14 | 8 | 4 | 0 | 10 |
| `crates/systems/wall-constructor/src/utils` | 22 | 22 | 15 | 12 | 0 | 10 |
| `crates/systems/water` | 4 | 4 | 3 | 3 | 1 | 0 |
| `crates/tabloid` | 5 | 5 | 13 | 4 | 0 | 1 |
| `crates/tg-gpu-selector` | 18 | 18 | 14 | 5 | 0 | 13 |
| `crates/tg-iced` | 10 | 10 | 10 | 9 | 0 | 1 |
| `crates/tg-integrity-report` | 0 | 0 | 0 | 0 | 0 | 0 |
| `crates/tg-package-integrity` | 4 | 4 | 3 | 3 | 0 | 1 |
| `crates/tg-text` | 42 | 42 | 35 | 34 | 0 | 8 |
| `crates/theme-defs` | 0 | 0 | 0 | 0 | 0 | 0 |
| `crates/theme-fetcher` | 0 | 0 | 0 | 0 | 0 | 0 |
| `crates/theme-server-proto` | 0 | 0 | 0 | 0 | 0 | 0 |
| `crates/tiny-glade` | 183 | 183 | 106 | 103 | 6 | 74 |
| `crates/utils` | 452 | 195 | 362 | 287 | 29 | 136 |
| `crates/utils/src/components` | 10 | 10 | 4 | 10 | 0 | 0 |
| `crates/utils/src/curve` | 105 | 105 | 82 | 66 | 4 | 35 |
| `crates/utils/src/geometry` | 142 | 142 | 118 | 119 | 0 | 23 |

## 归属未知与排除

排除记录口径：`{"user_removed": 131, "callback_instance_not_independent_function": 5790, "auxiliary_review": 14, "daily_deferred": 11}`。这里计的是符号/内联来源记录，不与去重定义组混算。
自有命名空间无源码归属符号共 4087 条，其中 405 条无法用已有定义名定位，581 条也没有已归属业务定义的物理入口。完整图key/RVA见JSON，不计作确认缺业务。
图中的公共依赖、运行时及其余未知归属不纳入自有业务分母。结构模块0项表示当前图未给出可用的自有定义来源，不能解释为已恢复或无业务。

当前源码扫描期间变化：0 文件。清单只绑定本次文件SHA；后续修改需重跑受影响统计与证据检查。

本次执行只读图与生产源码，更新统计和审查清单；未修改生产Rust、运行游戏或把已有测试receipt提升为当前验收。
