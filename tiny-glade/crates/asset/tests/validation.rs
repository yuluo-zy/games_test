use asset::{
    inheritable::{Inherit, Inheritable, UnresolvedField},
    prefab::{config::PrefabConfig, scan as prefab_scan},
    texture::{
        blob::TextureAssetBlob,
        config::InheritableTextureOpts,
        desc::{Gamma, TextureAssetDesc, TextureOpts, validate_logical_name},
        scan as texture_scan,
    },
};
use relative_path::RelativePathBuf;
use std::{
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
};

struct TestDirectory(PathBuf);

impl TestDirectory {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let path = std::env::temp_dir().join(format!(
            "asset-validation-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }

    fn write(&self, name: &str, contents: &str) {
        let path = self.0.join(name);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, contents).unwrap();
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn texture_scan_inherits_options_and_rejects_every_name_collision() {
    let directory = TestDirectory::new();
    directory.write(
        "assets.ron",
        r#"(default:(gamma:sRGB),assets:{"leaf":(resize:None)})"#,
    );
    directory.write("leaf.png", "");
    directory.write("nested/other.jpg", "");
    let scanned = texture_scan::scan_asset_tree(&directory.0).unwrap();
    assert_eq!(
        scanned[&RelativePathBuf::from("leaf")].opts.gamma,
        Gamma::Srgb
    );
    assert_eq!(
        scanned[&RelativePathBuf::from("nested/other")].opts.gamma,
        Gamma::Srgb
    );
    assert_eq!(scanned[&RelativePathBuf::from("leaf")].opts.resize, None);

    for other in ["leaf.jpg", "array_leaf/0.png"] {
        let collision = TestDirectory::new();
        collision.write("leaf.png", "");
        collision.write(other, "");
        let error = texture_scan::scan_asset_tree(&collision.0)
            .unwrap_err()
            .to_string();
        assert!(error.contains("Duplicate texture name"), "{error}");
        assert!(error.contains("leaf.png"), "{error}");
        assert!(
            error.contains(if other.ends_with("jpg") {
                "leaf.jpg"
            } else {
                "array_leaf"
            }),
            "{error}"
        );
    }

    let invalid = TestDirectory::new();
    invalid.write("array_gap/1.png", "");
    assert!(
        texture_scan::scan_asset_tree(&invalid.0)
            .unwrap_err()
            .to_string()
            .contains("missing layer 0")
    );
}

#[test]
fn texture_names_reject_noncanonical_output_paths() {
    for name in [
        "", ".", "..", "/root", "root/", "a//b", "a/./b", "a/../b", "a\\b", "C:drive",
    ] {
        assert!(
            validate_logical_name(relative_path::RelativePath::new(name)).is_err(),
            "{name:?}"
        );
    }
    for name in ["wood", "nested/wood", "木头", "a..b"] {
        validate_logical_name(relative_path::RelativePath::new(name)).unwrap();
    }
    // The array prefix is removed before producing an output name, so a valid
    // directory name can otherwise become an empty or traversal component.
    for array in ["array_", "array_.", "array_..", "nested/array_.."] {
        let directory = TestDirectory::new();
        directory.write(&format!("{array}/0.png"), "");
        let error = texture_scan::scan_asset_tree(&directory.0).unwrap_err();
        let message = format!("{error:#}");
        assert!(
            message.contains("Invalid texture logical name"),
            "{message}"
        );
        assert!(
            message.contains(array.rsplit('/').next().unwrap()),
            "{message}"
        );
    }
}

#[test]
fn prefab_scan_reports_file_part_and_required_field_and_restores_dependency_stack() {
    for (body, field) in [
        (r#"(render:(mesh:"m"))"#, "render.shader"),
        (r#"(render:(mesh:"m",shader:"s"),rt:Some(()))"#, "rt.mesh"),
        (
            r#"(render:(mesh:"m",shader:"s"),audio:(trigger:(shape:Aabb)))"#,
            "audio.trigger.kind",
        ),
    ] {
        let directory = TestDirectory::new();
        directory.write("default.ron", "()");
        directory.write("broken.ron", &format!("(parts:{{\"body\":{body}}})"));
        let mut dependencies = vec![PathBuf::from("ancestor.ron")];
        let mut result = prefab_scan::PrefabScan::default();
        let error = prefab_scan::scan_asset_tree_walker(
            &directory.0,
            &RelativePathBuf::new(),
            &PrefabConfig::default(),
            &mut dependencies,
            &mut result,
        )
        .unwrap_err();
        let message = format!("{error:#}");
        for expected in ["broken.ron", "body", field] {
            assert!(message.contains(expected), "{message}");
        }
        assert_eq!(dependencies, [PathBuf::from("ancestor.ron")]);
    }

    let mut removed: PrefabConfig = ron::from_str(r#"(parts:{"removed":(delete:true)})"#).unwrap();
    removed.inherit_from(&PrefabConfig::default());
    removed.validate().unwrap();
    assert_eq!(
        Inheritable::<bool>::Inherit.try_into_concrete(),
        Err(UnresolvedField::InheritanceRequired)
    );
    assert_eq!(
        Inheritable::<bool>::MissingDefault.try_into_concrete(),
        Err(UnresolvedField::MissingDefault)
    );
    assert!(
        InheritableTextureOpts::default()
            .try_into_concrete()
            .unwrap_err()
            .to_string()
            .contains("resize")
    );
}

#[test]
fn texture_archive_validation_handles_unaligned_and_corrupt_files() {
    let blob = TextureAssetBlob {
        data: vec![1, 2, 3, 4],
        dimensions: [1, 1, 1],
        mips: 1,
        layers: 1,
        format: 37,
        kind: 0,
    };
    let bytes = blob.to_file_bytes().unwrap();
    let mut offset = vec![0];
    offset.extend_from_slice(&bytes);
    TextureAssetBlob::validate_file(&offset[1..]).unwrap();
    assert_eq!(
        TextureAssetBlob::load(&offset[1..])
            .unwrap()
            .to_file_bytes()
            .unwrap(),
        bytes
    );
    for length in [0, 7, 8, bytes.len() - 1] {
        assert!(
            TextureAssetBlob::validate_file(&bytes[..length]).is_err(),
            "length {length}"
        );
    }
    for index in [0, 4] {
        let mut corrupt = bytes.clone();
        corrupt[index] ^= 1;
        assert!(TextureAssetBlob::validate_file(&corrupt).is_err());
    }
}

#[test]
fn source_visitor_preserves_nested_layer_order() {
    fn leaf(path: &str) -> TextureAssetDesc {
        TextureAssetDesc {
            path: path.into(),
            opts: TextureOpts::default(),
            layers: Vec::new(),
        }
    }
    let mut nested = leaf("array-directory");
    nested.layers = vec![leaf("one.png"), leaf("two.dds")];
    let mut array = leaf("root-array");
    array.layers = vec![nested, leaf("three.jpg")];
    let expected = [
        Path::new("one.png"),
        Path::new("two.dds"),
        Path::new("three.jpg"),
    ];
    let mut visited = Vec::new();
    array.visit_source_files(&mut |path| visited.push(path));
    assert_eq!(visited, expected);
    assert_eq!(array.source_files(), expected);
}
