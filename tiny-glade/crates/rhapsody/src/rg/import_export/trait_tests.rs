use super::*;

use crate::rg::graph::OriginSource;

fn tex() -> PhysicalTexture {
    PhysicalTexture {
        desc: TextureDesc {
            words_at_00_through_18: [1, 2, 3, 4, 5, 6, 7],
            bytes_at_1c_through_1e: [0; 3],
        },
        token: 41,
    }
}

#[test]
fn unowned_and_owned_imports_differ_only_in_origin_tag() {
    let mut graph = RenderGraph::new();
    let a = tex().import_unowned(&mut graph, "a", 0).unwrap();
    let b = tex().import_owned(&mut graph, "b", 0).unwrap();
    let sources: Vec<_> = graph
        .declaration
        .resources
        .iter()
        .map(|r| r.source)
        .collect();
    assert!(matches!(
        sources[a.raw() as usize],
        OriginSource::ImportUnowned { token: 41, .. }
    ));
    assert!(matches!(
        sources[b.raw() as usize],
        OriginSource::ImportOwned { token: 41, .. }
    ));
}

#[test]
fn from_owned_rg_resource_checks_the_resource_kind() {
    let exported = ExportedResource::new(0, 9);
    let t = PhysicalTexture::from_owned_rg_resource(tex().descriptor(), exported).unwrap();
    assert_eq!(t.token, 9);
    assert!(PhysicalBuffer::from_owned_rg_resource(tex().descriptor(), exported).is_none());
}
