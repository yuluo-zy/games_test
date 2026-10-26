//! Complete original roof perch producer, 14231db27..14231e0a6.
use country_core::resources::roofs::{roof_shape::Roof, roof_ridge::{RoofRidgeDims, calc_roof_tip_center_ls}, roof_noise::apply_roof_noise, geometry::tiles::{random_splits, TileRng}};
use glam::{Vec2, Vec3};
use itertools::Itertools;
use utils::geometry::rectangle::Rectangle2d;
use crate::bird_spots::{BirdSpot, BirdSpotManager};

pub fn sample_roof_bird_spots(roof: &Roof, frame_count: u32, manager: &mut BirdSpotManager) -> Vec<BirdSpot> {
    let bytes = roof.observed();
    let at = |offset| f32::from_le_bytes(bytes[offset..offset+4].try_into().unwrap());
    if u32::from_le_bytes(bytes[8..12].try_into().unwrap()) != 1 || roof.is_flat() {
        return Vec::new();
    }
    let shape = Rectangle2d {
        axis: Vec2::new(at(0xc), at(0x10)),
        center: Vec2::new(at(0x14), at(0x18)),
        size: Vec2::new(at(0x1c), at(0x20)),
    };
    let orientation = shape.orientation_mat2();
    let dims = Vec2::from(RoofRidgeDims::from(roof).get());
    // Native arguments XMM1/XMM2 are both +0, not the ridge dimensions.
    let center = Vec2::from(calc_roof_tip_center_ls(roof, [0.0, 0.0]));
    let ridge = Rectangle2d::new_axis_aligned(center, dims);
    let (mut a, mut b) = ridge.get_edge_pts(usize::from(bytes[0x3c] == 0));
    if roof.is_gable() && bytes[0x54] == 0 {
        let delta = a - b;
        let trim = 0.1 / (delta.y * delta.y + delta.x * delta.x).sqrt();
        let keep = 1.0 - trim;
        let old_a = a;
        a = b * trim + old_a * keep;
        b = old_a * trim + b * keep;
    }
    let delta = a - b;
    let length = (delta.y * delta.y + delta.x * delta.x).sqrt();
    if !(length > 0.5) { return Vec::new(); }
    let mut rng = TileRng(utils::rng::init_rng(frame_count));
    let count = ((length / 0.3) as i32).clamp(1, 7) as usize;
    let spread = length / count as f32 + (-0.1);
    // Native MAXSS selects +0 on equality and unordered input.
    let spread = if spread > 0.0 { spread } else { 0.0 };
    let amplitude = (spread + spread) / length;
    let splits = random_splits(count + 1, amplitude, &mut rng);
    let mut points = Vec::with_capacity(count);
    for (left, right) in splits.iter().tuple_windows() {
        let random = rng.next_f32();
        let sample = random * 0.7 + (1.0 - random) * 0.3;
        let t = *right * sample + *left * (1.0 - sample);
        let local = b * t + a * (1.0 - t);
        let world = orientation * local + shape.center;
        let mut y = roof.height() + roof.floor_y();
        apply_roof_noise(world.x, world.y, 1.0, &mut y);
        points.push(BirdSpot { occupant: None, pos: Vec3::new(world.x, y, world.y), id: manager.gen_id() });
    }
    points
}
