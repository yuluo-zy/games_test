use super::*;
use crate::rg::graph::GraphDeclaration;
use crate::rg::graph::OriginSource;
use crate::rg::resource::AccessRange;
use crate::rg::{
    graph::RenderGraph,
    resource_info::PreviousContentRead,
    vulkan::{AccessTable, ResourceSpec},
};

fn top() -> AccessRange {
    AccessRange::Texture {
        aspect_mask: 1,
        base_mip_level: 1,
        level_count: 1,
        base_array_layer: 0,
        layer_count: 1,
    }
}
fn raw(strict_alias: bool, preceding_writer: bool, scoped: bool) -> GraphDeclaration {
    let mut graph = RenderGraph::new();
    let descriptor = ResourceSpec::Image(
        crate::resource::texture::TextureDesc::new_2d(16, 16, ash::vk::Format::R32_SFLOAT)
            .mip_levels(2),
    )
    .descriptor();
    let image = graph.import_unowned("full", descriptor, 7, 0x19).unwrap();
    if preceding_writer {
        let mut pass = graph.add_pass("real writer");
        pass.add_resource_access_range(image, 0x37, 0, top())
            .unwrap();
        pass.render(|_| true);
    }
    let declare = |graph: &mut RenderGraph| {
        let mut pass = graph.add_pass("top alias").with_side_effect();
        pass.add_undefined_content_read_range(image, 0x19, 1, top())
            .unwrap();
        pass.add_resource_access_range(image, 0x37, 0, top())
            .unwrap();
        if strict_alias {
            pass.add_preserving_resource_access_range(image, 0x37, 0, top())
                .unwrap();
        }
        pass.render(|_| true);
    };
    if scoped {
        graph
            .subgraph("reverse", |scope| {
                scope.input(image, top(), 0x19)?;
                declare(scope.graph());
                scope.output(image, top(), 0x37)?;
                Ok(())
            })
            .unwrap();
    } else {
        declare(&mut graph);
    }
    graph.build().declaration
}
fn definition(raw: &GraphDeclaration) -> crate::Result<GraphDefinition> {
    GraphDefinition::from_declaration(raw, &AccessTable::original(), PlanContext::default())
}

#[test]
fn borrowed_import_hit_rechecks_entry_content_for_saved_and_strict_aliases() {
    for strict in [false, true] {
        let mut raw = raw(strict, false, false);
        let key = definition(&raw).unwrap();
        raw.resources[0].source = OriginSource::ImportUnowned {
            token: 8,
            initial_access: 0,
        };
        let borrowed =
            key.matches_declaration(&raw, &AccessTable::original(), PlanContext::default());
        assert_eq!(borrowed.is_ok(), !strict);
        if !strict {
            assert!(borrowed.unwrap());
        }
        assert_eq!(definition(&raw).is_ok(), !strict);
    }
}

#[test]
fn saved_policy_is_in_full_borrowed_subgraph_and_colliding_cache_keys() {
    #[derive(Clone, PartialEq, Eq)]
    struct Collision(GraphDefinition);
    impl std::hash::Hash for Collision {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            state.write_u8(0);
        }
    }
    let mut raw = raw(false, true, true);
    let allowed = definition(&raw).unwrap();
    raw.passes[1].accesses[0].previous_content_read = PreviousContentRead::DefinedRequired;
    let strict = definition(&raw).unwrap();
    assert_ne!(allowed, strict);
    assert!(
        !allowed
            .matches_declaration(&raw, &AccessTable::original(), PlanContext::default())
            .unwrap()
    );
    let mut full = crate::collections::cache::BoundedCache::new(4, 4096 * 1024);
    full.insert(Collision(allowed.clone()), 1u32, 1);
    full.insert(Collision(strict.clone()), 2u32, 1);
    assert_eq!(full.get(&Collision(allowed.clone())), Some(&1));
    assert_eq!(full.get(&Collision(strict.clone())), Some(&2));
    let mut cache = super::super::subgraph::SubgraphPlanCache::default();
    for input in [allowed.clone(), allowed, strict] {
        let cold = compile_definition(input.clone()).unwrap();
        let cached = compile_with_subgraphs(input, &mut cache).unwrap();
        assert_eq!(cold.retained_passes, [0, 1]);
        assert_eq!(cold.retained_passes, cached.retained_passes);
        assert_eq!(cold.dependency_edges, cached.dependency_edges);
        assert_eq!(cold.range_program, cached.range_program);
        assert!(cached.dependency_edges.contains(&(0, 1)));
    }
    assert_eq!(cache.stats().compilations, 2);
    assert_eq!(cache.stats().hits, 1);
}

#[test]
fn independent_check_rejects_strict_same_pass_self_initialization_and_bad_saved_shape() {
    let allowed = definition(&raw(false, false, false)).unwrap();
    let mut strict = allowed.clone();
    strict.resources[0].origin = ResourceOriginKind::Create;
    strict.passes[0].accesses[1].previous_content_read = PreviousContentRead::DefinedRequired;
    assert!(super::super::plan_check::check_definition(&strict).is_err());
    let mut malformed = allowed;
    malformed.passes[0].accesses[0].access_code = 0x19;
    assert!(super::super::plan_check::check_definition(&malformed).is_err());
}
