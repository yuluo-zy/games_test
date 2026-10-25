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
    let target = GraphicsDevice::create_buffer(
        canonical.device(),
        &BufferDesc::new(8, 0x22, 2),
        "core bridge target",
        None,
    )
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

#[test]
fn built_graph_lease_survives_rejected_rebuild_and_releases_on_compiled_drop() {
    let Some(mut bridge) = bridge() else { return };
    let retired = Rc::new(Cell::new(0));
    let flag = retired.clone();
    let (_, built) = bridge
        .build_frame(|frame| {
            frame.with_graph(|graph| {
                graph
                    .add_pass("original pending declaration")
                    .with_side_effect()
                    .render(|_| true);
                graph.on_retire(move || flag.set(flag.get() + 1));
                Ok(())
            })
        })
        .unwrap();
    let invoked = Cell::new(false);
    assert!(
        bridge
            .build_frame(|_| {
                invoked.set(true);
                Ok(())
            })
            .is_err()
    );
    assert!(
        !invoked.get(),
        "rejected begin must not invoke another builder"
    );
    assert_eq!(retired.get(), 0, "the first declaration must remain live");
    let compiled = bridge.compile_built(built).unwrap();
    assert_eq!(
        compiled.snapshot().pass_order,
        ["original pending declaration"]
    );
    assert!(
        bridge.build_frame(|_| Ok(())).is_err(),
        "prepared frame still owns its session"
    );
    drop(compiled);
    assert_eq!(retired.get(), 1);
    let (_, retry) = bridge.build_frame(|_| Ok(())).unwrap();
    bridge.cancel_built(retry).unwrap();
    let (_, retry) = bridge.build_frame(|_| Ok(())).unwrap();
    bridge.cancel_built(retry).unwrap();
}

#[test]
fn foreign_built_graph_retires_the_originating_lease_without_submitting() {
    let Some(mut origin) = bridge() else { return };
    let mut other = CoreRenderBridge::from_owners(origin.warmup_owners().unwrap()).unwrap();
    let retired = Rc::new(Cell::new(false));
    let flag = retired.clone();
    let (_, foreign) = origin
        .build_frame(|frame| {
            frame.with_graph(|graph| {
                graph
                    .add_pass("foreign work")
                    .with_side_effect()
                    .render(|_| {
                        panic!("foreign graph must never record");
                    });
                graph.on_retire(move || flag.set(true));
                Ok(())
            })
        })
        .unwrap();
    let error = other.execute_built(foreign).unwrap_err();
    assert_eq!(error.kind(), rhapsody::ErrorKind::InvalidUsage);
    assert!(retired.get());
    assert_eq!(
        other.with_session(|session| session.pending_frame_count()),
        0
    );
    let (_, retry) = origin.build_frame(|_| Ok(())).unwrap();
    origin.cancel_built(retry).unwrap();
    let (_, retry) = other.build_frame(|_| Ok(())).unwrap();
    other.cancel_built(retry).unwrap();
}

#[test]
fn dropped_uncompiled_graph_cancels_imports_and_leaves_the_session_reusable() {
    let Some(mut bridge) = bridge() else { return };
    let buffers = bridge.hosts().buffers.clone();
    let target = buffers
        .create_buffer(&BufferDesc::new(8, 0x22, 2), "abandoned declaration", None)
        .unwrap();
    let retired = Rc::new(Cell::new(0));
    let flag = retired.clone();
    let (_, built) = bridge
        .build_frame(|frame| {
            frame.import_buffer(&target, 0)?;
            frame.with_graph(|graph| {
                graph.on_retire(move || flag.set(flag.get() + 1));
                Ok(())
            })
        })
        .unwrap();
    assert_eq!(
        bridge.with_session(|session| session.frame_import_count()),
        1
    );
    assert_eq!(retired.get(), 0);
    drop(built);
    assert_eq!(retired.get(), 1);
    assert_eq!(
        bridge.with_session(|session| session.frame_import_count()),
        0
    );
    assert_eq!(
        bridge.with_session(|session| session.pending_frame_count()),
        0
    );
    let (_, retry) = bridge.build_frame(|_| Ok(())).unwrap();
    bridge.cancel_built(retry).unwrap();
    assert!(buffers.release(&target));
}

#[test]
fn compute_entry_points_share_app_constants_keep_cancelled_chunks_and_isolate_other_apps() {
    use country_core::render::rg_helpers::rg_boundary::{RenderGraph as _, StagingAlloc};
    use rhapsody::resource::DynamicConstantsChunk;
    fn chunk(staging: &StagingAlloc) -> &DynamicConstantsChunk {
        staging.lease.as_ref().unwrap().downcast_ref().unwrap()
    }
    let Some(mut bridge) = bridge() else { return };
    let raster = bridge.raster_resources().clone();
    let before = raster.dynamic_constants().lock().stats();
    let ((first, second), built) = bridge
        .build_frame(|frame| {
            let first = frame.with_compute(|compute| compute.allocate_dynamic(64))?;
            let second = frame.with_compute(|compute| compute.allocate_dynamic(64))?;
            Ok((first, second))
        })
        .unwrap();
    let after = raster.dynamic_constants().lock().stats();
    assert_eq!(
        after.allocations - before.allocations,
        2,
        "both actual with_compute entries must allocate from this App owner"
    );
    assert_eq!(after.page_creations - before.page_creations, 1);
    assert!(Arc::ptr_eq(&chunk(&first).buffer, &chunk(&second).buffer));
    assert!(chunk(&first).offset + chunk(&first).size <= chunk(&second).offset);
    chunk(&first).write_bytes(&[0x35; 64]).unwrap();
    chunk(&second).write_bytes(&[0xa7; 64]).unwrap();
    bridge.cancel_built(built).unwrap();
    // 取消只释放图 owner，仍被 CPU 租约保留的内容不能被下一入口覆盖。
    let (third, built) = bridge
        .build_frame(|frame| frame.with_compute(|compute| compute.allocate_dynamic(64)))
        .unwrap();
    assert!(Arc::ptr_eq(&chunk(&first).buffer, &chunk(&third).buffer));
    assert!(chunk(&second).offset + chunk(&second).size <= chunk(&third).offset);
    chunk(&third).write_bytes(&[0x59; 64]).unwrap();
    bridge.cancel_built(built).unwrap();
    for (staging, expected) in [(&first, 0x35), (&second, 0xa7), (&third, 0x59)] {
        let chunk = chunk(staging);
        let mut actual = [0; 64];
        chunk.buffer.read_bytes(chunk.offset, &mut actual).unwrap();
        assert_eq!(actual, [expected; 64]);
    }
    let device = bridge.hosts().buffers.graphics_device().clone();
    let mut other =
        CoreRenderBridge::new(device, ".", Arc::new(AssetShaderLibrary::new())).unwrap();
    let (other_chunk, built) = other
        .build_frame(|frame| frame.with_compute(|compute| compute.allocate_dynamic(64)))
        .unwrap();
    assert!(
        !Arc::ptr_eq(&chunk(&first).buffer, &chunk(&other_chunk).buffer),
        "different Apps on one Device must retain distinct constants owners"
    );
    assert_eq!(
        raster.dynamic_constants().lock().stats().allocations - before.allocations,
        3
    );
    assert_eq!(
        other
            .raster_resources()
            .dynamic_constants()
            .lock()
            .stats()
            .allocations,
        1
    );
    other.cancel_built(built).unwrap();
}

#[test]
fn completed_old_frame_collection_preserves_new_compiled_imports_and_callbacks() {
    let Some(mut bridge) = bridge() else { return };
    let buffers = bridge.hosts().buffers.clone();
    let device = bridge.with_session(|session| session.backend.device.clone());
    let mut world = bevy_ecs::world::World::new();
    bridge.install_asset_libraries(&mut world).unwrap();
    let canonical = world.resource::<country_core::render::device::GraphicsDeviceResource>();
    let target = GraphicsDevice::create_buffer(
        canonical.device(), &BufferDesc::new(8, 0x22, 2), "compiled maintenance target", None,
    ).unwrap();
    let first_retired = Rc::new(Cell::new(false));
    let first_flag = first_retired.clone();
    let (_, first) = bridge.render_frame(|frame| {
        let id = frame.import_buffer(&target, 0)?;
        frame.with_compute(|compute| {
            rg_buffer_copy_from_slice_u32(compute, id.raw(), 8, &[11, 12], UploadContentVersion::Unknown)
        })?;
        frame.with_graph(|graph| {
            graph.export(id, 0x26)?;
            graph.on_retire(move || first_flag.set(true));
            Ok(())
        })
    }).unwrap();
    // 仅测试先确认全部真实回执完成，故意不走会 finish 旧帧的 session wait。
    let mut completion = rhapsody::sync::SyncSet::EMPTY;
    for point in &first.receipts {
        completion.insert(*point);
    }
    assert!(!completion.is_empty());
    device.wait(&completion, 10_000_000_000).unwrap();
    assert!(completion.is_reached(&device.poll().unwrap()));
    assert!(!first_retired.get());
    assert_eq!(bridge.with_session(|session| session.pending_frame_count()), 1);

    let second_retired = Rc::new(Cell::new(false));
    let second_flag = second_retired.clone();
    let (_, built) = bridge.build_frame(|frame| {
        let id = frame.import_buffer(&target, 0)?;
        frame.with_compute(|compute| {
            rg_buffer_copy_from_slice_u32(compute, id.raw(), 8, &[21, 22], UploadContentVersion::Unknown)
        })?;
        frame.with_graph(|graph| {
            graph.export(id, 0x26)?;
            graph.on_retire(move || second_flag.set(true));
            Ok(())
        })
    }).unwrap();
    let compiled = bridge.compile_built(built).unwrap();
    assert!(!first_retired.get(), "compile 不代替旧帧 finish");
    assert!(!second_retired.get());
    let imports_before = bridge.with_session(|session| session.frame_import_count());
    assert!(imports_before > 0);
    assert_eq!(bridge.collect_completed_frames().unwrap(), 1);
    assert!(first_retired.get());
    assert!(!second_retired.get(), "未提交 compiled 的 retire 回调不能被旧帧维护触发");
    let imports_after = bridge.with_session(|session| session.frame_import_count());
    assert!(imports_after > 0 && imports_after < imports_before,
        "只清旧 graph marker，新 compiled 的导入必须保留");
    let second = bridge.execute_compiled(compiled).unwrap();
    assert!(!second.receipts.is_empty());
    bridge.wait_submits_done(10_000_000_000).unwrap();
    assert!(second_retired.get());
    let expected: Vec<u8> = [21_u32, 22].into_iter().flat_map(u32::to_ne_bytes).collect();
    assert_eq!(&buffers.mapped_slice_mut(&target).unwrap()[..8], expected);
    GraphicsDevice::release_buffer(canonical.device(), &target);
}
