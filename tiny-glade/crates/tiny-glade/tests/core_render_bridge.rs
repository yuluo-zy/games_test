//! Integration checks through the public core/rhapsody frame bridge.
//! Set RHAPSODY_REQUIRE_VULKAN=1 to make unavailable GPU execution a failure.
use std::{cell::Cell, rc::Rc, sync::Arc};

use country_core::{
    asset_libraries::shader_library::AssetShaderLibrary,
    render::{
        rg_helpers::{rg_boundary::UploadContentVersion, rg_buffer_copy_from_slice_u32},
        ssbo::rhapsody_boundary::{BufferDesc, GraphicsDevice},
    },
};
use tiny_glade::render_bridge::CoreRenderBridge;

// Diagnostic storage used only to exercise the actual lazy-library boundary;
// this is not a recovered game SSBO or a substitute world producer.
#[repr(transparent)]
#[derive(Clone, Copy, bytemuck::NoUninit)]
struct LazyUsedWord(u32);
impl country_core::render::ssbo::StorageElement for LazyUsedWord {
    const NAME: &'static str = "test lazy used word";
}
impl country_core::asset_libraries::ssbo_library::SsboType for LazyUsedWord {
    type Storage = country_core::render::ssbo::TypedStorageBuffer<Self>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        Self::Storage::new_storage_buffer(device, initial)
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, bytemuck::NoUninit)]
struct LazyUnusedWord(u32);
impl country_core::render::ssbo::StorageElement for LazyUnusedWord {
    const NAME: &'static str = "test lazy unused word";
}
impl country_core::asset_libraries::ssbo_library::SsboType for LazyUnusedWord {
    type Storage = country_core::render::ssbo::TypedStorageBuffer<Self>;
    fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self::Storage {
        Self::Storage::new_storage_buffer(device, initial)
    }
}

fn bridge() -> Option<CoreRenderBridge> {
    let result =
        rhapsody::device::Device::new_headless(&rhapsody::device::DeviceConfig::headless())
            .and_then(|device| {
                CoreRenderBridge::new(device, ".", Arc::new(AssetShaderLibrary::new()))
            });
    match result {
        Ok(bridge) => Some(bridge),
        Err(error) => {
            assert!(
                !std::env::var("RHAPSODY_REQUIRE_VULKAN").is_ok_and(|value| value == "1"),
                "strict core bridge GPU initialization failed: {error}"
            );
            eprintln!("SKIP core bridge GPU initialization: {error}");
            None
        }
    }
}

#[test]
fn core_staging_upload_imports_and_retires_through_one_session() {
    let Some(mut bridge) = bridge() else { return };
    let buffers = bridge.hosts().buffers.clone();
    let mut world = bevy_ecs::world::World::new();
    bridge.install_asset_libraries(&mut world).unwrap();
    let canonical = world.resource::<country_core::render::device::GraphicsDeviceResource>();
    // Exercise the canonical ECS device, not a second host or the old
    // unsupported compatibility implementation. Its IDs must resolve here.
    let target = GraphicsDevice::create_buffer(canonical.device(),
        &BufferDesc::new(8, 0x22, 2), "core bridge target", None)
        .unwrap();
    assert!(buffers.managed_buffer(&target).is_some());
    let retired = Rc::new(Cell::new(false));
    let flag = retired.clone();
    bridge
        .render_frame(|frame| {
            let target_id = frame.import_buffer(&target, 0)?;
            frame.with_compute(|compute| {
                rg_buffer_copy_from_slice_u32(
                    compute,
                    target_id.raw(),
                    8,
                    &[0x1234_5678, 0x90ab_cdef],
                    UploadContentVersion::Unknown,
                )
            })?;
            frame.with_graph(|graph| {
                // Host read is an actual graph export, not a synthetic callback.
                graph.export(target_id, 0x26)?;
                graph.on_retire(move || flag.set(true));
                Ok(())
            })
        })
        .unwrap();
    bridge.wait_submits_done(10_000_000_000).unwrap();
    let expected = [0x1234_5678_u32.to_ne_bytes(), 0x90ab_cdef_u32.to_ne_bytes()].concat();
    assert_eq!(&buffers.mapped_slice_mut(&target).unwrap()[..8], expected);
    assert!(retired.get());
    GraphicsDevice::release_buffer(canonical.device(), &target);
    assert!(buffers.managed_buffer(&target).is_none());
    // A released generation is never admitted as an import.
    assert_eq!(
        bridge
            .render_frame(|frame| frame.import_buffer(&target, 0))
            .unwrap_err()
            .kind(),
        rhapsody::ErrorKind::InvalidUsage
    );
}

#[test]
fn library_ssbo_lazy_import_inside_compute_uses_the_same_frame_and_reimports_next_frame() {
    use country_core::asset_libraries::ssbo_library::AssetSsboLibrary;
    let Some(mut bridge) = bridge() else { return };
    let buffers = bridge.hosts().buffers.clone();
    let library = AssetSsboLibrary::new();
    library.add_new_with_data(buffers.as_ref(), &[LazyUsedWord(0), LazyUsedWord(0)]);
    library.add_new_with_data(buffers.as_ref(), &[LazyUnusedWord(99)]);
    let used = *library.get_buffer::<LazyUsedWord>().unwrap();
    let unused = *library.get_buffer::<LazyUnusedWord>().unwrap();
    let mut previous_generation = None;
    for words in [[0x1234_5678, 0x90ab_cdef], [0xfedc_ba98, 0x7654_3210]] {
        let (imported, receipt) =
            bridge
                .render_frame(|frame| {
                    let mut resource_graph = frame.resource_graph();
                    let imported = frame.with_compute(|compute| {
                        // This call was previously made while CoreGraphFrame held the
                        // session RefMut for the entire closure, making the real lazy
                        // import panic on its second borrow.
                        let imported = library.get_rg_buffer::<LazyUsedWord>(&mut resource_graph);
                        assert_eq!(
                            library.get_rg_buffer::<LazyUsedWord>(&mut resource_graph),
                            imported
                        );
                        rg_buffer_copy_from_slice_u32(
                            compute,
                            imported.resource as u32,
                            8,
                            &words,
                            UploadContentVersion::Unknown,
                        )?;
                        Ok(imported)
                    })?;
                    frame.with_graph(|graph| {
                assert_eq!(graph.declaration().resources.len(), 2,
                    "only the requested SSBO and the actual upload staging buffer are imported");
                Ok(())
            })?;
                    library.export_from_rg(&mut resource_graph);
                    drop(resource_graph);
                    Ok(imported)
                })
                .unwrap();
        assert!(!receipt.receipts.is_empty());
        if let Some(previous) = previous_generation {
            assert_ne!(
                imported.generation, previous,
                "the same physical SSBO must be reimported in the next graph scope"
            );
        }
        previous_generation = Some(imported.generation);
        bridge.wait_submits_done(10_000_000_000).unwrap();
        let expected: Vec<u8> = words.into_iter().flat_map(u32::to_ne_bytes).collect();
        assert_eq!(&buffers.mapped_slice_mut(&used).unwrap()[..8], expected);
        assert_eq!(
            &buffers.mapped_slice_mut(&unused).unwrap()[..4],
            99_u32.to_ne_bytes()
        );
    }
    drop(library);
    assert!(buffers.release(&used));
    assert!(buffers.release(&unused));
}

#[test]
fn failed_ignored_and_unwound_builders_retire_without_submission() {
    let Some(mut bridge) = bridge() else { return };
    let retired = Rc::new(Cell::new(0));
    let flag = retired.clone();
    let error = bridge
        .render_frame::<()>(|frame| {
            frame.with_graph(|graph| {
                graph.on_retire(move || flag.set(flag.get() + 1));
                Ok(())
            })?;
            Err(rhapsody::Error::Timeout)
        })
        .unwrap_err();
    assert_eq!(error.kind(), rhapsody::ErrorKind::Timeout);
    assert_eq!(retired.get(), 1);
    let flag = retired.clone();
    let error = bridge
        .render_frame(|frame| {
            // Even if a game callback ignores a failed phase, no prefix submits.
            let _ = frame.with_graph::<()>(|graph| {
                graph.on_retire(move || flag.set(flag.get() + 1));
                Err(rhapsody::Error::OutOfHostMemory)
            });
            Ok(())
        })
        .unwrap_err();
    assert_eq!(error.kind(), rhapsody::ErrorKind::InvalidUsage);
    assert_eq!(retired.get(), 2);
    let flag = retired.clone();
    let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = bridge.render_frame::<()>(|frame| {
            frame.with_graph(|graph| {
                graph.on_retire(move || flag.set(flag.get() + 1));
                Ok(())
            })?;
            panic!("game builder panic");
        });
    }));
    assert!(panic.is_err());
    assert_eq!(retired.get(), 3);
    // Cancellation must leave the same session reusable.
    bridge.render_frame(|_| Ok(())).unwrap();
}
