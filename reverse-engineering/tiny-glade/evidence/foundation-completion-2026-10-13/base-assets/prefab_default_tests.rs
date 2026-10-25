use super::*;
#[test]
fn enabled_optional_objects_finalize_without_an_enabled_parent() {
    let mut config:PrefabConfig=ron::from_str(r#"(parts:{"p":(render:(mesh:"m",shader:"s"),rt:Some(()),audio:(trigger:()))})"#).unwrap();
    config.inherit_from(&PrefabConfig::default());
    let part=&config.parts["p"];
    let rt=part.rt.as_concrete().as_ref().unwrap();
    assert!(matches!(rt.mesh,Inheritable::MissingDefault));
    assert_eq!(rt.mesh_scale.as_concrete(),&[1.0;3]);
    assert_eq!(rt.summer_albedo.as_concrete(),&[1.0;3]);
    assert_eq!(rt.autumn_albedo.as_concrete(),&[1.0;3]);
    assert_eq!(rt.winter_albedo.as_concrete(),&[1.0;3]);
    assert_eq!(rt.semantic.as_concrete(),&RtMeshSemantic::None);
    let trigger=part.audio.as_concrete().as_ref().unwrap().trigger.as_concrete();
    assert!(matches!(trigger.shape,Inheritable::MissingDefault));
    assert!(matches!(trigger.kind,Inheritable::MissingDefault));
    assert_eq!(trigger.shape_scale.as_concrete(),&[1.0;3]);
}
#[test]
fn inherited_explicit_values_precede_deep_defaults_and_none_remains_disabled() {
    let parent:PrefabConfig=ron::from_str(r#"(parts:{"p":(render:(mesh:"m",shader:"s"),rt:(mesh:Aabb,mesh_scale:[0.5,0.6,0.7],semantic:Canopy),audio:(trigger:(shape:Aabb,kind:"Tree")))})"#).unwrap();
    let mut child:PrefabConfig=ron::from_str(r#"(parts:{"p":(rt:(summer_albedo:[0.1,0.2,0.3]),audio:None)})"#).unwrap();
    child.inherit_from(&parent);
    let part=&child.parts["p"];
    let rt=part.rt.as_concrete().as_ref().unwrap();
    assert_eq!(rt.mesh.as_concrete(),&RtMesh::Aabb);
    assert_eq!(rt.mesh_scale.as_concrete(),&[0.5,0.6,0.7]);
    assert_eq!(rt.summer_albedo.as_concrete(),&[0.1,0.2,0.3]);
    assert_eq!(rt.autumn_albedo.as_concrete(),&[1.0;3]);
    assert_eq!(rt.semantic.as_concrete(),&RtMeshSemantic::Canopy);
    assert!(part.audio.as_concrete().is_none());
}
#[test]
#[should_panic(expected="cannot finalize an exhausted audio trigger")]
fn original_exhausted_trigger_object_is_rejected() {
    let mut audio=Some(PrefabAudioConfig {trigger:Inheritable::MissingDefault});
    audio.inherit_from(&None);
}

mod native_completion {include!("prefab_native_tests.rs");}
