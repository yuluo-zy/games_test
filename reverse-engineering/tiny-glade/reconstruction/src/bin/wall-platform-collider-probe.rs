pub use tiny_glade_reconstruction::*;
#[path = "../../building/wall_platform_collision.rs"]
#[allow(dead_code)]
mod collision;
use std::io::{self, BufRead};
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut p = line.split_whitespace();
        let first = p.next().unwrap();
        if first == "policy" {
            let exists = p.next().unwrap() == "1";
            let valid = p.next().unwrap() == "1";
            let old_hash = u64::from_str_radix(p.next().unwrap(), 16).unwrap();
            let new_hash = u64::from_str_radix(p.next().unwrap(), 16).unwrap();
            let old_wall = building_wall_state::WallId(p.next().unwrap().parse().unwrap());
            let new_wall = building_wall_state::WallId(p.next().unwrap().parse().unwrap());
            let mut pose = || collision::ColliderPose {
                translation: std::array::from_fn(|_| {
                    f32::from_bits(u32::from_str_radix(p.next().unwrap(), 16).unwrap())
                }),
                rotation: std::array::from_fn(|_| {
                    f32::from_bits(u32::from_str_radix(p.next().unwrap(), 16).unwrap())
                }),
            };
            let old = pose();
            let new = pose();
            let existing = if exists {
                Some((
                    if valid { Some(old) } else { None },
                    collision::PlatformStairColliderKey { wall: old_wall },
                    old_hash,
                ))
            } else {
                None
            };
            let action = collision::platform_collider_action(
                collision::PlatformStairColliderKey { wall: new_wall },
                existing,
                new,
                new_hash,
            );
            println!(
                "{}",
                match action {
                    collision::ColliderUpsertAction::Insert => 0,
                    collision::ColliderUpsertAction::Replace => 1,
                    collision::ColliderUpsertAction::Keep => 2,
                }
            );
            continue;
        }
        let n: usize = first.parse().unwrap();
        let nt: usize = p.next().unwrap().parse().unwrap();
        let na: i64 = p.next().unwrap().parse().unwrap();
        let nl: i64 = p.next().unwrap().parse().unwrap();
        let points = (0..n)
            .map(|_| {
                std::array::from_fn(|_| {
                    f32::from_bits(u32::from_str_radix(p.next().unwrap(), 16).unwrap())
                })
            })
            .collect::<Vec<_>>();
        let tris = (0..nt)
            .map(|_| std::array::from_fn(|_| p.next().unwrap().parse::<u32>().unwrap()))
            .collect::<Vec<_>>();
        let aux = (0..na.max(0))
            .map(|_| {
                std::array::from_fn(|_| f32::from_bits(p.next().unwrap().parse::<u32>().unwrap()))
            })
            .collect::<Vec<_>>();
        let lines = (0..nl.max(0))
            .map(|_| {
                std::array::from_fn(|_| f32::from_bits(p.next().unwrap().parse::<u32>().unwrap()))
            })
            .collect::<Vec<_>>();
        println!(
            "{:016x}",
            collision::indexed_mesh_hash(
                &points,
                &tris,
                if na < 0 { None } else { Some(&aux) },
                if nl < 0 { None } else { Some(&lines) }
            )
        );
    }
}
