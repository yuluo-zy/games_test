mod _shipped_root;

#[test]
fn all_shipped_prefabs_parse_and_merge_dependencies() {
    let root = _shipped_root::root().join("assets/prefabs");
    let result = asset::prefab::scan::scan_asset_tree(&root).unwrap();
    assert!(!result.prefabs.is_empty());
    assert!(!result.dependencies.by_file.is_empty());
    println!(
        "{} prefabs and {} dependency files",
        result.prefabs.len(),
        result.dependencies.by_file.len()
    );
}
