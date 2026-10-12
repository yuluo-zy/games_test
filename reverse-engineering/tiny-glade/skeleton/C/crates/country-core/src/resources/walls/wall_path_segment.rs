// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/resources/walls/wall_path_segment.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 432cb40ac5eb3bbe1ee6324dac7444f02de266d33035b2446dcf55f0b8c04efb
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: wall_path_segment.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 394, 528
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use glam; // namespace candidate
//   package identities: [{"name":"glam","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.30.10"}]
// use itertools; // namespace candidate
//   package identities: [{"name":"itertools","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_path_segment::impl$2::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 27
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x65F3
// 首条位置: {"module":394,"inlinee":26099}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":17840,"parent_record_offset":17764,"ancestor_record_offset":17764,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":394,"record_offset":19884,"parent_record_offset":17764,"ancestor_record_offset":17764,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_direct] utils::loop_range::impl$15::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/loop_range.rs
//     evidence={"module":394,"record_offset":19856,"parent_record_offset":17764,"ancestor_record_offset":17764,"inlinee":26101,"source_line":49,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":394,"record_offset":18284,"parent_record_offset":18260,"ancestor_record_offset":17764,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":394,"record_offset":18308,"parent_record_offset":18284,"ancestor_record_offset":17764,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":394,"record_offset":20084,"parent_record_offset":20056,"ancestor_record_offset":17764,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":394,"record_offset":20112,"parent_record_offset":20084,"ancestor_record_offset":17764,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":394,"record_offset":18332,"parent_record_offset":18308,"ancestor_record_offset":17764,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":394,"record_offset":20140,"parent_record_offset":20112,"ancestor_record_offset":17764,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":394,"record_offset":18260,"parent_record_offset":18080,"ancestor_record_offset":17764,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":394,"record_offset":20056,"parent_record_offset":19960,"ancestor_record_offset":17764,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":394,"record_offset":18020,"parent_record_offset":17992,"ancestor_record_offset":17764,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":394,"record_offset":19988,"parent_record_offset":19960,"ancestor_record_offset":17764,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":394,"record_offset":19960,"parent_record_offset":19932,"ancestor_record_offset":17764,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":394,"record_offset":18080,"parent_record_offset":18048,"ancestor_record_offset":17764,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":394,"record_offset":18048,"parent_record_offset":18020,"ancestor_record_offset":17764,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":394,"record_offset":19932,"parent_record_offset":19908,"ancestor_record_offset":17764,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":394,"record_offset":18116,"parent_record_offset":18080,"ancestor_record_offset":17764,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":394,"record_offset":19160,"parent_record_offset":19092,"ancestor_record_offset":17764,"inlinee":8468,"source_line":410,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":394,"record_offset":17948,"parent_record_offset":17876,"ancestor_record_offset":17764,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":17920,"parent_record_offset":17876,"ancestor_record_offset":17764,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":17992,"parent_record_offset":17948,"ancestor_record_offset":17764,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":394,"record_offset":17876,"parent_record_offset":17840,"ancestor_record_offset":17764,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":394,"record_offset":18140,"parent_record_offset":18116,"ancestor_record_offset":17764,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":394,"record_offset":18220,"parent_record_offset":18164,"ancestor_record_offset":17764,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":394,"record_offset":19660,"parent_record_offset":19608,"ancestor_record_offset":17764,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::take::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\take.rs
//     evidence={"module":394,"record_offset":19608,"parent_record_offset":19160,"ancestor_record_offset":17764,"inlinee":8469,"source_line":36,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":394,"record_offset":19812,"parent_record_offset":19160,"ancestor_record_offset":17764,"inlinee":4509,"source_line":563,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":394,"record_offset":18164,"parent_record_offset":18140,"ancestor_record_offset":17764,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":394,"record_offset":18192,"parent_record_offset":18164,"ancestor_record_offset":17764,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":394,"record_offset":20020,"parent_record_offset":19988,"ancestor_record_offset":17764,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":394,"record_offset":18388,"parent_record_offset":17948,"ancestor_record_offset":17764,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":394,"record_offset":18412,"parent_record_offset":18388,"ancestor_record_offset":17764,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":394,"record_offset":19908,"parent_record_offset":19884,"ancestor_record_offset":17764,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":394,"record_offset":19772,"parent_record_offset":19704,"ancestor_record_offset":17764,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":394,"record_offset":19744,"parent_record_offset":19704,"ancestor_record_offset":17764,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":394,"record_offset":19704,"parent_record_offset":19660,"ancestor_record_offset":17764,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00394.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_path_segment::WallPathSegment::pt_count
// 记录形式: compiled_procedure; instances=1
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":528,"record_offset":13592,"rva":10509632,"va":5379218752}
// 编译实例 RVAs: 0xa05d40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] utils::loop_range::LoopRange::len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t10482967","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379218752,"raw_hex":"e917f59f00","size":5,"soft_fail":false,"target_va":5389701728,"va":5379218756,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_path_segment::WallPathSegment::length_ws
// 记录形式: compiled_procedure; instances=1
// 源码行: 46
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":528,"record_offset":13744,"rva":10509648,"va":5379218768}
// 编译实例 RVAs: 0xa05d50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec3::Vec3::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":528,"record_offset":14208,"parent_record_offset":13744,"ancestor_record_offset":13744,"inlinee":11749,"source_line":578,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":528,"record_offset":13912,"parent_record_offset":13744,"ancestor_record_offset":13744,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":528,"record_offset":14440,"parent_record_offset":14412,"ancestor_record_offset":13744,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":528,"record_offset":14036,"parent_record_offset":14012,"ancestor_record_offset":13744,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":528,"record_offset":14084,"parent_record_offset":14060,"ancestor_record_offset":13744,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":528,"record_offset":14104,"parent_record_offset":14084,"ancestor_record_offset":13744,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":528,"record_offset":14304,"parent_record_offset":14244,"ancestor_record_offset":13744,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":528,"record_offset":14060,"parent_record_offset":14036,"ancestor_record_offset":13744,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":528,"record_offset":14152,"parent_record_offset":14128,"ancestor_record_offset":13744,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":528,"record_offset":13980,"parent_record_offset":13948,"ancestor_record_offset":13744,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":528,"record_offset":13948,"parent_record_offset":13912,"ancestor_record_offset":13744,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::Vec3::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":528,"record_offset":14360,"parent_record_offset":14244,"ancestor_record_offset":13744,"inlinee":9777,"source_line":248,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":528,"record_offset":14244,"parent_record_offset":14208,"ancestor_record_offset":13744,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":528,"record_offset":14012,"parent_record_offset":13912,"ancestor_record_offset":13744,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":528,"record_offset":14412,"parent_record_offset":14360,"ancestor_record_offset":13744,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_path_segment::WallPathSegment::append_pick_ribbon_triangles
// 记录形式: compiled_procedure; instances=1
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":528,"record_offset":7784,"rva":10506544,"va":5379215664}
// 编译实例 RVAs: 0xa05130
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140df0560
//     owner=unresolved_namespace; count=4
//     target_aliases=2 candidates; complete list: wall_path_segment.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4108288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379215664,"raw_hex":"e800b03e00","size":5,"soft_fail":false,"target_va":5383325024,"va":5379216731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t37183171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379215664,"raw_hex":"e8c35e3702","size":5,"soft_fail":false,"target_va":5416400480,"va":5379217304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t37403120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379215664,"raw_hex":"e8f0b93a02","size":5,"soft_fail":false,"target_va":5416621762,"va":5379218637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::intersect::Triangle::new
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t10987188","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379215664,"raw_hex":"e8b4a6a700","size":5,"soft_fail":false,"target_va":5390204480,"va":5379217287,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":528,"record_offset":10876,"parent_record_offset":7784,"ancestor_record_offset":7784,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_direct] core::array::iter::impl$4::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter.rs
//     evidence={"module":528,"record_offset":7996,"parent_record_offset":7784,"ancestor_record_offset":7784,"inlinee":5119,"source_line":239,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":528,"record_offset":8080,"parent_record_offset":7784,"ancestor_record_offset":7784,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wall_path_segment::impl$0::append_pick_ribbon_triangles::closure$0
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_path_segment.rs
//     evidence={"module":528,"record_offset":9156,"parent_record_offset":7784,"ancestor_record_offset":7784,"inlinee":31529,"source_line":61,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::walls::wall_path_segment::impl$0::append_pick_ribbon_triangles::closure$1
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/walls/wall_path_segment.rs
//     evidence={"module":528,"record_offset":8672,"parent_record_offset":7784,"ancestor_record_offset":7784,"inlinee":31528,"source_line":66,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::Vec3::cross
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":528,"record_offset":12432,"parent_record_offset":7784,"ancestor_record_offset":7784,"inlinee":9830,"source_line":262,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$24::mul
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":528,"record_offset":10412,"parent_record_offset":7784,"ancestor_record_offset":7784,"inlinee":9798,"source_line":1501,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$34::add
//     owner=registry_dependency_definition; count=7
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":528,"record_offset":10348,"parent_record_offset":7784,"ancestor_record_offset":7784,"inlinee":9800,"source_line":1589,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":528,"record_offset":13200,"parent_record_offset":7784,"ancestor_record_offset":7784,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":528,"record_offset":8812,"parent_record_offset":8780,"ancestor_record_offset":7784,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":528,"record_offset":8860,"parent_record_offset":8836,"ancestor_record_offset":7784,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":528,"record_offset":8836,"parent_record_offset":8812,"ancestor_record_offset":7784,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":528,"record_offset":8780,"parent_record_offset":8736,"ancestor_record_offset":7784,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":528,"record_offset":8708,"parent_record_offset":8672,"ancestor_record_offset":7784,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":528,"record_offset":8736,"parent_record_offset":8708,"ancestor_record_offset":7784,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":528,"record_offset":9204,"parent_record_offset":9156,"ancestor_record_offset":7784,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::PolymorphicIter::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":528,"record_offset":8020,"parent_record_offset":7996,"ancestor_record_offset":7784,"inlinee":5120,"source_line":161,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::array::iter::iter_inner::impl$8::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\array\iter\iter_inner.rs
//     evidence={"module":528,"record_offset":11532,"parent_record_offset":11500,"ancestor_record_offset":7784,"inlinee":7208,"source_line":167,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":528,"record_offset":9300,"parent_record_offset":9156,"ancestor_record_offset":7784,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":528,"record_offset":9696,"parent_record_offset":9672,"ancestor_record_offset":7784,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":528,"record_offset":8372,"parent_record_offset":8348,"ancestor_record_offset":7784,"inlinee":7042,"source_line":81,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\chain.rs
//     evidence={"module":528,"record_offset":8420,"parent_record_offset":8396,"ancestor_record_offset":7784,"inlinee":7430,"source_line":82,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":528,"record_offset":8156,"parent_record_offset":8116,"ancestor_record_offset":7784,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":528,"record_offset":8188,"parent_record_offset":8156,"ancestor_record_offset":7784,"inlinee":7231,"source_line":305,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":528,"record_offset":8444,"parent_record_offset":8420,"ancestor_record_offset":7784,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":528,"record_offset":11736,"parent_record_offset":11692,"ancestor_record_offset":7784,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$13::add
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":528,"record_offset":10376,"parent_record_offset":10348,"ancestor_record_offset":7784,"inlinee":8718,"source_line":105,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$141::mul
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":528,"record_offset":10444,"parent_record_offset":10412,"ancestor_record_offset":7784,"inlinee":8714,"source_line":352,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":528,"record_offset":9440,"parent_record_offset":9412,"ancestor_record_offset":7784,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::index_range::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\index_range.rs
//     evidence={"module":528,"record_offset":8044,"parent_record_offset":8020,"ancestor_record_offset":7784,"inlinee":5121,"source_line":130,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":528,"record_offset":8396,"parent_record_offset":8372,"ancestor_record_offset":7784,"inlinee":7428,"source_line":1647,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":528,"record_offset":11616,"parent_record_offset":11556,"ancestor_record_offset":7784,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":528,"record_offset":8252,"parent_record_offset":8228,"ancestor_record_offset":7784,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":528,"record_offset":11640,"parent_record_offset":11616,"ancestor_record_offset":7784,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":528,"record_offset":8900,"parent_record_offset":8736,"ancestor_record_offset":7784,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":528,"record_offset":11556,"parent_record_offset":11532,"ancestor_record_offset":7784,"inlinee":4887,"source_line":639,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":528,"record_offset":9232,"parent_record_offset":9204,"ancestor_record_offset":7784,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":528,"record_offset":11588,"parent_record_offset":11556,"ancestor_record_offset":7784,"inlinee":4183,"source_line":239,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":528,"record_offset":9260,"parent_record_offset":9232,"ancestor_record_offset":7784,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::__iterator_get_unchecked
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":528,"record_offset":8228,"parent_record_offset":8188,"ancestor_record_offset":7784,"inlinee":6630,"source_line":418,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":528,"record_offset":9640,"parent_record_offset":9608,"ancestor_record_offset":7784,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":528,"record_offset":9608,"parent_record_offset":9584,"ancestor_record_offset":7784,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":528,"record_offset":9584,"parent_record_offset":9556,"ancestor_record_offset":7784,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":528,"record_offset":9556,"parent_record_offset":9532,"ancestor_record_offset":7784,"inlinee":9811,"source_line":564,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize_or_zero
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":528,"record_offset":9532,"parent_record_offset":9156,"ancestor_record_offset":7784,"inlinee":9810,"source_line":581,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":528,"record_offset":9504,"parent_record_offset":9156,"ancestor_record_offset":7784,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":528,"record_offset":9412,"parent_record_offset":9156,"ancestor_record_offset":7784,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$31::collect_from_iter_no_buf
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":528,"record_offset":8348,"parent_record_offset":8116,"ancestor_record_offset":7784,"inlinee":7466,"source_line":374,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":528,"record_offset":8116,"parent_record_offset":8080,"ancestor_record_offset":7784,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":528,"record_offset":9672,"parent_record_offset":9640,"ancestor_record_offset":7784,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_path_segment::impl$0::append_pick_ribbon_triangles::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7B29
// 首条位置: {"module":528,"inlinee":31529}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":528,"record_offset":9204,"parent_record_offset":9156,"ancestor_record_offset":9156,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::min
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":528,"record_offset":9300,"parent_record_offset":9156,"ancestor_record_offset":9156,"inlinee":6426,"source_line":1072,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_direct] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":528,"record_offset":11736,"parent_record_offset":11692,"ancestor_record_offset":11692,"inlinee":9345,"source_line":2344,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::normalize_or_zero
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":528,"record_offset":9532,"parent_record_offset":9156,"ancestor_record_offset":9156,"inlinee":9810,"source_line":581,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::perp
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":528,"record_offset":9504,"parent_record_offset":9156,"ancestor_record_offset":9156,"inlinee":9809,"source_line":994,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec3::impl$50::sub
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec3.rs
//     evidence={"module":528,"record_offset":9412,"parent_record_offset":9156,"ancestor_record_offset":9156,"inlinee":9774,"source_line":1729,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":528,"record_offset":9696,"parent_record_offset":9672,"ancestor_record_offset":9156,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":528,"record_offset":9440,"parent_record_offset":9412,"ancestor_record_offset":9156,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":528,"record_offset":9232,"parent_record_offset":9204,"ancestor_record_offset":9156,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":528,"record_offset":9260,"parent_record_offset":9232,"ancestor_record_offset":9156,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":528,"record_offset":9640,"parent_record_offset":9608,"ancestor_record_offset":9156,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":528,"record_offset":9608,"parent_record_offset":9584,"ancestor_record_offset":9156,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::length_recip
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":528,"record_offset":9584,"parent_record_offset":9556,"ancestor_record_offset":9156,"inlinee":9812,"source_line":480,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::normalize_or
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":528,"record_offset":9556,"parent_record_offset":9532,"ancestor_record_offset":9156,"inlinee":9811,"source_line":564,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":528,"record_offset":9672,"parent_record_offset":9640,"ancestor_record_offset":9156,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_path_segment::impl$0::append_pick_ribbon_triangles::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7B28
// 首条位置: {"module":528,"inlinee":31528}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":528,"record_offset":8708,"parent_record_offset":8672,"ancestor_record_offset":8672,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":528,"record_offset":8812,"parent_record_offset":8780,"ancestor_record_offset":8672,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":528,"record_offset":8860,"parent_record_offset":8836,"ancestor_record_offset":8672,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":528,"record_offset":8836,"parent_record_offset":8812,"ancestor_record_offset":8672,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":528,"record_offset":8780,"parent_record_offset":8736,"ancestor_record_offset":8672,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":528,"record_offset":8736,"parent_record_offset":8708,"ancestor_record_offset":8672,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":528,"record_offset":8900,"parent_record_offset":8736,"ancestor_record_offset":8672,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::walls::wall_path_segment::WallPathSegment::length_ws_2d
// 记录形式: compiled_procedure; instances=1
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":528,"record_offset":7124,"rva":10506256,"va":5379215376}
// 编译实例 RVAs: 0xa05010
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] glam::f32::vec2::Vec2::distance
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":528,"record_offset":7432,"parent_record_offset":7124,"ancestor_record_offset":7124,"inlinee":11325,"source_line":487,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_direct] itertools::tuple_impl::impl$7::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"itertools","version":"0.14.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/itertools-0.14.0/src/tuple_impl.rs
//     evidence={"module":528,"record_offset":7292,"parent_record_offset":7124,"ancestor_record_offset":7124,"inlinee":9808,"source_line":193,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":528,"record_offset":7724,"parent_record_offset":7688,"ancestor_record_offset":7124,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::arith::impl$77::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\arith.rs
//     evidence={"module":528,"record_offset":7544,"parent_record_offset":7472,"ancestor_record_offset":7124,"inlinee":8710,"source_line":218,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":528,"record_offset":7368,"parent_record_offset":7328,"ancestor_record_offset":7124,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":528,"record_offset":7396,"parent_record_offset":7328,"ancestor_record_offset":7124,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":528,"record_offset":7328,"parent_record_offset":7292,"ancestor_record_offset":7124,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":528,"record_offset":7608,"parent_record_offset":7472,"ancestor_record_offset":7124,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::impl$50::sub
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":528,"record_offset":7472,"parent_record_offset":7432,"ancestor_record_offset":7124,"inlinee":8708,"source_line":1529,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":528,"record_offset":7688,"parent_record_offset":7608,"ancestor_record_offset":7124,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00528.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// === 完整可用 TPI 类型引用图（包含参数表、返回类型、字段等实际记录） ===
// TPI 0x1000:
// ---
// Kind:            LF_ARGLIST
// ArgList:
//   ArgIndices:      [  ]
// ...
// TPI 0x1001:
// ---
// Kind:            LF_PROCEDURE
// Procedure:
//   ReturnType:      3
//   CallConv:        NearC
//   Options:         [ None ]
//   ParameterCount:  0
//   ArgumentList:    4096
// ...

// === PDB STRUCTURES V1 BEGIN ===
// 数据结构事实按版本保留；PDB 定义 / 仅名称关系 / 原指令观察分别标注。
// 完整结构及原始类型记录: wall_path_segment.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::walls::wall_path_segment::WallPathSegment
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 16
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::resources::walls::wall_path_segment::WallPathSegment", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["country_core::resources::walls::WallId", "alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::map::HashMap<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::map::HashMap<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["country_core::resources::walls::wall_path_segment::WallPathSegment", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::drain::Drain<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>", "constructor": "alloc::vec::drain::Drain", "arguments": ["country_core::resources::walls::wall_path_segment::WallPathSegment", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::drain::Drain<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>,country_core::resources::walls::arches::ArchSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)>", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::drain::Drain<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>", "country_core::resources::walls::arches::ArchSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$1::spec_extend<country_core::resources::walls::arches::ArchSegment,core::iter::adapters::map::Map<alloc::vec::drain::Drain<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>,country_core::resources::walls::arches::ArchSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)>,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$1::spec_extend", "arguments": ["country_core::resources::walls::arches::ArchSegment", "core::iter::adapters::map::Map<alloc::vec::drain::Drain<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>,country_core::resources::walls::arches::ArchSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::drain::Drain<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>,country_core::resources::walls::arches::LintelSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)>", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::drain::Drain<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>", "country_core::resources::walls::arches::LintelSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_extend::impl$1::spec_extend<country_core::resources::walls::arches::LintelSegment,core::iter::adapters::map::Map<alloc::vec::drain::Drain<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>,country_core::resources::walls::arches::LintelSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)>,alloc::alloc::Global>", "constructor": "alloc::vec::spec_extend::impl$1::spec_extend", "arguments": ["country_core::resources::walls::arches::LintelSegment", "core::iter::adapters::map::Map<alloc::vec::drain::Drain<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>,country_core::resources::walls::arches::LintelSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::drain::Drain<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::drain::Drain<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["country_core::resources::walls::wall_path_segment::WallPathSegment", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::map::Map<alloc::vec::drain::Drain<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>,country_core::resources::walls::arches::ArchSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::map::Map<alloc::vec::drain::Drain<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>,country_core::resources::walls::arches::ArchSegment (*)(country_core::resources::walls::wall_path_segment::WallPathSegment)>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::walls::wall_path_segment::WallPathSegment>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::walls::wall_path_segment::WallPathSegment"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::resources::walls::WallId", "alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global> >", "constructor": "tuple$", "arguments": ["country_core::resources::walls::WallId", "alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::equivalent_key::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global> >", "constructor": "hashbrown::map::equivalent_key::closure_env$0", "arguments": ["country_core::resources::walls::WallId", "country_core::resources::walls::WallId", "alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::remove_entry<tuple$<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global> > >", "constructor": "hashbrown::raw::RawTable::remove_entry", "arguments": ["tuple$<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global> >", "alloc::alloc::Global", "hashbrown::map::equivalent_key::closure_env$0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_path_segment::WallPathSegment,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: wall_path_segment.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/building-segmentation/three-version-owner-map.json; mixed_or_unspecified; applicability requires per-version original evidence; [{"kind": "explicit_source_path_mention", "path": "crates/country-core/src/resources/walls/wall_path_segment.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/country-core/src/resources/walls/wall_path_segment.rs", "status": "source mention; exact assertion applicability remains version-scoped"}, {"kind": "explicit_source_path_mention", "path": "crates/country-core/src/resources/walls/wall_path_segment.rs", "status": "source mention; exact assertion applicability remains version-scoped"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: wall_path_segment.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 10; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::clutter_registry::ClutterId>,ref$<country_core::resources::clutter_registry::ClutterId>)
// 候选参数类型: ["ref$<country_core::resources::clutter_registry::ClutterId>", "ref$<country_core::resources::clutter_registry::ClutterId>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<u64>,ref$<u64>)
// 候选参数类型: ["ref$<u64>", "ref$<u64>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<ref$<alloc::string::String> >,ref$<ref$<alloc::string::String> >)
// 候选参数类型: ["ref$<ref$<alloc::string::String> >", "ref$<ref$<alloc::string::String> >"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<tuple$<alloc::string::String,std::path::PathBuf> >,ref$<tuple$<alloc::string::String,std::path::PathBuf> >)
// 候选参数类型: ["ref$<tuple$<alloc::string::String,std::path::PathBuf> >", "ref$<tuple$<alloc::string::String,std::path::PathBuf> >"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorId>)
// 候选参数类型: ["ref$<country_core::resources::walls::decorator::DecoratorId>", "ref$<country_core::resources::walls::decorator::DecoratorId>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<mesh_to_sdf::generate::grid::State>,ref$<mesh_to_sdf::generate::grid::State>)
// 候选参数类型: ["ref$<mesh_to_sdf::generate::grid::State>", "ref$<mesh_to_sdf::generate::grid::State>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<u32>,ref$<u32>)
// 候选参数类型: ["ref$<u32>", "ref$<u32>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<ref$<str$> >,ref$<ref$<str$> >)
// 候选参数类型: ["ref$<ref$<str$> >", "ref$<ref$<str$> >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_path_segment::WallPathSegment::append_pick_ribbon_triangles
// 信息 ID: e0e6ba57ea0dd303faa6d322
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::ui_arch_controls::ui_arch_controls @ 0x80efa3 -> 0xa05130 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::ui_lintel_controls::ui_lintel_controls @ 0x814b07 -> 0xa05130 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::collision::world::update_rapier_colliders @ 0x9fc8cf -> 0xa05130 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_path_segment::WallPathSegment::length_ws
// 信息 ID: 55f14ee1faab681c7eb8cd83
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7ecdb8 -> 0xa05d50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs @ 0x141cfa3 -> 0xa05d50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> @ 0x141e310 -> 0xa05d50 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> @ 0x141e32e -> 0xa05d50 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_path_segment::WallPathSegment::length_ws_2d
// 信息 ID: 1b52f9c8ed5a211ce51cf9ba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::construct_platform_stairs @ 0x7f2897 -> 0xa05010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs @ 0x141adad -> 0xa05010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs @ 0x141b397 -> 0xa05010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::wooden::construct_wooden_platform_stairs @ 0x141b4be -> 0xa05010 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_platform_stairs::stone::construct_stone_platform_stairs<system_wall_constructor::construct_platform_stairs::construct_platform_stairs::closure_env$0> @ 0x141e33b -> 0xa05010 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_path_segment::WallPathSegment::pt_count
// 信息 ID: 3be87edc2333963af32898b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 12 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7eb721 -> 0xa05d40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7ecd6f -> 0xa05d40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7ece2a -> 0xa05d40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7ece6e -> 0xa05d40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7ecea1 -> 0xa05d40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7ecedd -> 0xa05d40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_arches::construct_arches @ 0x7ecf1a -> 0xa05d40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::construct_lintels @ 0x811a76 -> 0xa05d40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::construct_lintels @ 0x811ab3 -> 0xa05d40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::construct_lintels @ 0x811ae8 -> 0xa05d40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::construct_lintels @ 0x811b1c -> 0xa05d40 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_lintels::construct_lintels @ 0x811b51 -> 0xa05d40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_path_segment::impl$0::append_pick_ribbon_triangles::closure$0
// 信息 ID: 6809451e0939507968a9fb17
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::walls::wall_path_segment::impl$0::append_pick_ribbon_triangles", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::wall_path_segment::WallPathSegment::append_pick_ribbon_triangles; module=528, inline_record=9156; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::wall_path_segment::WallPathSegment::append_pick_ribbon_triangles; module=528, inline_record=9752; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::wall_path_segment::WallPathSegment::append_pick_ribbon_triangles; module=528, inline_record=11692; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::wall_path_segment::WallPathSegment::append_pick_ribbon_triangles; module=528, inline_record=12540; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_path_segment::impl$0::append_pick_ribbon_triangles::closure$1
// 信息 ID: c6cedefdaf90f22d43d4c4fb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::walls::wall_path_segment::impl$0::append_pick_ribbon_triangles", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::walls::wall_path_segment::WallPathSegment::append_pick_ribbon_triangles; module=528, inline_record=8672; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::walls::wall_path_segment::WallPathSegment::append_pick_ribbon_triangles; module=528, inline_record=10972; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::walls::wall_path_segment::impl$2::clone
// 信息 ID: 966b06baaa7ca52ae5e85673
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] system_wall_constructor::construct_arches::construct_arches; module=394, inline_record=17764; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
