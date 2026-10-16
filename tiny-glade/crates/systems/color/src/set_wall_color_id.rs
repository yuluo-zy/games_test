//! set_wall_color_id.rs — 墙颜色写入（B 0x142290750 init_color_ids_buffer /
//! 0x142290820 set_wall_color_id，O；re-analyst-A 契约 §2）。
//!
//! - `init_color_ids_buffer`：三色表资源注册（`WallColorIds::new` 0x140b19890 /
//!   `GateColorIds::new` 0x140b19900 / `HalftimberColorIds::new` 0x140b19970
//!   insert_resource）+ 4×0x140987ca0 菜单登记。
//! - `set_wall_color_id`：命令（0x18B 条目 {id@+8, 新色 u32@+0x10, custom u8@+0x14}）
//!   遍历：`idx = id→索引`（0x1400885d0，PDB 双名冲突登记）；断言
//!   `idx < 0x4fff`（MAX_NUMBER_OF_WALLS=20480）；写 `SsboVec[idx]=色`；
//!   PublicWalls::get → custom 时 AABB 中心 + max_y×0.4922 打包推 0x20B 记录
//!   （tag 6）；脏色分类（新色 ∈ {4..10,12} → 自定义列表、旧色 ∈ 该集且新色非
//!   → 回默认列表）；末尾整表 flush（0x140beefa0，len==0 提前返回）。

use bevy_ecs::prelude::*;

use country_core::resources::color_id::{SetWallColorIdCmd, WallColorIds, MAX_NUMBER_OF_WALLS};
use country_core::resources::walls::public_walls::PublicWalls;
use country_core::resources::walls::WallId;
use country_core::systems::audio::PlayWallUiSound;


/// 0x20B 实体颜色记录（O：{u64 实体/id, u64 打包中心, f32 max_y×0.4922, u8 tag}；
/// tag 6=墙、7=门、0xc=半木、10=装饰物；re-analyst §2）。
#[derive(Clone, Copy, Debug, Default)]
pub struct EntityColorRecord {
    /// 实体/id。
    pub id: u64,
    /// 打包中心（Aabb2::center 0x1412400c0 + pack 0x140c9cde0；位布局 U）。
    pub packed_center: u64,
    /// max_y × 0.4922（0x140b2a620 max_y 访问器族）。
    pub scaled_max_y: f32,
    /// tag（6/7/0xc/10）。
    pub tag: u8,
}

/// max_y 缩放（O：0.4922，0x142925874 区）。
pub const MAX_Y_SCALE: f32 = 0.4922;
/// 自定义色集合（O：新色 ∈ {4..10, 12} → 自定义列表）。
pub const CUSTOM_COLOR_SET: [u32; 8] = [4, 5, 6, 7, 8, 9, 10, 12];

/// `init_color_ids_buffer` @0x142290750（O 面：三色表 insert_resource；参数面
/// BASELINE 的 `Res<GraphicsDeviceResource>` 待 rhapsody 域接线，以 library 面
/// 落地）。
pub fn init_color_ids_buffer(
    mut commands: Commands,
    _library: Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,
) {
    use country_core::resources::color_id::{GateColorIds, HalftimberColorIds};
    commands.insert_resource(WallColorIds::new());
    commands.insert_resource(GateColorIds::new());
    commands.insert_resource(HalftimberColorIds::new());
}

/// `set_wall_color_id` @0x142290820（O 面；reviewer-A FIX：8 参面 =
/// Res<AssetSsboLibrary> + ER + ResMut<WallColorIds> + EW<PlayWallUiSound> +
/// Res<PublicWalls> + EW<CreatePlasterCmd> + EW<DeletePlasterCmd>——脏色分类
/// 的机器码落点 = 石膏重建两通道；自造 ColorMenuState 移除（脏列表/0x20B 记录
/// 的 core 内表字段未恢复，登记 GAP 核心契约）。
#[allow(clippy::too_many_arguments)]
pub fn set_wall_color_id(
    _library: Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,
    mut cmds: EventReader<SetWallColorIdCmd>,
    mut wall_colors: ResMut<WallColorIds>,
    mut sounds: EventWriter<PlayWallUiSound>,
    walls: Res<PublicWalls>,
    mut plaster_create: EventWriter<country_core::resources::walls::plaster::CreatePlasterCmd>,
    mut plaster_delete: EventWriter<country_core::resources::walls::plaster::DeletePlasterCmd>,
) {
    for cmd in cmds.read() {
        // O：断言 `idx < 0x4fff`（MAX_NUMBER_OF_WALLS；panic 文案
        // 'assertion failed: wall_index < MAX_NUMBER_OF_WALLS' 0x14303b160）。
        let idx = cmd.wall_id as usize;
        assert!(
            idx < MAX_NUMBER_OF_WALLS,
            "wall_index < MAX_NUMBER_OF_WALLS"
        );
        // O：写 SsboVec[idx] = 色。
        let old = wall_colors.0[idx];
        wall_colors.0[idx] = cmd.color;
        // O：脏色分类（O：新色 ∈ {4..10,12} → 自定义（石膏重建 CreatePlaster）；
        // 旧色 ∈ 该集且新色非 → 回默认（DeletePlaster））。
        let new_custom = CUSTOM_COLOR_SET.contains(&cmd.color);
        let old_custom = CUSTOM_COLOR_SET.contains(&old);
        if new_custom && !old_custom {
            plaster_create.write(country_core::resources::walls::plaster::CreatePlasterCmd(
                WallId(cmd.wall_id),
            ));
        } else if old_custom && !new_custom {
            plaster_delete.write(country_core::resources::walls::plaster::DeletePlasterCmd(
                WallId(cmd.wall_id),
            ));
        }
        // O：PublicWalls::get → custom 时 0x20B 记录（tag 6；真实 max_y；AABB
        // 中心打包面 U——core 内表字段未恢复，登记 GAP）。
        if cmd.custom != 0 {
            if let Some(wall) = walls.get(WallId(cmd.wall_id)) {
                let _ = wall.max_y();
            }
        }
        let _ = &mut sounds;
    }
}
