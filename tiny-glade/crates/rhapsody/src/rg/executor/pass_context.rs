//! Per-pass recording context and coverage.
use ash::vk;
use crate::command::CpuArenaLease;
use crate::command::recording;

/// One pass recorder. Closures are recorders, so every existing callback keeps
/// working; a retained graph supplies long-lived recorders that read per-frame
/// parameters instead of capturing them. Recording may run on worker threads.
pub trait PassRecord: Send {
    fn record(&mut self, context: PassContext<'_>) -> bool;
}

impl<F: FnMut(PassContext<'_>) -> bool + Send> PassRecord for F {
    fn record(&mut self, context: PassContext<'_>) -> bool {
        self(context)
    }
}

/// The executor picks the retained passes out of the caller's slice as
/// `&mut dyn PassRecord`; that reference is itself a recorder.
impl PassRecord for &mut dyn PassRecord {
    fn record(&mut self, context: PassContext<'_>) -> bool {
        (**self).record(context)
    }
}

/// 本次录制的命令和编译图；绑定消费者不能另外分配序号或重新构造资源映射。
pub struct PassContext<'a> {
    /// Callback-local Copy accumulator. Fold one completed API object into it;
    /// the executor publishes it only after this callback succeeds.
    #[cfg(feature = "cpu-probe")]
    pub command_workload: &'a std::cell::Cell<crate::profile::CommandWorkload>,
    pub pass_index: usize,
    /// CPU-only scratch, valid through this callback; never GPU upload backing.
    pub cpu: Option<&'a CpuArenaLease>,
    pub command: &'a recording::Command,
    pub compiled: &'a crate::rg::graph::FrameInstance,
    pub managed: Option<&'a crate::rg::pass_bindings::ManagedPassResources>,
    /// 由 recorder 收集并交给同一命令拥有者，callback 返回后仍需保活。
    pub owners: &'a std::cell::RefCell<super::super::owner_storage::ResourceOwners>,
    pub errors: &'a std::cell::RefCell<Option<crate::Error>>,
    /// Shader-visible declared accesses of this pass indexed by physical
    /// resource, built on the first bindless lease check of the callback.
    pub coverage: PassCoverage,
}

/// Lazily built per-callback index used to validate bindless leases against the
/// pass's declared accesses without rescanning every declaration per lease.
#[derive(Default)]
pub struct PassCoverage {
    entries: std::cell::OnceCell<Vec<CoverageEntry>>,
}

#[derive(Clone, Copy)]
pub(crate) struct CoverageEntry {
    /// Address of the physical `Buffer`/`Texture`, stable while the pass records.
    pub(crate) resource: usize,
    /// `None` for buffers; the declared native layout for images.
    pub(crate) layout: Option<vk::ImageLayout>,
    pub(crate) range: crate::rg::resource::AccessRange,
}

impl PassCoverage {
    pub(crate) fn entries(&self, build: impl FnOnce() -> Vec<CoverageEntry>) -> &[CoverageEntry] {
        self.entries.get_or_init(build)
    }
}

impl PassContext<'_> {
    #[cfg(feature = "cpu-probe")]
    pub fn record_command_workload(&self, workload: crate::profile::CommandWorkload) {
        let mut local = self.command_workload.get();
        local.merge(workload);
        self.command_workload.set(local);
    }

    /// Native layout of this exact image range in the current scheduled pass.
    /// Queries validated declarations only; it grants no additional access.
    /// A descriptor spanning missing uses or different layouts is rejected.
    pub fn image_layout_for_range(
        &self,
        resource: u32,
        range: crate::rg::AccessRange,
    ) -> crate::Result<vk::ImageLayout> {
        image_layout_from_plan(&self.compiled.plan, self.pass_index, resource, range)
    }

    pub fn history_valid(&self, texture: crate::rg::resource::TextureId) -> crate::Result<bool> {
        self.named_history_valid(texture.raw(), true)
    }
    pub fn buffer_history_valid(
        &self,
        buffer: crate::rg::resource::BufferId,
    ) -> crate::Result<bool> {
        self.named_history_valid(buffer.raw(), false)
    }
    fn named_history_valid(
        &self,
        handle: crate::rg::resource::ResourceHandle,
        image: bool,
    ) -> crate::Result<bool> {
        let graph = &self.compiled.declarations;
        if graph
            .passes
            .get(self.pass_index)
            .and_then(|pass| pass.graph_id)
            != Some(handle.graph_id())
        {
            return Err(crate::Error::invalid_usage(
                "history handle belongs to another graph",
            ));
        }
        let index = handle.raw() as usize;
        let declaration = graph
            .resources
            .get(index)
            .ok_or_else(|| crate::Error::invalid_usage("history handle is out of range"))?;
        if matches!(
            declaration.descriptor,
            crate::rg::graph::Descriptor::Image(_)
        ) != image
        {
            return Err(crate::Error::invalid_usage(
                "history handle has the wrong resource type",
            ));
        }
        self.compiled
            .history_validity
            .as_ref()
            .and_then(|validity| validity.get(index))
            .copied()
            .flatten()
            .ok_or_else(|| {
                crate::Error::invalid_usage("resource is not a session-owned named resource")
            })
    }

    pub fn report_error(&self, error: crate::Error) {
        self.errors.borrow_mut().get_or_insert(error);
    }
    /// Retain a pipeline, query or other native object with no image/buffer
    /// memory domain. Graph memory must be declared before recording begins.
    pub fn retain_object<T: crate::tracking::GpuUse + 'static>(
        &self,
        owner: std::sync::Arc<T>,
    ) -> crate::Result<()> {
        let result = owner
            .visit_uses(&mut |cell| {
                if cell.ownership_range().is_some() {
                    Err(crate::Error::invalid_usage(
                        "graph memory references require a declared resource access",
                    ))
                } else {
                    Ok(())
                }
            })
            .and_then(|()| self.owners.borrow_mut().add_object(owner));
        self.retention_result(result)
    }
    /// # Safety
    /// Every native memory reference must be covered by this pass's validated
    /// declarations or the formal immutable upload protocol.
    pub(crate) unsafe fn retain_recorded<T: crate::tracking::GpuUse + 'static>(
        &self,
        owner: std::sync::Arc<T>,
    ) -> crate::Result<()> {
        let result = self.owners.borrow_mut().add_recorded(owner);
        self.retention_result(result)
    }
    pub(crate) fn retain_upload_read(
        &self,
        buffer: std::sync::Arc<crate::resource::buffer::Buffer>,
        offset: u64,
        size: u64,
        scope: crate::native::AccessScope,
    ) -> crate::Result<()> {
        let result = self
            .owners
            .borrow_mut()
            .add_upload_read(buffer, offset, size, scope);
        self.retention_result(result)
    }
    fn retention_result(&self, result: crate::Result<()>) -> crate::Result<()> {
        result.map_err(|error| {
            let shared = std::sync::Arc::new(error);
            self.report_error(shared.clone().into());
            shared.into()
        })
    }
}

pub(super) fn image_layout_from_plan(
    plan: &super::super::plan::CompiledPlan,
    pass_index: usize,
    resource: u32,
    range: super::super::resource::AccessRange,
) -> crate::Result<vk::ImageLayout> {
    use crate::tracking::range_state::{difference, intersection};
    let descriptor = plan
        .definition
        .resources
        .get(resource as usize)
        .ok_or_else(|| crate::Error::invalid_usage("image layout resource is out of range"))?
        .descriptor;
    if !matches!(descriptor, super::super::graph::Descriptor::Image(_)) {
        return Err(crate::Error::invalid_usage(
            "image layout query refers to a buffer",
        ));
    }
    let program = &plan.range_program;
    let pass = program
        .passes
        .get(pass_index)
        .filter(|pass| pass.pass == pass_index)
        .ok_or_else(|| crate::Error::invalid_usage("image layout scheduled pass is absent"))?;
    let uses = program
        .uses
        .get(pass.uses.clone())
        .ok_or_else(|| crate::Error::invalid_usage("image layout pass uses are out of range"))?;
    let requested =
        super::super::sync_plan::native_range(range, descriptor, program.separate_depth_stencil)?;
    let mut missing = smallvec::SmallVec::<[_; 8]>::from_slice(&[requested]);
    let mut layout = None;
    for usage in uses.iter().filter(|usage| usage.resource == resource) {
        if intersection(usage.range, requested).is_none() {
            continue;
        }
        if layout.is_some_and(|layout| layout != usage.layout) {
            return Err(crate::Error::invalid_usage(
                "one image descriptor spans different native layouts",
            ));
        }
        layout = Some(usage.layout);
        let mut next = smallvec::SmallVec::<[_; 8]>::new();
        for range in missing {
            next.extend(difference(range, usage.range));
        }
        missing = next;
    }
    if !missing.is_empty() {
        return Err(crate::Error::invalid_usage(
            "image descriptor range has no complete declared native use",
        ));
    }
    layout.ok_or_else(|| {
        crate::Error::invalid_usage("image descriptor has no declared native layout")
    })
}
