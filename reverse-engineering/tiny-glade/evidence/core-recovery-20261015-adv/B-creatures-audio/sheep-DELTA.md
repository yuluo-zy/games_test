# B 组 sheep DELTA（2026-10-16 第三轮，核对用户批改真实性）

> 核对对象：`D:/game/tiny-glade/crates/country-core/src/resources/critters/{sheep.rs, sheep/control.rs, mod.rs}`。
> SPEC：`sheep-SPEC.md`（2026-10-15 第二轮）。本表逐 SPEC 项对照当前源码与原始机码。
> 状态判定：真实实现 / 有水分 / 仍缺 / 待取证。原侧抽查用 `rne-read.py --hex`（B RNE SHA256 f507a6ce…）。
> 注意：sheep.rs/control.rs 在 f510c7e7 未动（git log 只到 fa337fd4/6845e43f），非用户批改本次内容；批改后整体复验。

| SPEC 项 | 当前状态 | 证据（源码位置 / 原地址） | 备注 |
|---|---|---|---|
| §2.2 SheepMoveState 枚举（wire/内存判别） | **真实实现** | `resources/critters/sheep.rs` 整文件；serde 序 Idle/Walking 与 SPEC 串表一致 | 判别@+0 注释准确（idle/walking），Walking 字段内存序 `goal_change_timeout,<f32>,force_reach_goal` 中字段名诚实标待取证 |
| §2.3 CritterGoal（16B Option<Vec3>） | **真实实现** | `resources/critters/mod.rs` `#[serde(transparent)] pub struct CritterGoal(pub Option<Vec3>)` | 与 capture/restore 16B-stride 拷贝一致 |
| §2.4 SheepSnapshot / SheepControlSnapshot | **真实实现** | `resources/critters/sheep/control.rs` L17-27；测试 L70-95 断言 wire `{"sheep":[{"position":...,"goal":...,"move_state":{...}}]}` | 序列化字段序 position/goal/move_state 与 0x1416f9eb0 一致 |
| capture_sheep_snapshot / apply_sheep_snapshot | **真实实现** | `control.rs` L31-64；测试 L97-125 验证 query 序 zip、短输入停、不动 rotation/scale/其它字段 | 与 0x1412f03f0 / 0x1416f24f0 语义一致；函数为纯函数（无 ECS 系统）符合"capture/apply 是 SheepControl 调试资源驱动的一对函数" |
| §2.1 完整 Sheep component owner（stride 0x68、move_state@+0x30、being_petted f32@+0x58） | **仍缺** | 全 crate 无 `struct Sheep`（grep `struct Sheep` 仅 render 侧 `SheepAnimationSsbo`）；`sheep.rs` 只含枚举与 control 子模块 | SPEC"可直接落地项 1"（补 `Sheep` component：move_state+being_petted_timer）**未落地**；`sheep.rs` 头注释诚实声明"不替换为部分/默认 owner" |
| being_petted 谓词（0x14089d4c0，常量 0x142930f74） | **仍缺** | grep `being_petted` 无命中；未实现 `Sheep::being_petted` 或等价谓词 | 常量已复核：rne-read `0x142930f74 = 2f 29 00 00`（f32 0x292f≈1.48e-41，即"正值非零"）——原字节在，谓词没落地 |
| §2.5 真实 Sheep query 所有者（systems/debug/sheep_controls @0x1416f11a0） | **仍缺** | grep `sheep_controls` 无命中（仅 `resources/gui.rs` `show_sheep_controls` UI 勾选项存在） | `(Entity,&mut Transform,&mut Sheep,&mut CritterGoal)` 的 owner 系统不在本 crate；`gui.rs` 的勾选项是注册表项非接线 |
| §2.5 save/load "Sheep" 节 Option\<SheepControlSnapshot\>@0x140a51d40 | **仍缺（接线断）** | `SheepControlSnapshot` 仅被 control.rs 自身引用；`systems/save/save_game.rs` `assemble_game_save_sections` 有 `sheep: SerializedJson` 参数但**无调用方**、无 producer；`resources/save_file.rs` `SaveFileSection::Sheep = 5` + 名称 `"Sheep"` + padding 128 存在 | 节槽位与名称真实；载荷链（`to_json_string<Option<SheepControlSnapshot>>`）未接；save_game.rs 注释诚实"其余节由所属域提供，缺失即 panic，不伪造" |
| §2.6 spawn（startup_sheep 是资产载入非实体 spawn） | **待取证（未做，正确）** | 无 spawn 代码；控制台证据中 `startup_sheep @0x140a093e0` 属资产载入 | 符合 SPEC 结论（实体 spawn 大概率在 system_critters，不在本组）；未伪造 spawn |
| §2.6 movement 状态机 Idle↔Walking | **待取证（未做，正确）** | 无 movement 谓词代码 | SPEC 明令"旧草稿 movement 谓词作废，不要抄"——**源码确实没有抄**，未发现旧草稿残留 |
| §4 T5 register_required_components ×5 | **待取证** | 无 required-component 注册（crate 无 `#[require]`） | 因无 `Sheep` component 故无从 require；保持登记 |
| sheep_audio 消费链（kind 0..7 + Pitch） | **真实实现** | `systems/audio/sheep_audio.rs`；`PlaySheepReactSound{kind,pitch}`（f510c7e7 把误读 `FirstPersonOff` 改 `Pitch`，见 diff） | 7 one-shot 事件名 + `Baah` kind4 loop + `Pitch` 参数；与本 DELTA 的 sheep 组件无关但属消费证据 |
| render SheepAnimationSsbo（后置） | **待取证（后置）** | `systems/render_loop/resources/critters.rs` 0x3e0B TypedStorageBuffer | GPU 侧，明确后置，不计 |

## 合成 agent 应关闭的真实剩余缺口（sheep）

1. **`Sheep` ECS component**（stride 0x68）：落地 `move_state: SheepMoveState`（+0x30）+ `being_petted_timer: f32`（+0x58）+ 布局注释；其余区间（+0x00..0x2f / +0x3c..0x57 / +0x5c..0x67）无证据，勿填占位。
2. **`Sheep::being_petted` 谓词**：`self.being_petted_timer > f32::from_bits(0x292f)`（0x142930f74 已复核）。
3. **save "Sheep" 节载荷接线**：`systems/save/save_game.rs`（C 组）的 `sheep: SerializedJson` 产自 `capture_sheep_snapshot` 的 `Option<SheepControlSnapshot>`（None→null），需中央 agent 仲裁后接通；现为纯接口无消费方。
4. **真实 query owner**（`systems/debug/sheep_controls` @0x1416f11a0）落地或登记——文件在 `systems/debug/`（组外，需仲裁）。
5. 保持：spawn/movement/完整字段取证（T1/T2/T3）仍属待取证，不凭空造。
