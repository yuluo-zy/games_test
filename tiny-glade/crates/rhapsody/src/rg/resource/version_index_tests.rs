use super::*;
use crate::rg::graph::RenderGraph;
use crate::rg::resource_registry::BufferDesc;
fn scan(graph: &RenderGraph, handle: ResourceHandle) -> Option<u32> {
    let mut version = 0u32;
    for access in graph.declaration.passes.iter().flat_map(|p| &p.accesses)
        .filter(|a| a.resource_index == handle.raw()) {
        match access.version {
            AccessVersion::Write { current, .. } => version = current,
            AccessVersion::Automatic if crate::native::access_table::is_write(access.access_code) => {
                version = version.checked_add(1)?;
            }
            _ => {}
        }
    }
    Some(version)
}
#[test]
fn committed_index_matches_scan_with_abandoned_passes_and_graph_roundtrip() {
    let mut graph = RenderGraph::new();
    let handles: Vec<_> = (0..31).map(|i| graph.create_buffer(format!("buffer {i}"), BufferDesc {
        word_at_00: 64, word_at_08: 0, bytes_at_0c_through_0e: [0; 3],
    }).unwrap().raw()).collect();
    let mut rng = 0x12345678u32;
    for step in 0..1024 {
        rng ^= rng << 13; rng ^= rng >> 17; rng ^= rng << 5;
        let handle = handles[rng as usize % handles.len()];
        let mut pass = graph.add_pass("random version");
        pass.add_resource_access(handle, if rng & 1 == 0 { 0x34 } else { 0x26 }, 0).unwrap();
        if step % 7 != 0 { pass.render(|_| true); } else { drop(pass); }
        for &h in &handles { assert_eq!(graph.current_version(h).ok(), scan(&graph, h)); }
    }
    let graph = graph.build().into_graph();
    for &h in &handles { assert_eq!(graph.current_version(h).ok(), scan(&graph, h)); }
    let mut other = RenderGraph::new();
    other.create_buffer("other", BufferDesc { word_at_00: 64, word_at_08: 0, bytes_at_0c_through_0e: [0; 3] }).unwrap();
    assert!(matches!(other.current_version(handles[0]), Err(crate::rg::graph::BuildError::WrongGraph { .. })));

    // Keep this a diagnostic, with no hardware-dependent timing assertion.
    let start = std::time::Instant::now();
    let indexed: u64 = (0..100_000).map(|i| graph.current_version(std::hint::black_box(handles[i % handles.len()])).unwrap() as u64).sum();
    let index_ns = start.elapsed().as_nanos();
    let start = std::time::Instant::now();
    let scanned: u64 = (0..100_000).map(|i| scan(&graph, std::hint::black_box(handles[i % handles.len()])).unwrap() as u64).sum();
    assert_eq!(indexed, scanned);
    eprintln!("VERSION_QUERY_DIAGNOSTIC queries=100000 passes=877 index_ns={index_ns} scan_ns={}", start.elapsed().as_nanos());
}
#[test]
fn overflow_is_sticky_and_explicit_versions_are_indexed() {
    use crate::rg::resource_info::AccessDeclaration;
    let mut versions = [Some(0)];
    let mut access = AccessDeclaration { resource_index: 0, access_code: 0x34, ..Default::default() };
    access.version = AccessVersion::Write { previous: 0, current: u32::MAX };
    RenderGraph::advance_versions(&mut versions, &[access]);
    assert_eq!(versions, [Some(u32::MAX)]);
    access.version = AccessVersion::Automatic;
    RenderGraph::advance_versions(&mut versions, &[access]);
    assert_eq!(versions, [None]);
    access.version = AccessVersion::Write { previous: 0, current: 1 };
    RenderGraph::advance_versions(&mut versions, &[access]);
    assert_eq!(versions, [None]);
}
