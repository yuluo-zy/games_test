//! rg 缺陷修复的 CPU 回归：导出但未访问的 Create 资源、失败路径的资源归还、temporal 记录的放弃/确认、
//! pass 数上限的构图期错误。不需要 Vulkan 设备。
use ash::vk;
use rhapsody::raw::{
    managed::lifecycle::{self, ResourceHandle, SequenceRuntime},
    submit,
};
use rhapsody::rg::{
    executor::{Error, ExecutionBackend, ExecutionChain},
    graph::{
        self, AccessDependency, BackendFailure, BufferDesc, BuildError, Descriptor, EventBackend,
        OriginSource, Pool, RenderGraph, ResourceBackend, TextureDesc,
    },
    resource::ResourceOrigin,
    state::RenderGraphState,
    temporal::{
        TEMPORAL_EXPORT_ACCESS, TemporalError, TemporalRecordState, TemporalRenderGraphState,
    },
};

const WRITE: u8 = 0x34;
/// 测试用访问掩码：缓冲 → STORAGE_BUFFER(0x20)。
struct Dependency;
impl AccessDependency for Dependency {
    fn access_mask(&mut self, _code: u8) -> u32 {
        0x20
    }
    fn is_write(&mut self, code: u8) -> bool {
        code == WRITE
    }
}

fn buffer() -> Descriptor {
    Descriptor::Buffer(BufferDesc {
        word_at_00: 64,
        word_at_08: 0,
        bytes_at_0c_through_0e: [0; 3],
    })
}
fn pooled_buffer(handle: u64) -> graph::Buffer<u64> {
    let Descriptor::Buffer(desc) = buffer().with_usage(0x20) else {
        unreachable!()
    };
    graph::Buffer { handle, desc }
}
fn create(graph: &mut RenderGraph, name: &str) -> rhapsody::rg::resource::ResourceHandle {
    graph
        .create_raw_resource(
            name,
            ResourceOrigin {
                descriptor: buffer(),
                source: OriginSource::Create,
            },
        )
        .unwrap()
}

/// CPU 后端：记录创建、释放与放弃的命令；可让第 N 次创建、prepare 或回调失败。
#[derive(Default)]
struct Mock {
    next_token: u64,
    creations: usize,
    fail_creation_at: Option<usize>,
    creation_error: Option<rhapsody::Error>,
    fail_prepare: bool,
    released: Vec<u64>,
    abandoned: Vec<u64>,
    next_command: u64,
    next_submit: u64,
}
impl ResourceBackend for Mock {
    fn create_without_initial_data(
        &mut self,
        _descriptor: Descriptor,
        name: &str,
    ) -> Result<u64, BackendFailure> {
        let attempt = self.creations;
        self.creations += 1;
        if self.fail_creation_at == Some(attempt) {
            return Err(self
                .creation_error
                .take()
                .unwrap_or_else(|| BackendFailure::invalid_usage(format!("fail {name}"))));
        }
        self.next_token += 1;
        Ok(500 + self.next_token)
    }
    fn set_debug_name(&mut self, _token: u64, _descriptor: Descriptor, _name: &str) {}
    fn release_created_resource(&mut self, token: u64) {
        self.released.push(token);
    }
}
impl ExecutionBackend for Mock {
    fn prepare_compiled_resources(&mut self, _compiled: &graph::CompiledGraph) -> bool {
        !self.fail_prepare
    }
    fn managed_resource(
        &mut self,
        token: u64,
        lifecycle: &mut SequenceRuntime,
    ) -> Option<ResourceHandle> {
        Some(lifecycle.create_resource(token, 1))
    }
    fn command_handle(&mut self) -> Result<u64, lifecycle::Failure> {
        self.next_command += 1;
        Ok(1000 + self.next_command)
    }
    fn begin_command(&mut self, _handle: u64) -> Result<(), lifecycle::Failure> {
        Ok(())
    }
    fn end_command(&mut self, _handle: vk::CommandBuffer) -> bool {
        true
    }
    fn submit(
        &mut self,
        _queue: vk::Queue,
        _prepared: &[submit::PreparedSubmit],
    ) -> rhapsody::Result<rhapsody::sync::SyncPoint> {
        self.next_submit += 1;
        Ok(rhapsody::sync::SyncPoint {
            queue: rhapsody::sync::QueueId::new(0).unwrap(),
            value: self.next_submit,
        })
    }
    fn abandon_command(&mut self, handle: u64) {
        self.abandoned.push(handle);
    }
}

fn chain() -> ExecutionChain {
    ExecutionChain::new(vk::Queue::null(), vec![], 1)
}

/// 三个缓冲在同一 pass 中写入：r0 命中资源池，r1 新建，r2 第二次创建（可配置失败）。
fn three_buffer_graph(callback_ok: bool) -> graph::GraphDeclaration {
    let mut graph = RenderGraph::new();
    let handles = ["r0", "r1", "r2"].map(|name| create(&mut graph, name));
    let mut pass = graph.add_pass("write");
    for handle in handles {
        pass.add_resource_access(handle, WRITE, 0).unwrap();
    }
    pass.render(move |_| callback_ok);
    graph.build().into_parts().0
}

fn run(
    chain: &mut ExecutionChain,
    declaration: graph::GraphDeclaration,
    pool: &mut Pool,
    backend: &mut Mock,
    ok: bool,
) -> Result<(), Error> {
    let mut callbacks = (0..declaration.passes.len())
        .map(|_| move |_: u64| ok)
        .collect::<Vec<_>>();
    chain
        .execute_graph(declaration, pool, &mut Dependency, backend, &mut callbacks)
        .map(|_| ())
}

// ---------------------------------------------------------------- 1. 导出但从未访问的 Create 资源

#[test]
fn an_exported_but_unaccessed_create_resource_compiles_without_panicking() {
    let mut graph = RenderGraph::new();
    let touched = create(&mut graph, "touched");
    let exported = create(&mut graph, "export only");
    let mut pass = graph.add_pass("touch");
    pass.add_resource_access(touched, WRITE, 0).unwrap();
    pass.render(|_| true);
    graph.export(exported, 0x26).unwrap();
    let declaration = graph.build().into_parts().0;
    let compiled = graph::compile_graph(
        declaration,
        &mut Pool::default(),
        &mut EventBackend::default(),
        &mut Dependency,
    )
    .expect("导出未访问资源不应让编译 panic");
    let mapping = &compiled.snapshot.physical_mapping[exported.raw() as usize];
    assert_eq!(
        (mapping.physical, mapping.token),
        (u16::MAX, None),
        "未填充的资源保持无物理映射"
    );
    assert!(
        compiled
            .snapshot
            .export
            .iter()
            .all(|t| t.resource != exported.raw()),
        "未填充资源不产生导出屏障"
    );
    let exports = compiled.into_exports();
    assert!(
        !exports.exported.contains_key(&(exported.raw() as u16)),
        "未填充资源不能作为导出取出"
    );
}

#[test]
fn executing_an_exported_but_unaccessed_create_resource_reports_resource_unavailable() {
    let mut graph = RenderGraph::new();
    let touched = create(&mut graph, "touched");
    let exported = create(&mut graph, "export only");
    let mut pass = graph.add_pass("touch");
    pass.add_resource_access(touched, WRITE, 0).unwrap();
    pass.render(|_| true);
    graph.export(exported, 0x26).unwrap();
    let declaration = graph.build().into_parts().0;
    let mut backend = Mock::default();
    let mut pool = Pool::default();
    let error = run(&mut chain(), declaration, &mut pool, &mut backend, true).unwrap_err();
    assert!(
        matches!(error, Error::ResourceUnavailable(id) if id == exported.raw() as usize),
        "{error:?}"
    );
    // 被访问资源已创建，失败后交还后端而不是悄悄泄漏。
    assert_eq!(backend.released, vec![501]);
}

// ---------------------------------------------------------------- 2. 失败路径归还资源

#[test]
fn a_population_failure_returns_pool_resources_and_releases_created_tokens() {
    let mut pool = Pool::default();
    pool.insert_buffer(pooled_buffer(900));
    let mut backend = Mock {
        fail_creation_at: Some(1),
        ..Default::default()
    };
    let error = run(
        &mut chain(),
        three_buffer_graph(true),
        &mut pool,
        &mut backend,
        true,
    )
    .unwrap_err();
    assert!(matches!(error, Error::Compile(_)), "{error:?}");
    let Descriptor::Buffer(desc) = buffer().with_usage(0x20) else {
        unreachable!()
    };
    assert_eq!(
        pool.take_buffer(&desc).map(|b| b.handle),
        Some(900),
        "资源池命中必须归还"
    );
    assert_eq!(
        backend.released,
        vec![501],
        "失败前已创建的 token 必须交还后端"
    );
}

#[test]
fn a_failure_after_compilation_returns_pool_resources_and_releases_created_tokens() {
    let mut pool = Pool::default();
    pool.insert_buffer(pooled_buffer(900));
    let mut backend = Mock {
        fail_prepare: true,
        ..Default::default()
    };
    let error = run(
        &mut chain(),
        three_buffer_graph(true),
        &mut pool,
        &mut backend,
        true,
    )
    .unwrap_err();
    assert!(
        matches!(error, Error::ResourceUnavailable(usize::MAX)),
        "{error:?}"
    );
    let Descriptor::Buffer(desc) = buffer().with_usage(0x20) else {
        unreachable!()
    };
    assert_eq!(pool.take_buffer(&desc).map(|b| b.handle), Some(900));
    let mut released = backend.released.clone();
    released.sort_unstable();
    assert_eq!(released, vec![501, 502]);
    assert!(
        backend.abandoned.is_empty(),
        "命令开始前失败不应有命令需要放弃"
    );
}

#[test]
fn a_recording_failure_abandons_begun_commands_and_reclaims_resources() {
    let mut pool = Pool::default();
    pool.insert_buffer(pooled_buffer(900));
    let mut backend = Mock::default();
    let mut chain = chain();
    let error = run(
        &mut chain,
        three_buffer_graph(false),
        &mut pool,
        &mut backend,
        false,
    )
    .unwrap_err();
    assert!(matches!(error, Error::Record(_)), "{error:?}");
    let mut abandoned = backend.abandoned.clone();
    abandoned.sort_unstable();
    assert_eq!(abandoned, vec![1001], "已开始但未提交的命令全部放弃");
    let Descriptor::Buffer(desc) = buffer().with_usage(0x20) else {
        unreachable!()
    };
    assert_eq!(
        pool.take_buffer(&desc).map(|b| b.handle),
        Some(900),
        "未提交任何批次时资源池命中可安全归还"
    );
    assert_eq!(backend.released.len(), 2);
}

// ---------------------------------------------------------------- 5. temporal 记录放弃 / 确认

fn image() -> Descriptor {
    Descriptor::Image(TextureDesc {
        words_at_00_through_18: [1, 2, 3, 4, 5, 6, 7],
        bytes_at_1c_through_1e: [0; 3],
    })
}

#[test]
fn an_abandoned_temporal_request_is_released_at_the_next_frame_begin() {
    let mut state = TemporalRenderGraphState::default();
    let mut graph = RenderGraph::new();
    state
        .get_or_create_temporal(&mut graph, "hist", image(), || Ok(7))
        .unwrap();
    state.export_temporal(&mut graph).unwrap();
    // 下一帧请求后放弃构图（不导出、不编译）。
    let mut graph = RenderGraph::new();
    state
        .get_or_create_temporal(&mut graph, "hist", image(), || panic!("must reuse"))
        .unwrap();
    drop(graph);
    // 同一帧内重复请求仍按原合同报错。
    let mut same = RenderGraph::new();
    assert!(matches!(
        state.get_or_create_temporal(&mut same, "hist", image(), || Ok(8)),
        Err(TemporalError::AlreadyRequested(_))
    ));
    state.begin_frame();
    let mut graph = RenderGraph::new();
    let again = state
        .get_or_create_temporal(&mut graph, "hist", image(), || panic!("must reuse"))
        .unwrap();
    assert!(again.reused);
    assert_eq!(again.resource.token, 7);
}

#[test]
fn render_graph_state_begin_frame_releases_abandoned_requests() {
    let mut state = RenderGraphState::default();
    let mut graph = state.begin_frame();
    state
        .temporal
        .get_or_create_temporal(&mut graph, "hist", image(), || Ok(7))
        .unwrap();
    drop(graph);
    let mut graph = state.begin_frame();
    let lookup = state
        .temporal
        .get_or_create_temporal(&mut graph, "hist", image(), || panic!("must reuse"))
        .unwrap();
    assert!(lookup.reused);
    assert_eq!(
        lookup.resource.access, 0,
        "从未执行过的新资源首次使用仍是 undefined"
    );
}

#[test]
fn an_unexecuted_export_does_not_promote_a_new_resource_beyond_undefined() {
    let mut state = TemporalRenderGraphState::default();
    let mut graph = RenderGraph::new();
    state
        .get_or_create_temporal(&mut graph, "hist", image(), || Ok(7))
        .unwrap();
    state.export_temporal(&mut graph).unwrap();
    // 原 export_temporal 写入 0x37。
    assert_eq!(state.get("hist").unwrap().access, TEMPORAL_EXPORT_ACCESS);
    // 本帧没有真正执行：放弃后回到 GPU 已确认的访问（新资源 = undefined）。
    state.abandon_frame();
    assert_eq!(state.get("hist").unwrap().access, 0);
    assert_eq!(
        state.record_state("hist"),
        Some(TemporalRecordState::Exported)
    );
    // 执行确认后再放弃不回退。
    let mut graph = RenderGraph::new();
    state
        .get_or_create_temporal(&mut graph, "hist", image(), || panic!("must reuse"))
        .unwrap();
    state.export_temporal(&mut graph).unwrap();
    state.confirm_exports();
    state.abandon_frame();
    assert_eq!(state.get("hist").unwrap().access, TEMPORAL_EXPORT_ACCESS);
}

// ---------------------------------------------------------------- 7. pass 数上限

#[test]
fn exceeding_the_pass_packing_domain_is_a_build_error() {
    let mut graph = RenderGraph::new();
    let resource = create(&mut graph, "r");
    for _ in 0..32768 {
        graph.add_pass("p").render(|_| true);
    }
    assert!(matches!(
        graph.try_add_pass("overflow"),
        Err(BuildError::PassLimit)
    ));
    let mut pass = graph.add_pass("overflow");
    assert!(matches!(
        pass.add_resource_access(resource, WRITE, 0),
        Err(BuildError::PassLimit)
    ));
}

#[test]
fn malformed_declarations_fail_before_creating_resources() {
    use rhapsody::rg::graph::{AccessDeclaration, ExportDeclaration, PassDeclaration};
    let invalid_access = graph::GraphDeclaration {
        passes: vec![PassDeclaration {
            name: "invalid".into(),
            accesses: vec![AccessDeclaration {
                resource_index: u32::MAX,
                synchronization_group: 0,
                access_code: 0x34,
            }],
        }],
        ..Default::default()
    };
    let invalid_export = graph::GraphDeclaration {
        exports: vec![ExportDeclaration {
            tag: 0,
            resource_index: 0,
            access_code: 0x26,
        }],
        ..Default::default()
    };
    for declaration in [invalid_access, invalid_export] {
        let mut backend = Mock::default();
        let error = graph::compile_graph(
            declaration,
            &mut graph::Pool::default(),
            &mut backend,
            &mut rhapsody::rg::vulkan::AccessTable::original(),
        )
        .err()
        .unwrap();
        assert_eq!(error.kind(), rhapsody::ErrorKind::InvalidUsage);
        assert_eq!(backend.creations, 0);
    }
}

#[test]
fn resource_creation_preserves_device_error_categories() {
    for injected in [
        rhapsody::Error::DeviceLost,
        rhapsody::Error::OutOfHostMemory,
        rhapsody::Error::OutOfDeviceMemory,
    ] {
        let expected = injected.kind();
        let mut backend = Mock {
            fail_creation_at: Some(1),
            creation_error: Some(injected),
            ..Default::default()
        };
        let error = run(
            &mut chain(),
            three_buffer_graph(true),
            &mut Pool::default(),
            &mut backend,
            true,
        )
        .unwrap_err();
        assert_eq!(rhapsody::Error::from(error).kind(), expected);
        assert_eq!(
            backend.released,
            [501],
            "部分创建的资源在返回设备错误前仍须归还"
        );
    }
}
