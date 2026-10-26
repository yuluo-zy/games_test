//! `render::ssbo` — typed GPU buffer wrappers (layer A1).
//!
//! 行为全部来自指令级核对（O），证据与反汇编复核方法见 `evidence/country-core-audit/L0_NOTES.md`：
//!  * `TypedStorageBuffer::new<T>`          0x1416152f0 (BUFFER_LEN=0 ×45) / 0x1416154c0 (=1) / 0x141616c00 (=0x6c0)
//!  * `impl$7::new_storage_buffer<T>`       0x1416207e0  (InstanceBuffer)
//!  * `impl$4::new_storage_buffer<T>`       0x14161ad70  (MappedStorageBuffer, 256 KiB)
//!  * `impl$0::new_storage_buffer<T>`       0x14160f920  (按字节数据的创建器)
//!  * `gpu_write_ranges_nonoverlapping<T>`  0x14161ab00
//! 复用库（rhapsody）以 `rhapsody_boundary` 占位——只复刻已观察到的调用形状，**不**自造其行为。

// Preserve the original instruction-level expression order/NaN policy, API call shapes, and evidence/test transcription. See README.md.
#![allow(
    clippy::doc_lazy_continuation,
    clippy::single_range_in_vec_init,
    clippy::type_complexity
)]
#![allow(clippy::needless_doctest_main)]

/// 占位：`rhapsody::raw::{buffer,device}`。签名由原版调用点的寄存器/栈参数反推（O）：
///  * `BufferDesc::new(size, usage, memory)` ← `edx=size, r8d=usage, r9b=memory`
///  * `BufferDesc::alignment(self, a)` ← `r8d=a`
///  * `create_buffer(device, &desc, name, data: Option<&[u8]>) -> Result<Buffer, _>`
///    ← `[rsp+0x20]=&desc, r8/r9=name(ptr,len), [rsp+0x28]=data.ptr, [rsp+0x30]=data.len`
///  * `update_buffer_bytes(device, &buffer, data, ranges, offset)`
///    ← `r8=&buffer, r9=data.ptr, [rsp+0x20]=data.len, [rsp+0x28]=ranges.ptr, [rsp+0x30]=ranges.len, [rsp+0x38]=offset`
pub mod rhapsody_boundary {
    use core::ops::Range;

    /// Vulkan `VkBufferUsageFlagBits` 值（O：立即数 0x22/0x12/0x40/0x80 与该枚举逐位吻合）。
    pub mod usage {
        pub const TRANSFER_DST: u32 = 0x02;
        pub const UNIFORM_BUFFER: u32 = 0x10;
        pub const STORAGE_BUFFER: u32 = 0x20;
        pub const INDEX_BUFFER: u32 = 0x40;
        pub const VERTEX_BUFFER: u32 = 0x80;
    }

    /// rhapsody `Buffer`（40 字节，O：`create_buffer` 的 Ok 载荷被整块拷进 `TypedStorageBuffer`；
    /// `gpu_flush` 在 `+0x18` 读取字节容量，对应 `desc.size`）。其余字段 U。
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct Buffer {
        pub id: u64,
        pub generation: u64,
        pub _gap10: u64,
        /// +0x18：`desc.size`（O）。
        pub byte_size: u64,
        pub _gap20: u64,
    }

    /// `rhapsody::resource::BufferDesc`（占位，仅保留调用点传入的信息）。
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct BufferDesc {
        pub size: usize,
        pub usage: u32,
        /// `r9b`：1 = 设备本地（Typed / Vertex），2 = 主机可见（Mapped、`impl$0`）。含义 U，值 O。
        pub memory: u8,
        pub alignment: usize,
        pub gpu_only: bool,
    }

    impl BufferDesc {
        pub fn new(size: usize, usage: u32, memory: u8) -> Self {
            Self {
                size,
                usage,
                memory,
                alignment: 1,
                gpu_only: false,
            }
        }
        /// `BufferDesc::new_gpu_only(size, usage)`（O：0x140bd4e20，`build_vao`/顶点缓冲使用）。
        pub fn new_gpu_only(size: usize, usage: u32) -> Self {
            Self {
                size,
                usage,
                memory: 1,
                alignment: 1,
                gpu_only: true,
            }
        }
        /// `BufferDesc::alignment`（O：0x140bd4f20）。
        pub fn alignment(mut self, a: usize) -> Self {
            self.alignment = a;
            self
        }
    }

    /// Preserve the renderer's concrete failure category across the host boundary.
    pub use rhapsody::Error as BackendError;

    /// Owns the native allocation and its host-write synchronization authority.
    /// A production CPU upload must go through this lease, not a retained raw
    /// mapping that can bypass a queued render frame or a prior GPU read.
    pub trait MappedUploadWriter: Send + Sync {
        fn write(&self, offset: usize, data: &[u8]) -> Result<(), BackendError>;
        fn try_read(
            &self, offset: usize, bytes: usize, read: &mut dyn FnMut(),
        ) -> Result<bool, BackendError>;
    }

    /// 占位：`rhapsody::device::Device`（只含本层用到的两个调用）。
    pub trait GraphicsDevice {
        fn create_buffer(
            &self,
            desc: &BufferDesc,
            name: &str,
            data: Option<&[u8]>,
        ) -> Result<Buffer, BackendError>;

        /// 释放由同一设备创建的缓冲；CPU 记录设备不拥有物理资源。
        fn release_buffer(&self, _buffer: &Buffer) {}

        /// 把 `data[r]` 写到缓冲区 `offset + r` 处，`ranges` 互不重叠（O：来自两个调用点的实参）。
        fn update_buffer_bytes(
            &self,
            buffer: &Buffer,
            data: &[u8],
            ranges: &[Range<usize>],
            offset: usize,
        ) -> Result<(), BackendError>;

        /// Opt in a CPU-owned, reconstructible buffer to physical versioning.
        ///
        /// # Safety
        /// The entire allocation has one authoritative CPU source. GPU passes
        /// only read it; no persistent mapping, native handle or device address
        /// escapes. Updates must use update_cpu_buffer_bytes with that source.
        unsafe fn register_cpu_authoritative_buffer(&self, _buffer: &Buffer) {}

        /// `data` is the complete allocation; ranges select this frame's dirty
        /// bytes. A versioning host may upload the full source to a new owner
        /// when a queued frame still pins the previous physical contents.
        fn update_cpu_buffer_bytes(
            &self, buffer: &Buffer, data: &[u8], ranges: &[Range<usize>],
        ) -> Result<(), BackendError> {
            self.update_buffer_bytes(buffer, data, ranges, 0)
        }

        /// `rhapsody::resource::Buffer::mapped_slice_mut(&self, device)`（O 0x140bd4cd0）：持久映射的主机可见缓冲的字节切片；
        /// 设备不映射该缓冲 → `None`（`MappedStorageBuffer::new_storage_buffer` 随后 `Option::unwrap` panic）。
        /// B 阶段（i_api）并入本 trait：原先独立的 `MappedBuffers` 导致 `&dyn GraphicsDevice`（库侧 `SsboType::new_storage_buffer`
        /// 的唯一设备参数）构造不了 `MappedStorageBuffer`。占位默认值 `None`，真实实现 = rhapsody 的持久映射。
        fn mapped_slice_mut(&self, _buffer: &Buffer) -> Option<&mut [u8]> {
            None
        }
        fn mapped_upload_writer(&self, _buffer: &Buffer)
            -> Option<std::sync::Arc<dyn MappedUploadWriter>> { None }

        /// `rhapsody::resource::Buffer::device_address(&self, device)`（O 0x140bd4c00；`nani::trimesh` 的
        /// `Trimesh::build` 把块地址加上它）。原先在 `nani::trimesh` 单列的 `BufferDeviceAddress` trait 已并入此处。
        /// 占位默认值 0（真实实现 = `vkGetBufferDeviceAddress`，由 rhapsody 持有；不自造行为）。
        fn device_address(&self, _buffer: &Buffer) -> u64 {
            panic!("Buffer device-address backend is not connected")
        }

        /// Native ivy leaf element scale (0x1408c1631 / 0x1408c181d): when the
        /// GraphicsDevice byte `[device+0x1d]` is nonzero the packed leaf
        /// `size` is scaled to 0.85 (`0x3f59999a`), otherwise 1.0
        /// (`0x3f800000`). The crate models the factor with this hook; the
        /// D-group render backend reads the device byte.
        fn ivy_leaf_scale(&self) -> f32 {
            1.0
        }
    }

    /// `rhapsody::raw::bytes::slice_as_bytes<T>`.
    ///
    /// Kept as a `Copy` boundary helper: callers include `StorageElement` types
    /// that do not implement `bytemuck::NoUninit`; see the dependency audit.
    pub fn slice_as_bytes<T: Copy>(slice: &[T]) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(slice.as_ptr() as *const u8, core::mem::size_of_val(slice))
        }
    }

    /// 占位：`rhapsody::rg::ResourceHandle`（`sparse_instance_buffer` 的转换用）。
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct ResourceHandle(pub u64, pub u64);
}

/// 游戏侧 `BufferDesc`（`{size:usize, usage:u32, memory:u8, alignment:usize, gpu_only:bool}`）→
/// 真实 `rhapsody::resource::BufferDesc`（WIRING §2.3 映射表）。memory 1/2/3/4 →
/// `MemoryLocation::{GpuOnly, CpuToGpu, CpuToGpuLowPrio, GpuToCpu}`。
pub fn convert_buffer_desc(desc: &rhapsody_boundary::BufferDesc) -> rhapsody::resource::BufferDesc {
    use rhapsody::resource::MemoryLocation;
    let memory = match desc.memory {
        2 => MemoryLocation::CpuToGpu,
        3 => MemoryLocation::CpuToGpuLowPrio,
        4 => MemoryLocation::GpuToCpu,
        _ => MemoryLocation::GpuOnly,
    };
    let mut out = rhapsody::resource::BufferDesc::new(
        desc.size as u64,
        ash::vk::BufferUsageFlags::from_raw(desc.usage),
        memory,
    );
    if desc.alignment > 1 {
        out = out.with_alignment(desc.alignment as u16);
    }
    out
}

/// 全 crate 唯一的设备类型 `rhapsody_contract::GraphicsDevice` 实现本层的设备 trait
/// （step 0：原先 ssbo 的 trait 占位与 lib.rs 的结构体占位互不相通）。
/// D 组接入（WIRING §2.3/§3.3）：游戏侧 40B `Buffer` 记录装不下 `ManagedBuffer`（**登记缺口 §4.8**），
/// 真实设备调用已接到 `rhapsody_contract::GraphicsDevice::*`（`create_buffer`/`update_buffer_bytes`/
/// `buffer_mapped_slice_mut`/`buffer_device_address`，均为 device_api.rs 同名方法）。此实现记录日志并
/// 返回未接错误，不补造、不假成功。`convert_buffer_desc` 是真实 desc 换算（WIRING §2.3 映射表）。
#[cfg(not(test))]
impl GraphicsDevice for crate::rhapsody_contract::GraphicsDevice {
    unsafe fn register_cpu_authoritative_buffer(&self, buffer: &Buffer) {
        if let Some(host) = self.buffer_host() {
            // SAFETY: the caller supplies the same authority contract.
            unsafe { host.register_cpu_authoritative_buffer(buffer); }
        }
    }

    fn update_cpu_buffer_bytes(
        &self, buffer: &Buffer, data: &[u8], ranges: &[Range<usize>],
    ) -> Result<(), rhapsody_boundary::BackendError> {
        if let Some(host) = self.buffer_host() {
            return host.update_cpu_buffer_bytes(buffer, data, ranges);
        }
        <Self as GraphicsDevice>::update_buffer_bytes(self, buffer, data, ranges, 0)
    }

    fn release_buffer(&self, buffer: &Buffer) {
        if let Some(host) = self.buffer_host() {
            host.release_buffer(buffer);
        }
    }
    fn create_buffer(
        &self,
        desc: &BufferDesc,
        name: &str,
        data: Option<&[u8]>,
    ) -> Result<Buffer, rhapsody_boundary::BackendError> {
        if let Some(host) = self.buffer_host() {
            return host.create_buffer(desc, name, data);
        }
        log::warn!(
            target: "country_core::render::ssbo",
            "create_buffer({name}): registered gap (WIRING §4.8), game-side 40B Buffer cannot hold ManagedBuffer; real desc = {:?}",
            convert_buffer_desc(desc)
        );
        Err(rhapsody_boundary::BackendError::unsupported(
            "SSBO backend requires a connected managed-buffer host",
        ))
    }

    fn update_buffer_bytes(
        &self,
        buffer: &Buffer,
        data: &[u8],
        ranges: &[Range<usize>],
        offset: usize,
    ) -> Result<(), rhapsody_boundary::BackendError> {
        if let Some(host) = self.buffer_host() {
            return host.update_buffer_bytes(buffer, data, ranges, offset);
        }
        log::warn!(
            target: "country_core::render::ssbo",
            "update_buffer_bytes: registered gap (WIRING §4.8), needs ManagedBuffer; real path = device_api::update_buffer_bytes"
        );
        Err(rhapsody_boundary::BackendError::unsupported(
            "SSBO backend requires a connected managed-buffer host",
        ))
    }

    /// 登记缺口 §4.8：真实 `buffer_mapped_slice_mut(&ManagedBuffer)`（device_api.rs:137）。
    /// 保持占位默认 `None`（调用方 `Option::unwrap` 即 panic，诚实未接）。
    fn mapped_slice_mut(&self, buffer: &Buffer) -> Option<&mut [u8]> {
        if let Some(host) = self.buffer_host() {
            return host.mapped_slice_mut(buffer);
        }
        log::warn!(
            target: "country_core::render::ssbo",
            "mapped_slice_mut: registered gap (WIRING §4.8), needs ManagedBuffer"
        );
        None
    }
    fn mapped_upload_writer(&self, buffer: &Buffer)
        -> Option<std::sync::Arc<dyn rhapsody_boundary::MappedUploadWriter>> {
        self.buffer_host().and_then(|host| host.mapped_upload_writer(buffer))
    }

    /// 登记缺口 §4.8：真实 `buffer_device_address(&ManagedBuffer)`（device_api.rs:144）。
    fn device_address(&self, buffer: &Buffer) -> u64 {
        if let Some(host) = self.buffer_host() {
            return host.device_address(buffer);
        }
        log::warn!(
            target: "country_core::render::ssbo",
            "device_address: registered gap (WIRING §4.8), needs ManagedBuffer; real path = device_api::buffer_device_address"
        );
        panic!("registered gap (WIRING §4.8): buffer_device_address needs ManagedBuffer")
    }
}

#[cfg(test)]
impl GraphicsDevice for crate::rhapsody_contract::GraphicsDevice {
    fn create_buffer(
        &self,
        desc: &BufferDesc,
        _name: &str,
        _data: Option<&[u8]>,
    ) -> Result<Buffer, rhapsody_boundary::BackendError> {
        Ok(Buffer {
            byte_size: desc.size as u64,
            ..Buffer::default()
        })
    }

    fn update_buffer_bytes(
        &self,
        _buffer: &Buffer,
        _data: &[u8],
        _ranges: &[Range<usize>],
        _offset: usize,
    ) -> Result<(), rhapsody_boundary::BackendError> {
        Ok(())
    }

    /// 占位：没有 GPU，持久映射区以泄漏的零页代替（16 B 对齐，长度 = `desc.size`）；真实实现由 rhapsody 的映射提供。
    fn mapped_slice_mut(&self, buffer: &Buffer) -> Option<&mut [u8]> {
        let words = (buffer.byte_size as usize).div_ceil(16);
        let page: &'static mut [u128] = Box::leak(vec![0u128; words].into_boxed_slice());
        // SAFETY: `u128` 切片按字节重解释；长度截到 `byte_size`，对齐 16 ≥ 任意 `StorageElement` 的对齐（含 Vec4）。
        Some(unsafe {
            core::slice::from_raw_parts_mut(page.as_mut_ptr() as *mut u8, buffer.byte_size as usize)
        })
    }
}

use core::marker::PhantomData;
use core::ops::Range;
use rhapsody_boundary::{Buffer, BufferDesc, GraphicsDevice, slice_as_bytes, usage};

/// 缓冲元素的静态描述（O：每个 `T` 的单态化各自带着调试名、`BUFFER_LEN` 与 `default_value`）。
pub trait StorageElement: Copy {
    /// 传给 `create_buffer` 的调试名（O：如 "VelocityOnlyInstanceData"）。
    const NAME: &'static str;
    /// 0 = 长度由初始数据决定；1 = 单个结构体（uniform）；N>1 = 固定 N 个元素。
    const BUFFER_LEN: usize = 0;
    /// `BUFFER_LEN == 1` 且无初始数据时使用（O：`NaniDrawListOffsetDefaults::default_value`）。
    fn default_value() -> Option<Self> {
        None
    }
    /// `MappedStorageBuffer` 的主机内存种类（`BufferDesc::new(.., memory)` 的 `r9b`）。21 个 `impl$4::new_storage_buffer`
    /// 实例里只观测到 2 或 4：2 = Cobblestone/Garden/Path/WaterRasterStaging、Butterfly、Firefly、MouseTerrainRaycast、
    /// NormalDebug；4 = TerrainHeight/VisualTerrainHeight、四个 mask、BushCpu、StairsCounter、MouseDepthValue、
    /// DofFocusReadback、FireflyVisibility。语义 U（I：2 = CPU→GPU 上传，4 = GPU→CPU 回读）。仅 `MappedStorageBuffer` 使用。
    const MAPPED_MEMORY: u8 = MAPPED_MEMORY_UPLOAD;
}

/// `MappedStorageBuffer` 内存种类 2（O 数值；名称 I）。
pub const MAPPED_MEMORY_UPLOAD: u8 = 2;
/// `MappedStorageBuffer` 内存种类 4（O 数值；名称 I）。
pub const MAPPED_MEMORY_READBACK: u8 = 4;

/// 能回答“实例数”的存储（O：`InstanceBuffer<T>` `+0` 的 u64；`ivy_leaf_shader`/`roof_spire`/`roof_support_beam` 经虚表槽 +0x18
/// （`as_any`，返回具体对象指针）直接读 `[rax]`）。
pub trait InstanceCounted {
    fn instance_count(&self) -> usize;
}

impl<T> InstanceCounted for InstanceBuffer<T> {
    fn instance_count(&self) -> usize {
        self.instance_count
    }
}

/// `GraphicsDevice::update_buffer_bytes` 失败的包装（O：调用点 `Result::unwrap`，故写入失败 = panic）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BufferWriteError {
    OutOfBounds,
    Device,
}

/// 40 字节的类型化存储缓冲（O：`new` 把 `create_buffer` 的 Ok 载荷整块拷入）。
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TypedStorageBuffer<T> {
    pub buffer: Buffer,
    _marker: PhantomData<T>,
}

impl<T: StorageElement> TypedStorageBuffer<T> {
    /// Source-host carrier for a buffer created by a specialized original
    /// producer (for example indirect arguments with usage 0x122). This does
    /// not allocate, upload, choose usage, or change `new`'s native policy.
    pub fn from_created_buffer(buffer: Buffer) -> Self {
        Self {
            buffer,
            _marker: PhantomData,
        }
    }

    /// 创建固定容量设备缓冲，不分配临时清零数组；调用方只发布已写入的槽位。
    pub fn try_new_uninitialized(
        device: &dyn GraphicsDevice,
    ) -> Result<Self, rhapsody_boundary::BackendError> {
        let byte_size = T::BUFFER_LEN
            .checked_mul(core::mem::size_of::<T>())
            .filter(|&size| size > 0)
            .ok_or_else(|| rhapsody_boundary::BackendError::invalid_usage("缓冲容量无效或溢出"))?;
        let description =
            BufferDesc::new(byte_size, usage::STORAGE_BUFFER | usage::TRANSFER_DST, 1).alignment(4);
        let buffer = device.create_buffer(&description, T::NAME, None)?;
        Ok(Self {
            buffer,
            _marker: PhantomData,
        })
    }

    /// `TypedStorageBuffer::new<T>`（O）。
    ///  * 有初始数据：`BUFFER_LEN > 0 && len > BUFFER_LEN` → panic `Excessive data size supplied for buffer {type}`；
    ///  * 无初始数据：`BUFFER_LEN == 0` → panic `Storage buffer {name} (type {type}) has \`BUFFER_LEN\` of zero. …`；
    ///    `BUFFER_LEN == 1` → 用 `T::default_value()`（可无）；`BUFFER_LEN > 1` → 全 0；
    ///  * 缓冲大小：`BUFFER_LEN == 0` → `len*size_of::<T>()`，否则 `BUFFER_LEN*size_of::<T>()`；
    ///  * 用途：`BUFFER_LEN == 1` → `UNIFORM|TRANSFER_DST`(0x12)，否则 `STORAGE|TRANSFER_DST`(0x22)；`memory=1`、对齐 4。
    pub fn new(device: &dyn GraphicsDevice, initial: Option<&[T]>) -> Self {
        let es = core::mem::size_of::<T>();
        let owned: Vec<T>;
        let data: Option<&[T]> = match initial {
            Some(s) => {
                if T::BUFFER_LEN > 0 && s.len() > T::BUFFER_LEN {
                    panic!(
                        "Excessive data size supplied for buffer {}",
                        core::any::type_name::<T>()
                    );
                }
                Some(s)
            }
            None => match T::BUFFER_LEN {
                0 => panic!(
                    "Storage buffer {} (type {}) has `BUFFER_LEN` of zero. Such buffers must be created with initial data.",
                    T::NAME,
                    core::any::type_name::<T>()
                ),
                1 => match T::default_value() {
                    Some(v) => {
                        owned = vec![v];
                        Some(&owned[..])
                    }
                    None => None,
                },
                n => {
                    // O：分配 BUFFER_LEN*es 字节并清零，作为初始数据整块上传。
                    let z: Vec<u8> = vec![0u8; n * es];
                    return Self::create(device, n * es, T::BUFFER_LEN, Some(&z[..]));
                }
            },
        };
        let len = data.map_or(0, |s| s.len());
        let elems = if T::BUFFER_LEN == 0 {
            len
        } else {
            T::BUFFER_LEN
        };
        Self::create(device, elems * es, T::BUFFER_LEN, data.map(slice_as_bytes))
    }

    /// `impl$0::new_storage_buffer<T>`（O：0x14160f920 起共 38 个单态化，例 `SheepAnimationSsbo`）。
    /// 按**字节**给初始数据的创建器：
    ///  * `None` → 与 `new` 相同文案的 panic（`BUFFER_LEN` of zero），即此入口必须带数据；
    ///  * 断言 `len % size_of::<T>() == 0`、`ptr % 4 == 0`（`assert_failed<usize,usize>`）；
    ///  * `BufferDesc::new(len, STORAGE|TRANSFER_DST = 0x22, memory = 2).alignment(4)`，名称 `T::NAME`，
    ///    数据整块作为初始内容；`Result::unwrap` 失败即 panic。
    ///  与 `new` 的差别：memory 为 2（`new` 为 1），大小取字节长度而非 `BUFFER_LEN`。
    ///
    /// **`BUFFER_LEN > 0` 的单态化形态**（O：`DofAutofocusSsbo` 0x1416101f0、`ShadowMaskHistogramSsbo` 0x141612170、
    /// `ShadowMaskMeanSsbo` 0x141613720、`BushSsbo` 0x141610b60、`FlowerSsbo` 0x141613b90 等）：固定大小
    /// `BUFFER_LEN × size_of::<T>()`，`BufferDesc::new(size, 0x22, memory = 1).alignment(4)`——用途**始终 0x22**
    /// （`ShadowMaskMeanSsbo` 的 `BUFFER_LEN == 1` 也是 0x22，而 [`Self::new`] 对 `BUFFER_LEN == 1` 用 uniform 0x12）；
    /// 有初始数据时先断言 `len % size_of::<T>() == 0`、指针 4 对齐，`len >= size + size_of::<T>()`（即元素数超过
    /// `BUFFER_LEN`）→ `panic!("Excessive data size supplied for buffer {type}")`；**无初始数据不上传**。
    pub fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self {
        if T::BUFFER_LEN > 0 {
            return Self::new_fixed_storage_buffer(device, initial);
        }
        let Some(bytes) = initial else {
            panic!(
                "Storage buffer {} (type {}) has `BUFFER_LEN` of zero. Such buffers must be created with initial data.",
                T::NAME,
                core::any::type_name::<T>()
            );
        };
        checked_element_count::<T>(bytes);
        let desc = BufferDesc::new(bytes.len(), usage::STORAGE_BUFFER | usage::TRANSFER_DST, 2)
            .alignment(4);
        let buffer = device
            .create_buffer(&desc, T::NAME, Some(bytes))
            .expect("called `Result::unwrap()` on an `Err` value: create_buffer");
        Self {
            buffer,
            _marker: PhantomData,
        }
    }

    /// [`Self::new_storage_buffer`] 的 `BUFFER_LEN > 0` 分支。
    fn new_fixed_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self {
        let es = core::mem::size_of::<T>();
        let size = T::BUFFER_LEN * es;
        if let Some(bytes) = initial {
            checked_element_count::<T>(bytes);
            if bytes.len() >= size + es {
                panic!(
                    "Excessive data size supplied for buffer {}",
                    core::any::type_name::<T>()
                );
            }
        }
        let desc =
            BufferDesc::new(size, usage::STORAGE_BUFFER | usage::TRANSFER_DST, 1).alignment(4);
        let buffer = device
            .create_buffer(&desc, T::NAME, initial)
            .expect("called `Result::unwrap()` on an `Err` value: create_buffer");
        Self {
            buffer,
            _marker: PhantomData,
        }
    }

    fn create(
        device: &dyn GraphicsDevice,
        size: usize,
        buffer_len: usize,
        data: Option<&[u8]>,
    ) -> Self {
        let u = if buffer_len == 1 {
            usage::UNIFORM_BUFFER
        } else {
            usage::STORAGE_BUFFER
        } | usage::TRANSFER_DST;
        let desc = BufferDesc::new(size, u, 1).alignment(4);
        let buffer = device
            .create_buffer(&desc, T::NAME, data)
            .expect("called `Result::unwrap()` on an `Err` value: create_buffer");
        Self {
            buffer,
            _marker: PhantomData,
        }
    }

    /// 缓冲字节容量（O：`gpu_flush` 读取的 `buffer.desc.size`）。
    pub fn byte_size(&self) -> usize {
        self.buffer.byte_size as usize
    }

    /// `gpu_write_ranges_nonoverlapping`（O：0x14161ab00）：把元素区间换算成字节区间
    /// `(start*es .. end*es)` 后一次 `update_buffer_bytes(.., data, ranges, offset)`；`ranges` 为空直接返回。
    /// 失败时原版 `Result::unwrap` → panic。
    pub fn gpu_write_ranges_nonoverlapping(
        &self,
        device: &dyn GraphicsDevice,
        data: &[T],
        ranges: &[Range<usize>],
        offset: usize,
    ) {
        if ranges.is_empty() {
            return;
        }
        let es = core::mem::size_of::<T>();
        let byte_ranges: Vec<Range<usize>> =
            ranges.iter().map(|r| r.start * es..r.end * es).collect();
        device
            .update_buffer_bytes(&self.buffer, slice_as_bytes(data), &byte_ranges, offset)
            .expect("called `Result::unwrap()` on an `Err` value: update_buffer_bytes");
    }

    /// I（内联、未见独立符号）：整块写入。
    pub fn gpu_write(
        &self,
        device: &dyn GraphicsDevice,
        data: &[T],
    ) -> Result<(), BufferWriteError> {
        self.gpu_write_with_offset(device, data, 0)
    }

    /// I：带字节偏移的写入，越界返回 `OutOfBounds`。
    pub fn gpu_write_with_offset(
        &self,
        device: &dyn GraphicsDevice,
        data: &[T],
        offset: usize,
    ) -> Result<(), BufferWriteError> {
        let bytes = slice_as_bytes(data);
        match offset.checked_add(bytes.len()) {
            Some(end) if end <= self.byte_size() => {}
            _ => return Err(BufferWriteError::OutOfBounds),
        }
        device
            .update_buffer_bytes(&self.buffer, bytes, &[0..bytes.len()], offset)
            .map_err(|error| {
                log::error!("SSBO {} upload at {offset} ({} bytes): {error}", T::NAME, bytes.len());
                BufferWriteError::Device
            })
    }
}

/// `data.len() % size_of::<T>() == 0` 与 `ptr % 4 == 0` 两个断言（O：`impl$7` / `impl$0` 共有）。
fn checked_element_count<T: StorageElement>(bytes: &[u8]) -> usize {
    let es = core::mem::size_of::<T>();
    assert_eq!(bytes.len() % es, 0);
    assert_eq!((bytes.as_ptr() as usize) % 4, 0);
    bytes.len() / es
}

/// `InstanceBuffer<T>`：`{ instance_count, buffer }`，48 字节（O：`impl$7::new_storage_buffer`）。
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct InstanceBuffer<T> {
    pub instance_count: usize,
    pub buffer: TypedStorageBuffer<T>,
}

impl<T: StorageElement> InstanceBuffer<T> {
    /// `impl$7::new_storage_buffer<T>`（O：0x1416207e0）。
    ///  * `Some(bytes)`：断言 `len % size_of::<T>() == 0` 与 4 字节对齐，`instance_count = len/size_of::<T>()`，
    ///    以该切片调用 `TypedStorageBuffer::new`；
    ///  * `None`：`T::default_value()` 为 `Some(v)` → 用单元素 `[v]` 创建，`instance_count = 1`；
    ///    否则 `TypedStorageBuffer::new(.., None)`，`instance_count = 0`。
    ///
    /// 旧实现里的 `count % 4 == 0` 断言在原函数中不存在（已删除）。
    pub fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self {
        match initial {
            Some(bytes) => {
                let count = checked_element_count::<T>(bytes);
                let typed: &[T] =
                    unsafe { core::slice::from_raw_parts(bytes.as_ptr() as *const T, count) };
                Self {
                    instance_count: count,
                    buffer: TypedStorageBuffer::new(device, Some(typed)),
                }
            }
            None => match T::default_value() {
                Some(v) => {
                    let one = [v];
                    Self {
                        instance_count: 1,
                        buffer: TypedStorageBuffer::new(device, Some(&one)),
                    }
                }
                None => Self {
                    instance_count: 0,
                    buffer: TypedStorageBuffer::new(device, None),
                },
            },
        }
    }
}

/// 历史常量：`Cobblestone*/Garden/Path/WaterRasterStagingSsbo` 这 6 个 u8 元素的暂存缓冲恰好是 0x40000 B
/// （O：`BUFFER_LEN × size_of::<T>()` = 0x40000 × 1）。**不是** `MappedStorageBuffer` 的通用大小。
pub const MAPPED_BUFFER_BYTES: usize = 0x40000;

/// `MappedStorageBuffer<T>`（O：`impl$4::new_storage_buffer<T>` 共 21 个实例，0x14161ad70 起；返回 0x38 B）：
/// `+0x00` 映射切片指针、`+0x08` 映射切片**元素数**（字节数 / `size_of::<T>()`）、`+0x10` `Buffer`（0x28 B）。
///
/// 21 个实例的真实形态（逐实例核对 `BufferDesc::new(size, 0x22, memory).alignment(a)` 与上限比较）：
///  * 缓冲大小 = `T::BUFFER_LEN * size_of::<T>()`（Cobblestone2 的 0x40000 = u8 元素 × 0x40000；TerrainHeight 0x80000 = 4 B × 0x20000；
///    Butterfly 0x300 = 32 B × 24；Firefly 0x600 = 16 B × 96；FireflyVisibility 0x180 = 4 B × 96；BushCpu 0x12c08 = 8 B × 0x2581；
///    Mouse 0x20 = 0x20 B × 1；StairsCounter 4 …）；usage 恒 0x22；memory = `T::MAPPED_MEMORY`（2 / 4）；
///    对齐 = `align_of::<T>()`（staging 1，Mouse 0x10，其余 4）；
///  * `Some(data)`：先 `assert_eq!(data.len() % size_of::<T>(), 0)`、`assert_eq!(data.ptr % align_of::<T>(), 0)`
///    （`assert_failed<usize,usize>`，无自定义消息；`size_of::<T>() == 1` 时前者被折叠），再
///    `data.len() >= size + size_of::<T>()`（即 `> size`）→ `panic!("Excessive data size supplied for buffer {type}")`；
///  * `None`：分配 `size` 字节：有缺省元素（`ButterflySsbo`：`position 0、anim_t 0、forward (0,0,-1)`）则逐元素填充，否则全零，作为初始数据；
///  * 创建（`Result::unwrap`）→ `mapped_slice_mut`（`Option::unwrap`）→ `assert!(mapped.len() >= desc.size)`
///    （文案 "assertion failed: data.len() >= buffer.buffer.desc.size"）→ 把映射字节切片重解释成 `&mut [T]`
///    （`assert_eq!(mapped.len() % size_of::<T>(), 0)`、`assert_eq!(mapped.ptr % align_of::<T>(), 0)`）。
pub struct MappedStorageBuffer<'a, T> {
    /// `+0x00/+0x08`：映射区，按元素类型化。
    pub mapped: &'a mut [T],
    /// `+0x10`。
    pub buffer: Buffer,
    upload_writer: Option<std::sync::Arc<dyn rhapsody_boundary::MappedUploadWriter>>,
    /// Writer-backed owners: the game's authoritative CPU contents. Edits run
    /// here and only changed spans travel as owned GPU-ordered uploads, so a
    /// producer never touches (or waits for) the mapping an in-flight frame reads.
    shadow: Option<Vec<T>>,
    /// Reused pre-edit copy for change detection.
    before: Vec<T>,
    pending: Vec<Range<usize>>,
}

impl<'a, T: StorageElement> MappedStorageBuffer<'a, T> {
    /// Adapt an existing mapping whose synchronization remains with its owner
    /// (CPU test mappings and GPU readback owners use this constructor).
    pub fn from_mapping(mapped: &'a mut [T], buffer: Buffer) -> Self {
        Self { mapped, buffer, upload_writer: None, shadow: None, before: Vec::new(), pending: Vec::new() }
    }

    /// Adapt a manually constructed native mapping while retaining the host's
    /// range synchronization and the actual allocation owner.
    pub fn from_mapping_with_access(
        mapped: &'a mut [T], buffer: Buffer,
        access: Option<std::sync::Arc<dyn rhapsody_boundary::MappedUploadWriter>>,
    ) -> Self {
        Self { mapped, buffer, upload_writer: access, shadow: None, before: Vec::new(), pending: Vec::new() }
    }
    /// 缓冲字节大小：`BUFFER_LEN * size_of::<T>()`（编译期常量）。
    pub const fn byte_len() -> usize {
        T::BUFFER_LEN * core::mem::size_of::<T>()
    }

    /// O：见类型文档。映射区由设备拥有，这里以设备返回的切片（生命周期被延长到 `'a`）持有——与原版保存裸指针等价。
    pub fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self {
        let es = core::mem::size_of::<T>();
        let al = core::mem::align_of::<T>();
        let size = Self::byte_len();
        let owned: Vec<u8>;
        let data: &[u8] = match initial {
            Some(d) => {
                // 先两个断言，再上限（O 顺序：`and rax, es-1; jne` → `and r8, 3; jne` → `cmp r9, size+es; jae`）。
                if es > 1 {
                    assert_eq!(d.len() % es, 0);
                }
                if al > 1 {
                    assert_eq!((d.as_ptr() as usize) % al, 0);
                }
                if d.len() >= size + es {
                    panic!(
                        "Excessive data size supplied for buffer {}",
                        core::any::type_name::<T>()
                    );
                }
                d
            }
            None => {
                owned = match T::default_value() {
                    Some(v) => (0..T::BUFFER_LEN)
                        .flat_map(|_| slice_as_bytes(core::slice::from_ref(&v)).to_vec())
                        .collect(),
                    None => vec![0u8; size],
                };
                &owned
            }
        };
        let desc = BufferDesc::new(
            size,
            usage::STORAGE_BUFFER | usage::TRANSFER_DST,
            T::MAPPED_MEMORY,
        )
        .alignment(al);
        let buffer = device
            .create_buffer(&desc, T::NAME, Some(data))
            .expect("called `Result::unwrap()` on an `Err` value: create_buffer");
        let mapped = device
            .mapped_slice_mut(&buffer)
            .expect("called `Option::unwrap()` on a `None` value");
        assert!(
            mapped.len() >= desc.size,
            "assertion failed: data.len() >= buffer.buffer.desc.size"
        );
        if es > 1 {
            assert_eq!(mapped.len() % es, 0);
        }
        if al > 1 {
            assert_eq!((mapped.as_ptr() as usize) % al, 0);
        }
        let (ptr, len) = (mapped.as_mut_ptr() as *mut T, mapped.len() / es);
        // SAFETY: 对齐与整除已断言；`T: Copy` 且映射区由设备持有至缓冲销毁（与原版保存的裸指针等价）；字节内容按元素重解释
        // （`T` 非 `Pod` 派生，故不能用 `bytemuck::cast_slice_mut`——离线缓存缺 `bytemuck_derive`）。
        let mapped: &'a mut [T] = unsafe { core::slice::from_raw_parts_mut(ptr, len) };
        Self { mapped, buffer, upload_writer: device.mapped_upload_writer(&buffer), shadow: None, before: Vec::new(), pending: Vec::new() }
    }

    /// `cpu_write`（O：`impl$6::cpu_write<ButterflySsbo/FireflySsbo>` 0x14230edb0 / 0x14230f1e0，各 1057 B，puffin scope 之外就是一次
    /// `copy_from_slice`）：`n = min(data.len(), mapped.len())`，`mapped[..n].copy_from_slice(data)`——
    /// 数据比映射区长时 `n != data.len()`，触发 `copy_from_slice` 的长度不匹配 panic（`len_mismatch_fail`）。
    pub fn cpu_write(&mut self, data: &[T]) {
        let n = data.len().min(self.mapped.len());
        if data.len() != n {
            // Preserve the existing short-mapping failure before any write.
            self.mapped[..n].copy_from_slice(data);
            unreachable!();
        }
        if data.is_empty() { return; }
        self.with_cpu_write_range(0..n, |mapped| mapped.copy_from_slice(data))
            .expect("mapped upload write failed");
    }

    /// Edit the existing mapping in place under its owner's GPU range guard.
    /// Local CPU fixtures/readback adapters constructed with from_mapping keep
    /// their caller-owned synchronization; production upload owners have a writer.
    pub fn with_cpu_write(&mut self, edit: impl FnOnce(&mut [T]))
        -> Result<(), rhapsody_boundary::BackendError> {
        self.with_cpu_write_range(0..self.mapped.len(), edit)
    }

    pub fn with_cpu_write_range(&mut self, range: Range<usize>, edit: impl FnOnce(&mut [T]))
        -> Result<(), rhapsody_boundary::BackendError> {
        let offset = range.start.checked_mul(core::mem::size_of::<T>())
            .ok_or_else(|| rhapsody_boundary::BackendError::invalid_usage("mapped edit offset overflow"))?;
        let mapped = self.mapped.get_mut(range.clone())
            .ok_or_else(|| rhapsody_boundary::BackendError::invalid_usage("mapped edit exceeds its owner"))?;
        let Some(_writer) = &self.upload_writer else {
            edit(mapped);
            return Ok(());
        };
        let _ = offset;
        // Seed once from the mapping: until the first CPU edit it holds only
        // its creation contents (CPU-to-GPU memory is never written by the GPU).
        let shadow = self.shadow.get_or_insert_with(|| self.mapped.to_vec());
        let target = &mut shadow[range.clone()];
        self.before.clear();
        self.before.extend_from_slice(target);
        edit(target);
        // Upload only changed spans (CHUNK-granular, adjacent spans merged).
        const CHUNK: usize = 256;
        let size = core::mem::size_of::<T>();
        let base = range.start * size;
        let new = slice_as_bytes(target);
        let old = slice_as_bytes(&self.before);
        let mut start = None;
        let mut at = 0;
        while at <= new.len() {
            let end = (at + CHUNK).min(new.len());
            let changed = at < new.len() && new[at..end] != old[at..end];
            match (changed, start) {
                (true, None) => start = Some(at),
                (false, Some(from)) => { self.pending.push(base + from..base + at); start = None; }
                _ => {}
            }
            if at == new.len() { break; }
            at = end;
        }
        self.merge_and_flush_pending()
    }

    /// Edit authoritative CPU bytes and append the producer's known changed
    /// byte ranges. Raster already hashes dirty tiles; avoid copying and
    /// comparing its whole staging allocation again.
    /// Ranges are absolute byte offsets into the full mapping and must cover
    /// every write. Append without clearing earlier ranges awaiting retry.
    pub fn with_cpu_write_regions(&mut self, edit: impl FnOnce(&mut [T], &mut Vec<Range<usize>>))
        -> Result<(), rhapsody_boundary::BackendError> {
        if self.upload_writer.is_none() {
            edit(self.mapped, &mut self.pending);
            self.pending.clear();
            return Ok(());
        }
        let shadow = self.shadow.get_or_insert_with(|| self.mapped.to_vec());
        edit(shadow, &mut self.pending);
        self.merge_and_flush_pending()
    }

    fn merge_and_flush_pending(&mut self) -> Result<(), rhapsody_boundary::BackendError> {
        self.pending.sort_unstable_by_key(|range| range.start);
        let mut merged = 0;
        for index in 0..self.pending.len() {
            let range = self.pending[index].clone();
            if merged > 0 && self.pending[merged - 1].end >= range.start {
                self.pending[merged - 1].end = self.pending[merged - 1].end.max(range.end);
            } else { self.pending[merged] = range; merged += 1; }
        }
        self.pending.truncate(merged);
        // Capacity rejection leaves authoritative bytes and dirty spans with
        // this producer. The hand-over retries before extracting any frame.
        match self.flush_pending_uploads() {
            Err(rhapsody_boundary::BackendError::Exhausted(_)) => Ok(()),
            result => result,
        }
    }

    pub fn flush_pending_uploads(&mut self) -> Result<(), rhapsody_boundary::BackendError> {
        let (Some(writer), Some(shadow)) = (&self.upload_writer, &self.shadow) else { return Ok(()); };
        let bytes = slice_as_bytes(shadow);
        let mut accepted = 0;
        let result = (|| {
            for range in &self.pending {
                writer.write(range.start, &bytes[range.clone()])?;
                accepted += 1;
            }
            Ok(())
        })();
        self.pending.drain(..accepted);
        result
    }

    /// The owner's latest CPU contents: the shadow of a writer-backed buffer
    /// (its GPU copy follows asynchronously), otherwise the mapping itself.
    pub fn cpu_contents(&self) -> &[T] {
        self.shadow.as_deref().unwrap_or(self.mapped)
    }

    /// Observe the latest complete GPU contents without stalling simulation.
    /// None means busy; callers retain their previous CPU value in that case.
    pub fn try_cpu_read<R>(&self, read: impl FnOnce(&[T]) -> R)
        -> Result<Option<R>, rhapsody_boundary::BackendError> {
        if let Some(access) = &self.upload_writer {
            let mut read = Some(read);
            let mut value = None;
            let ready = access.try_read(0, core::mem::size_of_val(self.mapped), &mut || {
                value = Some(read.take().expect("one protected mapped read")(self.mapped));
            })?;
            if ready { Ok(value) } else { Ok(None) }
        } else {
            Ok(Some(read(self.mapped)))
        }
    }
}

// Send/Sync derive from the typed borrow and the writer lease; no unsafe
// marker implementation is needed to move this storage between owners.

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use core::cell::RefCell;

    /// 记录调用并维护一块“GPU 内存”的假设备。
    #[derive(Default)]
    pub struct MockDevice {
        pub created: RefCell<Vec<(BufferDesc, String, Option<Vec<u8>>)>>,
        pub mem: RefCell<Vec<u8>>,
        pub writes: RefCell<Vec<(Vec<Range<usize>>, usize, usize)>>,
        pub map: RefCell<Vec<u8>>,
    }

    impl GraphicsDevice for MockDevice {
        fn create_buffer(
            &self,
            desc: &BufferDesc,
            name: &str,
            data: Option<&[u8]>,
        ) -> Result<Buffer, rhapsody_boundary::BackendError> {
            self.created
                .borrow_mut()
                .push((*desc, name.to_string(), data.map(|d| d.to_vec())));
            let mut m = self.mem.borrow_mut();
            *m = vec![0u8; desc.size];
            if let Some(d) = data {
                m[..d.len()].copy_from_slice(d);
            }
            Ok(Buffer {
                id: 7,
                generation: 1,
                _gap10: 0,
                byte_size: desc.size as u64,
                _gap20: 0,
            })
        }
        fn update_buffer_bytes(
            &self,
            _b: &Buffer,
            data: &[u8],
            ranges: &[Range<usize>],
            offset: usize,
        ) -> Result<(), rhapsody_boundary::BackendError> {
            self.writes
                .borrow_mut()
                .push((ranges.to_vec(), offset, data.len()));
            let mut m = self.mem.borrow_mut();
            for r in ranges {
                m[offset + r.start..offset + r.end].copy_from_slice(&data[r.clone()]);
            }
            Ok(())
        }
        fn mapped_slice_mut(&self, b: &Buffer) -> Option<&mut [u8]> {
            let mut m = self.map.borrow_mut();
            if m.is_empty() {
                // u128 对齐的映射页，长度取缓冲大小（O：映射区 ≥ desc.size）。
                let words = (b.byte_size as usize).div_ceil(16).max(1);
                let page: &'static mut [u128] = Box::leak(vec![0u128; words].into_boxed_slice());
                let p = page.as_mut_ptr() as *mut u8;
                let n = b.byte_size as usize;
                return Some(unsafe { core::slice::from_raw_parts_mut(p, n) });
            }
            let p = m.as_mut_ptr();
            let n = m.len();
            Some(unsafe { core::slice::from_raw_parts_mut(p, n) })
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    #[repr(C)]
    pub struct Dyn(pub [f32; 4]);
    impl StorageElement for Dyn {
        const NAME: &'static str = "Dyn";
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    #[repr(C)]
    pub struct One(pub [u32; 3]);
    impl StorageElement for One {
        const NAME: &'static str = "One";
        const BUFFER_LEN: usize = 1;
        fn default_value() -> Option<Self> {
            Some(One([9, 8, 7]))
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    #[repr(C)]
    pub struct Fixed(pub u32);
    impl StorageElement for Fixed {
        const NAME: &'static str = "Fixed";
        const BUFFER_LEN: usize = 8;
    }

    #[test]
    fn dynamic_buffer_sizes_and_usage() {
        let dev = MockDevice::default();
        let data = [Dyn([1.0; 4]), Dyn([2.0; 4]), Dyn([3.0; 4])];
        let b = TypedStorageBuffer::new(&dev, Some(&data));
        let c = dev.created.borrow();
        assert_eq!(c[0].0.size, 3 * 16);
        assert_eq!(c[0].0.usage, 0x22);
        assert_eq!(c[0].0.alignment, 4);
        assert_eq!(c[0].1, "Dyn");
        assert_eq!(b.byte_size(), 48);
    }

    #[test]
    fn byte_creator_uses_memory2_and_byte_len() {
        let dev = MockDevice::default();
        let words = [1u32, 2, 3, 4]; // 16 B，4 字节对齐
        let bytes: &[u8] = slice_as_bytes(&words);
        let b = TypedStorageBuffer::<Dyn>::new_storage_buffer(&dev, Some(bytes));
        let c = dev.created.borrow();
        assert_eq!(
            (c[0].0.size, c[0].0.usage, c[0].0.memory, c[0].0.alignment),
            (16, 0x22, 2, 4)
        );
        assert_eq!(c[0].2.as_deref(), Some(bytes));
        assert_eq!(b.byte_size(), 16);
    }

    #[test]
    #[should_panic]
    fn byte_creator_rejects_partial_element() {
        let dev = MockDevice::default();
        let words = [0u32; 3]; // 12 B 不是 16 B 的整数倍
        TypedStorageBuffer::<Dyn>::new_storage_buffer(&dev, Some(slice_as_bytes(&words)));
    }

    #[test]
    #[should_panic(expected = "has `BUFFER_LEN` of zero")]
    fn byte_creator_without_data_panics() {
        TypedStorageBuffer::<Dyn>::new_storage_buffer(&MockDevice::default(), None);
    }

    #[test]
    #[should_panic(expected = "has `BUFFER_LEN` of zero")]
    fn dynamic_buffer_without_data_panics() {
        let dev = MockDevice::default();
        let _ = TypedStorageBuffer::<Dyn>::new(&dev, None);
    }

    #[test]
    fn single_element_buffer_uses_default_and_uniform_usage() {
        let dev = MockDevice::default();
        let _ = TypedStorageBuffer::<One>::new(&dev, None);
        let c = dev.created.borrow();
        assert_eq!(c[0].0.usage, 0x12);
        assert_eq!(c[0].0.size, 12);
        let bytes = c[0].2.as_ref().unwrap();
        assert_eq!(bytes.len(), 12);
        assert_eq!(&bytes[0..4], &9u32.to_le_bytes());
    }

    #[test]
    #[should_panic(expected = "Excessive data size")]
    fn too_many_elements_panics() {
        let dev = MockDevice::default();
        let _ = TypedStorageBuffer::new(&dev, Some(&[One([0; 3]), One([1; 3])]));
    }

    #[test]
    fn fixed_len_buffer_zero_fills_and_keeps_full_size() {
        let dev = MockDevice::default();
        let _ = TypedStorageBuffer::<Fixed>::new(&dev, None);
        let _ = TypedStorageBuffer::new(&dev, Some(&[Fixed(5), Fixed(6)]));
        let c = dev.created.borrow();
        assert_eq!(c[0].0.size, 32);
        assert_eq!(c[0].2.as_ref().unwrap(), &vec![0u8; 32]);
        // 有初始数据：缓冲仍是 BUFFER_LEN*es，数据只有 2 个元素
        assert_eq!(c[1].0.size, 32);
        assert_eq!(c[1].2.as_ref().unwrap().len(), 8);
    }

    #[test]
    fn ranges_are_scaled_to_bytes_and_empty_is_noop() {
        let dev = MockDevice::default();
        let data = [Dyn([0.0; 4]); 4];
        let b = TypedStorageBuffer::new(&dev, Some(&data));
        b.gpu_write_ranges_nonoverlapping(&dev, &data, &[], 0);
        assert!(dev.writes.borrow().is_empty());
        let src = [Dyn([1.0; 4]), Dyn([2.0; 4]), Dyn([3.0; 4]), Dyn([4.0; 4])];
        b.gpu_write_ranges_nonoverlapping(&dev, &src, &[1..2, 3..4], 0);
        assert_eq!(dev.writes.borrow()[0].0, vec![16..32, 48..64]);
        let m = dev.mem.borrow();
        assert_eq!(&m[16..20], &2.0f32.to_le_bytes());
        assert_eq!(&m[0..4], &0.0f32.to_le_bytes());
        assert_eq!(&m[48..52], &4.0f32.to_le_bytes());
    }

    #[test]
    fn instance_buffer_counts() {
        let dev = MockDevice::default();
        let data = vec![Dyn([1.0; 4]); 5];
        let bytes = slice_as_bytes(&data);
        let ib = InstanceBuffer::<Dyn>::new_storage_buffer(&dev, Some(bytes));
        assert_eq!(ib.instance_count, 5);
        let one = InstanceBuffer::<One>::new_storage_buffer(&dev, None);
        assert_eq!(one.instance_count, 1);
    }

    #[test]
    #[should_panic]
    fn instance_buffer_rejects_partial_element() {
        let dev = MockDevice::default();
        let bytes = vec![0u8; 20];
        let _ = InstanceBuffer::<Dyn>::new_storage_buffer(&dev, Some(&bytes));
    }

    /// u8 元素、`BUFFER_LEN = 0x40000`：`Cobblestone*RasterStagingSsbo` 家族（O：size 0x40000、memory 2、对齐 1）。
    #[derive(Clone, Copy, Debug, PartialEq)]
    #[repr(C)]
    pub struct Staging(pub u8);
    impl StorageElement for Staging {
        const NAME: &'static str = "StagingProbe";
        const BUFFER_LEN: usize = 0x40000;
    }

    /// 4 B 元素、`BUFFER_LEN = 0x20000`、memory 4：`TerrainHeightSsbo`（O：size 0x80000、对齐 4、上限 0x80004 比较）。
    #[derive(Clone, Copy, Debug, PartialEq)]
    #[repr(C)]
    pub struct Height(pub f32);
    impl StorageElement for Height {
        const NAME: &'static str = "HeightProbe";
        const BUFFER_LEN: usize = 0x20000;
        const MAPPED_MEMORY: u8 = MAPPED_MEMORY_READBACK;
    }

    /// 32 B 元素、`BUFFER_LEN = 24`、有缺省元素：`ButterflySsbo`（O：size 0x300、上限 0x320 比较、缺省元素逐个填充）。
    #[derive(Clone, Copy, Debug, PartialEq)]
    #[repr(C)]
    pub struct Fly {
        pub position: [f32; 3],
        pub anim_t: f32,
        pub forward: [f32; 3],
        pub pad: f32,
    }
    impl StorageElement for Fly {
        const NAME: &'static str = "FlyProbe";
        const BUFFER_LEN: usize = 24;
        fn default_value() -> Option<Self> {
            Some(Fly {
                position: [0.0; 3],
                anim_t: 0.0,
                forward: [-0.0, -0.0, -1.0],
                pad: 0.0,
            })
        }
    }

    #[test]
    fn mapped_staging_family_is_0x40000_bytes_align1_memory2() {
        let dev = MockDevice::default();
        let m = MappedStorageBuffer::<Staging>::new_storage_buffer(&dev, Some(&[1, 2, 3]));
        assert_eq!(m.buffer.byte_size as usize, MAPPED_BUFFER_BYTES);
        assert_eq!(
            MappedStorageBuffer::<Staging>::byte_len(),
            MAPPED_BUFFER_BYTES
        );
        let c = dev.created.borrow();
        assert_eq!(
            (c[0].0.usage, c[0].0.memory, c[0].0.alignment, c[0].0.size),
            (0x22, 2, 1, 0x40000)
        );
        assert_eq!(c[0].2.as_ref().unwrap().len(), 3);
        assert_eq!(m.mapped.len(), 0x40000, "映射区元素数 = 字节数 / 1");
    }

    #[test]
    fn mapped_size_is_buffer_len_times_element_size_not_a_constant() {
        let dev = MockDevice::default();
        let m = MappedStorageBuffer::<Height>::new_storage_buffer(&dev, None);
        let c = dev.created.borrow();
        assert_eq!(
            (c[0].0.size, c[0].0.memory, c[0].0.alignment),
            (0x80000, 4, 4)
        );
        assert_eq!(
            c[0].2.as_ref().unwrap().len(),
            0x80000,
            "无数据：整块零作为初始内容"
        );
        assert_eq!(m.mapped.len(), 0x20000, "len 以元素计（`shr rdx, 2`）");
        drop(c);
        let dev = MockDevice::default();
        let fly = MappedStorageBuffer::<Fly>::new_storage_buffer(&dev, None);
        let c = dev.created.borrow();
        assert_eq!(
            (c[0].0.size, c[0].0.alignment, c[0].0.memory),
            (0x300, 4, 2)
        );
        assert_eq!(fly.mapped.len(), 24);
    }

    #[test]
    fn mapped_without_data_fills_the_default_element() {
        let dev = MockDevice::default();
        let _ = MappedStorageBuffer::<Fly>::new_storage_buffer(&dev, None);
        let c = dev.created.borrow();
        let bytes = c[0].2.as_ref().unwrap();
        assert_eq!(bytes.len(), 0x300);
        // 每个 32 B 元素的 forward.z = -1.0f32（`0xbf800000` @ +0x18）。
        for i in 0..24 {
            assert_eq!(
                &bytes[i * 32 + 0x18..i * 32 + 0x1c],
                &(-1.0f32).to_le_bytes()
            );
            assert_eq!(&bytes[i * 32..i * 32 + 4], &0f32.to_le_bytes());
        }
    }

    #[test]
    fn mapped_data_limit_is_buffer_len_elements_and_partial_elements_assert() {
        let dev = MockDevice::default();
        let full = vec![0u32; 0x20000];
        let _ok =
            MappedStorageBuffer::<Height>::new_storage_buffer(&dev, Some(slice_as_bytes(&full)));
        let over = vec![0u32; 0x20001];
        let r = std::panic::catch_unwind(|| {
            MappedStorageBuffer::<Height>::new_storage_buffer(
                &MockDevice::default(),
                Some(slice_as_bytes(&over)),
            )
        });
        let e = r.err().unwrap();
        let msg = e.downcast_ref::<String>().cloned().unwrap_or_default();
        assert!(
            msg.starts_with("Excessive data size supplied for buffer "),
            "{msg}"
        );
        let r = std::panic::catch_unwind(|| {
            MappedStorageBuffer::<Height>::new_storage_buffer(
                &MockDevice::default(),
                Some(&[0u8; 6]),
            )
        });
        assert!(r.is_err(), "6 B 不是 4 B 元素的整数倍");
    }

    #[test]
    fn rejected_mapped_upload_keeps_dirty_cpu_bytes_for_handover_retry() {
        use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
        struct Writer { busy: AtomicBool, bytes: parking_lot::Mutex<Vec<u8>> }
        impl rhapsody_boundary::MappedUploadWriter for Writer {
            fn try_read(&self, _: usize, _: usize, _: &mut dyn FnMut()) -> Result<bool, rhapsody_boundary::BackendError> {
                unreachable!("upload-only fixture cannot be used as a GPU readback source")
            }
            fn write(&self, offset: usize, data: &[u8]) -> Result<(), rhapsody_boundary::BackendError> {
                if self.busy.load(Ordering::Acquire) { return Err(rhapsody_boundary::BackendError::exhausted("full")); }
                self.bytes.lock()[offset..offset + data.len()].copy_from_slice(data);
                Ok(())
            }
        }
        let mut initial = [Height(0.0); 128];
        let writer = Arc::new(Writer { busy: AtomicBool::new(true), bytes: parking_lot::Mutex::new(vec![0; 512]) });
        let mut storage = MappedStorageBuffer::from_mapping_with_access(&mut initial,
            Buffer { id: 1, generation: 0, byte_size: 512, _gap10: 0, _gap20: 0 }, Some(writer.clone()));
        storage.with_cpu_write(|values| values[0] = Height(17.0)).unwrap();
        assert!(matches!(storage.flush_pending_uploads(), Err(rhapsody_boundary::BackendError::Exhausted(_))));
        assert_eq!(storage.cpu_contents()[0].0, 17.0);
        assert_eq!(storage.mapped[0].0, 0.0, "CPU editing never races the live GPU mapping");
        // A second edit must not erase the first rejected dirty span.
        storage.with_cpu_write(|values| values[80] = Height(29.0)).unwrap();
        writer.busy.store(false, Ordering::Release);
        storage.flush_pending_uploads().unwrap();
        let bytes = writer.bytes.lock();
        assert_eq!(f32::from_ne_bytes(bytes[0..4].try_into().unwrap()), 17.0);
        assert_eq!(f32::from_ne_bytes(bytes[320..324].try_into().unwrap()), 29.0);
        assert!(storage.pending.is_empty());
    }

    #[test]
    fn producer_ranges_preserve_rejected_spans_without_copying_or_uploading_holes() {
        use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
        struct Writer { busy: AtomicBool, writes: parking_lot::Mutex<Vec<(usize, Vec<u8>)>> }
        impl rhapsody_boundary::MappedUploadWriter for Writer {
            fn try_read(&self, _: usize, _: usize, _: &mut dyn FnMut()) -> Result<bool, rhapsody_boundary::BackendError> {
                unreachable!("upload-only fixture cannot be used as a GPU readback source")
            }
            fn write(&self, offset: usize, data: &[u8]) -> Result<(), rhapsody_boundary::BackendError> {
                if self.busy.load(Ordering::Acquire) { return Err(rhapsody_boundary::BackendError::exhausted("full")); }
                self.writes.lock().push((offset, data.to_vec()));
                Ok(())
            }
        }
        let mut initial = [Height(0.0); 128];
        let writer = Arc::new(Writer { busy: AtomicBool::new(true), writes: Default::default() });
        let mut storage = MappedStorageBuffer::from_mapping_with_access(&mut initial,
            Buffer { id: 1, generation: 0, byte_size: 512, _gap10: 0, _gap20: 0 }, Some(writer.clone()));
        storage.with_cpu_write_regions(|values, dirty| {
            values[0] = Height(17.0); dirty.push(0..4);
        }).unwrap();
        storage.with_cpu_write_regions(|values, dirty| {
            values[80] = Height(29.0); dirty.push(320..324);
        }).unwrap();
        assert!(storage.before.is_empty());
        assert_eq!(storage.mapped[0].0, 0.0);
        assert_eq!(storage.pending, [0..4, 320..324]);
        writer.busy.store(false, Ordering::Release);
        storage.flush_pending_uploads().unwrap();
        assert_eq!(*writer.writes.lock(), [(0, 17.0f32.to_ne_bytes().to_vec()),
            (320, 29.0f32.to_ne_bytes().to_vec())]);
        assert!(storage.pending.is_empty());
    }

    #[test]
    fn cpu_write_copies_prefix_and_rejects_longer_slices() {
        let dev = MockDevice::default();
        let mut m = MappedStorageBuffer::<Fly>::new_storage_buffer(&dev, None);
        let one = Fly {
            position: [1.0, 2.0, 3.0],
            anim_t: 4.0,
            forward: [0.0; 3],
            pad: 0.0,
        };
        m.cpu_write(&[one, one]);
        assert_eq!(m.mapped[0], one);
        assert_eq!(m.mapped[1], one);
        assert_eq!(
            m.mapped[2].forward[2], 0.0,
            "第 3 个元素未被写，保持映射区原值（零页）"
        );
        m.cpu_write(&[]);
        let too_long = vec![one; 25];
        let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| m.cpu_write(&too_long)));
        assert!(r.is_err(), "比映射区长 → copy_from_slice 长度不匹配");
    }

    #[test]
    fn mapped_without_device_mapping_unwraps_none() {
        struct NoMap(MockDevice);
        impl GraphicsDevice for NoMap {
            fn create_buffer(
                &self,
                d: &BufferDesc,
                n: &str,
                data: Option<&[u8]>,
            ) -> Result<Buffer, rhapsody_boundary::BackendError> {
                self.0.create_buffer(d, n, data)
            }
            fn update_buffer_bytes(
                &self,
                b: &Buffer,
                d: &[u8],
                r: &[Range<usize>],
                o: usize,
            ) -> Result<(), rhapsody_boundary::BackendError> {
                self.0.update_buffer_bytes(b, d, r, o)
            }
        }
        let r = std::panic::catch_unwind(|| {
            MappedStorageBuffer::<Height>::new_storage_buffer(&NoMap(MockDevice::default()), None)
        });
        let msg = r
            .err()
            .unwrap()
            .downcast_ref::<String>()
            .cloned()
            .unwrap_or_default();
        assert!(
            msg.contains("called `Option::unwrap()` on a `None` value"),
            "{msg}"
        );
    }

    #[test]
    fn instance_buffer_reports_its_count_through_the_trait() {
        let dev = MockDevice::default();
        let ib = InstanceBuffer::<Dyn>::new_storage_buffer(
            &dev,
            Some(slice_as_bytes(&[Dyn([0.0; 4]); 3])),
        );
        assert_eq!(InstanceCounted::instance_count(&ib), 3);
    }
}

#[cfg(test)]
mod contract_device_tests {
    use super::rhapsody_boundary::{BufferDesc, usage};
    use super::*;

    #[test]
    fn contract_device_is_a_ssbo_device() {
        let dev = crate::render::test_device::TestDevice::default();
        let d: &dyn GraphicsDevice = &dev;
        let b = d
            .create_buffer(&BufferDesc::new(64, usage::STORAGE_BUFFER, 1), "t", None)
            .unwrap();
        assert_eq!(b.byte_size, 64);
    }
}
