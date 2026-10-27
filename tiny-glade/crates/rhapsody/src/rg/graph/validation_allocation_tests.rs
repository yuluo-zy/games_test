use super::*;
use crate::rg::graph::declaration::INLINE_NAME_LIMIT;

fn declaration(names: &[String]) -> GraphDeclaration {
    let origin = crate::rg::resource::ResourceOrigin {
        descriptor: crate::rg::vulkan::ResourceSpec::Buffer(crate::resource::BufferDesc::new_gpu_only(
            16,
            ash::vk::BufferUsageFlags::STORAGE_BUFFER,
        ))
        .descriptor(),
        source: OriginSource::Create,
    };
    let mut graph = RenderGraph::new();
    for name in names {
        graph.create_raw_resource(name.as_str(), origin.clone()).unwrap();
    }
    graph.declaration
}
fn names(count: usize) -> Vec<String> {
    (0..count).map(|index| format!("validate-name-{index}")).collect()
}

#[test]
fn duplicate_names_are_rejected_on_both_the_inline_and_the_set_path() {
    for count in [2, INLINE_NAME_LIMIT, INLINE_NAME_LIMIT + 1, 200] {
        let mut declared = declaration(&names(count));
        assert!(declared.validate().is_ok(), "{count} distinct names");
        // Make the last resource repeat the first one's name.
        declared.resources[count - 1].name = declared.resources[0].name;
        assert!(
            matches!(declared.validate(), Err(BuildError::DuplicateResourceName(_))),
            "{count} names with one duplicate"
        );
    }
}

#[test]
fn declaration_shape_reserves_without_pinning_pathological_sizes() {
    let declared = declaration(&names(5));
    let shape = DeclarationShape::of(&declared);
    let graph = RenderGraph::with_shape(shape);
    assert!(graph.declaration.resources.capacity() >= 5);
    assert!(graph.current_versions.capacity() >= 5);
    assert_eq!(graph.declaration.resources.len(), 0);
    // The union keeps the larger of two alternating graphs.
    let small = DeclarationShape::of(&declaration(&names(2)));
    assert_eq!(small.union(shape), shape.union(small));
    assert_eq!(small.union(shape), shape);
}
