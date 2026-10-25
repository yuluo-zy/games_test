
/// Bounded original W1 -> W2/W12 consumer proof. W3..W11 are not synthesized.
/// Constant nonzero rasters expose accidental pruning of their uploaded red
/// channel and loss of the water temporary's live G channel on the third pass.
#[test]
fn live_water_and_garden_masks_consume_uploads_keep_channels_and_skip_clean_frames() {
    use bevy_ecs::change_detection::DetectChanges;
    let Some(mut fixture) = Fixture::new() else { return; };
    let buffers = fixture.bridge.hosts().buffers.clone();
    macro_rules! constant_raster {
        ($r:ty, $s:ty, $value:expr) => {{
            fixture.world.resource::<AssetSsboLibrary>().add_new::<$s>(buffers.as_ref());
            fixture.world.insert_resource(<$r>::new(DirtyTilePublisher::unique()));
            fixture.world.resource_mut::<$r>().write_composite(Some(&vec![$value; 512 * 512]));
            fixture.world.resource_scope(|world, mut raster: bevy_ecs::change_detection::Mut<$r>| {
                let mut library = world.resource_mut::<AssetSsboLibrary>();
                let staging = library.get_mut::<$s>().unwrap();
                raster.copy_to_staging(0, bytemuck::cast_slice_mut(staging.mapped));
            });
        }};
    }
    constant_raster!(WaterRaster, WaterRasterStagingSsbo, 73);
    constant_raster!(GardenRaster, GardenRasterStagingSsbo, 149);
    let mut last_changed = None;
    for frame_index in 0..2 {
        fixture.world.increment_change_tick();
        let tick = fixture.world.read_change_tick();
        let mut outputs = Vec::new();
        let (_, built) = fixture.bridge.build_frame(|frame| {
            tiny_glade::frame_setup::with_frame_resources(frame, &mut fixture.world, |frame, world| {
                for slot in [0x13, 0x14] {
                    let result = scene_compute::upload_raster_staging_to_texture(frame, world, slot)?;
                    assert_eq!(result.is_some(), frame_index == 0);
                }
                let water = scene_compute::water_rasterize_from_cpu(frame, world)?;
                let garden = scene_compute::garden_rasterize_from_cpu(frame, world)?;
                assert_eq!(water, (frame_index == 0).then_some(tick.get()));
                assert_eq!(garden, (frame_index == 0).then_some(tick.get()));
                for slot in [0x13, 0x14] {
                    let source = world.resource::<AssetTextureLibrary>().get_rg_texture(TexturePreset(slot));
                    outputs.push(readback(frame, world, &fixture.shaders, &fixture.device,
                        RgTexture { resource: source.resource as u32, desc: source.desc })?);
                }
                Ok(())
            })
        }).unwrap();
        let changed = [fixture.world.get_resource_ref::<WaterRaster>().unwrap().last_changed(),
            fixture.world.get_resource_ref::<GardenRaster>().unwrap().last_changed()];
        if let Some(previous) = last_changed { assert_eq!(changed, previous, "clean reads must not mark the source resources changed"); }
        else { assert_eq!(changed, [tick; 2]); last_changed = Some(changed); }
        assert!(!fixture.world.resource::<WaterRaster>().reupload_pending());
        assert!(!fixture.world.resource::<GardenRaster>().reupload_pending());
        assert!(fixture.world.resource::<WaterRaster>().dirty().is_empty());
        assert!(fixture.world.resource::<GardenRaster>().dirty().is_empty());
        fixture.bridge.shader_backend().prewarm(Duration::from_secs(30)).unwrap();
        let receipt = fixture.bridge.execute_built(built).unwrap();
        for (name, initial_count) in [("Upload cpu raster", 2), ("Blur water mask", 4), ("Blur garden mask", 2)] {
            let count = receipt.snapshot.pass_order.iter().filter(|x| x.as_str() == name).count();
            assert_eq!(count, if frame_index == 0 { initial_count } else { 0 }, "original pass {name}");
        }
        fixture.bridge.wait_submits_done(10_000_000_000).unwrap();
        for (kind, (output, seed)) in outputs.into_iter().zip([73, 149]).enumerate() {
            let mut bytes = vec![0; 512 * 512 * 4];
            output.read_bytes(0, &mut bytes).unwrap();
            let expected = original_resolve_byte(seed);
            for (x, y) in [(0, 0), (7, 9), (83, 137), (511, 511)] {
                let rgb = &bytes[(y * 512 + x) * 4..(y * 512 + x) * 4 + 3];
                assert!(rgb[0].abs_diff(expected) <= 1, "preserved red frame{frame_index} kind{kind} {x},{y}: {rgb:?} expected{expected}");
                assert!(rgb[1].abs_diff(expected) <= 2, "original blur G frame{frame_index} kind{kind} {x},{y}: {rgb:?} expected{expected}");
                if kind == 0 { assert!(rgb[2].abs_diff(expected) <= 2, "original water B frame{frame_index} {x},{y}: {rgb:?} expected{expected}"); }
                else { assert_eq!(rgb[2], 0, "garden must retain its original blue channel"); }
            }
        }
    }
}
