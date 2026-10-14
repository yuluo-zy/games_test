//! 按 PassBuilder::add_resource_access / render 重建构图入口。
//! 泛型参数、原作闭包布局及错误类型尚未恢复；此处提供可执行的候选接口。
use super::graph::{BuildError, PassDeclaration, RenderCallback, RenderGraph};
use super::resource::ResourceHandle;

pub struct PassBuilder<'graph> {
    pub(crate) graph: &'graph mut RenderGraph,
    pub(crate) declaration: PassDeclaration,
}
impl PassBuilder<'_> {
    /// 保留声明顺序与重复访问，交给编译模块处理访问链。
    pub fn add_resource_access(
        &mut self,
        resource: ResourceHandle,
        access_code: u8,
        synchronization_group: u32,
    ) -> Result<&mut Self, BuildError> {
        self.graph.check_handle(resource)?;
        super::access_ledger::collect_resource_access(
            &mut self.declaration.accesses,
            resource.raw(),
            access_code,
            synchronization_group,
        )
        .map_err(BuildError::IncompatibleAccess)?;
        Ok(self)
    }
    /// 完成一个 pass 才提交声明；丢弃 builder 不产生半成品 pass。
    pub fn render(self, callback: impl FnOnce(u64) -> bool + Send + 'static) {
        self.render_with_context(move |context| callback(context.command.handle()));
    }
    /// 原 FnOnce 的消费顺序不变；上下文来自当前 recorder，不能由外部 counter 仿造。
    pub fn render_with_context(
        self,
        callback: impl FnOnce(super::executor::PassContext<'_>) -> bool + Send + 'static,
    ) {
        let mut callback = Some(callback);
        let callback: RenderCallback = Box::new(move |context| {
            // 首次执行前取出闭包，避免错误重试再次消费原闭包。
            callback.take().is_some_and(|render| render(context))
        });
        self.graph.declaration.passes.push(self.declaration);
        self.graph.callbacks.push(callback);
    }
}

impl<'graph> PassBuilder<'graph> {
    /// 消费 builder 返回所属图的可变引用；用于在 pass 声明之间继续构图。
    /// 原 rg_mut 用于内部访问；候选接口避免在此保留 &mut 借用与被 self 借用的冲突。
    pub fn rg_mut(self) -> &'graph mut RenderGraph {
        self.graph
    }
    /// 写访问声明；读→写升级、同组去重由共享账本保留原两条 0x37/group0 记录规则。
    pub fn write_impl(
        &mut self,
        resource: ResourceHandle,
        access_code: u8,
        synchronization_group: u32,
    ) -> Result<&mut Self, BuildError> {
        self.add_resource_access(resource, access_code, synchronization_group)
    }
    pub fn buffer_ref<'borrow>(
        &'borrow mut self,
        resource: ResourceHandle,
        access_code: u8,
        group: u32,
    ) -> Result<
        super::resource::Ref<'borrow, 'graph, super::resource::BufferRef>,
        super::pass_bindings::PassBindingError,
    > {
        self.graph.check_handle(resource)?;
        if !matches!(
            self.graph.declaration.resources[resource.raw() as usize].descriptor,
            super::graph::Descriptor::Buffer(_)
        ) {
            return Err(super::pass_bindings::PassBindingError::WrongResourceKind);
        }
        Ok(super::resource::Ref {
            builder: self,
            resource,
            access_code,
            group,
            kind: std::marker::PhantomData,
        })
    }
    pub fn texture_ref<'borrow>(
        &'borrow mut self,
        resource: ResourceHandle,
        access_code: u8,
        group: u32,
    ) -> Result<
        super::resource::Ref<'borrow, 'graph, super::resource::TextureRef>,
        super::pass_bindings::PassBindingError,
    > {
        self.graph.check_handle(resource)?;
        if !matches!(
            self.graph.declaration.resources[resource.raw() as usize].descriptor,
            super::graph::Descriptor::Image(_)
        ) {
            return Err(super::pass_bindings::PassBindingError::WrongResourceKind);
        }
        Ok(super::resource::Ref {
            builder: self,
            resource,
            access_code,
            group,
            kind: std::marker::PhantomData,
        })
    }
    /// 把本 pass 的访问声明和绑定计划一起捕获，callback 不需要再手填资源访问表。
    pub fn render_bound(
        self,
        bindings: std::collections::BTreeMap<String, Vec<super::pass_bindings::PassBinding>>,
        callback: impl FnOnce(u64, &super::pass_bindings::BoundPassBindings) -> bool + Send + 'static,
    ) {
        let plan = super::pass_bindings::BoundPassBindings {
            pass: self.declaration.clone(),
            bindings,
        };
        self.render(move |command| callback(command, &plan));
    }
    pub fn render_bound_with_context(
        self,
        bindings: std::collections::BTreeMap<String, Vec<super::pass_bindings::PassBinding>>,
        callback: impl FnOnce(
            super::executor::PassContext<'_>,
            &super::pass_bindings::BoundPassBindings,
        ) -> bool
        + Send
        + 'static,
    ) {
        let plan = super::pass_bindings::BoundPassBindings {
            pass: self.declaration.clone(),
            bindings,
        };
        self.render_with_context(move |context| callback(context, &plan));
    }
}
