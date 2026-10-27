//! Descriptor pages, page pools and page cursors for group reservations.
use crate::Error;
use crate::Result;
use crate::pipeline::shader::BindlessShaderAbi;
use crate::profile::RecordCount;
use crate::profile::RecordPhase;
use crate::profile::RecordSpan;
use crate::profile::record_count;
use parking_lot::Mutex;
use std::sync::Arc;
use super::BindlessHeap;
use super::parameters;
use super::parameters::BindlessParameters;
use super::parameters::PreparedBindlessParameters;
use super::stats::BindlessOccupancy;

/// 页选择只服务现有绑定 ABI；每个物理页仍是独立 BindlessHeap。
pub struct BindlessPagePool {
    template: Arc<BindlessHeap>,
    pages: Mutex<crate::collections::temp_list::TempList<Arc<BindlessPage>>>,
    growth: Mutex<()>,
    frame_bank: std::sync::atomic::AtomicUsize,
    parameter_cpu: parameters::cpu::ParameterCpuPool,
    _memory: crate::accounting::CpuMemoryReservation,
}

pub(super) struct BindlessPage {
    heap: Arc<BindlessHeap>,
    bank: usize,
    group: Mutex<()>,
    _memory: crate::accounting::CpuMemoryReservation,
}

/// 只在完整组准备期间借用；实际 leases 返回后仍由命令完成点保护。
pub struct GroupReservation<'a> {
    heap: &'a Arc<BindlessHeap>,
    cpu_pool: &'a parameters::cpu::ParameterCpuPool,
    cpu: &'a mut parameters::cpu::ParameterCpuCursor,
}

#[derive(Debug)]
pub enum GroupPreparationError {
    InsufficientSlots { required: [usize; 4] },
    Error(Error),
}

impl From<Error> for GroupPreparationError {
    fn from(error: Error) -> Self {
        Self::Error(error)
    }
}

pub type GroupPreparationResult<T> = std::result::Result<T, GroupPreparationError>;

impl GroupReservation<'_> {
    pub fn heap(&self) -> &Arc<BindlessHeap> {
        self.heap
    }
    pub fn parameters(&mut self, abi: Arc<BindlessShaderAbi>, push: &[u8]) -> Result<BindlessParameters> {
        BindlessParameters::from_credit(self.heap.clone(), abi, push,
            |bytes| self.cpu.claim(self.cpu_pool, bytes))
    }
    /// A previous block can only be reused inside its actual physical page.
    pub fn with_push(&mut self, previous: &PreparedBindlessParameters, push: &[u8],
        allocate: impl FnOnce(u64) -> Result<crate::resource::DynamicConstantsChunk>,
    ) -> GroupPreparationResult<Option<PreparedBindlessParameters>> {
        if !previous.belongs_to(self.heap) { return Ok(None); }
        self.require_slots([0, 0, 1, 0])?;
        previous.with_push_credit(push, allocate, |bytes| self.cpu.claim(self.cpu_pool, bytes))
            .map(Some).map_err(Into::into)
    }
    /// 所有页 writer 都在同一互斥事务内；准入后其他 writer 不能抢占空槽。
    /// 需求必须包含本次缺失缓存、实际新增资源和不可变参数块。
    pub fn require_slots(&self, required: [usize; 4]) -> GroupPreparationResult<()> {
        let _trace = RecordSpan::new(RecordPhase::RequireSlots);

        let check = || {
            let slots = self.heap.slots.lock();
            let capacity = std::array::from_fn(|kind| slots.entries[kind].len());
            let free = std::array::from_fn(|kind| slots.free[kind].len());
            (capacity, free)
        };
        let (capacity, free): ([usize; 4], [usize; 4]) = check();
        if required.iter().zip(capacity).any(|(need, cap)| *need > cap) {
            return Err(Error::unsupported(format!(
                "complete bindless group exceeds a fresh page: required={required:?}, capacity={capacity:?}"
            )).into());
        }
        if required.iter().zip(free).all(|(need, free)| *need <= free) {
            return Ok(());
        }
        // manager 锁已释放；只读取实际完成值，不等待 GPU 或设备 idle。
        self.heap.collect()?;
        let (_, free) = check();
        if required.iter().zip(free).all(|(need, free)| *need <= free) {
            Ok(())
        } else {
            Err(GroupPreparationError::InsufficientSlots { required })
        }
    }
}

impl BindlessPagePool {
    pub fn new(template: Arc<BindlessHeap>) -> Result<Arc<Self>> {
        let budget = &template.device.memory.stats;
        let memory = crate::accounting::reserve_shared_cpu_metadata::<Self>(budget, 0)?;
        let page = Self::page(template.clone(), 0)?;
        // 页数另受 CPU/native descriptor 全池预算约束；此限只约束元数据地址空间。
        let mut pages = crate::collections::temp_list::TempList::new(65536, budget.clone())?;
        pages.add(page)?;
        let parameter_cpu = parameters::cpu::ParameterCpuPool::new(budget.clone())?;
        Ok(Arc::new(Self {
            template,
            pages: Mutex::new(pages),
            growth: Mutex::new(()),
            frame_bank: std::sync::atomic::AtomicUsize::new(0),
            parameter_cpu,
            _memory: memory,
        }))
    }
    fn page(heap: Arc<BindlessHeap>, bank: usize) -> Result<Arc<BindlessPage>> {
        heap.set_residency(true);
        let memory = crate::accounting::reserve_shared_cpu_metadata::<BindlessPage>(
            &heap.device.memory.stats,
            0,
        )?;
        Ok(Arc::new(BindlessPage {
            heap,
            bank,
            group: Mutex::new(()),
            _memory: memory,
        }))
    }
    pub fn template(&self) -> &Arc<BindlessHeap> {
        &self.template
    }
    pub fn cursor(self: &Arc<Self>) -> BindlessPageCursor {
        BindlessPageCursor {
            pool: self.clone(),
            current: None,
            bank: self.frame_bank.load(std::sync::atomic::Ordering::Relaxed),
            cpu: parameters::cpu::ParameterCpuCursor::default(),
        }
    }
    /// Rotate descriptor arenas with the host's display-frame boundary. Each
    /// arena learns its capacity during startup even when the GPU happens to
    /// finish cold frames immediately. Leases still govern actual reuse; this
    /// never resets live slots or guesses GPU completion from a frame index.
    pub fn advance_frame(&self) {
        self.parameter_cpu.trim();
        let count = self.template.device.frames_in_flight() as usize;
        self.frame_bank
            .fetch_update(
                std::sync::atomic::Ordering::Relaxed,
                std::sync::atomic::Ordering::Relaxed,
                |bank| Some((bank + 1) % count),
            )
            .unwrap();
    }
    /// Sample the pages present at request start. Each heap is read once,
    /// outside the manager lock. Pages only append while this pool is alive;
    /// newly appended pages are included by the next request.
    pub fn sampled_occupancy(&self) -> BindlessOccupancy {
        let count = self.page_count();
        let mut sampled = BindlessOccupancy::default();
        for index in 0..count {
            let heap = {
                let pages = self.pages.lock();
                pages.as_slice()[index].heap.clone()
            };
            sampled.include(heap.stats());
        }
        sampled
    }
    pub fn page_count(&self) -> usize {
        self.pages.lock().as_slice().len()
    }
    fn available(&self, excluded: &[Arc<BindlessPage>], bank: usize) -> Option<Arc<BindlessPage>> {
        let pages = self.pages.lock();
        pages
            .as_slice()
            .iter()
            .filter(|page| {
                page.bank == bank && !excluded.iter().any(|used| Arc::ptr_eq(used, page))
            })
            // Prefer an idle cursor's page, but a held cursor does not imply
            // that its descriptor slots are full. Group admission is already
            // serialized by page.group; try those slots before growing.
            .min_by_key(|page| usize::from(Arc::strong_count(page) != 1))
            .cloned()
    }
    fn append_page(
        &self,
        excluded: &[Arc<BindlessPage>],
        bank: usize,
    ) -> Result<(Arc<BindlessPage>, bool)> {
        let _growth = self.growth.lock();
        // Another recorder may have added usable capacity since our search.
        if let Some(page) = self.available(excluded, bank) {
            return Ok((page, false));
        }
        // 原生创建和潜在失败清理都在 manager 锁外。
        let page = Self::page(self.template.new_page()?, bank)?;
        let admitted = {
            let mut pages = self.pages.lock();
            pages.add(page.clone()).map(|_| ())
        };
        // add 失败时 manager 内只释放一个 clone，最后的 page owner 在锁外释放。
        admitted?;
        Ok((page, true))
    }
}

impl Drop for BindlessPagePool {
    fn drop(&mut self) {
        // 最后一个 pool Arc 消失后没有 cursor；此处不持 manager 锁获取 Slots。
        for page in self.pages.get_mut().as_slice() {
            // Resident leases hold their heap: break the cycle with the pool.
            page.heap.set_residency(false);
        }
    }
}

/// 一个录制 callback 的页游标；同页快路径不访问 manager，也不按 draw 扫描页池。
pub struct BindlessPageCursor {
    pool: Arc<BindlessPagePool>,
    current: Option<Arc<BindlessPage>>,
    bank: usize,
    cpu: parameters::cpu::ParameterCpuCursor,
}

impl BindlessPageCursor {
    pub fn prepare(
        &mut self,
        mut build: impl FnMut(
            &mut GroupReservation<'_>,
        ) -> GroupPreparationResult<PreparedBindlessParameters>,
    ) -> Result<PreparedBindlessParameters> {
        let _trace = RecordSpan::new(RecordPhase::CompleteGroup);
        record_count(RecordCount::CompleteGroupCalls, 1);

        let mut excluded = Vec::new();
        let mut created = false;
        loop {
            record_count(RecordCount::GroupAttempts, 1);
            let page = if let Some(current) = self.current.take() {
                record_count(RecordCount::CurrentPageReuse, 1);
                current
            } else if let Some(page) = {
                record_count(RecordCount::PageSearches, 1);
                self.pool.available(&excluded, self.bank)
            } {
                page
            } else if !created {
                let (page, appended) = self.pool.append_page(&excluded, self.bank)?;
                created = appended;
                if appended {
                    record_count(RecordCount::PagesCreated, 1);
                }
                page
            } else {
                return Err(Error::invalid_usage(
                    "fresh bindless page did not satisfy the admitted group",
                ));
            };
            let result = {
                let group_wait = RecordSpan::new(RecordPhase::GroupLockWait);
                let _guard = page.group.lock();
                drop(group_wait);
                build(&mut GroupReservation { heap: &page.heap,
                    cpu_pool: &self.pool.parameter_cpu, cpu: &mut self.cpu })
            };
            match result {
                Ok(prepared) => {
                    self.current = Some(page);
                    return Ok(prepared);
                }
                Err(GroupPreparationError::InsufficientSlots { .. }) => {
                    record_count(RecordCount::InsufficientRetries, 1);
                    excluded
                        .try_reserve(1)
                        .map_err(|_| Error::OutOfHostMemory)?;
                    excluded.push(page);
                }
                Err(GroupPreparationError::Error(error)) => {
                    self.current = Some(page);
                    return Err(error);
                }
            }
        }
    }
}
