
#[cfg(test)]
mod raster_attachment_range_tests {
    use super::*;
    use rhapsody::rg::{AccessRange, AccessTable, GraphDefinition, PlanContext, ResourceSpec};

    fn color(tag: u32, bits: [u32; 4]) -> RasterPassColorAttachment {
        RasterPassColorAttachment { id: 0, format: 100, tag, sample_count: 1, view: bits }
    }
    fn depth(layer: u32, clear: DepthClear) -> RasterPassDepth {
        RasterPassDepth { texture_id: 0, format: 126, sample_count: 1, clear, writable: true, layer }
    }
    fn depth_desc() -> rhapsody::rg::Descriptor {
        ResourceSpec::Image(rhapsody::resource::TextureDesc::new_2d_array(
            64, 64, 3, vk::Format::D32_SFLOAT).mip_levels(4)
            .usage(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)).descriptor()
    }
    #[test]
    fn native_color_payload_is_clear_bits_and_only_depth_restricts_array_layer_count() {
        let bits = [0x3f80_0000, 0x8000_0000, 0x7fc0_0123, 0x3e80_0000];
        let c = color(1, bits);
        assert_eq!(c.clear_color_bits(), bits);
        let decoded = c.clear_color_bits().map(f32::from_bits);
        assert_eq!(decoded.map(f32::to_bits), bits);
        let plans = raster_attachment_accesses(&[c], Some(&depth(2, DepthClear::Load))).unwrap();
        assert_eq!(plans[0].access, 0x2e);
        assert!(!plans[0].load);
        assert!(plans[1].load);
        assert_eq!(plans[0].view.level_count, Some(1));
        assert_eq!(plans[0].view.layer_count, None);
        assert_eq!(plans[1].view.layer_count, Some(1));
        assert_eq!(plans[1].view.base_layer, 2);
        let desc = ResourceSpec::Image(rhapsody::resource::TextureDesc::new_2d_array(
            64, 64, 3, vk::Format::R32_SFLOAT).mip_levels(4)).descriptor();
        assert_eq!(compute_view_range(desc, &plans[0].view).unwrap(), AccessRange::Texture {
            aspect_mask: 1, base_mip_level: 0, level_count: 1,
            base_array_layer: 0, layer_count: 3,
        });
        assert_eq!(compute_view_range(depth_desc(), &plans[1].view).unwrap(), AccessRange::Texture {
            aspect_mask: 2, base_mip_level: 0, level_count: 1,
            base_array_layer: 2, layer_count: 1,
        });
        assert!(raster_attachment_accesses(&[color(3, bits)], None).is_err());
    }

    #[test]
    fn native_depth_layer_load_keeps_its_producer_and_does_not_initialize_other_mips_or_layers() {
        let plans = raster_attachment_accesses(&[], Some(&depth(1, DepthClear::Load))).unwrap();
        let selected = compute_view_range(depth_desc(), &plans[0].view).unwrap();
        let other = compute_view_range(depth_desc(), &plans[0].view.base_layer(2)).unwrap();
        let mut graph = RhGraph::new();
        let texture = graph.import_unowned("shadow cascades", depth_desc(), 1, 0).unwrap();
        assert!(!raster_has_previous_contents(&graph, texture.raw(), selected));
        let mut writer = graph.add_pass("cascade 1 depth clear and draw");
        writer.add_resource_access_range(texture, 0x2f, 1, selected).unwrap();
        writer.render(|_| true);
        assert!(raster_has_previous_contents(&graph, texture.raw(), selected));
        assert!(!raster_has_previous_contents(&graph, texture.raw(), other));
        let mut load = graph.add_pass("cascade 1 load");
        load.add_preserving_resource_access_range(texture, 0x2f, 1, selected).unwrap();
        load.side_effect(); load.render(|_| true);
        let definition = GraphDefinition::from_declaration(&graph.build().declaration,
            &mut AccessTable::original(), PlanContext::default()).unwrap();
        let plan = rhapsody::rg::compile_definition(definition).unwrap();
        assert_eq!(plan.retained_passes, [0, 1]);
        rhapsody::rg::check_plan(&plan.definition, &plan).unwrap();
        let mut wrong = RhGraph::new();
        let texture = wrong.import_unowned("undefined cascade 2", depth_desc(), 2, 0).unwrap();
        let mut writer = wrong.add_pass("cascade 1 only");
        writer.add_resource_access_range(texture, 0x2f, 1, selected).unwrap(); writer.render(|_| true);
        let mut reader = wrong.add_pass("cascade 2 actual read");
        reader.add_resource_access_range(texture, 0x17, 1, other).unwrap(); reader.side_effect(); reader.render(|_| true);
        assert!(GraphDefinition::from_declaration(&wrong.build().declaration,
            &mut AccessTable::original(), PlanContext::default()).is_err());
        assert!(compute_view_range(depth_desc(), &plans[0].view.base_layer(3)).is_err());
    }
}
