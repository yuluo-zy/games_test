//! Original radial-menu ownership and creation. B1425f27c0 is 5531 bytes;
//! the former 1947-byte decompilation truncated its two jump tables in rdata.
//! Full original instructions and both tables are retained in round12/render.
//! The real create leaf is `spawn`; the old show producer's data-only path is
//! kept explicitly incomplete until all its target/selection branches are ported.

use bevy_ecs::prelude::*;

use country_core::resources::walls::decorator::ClutterVersion;

/// 图标信息（O：40B 布局；`new_separator`/`new_flat_roof_rim_ty` 逐字节写面）。
#[derive(Clone, Debug, Default)]
pub struct IconInfo {
    /// 图标网格路径名（String：len@+0 / ptr@+8 / cap@+0x10）。
    pub name: String,
    /// kind（O：5=separator、8=类型图标；其余值见 create 的分派面）。
    pub kind: u8,
    /// +0x19（部分构造器使用）。
    pub b19: u8,
    /// +0x1a。
    pub u1a: u16,
    /// +0x1c：u32/f32 值（颜色 id 或开关值；demo 分支写 0x309=777）。
    pub v1c: u32,
    /// +0x20。
    pub v20: u32,
    /// +0x24（new_* 均置 0）。
    pub v24: u32,
}

/// `IconInfo::new_separator` @0x1425f2680（106B，判等）。
///
/// fresh 反编译写面：`"white"`(len 5)、`kind=5`、`+0x24=0`。
pub fn new_separator() -> IconInfo {
    IconInfo {
        name: "white".to_string(),
        kind: 5,
        ..IconInfo::default()
    }
}

/// `IconInfo::new_flat_roof_rim_ty` @0x1425f26f0（201B，判等）。
///
/// fresh 反编译：读 `ClutterVersion::is_initial()`（0x140833230）；
/// true → `"color_icon/freehand_ty_terrain"`(0x1e=30B)；false →
/// `"color_icon/freehand_ty_constant"`(0x1f=31B)。`kind=8`、`+0x24=0`。
/// PDB 符号名 `new_flat_roof_rim_ty` 与图标串 `freehand_ty_*` 错位（登记，不发明
/// 改名）。
pub fn new_flat_roof_rim_ty() -> IconInfo {
    let initial = ClutterVersion::is_initial(ClutterVersion::default());
    IconInfo {
        name: if initial {
            "color_icon/freehand_ty_terrain".to_string()
        } else {
            "color_icon/freehand_ty_constant".to_string()
        },
        kind: 8,
        ..IconInfo::default()
    }
}

/// Initial selection copied into the owner by B1425f2a56, independently of
/// the current selection. Cancellation restores every stored value.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ColorSelectionSnapshot {
    pub color: u32,
    pub brightness: f32,
    pub plaster_peel: f32,
    pub magic_color: u32,
    pub flat_roof_style: [u8; 12],
    pub railing: u8,
    pub wall_style: u8,
    pub flag_connection: u8,
    pub freehand_ty: u8,
    pub flat_roof_rim_style: u8,
    pub chimney_smoke: u8,
    pub reserved_22: [u8; 2],
}
/// Projection of the original 16-byte menu target. Decorator ids use payload;
/// wall ids use id, preserving all 64 bits.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ColorMenuTarget {
    pub kind: u8,
    pub variant: u8,
    pub reserved: u16,
    pub payload: u32,
    pub id: u64,
}

#[derive(Component, Clone, Debug, Default)]
pub struct ColorRadialMenu {
    pub center: glam::Vec2,
    pub radius: f32,
    pub icons: Vec<IconInfo>,
    pub icon_entities: Vec<Entity>,
    pub target: ColorMenuTarget,
    pub initial: ColorSelectionSnapshot,
    pub angle_range: Option<utils::angle_range::AngleRange>,
    pub selected_index: usize,
    pub interaction_start_index: Option<usize>,
    pub navigation_indices: Vec<usize>,
    pub navigation_state: u32,
    pub icon_size: f32,
    pub world_anchor: glam::Vec3,
}
/// Original arguments after register/stack projection. Position, radius and
/// selection must be supplied by the real producer, not guessed in this leaf.
pub struct RadialMenuCreate {
    pub center: glam::Vec2,
    pub radius: f32,
    pub start_angle: f32,
    pub icon_size: f32,
    pub spacing: f32,
    pub appearance_divisor: f32,
    pub icons: Vec<IconInfo>,
    pub target: ColorMenuTarget,
    pub initial: ColorSelectionSnapshot,
    pub world_anchor: glam::Vec3,
}
impl ColorRadialMenu {
    /// Data-only legacy producer surface; this does not create native icons.
    /// The unrecovered show producer remains explicitly incomplete.
    pub fn create(center: glam::Vec2, radius: f32, icons: Vec<IconInfo>) -> Self {
        Self {
            center,
            radius,
            icons,
            ..Self::default()
        }
    }
    /// B1425f27c0, complete 5531-byte body including both 12-way jump tables.
    /// Return matches the original `(Vec<Entity>, Entity)`; separators are kept
    /// in the menu's owner list but omitted from the returned interactive list.
    pub fn spawn(params: RadialMenuCreate, commands: &mut Commands) -> (Vec<Entity>, Entity) {
        use country_core::components::{Ui2dExtent, Ui2dImage, UiDrawOrder};
        use country_core::resources::color_icon::*;
        use country_core::resources::roofs::customization::FlatRoofStyleObserved;
        let RadialMenuCreate {
            center,
            radius,
            start_angle,
            icon_size,
            spacing,
            appearance_divisor,
            icons,
            target,
            initial,
            world_anchor,
        } = params;
        let count = icons.iter().filter(|icon| icon.kind != 5).count();
        let span = (icon_size + spacing) * (count as f32 - 1.0) / radius;
        let start = span * -0.5 + std::f32::consts::FRAC_PI_4 + start_angle;
        let angle_range = utils::angle_range::AngleRange::from_min_and_len(start, span);
        let mut arc_position = start * radius;
        let mut selected_index = 0;
        let mut matched_initial = false;
        let mut interactive = Vec::with_capacity(count);
        let mut entities = Vec::with_capacity(icons.len());
        for (index, icon) in icons.iter().enumerate() {
            let separator = icon.kind == 5;
            let size = if separator {
                icon_size * 0.75
            } else {
                icon_size
            };
            if separator {
                arc_position -= icon_size * 0.25;
            }
            let angle = arc_position / radius;
            let position = glam::Vec2::new(angle.cos(), angle.sin()) * radius + center;
            let selected = icon.is_initial_selection(&initial);
            if selected && !matched_initial {
                selected_index = index;
                matched_initial = true;
            }
            let appearance = (index as f32 * -0.1 + if start_angle > 0.0 { -0.75 } else { 0.0 })
                / appearance_divisor;
            let mut entity = commands.spawn((
                Ui2dExtent {
                    pos: center,
                    size: glam::Vec2::ZERO,
                },
                Ui2dImage {
                    name: icon.name.clone(),
                    layer: icon.v24,
                },
                CustomizationIcon {
                    size: glam::Vec2::splat(size * if separator { 0.3 } else { 1.0 }),
                    position,
                    initial_position: center,
                    alpha_18: appearance,
                    hover_progress: 0.0,
                    hovered: 0,
                    flag_21: u8::from(selected),
                    opaque_22: [0; 2],
                },
                UiDrawOrder(1),
            ));
            match icon.kind {
                3 => {
                    entity.insert(ColorIcon(icon.v1c));
                }
                4 => {
                    entity.insert(WallStyleIcon(icon.b19));
                }
                5 => {
                    entity.insert(ColorWheelSeparator);
                }
                6 => {
                    entity.insert(FlagConnectionIcon);
                }
                7 => {
                    entity.insert(FreehandHeightTypeToggleIcon);
                }
                8 => {
                    entity.insert(FlatRoofRimTypeToggle);
                }
                9 => {
                    entity.insert(ColorMenuBrightnessIcon(
                        ColorMenuBrightnessValue::from_tag(icon.b19)
                            .expect("native brightness tag must be in 0..3"),
                    ));
                }
                10 => {
                    entity.insert(ChimneySmokeToggle);
                }
                12 => {
                    entity.insert(PlasterPeelIcon(f32::from_bits(icon.v1c)));
                }
                13 => {
                    entity.insert(StairRailingIcon(icon.b19));
                }
                14 => {
                    entity.insert(MagicColorIcon(icon.v1c));
                }
                _ => {
                    entity.insert(FlatRoofStyleIcon(FlatRoofStyleObserved::from_observed(
                        icon.flat_style_bytes(),
                    )));
                }
            }
            let id = entity.id();
            entities.push(id);
            if !separator {
                interactive.push(id);
            }
            arc_position += size + spacing;
        }
        let menu = commands
            .spawn(Self {
                center,
                radius,
                icons,
                icon_entities: entities,
                target,
                initial,
                angle_range: Some(angle_range),
                selected_index,
                interaction_start_index: None,
                navigation_indices: Vec::new(),
                navigation_state: 0,
                icon_size,
                world_anchor,
            })
            .id();
        (interactive, menu)
    }
}
impl IconInfo {
    pub fn flat_style_bytes(&self) -> [u8; 12] {
        let mut bytes = [0; 12];
        bytes[0] = self.kind;
        bytes[1] = self.b19;
        bytes[2..4].copy_from_slice(&self.u1a.to_le_bytes());
        bytes[4..8].copy_from_slice(&self.v1c.to_le_bytes());
        bytes[8..12].copy_from_slice(&self.v20.to_le_bytes());
        bytes
    }
    fn is_initial_selection(&self, initial: &ColorSelectionSnapshot) -> bool {
        match self.kind {
            3 => initial.color == self.v1c,
            4 => initial.wall_style == self.b19,
            5 | 6 | 7 | 8 | 10 => false,
            9 => country_core::resources::color_icon::ColorMenuBrightnessValue::from_tag(self.b19)
                .is_some_and(|value| (initial.brightness - value.to_f32()).abs() < 0.001),
            12 => initial.plaster_peel == f32::from_bits(self.v1c),
            13 => initial.railing == self.b19,
            14 => initial.magic_color == self.v1c,
            _ => {
                initial.flat_roof_style[0] == self.kind
                    && (initial.flat_roof_style[0] != 2
                        || (initial.flat_roof_style[1] == self.b19
                            && initial.flat_roof_style[4..8] == self.v1c.to_le_bytes()
                            && initial.flat_roof_style[8..12] == self.v20.to_le_bytes()))
            }
        }
    }
}
pub fn register_required_components(world: &mut World) {
    world.register_component::<country_core::resources::color_icon::CustomizationIcon>();
}

#[cfg(test)]
mod native_create_tests {
    use super::*;
    use country_core::{
        components::{Ui2dExtent, Ui2dImage, UiDrawOrder},
        resources::color_icon::*,
    };
    #[test]
    fn all_native_icon_payloads_and_owners_are_created() {
        let mut world = World::new();
        let mut queue = bevy_ecs::world::CommandQueue::default();
        let icons = (0..=14)
            .map(|kind| IconInfo {
                name: format!("icon{kind}"),
                kind,
                b19: if kind == 9 { 1 } else { 0 },
                v24: u32::from(kind),
                ..Default::default()
            })
            .collect();
        let (interactive, menu) = ColorRadialMenu::spawn(
            RadialMenuCreate {
                center: glam::Vec2::new(400., 300.),
                radius: 100.,
                start_angle: 0.,
                icon_size: 20.,
                spacing: 5.,
                appearance_divisor: 1.,
                icons,
                target: ColorMenuTarget {
                    kind: 0,
                    variant: 0,
                    reserved: 0,
                    payload: 0,
                    id: 0x100000008,
                },
                initial: ColorSelectionSnapshot {
                    brightness: 1.,
                    ..Default::default()
                },
                world_anchor: glam::Vec3::new(10., 20., 30.),
            },
            &mut Commands::new(&mut queue, &world),
        );
        queue.apply(&mut world);
        assert_eq!(interactive.len(), 14);
        let owner = world.get::<ColorRadialMenu>(menu).unwrap();
        assert_eq!(owner.icon_entities.len(), 15);
        assert_eq!(owner.selected_index, 0);
        assert_eq!(owner.world_anchor, glam::Vec3::new(10., 20., 30.));
        let ids = owner.icon_entities.clone();
        assert!(!interactive.contains(&ids[5]));
        assert!(world.get::<ColorWheelSeparator>(ids[5]).is_some());
        assert_eq!(world.get::<ColorIcon>(ids[3]).unwrap().0, 0);
        assert_eq!(world.get::<WallStyleIcon>(ids[4]).unwrap().0, 0);
        assert!(world.get::<FlagConnectionIcon>(ids[6]).is_some());
        assert!(world.get::<FreehandHeightTypeToggleIcon>(ids[7]).is_some());
        assert!(world.get::<FlatRoofRimTypeToggle>(ids[8]).is_some());
        assert_eq!(
            world
                .get::<ColorMenuBrightnessIcon>(ids[9])
                .unwrap()
                .0
                .to_f32(),
            1.
        );
        assert!(world.get::<ChimneySmokeToggle>(ids[10]).is_some());
        assert!(world.get::<FlatRoofStyleIcon>(ids[11]).is_some());
        assert_eq!(world.get::<PlasterPeelIcon>(ids[12]).unwrap().0, 0.);
        assert_eq!(world.get::<StairRailingIcon>(ids[13]).unwrap().0, 0);
        assert_eq!(world.get::<MagicColorIcon>(ids[14]).unwrap().0, 0);
        for (index, id) in ids.into_iter().enumerate() {
            let extent = world.get::<Ui2dExtent>(id).unwrap();
            assert_eq!(extent.pos, glam::Vec2::new(400., 300.));
            assert_eq!(extent.size, glam::Vec2::ZERO);
            assert_eq!(world.get::<Ui2dImage>(id).unwrap().layer, index as u32);
            assert_eq!(world.get::<UiDrawOrder>(id).unwrap().0, 1);
            let icon = world.get::<CustomizationIcon>(id).unwrap();
            assert_eq!(icon.alpha_18, index as f32 * -0.1);
            assert!((icon.position.distance(icon.initial_position) - 100.).abs() < 0.0001);
            if index == 5 {
                assert_eq!(icon.size, glam::Vec2::splat(4.5));
            }
        }
    }
}
