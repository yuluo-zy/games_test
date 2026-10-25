//! Tests run the real ECS upload systems against the actual mapped SSBO owner.
use crate::{
    butterfly,
    components::{Butterfly, Firefly, Transform},
    firefly,
};
use bevy_ecs::prelude::*;
use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
use country_core::render::ssbo::rhapsody_boundary::{
    BackendError, Buffer, BufferDesc, GraphicsDevice,
};
use country_core::systems::render_loop::resources::critters::{ButterflySsbo, FireflySsbo};
use glam::{Quat, Vec3};
use std::{cell::RefCell, ops::Range};

#[derive(Default)]
struct Device {
    allocations: RefCell<Vec<(*mut u8, usize)>>,
}
impl GraphicsDevice for Device {
    fn create_buffer(
        &self,
        desc: &BufferDesc,
        _: &str,
        data: Option<&[u8]>,
    ) -> Result<Buffer, BackendError> {
        let page = Box::leak(vec![0u128; desc.size.div_ceil(16)].into_boxed_slice());
        let ptr = page.as_mut_ptr().cast::<u8>();
        if let Some(data) = data {
            assert!(data.len() <= desc.size);
            unsafe {
                std::ptr::copy_nonoverlapping(data.as_ptr(), ptr, data.len());
            }
        }
        let mut allocations = self.allocations.borrow_mut();
        allocations.push((ptr, desc.size));
        Ok(Buffer {
            id: allocations.len() as u64,
            generation: 1,
            _gap10: 0,
            byte_size: desc.size as u64,
            _gap20: 0,
        })
    }
    fn update_buffer_bytes(
        &self,
        _: &Buffer,
        _: &[u8],
        _: &[Range<usize>],
        _: usize,
    ) -> Result<(), BackendError> {
        panic!("mapped writes must not use the device update path")
    }
    fn mapped_slice_mut(&self, buffer: &Buffer) -> Option<&mut [u8]> {
        let (ptr, len) = self.allocations.borrow()[buffer.id as usize - 1];
        // Each creation is mapped once by the real MappedStorageBuffer owner.
        Some(unsafe { std::slice::from_raw_parts_mut(ptr, len) })
    }
}

fn run(world: &mut World) {
    let mut schedule = Schedule::default();
    schedule.add_systems((
        butterfly::butterflies_send_to_gpu,
        firefly::fireflies_send_to_gpu,
    ));
    schedule.run(world);
}
fn transform(i: usize) -> Transform {
    Transform {
        translation: Vec3::new(i as f32, 2.0, -3.0),
        rotation: Quat::from_rotation_y(i as f32 * 0.1),
        ..Default::default()
    }
}
fn populate(world: &mut World, butterflies: usize, fireflies: usize) {
    for i in 0..butterflies {
        world.spawn((
            transform(i),
            Butterfly {
                phase_ref: i as f32 * 0.125,
                ..Default::default()
            },
        ));
    }
    for i in 0..fireflies {
        world.spawn((transform(i), Firefly::default()));
    }
}

#[test]
fn actual_query_projection_reaches_real_mapped_buffers_and_empty_keeps_last_values() {
    let mut world = World::new();
    let device = Device::default();
    let buffers = AssetSsboLibrary::new();
    buffers.add_new::<ButterflySsbo>(&device);
    buffers.add_new::<FireflySsbo>(&device);
    world.insert_resource(buffers);
    populate(&mut world, 24, 96);
    run(&mut world);
    let buffers = world.resource::<AssetSsboLibrary>();
    let butterflies = buffers.get::<ButterflySsbo>().unwrap().mapped.to_vec();
    let fireflies = buffers.get::<FireflySsbo>().unwrap().mapped.to_vec();
    for element in &butterflies {
        let i = element.position.x as usize;
        assert!(i < 24);
        assert_eq!(
            *element,
            ButterflySsbo::from_components(
                transform(i).translation,
                transform(i).rotation,
                i as f32 * 0.125
            )
        );
    }
    assert_eq!(butterflies.len(), 24);
    for element in &fireflies {
        let i = element.position.x as usize;
        assert!(i < 96);
        assert_eq!(
            *element,
            FireflySsbo::from_translation(transform(i).translation)
        );
    }
    assert_eq!(fireflies.len(), 96);
    world.clear_entities();
    run(&mut world);
    let buffers = world.resource::<AssetSsboLibrary>();
    assert_eq!(buffers.get::<ButterflySsbo>().unwrap().mapped, butterflies);
    assert_eq!(buffers.get::<FireflySsbo>().unwrap().mapped, fireflies);
}

#[test]
fn missing_buffers_are_allowed_only_after_exact_population_validation() {
    for (butterflies, fireflies) in [(0, 0), (24, 0), (0, 96), (24, 96)] {
        let mut world = World::new();
        world.insert_resource(AssetSsboLibrary::new());
        populate(&mut world, butterflies, fireflies);
        run(&mut world);
    }
    for (butterflies, fireflies) in [(1, 0), (23, 0), (25, 0), (0, 1), (0, 95), (0, 97)] {
        let mut world = World::new();
        world.insert_resource(AssetSsboLibrary::new());
        populate(&mut world, butterflies, fireflies);
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run(&mut world))).is_err()
        );
    }
}

#[test]
fn real_mapped_write_preserves_native_short_mapping_failure() {
    let mut world = World::new();
    let device = Device::default();
    let mut buffers = AssetSsboLibrary::new();
    buffers.add_new::<FireflySsbo>(&device);
    let mapped = &mut buffers.get_mut::<FireflySsbo>().unwrap().mapped;
    *mapped = std::mem::take(mapped).split_at_mut(95).0;
    world.insert_resource(buffers);
    populate(&mut world, 0, 96);
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run(&mut world))).is_err());
}
