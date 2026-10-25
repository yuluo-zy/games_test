# 第 11 轮 DEC-D 组「视觉构件」交付

日期：2026-10-19
目标 crate：`D:/game/tiny-glade/crates/systems/decorator`
契约：`evidence/round11/contracts/dec-decorator-CONTRACT.md`
范围：`decorator_visual/**` 中 DEC-D 文件权内文件（未触碰 `stairs*`、`lantern.rs`、
`mod.rs`、`lib.rs`、`ui_systems/**` 及 DEC②/ DEC-B/ DEC-C 文件）。

记号：O = 机器码/反编译直接观察；I = 由调用面推断；U = 证据不足（显式 None/空
结果，不以默认值或伪造几何冒充）。

---

## 1. 逐项状态

### 1.1 旗帜/布绳（`flag.rs` / `flag_connections.rs` / `generate_clothline.rs`）

| 项 | 状态 | 说明 |
|---|---|---|
| `classify_flag_subtype`（0x1407ff6c0 行 483-517） | O | `subtype-0xb<=1` → 不产出；`subtype-0xd<2` → 旗；`==0xf` → 连接；其余返回 None（调用方保持原 fmt panic） |
| 旗布基常量 | **O 纠错** | 0x142a887e0..0xec 实为 `{-0.0,-0.70710677,-0.0,+0.70710677}`（绕 Y −90° 四元数），旧注释 `{0.5,-0.7071,0.7071,0.5}` 错误，已改 |
| `flag_cloth_corners` | O 结构 + U | 基四元数 + 半宽 0.48（0x142a88848）+ 下垂 0.08（0x142a88890）；四角局部槽位语义 U |
| `greedily_connect`（0x1407e1430） | **O 全文结构** | 距离窗：异墙 `d²<10²`、同墙 `2²<d²<5.5²`（调用点实参 10.0/5.5/2.0）；`+0x4d` 源/目标标记；候选对去重；两端容量 `<2`；坡度门 `2*|Δy|<=水平距`；最小距 0.02（0x1429cd2d0）；遮挡回调长度 `dist-0.01`（0x142a82410） |
| `clothline`（0x1407e2510） | **O 公式** | `n=max(2,round(dist/spacing))`；`blend=clamp(sag_len/dist,0,0.25)`（0x142925950）；`t'=t+(1-t)*blend`；下垂 `(1-(2t'-1)²)*0.08*dist`；半宽四角展开（角点归一化顺序 U） |
| 布绳网格名 | O | `decorators/setdressing_clothline_thread`（0x142a88b30，len 0x27）、`decorators/flag_line`（0x142a88b57，len 0x14） |
| `generate_flag_connections` | O 结构 + U | 锚点来自 `try_as_flag` + `CachedDecoratorTransforms::position`；墙侧相邻查询 FUN_1407df050 与 RaycastWorld 遮挡由宿主提供（U） |
| `generate_clothline_visuals` | O 结构 | 真实位置锚点 + 相邻段布绳几何；实体构建（`build<Transient>`）需 Commands/网格库，宿主完成 |

### 1.2 门窗（箭头缝 / 农舍·哥特阳台门 / 农舍·哥特墙窗）

| 项 | 状态 | 说明 |
|---|---|---|
| `arrow_slit_profile_curve` | O 常量 | 剖面 `{0.4,1.7},{0.6049,0.55},{0.4,1.5},{0.14,0.7}`；附加 `{0.14,0.72}` + 2.54；洞尾 `2.5/0.05` |
| `is_arrow_slit` | O（类型族）+ U | 取 DecoratorType 0x3a；精确谓词 U |
| `generate_arrow_slits` | O 结构 | premade 形状 + 洞记录 + 蓝图 colliders/outline + 视觉 builder；位置改读缓存变换 |
| `add_door_autoclutter`（0x1407f5e70） | **O 全文** | 门控 `info.u45>3`；机器哈希 `((x+A)^B)*(x+A)>>30`（A=0xa0761d6478bd642f, B=0xe7037ed1a0b428db）；`h&3==0` → mesh `doorbell`、type 3；`h&3==1` → mesh `krans`、type `seed|4`；0x50B 记录字段 |
| 农舍阳台门判别 | O | 子类型 == 4（0x1407f3da0 行 396） |
| `check_for_door_stairs`（0x1407f3340） | O 公式 + U | `sill=door_h+(top-bottom)*(-0.5)`；地形≤0 分支 `wall_y<=sill-0.2` 无梯；地形>0 分支 `sill<=0.1 || sill-0.2<wall_y`；`0.1<wall_y-second`、`1.5<=wall_y-best` 门控；clearance/dir 输出 |
| 哥特门判别 | O | 子类型 == 9（0x1407f0900 行 347） |
| `add_lintels`（0x1407ef990） | **O 全文** | `(wall_h+1.43)*0.5`；条目高 `entry+0.30+0.36`；0x14B `{handle,wall,height,0.2,0}`；scale 首尾 0.82、中部 0.64；rank≥2 位置加 0.081 旋转偏移；flag bit0 `*= height/0.9`；mesh/shader 名 |
| `flowerbed_xforms` | O（原有保留） | mode/subtype 跳表 + 网格位置 |
| 农舍/哥特窗判别 | O | 0x38 / 0x39 精确类型 |

### 1.3 烟囱/活板门/杂乱

| 项 | 状态 | 说明 |
|---|---|---|
| 屋顶烟囱 | O | 子类型 0x15；高 `extra.f+0.1`；立方体 0.85；nani `nani_smoke`；blueprint.add_chimney |
| 墙烟囱 | O + U | 子类型 {0x13,0x14}；as_chimney 载荷；`is_valid_body_placement` 准则 U（未过滤，显式登记） |
| 活板门 | O | 子类型 {0x05,0x0a}；mesh `decorators/trap_door_cottage|gothic`；nani `nani_balcony_door`；常量 1.3/1.125；变换位置 |
| 杂乱 | O + U | 类型谓词 `{0x38..0x3f}\{0x3e}`；高度 `max(h-1.0,0.1)`；`clutter_mesh_ty_data` 位表 U；Rectangle3d 尺寸 + birdspot |

### 1.4 老虎窗

| 项 | 状态 | 说明 |
|---|---|---|
| `gothic_dormer_profile_curve` | O（原有保留） | rank 1/2/3 点表；新增测试 |
| 哥特老虎窗判别 | O | 子类型 8（0x1407c5650 行 430） |
| 农舍老虎窗判别 | O | 子类型 2（0x1407c7ac0 行 402） |
| `generate_dormer_roofs` 子类型 | O | 位掩码 0x800186（子类型 {1,2,7,8,23}）+ 特殊 0x17；哥特 8 → gothic 装配，其余 cottage |
| `is_out_of_roof_bounds` | O（原有保留） | 模式表 2.1/0.1/0.15、base 0.5/0.1 |
| `DormerKey::quantize` | O | `round(dist*5.0)`；键 `{i32 x,i32 z,i32 count,i8 roof_idx}`；缓存 `get_or_build` |
| `get_collision_mesh` | U | 网格细分数学主体未取证 → 显式空结果，缓存/量化结构已就位 |
| `dormer_roof_height` / `is_bay_window` / `cottage_dormer_dims` / `get_dormer_panel_xforms` | U | 改为 `Option`，未恢复返回 None（不再用 0/空 Vec 冒充） |
| `assemble_gothic_dormer_tiles` | U + 工具 | 瓦列数学主体 U → 空；`halver to_f16` 打包已落地（0x140c8ffb0） |

### 1.5 差分重装（`deco_differential_state.rs` + 两个 assemble 入口）

| 项 | 状态 | 说明 |
|---|---|---|
| `DecoDiff` 96B | O（原有保留） | 代际计数器 + header `{0x4294f6e0,1,0,..}` + defaults `{0x429b0010,1,0,0}` |
| `compare_assembly_params` | O 结构 | 40B `[u32;10]` 逐字段；返回“已变化” |
| `WallAttachedDecoTracker::with_diffs` | O 结构 | 仅新增/变化项回调；返回重装数 |
| `chimney_reassemble` / `door_stairs_reassemble` | O 壳 | 以 tracker 驱动，变化项才进入宿主砖列回调（砖列主体 U） |
| `chimney_assemble_bricks`（0x1407f8db0） | **O 全文** | ResMut 槽位写回 `value=tick` + DerivedDecoratorInfo::new + 差分转发 |
| `door_stairs_assemble_bricks`（0x1407ff130） | **O 全文** | 同构槽位写回 + 差分转发 |

### 1.6 实体维护

`maintain_visual_entities.rs` 上轮已为真实判等实现（瞬态 drain + despawn；墙洞
帧推进），本轮未改。

---

## 2. U 汇总（显式缺口，未用默认值冒充）

1. `greedily_connect` 的墙侧相邻查询 FUN_1407df050 与 RaycastWorld 遮挡（回调由宿主提供）。
2. `clothline` 10 参面 `param_4/param_6` 语义与四角归一化顺序。
3. 箭窗精确类型谓词（取证据族 0x3a）与剖面装配逐点数学。
4. `add_lintels` 12 参面其余语义；窗框 sources 几何。
5. `check_for_door_stairs` 三次 wall-space 采样位置（输入显式承载）。
6. 墙烟囱 `is_valid_body_placement` 准则。
7. `clutter_mesh_ty_data`（0x1408b8e90）子类型位表。
8. 老虎窗碰撞网格细分、`DormerKey` x/z/count 来源、瓦列步进/索引。
9. 差分 96B 记录槽语义、砖列装配主体。
10. 所有 `build`/GPU 侧实体构建（本组只产出蓝图/几何/记录，不伪造 Entity）。

---

## 3. 跨 crate 需求（不改 country-core）

- `CachedDecoratorTransforms::position` 未命中 panic（原语义）；生成系统对 modified
  装饰调用时要求同帧缓存已建立（调度顺序约束）。
- 视觉实体构建需要 `Commands` + `AssetMeshLibrary` + `AssetShaderLibrary`；
  `add_decorator_systems`（lib.rs owner）注册生成族时需补齐这些资源与顺序。
- `DecoratorInfo` 的 `u45` 已用于 autoclutter 门控（O 字段）。

## 4. 编译与测试

- 命令：`cd D:/game/tiny-glade && CARGO_TARGET_DIR=/d/game/tiny-glade/target-agent-decd cargo check -p system-decorator`
- 结果：**通过**（本组文件 0 error / 0 新增 warning）。
- 测试：`cargo test -p system-decorator decorator_visual` → **73 passed, 0 failed**；
  `cargo test -p system-decorator --lib` → **143 passed, 0 failed**。
- 说明：`cargo test -p system-decorator`（含 doctest）仍失败于
  `apply_decorator_op.rs:307` 的伪代码块（非本组文件，DEC② owner 范围：代码块
  缺 ```text 标记导致 rustdoc 按 Rust 编译）。本组未触碰该文件。

## 5. 与其它 DEC 组的隔离

- 未修改：`stairs.rs`、`stairs_assemble.rs`、`stairs_railings_assemble.rs`、
  `lantern.rs`、`mod.rs`、`lib.rs`、`ui_systems/**`、`apply_decorator_op.rs`、
  `backup.rs`、`restore_archived_decorators.rs`、`replay_decorator.rs`、
  `merge_proposal_*`、`decorator_on_*`、`derive_decorator_subtype.rs`、
  `collision_detection.rs`、`chimney_smoke_anim_thickness.rs`、
  `decorator_resources.rs`、`decorator_visual_state.rs`。
- 未修改 country-core / utils / 其他 crate。
