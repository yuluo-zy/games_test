//! WP2 GPU buffer primitives for `country_core::render` (original `crates/country-core/src/render/`).
//!
//! This is a contract-aligned Rust reconstruction of the B build (EXE cf048e27…, PDB 2d9686b8…),
//! written from decompilation + PDB skeleton evidence. It is NOT recovered author source.
//! Every field is tagged O (observed) / I (inferred) / U (unknown typed gap).
//!
//! Reuse boundary: `rhapsody` (reference `reconstruction/rhapsody-rebuild`), `bumpalo`,
//! `puffin`, `parking_lot`, `core::sync::atomic` are REUSED deps — we only call them, never
//! reimplement. Unknown fields are typed gaps (`_gapN`), never guessed.

#![allow(clippy::needless_doctest_main)]

/// Minimal REUSED-rhapsody surface WP2 depends on. Maps to `rhapsody::raw::{buffer,device,bytes}`.
/// See `reconstruction/rhapsody-rebuild` for the real types.
pub mod rhapsody_boundary {

    /// rhapsody raw GPU buffer handle (16 bytes: resource id + device/generation).
    #[repr(C)]
    #[derive(Clone, Copy, Debug)]
    pub struct Buffer {
        pub lo: u64,
        pub hi: u64,
    }

    /// `rhapsody::raw::buffer::BufferDesc` (construction params).
    #[derive(Clone, Copy, Debug, Default)]
    pub struct BufferDesc {
        pub byte_size: usize,
        // alignment/usage/... follow; only what WP2 passes is defined here.
        pub gap1: u64, // U
    }

    impl BufferDesc {
        pub fn new(byte_size: usize) -> Self {
            Self { byte_size, gap1: 0 }
        }
        /// rhapsody alignment helper (O: `BufferDesc::alignment` called by constructors).
        pub fn alignment(&mut self) -> &mut Self {
            self
        }
        /// `BufferDesc::new_gpu_only` (O: used by `vao::build_vao`).
        pub fn new_gpu_only(byte_size: usize) -> Self {
            Self::new(byte_size)
        }
    }

    /// Result of `GraphicsDevice::create_buffer` / `update_buffer_bytes` / `mapped_slice_mut`.
    /// WP2 only inspects the Ok tag.
    #[derive(Clone, Copy, Debug)]
    pub enum GpuResult<T> {
        Ok(T),
        Err, // U: error payload is rhapsody-owned
    }

    /// REUSED rhapsody `GraphicsDevice` (only the calls WP2 makes).
    pub trait GraphicsDevice {
        /// `create_buffer<Option<&[u8]>>` — returns buffer + (byte_size for the mapped variant).
        fn create_buffer(&self, desc: &BufferDesc) -> GpuResult<(Buffer, usize)>;
        /// `update_buffer_bytes(buffer, &byte_ranges, data)` — one GPU write of non-overlapping ranges.
        fn update_buffer_bytes(
            &self,
            buffer: &Buffer,
            ranges: &[(usize, usize)],
            data: &[u8],
        ) -> GpuResult<()>;
    }

    /// `rhapsody::raw::bytes::slice_as_bytes<T>` (REUSED).
    pub fn slice_as_bytes<T>(slice: &[T]) -> &[u8] {
        unsafe { core::slice::from_raw_parts(slice.as_ptr() as *const u8, core::mem::size_of_val(slice)) }
    }

    /// `Buffer::mapped_slice_mut` — persistent-mapped view (REUSED). Returns ptr + byte len.
    pub fn mapped_slice_mut(_buffer: &Buffer) -> GpuResult<(*mut u8, usize)> {
        // Real impl lives in rhapsody; WP2 maps it through `GpuResult`.
        GpuResult::Err
    }

    /// REUSED rhapsody capability: CPU (persistent-mapped) view of a buffer.
    /// Used by `SparseInstanceBuffer::gpu_flush` as the upload data source (I).
    pub fn buffer_cpu_ptr(_buffer: &Buffer) -> *mut u8 {
        core::ptr::null_mut()
    }

    /// `rhapsody::rg::resource::ResourceHandle` (REUSED).
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct ResourceHandle(pub u64, pub u64);
}

use core::ops::Range;
use rhapsody_boundary::{Buffer, BufferDesc, GraphicsDevice};

// ===========================================================================
// ssbo.rs
// ===========================================================================

/// Error type returned by typed GPU writes. Enum; exact variants not recovered.
/// (O: `drop_in_place<BufferWriteError>` exists; Debug for `Result<(),BufferWriteError>`
/// prints an Ok-like unit variant and an Err-like 1-field tuple variant.)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BufferWriteError {
    /// U: unit variant (exact name not recovered).
    OutOfBounds,
    /// U: 1-field tuple variant (exact payload not recovered).
    Device,
}

/// Typed storage buffer: a rhapsody GPU buffer stamped with a T element layout.
/// 40 bytes (5 qwords). See `contracts/ssbo.md`.
#[repr(C)]
#[derive(Clone, Debug)]
pub struct TypedStorageBuffer<T> {
    /// O — rhapsody raw buffer handle.
    pub buffer: Buffer,
    /// O — element count.
    pub length: usize,
    /// O — GPU buffer byte capacity (guard in `gpu_flush` uses this).
    pub byte_size: usize,
    /// U — typed gap (not referenced by any decompiled WP2 consumer).
    pub _gap_field5: u64,
    /// Marker so `T` participates in the type.
    pub _marker: core::marker::PhantomData<T>,
}

impl<T> TypedStorageBuffer<T> {
    /// O (`TypedStorageBuffer::new` 0x141618e00): panics when the byte size is 0.
    /// Arg convention (count vs byte size; which arg is the device) is partially I.
    pub fn new(count: usize, byte_size: usize, device: &dyn GraphicsDevice) -> Self {
        assert!(count != 0, "TypedStorageBuffer::new: zero size");
        let mut desc = BufferDesc::new(byte_size);
        desc.alignment();
        let buf = match device.create_buffer(&desc) {
            rhapsody_boundary::GpuResult::Ok((b, _)) => b,
            rhapsody_boundary::GpuResult::Err => panic!("create_buffer failed"),
        };
        Self {
            buffer: buf,
            length: count,
            byte_size,
            _gap_field5: 0,
            _marker: core::marker::PhantomData,
        }
    }

    /// U — inlined, not decompiled. Name implies an arbitrary byte-layout constructor.
    pub fn new_arbitrary(_byte_size: usize, _device: &dyn GraphicsDevice) -> Self {
        unreachable!("new_arbitrary: typed gap — not recovered")
    }

    /// I — inline, delegates to `gpu_write_with_offset(data, 0)`.
    pub fn gpu_write(&mut self, data: &[T], device: &dyn GraphicsDevice) -> Result<(), BufferWriteError> {
        self.gpu_write_with_offset(data, 0, device)
    }

    /// I — inline (PDB line 208): bounds-check then one `update_buffer_bytes`.
    pub fn gpu_write_with_offset(
        &mut self,
        data: &[T],
        offset: usize,
        device: &dyn GraphicsDevice,
    ) -> Result<(), BufferWriteError> {
        let es = core::mem::size_of::<T>();
        let byte_len = data.len().checked_mul(es).unwrap_or(usize::MAX);
        if offset.checked_add(byte_len).is_none() || offset + byte_len > self.byte_size {
            return Err(BufferWriteError::OutOfBounds);
        }
        let ranges = [(offset, offset + byte_len)];
        match device.update_buffer_bytes(&self.buffer, &ranges, rhapsody_boundary::slice_as_bytes(data)) {
            rhapsody_boundary::GpuResult::Ok(()) => Ok(()),
            rhapsody_boundary::GpuResult::Err => Err(BufferWriteError::Device),
        }
    }

    /// O (`gpu_write_ranges_nonoverlapping` 0x14161ab00 / 0x141fd3700):
    /// element-indexed non-overlapping ranges are byte-scaled by `size_of::<T>()`
    /// into a bumpalo arena and uploaded in one `update_buffer_bytes`. No-op when empty.
    pub fn gpu_write_ranges_nonoverlapping(
        &mut self,
        ranges: &[Range<usize>],
        data: &[T],
        device: &dyn GraphicsDevice,
    ) -> Result<(), BufferWriteError> {
        if ranges.is_empty() {
            return Ok(());
        }
        let es = core::mem::size_of::<T>();
        // bumpalo::Bump is used for the byte-range array (O); replaced here by a Vec
        // since bumpalo is a REUSED allocation detail, not game logic.
        let mut byte_ranges: alloc::vec::Vec<(usize, usize)> = alloc::vec::Vec::with_capacity(ranges.len());
        for r in ranges {
            byte_ranges.push((r.start * es, r.end * es));
        }
        match device.update_buffer_bytes(&self.buffer, &byte_ranges, rhapsody_boundary::slice_as_bytes(data)) {
            rhapsody_boundary::GpuResult::Ok(()) => Ok(()),
            rhapsody_boundary::GpuResult::Err => Err(BufferWriteError::Device),
        }
    }

    /// U — single-element write; not decompiled.
    pub fn gpu_write_element(&mut self, _index: usize, _value: &T, _device: &dyn GraphicsDevice) -> Result<(), BufferWriteError> {
        unreachable!("gpu_write_element: typed gap — not recovered")
    }
}

impl<T> core::ops::Deref for TypedStorageBuffer<T> {
    type Target = Buffer;
    /// O — `impl$2::deref` exists; Target=Buffer is I (derefs to the raw buffer handle is the
    /// only shape consistent with the ssbo.rs wrapper types).
    fn deref(&self) -> &Buffer {
        &self.buffer
    }
}

/// `InstanceBuffer<T>` = `{ instance_count: usize, buffer: TypedStorageBuffer<T> }` (48 bytes).
/// O (`impl$7::new_storage_buffer` 0x1416207e0).
#[repr(C)]
#[derive(Clone, Debug)]
pub struct InstanceBuffer<T> {
    /// O — element count kept by the CPU side.
    pub instance_count: usize,
    /// O — embedded typed buffer.
    pub buffer: TypedStorageBuffer<T>,
}

impl<T: Default + Copy> InstanceBuffer<T> {
    /// O — `impl$7::new_storage_buffer<T>` 0x1416207e0.
    /// count==0: seed one slot from `T::default_value()` if it is Some, else an empty buffer.
    /// Else assert `byte_size % size_of::<T>() == 0` and `count % 4 == 0`; count = byte_size/es.
    pub fn new_storage_buffer(count: usize, byte_size: usize, device: &dyn GraphicsDevice) -> Self {
        if count == 0 {
            // default_value() is `Option<T>`-like (tag 1 = Some); only its byte copy is observed.
            // The default struct is `NaniDrawListOffsetDefaults`-sized (0x1b0).
            let default = T::default();
            let es = core::mem::size_of::<T>();
            let _ = core::mem::size_of_val(&default);
            let buf = TypedStorageBuffer::new(1, es, device);
            return Self { instance_count: 1, buffer: buf };
        }
        let es = core::mem::size_of::<T>();
        assert!(byte_size % es == 0, "InstanceBuffer::new_storage_buffer: byte_size % size_of::<T>() != 0");
        assert!(count % 4 == 0, "InstanceBuffer::new_storage_buffer: count % 4 != 0");
        let buf = TypedStorageBuffer::new(count, byte_size, device);
        Self { instance_count: byte_size / es, buffer: buf }
    }

    /// I — inlined.
    pub fn set_instance_count(&mut self, count: usize) {
        self.instance_count = count;
    }
    /// I — inlined.
    pub fn instance_count(&self) -> usize {
        self.instance_count
    }
    /// I — inlined.
    pub fn gpu_write(&mut self, data: &[T], device: &dyn GraphicsDevice) -> Result<(), BufferWriteError> {
        self.buffer.gpu_write(data, device)
    }
    /// I — inlined; subset write form.
    pub fn gpu_write_subset(
        &mut self,
        _offset: usize,
        _data: &[T],
        _device: &dyn GraphicsDevice,
    ) -> Result<(), BufferWriteError> {
        unreachable!("gpu_write_subset: typed gap — not recovered")
    }
}

/// `MappedStorageBuffer<T>` = `{ mapped_ptr, length, buffer, +3 }` (56 bytes).
/// O (`impl$4::new_storage_buffer` 0x14161ad70; `impl$6::cpu_write` 0x14230edb0).
#[repr(C)]
#[derive(Debug)]
pub struct MappedStorageBuffer<T> {
    /// O — pointer into persistent-mapped GPU memory.
    pub mapped_ptr: *mut u8,
    /// O — element count.
    pub length: usize,
    /// O — rhapsody buffer handle.
    pub buffer: Buffer,
    /// U — typed gaps (not referenced by decompiled WP2 consumers).
    pub _gap4: u64,
    pub _gap5: u64,
    pub _gap6: u64,
    /// Marker for T (element size is used by `cpu_write`).
    pub _marker: core::marker::PhantomData<T>,
}

impl<T> MappedStorageBuffer<T> {
    /// O — `impl$4::new_storage_buffer` 0x14161ad70.
    /// `initial: None` → 0x40000-byte zeroed CPU staging; `Some` → byte_len must be ≤ 0x40000.
    /// Creates + maps the GPU buffer, copies initial data into the mapping.
    pub fn new_storage_buffer(
        initial: Option<&[T]>,
        device: &dyn GraphicsDevice,
    ) -> Self {
        const MAX_BYTES: usize = 0x40000;
        let byte_len = match initial {
            None => 0,
            Some(s) => {
                let n = core::mem::size_of_val(s);
                assert!(n <= MAX_BYTES, "MappedStorageBuffer: initial data > 0x40000");
                n
            }
        };
        let es = core::mem::size_of::<T>();
        let count = if byte_len == 0 { MAX_BYTES / es } else { byte_len / es };
        let mut desc = BufferDesc::new(if byte_len == 0 { MAX_BYTES } else { byte_len });
        desc.alignment();
        let buf = match device.create_buffer(&desc) {
            rhapsody_boundary::GpuResult::Ok((b, _)) => b,
            rhapsody_boundary::GpuResult::Err => panic!("create_buffer failed"),
        };
        let (mapped, mapped_len) = match rhapsody_boundary::mapped_slice_mut(&buf) {
            rhapsody_boundary::GpuResult::Ok(m) => m,
            rhapsody_boundary::GpuResult::Err => panic!("mapped_slice_mut failed"),
        };
        assert!(mapped_len / es >= count, "MappedStorageBuffer: mapped length too small");
        match initial {
            Some(s) => unsafe {
                core::ptr::copy_nonoverlapping(s.as_ptr() as *const u8, mapped, core::mem::size_of_val(s));
            },
            // I — None path creates a zero-initialized 256 KiB mapped buffer (O: 0x40000 zeros).
            None => unsafe {
                core::ptr::write_bytes(mapped, 0, mapped_len);
            },
        }
        Self {
            mapped_ptr: mapped,
            length: count,
            buffer: buf,
            _gap4: 0,
            _gap5: 0,
            _gap6: 0,
            _marker: core::marker::PhantomData,
        }
    }

    /// I — inlined accessor.
    pub fn mapped_data(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self.mapped_ptr as *const T, self.length) }
    }
    /// I — inlined accessor.
    pub fn mapped_data_mut(&mut self) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(self.mapped_ptr as *mut T, self.length) }
    }

    /// O — `impl$6::cpu_write` 0x14230edb0: copy `data` into the mapping.
    /// Panics (`len_mismatch_fail`) when `data.len() > self.length`.
    pub fn cpu_write(&self, data: &[T]) {
        assert!(data.len() <= self.length, "cpu_write: len mismatch");
        let es = core::mem::size_of::<T>();
        unsafe {
            core::ptr::copy_nonoverlapping(data.as_ptr() as *const u8, self.mapped_ptr, data.len() * es);
        }
    }
}
