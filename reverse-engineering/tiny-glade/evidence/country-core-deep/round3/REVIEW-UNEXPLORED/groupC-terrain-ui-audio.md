# 组 C（terrain-water + ui-input + audio-misc）未探究项审查

- 日期：2026-10-15；依据：三域 DIGEST/REVIEW/CLOSURE + round3 oa2（w6/w9）+ round3 O-AUDIT-REVIEW（seed=42 40 条）+ SOURCE_MAP 交叉 + 本组独立 10 行抽查。
- 判定记号：O=机器码证实 / I=有界推断 / U=未恢复；**已修**=章节/source 已按抽查修正，**未修**=抽查指出的瑕疵仍在。
- 未修改任何源码/章节（仅本报告）。

---

## 1. U/I 行盘点（全量）

### 1.1 terrain-water（I=6）

| # | 行 | 级别 | 内容 | 处置状态 |
|---|---|---|---|---|
| 1 | build_tile_hasher 0x9393e0 | O 字节/I 语义 | 4×u64 种子常量（ahash 风格 4-key RandomState） | 未重开，保持 I（瓦片哈希器 seed 语义） |
| 2 | ui_draw_terrain / ui_edit_terrain（0xa9fdc0/0xaef1f0） | O 签名/I 内部 | 事件/命令生成面，ui-input 域共享 | ui-input round2 未接手地形事件面；terrain REVIEW 标 STUB（反编译已落盘，落码级缺口） |
| 3 | ui_terrain_focus_mode 0x9a4fb0 + 4 闭包 | I/U | 167KB 全内联；闭包地址 O，内部状态机 U | round2 CLOSURE #6 边界 O（曲线采样/进入退出高亮）；章节已按粗粒度语义升级；细粒度状态转换语义仍 U |
| 4 | garden::build 0xb6d290 | O 签名/I 全表 | 系统注册面 | 未重开（I 保持，注册清单契约已有） |
| 5 | replay_terrain / TerrainReplayCmd 0x995150 | O 框架/I 细节 | 0x40B 条目迭代、forward/backward 1482B pair 应用/撤销细节 | 未重开（I 保持） |
| 6 | 0x07954dbb 瓦片状态字 | O 结构/I 语义 | 初始化+clear_all 复位点 O（round2 CLOSURE #2）；语义推断"无水/空瓦片"denormal 哨兵 | 结构闭合；编码语义 I 级（未获得新证据） |

另有 U（不计入 I=6）：+0x29b 置位动态路径（§2.1）、+0x48..+0x5f 次级 Vec 用途、subscribe 记录游戏侧回调语义（tabloid 边界）、WaterMaskSsbo/WaterExcludeClutterMaskSsbo/GardenMaskSsbo/RippleSourceSsbo 字段（PDB name-only）、rasterize_stroke 三 blend 闭包 zeno 定点细节（prior 已标边界不重开）。

### 1.2 ui-input（I=3）

| # | 行 | 级别 | 内容 | 处置状态 |
|---|---|---|---|---|
| 1 | check_if_can_switch 语义 0x140afa8a0 | I | 模式切换请求→pending=Replay(0xd) 联动 | **已闭合**（round2 CLOSURE #1：尾段=unreachable!() panic；模式 id=10+5k×类别视觉序全映射 O）；残留 env_4/env_5 动态类别细节 U |
| 2 | Events fork 双布局（{event@0,id@8} vs {id@0,event@8}） | I 待定 | mouse_capture 0x10B push 与 translate_input_to_cursor_moved 0x10B push 形状并存，与官方 bevy_ecs 0.16 EventInstance{id@0,event@8} 不一致 | **未闭合**——round3 无引擎层裁决；input 域另发现 MainActionSet 双形态（0x170B vs 0x58B）为相关但独立问题 |
| 3 | PhotoModeCameraMode 0..3 ↔ CameraMode 槽 6..12 | I | 槽 6=6 姿态子跳表、槽 7..12 fov 偏移 O；0..3 判别→槽的写入点在 toolbar::photomode | **结构闭合、判别映射 ⛔**——toolbar crate 不在 skeleton；camera-time 侧同卡点（previous 表仅覆盖 0..7），双向挂起 |

另有 I（未计入）：UiFocus 变体序 None=0/Toolbar=1/Mei=2/World=3、ui_edit_focus 结尾 0x18B 事件（UiHistoryPlaySelectAnimation 类）、FUN_140a1c930 谓词（需 history 域确认，地址已给）。U：EscMenuState 变体名、update_mod_key_state 真实地址（ICF）、Modifiers 位标志完整位定义、Toolbar::new 类别表 (0xb,0x3f)/(0x12,0) 条目真伪（Ghidra 类型混淆风险）、switch_mode panic 串完整原文、next_mode 真实地址。

### 1.3 audio-misc（I=1 / U=1）

| # | 行 | 级别 | 内容 | 处置状态 |
|---|---|---|---|---|
| 1 | placement_data::to_affine3a 末元素（m33=position.z） | I | 机器码证实原版把 position.z 写进 w 行第 4 元素；现有代码为忠实转录非笔误；消费面未定位 | 保持 I（原版怪癖已登记，建议章节加注） |
| 2 | update_steam_inputs 逐动作→游戏输入码配对 0x140ae9740 | U | 14 同构块归纳 O（输入码 0x200..0xf00、+0x170/0x178 按住/释放簿记、2B 元素环形队列）；码→动作名配对需 ui-input 域游戏输入枚举 | CLOSURE #2 PARTIAL：结构闭合，配对卡点（跨域） |

其余 U 已 CLOSED：MusicPlayer::update 双跳表（CLOSURE #1 O，4 状态）、0x14362c420（=panic 进行中标志，与 history 域互证）、0x140949c40（=进程静态析构注册，唯一 DATA xref 0x14368e2fc）、FeedbackReportResources +0x18..+0x36（结构级）。残留 U：MusicPlayer A1/A2/B1/B2 阈值常量语义、0x1408b5b52 后精确动作、AudioLoopPlayer 参数 Vec 精确字段、FeedbackReport 元数据 6 qword 字段含义。

### 1.4 已抽分片报告瑕疵核实（w6/w9 → 是否已修）

| 瑕疵 | 抽查结论（w6/w9） | 现状核实 | 已修? |
|---|---|---|---|
| DebugSphere::radius 0x140b024e0（w9） | PDB size 31B==机器码；形态="改临时+整结构拷贝" by-value builder（float 走 xmm2→+0x10）；"reconstruction 签名需复核" | reconstruction 现为 `radius(mut self, r: f32) -> Self`——与 40B 结构 by-value builder 的机器码形态**一致**，签名无需改；但章节证据仍只写"原字节短函数"（薄，未错） | 签名：无需改（一致）；章节：未补形态描述 |
| DebugRectangle3d::dashed_line 0x140b02ea0（w9） | 38B==PDB；bool→+0x34 + 56B 整拷贝 | 章节"原字节短函数"方向正确 | 一致（薄证据） |
| into_drag_state 0x14092df10（w6 不一致） | 真实 26B 函数（非折叠）：默认返回 **2**；`[+0x18]>=2 && [+0x38]==[rcx+8]` 时返回 byte@+0x19；语义（is_started）未证实 | 章节 **仍写**"O：折叠（NotFound）——参数校验（与 UiWidgetContext::dragged 同址折叠）"；reconstruction `into_drag_state()` 仍无条件返回 `DragState::None`（机器码默认 2 不符）。注：w7 复核 0x14092e6c0=DragState::is_started（PDB exact）→ 判别链可钉变体序 | **未修**（章节+源码双遗留） |
| on_drag_release 0x1409a90c0（w6 不一致） | 真实 70B 结构复制/重排（"折叠"证伪；槽 6-8 触及 0x30..0x48） | 章节已注"round3 round2 复查修正：真实 70B 结构复制/重排" | **已修** |
| SignifierEntityBuilder::new 0x14089d9b0（w6 不一致） | 唯一引用证据"不存在"+“35 callers”无 xrefs；w6 断言 wp15 目录整体缺失 | **w6 前提错误**：wp15 与 `wp15/analysis/b6_widgets/new_0x14089d9b0.c` 实际存在（52 行真实反编译，默认值序列实证）；章节已删除"35 callers"并改引真实路径 | **已修**（但 w6 该行的"目录缺失"依据本身要作废） |
| move_gizmo_and_trigger_vol 0x14097d280（w6 证据不足） | "wp20 目录整体缺失" | wp20 与 `wp20/analysis/0x14097d280_move_gizmo_and_trigger_vol.txt` 实际存在（356 行真实反编译）；章节已补证 | **已修**（同上：w6 依据作废） |
| Toolbar::new 0x140af8c50（w6 证据不足） | 行为断言无证据；0x140af9000 在其体内 +0x1b0，"不同单态"可疑 | 章节已降级为"O(部分，round3 round2 复查修正)"；0x140af9000 实为函数体内地址 | **已修**（降级） |
| texture_watch 0x14003bb80"句柄释放对"（w9） | 0x14003bb80 实为 1B ret no-op（Plugin::finish ICF） | chapters/systems/backend/texture_watch.rs.md L11 仍写"`FUN_140be24b0`/`func_0x00014003bb80`（句柄释放对）" | **未修**（次要；应改注） |
| 0x14003bb80 簇（O-AUDIT 5 处） | 实为 Plugin::finish<MainCameraPlugin> 1B ret | draw_path/slider3d/steam 各 3 行已注"round3 复查修正"；world_raster.rs.md 该址 3 行本就正确（ICF=裸 ret） | 已修 |
| 残留模式 | — | chapters/systems/ui/signifiers/mod.rs.md L24 `animation_duration` 仍带"（35 callers）"（无 xrefs 支撑，与 new/rotation 同模式未清） | **未修**（同类数量断言残留） |

---

## 2. 遗留卡点复查表（round2 CLOSURE 逐项）

| 卡点 | round2 结论 | 本轮复核 | 状态 |
|---|---|---|---|
| **+0x29b 再上传脏字节置位点** | U：.text 穷举扫描无任何直接寻址置 1（读=water/garden/path-masks rasterize_from_cpu，写=仅清 0）；收窄到 (a) 计算寻址或 (b) GPU→CPU 写回；tabloid 回调链=依赖边界不复原 | round3 无新证据；该 U 仅存于 DIGEST（章节 U 节未收录 +0x29b） | **未闭合（U）**；建议章节登记 U + 依赖边界标注 |
| **0x07954dbb 瓦片状态字** | O 闭合（new 0xa15530 + clear_all 0xa15da0 双写点）；语义 I：denormal≈0 的"空/无水"魔数哨兵（非精确 0.0） | 无新证据；章节 clear_all 行已落值 | 结构闭合；**编码语义 I 级**（可作注释约定，无需再挖） |
| **ui_terrain_focus_mode 闭包命名** | I：边界 O（4 闭包地址+调用结构）；语义命名需 ui-input 域 | 章节已升级：closure$1::closure$0=曲线采样（FUN_140c97e10, 0.5 参数）、其余=状态转换（OutlineWallCmd/PublicWalls 相关）——仍 I | **部分推进**（粗粒度语义已登记）；细粒度状态转换语义仍 U，ui-input 域未接手 |
| **photo_mode↔CameraMode 判别映射** | 结构闭合；0..3→6 子索引/槽 7..12 的写入点 ⛔（toolbar::photomode，crate 不在 skeleton） | camera-time DIGEST 同卡点（"需 ui-input 域联动"）；无新证据 | **未闭合（⛔ 双向挂起）**；可交付面已足（next/next_demo 判别、PhotomodeState 3.0/1.0、6 姿态 body+fov 对 O） |
| **EscMenuState 变体名** | 结构闭合（8B {disc,sub}；is_open ∈{3,4}∨{7,1}）；变体名 ⛔（toolbar/tiny_glade crate 不在 skeleton） | **O-AUDIT #16 新证据**：PrevEscMenuState::get 0x14093fbb0 机器码 `cmp dword[rcx],0xb; cmovne` = 判别 **0xb→None**（0xb 哨兵新增入判别空间 {2,3,4,7,0xb}） | 结构+1 判别值；**变体名仍 ⛔** |
| **Events fork 双布局** | I 待定：`{event@0,id@8}`（mouse_capture）与 `{id@0,event@8}`（translate_input_to_cursor_moved）并存；grow Location 仍 bevy event/collections.rs；需中央引擎层裁决 | round3 无裁决；input 域 a6a412d5 另报 MainActionSet 双形态（运行时 0x170B vs serde 0x58B 字段序相反待裁决）——相关但独立 | **未闭合**（engine 层待裁决；与 MainActionSet 问题可合并排队） |
| **MusicPlayer** | CLOSED（CLOSURE #1 O：A/B 双表各 4 状态 + 节拍同步 0x143634c68/c90&3 + switch_track + 公共尾） | 无新证据推翻 | **已闭合（状态机级）**；残留：A1/A2/B1/B2 阈值常量语义、0x1408b5b52 后精确动作（U，不影响结构） |

---

## 3. 无结论行（SOURCE_MAP 业务函数 vs 章节覆盖）

- 三域 SOURCE_MAP 业务函数（kind=business 且带 VA）共 **575** 条，**全部**被章节文本地址级覆盖；**0 条** business 函数缺 VA；0 条完全无章节提及。
- 仅 **6 行**只有地址提及、无名称级结论：`input/action_set.rs`、`input/input_stream.rs` 的 serialize/deserialize/visit_str 各 3 条——全部是 serde derive 胶水（按项目规则复用 derive、不计独立业务）。
- **分类口径问题**（非缺口）：这 6 条在 SOURCE_MAP 中仍标为 kind="business"，与 AGENTS.md「serde visitor 直接复用 derive 不计业务」规则不符，建议 SOURCE_MAP 改标 glue（如 round3 v2 表对 terrain serde 单态的做法）。
- 结论质量分层：多数行有"地址+断言+证据文件"；少量行仅"地址+一句话"（如 debug_viz 两行"原字节短函数"、rocky_terrain load）——抽查证实方向正确但证据链薄，属补强级非错误级。

---

## 4. 算法未深挖：算法级 vs 结论级

| 主题 | 已闭合面 | 未深挖面 | 类别 |
|---|---|---|---|
| **WorldRaster 全链** | new/subscribe/clear×2/copy×2/rasterize_stroke/copy_to_staging/write/would_write_succeed/upload_to_staging 全 O（布局+行为+6 槽名序+serde） | ① +0x29b 置位动态路径（U，tabloid 依赖边界）；② +0x48..+0x5f 次级 Vec 用途（U）；③ 订阅记录消费方（I：tabloid publish 闭包换装，依赖边界）；④ WaterMask/GardenMask/RippleSource Ssbo 字段（U，name-only） | **结论级已闭合**；4 个 U 项全在依赖边界/name-only，不复原成本>收益 |
| **笔刷状态机** | ActiveRasterStroke 0x470B 布局 + start_new/add_point/solidify/resample_and_smooth（10 遍 0.25/0.5/0.25 平滑）O；draw_raster 状态机 O；弧长>1.0 触发固化 | ui_terrain_focus_mode 内部状态机（U）；+0x00/+0x08 字段名（O 行为、名 U）；solidify 间隔权重（O：0.5 定距） | **算法级已闭合（笔画列）**；**结论级缺口（focus_mode 状态命名）** |
| **ui_edit_focus 24 变体** | 4 张旋转跳表=同一 24 项表（idx 0..23→函数体内 0x14097745f..0x977e90）结构 O；24 个变体 serde 名 O（GeoSource 族：DecoratorCollision/…/UnsupportedStairStructural 恰 24 个） | **跳表 index→变体名映射 U**：每个 index 的目标块语义未逐个定名（match 入口 4 处共用同一表） | **结构级闭合**；逐项语义属**算法级未深挖**（可逐个手解 24 个目标块定名，成本中等） |
| **check_if_can_switch** | 尾段=unreachable!() panic（toolbar.rs line 40）；mode id=10+5k×类别视觉序；15 闭包两段式；env_0=timelapse 元命令入队；force-push 0x20B 记录 | env_4/env_5 动态类别变体细节（U）；mode 0/5 同形块语义名（U） | **算法级已闭合**（round2 CLOSURE #1） |
| **MusicPlayer** | 状态机 4 状态 O（A/B 双表、节拍同步、switch_track、公共尾）；get_timeline_beat_properties=6 进程全局发布器 O | A1/A2/B1/B2 阈值常量语义；0x1408b5b52 后精确动作 | **算法级已闭合**；常量级残留（U） |
| **Steam 动作映射** | 28+4 动作名全表 O（GameControls/Menu/CottageWindow/GothicWindow）；update_steam_inputs 14 同构块归纳 O（输入码 0x200..0xf00、簿记 +0x170/+0x178、2B 环形队列） | **码→动作名配对 U**（需 ui-input 域游戏输入枚举定义，跨域依赖） | **结构级已闭合**；配对为**结论级缺口（跨域）** |

---

## 5. 证据链抽查（独立 10 行，seed=42，避让 oa2/O-AUDIT 已抽地址）

取样：8 terrain-water + 2 ui-input（audio-misc 章节 O 行 17 条中 15 条已被 oa2/O-AUDIT 覆盖）。核验链：章节行 → 域 decomp 文件 → callee/常量/哨兵核对。

| # | 函数 | VA | 判定 | 证据要点 |
|---|---|---|---|---|
| 1 | RockyTerrainMesh::load | 0x1408bbf50 | 一致 | callee 集=read_to_string(FUN_140d59140)+serde 解析+SSBO add_new_with_data 族，与"fs→serde→add_new_with_data"吻合 |
| 2 | InProgressRasterStroke::solidify_fluid_into | 0x1408691b0 | 一致 | FUN_141467fa0(0.5 定距重采样)+FUN_141308760(迭代器视图)+FUN_1412f9e10(尾点追加)+FUN_1428a9a60(Vec 扩容) 全命中 |
| 3 | brush_preview impl$6::register_required_components | 0x140866800 | 一致（胶水） | callee=QueuedRegistration 族（与 O-AUDIT #9 同模式），按规则不计业务 |
| 4 | terrain_edit deserialize\<Value> | 0x1417c1ac0 | 一致（入口） | round3 v2 表按规则不反编译 serde 单态；"O(入口)"标记诚实 |
| 5 | world_raster_defs init_state | 0x14083fe20 | 一致 | 6× get_resource 调用序列（FUN_141372610/1413a6cc0/141398530/1413a68d0/1413ab8b0/141376630）写 param_1[0..5]，与"注册 6 个栅格资源"精确吻合 |
| 6 | TerrainHeightsData::sample_at_world_xz | 0x1409e9640 | 一致 | 0x1ff clamp×2、+0x201 debug 偏移路径（`(…)+0x201)*2`）命中 |
| 7 | read_mouse_depth_value_from_gpu | 0x140a1a480 | 一致 | 哨兵 `[+0x40]==-0x7ffffffffffffffd`(i64::MIN+3)、SSBO 名长 0x47+memcmp、NaN 过滤 `&0x7fffffff<0x7f800000`×5 全部命中 |
| 8 | mouse_terrain_raycast::camera_mouse_ray | 0x1408ef1c0 | 一致 | 光标 pos 读取→射线字段写入（+0/+0x10）+退化长度 SQRT≤0 早退，与"构建世界射线"吻合 |
| 9 | ui_clutter history_record_auto_clutter | 0x1409a9160 | 一致 | 真实函数（非占位），decomp 落盘 ui-input/decomp/ |
| 10 | debug_visualize_roof_uvs | 0x141454e60 | 一致 | wp15/analysis/b7_signifiers/debug_visualize_roof_uvs_0x141454e60.c 实存（356 行真实反编译）；"2 callers"数量未独立复核（低风险） |

**结果：10/10 一致，0 不一致，0 证据不足。** 与 round3 O-AUDIT（32/40=80%，实质独立错误点 5）一致向好；本组抽样未发现新错误，但 4/6/9/10 的结论均为"结构级/胶水级"而非全算法复推——与 §4 分类一致。

---

## 6. 未探究项汇总（按建议优先级）

| # | 未探究项 | 类别 | 缺口 | 建议 |
|---|---|---|---|---|
| 1 | **ui_edit_focus 24 变体 index→名称映射** | 算法级未深挖 | 4 跳表结构 O、24 serde 名 O，index↔名未对齐 | ui-input 域下轮逐个手解 24 个目标块（0x14097745f..0x977e90），成本中等收益高 |
| 2 | **into_drag_state 0x14092df10** | 结论错误残留 | 章节仍写"折叠（NotFound）"；源码返回 None 与机器码默认 2 不符；语义（is_started 判别）未证实 | 章节按 w6 机器码重写；源码对齐默认判别；用 DragState::is_started@0x14092e6c0 判别链钉变体序 |
| 3 | **photo_mode↔CameraMode 判别映射** | 跨域卡点 | 0..3→槽 6..12 写入点在 toolbar::photomode（crate 不在 skeleton） | 保持双向挂起；或按 PDB 注解定位 toolbar crate 残余符号表条目 |
| 4 | **Steam 输入码→动作名配对** | 跨域结论缺口 | 14 码 0x200..0xf00 已知，配对需 ui-input 游戏输入枚举 | ui-input 域把 InputEvent 枚举恢复后回填（跨域交换） |
| 5 | **Events fork 双布局 + MainActionSet 双形态** | 引擎层裁决 | 两处布局并存证据充分、裁决缺位 | 中央/集成 owner 一次裁决，两问题合并排队 |
| 6 | **focus_mode 闭包细粒度状态转换** | 结论级缺口 | 粗粒度已登记；进入/退出/高亮的具体状态命名 U | ui-input 域接手（CLOSURE #6 指定归属） |
| 7 | **+0x29b 置位动态路径** | 依赖边界 U | 直接寻址已穷尽排除；tabloid 回调链不复原 | 章节 U 节登记 + 依赖边界标注；不再投入 |
| 8 | **DebugSphere 布局 @+0x14/+0x20 三方不一致** | 证据级小疑点 | w9"Vec3 center 之后" vs 章节 new"center@0x14,alpha@0x20" vs 源码"line_width@0x14,solid@0x18, gap" | 一次反汇编 new 0x140b02490 定夺字段名；另 texture_watch L11 "0x14003bb80 句柄释放对"改注、animation_duration"35 callers"清除 |
| 9 | **MusicPlayer 阈值常量 / 0x1408b5b52 后动作** | 常量级残留 | A1/A2/B1/B2 阈值（0x142ad01cc 区浮点）语义 U | 低优先；状态机已闭合，不阻塞落码 |
| 10 | **SOURCE_MAP 6 条 serde 行分类** | 口径问题 | kind=business 与规则不符 | 改标 glue（与 round3 v2 表一致） |

## 附：本组抽查对既有结论的佐证/修正

- w6 两行"证据不足"（SignifierEntityBuilder::new、move_gizmo_and_trigger_vol）的**依据（wp15/wp20 目录缺失）是错的**——两目录及所引文件均实存；对应章节已另补证并修正，建议 oa2 汇总时作废该两行原始判定。
- 0x14003bb80 簇为三域系统性错误源（9 行×3 域，O-AUDIT 已修）；本组复核 world_raster.rs.md 该址 3 行（apply/new_archetype/queue）本就正确。
- terrain-water 域 O 抽查覆盖率最低（oa2 仅 brush_preview 1 行 + 本组 8 行），建议下轮补抽（候选：rasterize_stroke 三 blend 闭包、copy_to_staging、subscribe、mesh_follow_terrain、draw_garden）。
