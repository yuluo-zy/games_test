//! raw-core 域·描述符/着色器管线/渲染通道/绑定测试。
//! 需要 Vulkan 设备的用例在无设备时跳过（本机用 lavapipe）。
use ash::vk;
use rhapsody::raw::{
    init::create_headless_device,
    render_pass::{
        AttachmentArray, DepthAccess, RenderPass, RenderPassAttachmentDesc, RenderPassDesc,
    },
    sampler::BUILTIN_SAMPLER_NAMES,
};

fn color(format: vk::Format) -> RenderPassAttachmentDesc {
    RenderPassAttachmentDesc::new(format)
}

#[test]
fn render_pass_attachment_key_compares_only_live_entries() {
    // 哈希/相等只看有效切片。
    let mut a = AttachmentArray::new();
    a.push(color(vk::Format::R8G8B8A8_UNORM));
    let mut b = a;
    assert_eq!(a, b);
    b.push(color(vk::Format::R8G8B8A8_UNORM));
    assert_ne!(a, b);
}

#[test]
#[should_panic(expected = "capacity exceeded")]
fn ninth_attachment_panics_like_arrayvec_push() {
    let mut a = AttachmentArray::new();
    for _ in 0..9 {
        a.push(color(vk::Format::R8G8B8A8_UNORM));
    }
}

#[test]
fn depth_access_tags_decode_like_the_jump_table() {
    // 0x142b174cc：tag0→4(READ_ONLY)，tag1→3(ATTACHMENT)，tag2→1000117001，tag3→无深度。
    assert_eq!(
        DepthAccess::from_tag(0).unwrap().unwrap().layout().as_raw(),
        4
    );
    assert_eq!(
        DepthAccess::from_tag(1).unwrap().unwrap().layout().as_raw(),
        3
    );
    assert_eq!(
        DepthAccess::from_tag(2).unwrap().unwrap().layout().as_raw(),
        0x3b9c_9309
    );
    assert_eq!(DepthAccess::from_tag(3), Some(None));
    assert_eq!(DepthAccess::from_tag(4), None);
}

#[test]
fn pass_samples_follow_the_original_selection() {
    let mut desc = RenderPassDesc::default();
    // 无附件：原版 `attachments[0]` 越界 panic，这里是错误。
    assert!(desc.pass_samples().is_err());
    desc.colors
        .push(color(vk::Format::R8G8B8A8_UNORM).sample_count(4));
    assert_eq!(desc.pass_samples().unwrap(), 4);
    // 有深度时取深度附件的采样数。
    desc.depth = Some((
        color(vk::Format::D32_SFLOAT).sample_count(2),
        DepthAccess::ReadWrite,
    ));
    assert_eq!(desc.pass_samples().unwrap(), 2);
}

#[test]
fn flat_conversion_splits_depth_and_matches_attachment_key_semantics() {
    let attachments = [
        color(vk::Format::R8G8B8A8_UNORM).clear_input(),
        color(vk::Format::D32_SFLOAT),
    ];
    let desc = RenderPassDesc::from_flat(&attachments);
    assert_eq!(desc.colors.len(), 1);
    assert_eq!(desc.depth.unwrap().1, DepthAccess::ReadWrite);
    // 平铺键与 from_desc 同语义：load 取 clear_input 原值，stencil 恒 LOAD/STORE，initial=final。
    let key = attachments[0].to_attachment_key();
    assert_eq!(key.load, vk::AttachmentLoadOp::CLEAR.as_raw());
    assert_eq!(key.stencil_load, vk::AttachmentLoadOp::LOAD.as_raw());
    assert_eq!(key.stencil_store, vk::AttachmentStoreOp::STORE.as_raw());
    assert_eq!(
        key.initial_layout,
        vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL.as_raw()
    );
    assert_eq!(key.final_layout, key.initial_layout);
}

#[test]
fn from_desc_creates_the_original_render_pass_on_a_real_device() {
    let Ok(device) = create_headless_device() else {
        return;
    };
    let mut desc = RenderPassDesc::default();
    desc.colors
        .push(color(vk::Format::R8G8B8A8_UNORM).clear_input());
    desc.colors.push(color(vk::Format::R8G8B8A8_UNORM));
    desc.depth = Some((color(vk::Format::D32_SFLOAT), DepthAccess::ReadOnly));
    let pass = RenderPass::from_desc(device.clone(), &desc).unwrap();
    assert_ne!(pass.handle, vk::RenderPass::null());
    // 槽 +0x18 = 颜色附件数（不含深度），+0x20 = 采样数。
    assert_eq!((pass.color_count, pass.samples), (2, 1));
    // 只有深度也可以创建（采样数取深度）。
    let depth_only = RenderPassDesc {
        depth: Some((
            color(vk::Format::D32_SFLOAT).sample_count(1),
            DepthAccess::ReadWrite,
        )),
        ..Default::default()
    };
    let pass = RenderPass::from_desc(device, &depth_only).unwrap();
    assert_eq!((pass.color_count, pass.samples), (0, 1));
}

#[test]
fn builtin_sampler_names() {
    assert_eq!(BUILTIN_SAMPLER_NAMES.len(), 9);
    assert!(BUILTIN_SAMPLER_NAMES.contains(&"sampler_shadow_linear"));
}
