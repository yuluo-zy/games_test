//! 构图绑定登记访问，并由原账本规则合并；不能保留任意不兼容组合。
use rhapsody::rg::{graph::{Descriptor, OriginSource, RenderGraph}, pass_bindings::PassBindingError, resource::ResourceOrigin};
use rhapsody::rg::resource_registry::{BufferDesc, TextureDesc};
#[test]
fn typed_refs_register_access_and_preserve_original_upgrade_records() {
    let mut graph = RenderGraph::new();
    let buffer = graph
        .create_raw_resource(
            "buffer",
            ResourceOrigin {
                descriptor: Descriptor::Buffer(BufferDesc {
                    word_at_00: 16,
                    word_at_08: 16,
                    bytes_at_0c_through_0e: [0; 3],
                }),
                source: OriginSource::Create,
            },
        )
        .unwrap();
    let mut pass = graph.add_pass("读取").with_side_effect();
    let a = pass
        .buffer_ref(buffer, 0x1a, 3)
        .unwrap()
        .bind_uniform_buffer(0, 16)
        .unwrap();
    let b = pass
        .buffer_ref(buffer, 0x37, 4)
        .unwrap()
        .bind_storage_buffer(0, 16)
        .unwrap();
    assert_eq!(a.resource(), buffer);
    assert_eq!(b.resource(), buffer);
    pass.render(|_| true);
    let built = graph.build();
    assert_eq!(built.declaration.passes[0].accesses.len(), 2);
    assert_eq!(built.declaration.passes[0].accesses[0].access_code, 0x37);
    assert_eq!(
        built.declaration.passes[0].accesses[1].synchronization_group,
        0
    );
}
#[test]
fn typed_ref_rejects_wrong_kind_before_registration() {
    let mut graph = RenderGraph::new();
    let texture = graph
        .create_raw_resource(
            "texture",
            ResourceOrigin {
                descriptor: Descriptor::Image(TextureDesc {
                    words_at_00_through_18: [1; 7],
                    bytes_at_1c_through_1e: [0; 3],
                }),
                source: OriginSource::Create,
            },
        )
        .unwrap();
    let mut pass = graph.add_pass("类型检查").with_side_effect();
    assert!(matches!(
        pass.buffer_ref(texture, 4, 0),
        Err(PassBindingError::WrongResourceKind)
    ));
    pass.render(|_| true);
    assert!(graph.build().declaration.passes[0].accesses.is_empty());
}
