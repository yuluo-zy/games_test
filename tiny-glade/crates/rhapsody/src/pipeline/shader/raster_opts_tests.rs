use super::*;
use super::pipeline::pipeline_array_bytes;
use std::ffi::CString;
use crate::pipeline::shader::pipeline_cache::budgeted_key_vec;
use crate::pipeline::shader::pipeline_cache::slot_for;

#[test]
fn cache_slots_charge_actual_owners_and_join_full_borrowed_keys() {
    use std::cell::Cell;
    let budget = Arc::new(AllocatorStats::default());
    let table = Mutex::new(TempList::new(1, budget.clone()).unwrap());
    let first = slot_for(
        &table,
        |key: &Vec<u8>| key.as_slice() == b"first",
        || budgeted_key_vec(&budget, b"first"),
        1,
        &budget,
    )
    .unwrap();
    let charged = budget.cpu_memory_usage().total();
    let invoked = Cell::new(false);
    budget.set_total_budget(charged).unwrap();
    let same = slot_for(
        &table,
        |key| key.as_slice() == b"first",
        || {
            invoked.set(true);
            budgeted_key_vec(&budget, b"first")
        },
        1,
        &budget,
    )
    .unwrap();
    assert!(Arc::ptr_eq(&same, &first));
    assert!(
        !invoked.get(),
        "cache hit never allocates or builds an owned key"
    );
    assert_eq!(budget.cpu_memory_usage().total(), charged);
    assert!(matches!(
        slot_for(
            &table,
            |_| false,
            || {
                invoked.set(true);
                budgeted_key_vec(&budget, b"other")
            },
            1,
            &budget
        ),
        Err(crate::Error::Exhausted(_))
    ));
    assert!(
        !invoked.get(),
        "an in-flight compiler slot is never evicted"
    );
    drop(same);
    table.lock().clear();
    drop(table);
    assert!(
        budget.cpu_memory_usage().total() > 0,
        "external slot Arc retains its metadata credit"
    );
    drop(first);
    assert_eq!(budget.cpu_memory_usage().total(), 0);
}

#[test]
fn cache_key_admission_failure_preserves_the_old_slot_and_credit() {
    let budget = Arc::new(AllocatorStats::default());
    let table = Mutex::new(TempList::new(1, budget.clone()).unwrap());
    let first = slot_for(
        &table,
        |key: &Vec<u8>| key.as_slice() == b"first",
        || budgeted_key_vec(&budget, b"first"),
        1,
        &budget,
    )
    .unwrap();
    drop(first);
    let charged = budget.cpu_memory_usage().total();
    budget.set_total_budget(charged).unwrap();
    assert!(matches!(
        slot_for(
            &table,
            |_| false,
            || budgeted_key_vec(&budget, b"second"),
            1,
            &budget
        ),
        Err(crate::Error::OutOfHostMemory)
    ));
    assert_eq!(budget.cpu_memory_usage().total(), charged);
    assert_eq!(table.lock().as_slice()[0].key, b"first");
    budget.set_total_budget(1 << 20).unwrap();
    let next = slot_for(
        &table,
        |_| false,
        || budgeted_key_vec(&budget, b"second"),
        1,
        &budget,
    )
    .unwrap();
    assert_eq!(table.lock().as_slice()[0].key, b"second");
    drop((next, table));
    assert_eq!(budget.cpu_memory_usage().total(), 0);
}

#[test]
fn pipeline_array_accounting_includes_both_format_vectors_and_rejects_overflow() {
    let bytes = pipeline_array_bytes(2, 2, 4, 3).unwrap();
    assert_eq!(
        bytes,
        (2 * std::mem::size_of::<Arc<ShaderModule>>()
            + 2 * std::mem::size_of::<usize>()
            + 7 * std::mem::size_of::<vk::Format>()) as u64
    );
    assert!(matches!(
        pipeline_array_bytes(usize::MAX, 0, 0, 0),
        Err(crate::Error::OutOfHostMemory)
    ));
}

#[test]
fn invalid_raster_options_return_usage_errors_without_changing_encoded_state() {
    assert_eq!(
        RasterPipelineOpts(3).topology().unwrap_err().kind(),
        crate::ErrorKind::InvalidUsage
    );
    assert_eq!(
        RasterPipelineOpts(3 << 4)
            .depth_compare_op()
            .unwrap_err()
            .kind(),
        crate::ErrorKind::InvalidUsage
    );
    let mut options = RasterPipelineOpts(0xa5a5_5a5a);
    let original = options;
    for error in [
        options
            .set_topology(vk::PrimitiveTopology::PATCH_LIST)
            .unwrap_err(),
        options
            .set_depth_compare_op(vk::CompareOp::ALWAYS)
            .unwrap_err(),
        options
            .set_polygon_mode(vk::PolygonMode::POINT)
            .unwrap_err(),
    ] {
        assert_eq!(error.kind(), crate::ErrorKind::InvalidUsage);
    }
    assert_eq!(options, original);
}

#[test]
fn malformed_shader_and_reflection_use_public_error_categories() {
    assert_eq!(
        reflect_spirv_entries(&[]).unwrap_err().kind(),
        crate::ErrorKind::InvalidUsage
    );
    let reflection = shader_pipeline::reflection::ShaderReflection {
        descriptor_sets: vec![],
        uniform_block: None,
        binding_names: vec![],
        binding_name_hashes: vec![],
        vertex_inputs: vec![],
        local_size: None,
        push_constant_size: 0,
    };
    let error = validate_compute_reflection(&reflection).unwrap_err();
    assert_eq!(error.kind(), crate::ErrorKind::InvalidUsage);
    assert!(matches!(
        std::error::Error::source(&error)
            .and_then(|source| source.downcast_ref::<InterfaceError>()),
        Some(InterfaceError::MissingGroupSize)
    ));
}

#[test]
fn dynamic_formats_validate_aspects_limits_and_multiview() {
    let color = DynamicRenderingFormats::color(vk::Format::R8G8B8A8_UNORM);
    color.validate(1).unwrap();
    assert!(color.validate(0).is_err());
    let mut value = color.clone();
    value.colors[0] = vk::Format::D32_SFLOAT;
    assert!(value.validate(1).is_err());
    value = color.clone();
    value.depth = vk::Format::R8G8B8A8_UNORM;
    assert!(value.validate(1).is_err());
    value.depth = vk::Format::D24_UNORM_S8_UINT;
    value.stencil = vk::Format::D24_UNORM_S8_UINT;
    value.validate(1).unwrap();
    value.stencil = vk::Format::S8_UINT;
    assert!(value.validate(1).is_err());
    value = color;
    value.view_mask = 1;
    assert_eq!(
        value.validate(1).unwrap_err().kind(),
        crate::ErrorKind::Unsupported
    );
}
#[test]
fn dynamic_cache_signature_includes_actual_target_and_all_formats() {
    let key = |value: Option<&DynamicRenderingFormats>| {
        let mut key = Vec::new();
        visit_dynamic_rendering_key(value, |word| key.push(word));
        key
    };
    let color = DynamicRenderingFormats::color(vk::Format::R8G8B8A8_UNORM);
    assert_ne!(key(None), key(Some(&color)));
    for changed in 0..5 {
        let mut value = color.clone();
        match changed {
            0 => value.colors[0] = vk::Format::B8G8R8A8_UNORM,
            1 => value.colors.push(vk::Format::R8G8B8A8_UNORM),
            2 => value.depth = vk::Format::D32_SFLOAT,
            3 => value.stencil = vk::Format::S8_UINT,
            _ => value.view_mask = 1,
        }
        assert_ne!(key(Some(&color)), key(Some(&value)));
    }
}

#[test]
fn raster_cache_key_length_matches_the_actual_full_key_and_borrowed_lookup() {
    let device = match crate::device::selection::create_headless_device() {
        Ok(device) => device,
        Err(error) => {
            assert!(
                std::env::var("RHAPSODY_REQUIRE_VULKAN").as_deref() != Ok("1"),
                "strict raster key fixture requires Vulkan: {error}"
            );
            eprintln!("SKIP Vulkan raster key fixture: {error}");
            return;
        }
    };
    // Keys only read these owned identities; no null shader/layout is ever
    // passed to native pipeline creation by this metadata regression.
    let module = Arc::new(ShaderModule {
        device: device.clone(),
        handle: vk::ShaderModule::null(),
        interfaces: Arc::from([]),
        _memory: reserve_shared_cpu_metadata::<ShaderModule>(&device.memory.stats, 16).unwrap(),
    });
    let layout = Arc::new(PipelineLayout {
        retirement: Some(device.reserve_retirement().unwrap()),
        last_use: crate::tracking::SyncCell::for_device_budgeted(
            device.generation(),
            &device.memory.stats,
        )
        .unwrap(),
        device: device.clone(),
        handle: vk::PipelineLayout::null(),
        sets: Vec::new(),
        push_constants: Vec::new(),
        bindless_abi: None,
        _memory: reserve_shared_cpu_metadata::<PipelineLayout>(&device.memory.stats, 0)
            .unwrap(),
    });
    let mut desc = RasterPipelineDesc::new(module, layout);
    for index in 0..5 {
        desc.entry = CString::new(format!("vertex_{index}")).unwrap();
        desc.fragment_entry = Some(CString::new(format!("fragment_entry_{index}")).unwrap());
        desc.dynamic_rendering = (index != 0).then(|| DynamicRenderingFormats {
            colors: vec![vk::Format::R8G8B8A8_UNORM; index],
            depth: vk::Format::UNDEFINED,
            stencil: vk::Format::UNDEFINED,
            view_mask: 0,
        });
        desc.vertex_bindings = vec![vk::VertexInputBindingDescription::default(); index];
        desc.vertex_attributes =
            vec![vk::VertexInputAttributeDescription::default(); index + 1];
        desc.blend_attachments = vec![vk::PipelineColorBlendAttachmentState::default(); index];
        let key = raster_pipeline_key(&desc);
        assert_eq!(key.len(), raster_pipeline_key_len(&desc));
        assert!(raster_pipeline_key_matches(&desc, &key));
        desc.depth_write = !desc.depth_write;
        assert!(!raster_pipeline_key_matches(&desc, &key));
    }
}

#[test]
fn bit_positions_match_the_exe_decode() {
    let mut o = RasterPipelineOpts::new();
    o.set_topology(vk::PrimitiveTopology::LINE_STRIP).unwrap();
    assert_eq!(o.0, 0b10);
    let mut o = RasterPipelineOpts::new();
    o.set_cull_mode(vk::CullModeFlags::BACK);
    assert_eq!(o.0, 0b10 << 2);
    let mut o = RasterPipelineOpts::new();
    o.set_depth_compare_op(vk::CompareOp::EQUAL).unwrap();
    o.set_depth_write(true);
    o.set_depth_test_enable(true);
    assert_eq!(o.0, (2 << 4) | (1 << 6) | (1 << 7));
    let mut o = RasterPipelineOpts::new();
    o.set_stencil_test_enable(true);
    o.set_stencil_fail_op(vk::StencilOp::INCREMENT_AND_CLAMP);
    o.set_stencil_pass_op(vk::StencilOp::REPLACE);
    assert_eq!(o.0, (1 << 8) | (3 << 9) | (2 << 12));
    // depth-fail op 3 位：b1 bit7 为低位，b2 bit0–1 为高两位。
    let mut o = RasterPipelineOpts::new();
    o.set_stencil_depth_fail_op(vk::StencilOp::INVERT); // 5 = 0b101
    assert_eq!(o.0, (1 << 15) | (2 << 16));
    assert_eq!(o.stencil_depth_fail_op(), vk::StencilOp::INVERT);
    let mut o = RasterPipelineOpts::new();
    o.set_stencil_compare_op(vk::CompareOp::ALWAYS); // 7
    assert_eq!(o.0, 7 << 18);
    // compare mask 8 位：b2 bit5–7 低 3 位，b3 bit0–4 高 5 位。
    let mut o = RasterPipelineOpts::new();
    o.set_stencil_compare_mask(0xff);
    assert_eq!(o.0, (7 << 21) | (0x1f << 24));
    assert_eq!(o.stencil_compare_mask(), 0xff);
    let mut o = RasterPipelineOpts::new();
    o.set_blend_mode(3);
    o.set_polygon_mode(vk::PolygonMode::LINE).unwrap();
    assert_eq!(o.0, (3 << 29) | (1 << 31));
}

#[test]
fn split_fields_round_trip() {
    let mut o = RasterPipelineOpts::new();
    for (op, mask) in [(0u32, 0u32), (7, 0xa5), (3, 0x5a), (5, 0xff)] {
        o.set_stencil_depth_fail_op(vk::StencilOp::from_raw(op as i32));
        o.set_stencil_compare_mask(mask);
        o.set_stencil_compare_op(vk::CompareOp::LESS_OR_EQUAL);
        o.set_blend_mode(2);
        assert_eq!(o.stencil_depth_fail_op().as_raw(), op as i32);
        assert_eq!(o.stencil_compare_mask(), mask);
        assert_eq!(o.stencil_compare_op(), vk::CompareOp::LESS_OR_EQUAL);
        assert_eq!(o.blend_mode(), 2);
    }
}

#[test]
fn jump_table_enums_use_the_exe_ordering() {
    for (code, topology) in [
        (0, vk::PrimitiveTopology::TRIANGLE_LIST),
        (1, vk::PrimitiveTopology::LINE_LIST),
        (2, vk::PrimitiveTopology::LINE_STRIP),
    ] {
        assert_eq!(RasterPipelineOpts(code).topology().unwrap(), topology);
    }
    for (code, op) in [
        (0, vk::CompareOp::LESS),
        (1, vk::CompareOp::GREATER),
        (2, vk::CompareOp::EQUAL),
    ] {
        assert_eq!(
            RasterPipelineOpts(code << 4).depth_compare_op().unwrap(),
            op
        );
    }
}

#[test]
fn apply_to_replicates_one_blend_state_per_color_attachment() {
    let device = match crate::device::selection::create_headless_device() {
        Ok(device) => device,
        Err(_) => return,
    };
    let module = Arc::new(ShaderModule {
        device: device.clone(),
        handle: vk::ShaderModule::null(),
        interfaces: Arc::from([]),
        _memory: reserve_shared_cpu_metadata::<ShaderModule>(&device.memory.stats, 16).unwrap(),
    });
    let layout = Arc::new(PipelineLayout {
        retirement: Some(device.reserve_retirement().unwrap()),
        last_use: crate::tracking::SyncCell::for_device_budgeted(
            device.generation(),
            &device.memory.stats,
        )
        .unwrap(),
        device: device.clone(),
        handle: vk::PipelineLayout::null(),
        sets: Vec::new(),
        push_constants: Vec::new(),
        bindless_abi: None,
        _memory: reserve_shared_cpu_metadata::<PipelineLayout>(&device.memory.stats, 0)
            .unwrap(),
    });
    let mut desc = RasterPipelineDesc::new(module, layout);
    let mut o = RasterPipelineOpts::new();
    o.set_blend_mode(2);
    o.set_stencil_test_enable(true);
    o.set_stencil_compare_mask(0x81);
    o.apply_to(&mut desc, 3).unwrap();
    assert_eq!(desc.blend_attachments.len(), 3);
    assert_eq!(desc.blend_attachments[0].dst_color_blend_factor.as_raw(), 7);
    assert!(desc.stencil_test);
    assert_eq!(desc.stencil.compare_mask, 0x81);
    assert_eq!(desc.stencil.write_mask, 0xffff_ffff);
    assert_eq!(desc.front_face, vk::FrontFace::COUNTER_CLOCKWISE);
}

#[test]
fn map_vertex_attribute_format_follows_the_exe_table() {
    assert_eq!(
        map_vertex_attribute_format(4, 4).unwrap(),
        vk::Format::R32_SFLOAT
    );
    assert_eq!(
        map_vertex_attribute_format(4, 8).unwrap(),
        vk::Format::R32G32_SFLOAT
    );
    assert_eq!(
        map_vertex_attribute_format(4, 12).unwrap(),
        vk::Format::R32G32B32_SFLOAT
    );
    assert_eq!(
        map_vertex_attribute_format(4, 16).unwrap(),
        vk::Format::R32G32B32A32_SFLOAT
    );
    assert_eq!(
        map_vertex_attribute_format(3, 4).unwrap(),
        vk::Format::R32_UINT
    );
    assert_eq!(
        map_vertex_attribute_format(2, 16).unwrap(),
        vk::Format::R32G32B32A32_SINT
    );
    assert!(map_vertex_attribute_format(4, 20).is_err());
    assert!(map_vertex_attribute_format(5, 4).is_err());
}
