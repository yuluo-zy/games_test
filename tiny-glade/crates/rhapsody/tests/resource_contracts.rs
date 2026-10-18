//! Production graph IDs, resource versions and bounded access validation; CPU only.
use ash::vk;
use rhapsody::rg::{
    graph::{BufferDesc, BuildError, Descriptor, OriginSource, RenderGraph},
    plan::{GraphDefinition, PlanContext},
    resource::{AccessRange, AccessVersion, ResourceContractError, ResourceHandle, ResourceOrigin},
    vulkan::{AccessTable, ResourceSpec},
};

const READ: u8 = 0x26;
const WRITE: u8 = 0x34;
fn buffer_desc() -> BufferDesc {
    BufferDesc {
        word_at_00: 64,
        word_at_08: 0,
        bytes_at_0c_through_0e: [0; 3],
    }
}
fn graph_definition(graph: RenderGraph) -> Result<GraphDefinition, rhapsody::Error> {
    GraphDefinition::from_declaration(
        &graph.build().declaration,
        &mut AccessTable::original(),
        PlanContext::default(),
    )
}
fn imported(graph: &mut RenderGraph) -> ResourceHandle {
    graph
        .create_raw_resource(
            "import",
            ResourceOrigin {
                descriptor: Descriptor::Buffer(buffer_desc()),
                source: OriginSource::ImportUnowned {
                    token: 100,
                    initial_access: READ,
                },
            },
        )
        .unwrap()
}
fn bytes(offset: u64, size: u64) -> AccessRange {
    AccessRange::Buffer { offset, size }
}

#[test]
fn graph_scope_rejects_same_index_from_another_graph_and_unscoped_projection() {
    let mut first = RenderGraph::new();
    let foreign = first.create_buffer("first", buffer_desc()).unwrap();
    let mut second = RenderGraph::new();
    let local = second.create_buffer("second", buffer_desc()).unwrap();
    assert_eq!(foreign.raw().raw(), local.raw().raw());
    let mut pass = second.add_pass("write");
    assert!(matches!(
        pass.add_resource_access(foreign.raw(), WRITE, 0),
        Err(BuildError::WrongGraph { .. })
    ));
    assert!(matches!(
        pass.add_resource_access(ResourceHandle::from_raw(0), WRITE, 0),
        Err(BuildError::WrongGraph { .. })
    ));
    pass.add_resource_access(local.raw(), WRITE, 0).unwrap();
    pass.render(|_| true);
    assert!(graph_definition(second).is_ok());
}

#[test]
fn typed_conversion_rejects_wrong_resource_kind() {
    let mut graph = RenderGraph::new();
    let buffer = graph.create_buffer("buffer", buffer_desc()).unwrap();
    assert!(matches!(
        graph.texture_id(buffer.raw()),
        Err(BuildError::ResourceKindMismatch(0))
    ));
}

#[test]
fn writes_publish_versions_only_when_the_pass_is_committed() {
    let mut graph = RenderGraph::new();
    let v0 = graph.create_buffer("buffer", buffer_desc()).unwrap();
    {
        let mut discarded = graph.add_pass("discard");
        let v1 = discarded
            .write_buffer(v0, WRITE, AccessRange::Whole)
            .unwrap();
        assert_eq!(v1.version(), 1);
    }
    assert_eq!(graph.buffer_id(v0.raw()).unwrap().version(), 0);
    let mut writer = graph.add_pass("write");
    let v1 = writer.write_buffer(v0, WRITE, AccessRange::Whole).unwrap();
    assert!(matches!(
        writer.write_buffer(v0, WRITE, AccessRange::Whole),
        Err(BuildError::Contract(ResourceContractError::StaleVersion {
            expected: 1,
            actual: 0,
            ..
        }))
    ));
    writer.render(|_| true);
    let mut reader = graph.add_pass("read");
    reader.read_buffer(v1, READ, AccessRange::Whole).unwrap();
    reader.render(|_| true);
    let definition = graph_definition(graph).unwrap();
    assert_eq!(
        definition.passes[0].accesses[0].version,
        AccessVersion::Write {
            previous: 0,
            current: 1
        }
    );
    assert_eq!(
        definition.passes[1].accesses[0].version,
        AccessVersion::Read(1)
    );
}

#[test]
fn typed_mode_and_ranges_are_checked_before_mutating_the_pass() {
    let mut graph = RenderGraph::new();
    let buffer = graph.create_buffer("buffer", buffer_desc()).unwrap();
    let mut pass = graph.add_pass("invalid");
    assert!(matches!(
        pass.write_buffer(buffer, READ, AccessRange::Whole),
        Err(BuildError::Contract(
            ResourceContractError::WrongAccessMode { .. }
        ))
    ));
    for range in [bytes(64, 1), bytes(0, 0), bytes(u64::MAX, 2)] {
        assert!(matches!(
            pass.write_buffer(buffer, WRITE, range),
            Err(BuildError::Contract(
                ResourceContractError::InvalidRange { .. }
            ))
        ));
    }
    let texture_range = AccessRange::Texture {
        aspect_mask: 1,
        base_mip_level: 0,
        level_count: 1,
        base_array_layer: 0,
        layer_count: 1,
    };
    assert!(matches!(
        pass.write_buffer(buffer, WRITE, texture_range),
        Err(BuildError::Contract(
            ResourceContractError::WrongRangeKind { .. }
        ))
    ));
    pass.render(|_| true);
    assert!(graph.build().declaration.passes[0].accesses.is_empty());
}

#[test]
fn first_read_requires_an_import_or_covering_writes() {
    let mut graph = RenderGraph::new();
    let buffer = graph.create_buffer("buffer", buffer_desc()).unwrap();
    let mut reader = graph.add_pass("read undefined");
    reader
        .read_buffer(buffer, READ, AccessRange::Whole)
        .unwrap();
    reader.render(|_| true);
    assert_eq!(
        graph_definition(graph).unwrap_err().kind(),
        rhapsody::ErrorKind::InvalidUsage
    );

    let mut graph = RenderGraph::new();
    let buffer = imported(&mut graph);
    let typed = graph.buffer_id(buffer).unwrap();
    let mut reader = graph.add_pass("read import");
    reader.read_buffer(typed, READ, AccessRange::Whole).unwrap();
    reader.render(|_| true);
    assert_eq!(
        graph_definition(graph).unwrap().passes[0].accesses[0].version,
        AccessVersion::Read(0)
    );
}

#[test]
fn partial_writes_inherit_initialized_regions_and_union_covers_whole_buffer() {
    let mut graph = RenderGraph::new();
    let v0 = graph.create_buffer("buffer", buffer_desc()).unwrap();
    let mut first = graph.add_pass("left");
    let v1 = first.write_buffer(v0, WRITE, bytes(0, 32)).unwrap();
    first.render(|_| true);
    let mut second = graph.add_pass("right");
    let v2 = second.write_buffer(v1, WRITE, bytes(32, 32)).unwrap();
    second.render(|_| true);
    let mut reader = graph.add_pass("read all");
    reader.read_buffer(v2, READ, AccessRange::Whole).unwrap();
    reader.render(|_| true);
    assert!(graph_definition(graph).is_ok());
}

#[test]
fn partial_write_does_not_initialize_unwritten_bytes_or_export_them() {
    for export in [false, true] {
        let mut graph = RenderGraph::new();
        let v0 = graph.create_buffer("buffer", buffer_desc()).unwrap();
        let mut writer = graph.add_pass("left");
        let v1 = writer.write_buffer(v0, WRITE, bytes(0, 32)).unwrap();
        writer.render(|_| true);
        if export {
            graph.export(v1.raw(), READ).unwrap();
        } else {
            let mut reader = graph.add_pass("read hole");
            reader.read_buffer(v1, READ, bytes(31, 2)).unwrap();
            reader.render(|_| true);
        }
        assert_eq!(
            graph_definition(graph).unwrap_err().kind(),
            rhapsody::ErrorKind::InvalidUsage
        );
    }
}

#[test]
fn partial_export_keeps_only_its_producer_and_rejects_stale_or_uninitialized_ranges() {
    use rhapsody::rg::plan::compile_definition;
    let mut graph = RenderGraph::new();
    let original = graph.create_buffer("partial", buffer_desc()).unwrap();
    let mut left = graph.add_pass("left");
    let current = left.write_buffer(original, WRITE, bytes(0, 32)).unwrap();
    left.render(|_| true);
    assert!(matches!(
        graph.export_buffer_range(original, READ, bytes(0, 32)),
        Err(BuildError::Contract(
            ResourceContractError::StaleVersion { .. }
        ))
    ));
    graph
        .export_buffer_range(current, READ, bytes(0, 32))
        .unwrap();
    let mut right = graph.add_pass("unused right");
    right.write_buffer(current, WRITE, bytes(32, 32)).unwrap();
    right.render(|_| true);
    let definition = graph_definition(graph).unwrap();
    assert_eq!(definition.exports[0].range, bytes(0, 32));
    let plan = compile_definition(definition).unwrap();
    assert_eq!(plan.retained_passes, vec![0]);

    let mut graph = RenderGraph::new();
    let original = graph.create_buffer("hole", buffer_desc()).unwrap();
    let mut left = graph.add_pass("left");
    let current = left.write_buffer(original, WRITE, bytes(0, 32)).unwrap();
    left.render(|_| true);
    graph
        .export_buffer_range(current, READ, bytes(31, 2))
        .unwrap();
    assert!(graph_definition(graph).is_err());
}

#[test]
fn export_range_validation_rejects_wrong_kind_and_bounds_before_inserting() {
    let mut graph = RenderGraph::new();
    let buffer = graph.create_buffer("buffer", buffer_desc()).unwrap();
    for range in [bytes(64, 1), bytes(0, 0)] {
        assert!(graph.export_buffer_range(buffer, READ, range).is_err());
    }
    assert!(
        graph
            .export_buffer_range(
                buffer,
                READ,
                AccessRange::Texture {
                    aspect_mask: 1,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                }
            )
            .is_err()
    );
    assert!(graph.build().declaration.exports.is_empty());
}

fn texture_descriptor() -> rhapsody::rg::graph::TextureDesc {
    let desc = rhapsody::raw::texture::TextureDesc::new_2d(16, 16, vk::Format::D24_UNORM_S8_UINT)
        .mip_levels(2);
    let Descriptor::Image(desc) = ResourceSpec::Image(desc).descriptor() else {
        unreachable!()
    };
    desc
}
fn subresource(aspect: vk::ImageAspectFlags, mip: u32, mips: u32) -> AccessRange {
    AccessRange::Texture {
        aspect_mask: aspect.as_raw(),
        base_mip_level: mip,
        level_count: mips,
        base_array_layer: 0,
        layer_count: 1,
    }
}

#[test]
fn texture_aspect_and_mip_rectangles_union_without_initializing_other_aspects() {
    let mut graph = RenderGraph::new();
    let mut current = graph.create_texture("depth", texture_descriptor()).unwrap();
    for aspect in [vk::ImageAspectFlags::DEPTH, vk::ImageAspectFlags::STENCIL] {
        for mip in 0..2 {
            let mut writer = graph.add_pass("write aspect mip");
            current = writer
                .write_texture(current, WRITE, subresource(aspect, mip, 1))
                .unwrap();
            writer.render(|_| true);
        }
    }
    let mut reader = graph.add_pass("read all");
    reader
        .read_texture(current, READ, AccessRange::Whole)
        .unwrap();
    reader.render(|_| true);
    assert!(graph_definition(graph).is_ok());
}

#[test]
fn texture_bounds_and_aspect_are_validated_with_checked_arithmetic() {
    let mut graph = RenderGraph::new();
    let texture = graph.create_texture("depth", texture_descriptor()).unwrap();
    let mut pass = graph.add_pass("invalid texture range");
    for range in [
        subresource(vk::ImageAspectFlags::COLOR, 0, 1),
        subresource(vk::ImageAspectFlags::DEPTH, 1, 2),
        subresource(vk::ImageAspectFlags::DEPTH, u32::MAX, 1),
    ] {
        assert!(matches!(
            pass.write_texture(texture, WRITE, range),
            Err(BuildError::Contract(
                ResourceContractError::InvalidRange { .. }
            ))
        ));
    }
}

#[test]
fn raw_access_adapter_resolves_versions_and_rejects_unknown_access_codes() {
    let mut graph = RenderGraph::new();
    let buffer = graph.create_buffer("buffer", buffer_desc()).unwrap();
    let mut writer = graph.add_pass("write");
    writer.add_resource_access(buffer.raw(), WRITE, 0).unwrap();
    writer.render(|_| true);
    let mut reader = graph.add_pass("read");
    reader
        .add_resource_access_range(buffer.raw(), READ, 0, bytes(0, 8))
        .unwrap();
    reader.render(|_| true);
    let definition = graph_definition(graph).unwrap();
    assert_eq!(
        definition.passes[0].accesses[0].version,
        AccessVersion::Write {
            previous: 0,
            current: 1
        }
    );
    assert_eq!(
        definition.passes[1].accesses[0].version,
        AccessVersion::Read(1)
    );
    let mut graph = RenderGraph::new();
    let buffer = imported(&mut graph);
    let mut invalid = graph.add_pass("invalid access");
    invalid.add_resource_access(buffer, 255, 0).unwrap();
    invalid.render(|_| true);
    assert_eq!(
        graph_definition(graph).unwrap_err().kind(),
        rhapsody::ErrorKind::InvalidUsage
    );
}

#[test]
fn cached_plan_rebinding_revalidates_undefined_image_import_before_allocation() {
    let mut graph = RenderGraph::new();
    let image = graph
        .create_raw_resource(
            "external image",
            ResourceOrigin {
                descriptor: Descriptor::Image(texture_descriptor()),
                source: OriginSource::ImportUnowned {
                    token: 55,
                    initial_access: 0x25,
                },
            },
        )
        .unwrap();
    let typed = graph.texture_id(image).unwrap();
    let mut reader = graph.add_pass("read imported history");
    reader.side_effect();
    reader
        .read_texture(typed, 0x25, AccessRange::Whole)
        .unwrap();
    reader.render(|_| true);
    let mut declaration = graph.build().declaration;
    let definition = GraphDefinition::from_declaration(
        &declaration,
        &mut AccessTable::original(),
        PlanContext::default(),
    )
    .unwrap();
    let plan = std::sync::Arc::new(rhapsody::rg::plan::compile_definition(definition).unwrap());
    declaration.resources[0].source = OriginSource::ImportUnowned {
        token: 88,
        initial_access: 0,
    };
    let mut backend = rhapsody::rg::graph::EventBackend::default();
    let result = rhapsody::rg::graph::instantiate_plan(
        plan,
        declaration,
        &mut Default::default(),
        &mut backend,
        &mut AccessTable::original(),
    );
    assert_eq!(
        result.err().unwrap().kind(),
        rhapsody::ErrorKind::InvalidUsage
    );
    assert!(backend.events.is_empty());
}

#[test]
fn unsupported_image_access_returns_invalid_usage_without_panicking() {
    let mut graph = RenderGraph::new();
    let image = graph
        .create_raw_resource(
            "image",
            ResourceOrigin {
                descriptor: Descriptor::Image(texture_descriptor()),
                source: OriginSource::ImportUnowned {
                    token: 42,
                    initial_access: 0x25,
                },
            },
        )
        .unwrap();
    let typed = graph.texture_id(image).unwrap();
    let mut reader = graph.add_pass("unsupported host read");
    reader.side_effect();
    reader
        .read_texture(typed, 0x26, AccessRange::Whole)
        .unwrap();
    reader.render(|_| true);
    let result = graph_definition(graph).and_then(rhapsody::rg::plan::compile_definition);
    assert_eq!(
        result.unwrap_err().kind(),
        rhapsody::ErrorKind::InvalidUsage
    );
}

#[test]
fn binding_validation_checks_the_graph_scope_and_complete_declared_range() {
    use rhapsody::rg::pass_bindings::{PassBinding, PassBindingError};
    let mut graph = RenderGraph::new();
    let buffer = imported(&mut graph);
    let mut pass = graph.add_pass("partial buffer binding");
    pass.add_resource_access_range(buffer, READ, 0, bytes(8, 16))
        .unwrap();
    pass.render(|_| true);
    let pass = &graph.build().declaration.passes[0];
    let descriptor = Descriptor::Buffer(buffer_desc());
    PassBinding::buffer(buffer, 8, 16, true)
        .validate_declaration(pass, descriptor)
        .unwrap();
    assert!(matches!(
        PassBinding::buffer(buffer, 0, 64, true).validate_declaration(pass, descriptor),
        Err(PassBindingError::UndeclaredAccess)
    ));
    let mut other = RenderGraph::new();
    let foreign = imported(&mut other);
    assert!(matches!(
        PassBinding::buffer(foreign, 8, 16, true).validate_declaration(pass, descriptor),
        Err(PassBindingError::Build(BuildError::WrongGraph { .. }))
    ));
}

#[test]
fn resource_info_failure_preserves_metadata_and_rejects_invalid_indices() {
    use rhapsody::rg::resource_info::{AccessDeclaration, OriginKind, ResourceInfo};
    let mut info = ResourceInfo::default();
    let origins = [OriginKind::Tag1 { type_code: 1 }];
    let good = [AccessDeclaration {
        resource_index: 0,
        access_code: WRITE,
        ..Default::default()
    }];
    info.calculate(&origins, &[], &[&good], |_| 0x20).unwrap();
    let previous = info.accesses.clone();
    let bad = [AccessDeclaration {
        resource_index: u32::MAX,
        access_code: WRITE,
        ..Default::default()
    }];
    assert_eq!(
        info.calculate(&origins, &[], &[&bad], |_| 0x20)
            .unwrap_err()
            .kind(),
        rhapsody::ErrorKind::InvalidUsage
    );
    assert_eq!(info.accesses, previous);
    let image_origins = [OriginKind::Tag1 { type_code: 0 }];
    assert_eq!(
        info.calculate(&image_origins, &[], &[&good], |_| 0x2000)
            .unwrap_err()
            .kind(),
        rhapsody::ErrorKind::InvalidUsage
    );
    assert_eq!(info.accesses, previous);
}

#[test]
fn original_raw_first_write_does_not_invent_a_previous_content_read() {
    let mut graph = RenderGraph::new();
    let buffer = graph.create_buffer("original first write", buffer_desc()).unwrap();
    let mut writer = graph.add_pass("original writer with broad synchronization");
    writer.add_resource_access(buffer.raw(), 0x37, 0).unwrap();
    writer.render(|_| true);
    let current = graph.buffer_id(buffer.raw()).unwrap();
    let mut reader = graph.add_pass("consume actual written contents");
    reader.side_effect();
    reader.read_buffer(current, READ, AccessRange::Whole).unwrap();
    reader.render(|_| true);
    let definition = graph_definition(graph).unwrap();
    assert!(!definition.passes[0].accesses[0].reads_previous);
    let plan = rhapsody::rg::plan::compile_definition(definition).unwrap();
    assert_eq!(plan.retained_passes, [0, 1]);
    assert!(plan.dependency_edges.contains(&(0, 1)));
}

#[test]
fn read_modify_write_requires_initialized_contents_and_preserves_its_producer() {
    let mut undefined = RenderGraph::new();
    let buffer = undefined.create_buffer("undefined", buffer_desc()).unwrap();
    let mut update = undefined.add_pass("in-place update");
    update
        .write_buffer(buffer, 0x37, AccessRange::Whole)
        .unwrap();
    update.render(|_| true);
    assert_eq!(
        graph_definition(undefined).unwrap_err().kind(),
        rhapsody::ErrorKind::InvalidUsage
    );

    let mut graph = RenderGraph::new();
    let buffer = graph.create_buffer("buffer", buffer_desc()).unwrap();
    let mut first = graph.add_pass("initialize");
    let current = first
        .write_buffer(buffer, WRITE, AccessRange::Whole)
        .unwrap();
    first.render(|_| true);
    let mut update = graph.add_pass("in-place update");
    update.side_effect();
    update
        .write_buffer(current, 0x37, AccessRange::Whole)
        .unwrap();
    update.render(|_| true);
    let definition = graph_definition(graph).unwrap();
    assert!(definition.passes[1].accesses[0].reads_previous);
    let plan = rhapsody::rg::plan::compile_definition(definition).unwrap();
    assert_eq!(plan.retained_passes, [0, 1]);
    assert!(plan.dependency_edges.contains(&(0, 1)));
}

#[test]
fn explicit_discard_initializes_without_reading_previous_contents() {
    let mut graph = RenderGraph::new();
    let buffer = graph.create_buffer("discard", buffer_desc()).unwrap();
    let mut clear = graph.add_pass("discard contents");
    clear.side_effect();
    clear
        .write_buffer_discard(buffer, 0x37, AccessRange::Whole)
        .unwrap();
    clear.render(|_| true);
    let definition = graph_definition(graph).unwrap();
    assert!(!definition.passes[0].accesses[0].reads_previous);
    assert!(rhapsody::rg::plan::compile_definition(definition).is_ok());
}

#[test]
fn raw_upgrade_marks_both_records_as_consuming_previous_contents() {
    let mut graph = RenderGraph::new();
    let buffer = imported(&mut graph);
    let mut update = graph.add_pass("upgrade");
    update.add_resource_access(buffer, 0x19, 4).unwrap();
    update.add_resource_access(buffer, 0x37, 5).unwrap();
    update.render(|_| true);
    let definition = graph_definition(graph).unwrap();
    assert_eq!(definition.passes[0].accesses.len(), 2);
    assert!(
        definition.passes[0]
            .accesses
            .iter()
            .all(|access| access.reads_previous)
    );
}

#[test]
fn partial_read_modify_write_cannot_read_an_unwritten_hole() {
    let mut graph = RenderGraph::new();
    let buffer = graph.create_buffer("partial", buffer_desc()).unwrap();
    let mut first = graph.add_pass("initialize left");
    let current = first.write_buffer(buffer, WRITE, bytes(0, 32)).unwrap();
    first.render(|_| true);
    let mut update = graph.add_pass("update right");
    update.write_buffer(current, 0x37, bytes(32, 32)).unwrap();
    update.render(|_| true);
    assert_eq!(
        graph_definition(graph).unwrap_err().kind(),
        rhapsody::ErrorKind::InvalidUsage
    );
}
