#[test]
fn all_shipped_prefabs_parse_and_merge_dependencies() {
    let root = std::path::Path::new("D:/game/ljxsj_92385/Tiny Glade/assets/prefabs");
    let result = asset::prefab::scan::scan_asset_tree(root).unwrap();
    assert!(!result.prefabs.is_empty());
    assert!(!result.dependencies.by_file.is_empty());
    println!(
        "{} prefabs and {} dependency files",
        result.prefabs.len(),
        result.dependencies.by_file.len()
    );
}
