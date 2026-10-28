use country_core::asset_libraries::mesh_library::{AssetMeshLibrary, MeshAsset};
use country_core::render::mesh::{Mesh, SourceFile, ATTR_POSITION};
use country_core::utils::load_json::try_load_json_as_mesh;
use glam::Vec3;

#[test]
fn hot_reload_accepts_position_type_that_panics_when_read_as_positions() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("mesh.json");
    let mut old = Mesh::new("previous-valid-mesh");
    old.set_source_file(SourceFile { path: path.to_str().unwrap().into(), stamp: 0 });
    old.set_attribute(ATTR_POSITION, vec![Vec3::ZERO]);
    old.set_indices(vec![0]);
    let mut library = AssetMeshLibrary::new();
    let handle = library.add(MeshAsset::new(old, None));
    std::fs::write(&path, r#"{
        "indices":{"buffer":[0]}, "attributes":["Vertex_Position"],
        "Vertex_Position":{"type":["float",2],"buffer":[[1.0,2.0]]}
    }"#).unwrap();
    library.try_reload_mesh_src(&path).unwrap();
    let mesh = library.get(handle).unwrap();
    assert_eq!(mesh.label, "load_json_as_mesh");
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        mesh.positions_without_indices();
    })).is_err());
}

#[test]
fn recoverable_loader_returns_errors_for_missing_files_and_bad_json() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("mesh.json");
    assert!(try_load_json_as_mesh(path.to_str().unwrap()).is_err());
    std::fs::write(&path, "{").unwrap();
    assert!(try_load_json_as_mesh(path.to_str().unwrap()).is_err());
}
