//! B toolbar::menu::splash::ui141ec1390 and its closure140f99420.
//! Uses the original four named logo textures, not a reconstruction demo UI.
use crate::mei::{
    core::{FrameEvent, FrameKind, GlobalId, MeiContext, NodeContext},
    layout::{Justify, LayoutStyle, LengthPercentage},
};
use bevy_ecs::prelude::{Local, Res, ResMut};
use country_core::resources::{logo_animation::LogoAnimation, time::GameTime};

/// Registered by SplashScreenPlugin at original Update GameSystemSet36.
pub fn ui(
    mut mei: ResMut<MeiContext>,
    mut elapsed: Local<f32>,
    time: Res<GameTime>,
    logo: Res<LogoAnimation>,
) {
    let opacity = 1. - (logo.progress[4] / logo.durations[4]).clamp(0., 1.);
    draw_splash(&mut mei, opacity);
    *elapsed += time.delta;
}
fn draw_splash(mei: &mut MeiContext, opacity: f32) {
    let id = GlobalId::new("logos");
    let existed = mei.nodes.contains_key(&id);
    mei.reuse_or_create_node(id).style.align = 0;
    if !existed {
        mei.frame_events.push(FrameEvent {
            kind: FrameKind::FullWindow,
            id,
        });
    }
    let root = NodeContext::new(mei, id);
    root.layout(LayoutStyle {
        padding: [LengthPercentage::absolute(50.); 4],
        justify: Justify(3),
        align: 3,
        ..Default::default()
    });
    root.node_mut().local_opacity = opacity;
    let devs = root.child("devs");
    devs.layout(LayoutStyle::default());
    devs.image("pounce light", "menu/pounce_light_logo", 150.)
        .ignore_input();
    devs.child("pad0").layout(LayoutStyle {
        size: LengthPercentage::absolute(20.),
        ..Default::default()
    });
    devs.image("heart", "menu/heart", 75.).ignore_input();
    devs.child("pad1").layout(LayoutStyle {
        size: LengthPercentage::absolute(10.),
        ..Default::default()
    });
    devs.image("noknok", "menu/noknok_logo", 120.)
        .ignore_input();
    root.image("fmod", "menu/fmod_logo", 60.).ignore_input();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mei::widgets::ImageContent;
    #[test]
    fn splash_produces_original_assets_and_retains_exact_layout() {
        let mut mei = MeiContext::default();
        draw_splash(&mut mei, 0.4);
        let root = mei.nodes.get(&GlobalId::new("logos")).unwrap();
        assert_eq!(root.children.len(), 2);
        assert_eq!(root.style.justify.0, 3);
        assert_eq!(root.style.align, 3);
        assert_eq!(root.local_opacity, 0.4);
        assert_eq!(root.style.padding[0].value, 50.);
        let mut images: Vec<_> = mei
            .nodes
            .values()
            .filter_map(|node| {
                let any: &dyn std::any::Any = node.content.as_ref();
                any.downcast_ref::<ImageContent>()
                    .map(|image| (image.image.as_str(), image.height))
            })
            .collect();
        images.sort_by(|a, b| a.0.cmp(b.0));
        assert_eq!(
            images,
            vec![
                ("menu/fmod_logo", 60.),
                ("menu/heart", 75.),
                ("menu/noknok_logo", 120.),
                ("menu/pounce_light_logo", 150.)
            ]
        );
        assert_eq!(mei.frame_events.len(), 1);
        mei.recycle_frame_nodes();
        draw_splash(&mut mei, 0.2);
        assert_eq!(mei.nodes.len(), 8);
        assert!(mei.previous_nodes.is_empty());
    }
}
