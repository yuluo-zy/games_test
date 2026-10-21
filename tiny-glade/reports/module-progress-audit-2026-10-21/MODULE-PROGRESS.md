> 当前最新增量见 [1f73dbc73 更新](UPDATE-1f73dbc73.md)：30帧记录已通过；真实完整进程仍卡Logo；cache/Gothic/ICC缺口已有变化。下表保留f729历史基线。

# Tiny Glade 当前模块恢复进度（2026-10-21）

同步基线：`f72938c81fbbece4e6b3a894a20ddd3d37537ef4`，当前 HEAD 与本次 fetch 的 origin/main 相同。相较此前025bc5bce快进6个提交，2674个路径更新。原有未提交文档、sheep源与测试修改已备份保留；没有覆盖用户改动或把rhapsody退回旧实现。

本次是6个只读分工及中央抽查：核对当前生产源、主入口调用与最新已有验证记录。**没有运行Cargo、测试、GPU或新原码探针，没有修改生产源码。**

## 口径

以下区间是“已知业务逻辑实现与实际消费链”的保守工程估计，常见误差5–10个百分点，没有全模块统一需求分母，不是函数出现率、测试覆盖率或整游戏验收率。不能与旧toolbar60%/decorator27%直接相减，也不能按表项求全游戏平均。基础库的当前功能能力与用户重写后的原版等价验收单独记录。父/子模块有重叠，不能重复计分。

“接线”只表示producer/consumer进入实际宿主，不自动表示原算法正确或场景验收通过。已有测试和GPU日志的通过只适用各自源码快照及输入范围，未冒认是本次f729重跑。T13反馈捕获按用户要求排除。颜色UI文档自己的“T16”编号与渲染尾部T16不是同一工作项。

## 最新结论

meadow、grading、自由相机、摄影工具栏已真实装配；FlatRoof/tree-tool/stairs跨crate回放和单一Pop已接；decorator完整validator、双楼梯requests与插值已恢复；渲染T11/T12/T16/T21已有接线。旧总表的这些“缺失”结论已过期。

最大业务缺口仍是装饰物move/place、楼梯cache/主网格/扶手、完整set25/UI宿主；颜色自定义、次级菜单、提示；部分墙交点采样、Path/Garden/地形真实输入producer。完整场景长稳、真实gameplay窗口及编辑→保存→退出→新进程恢复尚未验收。

本轮还确认不能靠接线抹掉算法简化：鱼的四路PathRaster被单水格近似，鸟的32向射线仍为占位表；这些应按原逻辑恢复，不满足用户“业务和算法不允许简化”的要求。

## 模块表

| 类别 | 模块 | 保守估计 | 置信度 | 主要已完成／剩余缺口 |
| --- | --- | ---: | --- | --- |
| 主链 | 启动→装配→非渲染主循环 | 75–85% | 中低 | 真实 session、资产 owner、Startup/OnEnter 及大部分 set 已接；完整原插件顺序和卸载→第二会话待验。 |
| 主链 | 编辑→历史→回放 | 65–75% | 中 | 主要跨 crate consumers 和单 Pop 已挂；多域回放后的实体/碰撞/画面副作用待联合验收。 |
| 主链 | 帧循环→构图→GPU（排除T13） | 75–85% | 中低 | 已有真实场景出图；最新修改后的长稳、实际 gameplay 窗口、完整场景生产者未验收。 |
| 交互/状态 | 主／自由／步行相机 | 80–90% | 中 | 六body和set8/9/10已有真实更新；物理输入、模式、碰撞和最终画面联合验收不足。 |
| 交互/状态 | toolbar／摄影工具栏 | 65–75% | 中 | full/demo装配、滑块、焦点/动画/声音与undo布局已接；nav/hide/steal_mouse/demo仍缺。 |
| 交互/状态 | MEI／Taffy | 70–85% | 中低 | 真实文本/布局/prepare/draw/upload已接；部分widget、原load标签和完整导航/多页面未闭合。 |
| 交互/状态 | LUT gallery | 70–85% | 中低 | 真缩略图布局/点击/formation和set36；物理点击到最终画面仍未完整验收。 |
| 交互/状态 | Escape主菜单入口 | 75–90% | 中 | 17槽状态和save/profile/quit等producer已恢复；不能代替所有下一页业务。 |
| 交互/状态 | Load菜单／备份事务 | 65–80% | 中低 | 实际set36与modal/session/backup/error owner；非空选择、取消和异步handoff待验。 |
| 交互/状态 | Settings／语言菜单／Credits／TOD次级页 | 10–30% | 中 | 入口状态存在，多个下一页消费者和ui主体仍todo。 |
| 交互/状态 | 颜色自定义交互（system-color） | 20–35% | 中 | 事件、部分icons/radial/replay和颜色消费者已有；show/main/实际wall/gate/decorator触发仍缺。 |
| 交互/状态 | 输入／录制／捕获 | 75–90% | 中 | OS到actions、record、camera快照与set1/2/3/5；实际设备/Steam及多模式联合验收待做。 |
| 交互/状态 | onboarding／hints | 35–50% | 中 | 部分条件/冷却/音效；绘制空体和占位载荷仍在，toolbar/core与本地同名focus事件不相通。 |
| 建筑/世界 | 自由墙绘制／编辑 | 60–70% | 中 | draw/snapping/edit/publication主链已挂；39.6KB原交点采样当前仅取墙读length。 |
| 建筑/世界 | wall-constructor／门／栅栏 | 50–60% | 中 | set19/28/34/44局部活链已接；支撑、灰泥、楼板、曲面半木和平台楼梯仍缺。 |
| 建筑/世界 | 规则形状／公共建筑 | 45–60% | 中低 | 部分规则几何和权威状态；形状UX、完整输入/投影和消费者闭环未成。 |
| 建筑/世界 | 屋顶 | 70–80% | 中 | CPU/trigger producer与Roof插件/replay已恢复；部分widget适配、辅助调度及完整tile/交互待验。 |
| 建筑/世界 | 玩家楼梯／平台／扶手 | 30–45% | 中 | requests/插值和部分碰撞叶已恢复；完整cache、主网格、支撑/扶手和视觉宿主未闭合。 |
| 建筑/世界 | timberframe | 65–80% | 中 | 真实set27矩形/圆形/音效和卸载；原矩形9275B已恢复，上游形状UX与构造命令producer仍缺。 |
| 建筑/世界 | 地形编辑 | 50–65% | 中 | 主要CPU/历史及插件已有；hover/readback/focus适配输入仍无主宿主writer。 |
| 建筑/世界 | 路径／鹅卵石 | 40–55% | 中低 | draw/record存在；主App未挂完整用户绘制输入链。 |
| 建筑/世界 | 花园 | 40–55% | 中低 | draw/record存在；主App未挂，pin等仍有事件替身。 |
| 建筑/世界 | 水 | 65–75% | 中 | world raster/历史/实际音频已接；pin语义统一、完整编辑/存档/画面待验。 |
| 建筑/世界 | glade-border | 80–90% | 中 | 真实set36、虚线和越界音效；Demo overlay原事件归属/门禁适配及完整交互待验。 |
| 装饰/生态 | decorator总体 | 40–55% | 中低 | 真实loader/validator/rotate/typed patch已推进；完整move/place、cache、UI/set25宿主和卸载缺口。 |
| 装饰/生态 | clutter／autoclutter | 45–60% | 中低 | ClutterPlugin已实挂；region/filter/sort猜测及非Window输出合同未闭。 |
| 装饰/生态 | 窗户／玻璃／窗灯 | 50–65% | 中低 | canonical灯/TimeOfDay/玻璃数据已有；哥特动态几何与全部generator→Nani未闭。 |
| 装饰/生态 | 装饰门／阳台门 | 30–45% | 中低 | 资产与部分叶函数；builder/附加记录未真正消费，完整视觉未挂。 |
| 装饰/生态 | ivy | 45–60% | 中低 | startup owner已接；grow/spawn/prune/叶段上传主App消费未装全。 |
| 装饰/生态 | forest | 75–90% | 中 | 真实插件和placements/canopy已接；视图移除谓词仍有surrogate，多帧编辑/历史待验。 |
| 装饰/生态 | 羊 | 75–85% | 中 | 真实生成、移动/反应/资源和音频；跨帧多模式/历史及视觉听觉联合验收。 |
| 装饰/生态 | 鸭 | 70–85% | 中 | 计划/移动/建筑反应和真实音频；spawn/despawn残余近似及整世界行为待验。 |
| 装饰/生态 | 鱼 | 40–55% | 中 | 基础生成/boids；BoidFlock持久辅助状态缺失，原四路PathRaster被单water raster近似。 |
| 装饰/生态 | 蝴蝶 | 65–80% | 中 | state0–3及目标/速度/反应；state4–7、query派生写入和完整GPU消费未闭。 |
| 装饰/生态 | 萤火虫 | 75–90% | 中低 | 目标/速度/实例数据；原数值边界及完整GPU/物种联验。 |
| 装饰/生态 | 鹪鹩／鸟群 | 45–65% | 中 | 部分perch/状态机；32向射线仍golden-angle占位，认领/释放/树冠关联近似。 |
| 装饰/生态 | meadow／grass／sky＋TOD | 80–90% | 中 | 真实OnEnter、drawables、GPU owner和时间更新已挂；完整主题/季节/画面动态联验。 |
| 装饰/生态 | weather整体 | 75–85% | 中 | 有实际set36和完整小雪数值叶；更广的渲染、音频、季节动态未全验。 |
| 装饰/生态 | audio | 70–85% | 中低 | native/disabled owner、bank/worker、域事件、水声/music已接；完整载荷、听觉和多帧卸载取消待验。 |
| 存档/回放 | History账本／Undo／Redo／Pop | 80–90% | 中 | 真实owner/queue/cursor及单Pop；整游戏多步反复编辑和副作用重建待验。 |
| 存档/回放 | 文件／worker／备份／GC | 80–90% | 中 | 真实写盘/flush/close/join与错误owner；原格式多版本、多会话和当前基线联验待做。 |
| 存档/回放 | snapshot／加载与全域恢复 | 70–80% | 中 | 14节、真实域请求与六raster staging；stairs checksum格式及全部实体/几何/GPU消费未闭。 |
| 渲染 | rhapsody | 85–95% | 中 | 分页、租约、事务、动态图资源/提交已落地；生产接线粗估75–85，最新长稳与全owner寿命待验。 |
| 渲染 | W1–W54世界构图 | 85–90% | 中 | 世界图与实际GPU出图；生产接线75–85，缺完整场景producer和稳定验收。 |
| 渲染 | 尾部／截图／UI合成（排T13） | 85–95% | 中 | T11/T12/T16/T21和截图purpose等已接；接线75–85，ICC custom_lut仍None。 |
| 渲染 | Nani／实例上传 | 75–85% | 中低 | 实例/稀疏上传已有；主set44仅部分tuple，上游完整几何未挂，接线55–70。 |
| 渲染 | shader协议／编译链 | 80–90% | 中 | 协议、编译与真实后端消费已有；完整算法/反射/画面等价未验，接线75–85。 |
| 渲染 | 窗口／呈现 | 80–90% | 中低 | 真实loading WSI和gameplay严格入口已有；接线65–80，gameplay窗口及长稳未正式通过。 |
| 基础能力 | utils／geometry／asset／paths／lang-lib | 85–95% | 中 | 当前功能较完整；用户重写后的原版等价未统一复验，不能继承旧源码SHA的验收。 |
| 基础能力 | asset-pipe | 80–90% | 中 | mip/压缩/数组/cache等能力；完整资产编译→游戏→GPU待验。 |
| 基础能力 | motomoto／tabloid／integrity | 85–95% | 中 | 容器/发布/manifest等已有；payload schema、完整消费者和部署验收分别处理。 |
| 基础能力 | tg-text独立库 | 85–95% | 中 | 重写后真实CPU/GPU证据；游戏中全部页面/字体/平台验收仍独立。 |
| 基础能力 | tg-theme库／每日主题整链 | 80–90 / 35–50% | 中 | 原三主题库已合并；每日主题更新宿主仍no-op，未消费库订阅。 |
| 基础能力 | tg-gpu-selector库／宿主整链 | 70–85 / 40–55% | 中 | 库策略/GUI runner存在；真实生产入口与持久化/选择器未闭合。 |
| 基础能力 | tg-iced原功能 | 60–75% | 中 | native runner/基本对话框；原选项、默认值、完整样式与生产runner未全映射。 |

## 证据和完整工作区归类

完整分工与当前源码行号：

- [启动/相机/工具栏/菜单/输入](DETAIL-ui.md)
- [建筑/墙/屋顶/地形/水](DETAIL-building.md)
- [装饰物/clutter/ivy/门窗/楼梯](DETAIL-decorator.md)
- [历史与存档](DETAIL-save-history.md)
- [rhapsody/渲染/Nani/shader/窗口](DETAIL-render.md)
- [基础库与生态/音频/主题](DETAIL-foundation-ecology.md)

现有最新核心记录是 CURRENT-INTEGRATION.md 的最终V4：core2838通过/1ignored、decorator378通过/2ignored、utils160通过，默认及native-audio check通过。这是既有记录，未与本次全部最新源重新绑定。旧CENTRAL-REPORT中的364及FMOD/CR阻断不能覆盖后续进展；部分round12 v20实际编译失败，33存档/2120记录是语料数而非本次通过数。真实GPU已有原资源单帧和loading3次native成功；旧30帧实际执行后稳态断言失败。f729的lavapipe间接绘制修复已落源，未找到其后完整长稳成功收据，不能用旧失败直接断言新修复无效。

当前workspace共33成员。下面按包映射上表业务，避免误把工程抽离包和core重复计分：

| workspace package | 本报告范围 |
| --- | --- |
| tg-terrain | 地形编辑 |
| system-water | 水 |
| forest | forest |
| system-glade-border | glade-border |
| timberframe | timberframe |
| system-tree | Tree工具/forest/Tree历史；真实插件和19项既有回归不代表完整树编辑验收，Tree历史见65–75%细项 |
| system-color | 颜色自定义20–35%，颜色历史75–85%见细项，二者不能混算 |
| tg-building-runtime | 工程适配/独立harness，无main安装；不独立折算原版恢复率 |
| tg-history | 工程抽离格式访问层；不独立折算原版恢复率，历史主链另计 |
| tiny-glade | 启动装配、菜单、窗口呈现和各宿主接线 |
| country-core | 相机/输入/历史/存档/建筑/资源/生态/render等横切领域，不提供重复的crate平均 |
| system-roof | 屋顶 |
| system-wall-constructor | wall-constructor／门／栅栏及楼梯 |
| system-decorator | decorator、窗门及装饰楼梯 |
| system-critters | 羊/鸭/鱼/蝶/萤/鸟分列 |
| system-clutter | clutter/autoclutter |
| system-onboarding | onboarding/hints |
| toolbar | toolbar/摄影/MEI/LUT及菜单部分 |
| rhapsody | rhapsody |
| shader-pipeline | shader协议/编译/反射和shader ABI |
| utils | utils/geometry能力 |
| paths | paths |
| lang-lib | lang-lib |
| tabloid | tabloid |
| motomoto | motomoto |
| tg-package-integrity | integrity |
| tg-theme | 主题库与每日主题整链分列 |
| tg-text | tg-text |
| asset | asset |
| asset-pipe | asset-pipe |
| tg-iced | tg-iced |
| tg-gpu-selector | 库与宿主整链分列 |
| shader-compiler | shader协议/编译链 |

源码核对重点：run_game实际插件；decorator_plugin候选只有定义；cache stair4/roof63；history_runtime统一raster及单Pop；实际GUI空体；同名focus事件TypeId不一致。没有用旧扫描中“函数名匹配”或todo数量推导上述百分比。
