# 库内功能完整性核验总账(2026-10-16)

> 依据 eco/state/interaction 三组完整性审计(reports/<组>/completeness-audit.md)。
> 突破"限定"口径:逐函数对账 + 假实现扫描 + 残留分类(库内可闭合 vs 外部依赖)。

## 核心结论

1. **库内无假实现**:三组扫描零 stub/TODO/unimplemented! 冒充;命中的 panic/占位全部裁决为合法文档化路径
   (compute_pass_builder External 守卫、draw_lists/session_selection 忠实复现原版自身 todo!、panic_deprecated 原行为、
   observed_operation 字节保留、audio/weather 文档化占位)。
2. **编译通过(1709/1)不替代库内功能完整性**。
3. **未闭合项全部归类**:库内可闭合(需 fresh 反编译/实现) vs 外部依赖(宿主/render/他域 crate)。
4. **完整 core 验收被外部依赖阻塞,非库内缺失**:域资源收集/组装、全局调度、render/资产接线。

## 逐模块库内完整性 verdict

| 模块 | verdict | 说明 |
|---|---|---|
| garden | ✅ 库内完全闭合 | 四链全验证;pin 渲染/全局调度=外部 |
| audio 域 | ✅ 大部闭合 | 参数/事件名库内可闭合(rne-read);FMOD=外部 |
| decorator 族 | ✅ 大部闭合 | 主体 fresh 定案;u45/子侧公式库内可闭合;实体生成=外部 |
| ivy | ⚠️ 主算法未逐相闭合 | grower 10 相/二级表/spawner 地形格/pruner 空间哈希/send_to_gpu 映射/SsboId 键源=候选近似(证据挖取中) |
| sheep | ⚠️ 半闭合 | 捕获/恢复+serde 全验证;owner/being_petted 边界外(库内可闭合需跨域组件布局) |
| save | ✅ 高 | 编解码/线程/GC/节拍器核心 verified;域组装/宿主接线=外部 |
| session | ✅ 高 | schema/serde/切换链数据结构全核;world 切换调度=外部 |
| music | ✅ 高 | 播放器状态机/参数/适配全核;曲库生产/FMOD=外部 |
| history | ⚠️ 中-高 | EditType 15 表/编辑结构/回放核心 verified;40-op 变体/terrain 机制库内待挖;replay_garden/path/water/terrain=外部 |
| startup | ✅ 入口完整 | register_core_startup/9 文件核;27 缺全外部分类 |
| roofs | ✅ 高 | 主链+视觉+ridge+noise 核;roof_shape 剩余 ~13 谓词库内可闭合 |
| stairs | ✅ 高 | 权威图/结构/视觉/回放核;invalidate 库内可闭合(小) |
| meta_commander | ✅ 高 | 数据结构+分派主体核;sink 载荷=外部 |
| 域类型 | ✅ 中 | 3 存档类型核;GameProfile ron 字段序 FIX;camera_rig 控制器=外部 |
| input | ⚠️ PARTIAL-高 | 2 核心 verified;record_replay/translate_input pending_evidence;动作↔槽/Steam/帧 serde 分类 |
| terrain | ⚠️ PARTIAL-中 | 5 业务+回读链 verified;ui_draw/ui_edit/focus pending;注册 partial |
| ui_clutter | ⚠️ PARTIAL-低 | 历史/删除 verified;decoclutter/ui_focus 门存根;实体机制=外部 |
| camera/time | ⚠️ PARTIAL-中 | slice1/2+纯算法 verified;body 更新 20+/suncalc/光照 missing(大件后置) |
| ui_mode | ⚠️ PARTIAL-中 | AppMode/widget/slider3d verified;ui 顶层/signifier 动画 missing |
| wall | ✅ 既有链 verified | valid_enclosure/snapping/eraser/shapes 已实现(R4+R5);wall_compound 79 登记 |
| collision | ⚠️ PARTIAL | R1 核心查询族 verified;170 胶水/intersect_meshes/visualize missing |
| render_bridge | ⚠️ 消费桥 pending | 宿主顶层渲染循环=外部 |

## 库内可闭合缺口(已派实现轮 2026-10-16)

- **ivy 主算法**(re-analyst-eco 证据挖取中→证据后派 restorer 逐相实现)
- **history 40-op 变体并入 + serde/tag 对齐 + terrain 机制**(restorer-state)
- **save extend_on_load tag=3 + SaveBackupList 计数器 + game_profile 字段序**(restorer-state)
- **roofs roof_shape 剩余谓词 + stairs invalidate**(restorer-roofs-stairs)
- **audio 参数/事件名**(clutter param/sheep Pitch/firstperson WALK/weather/ridge 常量,restorer-audio)
- **decorator 子侧公式 + u45 消费端 + contains_shape**(restorer-decorator)
- **interaction 残余**(input 动作↔槽/帧 serde、terrain focus/ui_draw、camera body/suncalc、ui 顶层/signifier 动画——restorer-interaction2 专项)

## 外部依赖(宿主/render/他域,验收层前置)

GameSaveData 域组装、snapshot 域序列化器、commander_update sink 应用、GladeAudio 生产、27 startup 缺、
replay_garden/path/water/terrain、视觉 RtWorld/mesh、实体生成(nani)、GPU 提交、Steam 全链、camera_rig 控制器、
DecoratorStorage 实体生成、宿主顶层渲染循环、全局调度。
