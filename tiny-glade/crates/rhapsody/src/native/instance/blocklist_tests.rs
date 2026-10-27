use super::*;

#[test]
fn env_chain_matches_original_write_order() {
    let mut writes = Vec::new();
    use crate::native::instance_layers::{GraphicsInstanceLayerAllowlist, GraphicsInstanceLayerAllowlistEntry as E};
    let allowlist = GraphicsInstanceLayerAllowlist {
        version: 1,
        entries: vec![
            E::Layer("VK_LAYER_foo".into()),
            E::Vendor("NV".into()),
            E::Layer("other".into()),
        ],
    };
    apply_vulkan_blocklist_with(&allowlist, |name, value| {
        writes.push((name.to_owned(), value.to_owned()));
        Ok(())
    });
    assert_eq!(writes.len(), 18);
    assert_eq!(
        writes[0],
        ("VK_LOADER_LAYERS_DISABLE".into(), "~implicit~".into())
    );
    assert_eq!(
        writes[1],
        (
            "VK_LOADER_LAYERS_ALLOW".into(),
            "VK_LAYER_foo,VK_LAYER_NV*,other".into()
        )
    );
    assert!(writes[2..].iter().all(|(_, v)| v == "1"));
    assert_eq!(writes[10].0, "EOS_OVERLAY_DISABLE_VULKAN_WIN64");
    assert_eq!(writes[10].0.len(), 32);
    assert_eq!(writes[17].0, "DISABLE_VK_LAYER_gshade_1");
}

#[test]
fn env_name_lengths_match_the_string_pool() {
    let lens: Vec<usize> = VULKAN_LAYER_DISABLE_ENV.iter().map(|n| n.len()).collect();
    assert_eq!(
        lens,
        [
            18, 16, 32, 15, 20, 20, 26, 39, 32, 29, 31, 26, 29, 16, 13, 25
        ]
    );
}

#[test]
#[should_panic(expected = "failed to set environment variable `VK_LOADER_LAYERS_DISABLE`")]
fn set_var_failure_panics() {
    apply_vulkan_blocklist_with(&Default::default(), |_, _| {
        Err(std::io::Error::other("boom"))
    });
}
