# REBUILD.md — systems/audio 真实逻辑重建（rebuild-audio，2026-10-17）

> 协议：`planning/2026-10-16-logic-verify-protocol.md` §「重建前必做：宏生成函数 vs 真实业务函数 分类」。
> 对 LOGIC.md §二 `未实现-需重做` 的每个函数：先分类，再按 fresh 反编译/反汇编重建控制流，逐语句实现。
> 证据：复用 logic-verifier 的 fresh decomp（`logic/decomp/*`）+ 本轮 fresh ghidra
> （`logic/decomp/rebuild/0x140aa3860|0x140aa2fd0|0x140aa3360|0x140aa3590`）+ fresh radare2
> 于 RNE（wall_audio_ui 续段 0x1408b7b40+）+ fresh ghidra read_memory
> （wall_style 值表 0x142a9fa08、timberframe 串区 0x2ad34aa+）。未修改他模块。

## 一、分类结论（每函数：胶水 vs 真业务）

LOGIC §二 全部 23 个函数均为**真实业务**（有真实算法/控制流/调用链），无宏生成胶水：
derive Debug/serde/BeVy 注册等不在此列。无 `glue-legitimate` 标记项。逐个列于 §二。

## 二、重建详情（函数级）

| VA | 函数 | 分类 | 重建（fresh 依据） | verdict |
|---|---|---|---|---|
| 0x1408b7690 | `wall_audio_ui` | 真业务 | **全表重建**（disasm 0x1408b7690 + 续段 + 跳表 0x142a9fa44 + 值表 0x142a9fa08）：kind0 = 全局 `StructureSize` 参数(value@+0xc) + `Bulldoze`；kind1 = 默认 `WallSnapToEnd`；kind9 = `Flags/ToggleDisconnect\|Connect`(byte@+9)；0x11/0x12 = `WallSplit`(默认目标)；kind6 = ColorBricks+`isHalfTimbered`；0xd = ColorLantern+`LanternColor`；0xe = Lantern/TurnOn+`BrightnessValue`；0x10 = ChangeWallStyle+`wall_style`=表{0,1,2}[selector]；事件布局补 `selector@+9`/`value@+0xc`。修正 Rust 三处错（参数名/事件族/跳过）。 | 重建 |
| 0x14089a280 + 0x140a57540 | `AudioInstance::drop` + `stop_on_drop` | 真业务 | drop 0x14089a280：仅 state@+0x28≠2 才 `stop()`（失败 drop 不 unwrap）+ `release().unwrap()`；补 `stop_on_drop`（置 state=0）。修一次性音被截断。 | 重建 |
| 0x140a53a00 | `AudioCoreInner::update` | 真业务 | 前断言 `\|fwd\|−1<0.001`、`\|up\|−1<0.001`、`\|fwd·up\|<0.001`（panic）；`set_listener_attributes` 失败 unwrap；`Studio::update` 失败 log "FMOD update error: …" 后继续。trait 拆 `set_listener_attributes`+`update`。 | 重建 |
| 0x140a53db0 | `AudioCoreInner::set_parameter` | 真业务 | 失败路径 log "Couldn't set global FMOD parameter: {name} ({err})"（native 0x142ad2a10）+ 返回 Err 供观察；BTreeMap 全局报告槽 0x143634d60 为库外诊断，登记不建模。 | 重建 |
| 0x140a54810 | `AudioCoreInner::set_vca_volume` | 真业务 | get_vca 失败→log+继续；`Vca::set_volume` 失败→unwrap(panic)。trait 拆 `get_vca`+`VcaHandle`。 | 重建 |
| 0x140a53870 | `Bank::load_sample_data` | 真业务 | 首断言 `loaded`（未加载 unwrap_failed）；trait `load_sample_data` 改返回 Result + unwrap。 | 重建 |
| 0x140873630 | `water_audio_system` | 真业务 | 两 AppMode 分支等价：驱动 draw/erase 两 loop（garden 模板）+ **无条件 stop 两个 ice loop**（4 次 `update_draw_audio` 中的 2 次 stop）。 | 重建 |
| 0x140873140 | `water_audio_init_system` | 真业务 | `_DEV_OVERRIDE_IS_DEMO_MODE==0` 门（`DemoModeEnabled` 资源）才建 4 loop。 | 重建 |
| 0x1414638e0 | `tree_audio_system` | 真业务 | 按最近树仅驱动 **3 个 loop**（state+0x38/+0x70/+0xa8 = indices 1..3）；TreeMove(index 0) 的事件-kind 驱动残留登记。 | 重建 |
| 0x1409eafa0 | `trigger_audio_system` | 真业务 | `+0x58 speed==0(非 NaN)` 早退门；命中触发 Camera Tree Rustle/Camera Whoosh；wind 音量 = `(len/speed)×0.0125`，lerp(0.1/0.2) + clamp≤2；InForest z≥11/dist≤70 门。 | 重建 |
| 0x1409d5ea0 | `sheep_audio_system` | 真业务 | kind 0..3→Aww/Think/Sob/Angry、4→Baah loop、5..7→Idea/Eyes/Confused（kind7 非忽略）。**0x1b..0x51 大分派为另一事件流 待补挖**（LOGIC 同判）。 | 部分重建 |
| 0x1414618c0 | `terrain_audio_system` | 真业务 | 每个 start 的 loop 补 `set_parameter("editing_speed", cursor.speed/60)`（native SystemParam@+0x1c）。variant→loop 跳表 0x142c63274 逐条对账残留登记。 | 重建 |
| 0x1409d0f80 | `path_audio_system` | 真业务 | 栅格 gate + 无事件时 stop 全部 loop（native 4×stop）；有事件驱动模板。native 4+1 次调用与 loop 范围实参级映射 待对账。 | 重建(映射待对账) |
| 0x140a889b0 | `firstperson_steps` | 真业务 | FirstPerson 参数改读相机（非硬编码 1.0）；ground==3(Terrain) 且移动门；水/园栅格采样定材质→GroundType；ground==0 才走 loop。SSBO 位栅格细化 待接线。 | 重建(SSBO 待接线) |
| 0x141462dc0 | `terrain_stairs_audio_system` | 真业务 | **SSBO StairDrawMask 栅格值驱动**（比较存储值：相同→stop ring+return；不同→建 OneShot+push+ring start）；`SuppressPathSound` 改为**输出**（ring 实例播完时写）。删自造输入耦合。 | 重建(SsboType 待接线) |
| 0x140a89bb0 | `wall_audio_path` | 真业务 | 空壳实装：定义 `WallPathDraw` 事件，驱动 Wall Draw loop(request_play+frame_end)。native 的 DecoratorId 收集语义 待对账。 | 重建(producer 待接线) |
| 0x141234bf0 | `wall_audio_orchestrator` | 真业务 | 空循环实装：RequestWallMaterialAudio → 按 kind 映射材质参数族 `{WallHeight,FenceLength,wall_style,DrawBend}` 设到 Wall Draw loop 实例。GlobalId 查表/`simplified_curve_2d.is_valid()` 断言属 wall 域 待接线。 | 重建(wall 表待接线) |
| 0x140a67b90 | `timberframe_audio` | 真业务 | kind 0→Length loop、1→Width、3→Move（decomp 0x142ad558c 跳表）；2/4/5/6/7→one-shot（native 长度 0x1f/0x1c/0x1f/0x1a/0x22）。**one-shot 事件名与长度不匹配 待对账**（现用 list 为占位）。 | 重建(one-shot 名待对账) |
| 0x14145dc20 | `decorator_audio_system` | 真业务 | kind→loop 表为近似（self 注 partial 维持）；native 收集+距离稳定排序+逐 loop stop/start 对驱动未全验——残留登记，需 0x14145dc20 跳表逐条对账。 | 近似维持 |
| 0x1417d2ad0 | `building_audio` | 真业务 | **loop kinds{0,1,2,3,4,9,5} 事件驱动 7 loop**（不再被 one-shot 过滤丢弃；有事件→request_play+params，无→frame_end stop）；kind6/7/8 one-shot 保留；BuildingSize=inverse_lerp(area) 的 building 表(0x2a8) 待接线，editing_speed 占位。 | 重建 |
| 0x140aa3860 | `roof_audio_system` | 真业务 | 空壳实装（fresh decomp 0x140aa3860）：acc[0/1] += `+0x58` speed；kind0/1 acc>0.2 门 one-shot(RoofAppear/Remove)；kind2 选 RoofShape loop + 2 全局参数；kind3 臂 RoofTipMove；kind9 臂 InterRoofMerlonsRearrange；kind4/6 参数化 one-shot(RoofTilesLand/MerlonsAppear)；kind5/7/0xa/0xb 普通 one-shot；帧尾 3 loop 驱动。 | 重建(param 名待对账) |
| 0x1409ecea0 | `water_ambience_audio_system` | 真业务 | demo 门；cell 按 "exists" 标志激活（删自造 water_level≥0.5 阈值）；逐 active cell `start→set_position(eye.x0y)→set_volume`。0.015625/42.5 衰减常量 待取证。 | 重建 |
| 0x1409d6b90 | `signifier_audio_system` | 真业务 | 记录 tag 0/1 → `event:/UI/Camera/FirstPersonOff\|On` 瞬发（decomp 串拼接）；其余 tag 保留参数面。21 参数全表 待对账。 | 部分重建 |
| 0x140950920 | `wall_audio_fmod`（§三登记项） | 真业务 | 帧尾音量调制 `state.volume *= powf(0.1, speed×5.0)`（decomp 行 564，+0x58 speed）；`inverse_lerp` 改非钳制（native 0x140c904a0 无边，与 garden 一致）。 | 重建 |

## 三、自检

- **`cargo check`（lib）通过**：`Finished dev profile … in 4.70s`，13 warnings（均为既有 save/snapshot
  non_snake_case 等），**0 error**。音频域改动零 error 零新 warning。
- **`cargo test --lib` 不可运行**：测试编译有 **56 个既有错误**，全部位于非音频模块
  （rhapsody_contract `GraphicsDevice`、shape_placement_signifiers `Vec::count`、asset_libraries、
  render 域等，为并行 agent 在途/既有破损）。音频测试模块（audio_resources 的 drop/update/
  load_sample_data、wall_audio_ui 的 kind 表、roof/sheep 等）**无新增编译错误**。
- **trait 面变更（需集成 owner 裁决，FMOD 绑定库外）**：
  - `Studio`：`update(listener)` 拆为 `set_listener_attributes`+`update()`；`set_vca_volume` 拆为
    `get_vca`+`type Vca`（`VcaHandle::set_volume`）；`load_sample_data` 返回 `Result`。
  - `EventInstance`：新增 `release()`（默认 `Ok(())`，真实 libfmod binding 覆写）。
  - `PlayWallUiSound`：字段改 `{kind, selector@+9, value@+0xc, position}`（producer snapping_audio 同步）。

## 四、残留/待对账（诚实登记，不掩盖）

1. **sheep 0x1b..0x51 大分派**：jump table 0x142ac2cc8 的另一事件流，需定向补挖确认事件源（LOGIC §二同判）。
2. **timberframe one-shot 名**：native 长度 0x1f/0x1c/0x1f/0x1a/0x22 与既有 `Timberframe*` 名不匹配，待串表对账。
3. **decorator kind→loop 表**：近似维持，需 0x14145dc20 跳表逐条对账 + 距离衰减曲线。
4. **signifier 21 参数全表**：仅 FirstPersonOff/On 解码，其余 tag→参数映射待对账。
5. **roof / path / wall_audio_orchestrator 参数名与实参映射**：结构已按 decomp 重建，FMOD 参数赋值细节待对账。
6. **firstperson/terrain_stairs SSBO**：SsboType NAME（0x3f/0x45 名断言目标）待接线，现用资源模型承载。
7. **wall_audio_path producer**：native 收集 `(DecoratorId, Vec<PreferredNodeAlignment>)`，事件源待接线。
8. **FMOD 错误语义**：正常路径已判等；native unwrap/报告 vs Rust `?` 的区分按本表逐函数实现（update/set_parameter/set_vca_volume/load_sample_data/drop），其余批量登记维持 LOGIC 判定。

## 五、证据清单（本轮新增）

- `logic/decomp/rebuild/0x140aa3860.txt`（roof_audio_system fresh）、`0x140aa2fd0.txt`（init）、
  `0x140aa3360.txt`、`0x140aa3590.txt`（one-shot helpers）。
- `logic/results/rebuild-wallui-cont.json`（radare2 于 RNE，wall_audio_ui 0x1408b7b40..0x1408b7e8d 续段 240 条）。
- `logic/results/rebuild-wallstyle.json`（ghidra read_memory：0x142a9fa08 = {0.0,1.0,2.0}、0x142a9f9c0 = "wall_style"）。
- `logic/results/rebuild-tf.json`（ghidra read_memory：0x142ad558c 跳表）+ RNE 串搜（Timberframe 事件长度）。
