# REVIEW-REBUILD-AUDIO — systems/audio 重建复验（reviewer-eco，2026-10-17）

复核对象：rebuild-audio（REBUILD.md，23 分类全真业务、21 重建）。方法：与 fresh 反编译/反汇编
逐函数判等。实测：cargo check 过（13 既有 warning，0 error）；lib test 目标被 56 个既有无关注册
期错误（rhapsody GraphicsDevice 等）阻塞，音频模块无新增编译错误。

## 逐项核对（协调者 5 点）

### 1. wall_audio_ui 全表 — ✅ 判等

kind0=全局 `StructureSize`(value@+0xc)+`Bulldoze`；kind1=默认 `WallSnapToEnd`；kind6=
ColorBricks+`isHalfTimbered`(selector≠0→1.0)；kind9=`Flags/ToggleDisconnect|Connect`(selector)；
kind0xd=ColorLantern+`LanternColor`；kind0xe=Lantern/TurnOn+`BrightnessValue`；kind0x10=
ChangeWallStyle+`wall_style`=表{0,1,2}[selector]；0x11/0x12=WallSplit；事件布局
{kind@+8, selector@+9, value@+0xc, position@+0x10}——逐条对跳表 0x142a9fa44/值表 0x142a9fa08 ✓。

### 2. AudioInstance Drop / AudioCoreInner 错误语义 — ✅ 判等

- drop 0x14089a280：仅 state@+0x28≠2 才 `stop()`（失败不 unwrap）+ `release().unwrap()`；
  `stop_on_drop` 置 state=0（一次性音不被截断）✓。
- `update` 0x140a53a00：三断言（|fwd|-1/|up|-1/|fwd·up|<0.001 panic）+ set_listener_attributes
  失败 unwrap + Studio::update 失败 log 后继续 ✓。
- `set_parameter` 0x140a53db0：失败 log "Couldn't set global FMOD parameter: {name} ({err})"
  + 返回 Err（BTreeMap 全局报告槽 0x143634d60 库外诊断登记不建模）✓。
- `set_vca_volume` 0x140a54810：get_vca 失败 log+继续、Vca::set_volume 失败 unwrap ✓。
- `Bank::load_sample_data`：首断言 loaded（未加载 panic）+ load_sample_data 改 Result ✓。

### 3. water/tree/terrain/trigger/path — ✅ 判等

- water：init 4 loop 带 demo 门（DemoModeEnabled==0）；两 AppMode 分支均**无条件 stop 两个 ice
  loop** ✓。tree：按最近树仅驱动 3 loop（indices 1..3，TreeMove index0 残留登记）✓。
  terrain：每个 start loop 补 `set_parameter("editing_speed", cursor.speed/60)`（native +0x1c）✓。
  trigger：`+0x58 speed==0` 早退门 + Camera Tree Rustle/Whoosh 双瞬发 + wind 音量
  (len/speed)×0.0125 lerp(0.1/0.2) clamp≤2 + InForest 门 ✓。path：无事件 stop 全部 loop ✓。

### 4. terrain_stairs/building/roof/timberframe/等 — ✅ 判等

- terrain_stairs：**SSBO StairDrawMask 栅格值驱动**（比较存储值：相同→stop ring+return；
  不同→OneShot+ring start）；`SuppressPathSound` 改**输出**（ring 播完写），删自造输入耦合 ✓。
- building：**loop kinds {0,1,2,3,4,9,5} 事件驱动 7 loop**（有事件 request_play+params，无
  frame_end stop）；kind6/7/8 one-shot 保留；BuildingSize=inverse_lerp(area) 待接线 ✓。
- roof：空壳实装（acc[0/1]+=+0x58 speed、kind0/1 acc>0.2 one-shot、kind2 RoofShape loop、
  kind3 RoofTipMove、kind9 InterRoofMerlonsRearrange、kind4/6 参数化 one-shot、帧尾 3 loop 驱动）✓。
- timberframe/water_ambience（删自造 water_level 阈值）/signifier（tag 0/1 FirstPersonOff|On
  瞬发）/wall_path（空壳实装 WallPathDraw）/orchestrator（RequestWallMaterialAudio→材质参数族）/
  sheep kind5-7（Idea/Eyes/Confused，kind7 非忽略）/wall_fmod（**powf(0.1, speed×5.0)** 帧尾
  音量）✓。

### 5. trait 面变更 — ✅ 接受（我裁决）

`Studio` 拆 `set_listener_attributes`+`update()`、`get_vca`+`type Vca: VcaHandle`、
`load_sample_data`→Result；`EventInstance` 增 `release()`（默认 Ok，真实 binding 覆写）；
`PlayWallUiSound` 字段改 {kind, selector@+9, value@+0xc, position}。
**归属裁决**：库内接口面变更（音频域自有 trait + 事件），crate 内消费/测试同步更新无破损；
外部 libfmod 绑定（库外 render/audio host）需实现新方法（set_listener_attributes/get_vca/Vca::
set_volume/release/load_sample_data Result）——登记为集成接线项，接受。

## 残留（诚实登记，不掩盖）

sheep 0x1b..0x51 大分派（另一事件流待定向补挖）、decorator kind→loop 跳表逐条对账、
timberframe one-shot 事件名与长度不匹配（list 占位）、signifier 21 参数全表、wall_path
DecoratorId 收集语义、orchestrator wall 域查表/simplified_curve 断言、roof 参数名对账、
water_ambience 衰减常量、firstperson SSBO 位栅格细化。

## 结论与 DISPATCH 建议

- **audio 重建项 = 闭合**：21 重建函数 + wall_audio_ui 全表 + trait 面变更均与 fresh 反编译/
  反汇编判等；cargo check 过。
- **登记项（不阻塞）**：残留 8 项（sheep 大分派等）+ trait 新方法的外部 binding 实现。
- 建议：REBUILD-AUDIO=review_passed（限定）；sheep 0x1b..0x51 大分派为最高优先补挖项。
