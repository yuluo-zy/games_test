//! 检查会影响缓存正确性的兼容规则与原当前 pass 查找语义。
use ash::vk;
use rhapsody::{
    raw::render_pass::{AttachmentKey, RenderPassKey},
    rg::{
        attachments::{DepthAccess, RenderPassAttachmentDesc},
        graph::{AccessDeclaration, PassDeclaration},
        pass_api::{Command, CommandBackend, RenderPassApi},
        resource::ResourceHandle,
    },
};
struct Noop;
impl CommandBackend for Noop {
    fn dispatch(&mut self, _: vk::CommandBuffer, _: Command<'_>) {}
}
fn key() -> RenderPassKey {
    RenderPassKey {
        attachments: vec![AttachmentKey {
            format: vk::Format::R8G8B8A8_UNORM.as_raw(),
            samples: 1,
            load: 0,
            store: 0,
            stencil_load: 2,
            stencil_store: 1,
            initial_layout: 2,
            final_layout: 2,
            subpass_layout: 2,
            depth_stencil: false,
        }],
    }
}
#[test]
fn exact_cache_key_differs_while_pipeline_compatibility_stays() {
    let a = key();
    let mut b = a.clone();
    b.attachments[0].load = 1;
    b.attachments[0].final_layout = 6;
    assert_ne!(a, b);
    assert_eq!(a.compatibility_key(), b.compatibility_key());
    b.attachments[0].samples = 4;
    assert_ne!(a.compatibility_key(), b.compatibility_key());
}
#[test]
fn original_attachment_defaults_and_modifiers() {
    let a = RenderPassAttachmentDesc::new(vk::Format::R8G8B8A8_UNORM);
    assert_eq!(a.load, vk::AttachmentLoadOp::LOAD);
    assert_eq!(a.store, vk::AttachmentStoreOp::STORE);
    assert_eq!(a.samples, vk::SampleCountFlags::TYPE_1);
    let b = a.clear_input().sample_count(vk::SampleCountFlags::TYPE_4);
    assert_eq!(b.load, vk::AttachmentLoadOp::CLEAR);
    assert_eq!(b.samples, vk::SampleCountFlags::TYPE_4);
    assert_eq!(b.store, a.store);
    assert_eq!(DepthAccess::ReadOnly.layout().as_raw(), 4);
    assert_eq!(DepthAccess::ReadWrite.layout().as_raw(), 3);
    assert_eq!(
        DepthAccess::DepthWriteStencilRead.layout().as_raw(),
        1000117001
    );
}
#[test]
fn access_lookup_uses_current_pass_first_match() {
    let pass = PassDeclaration {
        name: "当前".into(),
        accesses: vec![
            AccessDeclaration {
                resource_index: 3,
                synchronization_group: 0,
                access_code: 4,
            },
            AccessDeclaration {
                resource_index: 3,
                synchronization_group: 1,
                access_code: 40,
            },
        ],
    };
    assert_eq!(
        RenderPassApi::<Noop>::find_resource_access_type(&pass, ResourceHandle::from_raw(3)),
        4
    );
    assert!(
        std::panic::catch_unwind(|| RenderPassApi::<Noop>::find_resource_access_type(
            &pass,
            ResourceHandle::from_raw(7)
        ))
        .is_err()
    );
}
