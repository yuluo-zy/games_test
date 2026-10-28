use super::*;
use std::collections::BTreeSet;
use crate::rg::graph::Descriptor;
use crate::rg::graph::OriginSource;
use crate::rg::{graph::RenderGraph, resource::ResourceOrigin, vulkan::AccessTable};
use crate::rg::resource_registry::BufferDesc;
const READ: u8 = 0x25;
const WRITE: u8 = 0x34;
/// `passes[i] = (code, side_effect)` all on one created buffer.
fn plan(passes: &[(u8, bool)]) -> CompiledPlan {
    let mut graph = RenderGraph::new();
    let descriptor = Descriptor::Buffer(BufferDesc {
        word_at_00: 64,
        word_at_08: 0,
        bytes_at_0c_through_0e: [0; 3],
    });
    let buffer = graph
        .create_raw_resource(
            "b",
            ResourceOrigin {
                descriptor,
                source: OriginSource::Create,
            },
        )
        .unwrap();
    for (index, &(code, side_effect)) in passes.iter().enumerate() {
        let mut pass = graph.add_pass(format!("p{index}"));
        if side_effect {
            pass = pass.with_side_effect();
        }
        pass.add_resource_access(buffer, code, 0).unwrap();
        pass.render(|_| true);
    }
    let declaration = graph.build().declaration;
    let definition = GraphDefinition::from_declaration(
        &declaration,
        &AccessTable::original(),
        PlanContext::default(),
    )
    .unwrap();
    compile_definition(definition).unwrap()
}
fn reaches(edges: &[(usize, usize)], from: usize, to: usize) -> bool {
    let mut pending = vec![from];
    let mut seen = BTreeSet::new();
    while let Some(next) = pending.pop() {
        for &(a, b) in edges {
            if a == next && seen.insert(b) {
                pending.push(b);
            }
        }
    }
    seen.contains(&to)
}
#[test]
fn culled_middle_writer_keeps_the_surviving_writers_ordered() {
    let plan = plan(&[(WRITE, true), (WRITE, false), (WRITE, true)]);
    assert_eq!(plan.retained_passes, [0, 2]);
    assert_eq!(plan.dependency_edges, [(0, 2)]);
}
#[test]
fn frontier_keeps_every_conflict_transitively() {
    // W0 R1 R2 W3 W4 R5: each conflicting pair stays ordered.
    let passes = [
        (WRITE, true),
        (READ, true),
        (READ, true),
        (WRITE, true),
        (WRITE, true),
        (READ, true),
    ];
    let plan = plan(&passes);
    for later in 0..passes.len() {
        for earlier in 0..later {
            if passes[earlier].0 == WRITE || passes[later].0 == WRITE {
                assert!(
                    reaches(&plan.dependency_edges, earlier, later),
                    "{earlier} -> {later} in {:?}",
                    plan.dependency_edges
                );
            }
        }
    }
    assert!(
        !reaches(&plan.dependency_edges, 1, 2),
        "two reads stay unordered"
    );
    // Covered entries are pruned: W4 does not need a direct edge from W0.
    assert!(!plan.dependency_edges.contains(&(0, 4)));
}
