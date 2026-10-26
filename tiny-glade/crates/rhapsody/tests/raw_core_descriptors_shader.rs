//! raw-core 域·描述符/着色器管线/渲染通道/绑定测试。
//! 需要 Vulkan 设备的用例在无设备时跳过（本机用 lavapipe）。
use ash::vk;
use rhapsody::{
    raw::sampler::BUILTIN_SAMPLER_NAMES,
    rg::attachments::contract::{
        AttachmentArray, DepthAccess, RenderPassAttachmentDesc, RenderPassDesc,
    },
};

fn color(format: vk::Format) -> RenderPassAttachmentDesc {
    RenderPassAttachmentDesc::new(format)
}

#[test]
fn render_pass_attachment_key_compares_only_live_entries() {
    // 哈希/相等只看有效切片。
    let mut a = AttachmentArray::new();
    a.push(color(vk::Format::R8G8B8A8_UNORM)).unwrap();
    let mut b = a;
    assert_eq!(a, b);
    b.push(color(vk::Format::R8G8B8A8_UNORM)).unwrap();
    assert_ne!(a, b);
}

#[test]
fn excessive_attachments_return_usage_errors_without_modifying_the_key() {
    let attachment = color(vk::Format::R8G8B8A8_UNORM);
    let mut array =
        AttachmentArray::try_from_iter([attachment; AttachmentArray::CAPACITY]).unwrap();
    let before = array;
    assert_eq!(
        array.push(attachment).unwrap_err().kind(),
        rhapsody::ErrorKind::InvalidUsage
    );
    assert_eq!(array, before);
    assert_eq!(
        AttachmentArray::try_from_iter([attachment; 9])
            .unwrap_err()
            .kind(),
        rhapsody::ErrorKind::InvalidUsage
    );
    assert_eq!(
        RenderPassDesc::from_flat(&[attachment; 9])
            .unwrap_err()
            .kind(),
        rhapsody::ErrorKind::InvalidUsage
    );
    let depth = color(vk::Format::D32_SFLOAT);
    assert_eq!(
        RenderPassDesc::from_flat(&[depth; 2]).unwrap_err().kind(),
        rhapsody::ErrorKind::InvalidUsage
    );
}

#[test]
fn depth_access_codes_reject_unknown_values() {
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
fn attachment_sample_count_requires_an_attachment() {
    let mut desc = RenderPassDesc::default();
    assert!(desc.pass_samples().is_err());
    desc.colors
        .push(color(vk::Format::R8G8B8A8_UNORM).sample_count(4))
        .unwrap();
    assert_eq!(desc.pass_samples().unwrap(), 4);
    // Mixed attachment sample counts are not a supported render-pass contract.
    desc.depth = Some((
        color(vk::Format::D32_SFLOAT).sample_count(2),
        DepthAccess::ReadWrite,
    ));
    assert!(desc.pass_samples().is_err());
    desc.depth.as_mut().unwrap().0.sample_count = 4;
    assert_eq!(desc.pass_samples().unwrap(), 4);
}

#[test]
fn flat_conversion_splits_depth_and_matches_attachment_key_semantics() {
    let attachments = [
        color(vk::Format::R8G8B8A8_UNORM).clear_input(),
        color(vk::Format::D32_SFLOAT),
    ];
    let desc = RenderPassDesc::from_flat(&attachments).unwrap();
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
fn builtin_sampler_names() {
    assert_eq!(BUILTIN_SAMPLER_NAMES.len(), 9);
    assert!(BUILTIN_SAMPLER_NAMES.contains(&"sampler_shadow_linear"));
}
