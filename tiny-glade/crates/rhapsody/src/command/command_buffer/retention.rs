//! Resources retained by a command buffer until completion.
use ash::vk;
use std::sync::Arc;
use super::VulkanCommandBuffer;
use super::recorded_state::OpaqueUseRange;

#[derive(Clone, Copy)]
pub(super) enum RetainMode {
    Opaque,
    Recorded,
    Object,
}

#[derive(Clone, Copy)]
pub(super) enum MetadataFailure {
    DeviceLost,
    HostMemory,
    DeviceMemory,
    Timeout,
    Unsupported,
    InvalidUsage,
    SurfaceLost,
    OutOfDate,
    Exhausted,
    Vulkan(vk::Result),
}

impl MetadataFailure {
    fn from_error(error: &crate::Error) -> Self {
        match error {
            crate::Error::DeviceLost => Self::DeviceLost,
            crate::Error::OutOfHostMemory => Self::HostMemory,
            crate::Error::OutOfDeviceMemory => Self::DeviceMemory,
            crate::Error::Timeout => Self::Timeout,
            crate::Error::Exhausted(_) => Self::Exhausted,
            crate::Error::Unsupported(_) => Self::Unsupported,
            crate::Error::InvalidUsage { .. } => Self::InvalidUsage,
            crate::Error::SurfaceLost => Self::SurfaceLost,
            crate::Error::SurfaceOutOfDate => Self::OutOfDate,
            crate::Error::Vulkan(error) => Self::Vulkan(*error),
        }
    }
    pub(super) fn error(self) -> crate::Error {
        match self {
            Self::DeviceLost => crate::Error::DeviceLost,
            Self::HostMemory => crate::Error::OutOfHostMemory,
            Self::DeviceMemory => crate::Error::OutOfDeviceMemory,
            Self::Timeout => crate::Error::Timeout,
            Self::Exhausted => {
                crate::Error::exhausted("command resource admission previously failed")
            }
            Self::Unsupported => {
                crate::Error::unsupported("command resource admission previously failed")
            }
            Self::InvalidUsage => {
                crate::Error::invalid_usage("command resource admission previously failed")
            }
            Self::SurfaceLost => crate::Error::SurfaceLost,
            Self::OutOfDate => crate::Error::SurfaceOutOfDate,
            Self::Vulkan(error) => crate::Error::Vulkan(error),
        }
    }
}

impl VulkanCommandBuffer {
    /// Retain an explicitly opaque raw reference. Every native memory domain
    /// contributes a conservative Unknown patch to the same submission protocol.
    pub fn retain_opaque<T: crate::tracking::GpuUse + 'static>(
        &mut self,
        owner: Arc<T>,
    ) -> crate::Result<()> {
        self.retain_owner(owner, RetainMode::Opaque)
    }

    /// Retain a GPU object that exposes no buffer/image memory domain.
    pub fn retain_object<T: crate::tracking::GpuUse + 'static>(
        &mut self,
        owner: Arc<T>,
    ) -> crate::Result<()> {
        self.retain_owner(owner, RetainMode::Object)
    }

    /// # Safety
    /// Actual memory accesses must be covered by the validated native range
    /// program. This is private to producers which have established that proof.
    pub(crate) unsafe fn retain_recorded<T: crate::tracking::GpuUse + 'static>(
        &mut self,
        owner: Arc<T>,
    ) -> crate::Result<()> {
        self.retain_owner(owner, RetainMode::Recorded)
    }

    /// # Safety
    /// The synchronous caller keeps every borrowed resource alive until the
    /// command's real completion; all memory accesses have native state patches.
    pub(crate) unsafe fn retain_borrowed_recorded(
        &mut self,
        owner: &dyn crate::tracking::GpuUse,
    ) -> crate::Result<()> {
        self.retain_borrowed(owner, RetainMode::Recorded)
    }

    fn retain_owner(
        &mut self,
        owner: crate::tracking::ResourceOwner,
        mode: RetainMode,
    ) -> crate::Result<()> {
        self.ensure_retention_allowed()?;
        let result = (|| {
            self.add_owner(&owner)?;
            self.visit_resource(owner.as_ref(), mode)
        })();
        self.remember_metadata_error(&result);
        result
    }

    pub(super) fn ensure_retention_allowed(&self) -> crate::Result<()> {
        if self.pool.device.is_device_lost() {
            return Err(crate::Error::DeviceLost);
        }
        if let Some(error) = self.range_failure {
            return Err(error.error());
        }
        if self.has_submissions() {
            return Err(crate::Error::invalid_usage(
                "cannot modify a submitted command's resources",
            ));
        }
        Ok(())
    }

    fn retain_borrowed(
        &mut self,
        owner: &dyn crate::tracking::GpuUse,
        mode: RetainMode,
    ) -> crate::Result<()> {
        self.ensure_retention_allowed()?;
        let result = self.visit_resource(owner, mode);
        self.remember_metadata_error(&result);
        result
    }

    fn visit_resource(
        &mut self,
        owner: &dyn crate::tracking::GpuUse,
        mode: RetainMode,
    ) -> crate::Result<()> {
        owner.visit_uses(&mut |cell| {
            cell.validate_device(self.pool.device.generation())?;
            if matches!(mode, RetainMode::Object) && cell.ownership_range().is_some() {
                return Err(crate::Error::invalid_usage(
                    "retain_object cannot retain a native memory resource",
                ));
            }
            if matches!(mode, RetainMode::Opaque) {
                self.note_opaque_use(cell.clone(), OpaqueUseRange::Whole)?;
            }
            self.add_use_cell(cell)
        })?;
        Ok(())
    }

    /// Keep `owner` alive until completion. A resource retained again by the
    /// same command (a heap or pipeline bound by every draw) is stored once.
    fn add_owner(&mut self, owner: &crate::tracking::ResourceOwner) -> crate::Result<()> {
        let key = Arc::as_ptr(owner) as *const () as usize as u64;
        if self.owner_set.contains_key(&key) {
            return Ok(());
        }
        self.owners.add(owner.clone())?;
        self.owner_set.insert(key, ());
        Ok(())
    }

    pub(super) fn add_use_cell(&mut self, cell: &Arc<crate::tracking::SyncCell>) -> crate::Result<()> {
        self.uses.insert(cell)
    }

    pub(crate) fn remember_metadata_error(&mut self, result: &crate::Result<()>) {
        if let Err(error) = result {
            if error.is_device_lost() {
                self.pool.device.queues.timelines.mark_device_lost();
            }
            self.range_failure
                .get_or_insert(MetadataFailure::from_error(error));
        }
    }

    /// Retain a buffer while describing exactly which bytes the native commands
    /// may access. Generic references to the same buffer still take precedence.
    /// # Safety
    /// The range must cover every read/write performed through this reference;
    /// matching Vulkan barriers and resource ownership are still required.
    pub unsafe fn retain_opaque_buffer_range(
        &mut self,
        buffer: &Arc<crate::resource::buffer::Buffer>,
        offset: u64,
        size: u64,
    ) -> crate::Result<()> {
        self.ensure_retention_allowed()?;
        let result = (|| {
            buffer
                .last_use
                .validate_device(self.pool.device.generation())?;
            let range = OpaqueUseRange::checked(offset, size, buffer.size)?;
            self.note_opaque_use(buffer.last_use.clone(), range)?;
            let owner: crate::tracking::ResourceOwner = buffer.clone();
            self.add_owner(&owner)?;
            self.visit_resource(buffer.as_ref(), RetainMode::Recorded)
        })();
        self.remember_metadata_error(&result);
        result
    }

    /// Only immutable, flushed formal constants-ring ranges use this role. The
    /// ring hands out bytes of a page only before its first submission, so the
    /// host write precedes the GPU read by queue-submit order alone; the page is
    /// retained for its completion point and carries no per-read native state.
    pub(crate) fn retain_upload_read(
        &mut self,
        buffer: &Arc<crate::resource::buffer::Buffer>,
        offset: u64,
        size: u64,
        scope: crate::native::AccessScope,
    ) -> crate::Result<()> {
        self.ensure_retention_allowed()?;
        let result = (|| {
            if !buffer.is_dynamic_constants_page()
                || scope.write
                || scope.stages.is_empty()
                || scope.access.is_empty()
                || !(vk::AccessFlags::SHADER_READ | vk::AccessFlags::UNIFORM_READ)
                    .contains(scope.access)
            {
                return Err(crate::Error::invalid_usage(
                    "upload read requires a formal immutable shader-read block",
                ));
            }
            OpaqueUseRange::checked(offset, size, buffer.size)?;
            buffer
                .last_use
                .validate_device(self.pool.device.generation())?;
            self.visit_resource(buffer.as_ref(), RetainMode::Recorded)?;
            Ok(())
        })();
        self.remember_metadata_error(&result);
        result
    }

    pub(crate) fn retain_resources(
        &mut self,
        owners: impl IntoIterator<Item = crate::command::binding::RetainedResource>,
    ) -> crate::Result<()> {
        for owner in owners {
            match owner.0 {
                crate::command::binding::Retention::Object(owner) => {
                    self.retain_owner(owner, RetainMode::Object)?
                }
                crate::command::binding::Retention::Recorded(owner) => {
                    self.retain_owner(owner, RetainMode::Recorded)?
                }
                crate::command::binding::Retention::UploadRead {
                    buffer,
                    offset,
                    size,
                    scope,
                } => self.retain_upload_read(&buffer, offset, size, scope)?,
            }
        }
        Ok(())
    }

    /// # Safety
    /// 调用者必须已等待覆盖此 CB 的提交凭据，不能仅依据 CPU 录制结束。
    pub unsafe fn release_completed_owners(&mut self) {
        self.owners.clear();
        self.owner_set.clear();
        self.range_uses.clear();
        self.state_patches.clear();
        self.range_failure = None;
        self.uses.release_completed(&self.pool.last_use);
        self.last_use = crate::sync::SyncSet::EMPTY;
    }
}
