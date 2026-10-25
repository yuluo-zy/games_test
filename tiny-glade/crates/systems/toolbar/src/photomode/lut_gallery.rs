//! B141ec0270 LUT browser GUI. The shared scroll content publishes the exact
//! metrics consumed by render_photomode_lut_gallery; controls write the actual
//! image-formation resource, so previews and rendered grading share one owner.
use crate::{
    mei::{core::MeiContext, widgets::PillButtonContent},
    menu::load::ThumbnailScrollContent,
};
use bevy_ecs::prelude::*;
use country_core::{
    game_config::GameConfig,
    resources::{
        grading_lut::GradingLuts,
        gui::{EscMenuState, NextEscMenuState},
        image_formation::ImageFormationConfigs,
    },
    systems::{audio::PlayMenuUiSound, render_loop::lut_gallery::LutGalleryState},
};

pub fn ui(
    luts: Res<GradingLuts>,
    mode: Res<EscMenuState>,
    gallery: Res<LutGalleryState>,
    mut hovered: Local<Option<usize>>,
    mut next: ResMut<NextEscMenuState>,
    mut formation: ResMut<ImageFormationConfigs>,
    mut mei: ResMut<MeiContext>,
    mut sounds: EventWriter<PlayMenuUiSound>,
    config: Res<GameConfig>,
) {
    if config.dev.demo {
        return;
    }
    if !matches!(*mode, EscMenuState::Photomode(true)) {
        *hovered = None;
        return;
    }
    let root = mei.fullscreen("lut browser");
    let scroll = root.thumbnail_scroll("luts", luts.metas.len() + 1, gallery.state.clone());
    let content = scroll.get_content_mut_impl::<ThumbnailScrollContent>(true);
    if !content.initialized {
        content.scroll = (content.gap_fraction / content.cell_fraction + 1.) * 0.;
    }
    content.cell_fraction = f32::from_bits(0x3ed0e560);
    content.gap_fraction = f32::from_bits(0x3c9374bc);
    content.navigation_speed = 2.;
    content.column_major = true;
    content.cells_per_line = 2;
    if content.hovered.is_some() && content.hovered != *hovered {
        sounds.write(PlayMenuUiSound { kind: 0 });
    }
    *hovered = content.hovered;
    if let Some(index) = content.clicked {
        next.set(EscMenuState::Closed);
        sounds.write(PlayMenuUiSound { kind: 2 });
        let selection = if index == 0 {
            let initial = formation
                .get(0)
                .expect("native image-formation initial configuration");
            Some((initial.name.clone(), initial.params_18[0]))
        } else {
            luts.metas
                .get(index - 1)
                .map(|meta| (meta.name.clone(), 1.))
        };
        if let Some((name, strength)) = selection {
            for configuration in formation.iter_non_default_mut() {
                configuration.name.clone_from(&name);
                configuration.params_18[0] = strength;
            }
        }
    }
    let back = mei
        .safe_frame("back", false)
        .template_with_id("menu/back", 0);
    back.with_slot("button", |slot| {
        let button = slot.back_button("menu/back");
        button
            .get_content_mut_impl::<PillButtonContent>(true)
            .action = 0x12;
        if button.node().used() {
            next.set(EscMenuState::Closed);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mei::{
        core::{GlobalId, NodeContext},
        widgets::DefaultNodeContent,
    };
    #[test]
    fn real_gallery_ui_uses_shared_metrics_and_writes_both_editable_configs() {
        let mut world = World::new();
        let mut mei = MeiContext::default();
        let node = NodeContext::new(&mut mei, GlobalId(0));
        // Test invokes the original UI via a schedule, including template and
        // retained clicked content. This never substitutes render metrics.
        let _ = node;
        let root = mei.fullscreen("lut browser");
        let gallery = LutGalleryState::default();
        let scroll = root.thumbnail_scroll("luts", 1, gallery.state.clone());
        scroll
            .get_content_mut_impl::<ThumbnailScrollContent>(true)
            .clicked = Some(0);
        let template = crate::mei::config::NodeConfig {
            own: crate::mei::config::NodeOwnConfig {
                slot: Some("button".into()),
                ..Default::default()
            },
            ..Default::default()
        };
        mei.configs
            .insert("menu/back".into(), std::sync::Arc::new(template));
        world.insert_resource(mei);
        world.insert_resource(gallery);
        world.insert_resource(GradingLuts::default());
        world.insert_resource(EscMenuState::Photomode(true));
        world.insert_resource(NextEscMenuState::default());
        let mut formation = ImageFormationConfigs::default();
        formation.configs[0].name = "original-default".into();
        formation.configs[0].params_18[0] = 0.625;
        world.insert_resource(formation);
        world.insert_resource(GameConfig::default());
        world.init_resource::<Events<PlayMenuUiSound>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(ui);
        schedule.run(&mut world);
        let formation = world.resource::<ImageFormationConfigs>();
        assert!(
            formation
                .configs
                .iter()
                .all(|c| c.name == "original-default" && c.params_18[0] == 0.625)
        );
        assert!(matches!(
            world.resource::<NextEscMenuState>().0,
            Some(EscMenuState::Closed)
        ));
        let mei = world.resource::<MeiContext>();
        let contents: Vec<_> = mei
            .nodes
            .values()
            .filter_map(|node| {
                let any: &dyn std::any::Any = node.content.as_ref();
                any.downcast_ref::<ThumbnailScrollContent>()
            })
            .collect();
        assert_eq!(contents.len(), 1);
        assert!(std::sync::Arc::ptr_eq(
            &contents[0].metrics,
            &world.resource::<LutGalleryState>().state
        ));
        assert_eq!(contents[0].cells_per_line, 2);
        assert_eq!(contents[0].count, 1);
        let _ = ThumbnailScrollContent::from_font_table(std::sync::Arc::new(
            tg_text::FontTable::default(),
        ));
    }
}
