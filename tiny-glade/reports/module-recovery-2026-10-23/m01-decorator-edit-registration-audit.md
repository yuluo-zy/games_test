# M01 形状编辑装饰物消费者原注册核验

2026-10-26。只读取原B，不修改run_game/consumer。原RNE身份同m01-height-recovery.md；新鲜Ghidra结果、原ASM均存同目录m01-decorator-*。

## 注册合同

原 `DecoratorPlugin::build` 140342c10 的 Update注册路径为：

- 140344e26 调用 `chain` 140105360，构建四组链：`auto_allow_decorator_culling` → **五成员普通元组** → `move_decorators_following_anchors` → `cache_decorator_transforms`。第三叶构造器14037d280的原PDB型明确为move_follow；第四叶1400c6200在1400c62b3引用cache名字14293604a（长度72）。
- 五成员元组构造器1400c5cc0，返回`ScheduleConfigs::Configs` len5，`+0x20=0`（1400c607e），没有调用`Chain::set_chained`，**元组内部没有原始顺序边**。
- 五成员构造顺序：move_shape → rectangle_edited → wall_height_changed → freehand_wall_edited → circle_rotated。该顺序不能冒充实际执行的before/after。
- 外四组的`Chain::set_chained`位于1401056e7；该组又被原顶层19项chain包含。
- 顶层调用1403451af `chain`1400d52f0；1403451b4写GameSystemSet **0x19=25**；1403451e2 `in_set_inner`；140345253 `SubApp::add_systems<Update>`。
- 此plugin及上述组无`run_if`调用、无条件系统Vec填充。应该沿用生产宿主既有GameSystemSet生命周期/状态条件，不在三consumer上新增推测的`is_gameplay_active`。

| 消费者 | 原body | 元组构造证据 | 原参数 |
|---|---:|---|---|
| decorator_on_move_shape | 140811fc0 | move名字142936092由1400c5dd8引用，首成员直接into_config@1400c5e8d | DecoratorStorage mut、OnShapeMoved Reader（body/原PDB另有完整面） |
| decorator_on_rectangle_edited | 140819c00 | 1400c5e99→into_configs140048e10 | **InnerWalls**、OnRectangleEdited Reader、DecoratorStorage mut、DecoratorArchivist mut、DecoratorArchivalResources |
| decorator_on_freehand_wall_edited | 14081bac0 | 1400c5eab→into_configs1405c7170 | PublicWalls、OnFreehandWallEdited Reader、DecoratorStorage mut |
| 配套wall_height_changed | 原同模块 | 1400c5ea2→into_configs1401691d0 | PublicWalls、AdjustWallHeightCmd/AdjustWallHeightUi/FinishAdjWallHeightMode/ReanchorDecorators四Reader、DecoratorStorage mut |
| 配套circle_rotated | 14081c060 | 1400c5eb7→into_configs1405c6df0 | PublicWalls、OnCircleRotated Reader、DecoratorStorage mut |

三个被询问consumer属于同一五成员元组；不能因为生产者在set17，就把消费者也放set17。它们应在墙体发布/高度更新完成后的set25运行，随后move_follow，随后cache。原顶层19项chain1400d52f0的slot0包含上述四组；slot2另有cache（1400d54e7调用1400c6200，返回值写config数组+e0于1400d58e5..5916），slot4为subtype派生、slot5为visual更新组、slot6为十五成员生成组。因此五消费者到既有cache/visual链的before约束有原传递顺序依据。生产挂接仍须逐一确认，不能以函数存在代替完成。

初次核验曾根据相邻业务猜测第三、四叶为两个cull；已由叶构造器名字和原ASM否定。历史文件`m01-decorator-cull-chain.asm`实际保存上述四组链，文件名不代表其内容属于cull，后续不得据此实现旧猜测。最终父树证据见`m01-decorator-set25-parent-native.json`、`m01-decorator-set25-parent.asm`及`m01-decorator-cache-config.asm`。

## 事件初始化

三种事件均由原 **WallSystems::add_wall_events 1404936b0** 注册（新鲜完整39236字符反编译见`m01-decorator-wall-events-native.json`）。它们不由原DecoratorPlugin或原add_ui注册。

| 事件 | 原register_event函数 |
|---|---:|
| country_core::resources::events::OnRectangleEdited | 140567510 |
| country_core::resources::events::OnShapeMoved | 140576c90 |
| country_core::resources::events::OnFreehandWallEdited | 14056acc0 |

原add_ui 140945c60只装其Update/set6 UI框架链、UiState、UiNavState、PlayUiSound/PlayPhotomodeSound两事件及UiSignifierStream，不拥有墙编辑事件。

安全Rust可在原所属生产者模块定义事件并由resources::events re-export保持同一TypeId；关键是生产者、消费者、App.add_event必须用同一个实际类型，不可用载荷近似的另一个本地Event。

## 初查缺口与收敛记录

- 初查时move/freehand尚缺生产host注册、rectangle仍是错误PublicWalls/本地80B近似事件合同、高度和circle未见直接注册；这些是初查状态，不代表最终实现。
- 中央随后按原普通tuple接入五consumer并添加到既有cache的before边；rectangle/circle由对应owner恢复真实参数与原事件，高度消费者本轮六参/四流恢复并通过5项定向测试，见`m01-height-decorator-recovery.md`。
- 后继move_follow原来是手动四参helper与虚构返回Vec，门底只返回typed gap，不能挂真实ECS。本轮继续恢复原九参、四事件流、两真实Tracker和门底profile，详见`m01-anchor-follow-recovery.md`。是否生产整条调度通过须以中央集成日志为准，不以函数存在替代运行验收。

## 取证经验

当注册函数的反编译只显示mangled泛型名且无系统名字，先在原二进制找精确system_name字串，再用get_xrefs_to追into_configs构造器。这样本轮由move名字142936092直接定位1400c5cc0，展开得到真正五成员与参数类型；避免把大插件里的构造调用先后当成运行顺序。
