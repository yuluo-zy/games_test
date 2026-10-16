# 库内完整性审计 — eco 组（reviewer-eco，2026-10-16）

突破"限定"口径，逐函数核验生态组各模块库内功能完整性。方法：inventory 枚举全部原业务函数 →
与当前 src 一一对照 → 逐函数裁决（implemented_verified / implemented_pending_evidence /
glue_legitimate / missing_in_library）→ 假实现扫描 → 残留分类（库内可闭合 vs 外部依赖）。
编译基线：cargo test --lib **1709/1**（唯一败 geometry/sphere 他组）。

## 0. 假实现扫描 — 通过

- 全 eco 模块（resources/ivy、systems/ivy、systems/garden、resources/critters、audio_resources、
  audio_instance、systems/audio/*、decorator*.rs、cached_decorator_transforms.rs）grep
  `unimplemented!()/not implemented/todo!/stub/FIXME`：**无隐藏假实现**。
- 命中的占位均为**文档化**：firstperson_steps WALK_LOOP 占位（待 host reconcile）、
  playermade_stairs is_stub_preview（注释）、wall_audio_orchestrator material_param（测试断言占位）、
  weather 季节参数（待取证）、ivy_audio_init_system（log 占位，instantiate 委托 host）、
  decorator_wall_change_pending（空接线点）、take_snapshot（返回 default，capture 待接线）。
- 协调者所指 `world_raster.rs:183 panic!("not implemented")` 属 interaction/terrain 组，非 eco 模块。

## 1. 逐模块函数对账 + 库内完整性判定

### ivy（13 原文件全在位）

| 原函数族 | 数量 | 裁决 | 说明 |
|---|---|---|---|
| ivy_storage（Ivy::add_point/last_point/last_segment_point_ids/spawn_ivy + 12 内联） | 20 | **implemented_verified** | 逐一对照 decomp（add_point 40B 布局/spawn_ivy RNG 序/spawn 断言） |
| ivy_leaf_storage（add + 5 内联） | 7 | **implemented_verified** | add 104B 布局对 decomp |
| ivy_direction_proposer（get_direction + new） | 3 | **implemented_verified** | 常量/seed/±80° 全对 |
| wall_coord（into_world_space + from_wall_u） | 3 | **implemented_verified** | FIX-1（y=v±offset_z）已修 |
| ivy.rs closest_segment_index | 1 | **implemented_verified** | 过滤1 方向 RNE ucomiss 定案 |
| ivy_grower（业务 4：grower/closest/get_wall_double_sided_normal/intersect） | 4 | grower=**implemented_pending_evidence**（10 相结构在，主闭包逐相语义/二级表 0x142ab3bf4 待取证）；get_wall_double_sided_normal/intersect=**implemented_pending_evidence**（近似） | advance_growth 完成路径 verified |
| ivy_spawner（业务 1 + 3 内联） | 5 | **implemented_pending_evidence** | 门/常量 verified；地形格采样近似 |
| ivy_pruner（业务 4） | 4 | cmd/remove=**implemented_verified**；ivy_pruner/is_point_still_valid=**implemented_pending_evidence**（空间哈希近似） | |
| leaf_spawner / leaf_animation / branch_thickness | 3 | **implemented_verified** | 逐一机器码核对（缺 ivy→日志非 panic 纠错） |
| send_to_gpu_leaf / segments | 8 | **implemented_pending_evidence** | collect+pack+gpu_write 到位；48B↔104B 字段映射待取证；SsboId 键来源非名字哈希（E5 定案） |
| mod.rs（add_ivy_systems/insert_ivy_resources/add_ivy_events） | 3 | **implemented_verified** | 22 系统注册 7/22（缺 GPU 上传+ivy_audio 等）→ glue_legitimate 边界 |
| **库内判定** | | **库内未完全闭合** | grower 主生长算法（逐相）、spawner 随机根地形格、pruner 空间哈希、send_to_gpu 48B 字段映射、ivy SsboId 名字来源 = **库内可闭合**（需 fresh 反编译/实现）；GPU 提交=外部 render |

### garden（eco 批范围 draw_garden + record_garden_history + build）

| 原函数 | 裁决 |
|---|---|
| draw_garden（业务 1） | **implemented_verified**（模式/笔刷/UI/边界/绘制/状态机全对 decomp 0x140ae4620） |
| record_garden_history（业务 1） | **implemented_verified**（slot5 历史记录对 0x141695e10） |
| systems/garden.rs build（业务 1） | **implemented_verified**（mod.rs build() 等价注册） |
| garden_audio/replay_garden/compute_garden/garden_mask | 他组（audio/history/render）范围，非 eco 批 |
| **库内判定** | **库内闭合**（四链全验证；remaining=pin 渲染/全局调度=外部） |

### sheep（eco 批范围 sheep.rs + control.rs）

| 原函数 | 裁决 |
|---|---|
| SheepMoveState serde（business 多为 serde glue） | **implemented_verified**（0x1413ff520+rne-read 字段名/序全对） |
| control.rs serialize/capture（3 业务 + 1 内联） | **implemented_verified**（0x1416f9eb0 顺序 + 字段名定案） |
| Sheep::being_petted（业务 1） | **missing_in_library**（Sheep 组件族未建，按 DISPATCH"不复制空壳"边界） |
| register_required_components ×5 | glue_legitimate（Bevy 注册，未建组件=边界） |
| startup_sheep / sheep_audio / sheep_controls | 他组（startup/audio/debug）范围 |
| **库内判定** | **库内半闭合**：捕获/恢复+serde 闭合；being_petted/Sheep owner=**库内可闭合**（需全组件布局，systems/critters 跨域）但按中央裁决边界外；存档节接入=save 组 |

### audio 域（37 原文件，eco 批覆盖 33 + music 3 属 state 组）

| 原函数 | 裁决 |
|---|---|
| audio_resources（19 业务 + 20 内联） | 主闭包=**implemented_verified**；instantiate_audio/AudioSamplesLoadedPromise/DelayedBankLoader=**implemented_pending_evidence**（crossbeam/通道待取证）；report 注册表（fmod_to_anyhow）=**implemented_pending_evidence** |
| audio_update（2）+ ivy_audio（1） | **implemented_verified**（5 VCA/听者/Baah 循环） |
| 31 系统文件（clutter/terrain/garden/sheep/…） | 结构=**implemented_verified**（O 结构）；参数/事件名（clutter param/sheep Baah 值/firstperson WALK/weather/ridge 常量）=**implemented_pending_evidence**（库内可闭合 rne-read） |
| music（music_player/music_system/debug_gui） | 属 state 组，不在本审计 |
| **库内判定** | **库内大部闭合**：FMOD 绑定=外部（libfmod 注入 trait）；待取证参数/事件名多为**库内可闭合**（rne-read/反编译）；跨域类型（TreeAmbience/水栅格采样）=外部/他组 |

### decorator 族（9 原文件）

| 原函数 | 裁决 |
|---|---|
| decorator.rs（56 业务） | 访问器/谓词/From=**implemented_verified**；u45/ExtraInfo 载荷字段语义=**implemented_pending_evidence**（库内可闭合） |
| decorator_storage.rs（60 业务） | 增删查/迁移/快照=**implemented_verified**（fresh 0x140a1f590/0x14155afe0 定案）；子侧状态字节公式/侧2-3 panic=**implemented_pending_evidence** |
| decorator_subtype.rs（20 业务） | 表插查/谓词=**implemented_verified**；dims_wall_space 等=**implemented_pending_evidence** |
| cached_decorator_transforms.rs（7 业务） | **implemented_verified**（10-f32 Quat 纠偏/三批有限性/双缓冲） |
| decorator_archivist.rs（22 业务） | re-export recovered_building（边界，backing 类型差异登记） |
| decorator_edit/decorator_collision/decorator_outline | history/render 组范围 |
| **库内判定** | **库内大部闭合**：卡5+A1+R2 主体 verified；待取证（u45/contains_shape/子侧公式）库内可闭合；实体生成/archivist backing=外部 |

## 2. 残留分类汇总

**库内可闭合（需 fresh 反编译/实现）**：
- ivy：grower 主闭包逐相语义/二级表、spawner 地形格采样、pruner 空间哈希/附加采样点、
  send_to_gpu 48B↔104B 映射、ivy SsboId 名字来源、max_ivy_grow_height 数值。
- sheep：Sheep 组件完整布局（being_petted）+ 组件注册（按中央裁决边界外）。
- audio：clutter/sheep/firstperson/weather/roof 等参数/事件名 rne-read、report 注册表。
- decorator：u45/contains_shape/子侧公式、+0x284/+0x29e 字段名。

**外部依赖（宿主/render/其他 crate/他组）**：
- GPU 提交（render 域）、装饰实体生成（render nani）、pin 渲染/scatter/audio（garden）、
  FMOD 绑定（libfmod 库外）、TreeAmbience/水地形栅格采样（跨域类型）、archivist backing
  （recovered_building）、存档 Sheep 节（save 组）、wall_compound_edit split/merge 拓扑（卡A2）。

## 3. 总体结论

- **库内完全闭合**：garden（四链全验证）。
- **库内大部闭合（待取证项库内可闭合，不阻塞主体）**：audio 域、decorator 族。
- **库内未完全闭合（主算法为近似，需库内深挖）**：**ivy**——grower 主生长算法（10 相逐相）、
  spawner 随机根、pruner 空间哈希、send_to_gpu 字段映射为候选链近似/待取证，非原算法逐相实现。
- **库内半闭合（边界明确）**：sheep（捕获/恢复闭合，being_petted/Sheep owner 按中央裁决边界外）。
- 无隐藏假实现；所有占位/近似均文档化。编译通过不替代库内功能完整性。
