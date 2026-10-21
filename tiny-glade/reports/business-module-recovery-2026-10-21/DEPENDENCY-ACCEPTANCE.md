# toolbar → 地形 → 建筑：依赖与逐模块验收

2026-10-21，独立审查。读取当前共享工作树及项目记忆、任务台账、恢复经验和 2026-10-21 三份历史审计；未运行 Cargo/GPU/游戏，未修改生产源码。本报告不重复旧百分比。当前各模块均 **未完成五项业务验收**；在途实现与验证结果应另行绑定源文件指纹。

## 推进顺序和退出条件

1. **toolbar 及共享物理输入仲裁**：真实 OS 输入 → 输入翻译 → widget 命中/遮挡/拖动阶段 → canonical UiState/UiNavState → toolbar button/category/mode/history request → 消费者。先完成这个入口，再允许地形模块报告完成。
2. **地形模块**：真实工具选择 → canvas/hover/readback/focus → stroke 权威状态 → 高度图/物理查询/相关建筑 dirty propagation → 画面；同一条链完成历史、磁盘和重入。
3. **建筑模块内部顺序**：公共墙权威/规则形状输入 → 墙轮廓/高度/相交分段 → 墙砖/洞口/楼板与平台 → roof/flat roof → 门窗及依附装饰 → 玩家楼梯/扶手。先恢复某项依赖的 producer，再恢复其 consumer；不能把门口短台阶等同玩家楼梯完整算法。
4. **联合验收**只检查已逐模块验收过的模块共存：例如地形变化带动墙/屋顶/装饰、混合 undo/redo、保存重入的关联身份和 GPU 资源。联合阶段不补前面遗漏，不将阶段 1 的输入断点转嫁至最终阶段。

| 模块 | 真实输入与原逻辑 | 权威/实体/碰撞/画面 | undo/redo 与保存/恢复 | 删除/卸载/再入 | 原证与实际验证 | 本轮退出门禁 |
|---|---|---|---|---|---|---|
| toolbar | 有真实全版/demo 构造树、按钮/分类/滑块消费者；物理 UiState producer 缺失，导航/遮挡/输入抢占还缺 | 需同时验证 AppMode/Toolbar 选择、组件 selected/keep、布局/命中区、最终 drawable 一致；UI 点击不得穿透成世界编辑 | toolbar 瞬态焦点/动画/冷却不是必须保存的业务状态；由其触发的真实世界编辑必须经过对应历史/存档链；undo/redo 按钮必须从实际输入到 History | 构造会重置 Taffy 并新增实体；必须检查真实会话生命周期下旧实体、导航焦点、widget 表/事件游标/Local 无错误残留，不能任意清除原作允许的持久 Local | 有历史原树/焦点/布局/按钮原证；本轮尚无完整物理输入窗口验收 | 鼠标和导航实际操作；遮挡/拖动/取消/模式变化；工具与历史请求正确；二次会话；原证差分；最终画面与状态一致 |
| 地形 | draw 已使用 CursorTerrainRaycast 和真实 UiState canvas；共享 UiState writer 断点仍阻塞；edit/focus/readback 适配边界需逐一替换 | set15 权威编辑和高度生产已有；需非空 stroke、边界/水/岩石分支、dirty 通知、物理采样与 GPU 实际结果 | set34 record、set40 replay、snapshot 已有接线，必须实际完整 stroke 多步撤销重做，磁盘恢复完整属性，不能只断言排队 | 删除 stroke 后高度/碰撞/引用/缓冲一致；卸载再入与失败取消无陈旧 stroke/cache/readback | 原 TerrainSystems 0x1409455f0/7c0/a20 有原调度；当前未做完整操作验收 | toolbar 入口已验收，且五列逐项有证据，才能称地形完成 |
| 建筑各子模块 | 原墙 set19 链、屋顶 UI/生命周期已存在；规则形状入口、部分 WidgetVolume 和复杂分段仍缺 | 非空墙/屋顶/洞口/装饰必须实体、公共权威、collision/RT、SSBO 和最终像素同步 | 有 typed 历史和 aux owner；完整字段、Stable ID/关联、反向副作用、保存重建仍须模块验收 | 单件/宿主级联删除、卸载、再入、代际复用及 GPU owner 释放须证明 | 不能用历史测试数、安装成功或单帧 GPU 代替业务闭环 | 每个建筑子模块单独关闭五项，不能合并成宽泛“建筑已完成” |

## 当前确认的最高优先缺口

### P1：canonical UiState 没有生产输入仲裁 writer

当前 `country-core/src/systems/ui/resources/ui_state.rs`：`interaction` 为私有字段；生产写入口为 `new` 和 `replace_interaction`。全 crates 检索 `UiInteraction {` / `replace_interaction`，除 `at_startup` 的 inactive 初始化，命中均为测试夹具。`systems/ui/mod.rs` 明确目前只有 widget 注册/query owner。宿主只安装 at_startup，toolbar `render_taffy_ui_to_widgets` 却从该 owner 读 clicked/dragged。

因此直接发送 UiClickEvent、写 active widget 或在测试构造 UiState::new 只能证明消费者，不能证明用户真的可以点击工具。需恢复原命中、遮挡、焦点与阶段状态机，并挂到原调度位置；不能用点击坐标到按钮的临时捷径。

原证入口：`UiState` 注释中 add_ui 0x140945c60/0x140945d25、add_with_id 0x14092e3e0、clicked 0x14092e5f0、dragged 0x14092df10；完整 producer 需继续取证，不能将这些 reader 叶函数冒充 producer。

### P1：toolbar 导航/遮挡/鼠标抢占/edition policy 缺口

`tiny-glade/src/toolbar_plugin.rs` 当前只安装已恢复 set7/8/44 的子集，`hide_ui`、`ui_nav_update`、`toolbar_steal_mouse_input`、demo policy 明确未恢复。菜单显示期间的世界输入抢占必须与上一项合并审查。

原注册证据：`reverse-engineering/tiny-glade/progress-20261020/phase2-toolbar-layout/native-schedule-config-names.json`、`schedule-source-map.json` 和对应 schedule-chains；焦点算法原证 `phase2-toolbar-focus/REPORT.md`、`native-nav-empty-query-gates.json`。空 query 门限 fixture 不证明非空导航算法。

### P2：TOD 按钮遗漏当前导航焦点清除，且误取最早 ModeJustChanged

独立读 `reverse-engineering/tiny-glade/evidence/core-recovery-20261015/gui/0x142342f30.asm`：

- `0x142343146` 检查 toggle 后 visible==1；`0x14234316b mov qword ptr [rax+0x10],0`。当前 `UiNavState.focus` 位于 +0x10，`UiFocus.tag`/`_pad` 是这 8 字节，payload 在 +0x18。应清 tag/pad 并保留 payload、staged_focus、timer、hold；将整个 UiNavState 或 focus payload 清空超过原写面。
- `0x14234322b..0x142343268` 分别计算两个 Event buffer 最后未读元素（`start + count*0x28 - 0x28`），新 buffer 非空优先，否则旧 buffer；`0x14234326c` 比较末条 tool 是否为 0xd。当前 `first_tool` 取首条与原作相反。读取末条同时必须消费全部 reader 游标。

这两项已交业务中央窄修，须覆盖开/关、无事件、两种多事件次序、跨双 buffer、导航其余字段保留；本报告写入时未宣称修复已验证。

### P2：toolbar → onboarding 焦点事件身份分裂

writer：`toolbar/src/ui_interaction/focus.rs` 使用 core canonical `resources::toolbar::ToolbarUiIsInFocus`；reader：`onboarding/src/hint_orchestrator.rs` 定义本地同名 ZST 并读取。宿主还单独注册本地类型，两者不会自动通信。原证 `phase2-toolbar-focus/REPORT.md` 对原函数 0x14233d680 的完整执行明确 canonical 事件身份。业务中央正在做 canonical re-export 修复；不能新增桥接双份事件掩盖身份错误。

## 对旧审计的纠正

- 旧 `DETAIL-building.md` 将 terrain draw 整体描述为 idle hover seam 已不精确：当前 `ui_draw_terrain` 参数实际为 `CursorTerrainRaycast`，用 `UiState::clearing_canvas_w_layer_and_sound`；保留 edit/focus/readback 适配缺口及共享物理输入断点。
- 旧 toolbar lib/模块头有大量历史“未实现”描述已被源码覆盖；不按注释或函数数量给完成率。
- `sample_wall_path_intersections` 当前仍只读取 curve.length，无交点输出；后续恢复墙模块必须关闭，不能把挂 set19 当恢复。该函数原 0x14074c720，约39.6KB，源码引用 building-production-authority/B-intersections。
- 历史/存档大量跨 crate consumer 已有，不再沿用“FlatRoof/tree-tool/stairs 未接”旧说法；剩余是各真实输入、完整副作用和非空多生命周期验收。

## 证据记录要求

每模块交付一份固定源指纹的证据账：原函数/分支与原始载荷、生产实现位置、工程适配边界、实际操作步骤及状态/实体/碰撞/画面输出、undo/redo/save/reload、删除/卸载/再入结果。缺一项则状态为部分；测试直接注入 clicked/interaction 必须标明仅 consumer 边界，不能登记成真实输入已通过。T13 不列入当前阻碍。

## 后续独立定位：共享 UiState producer 的恢复入口

已找到原函数身份（`evidence/country-core-audit/data/procs.tsv`）：

| 原入口 | PDB body | 职责/所需恢复 |
|---|---:|---|
| hover_intersection 0x1408383a0 | 3887 B | 非空 widget 体积射线命中、排序、原 occlusion/layer/distance 分支和 HoverIntersectionCache |
| ui_update 0x140b2de60 | 7577 B | 真正交互状态机 writer、按下/释放/取消、窗口焦点/菜单/Shutdown 及光标/声音/brush 副作用 |
| get_ui_volume 0x140b2dc60 | 503 B | 现有 UiState registry 的 widget 查找和缺失分支，不能自行简化体积集合 |

准确注册原证：`skeleton/task-index/work/TG-025d04787e7e6b72cd/boot-leaf-20261018/host-registration/140945c60-{calls.json,full-B-rne.asm}`。`0x140945c84` 调 `chain_ignore_deferred` 0x14175a380，`0x140945c89` 写 set=6，随后原 in_set_inner / Update add_systems。calls.json 首条保留完整九成员 tuple 的 SystemParam。tuple 中第一个为 hover_intersection，第4个为 ui_update；中间有 DebugGui 的 UI volume 和 roof volume 调试系统，后接 signifier 创建、删除标记、动画、transform、销毁的参数面。各名称和实际 callback 仍应随原 converter/vtable 独立核实，不能只凭参数名安装重复系统。

原 hover SystemParam：ResMut<UiState>、Res<MainCamera>、Res<CursorTerrainRaycast>、Res<CursorPositionSS>、Res<MouseDepthValue>、Res<WindowSize>、Local<HoverIntersectionCache>。

原 ui_update SystemParam：ResMut<UiState>、Res<ActionSetStates>、EventReader<WindowFocusEvent>、EventReader<EscapeMenuJustOpened>、EventReader<ShutdownGameCmd>、EventWriter<OverwriteBrushPreview>、EventWriter<OsCursorVisibilityCmd>、EventWriter<PlayUiSound>。

已有伪码：`reconstruction/country-core-render/wp15/analysis/b5_ui_state/{ui_update_0x140b2de60.c,hover_intersection_0x1408383a0.c}` 和 `evidence/country-core-deep/ui-input/decomp/decomp-0x140b2de60.txt`。**两份 ui_update 都带 bad instruction / overlapping / type propagation 告警，旧 chapter 还混入 wall 曲线断言，不能直接翻译为算法规范**。应以原 RNE 的 7577 字节范围重新建立完整 CFG，定位真正表跳转及冷块，再恢复注册/输入/声音/清理分支。旧 HIGHLIGHTS 也包含已被后续原证纠正的 toolbar 默认值等信息，仅读路径线索。

## 本轮窄修交叉审查补记

2026-10-21：业务中央已落 TOD 清焦点 header/末条 ModeJustChanged 和 onboarding canonical re-export。独立读生产源与 `toolbar/tests/tod_focus_event_order.rs`、`tiny-glade/tests/toolbar_focus_chain.rs`：与上述 ASM 的窄写面/末条事件语义一致，测试边界正确地限定为真实 button consumer 或 publisher→consumer；本审查没有执行测试，不能称通过。toolbar 模块 P1 物理输入链仍开放。

## round12 picker 候选与当前实现交叉审查

原候选与原执行证据已找到，不能重复花费时间重新恢复 `ui_update` 已经有的 600+150 原 fixture。picker 证据 `evidence/round12/ui-picker/INTEGRATION.md`、`native-picker-all-fixtures.json`（39）、`native-arbitration-fixtures.json`（21）；候选并未全部通过：`direct-candidate.log` 明确在第21项 screen-edge-1 失败，不应把 original fixture 数量写成 Rust 通过数量。

确认需窄修：

- **P1：现 toolbar/MEI tag3 producer 丢失完整矩形与distance。** toolbar `ui_interaction.rs` 和 `mei/systems.rs` 当前仅 scalar `WidgetVolumeShape{tag:3,size,...}`；候选 `resolved_geometry` 对未携完整 payload 的 tag3 会 panic。应使用拥有型 ScreenAabb，不得填假min/max/distance。
- **P2：screen helper 边界与原作不符。** 当前 `utils/geometry/aabb.rs::is_inside_bbx` 调 inclusive `Aabb2.contains`；原44B 0x14123ffc0 的 `cmpltps/seta` 四边严格排除，39原fixture已发现真实失败。仅修改此 helper 的四个严格比较，保留 Aabb2.contains/segment closed-box 等现实现。
- **P2：toolbar 和 MEI 误开启world occlusion。** 独立读 `restorer-mei-final/evidence/render_taffy_ui_to_widgets.asm.txt`：142339637 `xor r8d,r8d` 后call1409a8ee0；MEI同目录 `mei_pre_update.asm.txt` 140684e8b同样传false。当前两者true会被world depth错误遮挡。allow_negative=true/no_history原一致。

精确 ScreenAabb 表达式（不猜层级）：

- toolbar ASM1423395b9..5ef：`half=(extent.size+splat(icon_size_mult*10*size_mult))*0.5`，`min=extent.pos-half`、`max=extent.pos+half`，distance是 UiMouseInteractable 的4字节float（`f32::from_le_bytes(bytes)`，实际构造默认0.011）。不是 UiDrawOrder，不应自行加负号。
- MEI ASM140684e19..37：distance是 effective node.z（+ac）的符号位翻转；min/max使用布局像素矩形。ASM140684cd2..ce6分别将pos和size乘scale再相加，所以应 `min=pos*scale; max=size*scale+min`，不以 `(pos+size)*scale` 替换原运算序。

独立核对未发现错误的候选边：完整8种volume（Sphere/SphereCloud/TriangleMesh/ScreenAabb/WorldCanvas/TerrainCanvas/PlaneDisc/Empty）；ordinary stable distance排序，canvas unsigned layer stable排序后append；遮挡从ray/distance重算position，读取view Z row，严格 `depth+1 > -z`；negative/NaN未经clamp，由flags bit2决定保留，保留NaN遇排序比较仍按原panic；等距离mesh/cloud保留先命中元素。证据原 `picker_occlusion_retain.asm` 1412f6680..676b，`hover_intersection.asm` 140838c62..73，两个sort comparator原ASM。

**未确认P2候选，不能擅自改底层库**：当前 Plane::intersect_ray 有finite/parallel guard，而旧225B原decomp只按min_t<=t输出，且点乘求和序不同。39fixture覆盖普通parallel/behind但不覆盖全部大数/非有限。应按真实受影响分支取原225B指令定点核对，保留当前库结构，不回退整套geometry；此次未运行native/Cargo。scheduled picker test 的旧 MainCamera::default 也应适配当前MainCamera::new(aspect)，不加回旧接口。

## Plane::intersect_ray 225B 静态独立裁决（取代上段待定）

从当前提取原 `tiny-glade.rne` 完整反汇编地址 `0x1412803f0..0x1412804d1`，身份 SHA256 **f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03**；输出保存 `plane-intersect-ray-original.asm.txt`。使用 LLVM objdump 静态读取，没有执行原 exe/原函数，没有 Cargo。被审当前 `utils/src/geometry/plane.rs` SHA256 **ab4e7f8d9305aa2249ba542d28ec98c3ed43ffe9e6295e0b6a0a92e71f5efb65**。

**确定 P2：当前算法非等价。** 原函数没有 parallel/finite guard。`0x141280485 DIVSS` 后 `0x14128048b UCOMISS min_t,t`，`0x14128048e JA reject`；无 JP/JAE。unordered 的 CF=1/ZF=1/PF=1，所以 **NaN 不跳转，原返回 Some(tag1)**，不是旧伪码写的 `min_t<=t`。输出在accepted路径先写t与origin+direction*t，再写tag1；reject只写tag0。

精确运算分组，从 xmm shuffle/multiply/add 推导：

```text
delta = point - ray.origin
denominator = normal.z * direction.z
              + (normal.y * direction.y + normal.x * direction.x)
numerator = normal.z * delta.z
            + (normal.x * delta.x + normal.y * delta.y)
t = numerator / denominator
if min_t > t { None } else { Some((t, origin + direction*t)) }
```

两个点积不能替换成同一个 glam::dot 次序，不能额外拒绝 Inf/NaN/zero denominator。该合同应通过当前库的窄修或已有准确能力实现，不替换整个用户重写的geometry架构。当前picker的 PlaneDisc 分支会在下一步 `radius² > squared_distance` 排除 NaN，所以单独Plane返回Some(NaN)不等于picker接收NaN；但有限值求和序和严格边界仍必须保留。已有39fixture的常规平行case不能证明这一底层分支等价。

## toolbar Replay 分支与真实指示符二次交叉审查

原 `render_taffy_ui_to_widgets.asm.txt` 1423397b7 比较 0xd，JNZ进入142339890共享尾；无History且Replay(13)才进special分支。`is_active` false直接跳下一项142338f48；true构造并write signifier后142339884仍跳下一项。两条Replay路径均不执行后面的keep/nav/click/drag。当前 `!Photomode` 判定且继续共享尾错误。canonical AppMode映射在mode_manager/types.rs明确Replay13、Photomode19。

独立从同一f507原RNE静态完整读604B closure 142339f10..14233a16c，保存 `toolbar-replay-signifier-original.asm.txt`；既有函数名/插入类型证据 `components-review-20261018/evidence/0x142339f10.c`，PDB procs确认14089d9b0/new、14089dc90/animation_duration、14089dd50/build、140fdfd20/insert<(Ui2dExtent,Ui2dImage)>、140fdf380/insert<Ui2DFollowMouse>。

closure业务：无mesh的SignifierEntityBuilder，duration0；build实体；插入extent(cursor,splat(icon_size_mult*60))及image("icon_cross",layer0)；插Ui2DFollowMouse；返回单实体集合。caller分配32B含Arc强弱计数头16B，真实closure捕获只有cursor Vec2和size f32；不能把引用计数头转为游戏payload。复用当前from_closure/SignifierEntityBuilder等拥有型API即可。没有执行原程序，没有Cargo，等待input实现冻结后的再审查。

## 2026-10-21 冻结期间候选独立复核

本节仅静态审查，没有 Cargo、GPU 或游戏执行。候选保存在 reports/business-module-recovery-2026-10-21/proposed，不能计为生产已验收。

- WidgetParams：mesh_volume 入口保留；TriangleMesh Vec 移至 canonical geometry owner，无第二份 Vec。volume setter 清除 geometry，resolved_geometry 对缺完整载荷的 tag1/2/3/6 明确拒绝，不能依旧标量构造器伪造。已有 sphere/canvas 兼容面仍在。原完整 signifier 配置尚未放入该候选，不等于已恢复全部 WidgetParams 业务。
- UiState：intersection_point/tag setter 同步 56B payload，replace_intersection 反向同步既有 typed API；完整 replace 同步 active Arc。history active Arc 优先读取修正 map 被 ui_update 清空后的生命周期。旧 replace_interaction 仅低层 fixture 仍可能独立设置交互，生产须走真实 writer。
- picker：当前候选保留 8 分支、负距离/NaN 标志、世界遮挡、稳定排序、原分组优先；原 fixtures 应复用。当前 scheduled picker 做 registry_snapshot Vec 再 extend 缓存，会有每帧临时分配；这是性能审查项，不以删除原计算省时。
- utils 两个候选窄修与上述原证一致：Plane 求和分组及 unordered/Inf 分支正确；is_inside_bbx 严格四边界正确；公有 Aabb2.contains 与现 slab 等库架构未改。未运行候选负例，不宣称测试已过。
- toolbar ScreenAabb producer 候选保留 (size+expand)*0.5、center±half、UiMouseInteractable f32 distance、occlude=false。Replay13 两路径均 continue，History 仍走正常尾，与原 CFG 相符。
- replay.rs 引用 signifiers::SignifierMesh，但当前 signifiers/mod.rs 未 re-export 此类型；接入时需修导入或导出。真实 closure 本体符合 extent/image/followmouse 原合同，但现 consumer 不完整，不能仅凭 factory 入队称画面闭合。

## 原 MEI pre 与 UiState writer 顺序已查明

此前 U 已缩小：**mei_pre_update 必须 after(ui_update)，两者均 set6。** 原 mei-build.asm 路径为 skeleton/task-index/work/TG-c80526daecad7e4117/meihost-20261018/mei-build.asm。

- 14068732a 调 chain_ignore_deferred，PDB 140f98b20 是仅 mei_pre_update 签名的单成员 tuple。
- 140687336 LEA 的 vtable 地址为 142bb7370；140687344 intern，140687352 调 after_inner(140fdde70)。
- 1406873a4 写 set 判别 6；1406873d2 调 in_set_inner(140fddf80)。
- 静态读取原 RNE 的 vtable 142bb7370：+18=140fd9a40、+20=140fd64a0、+30=140fd78f0、+38=140fd7460、+40=140fd7000。PDB 140fd64a0/system_type、140fd7000/dyn_hash 的完整 SystemParam 类型精确对应 ui_update：UiState/ActionSetStates/WindowFocus/EscapeMenu/Shutdown/OverwriteBrush/OsCursor/PlayUiSound。
- 因此 after(ui_update) 有真实原证。原证没有要求 MEI pre 在整个 signifier 尾之后，不应额外发明该顺序。

## 原完整 set6 九系统与现有可接成员的边界

原链：hover → debug volumes → debug roof UV → ui_update → create → mark delete → animation_t → transform animation → despawn，chain_ignore_deferred。当前只有 picker/writer 候选可形成真实核心 slice，不能给其他位置放空函数占位。

| 成员 | 当前状态与问题 |
| --- | --- |
| hover / ui_update | 候选真实算法与原 fixture，尚待移植/生产验证 |
| debug volumes / roof UV | 未发现生产实现；原 1414558c0/9227B、141454e60/2642B，不能注册空体称恢复 |
| signifier_create | 工厂能执行；Raw 分支仅 spawn SignifierId 占位；未处理完整原 UiState signifier 配置，不能整体称原等价 |
| mark delete | 当前使用 stream.active_ids，原签名是 UiState；须核完整 active hash/状态语义 |
| animation_t | 真实时间/缓动逻辑已存在，但接入需保持原调度与生命周期 |
| transform animation | 当前忽略 UiState、SignifierId、ParentToVolumeOrigin，只有 scale，translation switch 缺失 |
| despawn | 当前所有 marked 直接 despawn；原 Query 含 Option<SignifierAnimation>，退出动画门缺失 |

现 add_signifier_systems 还将 mark 放在 animation 之后并用 chain() 自动 flush，与原 mark 在前且 chain_ignore_deferred 不同，禁止直接调用后宣称完整。

## 隐藏 toolbar 的局部门控核对

独立核对原 render_taffy 的 142338f48..142339694：前半是 Query 稠密/非稠密遍历；1423394dd 清 keep，4e9/4f5 的焦点命中仅将 keep 置1；142339515 直接读 extent.size，522读 configscale，694 完成注册。这里没有 state==0、size==0 或 extent 有效性 continue。函数业务入口142338c43..ce0也只是焦点暂存/输入位移逻辑；前面的字符串/追踪代码不能误认成隐藏门。不能为修隐藏点击凭空添加与原版不符的门；须查真实 size/position 动画、菜单覆盖拾取与输入取消链后定位。

## 性能也是每模块独立验收门槛

遵循 D:/game/PERFORMANCE-STANDARD.md：持续低于60 FPS不达标，60至不足120 FPS仅最低门槛，至少120 FPS才达到帧率合格门槛。须使用真实 Release、小窗口物理960×540、完整算法/业务/画质/资源，记录硬件驱动、exe哈希、场景、测试区间、平均FPS、1%低帧与p95/p99帧时间；帧预算最低16.67ms、合格8.33ms。不能以峰值、删逻辑、降画质或跳Logo/伪ready达标。启动、资源流入与存档恢复的主线程响应单列。toolbar、地形、建筑每模块交付都需满足此门，联合验收不负责补漏。当前未进行本agent性能实测，标记未验证。

## 指示符回收精确门与候选编译面追加

从同一原RNE再次静态核对：140929805读取min_lifespan(+4)，14092980a UCOMISS t，14092980d JA跳过。因此非live实体的寿命条件是 `!(min_lifespan > t)`，NaN也会标记，不能用 `<=` 取代。despawn在140929f51检查可选动画，无动画直接销毁；有动画则140929f5a读t(+0x14)，140929f5f比较0，140929f62 JNE跳过、140929f68 JNP销毁：只接受t==正/负0，NaN、正负非零都保留。无需执行原函数即可确证。

原create的active Arc参数还拥有完整signifiers Vec：stream factory先追加到该Vec，再对整个Vec创建/去重；原mark从该Vec重建active hash。当前stream.active_ids只是本帧消费请求，不能替代拖拽保持期间持有的完整参数配置。round12/signifier-runtime/0x140b2cfa0.c:293..352与0x140929160.c:206..220提供进一步静态取证入口。

候选迁移另有编译面：wall_splitter.rs:658测试直接读params.mesh_volume字段，改canonical geometry后须将该断言改为TriangleMesh载荷断言，不能为兼容断言保存第二份Vec。PlayUiSound跨toolbar/onboarding/color等crate结构体字面量亦须按新24B布局适配。未运行Cargo。

## transform translation 的 U 已由机器码缩小

新静态证据 signifier-transform-original.asm.txt 直接来自 f507 原 RNE；原 jump table142ad55ac按已知8种volume读取，结果：0→140a6a285，1→140a6a29d，2/3/4/5/7→140a69eec，6→140a6a28c。Ghidra旧稿误读8..16是越界跳表垃圾，不能作为业务分支恢复。

真实语义：scale flag8只控制scale段，不控制translation。translation需同时满足SignifierId存在、ParentToVolumeOrigin存在、UiState.hover!=0、SignifierId.widget==UiState.active_widget。原140a6a1c8按16B stride取SignifierId，140a6a241比较其+8；当前SignifierId(pub u64)只存hash，丢失widget id，是阻断正确跟随的类型缺口。随后Sphere取center，SphereCloud取centers[intersection.element]并保越界panic，PlaneDisc取point，其余tag不改translation。无需再猜未知全新config enum；当前完整WidgetGeometry足以承载这三类来源。原完整56B intersection已含element(+24)，可使用既有payload读取。

原create 0x140b2cfa0.c:388..511给SignifierId插入的两qword分别为 hash(widget,Option<name>) 与 widget id；不是两份hash，恢复时应由真实create统一覆盖工厂实体的占位标识。
