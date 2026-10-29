//! 原 B 140a64a40：半径加0.23的局部圆弧，以 eased 在0.75两侧展开。
use super::{
    super::ui_edit_circle::AnimatedCircleSignifierResource, edit_arrow::EditCircleSignifier,
};
use crate::{
    asset_libraries::{Handle, mesh_library::AssetMeshLibrary},
    render::{mesh::Mesh, pass_builder::draw_list::DrawTopology},
    resources::walls::WallId,
    systems::ui::signifiers::{
        SignifierAnimation, SignifierEntityBuilder, SignifierFactory, UiSignifierStream,
    },
    utils::geo::update_curve_mesh,
};
use bevy_ecs::prelude::*;
use glam::{Vec2, Vec3};
use std::sync::Arc;
use utils::geometry::Circle2d;

/// 原40字节载荷：局部圆16B、墙身份8B、曲线更新附加量16B。
#[derive(Component, Clone, Copy, Debug)]
pub struct SignifierEditCircleShapeLine {
    pub circle: Circle2d,
    pub wall: WallId,
    pub offset: Vec3,
    pub reserved: f32,
}

fn local_circle(radius: f32) -> Circle2d {
    let mut circle = Circle2d::new(Vec2::ZERO, radius + 0.23);
    circle.center -= circle.get_pos_at_u(0.75);
    circle
}
fn arc_points(circle: Circle2d, eased: f32) -> Vec<Vec3> {
    // 142925984的原值为0.1，0.23是半径扩张，均不是角度或90度比例。
    let half = eased * 0.1 + (1.0 - eased) * 0.01;
    circle
        .as_curve2_range(0.2, 0.75 - half, 0.75 + half)
        .points
        .into_iter()
        .map(|p| Vec3::new(p.x, 0.0, p.y))
        .collect()
}

pub fn signifier_edit_circle_line(
    mut query: Query<(
        &mut SignifierEditCircleShapeLine,
        &Handle<Mesh>,
        &SignifierAnimation,
    )>,
    subject: Res<AnimatedCircleSignifierResource>,
    mut meshes: ResMut<AssetMeshLibrary>,
    mut stream: ResMut<UiSignifierStream>,
) {
    if subject.dragging == Some(false) {
        let circle = local_circle(subject.circle.radius);
        let wall = subject.wall;
        let factory = SignifierFactory::from_closure(
            &format!("edit_circle_WidgetId {{ hash: {} }}_line", subject.widget.0),
            Arc::new(move |meshes, shaders, commands| {
                let entity = SignifierEntityBuilder::new(
                    crate::systems::ui::signifiers::signifier_factory::SignifierMesh::Empty,
                )
                .disable_default_animation()
                .animation_duration(0.3)
                .build(meshes, shaders, commands);
                commands.entity(entity).insert((
                    DrawTopology::Alt(3.0),
                    SignifierEditCircleShapeLine {
                        circle,
                        wall,
                        offset: Vec3::ZERO,
                        reserved: 0.0,
                    },
                    EditCircleSignifier,
                ));
                vec![entity]
            }),
        );
        stream.write_factory(&factory);
    }
    // 退出活动状态仍更新现有圆弧，使卸载动画可正常缩回。
    for (mut line, mesh, animation) in &mut query {
        if subject.dragging.is_some() {
            line.circle = local_circle(subject.circle.radius);
        }
        update_curve_mesh(
            meshes.get_mut(*mesh),
            arc_points(line.circle, animation.eased),
        );
    }
}
