# WP11 — country-core `resources::walls` 状态资源族

Tiny Glade 原作（B）`country_core/resources/walls` **状态类型**恢复（系统属 WP12，
装饰存储/归档属 WP13+）。目标 B `ljxsj_92385`；EXE sha256 `cf048e27…`，
PDB sha256 `2d9686b8…`。

## 交付内容

- `contracts/`
  - `state.md` — PublicWallState / WallState / InnerWalls / PublicWalls / WallStyle /
    StructureType / ArchType / FreehandHeightType / PlasterPeelAmount 家族。
  - `attachment.md` — wall_attachment.rs（WallAttachmentAnchor / WallAttachmentSide /
    WallCornerAttachment）+ decorator.rs 附着/信息类型（S09 backlog）。
  - `shape.md` — WallShape（circle/rectangle/freehand 28B 记录）、WallPathSegment、InProgressWall。
- `src/resources/walls/*.rs` — 骨架对齐 Rust（镜像装配 crate 布局；未知 → `typed_gap::Gap`）。
- `Cargo.toml` + `src/lib.rs` — 独立 offline `cargo check` 束（bevy_ecs 0.16.0 + glam 0.29.3，
  匹配原锁基线）。
- `evidence/` — 本轮 Ghidra 只读反编译（g1/g3/g4）+ 机器码 read_memory（g2）。
- `evidence.json` — 地址、源 SHA、先验复用裁决、编译门禁。

## 本包恢复内容（节选）

| 类型 | 状态 | 关键证据 |
|---|---|---|
| `WallId` | **O** u64 newtype | get_inner 0x140885d0；is_valid 0x140b34f20（u64!=0）；serde 十进制数字串 0x140b35980 |
| `PublicWallState` | **O** 0x2a0 记录 | max_y +0x108/+0x10c、gable +0x110/+0x114、形状 +0x258..0x274、style +0x299；方法面 16 个按骨架 |
| `WallShape` | **O** 判别 0/1/2 | get_wall_shape 0xb2a5d0（判别<2）、is_circle/is_rectangle 载荷复制、Roof 共享 28B |
| `WallStyle` | **O** Stone=0/StoneWithSkirts=1/HalfTimbered=2 | deserialize_wall_style 0x85ebc0 字符串比对；is_halftimbered 0x8c5670 `>=2` |
| `StructureType` | **O** Wall=0 | is_wall 0x833230 `==0`（与 ClutterVersion::is_initial COMDAT 折叠同址）；is_fence 判别读取 |
| `FreehandHeightType` | **O** 字节枚举 | toggle 0x8c5660 `*self ^= 1` |
| `WallAttachmentAnchor` | **O** 字段名+偏移 | anchor 序列化器 0x1416fa690；B Info +8..+22（先验 decorator-info） |
| `WallCornerAttachment` | **O** 字节枚举 | is_attached 0xb256b0 字节!=0；from/to_rectangle_corner_id 0xb256c0/f0（id<4） |
| `split_height_into_skirts` | **O** | 0x140b34f80：height>=5.9 → 4.5,+3.5 递增，`height>f+1.4` |
| `HighlightColor::to_rgb` | **O** | 0x140b34f30 两分支 RGB（机器码 xmm 常量） |
| `WallPathSegment` / `InProgressWall` | 方法面 O / 布局 U | pt_count/length_ws/length_ws_2d；new/fmt |

## 先验复用（未重推导）

`reconstruction/building/{wall_state,wall_publication,wall_height_authority,wall_height_command,wall_cache,wall_structures,decorator_info,decorator_subtype}.rs`、
`reconstruction/wall/height_rules.rs`、`evidence/building-decorator-info/findings.md`、
`evidence/building-state/findings.md`。本包以其组织骨架对齐 Rust + 方法面。

## 门禁

- `cargo check --offline`：0 错误。
- `cargo clippy --offline --all-targets`：0 警告 / 0 错误。

## 已知缺口（typed gap，未臆造）

- PublicWallState 内部 Curve2/3D outline/附加数组（+0xe8）完整布局；wall_space/u_space 返回类型；
  corner_pos_and_normal_xz 矩形几何（Rectangle2d）接线；max_ivy_grow_height/corner_normal_xz。
- WallState（inner）完整布局；WallStyle 其余变体；StructureType Fence 值；ArchType/PlasterPeelAmount
  变体；is_default_arch_height 默认常量值。
- WallCornerAttachment 角编码（I：字节=id+1）；decorator.rs Roof/Terrain/Stair 附着、DecoratorExtraInfo、
  DecoratorRotationXZ、DecoratorRank 值、ClutterVersion.latest 值。
- 序列化 JSON 精确格式（WallStyle 等 serde 派生）；DecoratorInfo 未观察字段（+0x28/+0x30..0x43）。
- 全部 `implemented_unverified`；无 runtime/GPU 验收。合同为审阅依据。
