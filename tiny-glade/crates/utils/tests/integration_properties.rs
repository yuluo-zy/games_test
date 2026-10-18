//! Deterministic differential and state-machine tests across utility consumers.
#[path = "support/spline_reference.rs"]
mod spline_reference;

use glam::{Vec2, Vec3};
use spline_reference::ReferenceSpline;
use std::{
    collections::{BTreeMap, HashSet},
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};
use utils::{
    float_grid::{ByteFloatGrid, FloatGridSampler},
    index_condenser::IndexCondenser,
    natural_cubic::Spline3D,
};

#[test]
fn spline_constructors_match_independent_dense_f64_solver() {
    let rng = fastrand::Rng::with_seed(0x62ac_0468_c7d0_1273);
    for case in 0..120 {
        let n = 3 + case % 9;
        let points: Vec<_> = (0..n)
            .map(|i| {
                Vec3::new(
                    i as f32 * 1.5 + rng.f32(),
                    rng.f32() * 8.0 - 4.0,
                    rng.f32() * 8.0 - 4.0,
                )
            })
            .collect();
        let periodic = case % 2 == 0;
        let start = (case % 3 == 0).then_some(Vec3::new(0.5, -0.25, 0.75));
        let end = (case % 5 == 0).then_some(Vec3::new(-0.25, 0.75, 0.5));
        let actual = if periodic {
            Spline3D::try_from_periodic_loop(&points).unwrap()
        } else {
            Spline3D::try_from_control_points_with_tangents(&points, start, end).unwrap()
        };
        let reference = ReferenceSpline::new(&points, periodic, start, end);
        assert_eq!(actual.segments.len(), reference.lengths.len());
        for (i, &span) in reference.lengths.iter().enumerate() {
            for fraction in [0.0, 0.125, 0.375, 0.75, 1.0] {
                let t = span * fraction;
                let expected = reference.position(i, t);
                let observed = actual.segments[i].position_at_local_t(t as f32);
                assert!(
                    (observed - expected).length() < 2e-4 * (1.0 + expected.length()),
                    "case={case} segment={i} fraction={fraction}, actual={observed:?}, expected={expected:?}"
                );
                let expected = reference.tangent(i, t);
                let observed = actual.segments[i].tangent_at_local_t(t as f32);
                assert!(
                    (observed - expected).length() < 4e-4 * (1.0 + expected.length()),
                    "tangent case={case} segment={i} fraction={fraction}"
                );
            }
        }
    }
}

#[test]
fn dense_index_handles_keep_parallel_payloads_consistent_through_random_edits() {
    let rng = fastrand::Rng::with_seed(0x731a_950f_d24e_185b);
    let mut indices = IndexCondenser::<u32>::default();
    let mut handles = BTreeMap::new();
    let mut payloads = Vec::<(u32, u64)>::new();
    let mut expected = BTreeMap::new();
    for step in 0..6_000 {
        let key = rng.u32(0..128);
        match rng.u8(0..20) {
            0 => {
                indices.clear();
                payloads.clear();
                expected.clear();
            }
            1..=8 => {
                if let Some(handle) = handles.get(&key) {
                    if let Some(index) = indices.remove_index(handle) {
                        assert_eq!(payloads.swap_remove(index as usize).0, key);
                        assert!(expected.remove(&key).is_some());
                    } else {
                        assert!(!expected.contains_key(&key));
                    }
                }
            }
            _ => {
                if let Some(handle) = indices.insert(key) {
                    let value = rng.u64(..);
                    assert_eq!(handle.last_known_index() as usize, payloads.len());
                    assert!(expected.insert(key, value).is_none());
                    handles.insert(key, handle);
                    payloads.push((key, value));
                } else {
                    assert!(expected.contains_key(&key));
                }
            }
        }
        assert_eq!(indices.len(), expected.len(), "step={step}");
        assert_eq!(indices.keys().len(), payloads.len());
        for (index, &(key, value)) in payloads.iter().enumerate() {
            assert_eq!(indices.keys()[index], key);
            assert_eq!(expected.get(&key), Some(&value));
            assert_eq!(indices.get(&handles[&key]), Some(index as u32));
        }
        for (key, handle) in &handles {
            assert_eq!(indices.get(handle).is_some(), expected.contains_key(key));
        }
    }
}

#[test]
fn both_grid_sampling_policies_match_f64_weighted_cell_reference() {
    let rng = fastrand::Rng::with_seed(0xd86e_073f_2109_ab32);
    for width in [2, 3, 7, 16, 31] {
        for height in [2, 4, 9, 17] {
            let values: Vec<_> = (0..width * height).map(|_| rng.u8(..)).collect();
            let grid = ByteFloatGrid {
                values: &values,
                width,
                height,
            };
            for _ in 0..80 {
                let world = Vec2::new(2.0 + rng.f32() * 100.0, 2.0 + rng.f32() * 100.0);
                let query = world * Vec2::new(rng.f32() * 3.0 - 1.5, rng.f32() * 3.0 - 1.5);
                for centered in [false, true] {
                    let axis = |value: f32, extent: f32, n: usize| {
                        let normalized = f64::from(value) / f64::from(extent) + 0.5;
                        let position = if centered {
                            (normalized.clamp(0.0, 1.0) * n as f64 - 0.5).max(0.0)
                        } else {
                            normalized.clamp(0.0, 1.0) * n as f64
                        };
                        let lower =
                            (position.floor() as usize).min(n - if centered { 1 } else { 2 });
                        (lower, (lower + 1).min(n - 1), position - lower as f64)
                    };
                    let (x0, x1, tx) = axis(query.x, world.x, width);
                    let (y0, y1, ty) = axis(query.y, world.y, height);
                    let cell = |x, y| f64::from(values[y * width + x]) / 255.0;
                    let expected = cell(x0, y0) * (1.0 - tx) * (1.0 - ty)
                        + cell(x1, y0) * tx * (1.0 - ty)
                        + cell(x0, y1) * (1.0 - tx) * ty
                        + cell(x1, y1) * tx * ty;
                    let actual = grid.bilinear_sample_at_world_xz(query, world, centered);
                    assert!(
                        (f64::from(actual) - expected).abs() < 1e-5,
                        "size={width}x{height} centered={centered} query={query:?} expected={expected} actual={actual}"
                    );
                }
            }
        }
    }
}

static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);
struct Directory(PathBuf);
impl Directory {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "utils-complex-backups-{}-{}",
            std::process::id(),
            NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Directory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn concurrent_public_backup_calls_preserve_every_copy_and_source() {
    let directory = Directory::new();
    let path = directory.0.join("session.ron");
    let data: Vec<u8> = (0..16_384).map(|i| (i * 37 + i / 31) as u8).collect();
    std::fs::write(&path, &data).unwrap();
    std::thread::scope(|scope| {
        let mut joins = Vec::new();
        for _ in 0..4 {
            let path = &path;
            joins.push(scope.spawn(move || {
                for _ in 0..8 {
                    utils::file::copy_file_to_backup(path).unwrap();
                }
            }));
        }
        for join in joins {
            join.join().unwrap();
        }
    });
    let entries: HashSet<_> = std::fs::read_dir(&directory.0)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    assert_eq!(entries.len(), 33);
    for entry in entries {
        assert_eq!(std::fs::read(entry).unwrap(), data);
    }
}
