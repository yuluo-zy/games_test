//! wave2：内置采样器规格与创建、描述符更新计数。
use ash::vk;
use rhapsody::resource::sampler::{BUILTIN_SAMPLER_NAMES, BUILTIN_SAMPLER_SPECS, builtin_sampler_spec};
use rhapsody::device::selection::create_headless_device;
use std::sync::Arc;

/// 内置采样器的创建/索引顺序。
const POOL_ORDER: [&str; 13] = [
    "sampler_lnc",
    "sampler_llr",
    "sampler_nnc",
    "sampler_llc",
    "sampler_llb",
    "sampler_lnc_iso",
    "sampler_llr_iso",
    "sampler_llc_iso",
    "sampler_llb_iso",
    "sampler_default",
    "sampler_mipmap",
    "sampler_shadow_nearest",
    "sampler_shadow_linear",
];

#[test]
fn specs_follow_the_creation_order() {
    let names: Vec<_> = BUILTIN_SAMPLER_SPECS.iter().map(|s| s.name).collect();
    assert_eq!(names, POOL_ORDER);
    // shader 反射出现的 9 个名字都能在规格里找到。
    for name in BUILTIN_SAMPLER_NAMES {
        assert!(builtin_sampler_spec(name).is_some(), "{name}");
    }
}

#[test]
fn only_shadow_linear_compares() {
    for spec in BUILTIN_SAMPLER_SPECS {
        assert_eq!(
            spec.compare_enable,
            spec.name == "sampler_shadow_linear",
            "{}",
            spec.name
        );
    }
    let s = builtin_sampler_spec("sampler_shadow_linear").unwrap();
    assert_eq!(s.compare_op, vk::CompareOp::LESS_OR_EQUAL);
    assert_eq!(s.border_color, vk::BorderColor::FLOAT_OPAQUE_WHITE);
    assert_eq!(s.max_lod, 0.0);
}

#[test]
fn device_creates_all_thirteen_samplers_and_looks_up_by_name() {
    let Ok(device) = create_headless_device() else {
        return;
    };
    let g = device;
    for (i, name) in POOL_ORDER.iter().enumerate() {
        let by_name = g.get_built_in_sampler_by_name(name).unwrap();
        let by_index = g.get_built_in_sampler(i).unwrap().unwrap();
        assert!(Arc::ptr_eq(&by_name, &by_index));
    }
    assert!(g.get_built_in_sampler(13).unwrap().is_none());
    assert!(
        g.try_get_built_in_sampler_by_name("nope")
            .unwrap()
            .is_none()
    );
}

#[test]
fn descriptor_update_counter_starts_at_zero() {
    let Ok(device) = create_headless_device() else {
        return;
    };
    assert_eq!(device.descriptor_update_count(), 0);
}
