use asset::texture::blob::TextureAssetBlob;

#[test]
fn original_b_archives_decode_and_roundtrip() {
    let root = std::path::Path::new("D:/game/ljxsj_92385/Tiny Glade/compiled-assets/textures");
    let mut count = 0;
    let names: Vec<relative_path::RelativePathBuf> =
        ron::from_str(&std::fs::read_to_string(root.join("list.ron")).unwrap()).unwrap();
    for name in names {
        let path = name.with_extension("texture").to_path(root);
        let bytes = std::fs::read(&path).unwrap();
        let blob =
            TextureAssetBlob::load(&bytes).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        assert!(blob.dimensions.iter().all(|d| *d > 0));
        assert!(blob.mips > 0 && blob.layers > 0);
        assert_eq!(blob.to_file_bytes().unwrap(), bytes, "{}", path.display());
        count += 1;
    }
    assert_eq!(count, 388, "expected the shipped B texture corpus");
    println!("{count} original texture archives decoded and byte-identical roundtripped");
}
