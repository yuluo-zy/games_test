use asset::{
    inheritable::{Inherit, Inheritable},
    prefab::config::*,
};

#[test]
fn bare_enum_option_and_explicit_none_keep_original_ron_rules() {
    for input in [
        "(mesh_processor:Code(name:\"proc\"))",
        "(mesh_processor:Some(Code(name:\"proc\")))",
    ] {
        let value: PrefabRenderConfig = ron::from_str(input).unwrap();
        assert!(
            matches!(value.mesh_processor,Inheritable::Value(Some(MeshProcessorConfig::Code{name})) if name=="proc")
        );
    }
    for input in ["(mesh_processor:None)", "(mesh_processor:())"] {
        let value: PrefabRenderConfig = ron::from_str(input).unwrap();
        assert!(matches!(value.mesh_processor, Inheritable::Value(None)));
    }
    assert!(
        ron::from_str::<PrefabRenderConfig>("(mesh_processor:Unrecognized(name:\"proc\"))")
            .is_err()
    );
    assert!(ron::from_str::<PrefabRtConfig>("(mesh_scale:[1.0,2.0])").is_err());
}
#[test]
fn config_merge_uses_child_fields_missing_parent_parts_and_explicit_none() {
    let mut child: PrefabConfig =
        ron::from_str(r#"(parts:{"body":(render:(shader:"new"),rt:None)})"#).unwrap();
    let parent:PrefabConfig=ron::from_str(r#"(parts:{"body":(render:(mesh:"mesh",shader:"old"),rt:(mesh:Aabb)),"extra":(delete:true)})"#).unwrap();
    child.inherit_from(&parent);
    let render = child.parts["body"].render.clone().into_concrete();
    assert_eq!(render.mesh.into_concrete(), "mesh");
    assert_eq!(render.shader.into_concrete(), "new");
    assert!(child.parts["body"].rt.clone().into_concrete().is_none());
    assert!(child.parts["extra"].delete.clone().into_concrete());
}

#[test]
fn shallow_inheritance_preserves_concrete_children_and_unresolved_markers() {
    use std::collections::HashMap;
    let parent = Inheritable::Value(HashMap::from([("parent", String::from("v"))]));
    let mut shallow = Inheritable::Value(HashMap::new());
    shallow.inherit_shallow(&parent);
    assert_eq!(shallow, Inheritable::Value(HashMap::new()));
    let mut deep = Inheritable::Value(HashMap::new());
    deep.inherit_from(&parent);
    assert_eq!(deep, parent);

    for parent in [
        Inheritable::Inherit,
        Inheritable::MissingDefault,
        Inheritable::Value(None),
        Inheritable::Value(Some(7)),
    ] {
        let mut missing = Inheritable::Inherit;
        missing.inherit_shallow(&parent);
        assert_eq!(missing, parent);
        let mut disabled = Inheritable::Value(None);
        disabled.inherit_shallow(&parent);
        assert_eq!(disabled, Inheritable::Value(None));
        let mut default_missing = Inheritable::MissingDefault;
        default_missing.inherit_shallow(&parent);
        assert_eq!(default_missing, Inheritable::MissingDefault);
    }
}

#[test]
fn concrete_borrow_preserves_identity_and_unresolved_fields_return_errors() {
    use asset::inheritable::UnresolvedField;
    let input = Inheritable::Value(String::from("borrowed"));
    let Inheritable::Value(value) = &input else {
        unreachable!()
    };
    assert!(std::ptr::eq(input.try_as_concrete().unwrap(), value));
    assert!(std::ptr::eq(input.as_concrete(), value));
    assert_eq!(
        Inheritable::<bool>::Inherit.try_as_concrete(),
        Err(UnresolvedField::InheritanceRequired)
    );
    assert_eq!(
        Inheritable::<bool>::MissingDefault.try_as_concrete(),
        Err(UnresolvedField::MissingDefault)
    );
}

#[test]
fn explicit_deserializer_delegates_typed_values_and_preserves_errors() {
    use asset::inheritable::deserialize_inheritable;
    #[derive(serde::Deserialize, Default)]
    #[serde(default)]
    struct Fields {
        #[serde(deserialize_with = "deserialize_inheritable")]
        flag: Inheritable<bool>,
    }
    assert_eq!(
        ron::from_str::<Fields>("()").unwrap().flag,
        Inheritable::Inherit
    );
    assert_eq!(
        ron::from_str::<Fields>("(flag:false)").unwrap().flag,
        Inheritable::Value(false)
    );
    assert_eq!(
        ron::from_str::<Fields>("(flag:true)").unwrap().flag,
        Inheritable::Value(true)
    );
    for input in ["7", "None", "\"true\""] {
        let direct = ron::from_str::<bool>(input).unwrap_err();
        let wrapped = ron::from_str::<Inheritable<bool>>(input).unwrap_err();
        assert_eq!(direct, wrapped);
    }
    // Ordinary typed Option keeps its ordinary serde rules; the permissive RON
    // option field adapter is a distinct original private interface.
    assert_eq!(
        ron::from_str::<Inheritable<Option<u32>>>("None").unwrap(),
        Inheritable::Value(None)
    );
    assert!(ron::from_str::<Inheritable<Option<u32>>>("7").is_err());
}

#[test]
fn concrete_texture_defaults_resolve_missing_fields_without_replacing_explicit_none() {
    use asset::texture::{
        config::InheritableTextureOpts,
        desc::{CompressionMethod, Gamma, MipChain},
    };
    let unresolved = InheritableTextureOpts::default();
    assert!(matches!(unresolved.resize, Inheritable::Inherit));
    assert!(matches!(unresolved.gamma, Inheritable::Inherit));
    let defaults = InheritableTextureOpts::concrete_default();
    assert_eq!(defaults.resize, Inheritable::Value(None));
    assert_eq!(defaults.gamma, Inheritable::Value(Gamma::Linear));
    assert_eq!(defaults.mips, Inheritable::Value(MipChain::None));
    assert_eq!(
        defaults.compression,
        Inheritable::Value(CompressionMethod::Color)
    );
    let mut child: InheritableTextureOpts =
        ron::from_str("(resize:None,compression:None)").unwrap();
    child.inherit_from(&defaults);
    assert_eq!(child.resize, Inheritable::Value(None));
    assert_eq!(
        child.compression,
        Inheritable::Value(CompressionMethod::None)
    );
    assert_eq!(child.gamma, defaults.gamma);
    assert_eq!(child.mips, defaults.mips);
}

#[test]
fn prefab_render_some_keeps_missing_required_fields_unresolved() {
    use asset::inheritable::Inheritable;
    use asset::prefab::config::PrefabConfig;
    let mut config: PrefabConfig = ron::from_str(r#"(parts:{"p":(render:(mesh:"m"))})"#).unwrap();
    config.inherit_from(&PrefabConfig::default());
    let part = &config.parts["p"];
    let render = part.render.as_concrete();
    assert_eq!(render.mesh.as_concrete(), "m");
    assert!(matches!(render.shader, Inheritable::MissingDefault));
    assert!(matches!(render.mesh_processor, Inheritable::Value(None)));
}
