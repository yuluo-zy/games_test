
#[cfg(test)]
mod raster_binding_range_tests {
    use super::*;
    use country_core::render::pass_builder::draw_stream::{PassAccess, ResourceAccess};
    use country_core::rhapsody_types::TextureViewDesc;
    use rhapsody::rg::{AccessRange, Descriptor, ResourceSpec};

    fn image() -> Descriptor {
        ResourceSpec::Image(
            rhapsody::resource::TextureDesc::new_2d_array(64, 64, 3, vk::Format::R32_SFLOAT)
                .mip_levels(4)
                .usage(vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::STORAGE),
        )
        .descriptor()
    }
    fn binding(mip: u32, layer: u32, access: u8) -> RenderPassBinding {
        RenderPassBinding {
            resource_index: 0,
            access_type: access,
            view: Some(TextureViewDesc::new()
                .base_mip_level(mip).level_count(Some(1))
                .base_layer(layer).layer_count(Some(1))),
        }
    }
    fn access(code: u8) -> PassAccess {
        PassAccess { resource: 0, access: code, group: 1 }
    }
    fn packet() -> DrawPacket {
        DrawPacket::new(
            country_core::rhapsody_contract::Handle(
                ManagedHandle(1), std::marker::PhantomData),
            [0, 0], RasterState::default())
    }

    #[test]
    fn repeated_owner_views_keep_ranges_slots_and_attachment_order() {
        let bindings = [binding(2, 1, 0x1d), binding(3, 2, 0x1d)];
        // The first access has an identical owner/code but belongs to the
        // framebuffer, before the two native shader binding operations.
        let ranges = raster_binding_ranges(
            &[access(0x1d), access(0x1d), access(0x1d)],
            &bindings, 1, |_| Ok(image())).unwrap();
        assert_eq!(ranges[0], (AccessRange::Whole, None));
        assert_eq!(ranges[1].1, Some(0));
        assert_eq!(ranges[2].1, Some(1));
        assert_eq!(ranges[1].0, AccessRange::Texture {
            aspect_mask: 1, base_mip_level: 2, level_count: 1,
            base_array_layer: 1, layer_count: 1,
        });
        assert!(!ranges[1].0.overlaps(ranges[2].0));
        let mut first = packet();
        first.accesses = vec![
            ResourceAccess { binding_index: 4, slot: 0 },
            ResourceAccess { binding_index: 5, slot: 1 },
        ];
        first.inactive_texture_bindings.push(5);
        let (_, inactive) = raster_content_intents([&first], &bindings, |_| Ok(true)).unwrap();
        assert_eq!(inactive[&0], false);
        assert_eq!(inactive[&1], true);
        // A second packet's active alias of slot 1 must still tighten that slot.
        let mut second = packet();
        second.accesses.push(ResourceAccess { binding_index: 8, slot: 1 });
        let (_, inactive) = raster_content_intents([&first, &second], &bindings, |_| Ok(true)).unwrap();
        assert_eq!(inactive[&1], false);
    }

    #[test]
    fn selected_shader_view_reads_only_its_real_producer_and_keeps_other_layers_undefined() {
        use rhapsody::rg::{AccessTable, GraphDefinition, PlanContext};
        let selected = raster_binding_ranges(
            &[access(0x1d)], &[binding(2, 1, 0x1d)], 0, |_| Ok(image())).unwrap()[0].0;
        let definition = |read_range: AccessRange| {
            let mut graph = RhGraph::new();
            let texture = graph.import_unowned("partial array", image(), 1, 0).unwrap();
            let mut writer = graph.add_pass("real selected producer");
            writer.add_resource_access_range(texture, 0x34, 0, selected).unwrap();
            writer.render(|_| true);
            let mut reader = graph.add_pass("raster selected view");
            reader.add_resource_access_range(texture, 0x1d, 1, read_range).unwrap();
            reader.side_effect();
            reader.render(|_| true);
            GraphDefinition::from_declaration(
                &graph.build().declaration, &mut AccessTable::original(), PlanContext::default())
        };
        assert!(definition(selected).is_ok());
        assert!(definition(AccessRange::Whole).is_err(),
            "owner-wide projection must retain its real uninitialized-content failure");
        let wrong_layer = AccessRange::Texture {
            aspect_mask: 1, base_mip_level: 2, level_count: 1,
            base_array_layer: 2, layer_count: 1,
        };
        assert!(definition(wrong_layer).is_err());
    }

    #[test]
    fn bad_view_missing_access_and_invalid_slot_fail_without_widening() {
        assert!(raster_binding_ranges(&[access(0x1d)], &[binding(4, 0, 0x1d)], 0,
            |_| Ok(image())).is_err());
        assert!(raster_binding_ranges(&[access(0x1d)], &[binding(0, 3, 0x1d)], 0,
            |_| Ok(image())).is_err());
        assert!(raster_binding_ranges(&[], &[binding(0, 0, 0x1d)], 0,
            |_| Ok(image())).is_err());
        assert!(raster_binding_ranges(&[access(0x1d)], &[binding(0, 0, 0x1d)], 2,
            |_| Ok(image())).is_err());
        let buffer = ResourceSpec::Buffer(rhapsody::resource::BufferDesc::new(
            64, vk::BufferUsageFlags::STORAGE_BUFFER,
            rhapsody::resource::MemoryLocation::GpuOnly)).descriptor();
        assert!(raster_binding_ranges(&[access(0x1d)], &[binding(0, 0, 0x1d)], 0,
            |_| Ok(buffer)).is_err());
    }
}
