# 最新实现复核与后续方向（2026-10-20）

基线：`fa53d49a61dd7496cddc9af054c6fb1899fa1676`。实际fetch成功，HEAD与origin/main一致。本轮依据当前源码、调用方、实际编译/测试与现有图像复核，不沿用旧缺源状态、函数统计或提交标题作为完成证明。没有改生产源码；用户的startup_sheep.rs工作区变化保留。

**结论：已进入真实场景经生产渲染链出图并具备连续帧测试的阶段；完整可编辑、可重载且长时间稳定运行的游戏主链仍未完成。**

## 本次实际执行

| 验证 | 当前结果 | 边界 |
|---|---|---|
| `cargo check --offline --locked -p tiny-glade` | 失败，3处E0433 | 本机macOS默认feature；NativeStudio引用未跟随native-audio模块门控 |
| 同命令加`--features native-audio` | 通过 | 主包类型检查，未完成链接/音频运行/窗口启动验收 |
| country-core native-physics，eval_gi_screen_radiance单元测试 | 7通过、0失败、0忽略 | 当前完整core测试目标实际编译，执行该模块7项CPU合同测试；不称GPU整帧测试 |
| 实际tracked场景图 | 已目视检查06-sheep-fixed-480x270.png | 可见地形、草、树、墙和小型羊像素；不是本轮重跑，也不是原作同机位画质等价证明 |

完整命令、耗时、日志和指纹见同目录AUDIT.json与三个原始.log/.json。编译失败日志同样保留，不通过开启feature隐藏默认入口错误。

## 当前已推进的主链

1. 旧缺源已变化：sampling、startup/clearing、preplaced_autoclutter、history_runtime均已落源；生产renderer使用country-core的systems/render_loop/world_frame，不再依赖旧上层假定路径。原“缺文件所以整包无法查验”的判断过期。
2. 正式入口保留OS事件→loading/gameplay App交接→更新→draw lists→生产world graph→提交/呈现。production_renderer默认遇到缺producer返回错误，只有显式TG_ALLOW_MINIMAL_WORLD_FRAME=1才允许clear/export诊断，不把诊断帧当正常玩法。
3. Startup/OnEnter(Game)资产与实体、主体墙/gates/fences、树草天空和羊已有真实接线。当前场景测试从原shipped session进入实际App与图，并走真正截图owner读回像素；不是单独画三角形。
4. Rhapsody已有AttachmentCache无外部引用view回收、动态常量页提交后封存、描述符类别/容量诊断；连续30帧测试现在会强制检查TAA历史、计划缓存命中及设备内存不增长。这是实现与断言进展，不能只看测试存在就认定最新运行已通过。
5. 最新GI修复将Reproject GI的output_tex标记为preserve，保住前序Eval screen radiance内容。当前7项测试实跑通过；原数值访问0x37保留不等于可以丢弃旧内容，修复沿当前图语义适配。
6. Toolbar和history_runtime已经进入主App，多个墙/树/颜色/楼梯回放与资源生命周期消费者新增接线。实际鼠标编辑、完整保存重启和同App第二会话仍需另外验收。

## 阻碍与优先级

### P0：本机默认入口构建

run_game.rs:329以及gameplay_startup.rs:263/285无条件引用native_audio::NativeStudio，但resources/mod.rs以feature=native-audio门控。默认Mac构建确实失败，开启feature后通过。先恢复受支持平台/feature组合的一致启动与卸载注册，保留真实音频失败边界；不能仅改验证命令就标记问题解决。

### P0：窗口长时间稳定性与跨平台实际运行

现窗口状态文档记录Linux/lavapipe玩法提交/排队呈现后，约240–420总帧间StorageBuffer bindless表耗尽；当前宿主需求仍为16384，并已补live/retiring/capacity错误输出。该记录的根因尚未闭合；本轮未重现，不能将它当新的实测失败，也不能因扩大容量就宣称修复。应区分真实live增长、GPU未退役积压和cache长持有，沿原owner寿命修复。

tracked30帧报告和旧allocations-frame05/29记录有0/28缓存命中、内存增长的数据；它们来自531e64625，而AttachmentCache修复ee331360d与严格测试975e2e1f1在之后。因此旧数据既不能证明新代码仍泄漏，也不能支持“最新稳态全部通过”。需要新HEAD绑定的完整日志、每帧指标和截图。还要实测真实GPU、窗口resize/最小化恢复/焦点/vsync/out-of-date/DeviceLost/OOM和第二session；排队present计数不等于完成屏幕呈现。

### P1：装饰物编辑、生产与卸载尚非闭环

主App调用的是decorator_plugin::build_startup_resources；build_recovered_visual_systems当前仅定义，无生产调用；register_ui_systems只由旧完整build包装调用，该包装同样未挂主入口。不能把资源初始化称为实际UI/完整set25安装。

ui_move_decorator.rs:62的主体仍只做dispatcher门控后空转，find_stair_placement_candidate返回None；ui_place_decorator.rs:49目前处理初始化/id生成，未完整执行选点→意图→修改Storage→visual更新。它们是可见的实质业务缺口。

window_lights.rs仍使用本地cycle_t、全局随机、猜测retain与sin插值；原查询/EventReader/TimeOfDay、expf和canonical玻璃链的正确合同已有证据，root工作目录的8源草稿未冻结/未合入，不能计完成。

lifecycle.rs是有明确边界的部分owner reset，未挂App，缺真实StairColliderRegistry等合同；decorator_plugin旧完整unload仍todo。墙的单独卸载与clearing实体清理不能替代装饰分配、碰撞、registry、deferred Commands和同GPU在途资源的完整卸载顺序。需按真实second-session场景验证。

### P1：墙体/屋顶/地形等场景覆盖仍不足

app_systems已恢复若干set19/28/34/44成员，但明确未挂完整set24支撑/柱碰撞、set32灰泥/曲面半木、完整楼板/平台楼梯等业务。wall-constructor仍有真实todo实现。可渲染shipped初始场景不覆盖全部可建造样式、地形修改、屋顶/Dormer与窗口衣物分支。先完成实际producer→同资源→geometry/upload→draw的闭包，再启用对应阶段，禁止空函数或默认成功。

### 后续：编辑与存档验收及其余核心模块

历史/保存有新源码和真实接线，但回放字段保留、加载成功或单次落盘不是“编辑→undo/redo→保存→退出→新进程恢复”的完整证明；当前渲染优先可先保该边界。主链稳态与关键编辑闭环完成后，再按核心业务模块逐个恢复。不要此时按原函数清单均摊深挖，造成入口和生命周期长期悬空。

## 建议推进顺序与验收

1. 修默认Mac/native-audio配置，固定Windows/Linux/macOS支持的明确构建与运行命令；实编译且不丢音频/卸载业务。
2. 当前HEAD跑真实GPU非空场景首帧及连续帧；补齐计划缓存/内存/descriptor的每帧证据，再做窗口长测和生命周期异常。以问题规模选择复现，不用软件提交耗时作真实GPU性能结论。
3. 沿已有原证完成窗灯→canonical builder→glass、装饰物放置/移动与真实set25可视化；同批补真实卸载/registry，测试再挂主入口。
4. 逐样式补齐墙/屋顶/楼梯/地形的必经geometry和宿主，进行实际鼠标交互、undo/redo和reload；每个分支都验证实际draw/碰撞，而不是资源存在。
5. 保存重启恢复与双会话资源退役通过后，再深入其余核心模块，持续登记原版业务、工程适配、未知项和实际验证边界。

本次未提交/推送；未套用旧候选覆盖新HEAD；未改用户脏文件。本文是当前实现审核，不是整体完成百分比。
