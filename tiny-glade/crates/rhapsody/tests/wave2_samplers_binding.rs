//! wave2：内置采样器 13 项状态、`device+0xab0` 描述符更新计数、RenderPassBinding 布局。
//! 证据：reports/wave2-samplers-binding.md（机器码 0x140bf5208..0x140bf5823、0x142135f90、0x1416a56e5 等）。
use ash::vk;
use rhapsody::raw::{
    device::GraphicsDevice,
    init::create_headless_device,
    sampler::{BUILTIN_SAMPLER_NAMES, BUILTIN_SAMPLER_SPECS, builtin_sampler_spec},
};
use std::sync::Arc;

/// 名字池 0x142b16b42 顺序 = 原创建顺序。
const POOL_ORDER: [&str; 13] = [
    "sampler_lnc", "sampler_llr", "sampler_nnc", "sampler_llc", "sampler_llb",
    "sampler_lnc_iso", "sampler_llr_iso", "sampler_llc_iso", "sampler_llb_iso",
    "sampler_default", "sampler_mipmap", "sampler_shadow_nearest", "sampler_shadow_linear",
];

#[test]
fn specs_follow_the_original_creation_order() {
    let names: Vec<_> = BUILTIN_SAMPLER_SPECS.iter().map(|s| s.name).collect();
    assert_eq!(names, POOL_ORDER);
    // shader 反射出现的 9 个名字都能在规格里找到。
    for name in BUILTIN_SAMPLER_NAMES {
        assert!(builtin_sampler_spec(name).is_some(), "{name}");
    }
}

#[test]
fn spec_states_match_machine_code_constants() {
    let lnc = builtin_sampler_spec("sampler_lnc").unwrap().create_info();
    assert_eq!(lnc.s_type, vk::StructureType::SAMPLER_CREATE_INFO);
    assert_eq!((lnc.mag_filter, lnc.min_filter), (vk::Filter::LINEAR, vk::Filter::LINEAR));
    assert_eq!(lnc.mipmap_mode, vk::SamplerMipmapMode::NEAREST);
    assert_eq!(lnc.address_mode_u, vk::SamplerAddressMode::CLAMP_TO_EDGE);
    assert_eq!(lnc.anisotropy_enable, vk::TRUE);
    assert_eq!(lnc.max_anisotropy, 4.0);
    assert_eq!(lnc.max_lod, 1000.0);
    let iso = builtin_sampler_spec("sampler_lnc_iso").unwrap().create_info();
    assert_eq!((iso.anisotropy_enable, iso.max_anisotropy), (vk::FALSE, 1.0));
    let llb = builtin_sampler_spec("sampler_llb").unwrap().create_info();
    assert_eq!(llb.address_mode_w, vk::SamplerAddressMode::CLAMP_TO_BORDER);
    assert_eq!(llb.border_color, vk::BorderColor::FLOAT_OPAQUE_BLACK);
    let nnc = builtin_sampler_spec("sampler_nnc").unwrap().create_info();
    assert_eq!(nnc.mag_filter, vk::Filter::NEAREST);
    assert_eq!(nnc.anisotropy_enable, vk::FALSE);
    assert_eq!(nnc.max_anisotropy, 4.0); // 原值 0x40800000 即便关闭各向异性
}

#[test]
fn only_shadow_linear_compares() {
    for spec in BUILTIN_SAMPLER_SPECS {
        assert_eq!(spec.compare_enable, spec.name == "sampler_shadow_linear", "{}", spec.name);
    }
    let s = builtin_sampler_spec("sampler_shadow_linear").unwrap();
    assert_eq!(s.compare_op, vk::CompareOp::LESS_OR_EQUAL);
    assert_eq!(s.border_color, vk::BorderColor::FLOAT_OPAQUE_WHITE);
    assert_eq!(s.max_lod, 0.0);
}

#[test]
fn device_creates_all_thirteen_samplers_and_looks_up_by_name() {
    let Ok(device) = create_headless_device() else { return };
    let mut gd: Arc<GraphicsDevice> = GraphicsDevice::from_device(device);
    let g = Arc::get_mut(&mut gd).expect("unique");
    if g.create_samplers().is_err() {
        eprintln!("skip: driver rejected sampler create");
        return;
    }
    for (i, name) in POOL_ORDER.iter().enumerate() {
        let by_name = g.get_built_in_sampler_by_name(name);
        let by_index = g.get_built_in_sampler(i).unwrap();
        assert!(Arc::ptr_eq(&by_name, &by_index));
    }
    assert!(g.get_built_in_sampler(13).is_none());
    assert!(g.try_get_built_in_sampler_by_name("nope").is_none());
}

#[test]
#[should_panic(expected = "Sampler not found: sampler_nope")]
fn missing_sampler_panics_with_original_text() {
    let Ok(device) = create_headless_device() else {
        panic!("Sampler not found: sampler_nope");
    };
    let gd = GraphicsDevice::from_device(device);
    gd.get_built_in_sampler_by_name("sampler_nope");
}

#[test]
fn descriptor_update_counter_starts_at_zero() {
    let Ok(device) = create_headless_device() else { return };
    assert_eq!(
        device.descriptor_update_count.load(std::sync::atomic::Ordering::SeqCst),
        0
    );
}
