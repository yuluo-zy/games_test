//! Original photo toolbar setup (B 0x1426246e0, PDB size 9053).
//! Both edition trees and nominal custom payload types were executed/captured
//! from the original binary. Root callback dispatch: 142623ff4..1426242ee.
//! Evidence: progress-20261020/phase2-toolbar-photo/photomode-original-trees.json
//! and photo-category-callbacks.json. Shared factory owns nodes and closures.
use crate::resouces_and_components::{
    PhotomodeButton, ShowOnlyInPhotomode, TimeOfDayButtonV2, UiLocked,
    taffy_layout::TaffyLayout,
    toolbar_layout::{PhotomodeToolbarLayout, ToolbarItem, ToolbarItemKind as K, ToolbarLayout},
};
use bevy_ecs::system::Commands;
use country_core::resources::{toolbar::ToolbarCategory, ui_settings::ToolbarAlignment};
use glam::Vec2;

pub fn photomode_category_items(
    demo: bool,
) -> Vec<(&'static str, ToolbarCategory, Vec<ToolbarItem>)> {
    if demo {
        vec![
            (
                "toolbar/photomode_folder",
                ToolbarCategory::PhotomodeFolder,
                vec![],
            ),
            (
                "toolbar/photomode_camera_mode",
                ToolbarCategory::PhotomodeCamera,
                vec![ToolbarItem::new(
                    K::Row(vec![
                        ToolbarItem::new(
                            K::Custom(Box::new(|entity| {
                                entity.insert(PhotomodeButton { tag: 0 });
                            })),
                            "toolbar/photomode_camera_tiltshift",
                            Vec2::new(1.0, 1.0),
                        ),
                        ToolbarItem::new(
                            K::Custom(Box::new(|entity| {
                                entity.insert(PhotomodeButton { tag: 1 });
                            })),
                            "toolbar/photomode_camera_free",
                            Vec2::new(1.0, 1.0),
                        ),
                        ToolbarItem::new(
                            K::Custom(Box::new(|entity| {
                                entity.insert(PhotomodeButton { tag: 2 });
                            })),
                            "toolbar/photomode_camera_walk",
                            Vec2::new(1.0, 1.0),
                        ),
                        ToolbarItem::new(
                            K::Locked,
                            "toolbar/photomode_camera_topdown_demo",
                            Vec2::new(1.0, 1.0),
                        ),
                    ]),
                    "",
                    Vec2::new(1.0, 1.0),
                )],
            ),
            (
                "toolbar/photomode_fov",
                ToolbarCategory::PhotomodeFov,
                vec![ToolbarItem::new(
                    K::Row(vec![
                        ToolbarItem::new(K::Spacer, "toolbar/photomode_fov", Vec2::new(1.0, 1.0)),
                        ToolbarItem::new(K::Slider(6), "toolbar/slider_bg", Vec2::new(3.0, 1.0)),
                        ToolbarItem::new(K::SliderReset(6), "toolbar/reset", Vec2::new(1.0, 1.0)),
                    ]),
                    "",
                    Vec2::new(1.0, 1.0),
                )],
            ),
            (
                "toolbar/photomode_dof",
                ToolbarCategory::PhotomodeDof,
                vec![ToolbarItem::new(
                    K::Row(vec![
                        ToolbarItem::new(K::Spacer, "toolbar/photomode_dof", Vec2::new(1.0, 1.0)),
                        ToolbarItem::new(K::Slider(8), "toolbar/slider_bg", Vec2::new(3.0, 1.0)),
                        ToolbarItem::new(K::SliderReset(8), "toolbar/reset", Vec2::new(1.0, 1.0)),
                    ]),
                    "",
                    Vec2::new(1.0, 1.0),
                )],
            ),
            (
                "toolbar/photomode_focus",
                ToolbarCategory::PhotomodeFocus,
                vec![ToolbarItem::new(
                    K::Row(vec![
                        ToolbarItem::new(
                            K::Custom(Box::new(|entity| {
                                entity.insert(PhotomodeButton { tag: 16 });
                            })),
                            "toolbar/photomode_pin",
                            Vec2::new(1.0, 1.0),
                        ),
                        ToolbarItem::new(K::Slider(7), "toolbar/slider_bg", Vec2::new(3.0, 1.0)),
                        ToolbarItem::new(K::SliderReset(7), "toolbar/reset", Vec2::new(1.0, 1.0)),
                    ]),
                    "",
                    Vec2::new(1.0, 1.0),
                )],
            ),
            (
                "toolbar/photomode_take_picture",
                ToolbarCategory::PhotomodeTakePic,
                vec![],
            ),
            (
                "toolbar/photomode_ev_shift",
                ToolbarCategory::PhotomodeEvShift,
                vec![ToolbarItem::new(
                    K::Row(vec![
                        ToolbarItem::new(K::Spacer, "toolbar/photomode_ev", Vec2::new(1.0, 1.0)),
                        ToolbarItem::new(K::Slider(0), "toolbar/slider_bg", Vec2::new(3.0, 1.0)),
                        ToolbarItem::new(K::SliderReset(0), "toolbar/reset", Vec2::new(1.0, 1.0)),
                    ]),
                    "",
                    Vec2::new(1.0, 1.0),
                )],
            ),
            (
                "toolbar/photomode_luts_demo",
                ToolbarCategory::PhotomodeLuts,
                vec![],
            ),
            (
                "toolbar/photomode_artistic_demo",
                ToolbarCategory::PhotomodeArtistic,
                vec![],
            ),
            ("toolbar/photomode_tod", ToolbarCategory::TimeOfDay, vec![]),
            (
                "toolbar/photomode_back",
                ToolbarCategory::PhotomodeBack,
                vec![],
            ),
        ]
    } else {
        vec![
            (
                "toolbar/photomode_folder",
                ToolbarCategory::PhotomodeFolder,
                vec![],
            ),
            (
                "toolbar/photomode_camera_mode",
                ToolbarCategory::PhotomodeCamera,
                vec![ToolbarItem::new(
                    K::Row(vec![
                        ToolbarItem::new(
                            K::Custom(Box::new(|entity| {
                                entity.insert(PhotomodeButton { tag: 0 });
                            })),
                            "toolbar/photomode_camera_tiltshift",
                            Vec2::new(1.0, 1.0),
                        ),
                        ToolbarItem::new(
                            K::Custom(Box::new(|entity| {
                                entity.insert(PhotomodeButton { tag: 1 });
                            })),
                            "toolbar/photomode_camera_free",
                            Vec2::new(1.0, 1.0),
                        ),
                        ToolbarItem::new(
                            K::Custom(Box::new(|entity| {
                                entity.insert(PhotomodeButton { tag: 2 });
                            })),
                            "toolbar/photomode_camera_walk",
                            Vec2::new(1.0, 1.0),
                        ),
                        ToolbarItem::new(
                            K::Custom(Box::new(|entity| {
                                entity.insert(PhotomodeButton { tag: 3 });
                            })),
                            "toolbar/photomode_camera_topdown",
                            Vec2::new(1.0, 1.0),
                        ),
                    ]),
                    "",
                    Vec2::new(1.0, 1.0),
                )],
            ),
            (
                "toolbar/photomode_fov",
                ToolbarCategory::PhotomodeFov,
                vec![ToolbarItem::new(
                    K::Column(vec![
                        ToolbarItem::new(
                            K::Row(vec![
                                {
                                    let mut item = ToolbarItem::new(
                                        K::Spacer,
                                        "toolbar/photomode_distortion",
                                        Vec2::new(1.0, 1.0),
                                    );
                                    item.show_only_fpv = true;
                                    item
                                },
                                {
                                    let mut item = ToolbarItem::new(
                                        K::Slider(4),
                                        "toolbar/slider_bg",
                                        Vec2::new(3.0, 1.0),
                                    );
                                    item.show_only_fpv = true;
                                    item
                                },
                                {
                                    let mut item = ToolbarItem::new(
                                        K::SliderReset(4),
                                        "toolbar/reset",
                                        Vec2::new(1.0, 1.0),
                                    );
                                    item.show_only_fpv = true;
                                    item
                                },
                            ]),
                            "",
                            Vec2::new(1.0, 1.0),
                        ),
                        ToolbarItem::new(
                            K::Row(vec![
                                ToolbarItem::new(
                                    K::Spacer,
                                    "toolbar/photomode_fov",
                                    Vec2::new(1.0, 1.0),
                                ),
                                ToolbarItem::new(
                                    K::Slider(6),
                                    "toolbar/slider_bg",
                                    Vec2::new(3.0, 1.0),
                                ),
                                ToolbarItem::new(
                                    K::SliderReset(6),
                                    "toolbar/reset",
                                    Vec2::new(1.0, 1.0),
                                ),
                            ]),
                            "",
                            Vec2::new(1.0, 1.0),
                        ),
                    ]),
                    "",
                    Vec2::new(1.0, 1.0),
                )],
            ),
            (
                "toolbar/photomode_dof",
                ToolbarCategory::PhotomodeDof,
                vec![ToolbarItem::new(
                    K::Column(vec![
                        ToolbarItem::new(
                            K::Row(vec![
                                ToolbarItem::new(
                                    K::Spacer,
                                    "toolbar/photomode_vignette",
                                    Vec2::new(1.0, 1.0),
                                ),
                                ToolbarItem::new(
                                    K::Slider(3),
                                    "toolbar/slider_bg",
                                    Vec2::new(3.0, 1.0),
                                ),
                                ToolbarItem::new(
                                    K::SliderReset(3),
                                    "toolbar/reset",
                                    Vec2::new(1.0, 1.0),
                                ),
                            ]),
                            "",
                            Vec2::new(1.0, 1.0),
                        ),
                        ToolbarItem::new(
                            K::Row(vec![
                                ToolbarItem::new(
                                    K::Spacer,
                                    "toolbar/photomode_dof",
                                    Vec2::new(1.0, 1.0),
                                ),
                                ToolbarItem::new(
                                    K::Slider(8),
                                    "toolbar/slider_bg",
                                    Vec2::new(3.0, 1.0),
                                ),
                                ToolbarItem::new(
                                    K::SliderReset(8),
                                    "toolbar/reset",
                                    Vec2::new(1.0, 1.0),
                                ),
                            ]),
                            "",
                            Vec2::new(1.0, 1.0),
                        ),
                    ]),
                    "",
                    Vec2::new(1.0, 1.0),
                )],
            ),
            (
                "toolbar/photomode_focus",
                ToolbarCategory::PhotomodeFocus,
                vec![ToolbarItem::new(
                    K::Row(vec![
                        ToolbarItem::new(
                            K::Custom(Box::new(|entity| {
                                entity.insert(PhotomodeButton { tag: 16 });
                            })),
                            "toolbar/photomode_pin",
                            Vec2::new(1.0, 1.0),
                        ),
                        ToolbarItem::new(K::Slider(7), "toolbar/slider_bg", Vec2::new(3.0, 1.0)),
                        ToolbarItem::new(K::SliderReset(7), "toolbar/reset", Vec2::new(1.0, 1.0)),
                    ]),
                    "",
                    Vec2::new(1.0, 1.0),
                )],
            ),
            (
                "toolbar/photomode_take_picture",
                ToolbarCategory::PhotomodeTakePic,
                vec![],
            ),
            (
                "toolbar/photomode_ev_shift",
                ToolbarCategory::PhotomodeEvShift,
                vec![ToolbarItem::new(
                    K::Column(vec![
                        ToolbarItem::new(
                            K::Row(vec![
                                ToolbarItem::new(
                                    K::Spacer,
                                    "toolbar/photomode_saturation",
                                    Vec2::new(1.0, 1.0),
                                ),
                                ToolbarItem::new(
                                    K::Slider(2),
                                    "toolbar/slider_bg",
                                    Vec2::new(3.0, 1.0),
                                ),
                                ToolbarItem::new(
                                    K::SliderReset(2),
                                    "toolbar/reset",
                                    Vec2::new(1.0, 1.0),
                                ),
                            ]),
                            "",
                            Vec2::new(1.0, 1.0),
                        ),
                        ToolbarItem::new(
                            K::Row(vec![
                                ToolbarItem::new(
                                    K::Spacer,
                                    "toolbar/photomode_contrast",
                                    Vec2::new(1.0, 1.0),
                                ),
                                ToolbarItem::new(
                                    K::Slider(1),
                                    "toolbar/slider_bg",
                                    Vec2::new(3.0, 1.0),
                                ),
                                ToolbarItem::new(
                                    K::SliderReset(1),
                                    "toolbar/reset",
                                    Vec2::new(1.0, 1.0),
                                ),
                            ]),
                            "",
                            Vec2::new(1.0, 1.0),
                        ),
                        ToolbarItem::new(
                            K::Row(vec![
                                ToolbarItem::new(
                                    K::Spacer,
                                    "toolbar/photomode_ev",
                                    Vec2::new(1.0, 1.0),
                                ),
                                ToolbarItem::new(
                                    K::Slider(0),
                                    "toolbar/slider_bg",
                                    Vec2::new(3.0, 1.0),
                                ),
                                ToolbarItem::new(
                                    K::SliderReset(0),
                                    "toolbar/reset",
                                    Vec2::new(1.0, 1.0),
                                ),
                            ]),
                            "",
                            Vec2::new(1.0, 1.0),
                        ),
                    ]),
                    "",
                    Vec2::new(1.0, 1.0),
                )],
            ),
            (
                "toolbar/photomode_luts",
                ToolbarCategory::PhotomodeLuts,
                vec![],
            ),
            (
                "toolbar/photomode_artistic",
                ToolbarCategory::PhotomodeArtistic,
                vec![ToolbarItem::new(
                    K::Row(vec![
                        ToolbarItem::new(
                            K::Custom(Box::new(|entity| {
                                entity.insert(PhotomodeButton { tag: 12 });
                            })),
                            "toolbar/photomode_painterly",
                            Vec2::new(1.0, 1.0),
                        ),
                        ToolbarItem::new(
                            K::Custom(Box::new(|entity| {
                                entity.insert(PhotomodeButton { tag: 13 });
                            })),
                            "toolbar/photomode_lightonly",
                            Vec2::new(1.0, 1.0),
                        ),
                        ToolbarItem::new(
                            K::Custom(Box::new(|entity| {
                                entity.insert(PhotomodeButton { tag: 14 });
                            })),
                            "toolbar/photomode_tree_seethrough",
                            Vec2::new(1.0, 1.0),
                        ),
                    ]),
                    "",
                    Vec2::new(1.0, 1.0),
                )],
            ),
            ("toolbar/photomode_tod", ToolbarCategory::TimeOfDay, vec![]),
            (
                "toolbar/photomode_back",
                ToolbarCategory::PhotomodeBack,
                vec![],
            ),
        ]
    }
}

/// Construct every category root and contextual tree, then link the actual
/// category entities into the original cyclic keyboard/controller navigation.
pub fn setup_photomode_toolbar(
    taffy: &mut TaffyLayout,
    commands: &mut Commands,
    demo: bool,
    alignment: ToolbarAlignment,
) -> PhotomodeToolbarLayout {
    let (screen, toolbar) = ToolbarLayout::setup_screen_and_toolbar_nodes(taffy);
    let mut layout = ToolbarLayout {
        node_a: screen,
        node_b: toolbar,
        ..Default::default()
    };
    for (label, category, items) in photomode_category_items(demo) {
        layout.add_category_button_ordered(
            taffy,
            category,
            label,
            commands,
            |entity| {
                use ToolbarCategory::*;
                match category {
                    TimeOfDay => {
                        entity.insert(TimeOfDayButtonV2);
                    }
                    PhotomodeFolder => {
                        entity.insert(PhotomodeButton { tag: 4 });
                    }
                    PhotomodeCamera => {
                        entity.insert(PhotomodeButton { tag: 5 });
                    }
                    PhotomodeFocus => {
                        entity.insert(PhotomodeButton { tag: 15 });
                    }
                    PhotomodeTakePic => {
                        entity.insert(PhotomodeButton { tag: 9 });
                    }
                    PhotomodeLuts => {
                        // Original demo branch intentionally queues no action.
                        if !demo {
                            entity.insert(PhotomodeButton { tag: 11 });
                        }
                    }
                    PhotomodeBack => {
                        entity.insert(PhotomodeButton { tag: 17 });
                    }
                    _ => {
                        entity.insert(UiLocked);
                    }
                }
                entity.insert(ShowOnlyInPhotomode);
            },
            (category == ToolbarCategory::PhotomodeTakePic).then_some(1.25),
            true,
        );
        layout.add_contextual_menu(taffy, category as u8, items, commands);
    }
    let ids: Vec<_> = layout
        .menus
        .iter()
        .map(|row| (row.parent_entity, row.parent_entity.to_bits()))
        .collect();
    layout.link_up_cyclic_list_nav(&ids, alignment.is_horizontal(), commands);
    PhotomodeToolbarLayout {
        menus: layout.menus,
        node_a: layout.node_a,
        node_b: layout.node_b,
        scale_progress: layout.scale_progress,
        hide_progress: layout.hide_progress,
    }
}

#[cfg(test)]
#[path = "setup_photomode_toolbar_tests.rs"]
mod setup_tests;
