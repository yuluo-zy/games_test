//! 按 PassBuilder::add_resource_access / render 重建构图入口。
//! 泛型参数、原作闭包布局及错误类型尚未恢复；此处提供可执行的候选接口。
use super::graph::{BuildError, PassDeclaration, RenderCallback, RenderGraph};
use super::resource::ResourceHandle;

pub struct PassBuilder<'graph> {
    pub(crate) graph: &'graph mut RenderGraph,
    pub(crate) declaration: PassDeclaration,
}
#[derive(Clone, Copy)]
enum AccessMode {
    Read,
    Write,
    Discard,
}
impl PassBuilder<'_> {
    pub fn on_queue(&mut self, class: crate::device::QueueClass) -> &mut Self {
        self.declaration.queue_class = class;
        self
    }
    pub fn with_queue(mut self, class: crate::device::QueueClass) -> Self {
        self.on_queue(class);
        self
    }

    /// Resolve legacy indices explicitly at the active graph boundary.
    pub fn resource_handle(&self, index: u32) -> Result<ResourceHandle, BuildError> {
        self.graph.resource_handle(index)
    }
    pub fn side_effect(&mut self) -> &mut Self {
        self.declaration.side_effect = true;
        self
    }
    pub fn with_side_effect(mut self) -> Self {
        self.side_effect();
        self
    }
    /// 保留声明顺序与重复访问，交给编译模块处理访问链。
    pub fn add_resource_access(
        &mut self,
        resource: ResourceHandle,
        access_code: u8,
        synchronization_group: u32,
    ) -> Result<&mut Self, BuildError> {
        // 本 pass 尚未入图：图里已满 MAX_PASSES 个时它不可能被编译。
        self.graph.check_pass_capacity()?;
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
    /// Declare a bounded access through the same production ledger as raw calls.
    pub fn add_resource_access_range(
        &mut self,
        resource: ResourceHandle,
        access_code: u8,
        synchronization_group: u32,
        range: super::resource::AccessRange,
    ) -> Result<&mut Self, BuildError> {
        self.add_versioned_access(
            resource,
            access_code,
            synchronization_group,
            range,
            super::resource::AccessVersion::Automatic,
            // Legacy resource/ref bindings retain original synchronization
            // codes. Content-preserving typed writes and explicit read→write
            // upgrades declare their previous-content dependency separately.
            false,
        )?;
        Ok(self)
    }

    /// Source-host adapter for a confirmed content-preserving operation, such
    /// as an attachment LOAD. The original synchronization code alone cannot
    /// express that extra content contract; the actual consumer supplies it.
    pub fn add_preserving_resource_access_range(
        &mut self,
        resource: ResourceHandle,
        access_code: u8,
        synchronization_group: u32,
        range: super::resource::AccessRange,
    ) -> Result<&mut Self, BuildError> {
        self.add_versioned_access(
            resource,
            access_code,
            synchronization_group,
            range,
            super::resource::AccessVersion::Automatic,
            true,
        )?;
        Ok(self)
    }

    /// Keep the real binding/synchronization access for a shader read whose
    /// uniformly disabled branch cannot read resource contents in this pass.
    /// This neither initializes the resource nor suppresses later active reads.
    pub fn add_inactive_resource_read_range(
        &mut self,
        resource: ResourceHandle,
        access_code: u8,
        synchronization_group: u32,
        range: super::resource::AccessRange,
    ) -> Result<&mut Self, BuildError> {
        self.graph.check_pass_capacity()?;
        self.graph.check_handle(resource)?;
        if usize::from(access_code) >= crate::raw::access_table::ACCESS_CODE_COUNT {
            return Err(BuildError::Contract(
                super::resource::ResourceContractError::InvalidAccessCode(access_code),
            ));
        }
        if crate::raw::access_table::is_write(access_code) {
            return Err(BuildError::Contract(
                super::resource::ResourceContractError::WrongAccessMode {
                    resource: resource.raw(),
                    access_code,
                },
            ));
        }
        range
            .validate(self.graph.declaration.resources[resource.raw() as usize].descriptor)
            .map_err(|error| super::resource_contracts::range_error(resource.raw(), error))?;
        super::access_ledger::collect_resource_access_range_with_contents(
            &mut self.declaration.accesses,
            resource.raw(),
            access_code,
            synchronization_group,
            range,
            super::resource::AccessVersion::Automatic,
            false,
            true,
        )
        .map_err(BuildError::IncompatibleAccess)?;
        Ok(self)
    }

    /// Preserve a real shader content read even when its original caller
    /// explicitly permits undefined initial texels. Ordinary reads and exports
    /// stay strict; existing producers still remain data dependencies.
    pub fn add_undefined_content_read_range(
        &mut self, resource: ResourceHandle, access_code: u8,
        synchronization_group: u32, range: super::resource::AccessRange,
    ) -> Result<&mut Self, BuildError> {
        self.graph.check_pass_capacity()?;
        self.graph.check_handle(resource)?;
        if usize::from(access_code) >= crate::raw::access_table::ACCESS_CODE_COUNT {
            return Err(BuildError::Contract(super::resource::ResourceContractError::InvalidAccessCode(access_code)));
        }
        if crate::raw::access_table::is_write(access_code) {
            return Err(BuildError::Contract(super::resource::ResourceContractError::WrongAccessMode {
                resource: resource.raw(), access_code,
            }));
        }
        range.validate(self.graph.declaration.resources[resource.raw() as usize].descriptor)
            .map_err(|error| super::resource_contracts::range_error(resource.raw(), error))?;
        super::access_ledger::collect_resource_access_range_with_policy(
            &mut self.declaration.accesses, resource.raw(), access_code, synchronization_group,
            range, super::resource::AccessVersion::Automatic, false, false, true,
        ).map_err(BuildError::IncompatibleAccess)?;
        Ok(self)
    }

    fn add_versioned_access(
        &mut self,
        resource: ResourceHandle,
        access_code: u8,
        synchronization_group: u32,
        range: super::resource::AccessRange,
        version: super::resource::AccessVersion,
        reads_previous: bool,
    ) -> Result<(), BuildError> {
        self.graph.check_pass_capacity()?;
        self.graph.check_handle(resource)?;
        range
            .validate(self.graph.declaration.resources[resource.raw() as usize].descriptor)
            .map_err(|error| super::resource_contracts::range_error(resource.raw(), error))?;
        super::access_ledger::collect_resource_access_range(
            &mut self.declaration.accesses,
            resource.raw(),
            access_code,
            synchronization_group,
            range,
            version,
            reads_previous,
        )
        .map_err(BuildError::IncompatibleAccess)?;
        Ok(())
    }

    fn latest_version(&self, resource: ResourceHandle) -> Result<u32, BuildError> {
        let mut version = self.graph.current_version(resource)?;
        for access in self
            .declaration
            .accesses
            .iter()
            .filter(|access| access.resource_index == resource.raw())
        {
            match access.version {
                super::resource::AccessVersion::Write { current, .. } => version = current,
                super::resource::AccessVersion::Automatic
                    if crate::raw::access_table::is_write(access.access_code) =>
                {
                    version = version.checked_add(1).ok_or(BuildError::Contract(
                        super::resource::ResourceContractError::VersionOverflow(resource.raw()),
                    ))?;
                }
                _ => {}
            }
        }
        Ok(version)
    }

    fn typed_access(
        &mut self,
        resource: ResourceHandle,
        version: u32,
        access_code: u8,
        range: super::resource::AccessRange,
        mode: AccessMode,
        texture: bool,
    ) -> Result<u32, BuildError> {
        let write = !matches!(mode, AccessMode::Read);
        self.graph.check_handle(resource)?;
        let descriptor = self.graph.declaration.resources[resource.raw() as usize].descriptor;
        if matches!(descriptor, super::graph::Descriptor::Image(_)) != texture {
            return Err(BuildError::ResourceKindMismatch(resource.raw()));
        }
        if usize::from(access_code) >= crate::raw::access_table::ACCESS_CODE_COUNT {
            return Err(BuildError::Contract(
                super::resource::ResourceContractError::InvalidAccessCode(access_code),
            ));
        }
        if crate::raw::access_table::is_write(access_code) != write {
            return Err(BuildError::Contract(
                super::resource::ResourceContractError::WrongAccessMode {
                    resource: resource.raw(),
                    access_code,
                },
            ));
        }
        let expected = self.latest_version(resource)?;
        if expected != version {
            return Err(BuildError::Contract(
                super::resource::ResourceContractError::StaleVersion {
                    resource: resource.raw(),
                    expected,
                    actual: version,
                },
            ));
        }
        let next = if write {
            version.checked_add(1).ok_or(BuildError::Contract(
                super::resource::ResourceContractError::VersionOverflow(resource.raw()),
            ))?
        } else {
            version
        };
        self.add_versioned_access(
            resource,
            access_code,
            0,
            range,
            if write {
                super::resource::AccessVersion::Write {
                    previous: version,
                    current: next,
                }
            } else {
                super::resource::AccessVersion::Read(version)
            },
            !matches!(mode, AccessMode::Discard) && matches!(access_code, 0x36 | 0x37),
        )?;
        Ok(next)
    }

    pub fn read_texture(
        &mut self,
        texture: super::resource::TextureId,
        access_code: u8,
        range: super::resource::AccessRange,
    ) -> Result<&mut Self, BuildError> {
        self.typed_access(
            texture.handle,
            texture.version,
            access_code,
            range,
            AccessMode::Read,
            true,
        )?;
        Ok(self)
    }
    pub fn write_texture(
        &mut self,
        texture: super::resource::TextureId,
        access_code: u8,
        range: super::resource::AccessRange,
    ) -> Result<super::resource::TextureId, BuildError> {
        let version = self.typed_access(
            texture.handle,
            texture.version,
            access_code,
            range,
            AccessMode::Write,
            true,
        )?;
        Ok(super::resource::TextureId { version, ..texture })
    }
    pub fn read_buffer(
        &mut self,
        buffer: super::resource::BufferId,
        access_code: u8,
        range: super::resource::AccessRange,
    ) -> Result<&mut Self, BuildError> {
        self.typed_access(
            buffer.handle,
            buffer.version,
            access_code,
            range,
            AccessMode::Read,
            false,
        )?;
        Ok(self)
    }
    pub fn write_buffer(
        &mut self,
        buffer: super::resource::BufferId,
        access_code: u8,
        range: super::resource::AccessRange,
    ) -> Result<super::resource::BufferId, BuildError> {
        let version = self.typed_access(
            buffer.handle,
            buffer.version,
            access_code,
            range,
            AccessMode::Write,
            false,
        )?;
        Ok(super::resource::BufferId { version, ..buffer })
    }
    /// Explicitly discard previous contents, even for a read/write Vulkan access mask.
    pub fn write_texture_discard(
        &mut self,
        texture: super::resource::TextureId,
        access_code: u8,
        range: super::resource::AccessRange,
    ) -> Result<super::resource::TextureId, BuildError> {
        let version = self.typed_access(
            texture.handle,
            texture.version,
            access_code,
            range,
            AccessMode::Discard,
            true,
        )?;
        Ok(super::resource::TextureId { version, ..texture })
    }
    /// Explicitly discard previous contents, even for a read/write Vulkan access mask.
    pub fn write_buffer_discard(
        &mut self,
        buffer: super::resource::BufferId,
        access_code: u8,
        range: super::resource::AccessRange,
    ) -> Result<super::resource::BufferId, BuildError> {
        let version = self.typed_access(
            buffer.handle,
            buffer.version,
            access_code,
            range,
            AccessMode::Discard,
            false,
        )?;
        Ok(super::resource::BufferId { version, ..buffer })
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
    pub fn render_with_result_context(
        self,
        callback: impl FnOnce(super::executor::PassContext<'_>) -> crate::Result<()> + Send + 'static,
    ) {
        self.render_with_context(move |context| {
            let errors = context.errors;
            match callback(context) {
                Ok(()) => true,
                Err(error) => {
                    errors.borrow_mut().get_or_insert(error);
                    false
                }
            }
        });
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
}
