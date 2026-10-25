pub use tiny_glade_reconstruction::*;
#[path = "../../building/wall_platform_rapier_backend.rs"]
#[allow(dead_code)]
mod backend;
use backend::*;
use building_wall_collection::TerrainHeightGrid512;
use building_wall_platform_collision::*;
use building_wall_platform_stairs::*;
use building_wall_state::WallId;
use std::{
    io::{self, BufRead},
    sync::Arc,
};
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut p = line.split_whitespace();
        let wall = WallId(p.next().unwrap().parse().unwrap());
        let parse = |s: &str| f32::from_bits(u32::from_str_radix(s, 16).unwrap());
        let height = parse(p.next().unwrap());
        let extra = parse(p.next().unwrap());
        let positions = p
            .collect::<Vec<_>>()
            .chunks_exact(3)
            .map(|v| [parse(v[0]), parse(v[1]), parse(v[2])])
            .collect::<Vec<_>>();
        let terrain = TerrainHeightGrid512 {
            encoded: Arc::from(vec![0u16; 512 * 512]),
            minimum_y: 0.,
            maximum_y: 3.,
            sampling_fix: false,
        };
        let cell = platform_stair_cell(WallPlatformStairCellSource {
            prepared_positions: &positions,
            coordinates: [(0, 0.), (0, 1.)],
            maximum_y: height,
            constrained_extra: extra,
            terrain: &terrain,
            index_offset: 0,
        })
        .unwrap();
        let geometry = WallPlatformStairGeometry {
            cells: vec![cell],
            stairs_length: 1.,
        };
        let raster = std::rc::Rc::new(std::cell::RefCell::new(Vec::<[f32; 4]>::new()));
        let sink = raster.clone();
        let mut world = PlatformStairColliderWorld::new(PlatformRapierBackend::new(move |rect| {
            sink.borrow_mut().push(rect)
        }));
        let a = world.update(wall, &geometry).unwrap();
        let h = match a {
            PlatformColliderUpdate::Inserted { current } => current,
            _ => panic!("insert"),
        };
        assert!(world.backend.colliders[h.collider].is_sensor());
        assert_eq!(h.subworld, 7);
        assert_eq!(world.backend.pose(&h), Ok(ColliderPose::IDENTITY));
        assert!(
            matches!(world.update(wall,&geometry).unwrap(),PlatformColliderUpdate::Kept{current}if current==h)
        );
        let mesh = world.colliders[&wall].mesh.clone();
        let center = mesh.positions.iter().fold([0.; 3], |mut s, x| {
            for i in 0..3 {
                s[i] += x[i] / mesh.positions.len() as f32;
            }
            s
        });
        world.backend.flush_queries();
        let hit = world
            .backend
            .cast_ray(
                [center[0], height + 10., center[2]],
                [0., -1., 0.],
                100.,
                true,
            )
            .unwrap();
        assert_eq!(hit.wall, wall);
        let mut changed = geometry.clone();
        for v in &mut changed.cells[0].vertices {
            v[1] += 0.2;
        }
        let r = world.update(wall, &changed).unwrap();
        let new = match r {
            PlatformColliderUpdate::Replaced { previous, current } => {
                assert_eq!(previous, h);
                current
            }
            _ => panic!("replace"),
        };
        assert_ne!(h, new);
        assert!(world.backend.colliders.get(h.collider).is_none());
        assert!(world.backend.meshes.get(h.mesh).is_none());
        world.backend.flush_queries();
        assert!(
            world
                .backend
                .cast_ray(
                    [center[0], height + 10., center[2]],
                    [0., -1., 0.],
                    100.,
                    true
                )
                .is_some()
        );
        assert!(
            matches!(world.remove(wall).unwrap(),PlatformColliderUpdate::Removed{previous}if previous==new)
        );
        world.backend.flush_queries();
        assert!(
            world
                .backend
                .cast_ray(
                    [center[0], height + 10., center[2]],
                    [0., -1., 0.],
                    100.,
                    true
                )
                .is_none()
        );
        assert!(matches!(
            world.remove(wall).unwrap(),
            PlatformColliderUpdate::Absent
        ));
        let kinds = world
            .backend
            .diffs
            .iter()
            .map(|d| d.original_discriminant().to_string())
            .collect::<Vec<_>>()
            .join(",");
        assert_eq!(kinds, "0,1,0,1");
        assert_eq!(raster.borrow().len(), 4);
        println!(
            "{:016x} {} {} {}",
            mesh.fingerprint,
            kinds,
            raster.borrow().len(),
            hit.time_of_impact.to_bits()
        );
    }
}
