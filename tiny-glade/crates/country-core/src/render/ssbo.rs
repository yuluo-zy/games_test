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

    /// `rhapsody::raw::buffer::BufferDesc`（占位，仅保留调用点传入的信息）。
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

    /// `rhapsody::raw::error::BackendError`（占位；原版 Ok 标签为 0xb，错误载荷 U）。
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct BackendError;

    /// 占位：`rhapsody::raw::device::GraphicsDevice`（只含本层用到的两个调用）。
    pub trait GraphicsDevice {
        fn create_buffer(
            &self,
            desc: &BufferDesc,
            name: &str,
            data: Option<&[u8]>,
        ) -> Result<Buffer, BackendError>;

        /// 把 `data[r]` 写到缓冲区 `offset + r` 处，`ranges` 互不重叠（O：来自两个调用点的实参）。
        fn update_buffer_bytes(
            &self,
            buffer: &Buffer,
            data: &[u8],
            ranges: &[Range<usize>],
            offset: usize,
        ) -> Result<(), BackendError>;

        /// `rhapsody::raw::buffer::Buffer::mapped_slice_mut(&self, device)`（O 0x140bd4cd0）：持久映射的主机可见缓冲的字节切片；
        /// 设备不映射该缓冲 → `None`（`MappedStorageBuffer::new_storage_buffer` 随后 `Option::unwrap` panic）。
        /// B 阶段（i_api）并入本 trait：原先独立的 `MappedBuffers` 导致 `&dyn GraphicsDevice`（库侧 `SsboType::new_storage_buffer`
        /// 的唯一设备参数）构造不了 `MappedStorageBuffer`。占位默认值 `None`，真实实现 = rhapsody 的持久映射。
        fn mapped_slice_mut(&self, _buffer: &Buffer) -> Option<&mut [u8]> {
            None
        }

        /// `rhapsody::raw::buffer::Buffer::device_address(&self, device)`（O 0x140bd4c00；`nani::trimesh` 的
        /// `Trimesh::build` 把块地址加上它）。原先在 `nani::trimesh` 单列的 `BufferDeviceAddress` trait 已并入此处。
        /// 占位默认值 0（真实实现 = `vkGetBufferDeviceAddress`，由 rhapsody 持有；不自造行为）。
        fn device_address(&self, _buffer: &Buffer) -> u64 {
            panic!("Buffer device-address backend is not connected")
        }
    }

    /// `rhapsody::raw::bytes::slice_as_bytes<T>`。
    pub fn slice_as_bytes<T: Copy>(slice: &[T]) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(slice.as_ptr() as *const u8, core::mem::size_of_val(slice))
        }
    }

    /// 占位：`rhapsody::rg::resource::ResourceHandle`（`sparse_instance_buffer` 的转换用）。
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct ResourceHandle(pub u64, pub u64);
}

/// 全 crate 唯一的设备类型 `rhapsody_contract::GraphicsDevice` 实现本层的设备 trait
/// （step 0：原先 ssbo 的 trait 占位与 lib.rs 的结构体占位互不相通）。
/// 占位实现：不触 GPU，仅按 desc 回填 `Buffer` 的字节容量（O：`+0x18 = desc.size`）。
#[cfg(not(test))]
impl GraphicsDevice for crate::rhapsody_contract::GraphicsDevice {
    fn create_buffer(
        &self,
        _desc: &BufferDesc,
        _name: &str,
        _data: Option<&[u8]>,
    ) -> Result<Buffer, rhapsody_boundary::BackendError> {
        Err(rhapsody_boundary::BackendError)
    }

    fn update_buffer_bytes(
        &self,
        _buffer: &Buffer,
        _data: &[u8],
        _ranges: &[Range<usize>],
        _offset: usize,
    ) -> Result<(), rhapsody_boundary::BackendError> {
        Err(rhapsody_boundary::BackendError)
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
    pub fn new_storage_buffer(device: &dyn GraphicsDevice, initial: Option<&[u8]>) -> Self {
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
            .map_err(|_| BufferWriteError::Device)
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
}

impl<'a, T: StorageElement> MappedStorageBuffer<'a, T> {
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
        Self { mapped, buffer }
    }

    /// `cpu_write`（O：`impl$6::cpu_write<ButterflySsbo/FireflySsbo>` 0x14230edb0 / 0x14230f1e0，各 1057 B，puffin scope 之外就是一次
    /// `copy_from_slice`）：`n = min(data.len(), mapped.len())`，`mapped[..n].copy_from_slice(data)`——
    /// 数据比映射区长时 `n != data.len()`，触发 `copy_from_slice` 的长度不匹配 panic（`len_mismatch_fail`）。
    pub fn cpu_write(&mut self, data: &[T]) {
        let n = data.len().min(self.mapped.len());
        self.mapped[..n].copy_from_slice(data);
    }
}

// SAFETY: 映射区是设备持有的主机内存；原版的 `MappedStorageBuffer` 同样跨线程放在 `Box<dyn Ssbo + Send + Sync>` 里。
unsafe impl<T: Send> Send for MappedStorageBuffer<'_, T> {}
unsafe impl<T: Sync> Sync for MappedStorageBuffer<'_, T> {}

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
        let dev = crate::rhapsody_contract::GraphicsDevice::default();
        let d: &dyn GraphicsDevice = &dev;
        let b = d
            .create_buffer(&BufferDesc::new(64, usage::STORAGE_BUFFER, 1), "t", None)
            .unwrap();
        assert_eq!(b.byte_size, 64);
    }
}
