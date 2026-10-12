// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/render/mesh.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 5ceb23923110785ef1290b37ba7cdb2e591d429a82ef7dc45d35f385038b99f8
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mesh.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 518, 521, 530, 533, 536, 537, 539, 540, 544, 551, 558, 576, 578, 588, 596, 600, 605, 1636, 1643, 1682, 1687, 1700, 1727, 1770, 1779, 3151, 3991
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// use utils; // namespace candidate
//   package identities: [{"name":"utils","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::impl$2::clone
// 记录形式: inline_instance, inlinee_declaration; instances=13
// 源码行: 10
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7E69
// 首条位置: {"module":533,"inlinee":32361}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":533,"record_offset":94116,"parent_record_offset":93992,"ancestor_record_offset":93992,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$6::clone
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":533,"record_offset":94728,"parent_record_offset":93992,"ancestor_record_offset":93992,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":95456,"parent_record_offset":93992,"ancestor_record_offset":93992,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$5::clone
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":533,"record_offset":94056,"parent_record_offset":93992,"ancestor_record_offset":93992,"inlinee":13788,"source_line":1397,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":94556,"parent_record_offset":94532,"ancestor_record_offset":93992,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":94580,"parent_record_offset":94556,"ancestor_record_offset":93992,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":95640,"parent_record_offset":95616,"ancestor_record_offset":93992,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":95664,"parent_record_offset":95640,"ancestor_record_offset":93992,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":94604,"parent_record_offset":94580,"ancestor_record_offset":93992,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":95688,"parent_record_offset":95664,"ancestor_record_offset":93992,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":94532,"parent_record_offset":94348,"ancestor_record_offset":93992,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":533,"record_offset":95616,"parent_record_offset":95528,"ancestor_record_offset":93992,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":539,"record_offset":23788,"parent_record_offset":23764,"ancestor_record_offset":23552,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":533,"record_offset":94284,"parent_record_offset":94256,"ancestor_record_offset":93992,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":533,"record_offset":95556,"parent_record_offset":95528,"ancestor_record_offset":93992,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":533,"record_offset":95528,"parent_record_offset":95504,"ancestor_record_offset":93992,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":539,"record_offset":23836,"parent_record_offset":23812,"ancestor_record_offset":23552,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":539,"record_offset":23812,"parent_record_offset":23788,"ancestor_record_offset":23552,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":533,"record_offset":94348,"parent_record_offset":94312,"ancestor_record_offset":93992,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":533,"record_offset":94312,"parent_record_offset":94284,"ancestor_record_offset":93992,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":533,"record_offset":95504,"parent_record_offset":95480,"ancestor_record_offset":93992,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":533,"record_offset":94388,"parent_record_offset":94348,"ancestor_record_offset":93992,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":576,"record_offset":9916,"parent_record_offset":9888,"ancestor_record_offset":9768,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":533,"record_offset":94216,"parent_record_offset":94148,"ancestor_record_offset":93992,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":539,"record_offset":23764,"parent_record_offset":23740,"ancestor_record_offset":23552,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":533,"record_offset":94188,"parent_record_offset":94148,"ancestor_record_offset":93992,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":533,"record_offset":94256,"parent_record_offset":94216,"ancestor_record_offset":93992,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":533,"record_offset":94872,"parent_record_offset":94844,"ancestor_record_offset":93992,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":533,"record_offset":94148,"parent_record_offset":94116,"ancestor_record_offset":93992,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":533,"record_offset":94844,"parent_record_offset":94816,"ancestor_record_offset":93992,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":533,"record_offset":94412,"parent_record_offset":94388,"ancestor_record_offset":93992,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":533,"record_offset":94492,"parent_record_offset":94436,"ancestor_record_offset":93992,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=14
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":533,"record_offset":94436,"parent_record_offset":94412,"ancestor_record_offset":93992,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":533,"record_offset":94464,"parent_record_offset":94436,"ancestor_record_offset":93992,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":533,"record_offset":95584,"parent_record_offset":95556,"ancestor_record_offset":93992,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":533,"record_offset":94660,"parent_record_offset":94216,"ancestor_record_offset":93992,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":94684,"parent_record_offset":94660,"ancestor_record_offset":93992,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":533,"record_offset":95480,"parent_record_offset":95456,"ancestor_record_offset":93992,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$0::clone
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":533,"record_offset":94080,"parent_record_offset":94056,"ancestor_record_offset":93992,"inlinee":13277,"source_line":191,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":533,"record_offset":94788,"parent_record_offset":94760,"ancestor_record_offset":93992,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":533,"record_offset":94760,"parent_record_offset":94728,"ancestor_record_offset":93992,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":533,"record_offset":94816,"parent_record_offset":94788,"ancestor_record_offset":93992,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/00533.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::impl$0::vertex_count
// 记录形式: inline_instance, inlinee_declaration; instances=39
// 源码行: 22
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x76FE
// 首条位置: {"module":518,"inlinee":30462}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":518,"record_offset":96708,"parent_record_offset":95972,"ancestor_record_offset":95972,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":518,"record_offset":96004,"parent_record_offset":95972,"ancestor_record_offset":95972,"inlinee":8400,"source_line":543,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$52::next
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":518,"record_offset":96260,"parent_record_offset":95972,"ancestor_record_offset":95972,"inlinee":10754,"source_line":2236,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=56
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":518,"record_offset":96204,"parent_record_offset":96180,"ancestor_record_offset":95972,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":518,"record_offset":96480,"parent_record_offset":96456,"ancestor_record_offset":95972,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=56
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":518,"record_offset":96600,"parent_record_offset":96576,"ancestor_record_offset":95972,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":518,"record_offset":96424,"parent_record_offset":96396,"ancestor_record_offset":95972,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=27
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":530,"record_offset":24332,"parent_record_offset":24304,"ancestor_record_offset":23912,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":518,"record_offset":96456,"parent_record_offset":96424,"ancestor_record_offset":95972,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=27
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":530,"record_offset":24364,"parent_record_offset":24332,"ancestor_record_offset":23912,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":518,"record_offset":96396,"parent_record_offset":96356,"ancestor_record_offset":95972,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=27
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":530,"record_offset":24304,"parent_record_offset":24264,"ancestor_record_offset":23912,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=56
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":518,"record_offset":96180,"parent_record_offset":96156,"ancestor_record_offset":95972,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=28
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":518,"record_offset":96052,"parent_record_offset":96028,"ancestor_record_offset":95972,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=28
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":518,"record_offset":96308,"parent_record_offset":96284,"ancestor_record_offset":95972,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=56
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":518,"record_offset":96576,"parent_record_offset":96356,"ancestor_record_offset":95972,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=28
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":518,"record_offset":96156,"parent_record_offset":96100,"ancestor_record_offset":95972,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=28
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":518,"record_offset":96356,"parent_record_offset":96332,"ancestor_record_offset":95972,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=28
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":518,"record_offset":96076,"parent_record_offset":96052,"ancestor_record_offset":95972,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=27
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":518,"record_offset":96128,"parent_record_offset":96100,"ancestor_record_offset":95972,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=28
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":518,"record_offset":96100,"parent_record_offset":96076,"ancestor_record_offset":95972,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=28
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":518,"record_offset":96332,"parent_record_offset":96308,"ancestor_record_offset":95972,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":518,"record_offset":96028,"parent_record_offset":96004,"ancestor_record_offset":95972,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=28
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":518,"record_offset":96284,"parent_record_offset":96260,"ancestor_record_offset":95972,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::impl$0::find_attrib
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x947C
// 首条位置: {"module":1727,"inlinee":38012}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":1727,"record_offset":162780,"parent_record_offset":162752,"ancestor_record_offset":162752,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1727,"record_offset":163468,"parent_record_offset":163440,"ancestor_record_offset":162752,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1727,"record_offset":163024,"parent_record_offset":163000,"ancestor_record_offset":162752,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1727,"record_offset":163108,"parent_record_offset":163084,"ancestor_record_offset":162752,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1727,"record_offset":163636,"parent_record_offset":162920,"ancestor_record_offset":162752,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1727,"record_offset":163260,"parent_record_offset":163232,"ancestor_record_offset":162752,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":1727,"record_offset":163232,"parent_record_offset":163204,"ancestor_record_offset":162752,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1727,"record_offset":163048,"parent_record_offset":163024,"ancestor_record_offset":162752,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1727,"record_offset":163376,"parent_record_offset":163352,"ancestor_record_offset":162752,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1727,"record_offset":163552,"parent_record_offset":163524,"ancestor_record_offset":162752,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1727,"record_offset":163580,"parent_record_offset":163552,"ancestor_record_offset":162752,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1727,"record_offset":163496,"parent_record_offset":163468,"ancestor_record_offset":162752,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1727,"record_offset":163168,"parent_record_offset":163140,"ancestor_record_offset":162752,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1727,"record_offset":163204,"parent_record_offset":163168,"ancestor_record_offset":162752,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":1727,"record_offset":163140,"parent_record_offset":162920,"ancestor_record_offset":162752,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1727,"record_offset":163000,"parent_record_offset":162920,"ancestor_record_offset":162752,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1727,"record_offset":163664,"parent_record_offset":162920,"ancestor_record_offset":162752,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":1727,"record_offset":163084,"parent_record_offset":162920,"ancestor_record_offset":162752,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":1727,"record_offset":162972,"parent_record_offset":162920,"ancestor_record_offset":162752,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":1727,"record_offset":163440,"parent_record_offset":163412,"ancestor_record_offset":162752,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1727,"record_offset":162808,"parent_record_offset":162780,"ancestor_record_offset":162752,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1727,"record_offset":163412,"parent_record_offset":163304,"ancestor_record_offset":162752,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":1727,"record_offset":162840,"parent_record_offset":162808,"ancestor_record_offset":162752,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1727,"record_offset":163352,"parent_record_offset":163328,"ancestor_record_offset":162752,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1727,"record_offset":163756,"parent_record_offset":162920,"ancestor_record_offset":162752,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1727,"record_offset":163328,"parent_record_offset":163304,"ancestor_record_offset":162752,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1727,"record_offset":162896,"parent_record_offset":162868,"ancestor_record_offset":162752,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1727,"record_offset":162868,"parent_record_offset":162808,"ancestor_record_offset":162752,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1727,"record_offset":162920,"parent_record_offset":162896,"ancestor_record_offset":162752,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/01727.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":1727,"record_offset":163304,"parent_record_offset":162920,"ancestor_record_offset":162752,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/01727.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::Mesh::new
// 记录形式: inline_instance, inlinee_declaration; instances=31
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x76F7
// 首条位置: {"module":518,"inlinee":30455}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":518,"record_offset":77820,"parent_record_offset":77784,"ancestor_record_offset":77784,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":518,"record_offset":78020,"parent_record_offset":77992,"ancestor_record_offset":77784,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":518,"record_offset":78072,"parent_record_offset":78048,"ancestor_record_offset":77784,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":518,"record_offset":78048,"parent_record_offset":77992,"ancestor_record_offset":77784,"inlinee":4677,"source_line":433,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=26
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":518,"record_offset":78096,"parent_record_offset":78072,"ancestor_record_offset":77784,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":596,"record_offset":52512,"parent_record_offset":52456,"ancestor_record_offset":51908,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":596,"record_offset":52092,"parent_record_offset":52056,"ancestor_record_offset":51908,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":596,"record_offset":52212,"parent_record_offset":52180,"ancestor_record_offset":51908,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":518,"record_offset":77964,"parent_record_offset":77936,"ancestor_record_offset":77784,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":596,"record_offset":52300,"parent_record_offset":52180,"ancestor_record_offset":51908,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":596,"record_offset":52324,"parent_record_offset":52300,"ancestor_record_offset":51908,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":518,"record_offset":77876,"parent_record_offset":77848,"ancestor_record_offset":77784,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":596,"record_offset":52260,"parent_record_offset":52236,"ancestor_record_offset":51908,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":596,"record_offset":52120,"parent_record_offset":52092,"ancestor_record_offset":51908,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":518,"record_offset":77992,"parent_record_offset":77936,"ancestor_record_offset":77784,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":518,"record_offset":77848,"parent_record_offset":77820,"ancestor_record_offset":77784,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":596,"record_offset":52148,"parent_record_offset":52120,"ancestor_record_offset":51908,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":596,"record_offset":52180,"parent_record_offset":52148,"ancestor_record_offset":51908,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":518,"record_offset":77936,"parent_record_offset":77904,"ancestor_record_offset":77784,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":518,"record_offset":77904,"parent_record_offset":77876,"ancestor_record_offset":77784,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::Mesh::is_valid
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 50
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7FAA
// 首条位置: {"module":536,"inlinee":32682}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":536,"record_offset":9044,"parent_record_offset":9020,"ancestor_record_offset":9020,"inlinee":5735,"source_line":3021,"archive":"_pdb/C/modules/00536.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::Mesh::set_source_file
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7804
// 首条位置: {"module":521,"inlinee":30724}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":521,"record_offset":30180,"parent_record_offset":30156,"ancestor_record_offset":30156,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":521,"record_offset":30452,"parent_record_offset":30428,"ancestor_record_offset":30156,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":521,"record_offset":30476,"parent_record_offset":30452,"ancestor_record_offset":30156,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":521,"record_offset":30500,"parent_record_offset":30476,"ancestor_record_offset":30156,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":521,"record_offset":30428,"parent_record_offset":30372,"ancestor_record_offset":30156,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":521,"record_offset":30400,"parent_record_offset":30372,"ancestor_record_offset":30156,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":521,"record_offset":30372,"parent_record_offset":30348,"ancestor_record_offset":30156,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":521,"record_offset":30348,"parent_record_offset":30324,"ancestor_record_offset":30156,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":521,"record_offset":30204,"parent_record_offset":30180,"ancestor_record_offset":30156,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::Mesh::has_attribute
// 记录形式: inline_instance, inlinee_declaration; instances=16
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7CB6
// 首条位置: {"module":530,"inlinee":31926}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":530,"record_offset":22828,"parent_record_offset":22804,"ancestor_record_offset":22804,"inlinee":21897,"source_line":1231,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":530,"record_offset":23536,"parent_record_offset":23508,"ancestor_record_offset":22804,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":530,"record_offset":23096,"parent_record_offset":23072,"ancestor_record_offset":22804,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":530,"record_offset":23180,"parent_record_offset":23156,"ancestor_record_offset":22804,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":530,"record_offset":23704,"parent_record_offset":22960,"ancestor_record_offset":22804,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":530,"record_offset":23328,"parent_record_offset":23300,"ancestor_record_offset":22804,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":530,"record_offset":23300,"parent_record_offset":23272,"ancestor_record_offset":22804,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":530,"record_offset":23120,"parent_record_offset":23096,"ancestor_record_offset":22804,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":530,"record_offset":23444,"parent_record_offset":23420,"ancestor_record_offset":22804,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":530,"record_offset":23620,"parent_record_offset":23592,"ancestor_record_offset":22804,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":530,"record_offset":23648,"parent_record_offset":23620,"ancestor_record_offset":22804,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":530,"record_offset":23564,"parent_record_offset":23536,"ancestor_record_offset":22804,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":530,"record_offset":23240,"parent_record_offset":23212,"ancestor_record_offset":22804,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":530,"record_offset":23272,"parent_record_offset":23240,"ancestor_record_offset":22804,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":530,"record_offset":23212,"parent_record_offset":22960,"ancestor_record_offset":22804,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":530,"record_offset":23072,"parent_record_offset":22960,"ancestor_record_offset":22804,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":530,"record_offset":23732,"parent_record_offset":22960,"ancestor_record_offset":22804,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=18
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":530,"record_offset":23156,"parent_record_offset":22960,"ancestor_record_offset":22804,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":530,"record_offset":23016,"parent_record_offset":22960,"ancestor_record_offset":22804,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":530,"record_offset":23508,"parent_record_offset":23480,"ancestor_record_offset":22804,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":530,"record_offset":22852,"parent_record_offset":22828,"ancestor_record_offset":22804,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":530,"record_offset":23480,"parent_record_offset":23372,"ancestor_record_offset":22804,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":530,"record_offset":22884,"parent_record_offset":22852,"ancestor_record_offset":22804,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":530,"record_offset":23420,"parent_record_offset":23396,"ancestor_record_offset":22804,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":530,"record_offset":23824,"parent_record_offset":22960,"ancestor_record_offset":22804,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":530,"record_offset":23396,"parent_record_offset":23372,"ancestor_record_offset":22804,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":530,"record_offset":22936,"parent_record_offset":22912,"ancestor_record_offset":22804,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":530,"record_offset":22912,"parent_record_offset":22852,"ancestor_record_offset":22804,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":539,"record_offset":18964,"parent_record_offset":18924,"ancestor_record_offset":18868,"inlinee":9370,"source_line":1370,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::len
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":539,"record_offset":18988,"parent_record_offset":18964,"ancestor_record_offset":18868,"inlinee":9371,"source_line":1364,"archive":"_pdb/C/modules/00539.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":530,"record_offset":22960,"parent_record_offset":22936,"ancestor_record_offset":22804,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":530,"record_offset":23044,"parent_record_offset":22960,"ancestor_record_offset":22804,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":530,"record_offset":23372,"parent_record_offset":22960,"ancestor_record_offset":22804,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,2>,alloc::alloc::Global> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":551,"record_offset":14996,"rva":11303280,"va":5380012400}
// 编译实例 RVAs: 0xac7970, 0x24e1aa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rhapsody..vertex..VertexAttributeValues$u20$as$u20$core..convert..From$LT$alloc..vec..Vec$LT$$u5b$f32$u3b$$u20$2$u5d$$GT$$GT$$GT$::from
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t3649209","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380012400,"raw_hex":"e8b9ae3700","size":5,"soft_fail":false,"target_va":5383661952,"va":5380012738,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4334142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380012400,"raw_hex":"e8c2ddbdff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380012825,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t-4334400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380012400,"raw_hex":"e8c0dcbdff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380013099,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11290619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380012400,"raw_hex":"e805b853ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380012806,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cfc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6531598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380012400,"raw_hex":"e8f2559cff","size":5,"soft_fail":false,"target_va":5373480896,"va":5380012489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141413ea0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17620514","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407382176,"raw_hex":"e8de21f3fe","size":5,"soft_fail":false,"target_va":5389762208,"va":5407382717,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417ae6e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13527886","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380012400,"raw_hex":"e84e6bce00","size":5,"soft_fail":false,"target_va":5393540832,"va":5380012941,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37044427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380012400,"raw_hex":"e8cb403502","size":5,"soft_fail":false,"target_va":5417057040,"va":5380012608,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37044200","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380012400,"raw_hex":"e8e83f3502","size":5,"soft_fail":false,"target_va":5417057056,"va":5380012851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36608097","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380012400,"raw_hex":"e861982e02","size":5,"soft_fail":false,"target_va":5416620794,"va":5380012692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":551,"record_offset":16244,"parent_record_offset":14996,"ancestor_record_offset":14996,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":551,"record_offset":16844,"parent_record_offset":14996,"ancestor_record_offset":14996,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":16872,"parent_record_offset":14996,"ancestor_record_offset":14996,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":551,"record_offset":15200,"parent_record_offset":14996,"ancestor_record_offset":14996,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":551,"record_offset":17868,"parent_record_offset":14996,"ancestor_record_offset":14996,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":16664,"parent_record_offset":16640,"ancestor_record_offset":14996,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":16688,"parent_record_offset":16664,"ancestor_record_offset":14996,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":17264,"parent_record_offset":17240,"ancestor_record_offset":14996,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":17288,"parent_record_offset":17264,"ancestor_record_offset":14996,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":16712,"parent_record_offset":16688,"ancestor_record_offset":14996,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":17312,"parent_record_offset":17288,"ancestor_record_offset":14996,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":16640,"parent_record_offset":16488,"ancestor_record_offset":14996,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":17240,"parent_record_offset":17148,"ancestor_record_offset":14996,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":16424,"parent_record_offset":16396,"ancestor_record_offset":14996,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":16996,"parent_record_offset":16972,"ancestor_record_offset":14996,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":16972,"parent_record_offset":16948,"ancestor_record_offset":14996,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":16488,"parent_record_offset":16452,"ancestor_record_offset":14996,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":16452,"parent_record_offset":16424,"ancestor_record_offset":14996,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":16948,"parent_record_offset":16924,"ancestor_record_offset":14996,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":16528,"parent_record_offset":16488,"ancestor_record_offset":14996,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":551,"record_offset":16308,"parent_record_offset":16272,"ancestor_record_offset":14996,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":551,"record_offset":16336,"parent_record_offset":16308,"ancestor_record_offset":14996,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":551,"record_offset":16364,"parent_record_offset":16336,"ancestor_record_offset":14996,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":551,"record_offset":16272,"parent_record_offset":16244,"ancestor_record_offset":14996,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":16396,"parent_record_offset":16364,"ancestor_record_offset":14996,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":551,"record_offset":16552,"parent_record_offset":16528,"ancestor_record_offset":14996,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":551,"record_offset":15904,"parent_record_offset":15876,"ancestor_record_offset":14996,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":551,"record_offset":15464,"parent_record_offset":15440,"ancestor_record_offset":14996,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":551,"record_offset":15548,"parent_record_offset":15524,"ancestor_record_offset":14996,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":551,"record_offset":16072,"parent_record_offset":15332,"ancestor_record_offset":14996,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":551,"record_offset":16600,"parent_record_offset":16576,"ancestor_record_offset":14996,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":551,"record_offset":16576,"parent_record_offset":16552,"ancestor_record_offset":14996,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":551,"record_offset":17028,"parent_record_offset":16996,"ancestor_record_offset":14996,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":551,"record_offset":15696,"parent_record_offset":15668,"ancestor_record_offset":14996,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":551,"record_offset":15668,"parent_record_offset":15640,"ancestor_record_offset":14996,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":551,"record_offset":16768,"parent_record_offset":16364,"ancestor_record_offset":14996,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":15488,"parent_record_offset":15464,"ancestor_record_offset":14996,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":16900,"parent_record_offset":16872,"ancestor_record_offset":14996,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":551,"record_offset":15812,"parent_record_offset":15788,"ancestor_record_offset":14996,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":551,"record_offset":15988,"parent_record_offset":15960,"ancestor_record_offset":14996,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":551,"record_offset":16016,"parent_record_offset":15988,"ancestor_record_offset":14996,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":551,"record_offset":15932,"parent_record_offset":15904,"ancestor_record_offset":14996,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":551,"record_offset":15608,"parent_record_offset":15580,"ancestor_record_offset":14996,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":551,"record_offset":15640,"parent_record_offset":15608,"ancestor_record_offset":14996,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":551,"record_offset":15580,"parent_record_offset":15332,"ancestor_record_offset":14996,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":551,"record_offset":15440,"parent_record_offset":15332,"ancestor_record_offset":14996,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":551,"record_offset":16100,"parent_record_offset":15332,"ancestor_record_offset":14996,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":551,"record_offset":15524,"parent_record_offset":15332,"ancestor_record_offset":14996,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":551,"record_offset":15384,"parent_record_offset":15332,"ancestor_record_offset":14996,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":551,"record_offset":15876,"parent_record_offset":15848,"ancestor_record_offset":14996,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":551,"record_offset":15224,"parent_record_offset":15200,"ancestor_record_offset":14996,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":551,"record_offset":15848,"parent_record_offset":15740,"ancestor_record_offset":14996,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":551,"record_offset":15256,"parent_record_offset":15224,"ancestor_record_offset":14996,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":15788,"parent_record_offset":15764,"ancestor_record_offset":14996,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":16192,"parent_record_offset":15332,"ancestor_record_offset":14996,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":15764,"parent_record_offset":15740,"ancestor_record_offset":14996,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":15308,"parent_record_offset":15284,"ancestor_record_offset":14996,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":15284,"parent_record_offset":15224,"ancestor_record_offset":14996,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":15332,"parent_record_offset":15308,"ancestor_record_offset":14996,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":15412,"parent_record_offset":15332,"ancestor_record_offset":14996,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":15740,"parent_record_offset":15332,"ancestor_record_offset":14996,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,3>,alloc::alloc::Global> >
// 记录形式: compiled_procedure; instances=2
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":551,"record_offset":24296,"rva":11305584,"va":5380014704}
// 编译实例 RVAs: 0xac8270, 0x24e1f20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rhapsody..vertex..VertexAttributeValues$u20$as$u20$core..convert..From$LT$alloc..vec..Vec$LT$$u5b$f32$u3b$$u20$3$u5d$$GT$$GT$$GT$::from
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t3647017","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380014704,"raw_hex":"e829a63700","size":5,"soft_fail":false,"target_va":5383662064,"va":5380015042,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4336446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380014704,"raw_hex":"e8c2d4bdff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380015129,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t-4336704","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380014704,"raw_hex":"e8c0d3bdff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380015403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11292923","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380014704,"raw_hex":"e805af53ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380015110,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cfc0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6533902","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380014704,"raw_hex":"e8f24c9cff","size":5,"soft_fail":false,"target_va":5373480896,"va":5380014793,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141413ea0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17621666","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5407383328,"raw_hex":"e85e1df3fe","size":5,"soft_fail":false,"target_va":5389762208,"va":5407383869,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417ae6e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13525582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380014704,"raw_hex":"e84e62ce00","size":5,"soft_fail":false,"target_va":5393540832,"va":5380015245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37042123","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380014704,"raw_hex":"e8cb373502","size":5,"soft_fail":false,"target_va":5417057040,"va":5380014912,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37041896","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380014704,"raw_hex":"e8e8363502","size":5,"soft_fail":false,"target_va":5417057056,"va":5380015155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36605793","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380014704,"raw_hex":"e8618f2e02","size":5,"soft_fail":false,"target_va":5416620794,"va":5380014996,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":551,"record_offset":25544,"parent_record_offset":24296,"ancestor_record_offset":24296,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":551,"record_offset":26144,"parent_record_offset":24296,"ancestor_record_offset":24296,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":26172,"parent_record_offset":24296,"ancestor_record_offset":24296,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":551,"record_offset":24500,"parent_record_offset":24296,"ancestor_record_offset":24296,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":551,"record_offset":27168,"parent_record_offset":24296,"ancestor_record_offset":24296,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":25964,"parent_record_offset":25940,"ancestor_record_offset":24296,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":25988,"parent_record_offset":25964,"ancestor_record_offset":24296,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":26564,"parent_record_offset":26540,"ancestor_record_offset":24296,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":26588,"parent_record_offset":26564,"ancestor_record_offset":24296,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":26012,"parent_record_offset":25988,"ancestor_record_offset":24296,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":26612,"parent_record_offset":26588,"ancestor_record_offset":24296,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":25940,"parent_record_offset":25788,"ancestor_record_offset":24296,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":26540,"parent_record_offset":26448,"ancestor_record_offset":24296,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":25724,"parent_record_offset":25696,"ancestor_record_offset":24296,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":26296,"parent_record_offset":26272,"ancestor_record_offset":24296,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":26272,"parent_record_offset":26248,"ancestor_record_offset":24296,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":25788,"parent_record_offset":25752,"ancestor_record_offset":24296,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":25752,"parent_record_offset":25724,"ancestor_record_offset":24296,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":26248,"parent_record_offset":26224,"ancestor_record_offset":24296,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":25828,"parent_record_offset":25788,"ancestor_record_offset":24296,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":551,"record_offset":25608,"parent_record_offset":25572,"ancestor_record_offset":24296,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":551,"record_offset":25636,"parent_record_offset":25608,"ancestor_record_offset":24296,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":551,"record_offset":25664,"parent_record_offset":25636,"ancestor_record_offset":24296,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":551,"record_offset":25572,"parent_record_offset":25544,"ancestor_record_offset":24296,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":25696,"parent_record_offset":25664,"ancestor_record_offset":24296,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":551,"record_offset":25852,"parent_record_offset":25828,"ancestor_record_offset":24296,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":551,"record_offset":25204,"parent_record_offset":25176,"ancestor_record_offset":24296,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":551,"record_offset":24764,"parent_record_offset":24740,"ancestor_record_offset":24296,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":551,"record_offset":24848,"parent_record_offset":24824,"ancestor_record_offset":24296,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":551,"record_offset":25372,"parent_record_offset":24632,"ancestor_record_offset":24296,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":551,"record_offset":25900,"parent_record_offset":25876,"ancestor_record_offset":24296,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":551,"record_offset":25876,"parent_record_offset":25852,"ancestor_record_offset":24296,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":551,"record_offset":26328,"parent_record_offset":26296,"ancestor_record_offset":24296,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":551,"record_offset":24996,"parent_record_offset":24968,"ancestor_record_offset":24296,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":551,"record_offset":24968,"parent_record_offset":24940,"ancestor_record_offset":24296,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":551,"record_offset":26068,"parent_record_offset":25664,"ancestor_record_offset":24296,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":24788,"parent_record_offset":24764,"ancestor_record_offset":24296,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":26200,"parent_record_offset":26172,"ancestor_record_offset":24296,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":551,"record_offset":25112,"parent_record_offset":25088,"ancestor_record_offset":24296,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":551,"record_offset":25288,"parent_record_offset":25260,"ancestor_record_offset":24296,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":551,"record_offset":25316,"parent_record_offset":25288,"ancestor_record_offset":24296,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":551,"record_offset":25232,"parent_record_offset":25204,"ancestor_record_offset":24296,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":551,"record_offset":24908,"parent_record_offset":24880,"ancestor_record_offset":24296,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":551,"record_offset":24940,"parent_record_offset":24908,"ancestor_record_offset":24296,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":551,"record_offset":24880,"parent_record_offset":24632,"ancestor_record_offset":24296,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":551,"record_offset":24740,"parent_record_offset":24632,"ancestor_record_offset":24296,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":551,"record_offset":25400,"parent_record_offset":24632,"ancestor_record_offset":24296,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":551,"record_offset":24824,"parent_record_offset":24632,"ancestor_record_offset":24296,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":551,"record_offset":24684,"parent_record_offset":24632,"ancestor_record_offset":24296,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":551,"record_offset":25176,"parent_record_offset":25148,"ancestor_record_offset":24296,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":551,"record_offset":24524,"parent_record_offset":24500,"ancestor_record_offset":24296,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":551,"record_offset":25148,"parent_record_offset":25040,"ancestor_record_offset":24296,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":551,"record_offset":24556,"parent_record_offset":24524,"ancestor_record_offset":24296,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":25088,"parent_record_offset":25064,"ancestor_record_offset":24296,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":25492,"parent_record_offset":24632,"ancestor_record_offset":24296,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":25064,"parent_record_offset":25040,"ancestor_record_offset":24296,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":24608,"parent_record_offset":24584,"ancestor_record_offset":24296,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":24584,"parent_record_offset":24524,"ancestor_record_offset":24296,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":24632,"parent_record_offset":24608,"ancestor_record_offset":24296,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":24712,"parent_record_offset":24632,"ancestor_record_offset":24296,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":25040,"parent_record_offset":24632,"ancestor_record_offset":24296,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,4>,alloc::alloc::Global> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":551,"record_offset":1668,"rva":11300080,"va":5380009200}
// 编译实例 RVAs: 0xac6cf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rhapsody..vertex..VertexAttributeValues$u20$as$u20$core..convert..From$LT$alloc..vec..Vec$LT$$u5b$f32$u3b$$u20$4$u5d$$GT$$GT$$GT$::from
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t3652633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380009200,"raw_hex":"e819bc3700","size":5,"soft_fail":false,"target_va":5383662176,"va":5380009538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4330942","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380009200,"raw_hex":"e842eabdff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380009625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-4331200","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380009200,"raw_hex":"e840e9bdff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380009899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11287419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380009200,"raw_hex":"e885c453ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380009606,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cfc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6528398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380009200,"raw_hex":"e872629cff","size":5,"soft_fail":false,"target_va":5373480896,"va":5380009289,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417ae6e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13531086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380009200,"raw_hex":"e8ce77ce00","size":5,"soft_fail":false,"target_va":5393540832,"va":5380009741,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37047627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380009200,"raw_hex":"e84b4d3502","size":5,"soft_fail":false,"target_va":5417057040,"va":5380009408,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37047400","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380009200,"raw_hex":"e8684c3502","size":5,"soft_fail":false,"target_va":5417057056,"va":5380009651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36611297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380009200,"raw_hex":"e8e1a42e02","size":5,"soft_fail":false,"target_va":5416620794,"va":5380009492,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":551,"record_offset":2916,"parent_record_offset":1668,"ancestor_record_offset":1668,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":551,"record_offset":3516,"parent_record_offset":1668,"ancestor_record_offset":1668,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":3544,"parent_record_offset":1668,"ancestor_record_offset":1668,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":551,"record_offset":1872,"parent_record_offset":1668,"ancestor_record_offset":1668,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":551,"record_offset":4540,"parent_record_offset":1668,"ancestor_record_offset":1668,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":3336,"parent_record_offset":3312,"ancestor_record_offset":1668,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":3360,"parent_record_offset":3336,"ancestor_record_offset":1668,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":3936,"parent_record_offset":3912,"ancestor_record_offset":1668,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":3960,"parent_record_offset":3936,"ancestor_record_offset":1668,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":3384,"parent_record_offset":3360,"ancestor_record_offset":1668,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":3984,"parent_record_offset":3960,"ancestor_record_offset":1668,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":3312,"parent_record_offset":3160,"ancestor_record_offset":1668,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":3912,"parent_record_offset":3820,"ancestor_record_offset":1668,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":3096,"parent_record_offset":3068,"ancestor_record_offset":1668,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":3668,"parent_record_offset":3644,"ancestor_record_offset":1668,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":3644,"parent_record_offset":3620,"ancestor_record_offset":1668,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":3160,"parent_record_offset":3124,"ancestor_record_offset":1668,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":3124,"parent_record_offset":3096,"ancestor_record_offset":1668,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":3620,"parent_record_offset":3596,"ancestor_record_offset":1668,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":3200,"parent_record_offset":3160,"ancestor_record_offset":1668,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":551,"record_offset":2980,"parent_record_offset":2944,"ancestor_record_offset":1668,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":551,"record_offset":3008,"parent_record_offset":2980,"ancestor_record_offset":1668,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":551,"record_offset":3036,"parent_record_offset":3008,"ancestor_record_offset":1668,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":551,"record_offset":2944,"parent_record_offset":2916,"ancestor_record_offset":1668,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":3068,"parent_record_offset":3036,"ancestor_record_offset":1668,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":551,"record_offset":3224,"parent_record_offset":3200,"ancestor_record_offset":1668,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":551,"record_offset":2576,"parent_record_offset":2548,"ancestor_record_offset":1668,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":551,"record_offset":2136,"parent_record_offset":2112,"ancestor_record_offset":1668,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":551,"record_offset":2220,"parent_record_offset":2196,"ancestor_record_offset":1668,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":551,"record_offset":2744,"parent_record_offset":2004,"ancestor_record_offset":1668,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":551,"record_offset":3272,"parent_record_offset":3248,"ancestor_record_offset":1668,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":551,"record_offset":3248,"parent_record_offset":3224,"ancestor_record_offset":1668,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":551,"record_offset":3700,"parent_record_offset":3668,"ancestor_record_offset":1668,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":551,"record_offset":2368,"parent_record_offset":2340,"ancestor_record_offset":1668,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":551,"record_offset":2340,"parent_record_offset":2312,"ancestor_record_offset":1668,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":551,"record_offset":3440,"parent_record_offset":3036,"ancestor_record_offset":1668,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":2160,"parent_record_offset":2136,"ancestor_record_offset":1668,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":3572,"parent_record_offset":3544,"ancestor_record_offset":1668,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":551,"record_offset":2484,"parent_record_offset":2460,"ancestor_record_offset":1668,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":551,"record_offset":2660,"parent_record_offset":2632,"ancestor_record_offset":1668,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":551,"record_offset":2688,"parent_record_offset":2660,"ancestor_record_offset":1668,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":551,"record_offset":2604,"parent_record_offset":2576,"ancestor_record_offset":1668,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":551,"record_offset":2280,"parent_record_offset":2252,"ancestor_record_offset":1668,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":551,"record_offset":2312,"parent_record_offset":2280,"ancestor_record_offset":1668,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":551,"record_offset":2252,"parent_record_offset":2004,"ancestor_record_offset":1668,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":551,"record_offset":2112,"parent_record_offset":2004,"ancestor_record_offset":1668,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":551,"record_offset":2772,"parent_record_offset":2004,"ancestor_record_offset":1668,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":551,"record_offset":2196,"parent_record_offset":2004,"ancestor_record_offset":1668,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":551,"record_offset":2056,"parent_record_offset":2004,"ancestor_record_offset":1668,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":551,"record_offset":2548,"parent_record_offset":2520,"ancestor_record_offset":1668,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":551,"record_offset":1896,"parent_record_offset":1872,"ancestor_record_offset":1668,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":551,"record_offset":2520,"parent_record_offset":2412,"ancestor_record_offset":1668,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":551,"record_offset":1928,"parent_record_offset":1896,"ancestor_record_offset":1668,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":2460,"parent_record_offset":2436,"ancestor_record_offset":1668,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":2864,"parent_record_offset":2004,"ancestor_record_offset":1668,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":2436,"parent_record_offset":2412,"ancestor_record_offset":1668,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":1980,"parent_record_offset":1956,"ancestor_record_offset":1668,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":1956,"parent_record_offset":1896,"ancestor_record_offset":1668,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":2004,"parent_record_offset":1980,"ancestor_record_offset":1668,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":2084,"parent_record_offset":2004,"ancestor_record_offset":1668,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":2412,"parent_record_offset":2004,"ancestor_record_offset":1668,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<f32,alloc::alloc::Global> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":551,"record_offset":19652,"rva":11304432,"va":5380013552}
// 编译实例 RVAs: 0xac7df0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rhapsody..vertex..VertexAttributeValues$u20$as$u20$core..convert..From$LT$alloc..vec..Vec$LT$f32$GT$$GT$$GT$::from
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t3647305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380013552,"raw_hex":"e849a73700","size":5,"soft_fail":false,"target_va":5383661200,"va":5380013890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4335294","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380013552,"raw_hex":"e842d9bdff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380013977,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-4335552","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380013552,"raw_hex":"e840d8bdff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380014251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11291771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380013552,"raw_hex":"e885b353ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380013958,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cfc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6532750","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380013552,"raw_hex":"e872519cff","size":5,"soft_fail":false,"target_va":5373480896,"va":5380013641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417ae6e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13526734","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380013552,"raw_hex":"e8ce66ce00","size":5,"soft_fail":false,"target_va":5393540832,"va":5380014093,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37043275","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380013552,"raw_hex":"e84b3c3502","size":5,"soft_fail":false,"target_va":5417057040,"va":5380013760,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37043048","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380013552,"raw_hex":"e8683b3502","size":5,"soft_fail":false,"target_va":5417057056,"va":5380014003,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36606945","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380013552,"raw_hex":"e8e1932e02","size":5,"soft_fail":false,"target_va":5416620794,"va":5380013844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":551,"record_offset":20888,"parent_record_offset":19652,"ancestor_record_offset":19652,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":551,"record_offset":21488,"parent_record_offset":19652,"ancestor_record_offset":19652,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":21516,"parent_record_offset":19652,"ancestor_record_offset":19652,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":551,"record_offset":19844,"parent_record_offset":19652,"ancestor_record_offset":19652,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":551,"record_offset":22512,"parent_record_offset":19652,"ancestor_record_offset":19652,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":21308,"parent_record_offset":21284,"ancestor_record_offset":19652,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":21332,"parent_record_offset":21308,"ancestor_record_offset":19652,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":21908,"parent_record_offset":21884,"ancestor_record_offset":19652,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":21932,"parent_record_offset":21908,"ancestor_record_offset":19652,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":21356,"parent_record_offset":21332,"ancestor_record_offset":19652,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":21956,"parent_record_offset":21932,"ancestor_record_offset":19652,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":21284,"parent_record_offset":21132,"ancestor_record_offset":19652,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":21884,"parent_record_offset":21792,"ancestor_record_offset":19652,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":21068,"parent_record_offset":21040,"ancestor_record_offset":19652,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":21640,"parent_record_offset":21616,"ancestor_record_offset":19652,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":21616,"parent_record_offset":21592,"ancestor_record_offset":19652,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":21132,"parent_record_offset":21096,"ancestor_record_offset":19652,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":21096,"parent_record_offset":21068,"ancestor_record_offset":19652,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":21592,"parent_record_offset":21568,"ancestor_record_offset":19652,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":21172,"parent_record_offset":21132,"ancestor_record_offset":19652,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":551,"record_offset":20952,"parent_record_offset":20916,"ancestor_record_offset":19652,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":551,"record_offset":20980,"parent_record_offset":20952,"ancestor_record_offset":19652,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":551,"record_offset":21008,"parent_record_offset":20980,"ancestor_record_offset":19652,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":551,"record_offset":20916,"parent_record_offset":20888,"ancestor_record_offset":19652,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":21040,"parent_record_offset":21008,"ancestor_record_offset":19652,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":551,"record_offset":21196,"parent_record_offset":21172,"ancestor_record_offset":19652,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":551,"record_offset":20548,"parent_record_offset":20520,"ancestor_record_offset":19652,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":551,"record_offset":20108,"parent_record_offset":20084,"ancestor_record_offset":19652,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":551,"record_offset":20192,"parent_record_offset":20168,"ancestor_record_offset":19652,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":551,"record_offset":20716,"parent_record_offset":19976,"ancestor_record_offset":19652,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":551,"record_offset":21244,"parent_record_offset":21220,"ancestor_record_offset":19652,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":551,"record_offset":21220,"parent_record_offset":21196,"ancestor_record_offset":19652,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":551,"record_offset":21672,"parent_record_offset":21640,"ancestor_record_offset":19652,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":551,"record_offset":20340,"parent_record_offset":20312,"ancestor_record_offset":19652,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":551,"record_offset":20312,"parent_record_offset":20284,"ancestor_record_offset":19652,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":551,"record_offset":21412,"parent_record_offset":21008,"ancestor_record_offset":19652,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":20132,"parent_record_offset":20108,"ancestor_record_offset":19652,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":21544,"parent_record_offset":21516,"ancestor_record_offset":19652,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":551,"record_offset":20456,"parent_record_offset":20432,"ancestor_record_offset":19652,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":551,"record_offset":20632,"parent_record_offset":20604,"ancestor_record_offset":19652,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":551,"record_offset":20660,"parent_record_offset":20632,"ancestor_record_offset":19652,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":551,"record_offset":20576,"parent_record_offset":20548,"ancestor_record_offset":19652,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":551,"record_offset":20252,"parent_record_offset":20224,"ancestor_record_offset":19652,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":551,"record_offset":20284,"parent_record_offset":20252,"ancestor_record_offset":19652,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":551,"record_offset":20224,"parent_record_offset":19976,"ancestor_record_offset":19652,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":551,"record_offset":20084,"parent_record_offset":19976,"ancestor_record_offset":19652,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":551,"record_offset":20744,"parent_record_offset":19976,"ancestor_record_offset":19652,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":551,"record_offset":20168,"parent_record_offset":19976,"ancestor_record_offset":19652,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":551,"record_offset":20028,"parent_record_offset":19976,"ancestor_record_offset":19652,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":551,"record_offset":20520,"parent_record_offset":20492,"ancestor_record_offset":19652,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":551,"record_offset":19868,"parent_record_offset":19844,"ancestor_record_offset":19652,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":551,"record_offset":20492,"parent_record_offset":20384,"ancestor_record_offset":19652,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":551,"record_offset":19900,"parent_record_offset":19868,"ancestor_record_offset":19652,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":20432,"parent_record_offset":20408,"ancestor_record_offset":19652,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":20836,"parent_record_offset":19976,"ancestor_record_offset":19652,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":20408,"parent_record_offset":20384,"ancestor_record_offset":19652,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":19952,"parent_record_offset":19928,"ancestor_record_offset":19652,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":19928,"parent_record_offset":19868,"ancestor_record_offset":19652,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":19976,"parent_record_offset":19952,"ancestor_record_offset":19652,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":20056,"parent_record_offset":19976,"ancestor_record_offset":19652,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":20384,"parent_record_offset":19976,"ancestor_record_offset":19652,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<i32,alloc::alloc::Global> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":551,"record_offset":10352,"rva":11302128,"va":5380011248}
// 编译实例 RVAs: 0xac74f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rhapsody..vertex..VertexAttributeValues$u20$as$u20$core..convert..From$LT$alloc..vec..Vec$LT$i32$GT$$GT$$GT$::from
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t3649721","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380011248,"raw_hex":"e8b9b03700","size":5,"soft_fail":false,"target_va":5383661312,"va":5380011586,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4332990","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380011248,"raw_hex":"e842e2bdff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380011673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-4333248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380011248,"raw_hex":"e840e1bdff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380011947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11289467","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380011248,"raw_hex":"e885bc53ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380011654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cfc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6530446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380011248,"raw_hex":"e8725a9cff","size":5,"soft_fail":false,"target_va":5373480896,"va":5380011337,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417ae6e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13529038","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380011248,"raw_hex":"e8ce6fce00","size":5,"soft_fail":false,"target_va":5393540832,"va":5380011789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37045579","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380011248,"raw_hex":"e84b453502","size":5,"soft_fail":false,"target_va":5417057040,"va":5380011456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37045352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380011248,"raw_hex":"e868443502","size":5,"soft_fail":false,"target_va":5417057056,"va":5380011699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36609249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380011248,"raw_hex":"e8e19c2e02","size":5,"soft_fail":false,"target_va":5416620794,"va":5380011540,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":551,"record_offset":11588,"parent_record_offset":10352,"ancestor_record_offset":10352,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::convert::impl$3::into
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":551,"record_offset":12188,"parent_record_offset":10352,"ancestor_record_offset":10352,"inlinee":5101,"source_line":777,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":12216,"parent_record_offset":10352,"ancestor_record_offset":10352,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":551,"record_offset":10544,"parent_record_offset":10352,"ancestor_record_offset":10352,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":551,"record_offset":13212,"parent_record_offset":10352,"ancestor_record_offset":10352,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":12008,"parent_record_offset":11984,"ancestor_record_offset":10352,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":12032,"parent_record_offset":12008,"ancestor_record_offset":10352,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":12608,"parent_record_offset":12584,"ancestor_record_offset":10352,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":12632,"parent_record_offset":12608,"ancestor_record_offset":10352,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":12056,"parent_record_offset":12032,"ancestor_record_offset":10352,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":12656,"parent_record_offset":12632,"ancestor_record_offset":10352,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":11984,"parent_record_offset":11832,"ancestor_record_offset":10352,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":12584,"parent_record_offset":12492,"ancestor_record_offset":10352,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":11768,"parent_record_offset":11740,"ancestor_record_offset":10352,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":12340,"parent_record_offset":12316,"ancestor_record_offset":10352,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":12316,"parent_record_offset":12292,"ancestor_record_offset":10352,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":11832,"parent_record_offset":11796,"ancestor_record_offset":10352,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":11796,"parent_record_offset":11768,"ancestor_record_offset":10352,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":12292,"parent_record_offset":12268,"ancestor_record_offset":10352,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":11872,"parent_record_offset":11832,"ancestor_record_offset":10352,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":551,"record_offset":11652,"parent_record_offset":11616,"ancestor_record_offset":10352,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":551,"record_offset":11680,"parent_record_offset":11652,"ancestor_record_offset":10352,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":551,"record_offset":11708,"parent_record_offset":11680,"ancestor_record_offset":10352,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":551,"record_offset":11616,"parent_record_offset":11588,"ancestor_record_offset":10352,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":11740,"parent_record_offset":11708,"ancestor_record_offset":10352,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":551,"record_offset":11896,"parent_record_offset":11872,"ancestor_record_offset":10352,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":551,"record_offset":11248,"parent_record_offset":11220,"ancestor_record_offset":10352,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":551,"record_offset":10808,"parent_record_offset":10784,"ancestor_record_offset":10352,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":551,"record_offset":10892,"parent_record_offset":10868,"ancestor_record_offset":10352,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":551,"record_offset":11416,"parent_record_offset":10676,"ancestor_record_offset":10352,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":551,"record_offset":11944,"parent_record_offset":11920,"ancestor_record_offset":10352,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":551,"record_offset":11920,"parent_record_offset":11896,"ancestor_record_offset":10352,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":551,"record_offset":12372,"parent_record_offset":12340,"ancestor_record_offset":10352,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":551,"record_offset":11040,"parent_record_offset":11012,"ancestor_record_offset":10352,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":551,"record_offset":11012,"parent_record_offset":10984,"ancestor_record_offset":10352,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":551,"record_offset":12112,"parent_record_offset":11708,"ancestor_record_offset":10352,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":10832,"parent_record_offset":10808,"ancestor_record_offset":10352,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":12244,"parent_record_offset":12216,"ancestor_record_offset":10352,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":551,"record_offset":11156,"parent_record_offset":11132,"ancestor_record_offset":10352,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":551,"record_offset":11332,"parent_record_offset":11304,"ancestor_record_offset":10352,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":551,"record_offset":11360,"parent_record_offset":11332,"ancestor_record_offset":10352,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":551,"record_offset":11276,"parent_record_offset":11248,"ancestor_record_offset":10352,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":551,"record_offset":10952,"parent_record_offset":10924,"ancestor_record_offset":10352,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":551,"record_offset":10984,"parent_record_offset":10952,"ancestor_record_offset":10352,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":551,"record_offset":10924,"parent_record_offset":10676,"ancestor_record_offset":10352,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":551,"record_offset":10784,"parent_record_offset":10676,"ancestor_record_offset":10352,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":551,"record_offset":11444,"parent_record_offset":10676,"ancestor_record_offset":10352,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":551,"record_offset":10868,"parent_record_offset":10676,"ancestor_record_offset":10352,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":551,"record_offset":10728,"parent_record_offset":10676,"ancestor_record_offset":10352,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":551,"record_offset":11220,"parent_record_offset":11192,"ancestor_record_offset":10352,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":551,"record_offset":10568,"parent_record_offset":10544,"ancestor_record_offset":10352,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":551,"record_offset":11192,"parent_record_offset":11084,"ancestor_record_offset":10352,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":551,"record_offset":10600,"parent_record_offset":10568,"ancestor_record_offset":10352,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":11132,"parent_record_offset":11108,"ancestor_record_offset":10352,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":11536,"parent_record_offset":10676,"ancestor_record_offset":10352,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":11108,"parent_record_offset":11084,"ancestor_record_offset":10352,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":10652,"parent_record_offset":10628,"ancestor_record_offset":10352,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":10628,"parent_record_offset":10568,"ancestor_record_offset":10352,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":10676,"parent_record_offset":10652,"ancestor_record_offset":10352,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":10756,"parent_record_offset":10676,"ancestor_record_offset":10352,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":11084,"parent_record_offset":10676,"ancestor_record_offset":10352,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::Mesh::set_attribute<enum2$<rhapsody::vertex::VertexAttributeValues> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 62
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":551,"record_offset":6324,"rva":11301232,"va":5380010352}
// 编译实例 RVAs: 0xac7170
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4332050","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380010352,"raw_hex":"e8eee5bdff","size":5,"soft_fail":false,"target_va":5375678688,"va":5380010733,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-4332279","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380010352,"raw_hex":"e809e5bdff","size":5,"soft_fail":false,"target_va":5375678704,"va":5380010978,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11288527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380010352,"raw_hex":"e831c053ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5380010714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cfc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6529550","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380010352,"raw_hex":"e8f25d9cff","size":5,"soft_fail":false,"target_va":5373480896,"va":5380010441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140acc860
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t21377","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380010352,"raw_hex":"e881530000","size":5,"soft_fail":false,"target_va":5380032608,"va":5380011226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1417ae6e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13530007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380010352,"raw_hex":"e89773ce00","size":5,"soft_fail":false,"target_va":5393540832,"va":5380010820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37046475","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380010352,"raw_hex":"e8cb483502","size":5,"soft_fail":false,"target_va":5417057040,"va":5380010560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37046296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380010352,"raw_hex":"e818483502","size":5,"soft_fail":false,"target_va":5417057056,"va":5380010755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36610145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380010352,"raw_hex":"e861a02e02","size":5,"soft_fail":false,"target_va":5416620794,"va":5380010644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":551,"record_offset":7576,"parent_record_offset":6324,"ancestor_record_offset":6324,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":8176,"parent_record_offset":6324,"ancestor_record_offset":6324,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":551,"record_offset":6520,"parent_record_offset":6324,"ancestor_record_offset":6324,"inlinee":5435,"source_line":1259,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":551,"record_offset":9144,"parent_record_offset":6324,"ancestor_record_offset":6324,"inlinee":5499,"source_line":1295,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":7996,"parent_record_offset":7972,"ancestor_record_offset":6324,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":8020,"parent_record_offset":7996,"ancestor_record_offset":6324,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":8568,"parent_record_offset":8544,"ancestor_record_offset":6324,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":8592,"parent_record_offset":8568,"ancestor_record_offset":6324,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":8044,"parent_record_offset":8020,"ancestor_record_offset":6324,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":8616,"parent_record_offset":8592,"ancestor_record_offset":6324,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":7972,"parent_record_offset":7820,"ancestor_record_offset":6324,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":551,"record_offset":8544,"parent_record_offset":8452,"ancestor_record_offset":6324,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":7756,"parent_record_offset":7728,"ancestor_record_offset":6324,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":8300,"parent_record_offset":8276,"ancestor_record_offset":6324,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":8276,"parent_record_offset":8252,"ancestor_record_offset":6324,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":7820,"parent_record_offset":7784,"ancestor_record_offset":6324,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":7784,"parent_record_offset":7756,"ancestor_record_offset":6324,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":8252,"parent_record_offset":8228,"ancestor_record_offset":6324,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":551,"record_offset":7860,"parent_record_offset":7820,"ancestor_record_offset":6324,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":551,"record_offset":7640,"parent_record_offset":7604,"ancestor_record_offset":6324,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":551,"record_offset":7668,"parent_record_offset":7640,"ancestor_record_offset":6324,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":551,"record_offset":7696,"parent_record_offset":7668,"ancestor_record_offset":6324,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":551,"record_offset":7604,"parent_record_offset":7576,"ancestor_record_offset":6324,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":551,"record_offset":7728,"parent_record_offset":7696,"ancestor_record_offset":6324,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":551,"record_offset":7884,"parent_record_offset":7860,"ancestor_record_offset":6324,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":551,"record_offset":7236,"parent_record_offset":7208,"ancestor_record_offset":6324,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":551,"record_offset":6796,"parent_record_offset":6772,"ancestor_record_offset":6324,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":551,"record_offset":6880,"parent_record_offset":6856,"ancestor_record_offset":6324,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":551,"record_offset":7404,"parent_record_offset":6664,"ancestor_record_offset":6324,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":551,"record_offset":7932,"parent_record_offset":7908,"ancestor_record_offset":6324,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":551,"record_offset":7908,"parent_record_offset":7884,"ancestor_record_offset":6324,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":551,"record_offset":8332,"parent_record_offset":8300,"ancestor_record_offset":6324,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":551,"record_offset":7028,"parent_record_offset":7000,"ancestor_record_offset":6324,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":551,"record_offset":7000,"parent_record_offset":6972,"ancestor_record_offset":6324,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":551,"record_offset":8100,"parent_record_offset":7696,"ancestor_record_offset":6324,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":6820,"parent_record_offset":6796,"ancestor_record_offset":6324,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":551,"record_offset":8204,"parent_record_offset":8176,"ancestor_record_offset":6324,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":551,"record_offset":7144,"parent_record_offset":7120,"ancestor_record_offset":6324,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":551,"record_offset":7320,"parent_record_offset":7292,"ancestor_record_offset":6324,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":551,"record_offset":7348,"parent_record_offset":7320,"ancestor_record_offset":6324,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":551,"record_offset":7264,"parent_record_offset":7236,"ancestor_record_offset":6324,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":551,"record_offset":6940,"parent_record_offset":6912,"ancestor_record_offset":6324,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":551,"record_offset":6972,"parent_record_offset":6940,"ancestor_record_offset":6324,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":551,"record_offset":6912,"parent_record_offset":6664,"ancestor_record_offset":6324,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":551,"record_offset":6772,"parent_record_offset":6664,"ancestor_record_offset":6324,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":551,"record_offset":7432,"parent_record_offset":6664,"ancestor_record_offset":6324,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":551,"record_offset":6856,"parent_record_offset":6664,"ancestor_record_offset":6324,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":551,"record_offset":6716,"parent_record_offset":6664,"ancestor_record_offset":6324,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":551,"record_offset":7208,"parent_record_offset":7180,"ancestor_record_offset":6324,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":551,"record_offset":6548,"parent_record_offset":6520,"ancestor_record_offset":6324,"inlinee":5436,"source_line":1452,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":551,"record_offset":7180,"parent_record_offset":7072,"ancestor_record_offset":6324,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":551,"record_offset":6584,"parent_record_offset":6548,"ancestor_record_offset":6324,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":7120,"parent_record_offset":7096,"ancestor_record_offset":6324,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":7524,"parent_record_offset":6664,"ancestor_record_offset":6324,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":7096,"parent_record_offset":7072,"ancestor_record_offset":6324,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":6640,"parent_record_offset":6612,"ancestor_record_offset":6324,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":6612,"parent_record_offset":6548,"ancestor_record_offset":6324,"inlinee":5437,"source_line":1259,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":6664,"parent_record_offset":6640,"ancestor_record_offset":6324,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":6744,"parent_record_offset":6664,"ancestor_record_offset":6324,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":551,"record_offset":7072,"parent_record_offset":6664,"ancestor_record_offset":6324,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00551.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::Mesh::set_indices
// 记录形式: inline_instance, inlinee_declaration; instances=15
// 源码行: 70
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x780E
// 首条位置: {"module":521,"inlinee":30734}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":521,"record_offset":33748,"parent_record_offset":33724,"ancestor_record_offset":33724,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":521,"record_offset":33936,"parent_record_offset":33912,"ancestor_record_offset":33724,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":521,"record_offset":33960,"parent_record_offset":33936,"ancestor_record_offset":33724,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":521,"record_offset":33984,"parent_record_offset":33960,"ancestor_record_offset":33724,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":521,"record_offset":33912,"parent_record_offset":33820,"ancestor_record_offset":33724,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":521,"record_offset":33848,"parent_record_offset":33820,"ancestor_record_offset":33724,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":521,"record_offset":33820,"parent_record_offset":33796,"ancestor_record_offset":33724,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":521,"record_offset":33796,"parent_record_offset":33772,"ancestor_record_offset":33724,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":521,"record_offset":33880,"parent_record_offset":33848,"ancestor_record_offset":33724,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":521,"record_offset":33772,"parent_record_offset":33748,"ancestor_record_offset":33724,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00521.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::Mesh::arrayed_indices
// 记录形式: compiled_procedure; instances=1
// 源码行: 74
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":596,"record_offset":7636,"rva":12754640,"va":5381463760}
// 编译实例 RVAs: 0xc29ed0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141603aa0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10328952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381463760,"raw_hex":"e8789b9d00","size":5,"soft_fail":false,"target_va":5391792800,"va":5381463843,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":596,"record_offset":7892,"parent_record_offset":7636,"ancestor_record_offset":7636,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::chunks_exact
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":596,"record_offset":7776,"parent_record_offset":7636,"ancestor_record_offset":7636,"inlinee":27399,"source_line":1242,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":596,"record_offset":7916,"parent_record_offset":7892,"ancestor_record_offset":7636,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":596,"record_offset":7940,"parent_record_offset":7916,"ancestor_record_offset":7636,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":596,"record_offset":7852,"parent_record_offset":7828,"ancestor_record_offset":7636,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":596,"record_offset":7828,"parent_record_offset":7800,"ancestor_record_offset":7636,"inlinee":11960,"source_line":2038,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::ChunksExact::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":596,"record_offset":7800,"parent_record_offset":7776,"ancestor_record_offset":7636,"inlinee":27401,"source_line":1851,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::impl$1::arrayed_indices::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13DA5
// 首条位置: {"module":1643,"inlinee":81317}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::Mesh::make_double_sided_by_duplicating_triangles
// 记录形式: compiled_procedure; instances=1
// 源码行: 81
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":596,"record_offset":14300,"rva":12756544,"va":5381465664}
// 编译实例 RVAs: 0xc2a640
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fc50
//     owner=unresolved_namespace; count=1
//     target_aliases=33 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34886711","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381465664,"raw_hex":"e837541402","size":5,"soft_fail":false,"target_va":5416352848,"va":5381466132,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6fd10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t34886833","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381465664,"raw_hex":"e8b1541402","size":5,"soft_fail":false,"target_va":5416353040,"va":5381466202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb40
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35591340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381465664,"raw_hex":"e8ac141f02","size":5,"soft_fail":false,"target_va":5417057088,"va":5381465743,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35154357","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381465664,"raw_hex":"e8b5691802","size":5,"soft_fail":false,"target_va":5416620794,"va":5381466432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=9
//     evidence={"assembly":"\tcallq\t35155506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381465664,"raw_hex":"e8326e1802","size":5,"soft_fail":false,"target_va":5416621762,"va":5381466251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::reserve_exact
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":596,"record_offset":14480,"parent_record_offset":14300,"ancestor_record_offset":14300,"inlinee":14813,"source_line":1475,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::resize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":596,"record_offset":14816,"parent_record_offset":14300,"ancestor_record_offset":14300,"inlinee":11070,"source_line":3465,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":596,"record_offset":15632,"parent_record_offset":14300,"ancestor_record_offset":14300,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$15::index_mut
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":596,"record_offset":15920,"parent_record_offset":14300,"ancestor_record_offset":14300,"inlinee":8424,"source_line":3811,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::rev::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\rev.rs
//     evidence={"module":596,"record_offset":15536,"parent_record_offset":14300,"ancestor_record_offset":14300,"inlinee":9302,"source_line":52,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::step_by
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":596,"record_offset":15420,"parent_record_offset":14300,"ancestor_record_offset":14300,"inlinee":10413,"source_line":435,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":596,"record_offset":15124,"parent_record_offset":15068,"ancestor_record_offset":14300,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":596,"record_offset":14932,"parent_record_offset":14904,"ancestor_record_offset":14300,"inlinee":5066,"source_line":338,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve_exact
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":596,"record_offset":14516,"parent_record_offset":14480,"ancestor_record_offset":14300,"inlinee":14814,"source_line":371,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":596,"record_offset":14652,"parent_record_offset":14628,"ancestor_record_offset":14300,"inlinee":4788,"source_line":616,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::grow_exact
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":596,"record_offset":14692,"parent_record_offset":14588,"ancestor_record_offset":14300,"inlinee":13399,"source_line":783,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::needs_to_grow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":596,"record_offset":14628,"parent_record_offset":14588,"ancestor_record_offset":14300,"inlinee":5068,"source_line":764,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":596,"record_offset":15180,"parent_record_offset":15152,"ancestor_record_offset":14300,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":596,"record_offset":15152,"parent_record_offset":15124,"ancestor_record_offset":14300,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":596,"record_offset":14960,"parent_record_offset":14932,"ancestor_record_offset":14300,"inlinee":5067,"source_line":653,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::reserve_exact
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":596,"record_offset":14552,"parent_record_offset":14516,"ancestor_record_offset":14300,"inlinee":13403,"source_line":706,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::set_ptr_and_cap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":596,"record_offset":14764,"parent_record_offset":14692,"ancestor_record_offset":14300,"inlinee":13393,"source_line":770,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_reserve_exact
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":596,"record_offset":14588,"parent_record_offset":14552,"ancestor_record_offset":14300,"inlinee":13398,"source_line":717,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":596,"record_offset":15068,"parent_record_offset":14848,"ancestor_record_offset":14300,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":596,"record_offset":14848,"parent_record_offset":14816,"ancestor_record_offset":14300,"inlinee":6984,"source_line":3591,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":596,"record_offset":14904,"parent_record_offset":14848,"ancestor_record_offset":14300,"inlinee":5090,"source_line":1445,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":596,"record_offset":15384,"parent_record_offset":14848,"ancestor_record_offset":14300,"inlinee":5115,"source_line":18,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":596,"record_offset":15288,"parent_record_offset":15256,"ancestor_record_offset":14300,"inlinee":5175,"source_line":1916,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::step_by::StepBy::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\step_by.rs
//     evidence={"module":596,"record_offset":15444,"parent_record_offset":15420,"ancestor_record_offset":14300,"inlinee":10415,"source_line":34,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::step_by::impl$19::setup
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\step_by.rs
//     evidence={"module":596,"record_offset":15468,"parent_record_offset":15444,"ancestor_record_offset":14300,"inlinee":12630,"source_line":428,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::step_by::impl$24::spec_next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\step_by.rs
//     evidence={"module":596,"record_offset":15592,"parent_record_offset":15564,"ancestor_record_offset":14300,"inlinee":37949,"source_line":516,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::step_by::impl$3::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\step_by.rs
//     evidence={"module":596,"record_offset":15564,"parent_record_offset":15536,"ancestor_record_offset":14300,"inlinee":37947,"source_line":107,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":596,"record_offset":15256,"parent_record_offset":15224,"ancestor_record_offset":14300,"inlinee":5173,"source_line":771,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\range.rs
//     evidence={"module":596,"record_offset":15224,"parent_record_offset":14848,"ancestor_record_offset":14300,"inlinee":5171,"source_line":856,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::div_ceil
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":596,"record_offset":15492,"parent_record_offset":15468,"ancestor_record_offset":14300,"inlinee":10418,"source_line":3657,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":596,"record_offset":15020,"parent_record_offset":14992,"ancestor_record_offset":14300,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":596,"record_offset":15328,"parent_record_offset":14848,"ancestor_record_offset":14300,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":596,"record_offset":14728,"parent_record_offset":14692,"ancestor_record_offset":14300,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":596,"record_offset":15660,"parent_record_offset":15632,"ancestor_record_offset":14300,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":596,"record_offset":15948,"parent_record_offset":15920,"ancestor_record_offset":14300,"inlinee":8426,"source_line":30,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":596,"record_offset":15688,"parent_record_offset":15660,"ancestor_record_offset":14300,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":596,"record_offset":15976,"parent_record_offset":15948,"ancestor_record_offset":14300,"inlinee":8559,"source_line":276,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::Mesh::has_color
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 106
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7CB5
// 首条位置: {"module":530,"inlinee":31925}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::mesh::Mesh::has_attribute
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":530,"record_offset":22804,"parent_record_offset":22780,"ancestor_record_offset":22780,"inlinee":31926,"source_line":58,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":530,"record_offset":23536,"parent_record_offset":23508,"ancestor_record_offset":22780,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":530,"record_offset":23096,"parent_record_offset":23072,"ancestor_record_offset":22780,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":530,"record_offset":23180,"parent_record_offset":23156,"ancestor_record_offset":22780,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":530,"record_offset":23704,"parent_record_offset":22960,"ancestor_record_offset":22780,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":530,"record_offset":23328,"parent_record_offset":23300,"ancestor_record_offset":22780,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":530,"record_offset":23300,"parent_record_offset":23272,"ancestor_record_offset":22780,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":530,"record_offset":23120,"parent_record_offset":23096,"ancestor_record_offset":22780,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":530,"record_offset":23444,"parent_record_offset":23420,"ancestor_record_offset":22780,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":530,"record_offset":23620,"parent_record_offset":23592,"ancestor_record_offset":22780,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":530,"record_offset":23648,"parent_record_offset":23620,"ancestor_record_offset":22780,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":530,"record_offset":23564,"parent_record_offset":23536,"ancestor_record_offset":22780,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":530,"record_offset":23240,"parent_record_offset":23212,"ancestor_record_offset":22780,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":530,"record_offset":23272,"parent_record_offset":23240,"ancestor_record_offset":22780,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":530,"record_offset":23212,"parent_record_offset":22960,"ancestor_record_offset":22780,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":530,"record_offset":23072,"parent_record_offset":22960,"ancestor_record_offset":22780,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":530,"record_offset":23732,"parent_record_offset":22960,"ancestor_record_offset":22780,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":530,"record_offset":23156,"parent_record_offset":22960,"ancestor_record_offset":22780,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":530,"record_offset":23016,"parent_record_offset":22960,"ancestor_record_offset":22780,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":530,"record_offset":23508,"parent_record_offset":23480,"ancestor_record_offset":22780,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":530,"record_offset":22852,"parent_record_offset":22828,"ancestor_record_offset":22780,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":530,"record_offset":23480,"parent_record_offset":23372,"ancestor_record_offset":22780,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":530,"record_offset":22884,"parent_record_offset":22852,"ancestor_record_offset":22780,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":530,"record_offset":23420,"parent_record_offset":23396,"ancestor_record_offset":22780,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":530,"record_offset":23824,"parent_record_offset":22960,"ancestor_record_offset":22780,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":530,"record_offset":23396,"parent_record_offset":23372,"ancestor_record_offset":22780,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":530,"record_offset":22936,"parent_record_offset":22912,"ancestor_record_offset":22780,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":530,"record_offset":22912,"parent_record_offset":22852,"ancestor_record_offset":22780,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":530,"record_offset":22960,"parent_record_offset":22936,"ancestor_record_offset":22780,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":530,"record_offset":23044,"parent_record_offset":22960,"ancestor_record_offset":22780,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":530,"record_offset":23372,"parent_record_offset":22960,"ancestor_record_offset":22780,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":530,"record_offset":22828,"parent_record_offset":22804,"ancestor_record_offset":22780,"inlinee":21897,"source_line":1231,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::Mesh::add_color
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=29
// 源码行: 115
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x76FC
// 首条位置: {"module":518,"inlinee":30460}
// 编译实例 RVAs: 0xc2a950
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140ac8270
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1451874","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381466448,"raw_hex":"e89ed8e9ff","size":5,"soft_fail":false,"target_va":5380014704,"va":5381466573,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f6400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10271302","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381466448,"raw_hex":"e846ba9c00","size":5,"soft_fail":false,"target_va":5391737856,"va":5381466549,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::render::mesh::impl$0::vertex_count
//     owner=private_source_ownership_candidate; count=21
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":518,"record_offset":95972,"parent_record_offset":95884,"ancestor_record_offset":95884,"inlinee":30462,"source_line":22,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::is_empty
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":518,"record_offset":95916,"parent_record_offset":95884,"ancestor_record_offset":95884,"inlinee":12122,"source_line":708,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":518,"record_offset":96204,"parent_record_offset":96180,"ancestor_record_offset":95884,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":518,"record_offset":96480,"parent_record_offset":96456,"ancestor_record_offset":95884,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":518,"record_offset":96708,"parent_record_offset":95972,"ancestor_record_offset":95884,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=42
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":518,"record_offset":96600,"parent_record_offset":96576,"ancestor_record_offset":95884,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":518,"record_offset":96424,"parent_record_offset":96396,"ancestor_record_offset":95884,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=20
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":530,"record_offset":24332,"parent_record_offset":24304,"ancestor_record_offset":23884,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":518,"record_offset":96456,"parent_record_offset":96424,"ancestor_record_offset":95884,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=20
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":530,"record_offset":24364,"parent_record_offset":24332,"ancestor_record_offset":23884,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.1/src/control/bitmask.rs
//     evidence={"module":518,"record_offset":96396,"parent_record_offset":96356,"ancestor_record_offset":95884,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=20
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":530,"record_offset":24304,"parent_record_offset":24264,"ancestor_record_offset":23884,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00530.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=42
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":518,"record_offset":96180,"parent_record_offset":96156,"ancestor_record_offset":95884,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=package_namespace_candidate; count=20
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":518,"record_offset":95940,"parent_record_offset":95916,"ancestor_record_offset":95884,"inlinee":12123,"source_line":845,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=21
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":518,"record_offset":96052,"parent_record_offset":96028,"ancestor_record_offset":95884,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=21
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":518,"record_offset":96308,"parent_record_offset":96284,"ancestor_record_offset":95884,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=42
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":518,"record_offset":96576,"parent_record_offset":96356,"ancestor_record_offset":95884,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=21
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":518,"record_offset":96156,"parent_record_offset":96100,"ancestor_record_offset":95884,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=21
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":518,"record_offset":96356,"parent_record_offset":96332,"ancestor_record_offset":95884,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=21
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":518,"record_offset":96076,"parent_record_offset":96052,"ancestor_record_offset":95884,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=20
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":518,"record_offset":96128,"parent_record_offset":96100,"ancestor_record_offset":95884,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=21
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":518,"record_offset":96100,"parent_record_offset":96076,"ancestor_record_offset":95884,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=21
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":518,"record_offset":96332,"parent_record_offset":96308,"ancestor_record_offset":95884,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":518,"record_offset":96028,"parent_record_offset":96004,"ancestor_record_offset":95884,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":518,"record_offset":96004,"parent_record_offset":95972,"ancestor_record_offset":95884,"inlinee":8400,"source_line":543,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":518,"record_offset":96284,"parent_record_offset":96260,"ancestor_record_offset":95884,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$52::next
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":518,"record_offset":96260,"parent_record_offset":95972,"ancestor_record_offset":95884,"inlinee":10754,"source_line":2236,"archive":"_pdb/C/modules/00518.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::Mesh::with_color
// 记录形式: inline_instance, inlinee_declaration; instances=20
// 源码行: 126
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x83C5
// 首条位置: {"module":544,"inlinee":33733}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::render::mesh::Mesh::add_color
//     owner=private_source_ownership_candidate; count=16
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":544,"record_offset":2992,"parent_record_offset":2952,"ancestor_record_offset":2952,"inlinee":30460,"source_line":115,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":544,"record_offset":3312,"parent_record_offset":3288,"ancestor_record_offset":2952,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":544,"record_offset":3588,"parent_record_offset":3564,"ancestor_record_offset":2952,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":544,"record_offset":3816,"parent_record_offset":3080,"ancestor_record_offset":2952,"inlinee":5839,"source_line":1224,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=32
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":544,"record_offset":3708,"parent_record_offset":3684,"ancestor_record_offset":2952,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$0::vertex_count
//     owner=private_source_ownership_candidate; count=16
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":544,"record_offset":3080,"parent_record_offset":2992,"ancestor_record_offset":2952,"inlinee":30462,"source_line":22,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=16
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":544,"record_offset":3532,"parent_record_offset":3504,"ancestor_record_offset":2952,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=16
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":544,"record_offset":3564,"parent_record_offset":3532,"ancestor_record_offset":2952,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=16
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":544,"record_offset":3504,"parent_record_offset":3464,"ancestor_record_offset":2952,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=32
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":544,"record_offset":3288,"parent_record_offset":3264,"ancestor_record_offset":2952,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=package_namespace_candidate; count=16
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":544,"record_offset":3048,"parent_record_offset":3024,"ancestor_record_offset":2952,"inlinee":12123,"source_line":845,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=16
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":544,"record_offset":3160,"parent_record_offset":3136,"ancestor_record_offset":2952,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=16
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":544,"record_offset":3416,"parent_record_offset":3392,"ancestor_record_offset":2952,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=32
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":544,"record_offset":3684,"parent_record_offset":3464,"ancestor_record_offset":2952,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=16
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":544,"record_offset":3264,"parent_record_offset":3208,"ancestor_record_offset":2952,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=16
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":544,"record_offset":3464,"parent_record_offset":3440,"ancestor_record_offset":2952,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=16
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":544,"record_offset":3184,"parent_record_offset":3160,"ancestor_record_offset":2952,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=16
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":544,"record_offset":3236,"parent_record_offset":3208,"ancestor_record_offset":2952,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=16
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":544,"record_offset":3208,"parent_record_offset":3184,"ancestor_record_offset":2952,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=16
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":544,"record_offset":3440,"parent_record_offset":3416,"ancestor_record_offset":2952,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::is_empty
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":544,"record_offset":3024,"parent_record_offset":2992,"ancestor_record_offset":2952,"inlinee":12122,"source_line":708,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":544,"record_offset":3136,"parent_record_offset":3112,"ancestor_record_offset":2952,"inlinee":8401,"source_line":638,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":544,"record_offset":3112,"parent_record_offset":3080,"ancestor_record_offset":2952,"inlinee":8400,"source_line":543,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$38::next
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":544,"record_offset":3392,"parent_record_offset":3368,"ancestor_record_offset":2952,"inlinee":8501,"source_line":2077,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$52::next
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":544,"record_offset":3368,"parent_record_offset":3080,"ancestor_record_offset":2952,"inlinee":10754,"source_line":2236,"archive":"_pdb/C/modules/00544.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::Mesh::aabb_bounds
// 记录形式: compiled_procedure; instances=1
// 源码行: 160
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":596,"record_offset":4528,"rva":12753632,"va":5381462752}
// 编译实例 RVAs: 0xc29ae0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cfc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7981908","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381462752,"raw_hex":"e8ac3486ff","size":5,"soft_fail":false,"target_va":5373480896,"va":5381462799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35158438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381462752,"raw_hex":"e8a6791802","size":5,"soft_fail":false,"target_va":5416621600,"va":5381463157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::vertex::VertexAttributeValues::as_f32x3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t2224682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381462752,"raw_hex":"e82af22100","size":5,"soft_fail":false,"target_va":5383687680,"va":5381462993,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb3::expand_to_contain_point
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1839419","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381462752,"raw_hex":"e83b111c00","size":5,"soft_fail":false,"target_va":5383302544,"va":5381463120,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":596,"record_offset":6060,"parent_record_offset":4528,"ancestor_record_offset":4528,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":596,"record_offset":4668,"parent_record_offset":4528,"ancestor_record_offset":4528,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] utils::geometry::aabb::Aabb3::from_points
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/utils/src/geometry/aabb.rs
//     evidence={"module":596,"record_offset":5712,"parent_record_offset":4528,"ancestor_record_offset":4528,"inlinee":29652,"source_line":242,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":596,"record_offset":5372,"parent_record_offset":5344,"ancestor_record_offset":4528,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":596,"record_offset":4932,"parent_record_offset":4908,"ancestor_record_offset":4528,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":596,"record_offset":5016,"parent_record_offset":4992,"ancestor_record_offset":4528,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":596,"record_offset":5540,"parent_record_offset":4800,"ancestor_record_offset":4528,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":596,"record_offset":5784,"parent_record_offset":5760,"ancestor_record_offset":4528,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":596,"record_offset":5760,"parent_record_offset":5712,"ancestor_record_offset":4528,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":596,"record_offset":5164,"parent_record_offset":5136,"ancestor_record_offset":4528,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":596,"record_offset":5136,"parent_record_offset":5108,"ancestor_record_offset":4528,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":596,"record_offset":5864,"parent_record_offset":5784,"ancestor_record_offset":4528,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":596,"record_offset":4956,"parent_record_offset":4932,"ancestor_record_offset":4528,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":596,"record_offset":5280,"parent_record_offset":5256,"ancestor_record_offset":4528,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":596,"record_offset":5832,"parent_record_offset":5808,"ancestor_record_offset":4528,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":596,"record_offset":5456,"parent_record_offset":5428,"ancestor_record_offset":4528,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":596,"record_offset":5484,"parent_record_offset":5456,"ancestor_record_offset":4528,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":596,"record_offset":5808,"parent_record_offset":5784,"ancestor_record_offset":4528,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":596,"record_offset":5400,"parent_record_offset":5372,"ancestor_record_offset":4528,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":596,"record_offset":5076,"parent_record_offset":5048,"ancestor_record_offset":4528,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":596,"record_offset":5108,"parent_record_offset":5076,"ancestor_record_offset":4528,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":596,"record_offset":5048,"parent_record_offset":4800,"ancestor_record_offset":4528,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":596,"record_offset":4908,"parent_record_offset":4800,"ancestor_record_offset":4528,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":596,"record_offset":5568,"parent_record_offset":4800,"ancestor_record_offset":4528,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":596,"record_offset":4992,"parent_record_offset":4800,"ancestor_record_offset":4528,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":596,"record_offset":4852,"parent_record_offset":4800,"ancestor_record_offset":4528,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":596,"record_offset":5344,"parent_record_offset":5316,"ancestor_record_offset":4528,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":596,"record_offset":4692,"parent_record_offset":4668,"ancestor_record_offset":4528,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":596,"record_offset":5316,"parent_record_offset":5208,"ancestor_record_offset":4528,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":596,"record_offset":4724,"parent_record_offset":4692,"ancestor_record_offset":4528,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":5256,"parent_record_offset":5232,"ancestor_record_offset":4528,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":5660,"parent_record_offset":4800,"ancestor_record_offset":4528,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":5232,"parent_record_offset":5208,"ancestor_record_offset":4528,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":4776,"parent_record_offset":4752,"ancestor_record_offset":4528,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":4752,"parent_record_offset":4692,"ancestor_record_offset":4528,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":4800,"parent_record_offset":4776,"ancestor_record_offset":4528,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":4880,"parent_record_offset":4800,"ancestor_record_offset":4528,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":5208,"parent_record_offset":4800,"ancestor_record_offset":4528,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::Mesh::collision_bbx
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 172
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x945D
// 首条位置: {"module":596,"record_offset":7512,"rva":12754464,"va":5381463584}
// 编译实例 RVAs: 0xc29e20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::aabb_bounds
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381463584,"raw_hex":"e89afcffff","size":5,"soft_fail":false,"target_va":5381462752,"va":5381463617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb3::center
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1839607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381463584,"raw_hex":"e8f7111c00","size":5,"soft_fail":false,"target_va":5383303280,"va":5381463668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::aabb::Aabb3::extent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1839805","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381463584,"raw_hex":"e8bd121c00","size":5,"soft_fail":false,"target_va":5383303440,"va":5381463630,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] utils::geometry::rectangle::Rectangle2d::new_axis_aligned
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"utils","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1850667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381463584,"raw_hex":"e82b3d1c00","size":5,"soft_fail":false,"target_va":5383314384,"va":5381463712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::Mesh::positions
// 记录形式: compiled_procedure; instances=1
// 源码行: 186
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":596,"record_offset":17448,"rva":12757472,"va":5381466592}
// 编译实例 RVAs: 0xc2a9e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cfc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7985746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381466592,"raw_hex":"e8ae2586ff","size":5,"soft_fail":false,"target_va":5373480896,"va":5381466637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35154712","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381466592,"raw_hex":"e8186b1802","size":5,"soft_fail":false,"target_va":5416621600,"va":5381466883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::vertex::VertexAttributeValues::as_f32x3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t2220846","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381466592,"raw_hex":"e82ee32100","size":5,"soft_fail":false,"target_va":5383687680,"va":5381466829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":596,"record_offset":18716,"parent_record_offset":17448,"ancestor_record_offset":17448,"inlinee":7311,"source_line":789,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":596,"record_offset":18772,"parent_record_offset":17448,"ancestor_record_offset":17448,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":596,"record_offset":18632,"parent_record_offset":17448,"ancestor_record_offset":17448,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":596,"record_offset":17588,"parent_record_offset":17448,"ancestor_record_offset":17448,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":596,"record_offset":18292,"parent_record_offset":18264,"ancestor_record_offset":17448,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":596,"record_offset":17852,"parent_record_offset":17828,"ancestor_record_offset":17448,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":596,"record_offset":17936,"parent_record_offset":17912,"ancestor_record_offset":17448,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":596,"record_offset":18460,"parent_record_offset":17720,"ancestor_record_offset":17448,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":596,"record_offset":18740,"parent_record_offset":18716,"ancestor_record_offset":17448,"inlinee":7313,"source_line":68,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":596,"record_offset":18084,"parent_record_offset":18056,"ancestor_record_offset":17448,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":596,"record_offset":18056,"parent_record_offset":18028,"ancestor_record_offset":17448,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":596,"record_offset":17876,"parent_record_offset":17852,"ancestor_record_offset":17448,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":596,"record_offset":18680,"parent_record_offset":18656,"ancestor_record_offset":17448,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":596,"record_offset":18200,"parent_record_offset":18176,"ancestor_record_offset":17448,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":596,"record_offset":18376,"parent_record_offset":18348,"ancestor_record_offset":17448,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":596,"record_offset":18404,"parent_record_offset":18376,"ancestor_record_offset":17448,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":596,"record_offset":18656,"parent_record_offset":18632,"ancestor_record_offset":17448,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":596,"record_offset":18320,"parent_record_offset":18292,"ancestor_record_offset":17448,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":596,"record_offset":17996,"parent_record_offset":17968,"ancestor_record_offset":17448,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":596,"record_offset":18028,"parent_record_offset":17996,"ancestor_record_offset":17448,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":596,"record_offset":17968,"parent_record_offset":17720,"ancestor_record_offset":17448,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":596,"record_offset":17828,"parent_record_offset":17720,"ancestor_record_offset":17448,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":596,"record_offset":18488,"parent_record_offset":17720,"ancestor_record_offset":17448,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":596,"record_offset":17912,"parent_record_offset":17720,"ancestor_record_offset":17448,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":596,"record_offset":17772,"parent_record_offset":17720,"ancestor_record_offset":17448,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":596,"record_offset":18264,"parent_record_offset":18236,"ancestor_record_offset":17448,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":596,"record_offset":17612,"parent_record_offset":17588,"ancestor_record_offset":17448,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":596,"record_offset":18236,"parent_record_offset":18128,"ancestor_record_offset":17448,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":596,"record_offset":17644,"parent_record_offset":17612,"ancestor_record_offset":17448,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":18176,"parent_record_offset":18152,"ancestor_record_offset":17448,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":18580,"parent_record_offset":17720,"ancestor_record_offset":17448,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":18152,"parent_record_offset":18128,"ancestor_record_offset":17448,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":17696,"parent_record_offset":17672,"ancestor_record_offset":17448,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":17672,"parent_record_offset":17612,"ancestor_record_offset":17448,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":17720,"parent_record_offset":17696,"ancestor_record_offset":17448,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":17800,"parent_record_offset":17720,"ancestor_record_offset":17448,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":18128,"parent_record_offset":17720,"ancestor_record_offset":17448,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::impl$1::positions::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=17
// 源码行: 196
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8FB5
// 首条位置: {"module":578,"inlinee":36789}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::Mesh::positions_without_indices
// 记录形式: compiled_procedure; instances=1
// 源码行: 200
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":596,"record_offset":12988,"rva":12756288,"va":5381465408}
// 编译实例 RVAs: 0xc2a540
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cfc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7984555","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381465408,"raw_hex":"e8552a86ff","size":5,"soft_fail":false,"target_va":5373480896,"va":5381465446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35155938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381465408,"raw_hex":"e8e26f1802","size":5,"soft_fail":false,"target_va":5416621600,"va":5381465657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::vertex::VertexAttributeValues::as_f32x3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t2222046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381465408,"raw_hex":"e8dee72100","size":5,"soft_fail":false,"target_va":5383687680,"va":5381465629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":596,"record_offset":14268,"parent_record_offset":12988,"ancestor_record_offset":12988,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":596,"record_offset":14184,"parent_record_offset":12988,"ancestor_record_offset":12988,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":596,"record_offset":13140,"parent_record_offset":12988,"ancestor_record_offset":12988,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":596,"record_offset":13844,"parent_record_offset":13816,"ancestor_record_offset":12988,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":596,"record_offset":13404,"parent_record_offset":13380,"ancestor_record_offset":12988,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":596,"record_offset":13488,"parent_record_offset":13464,"ancestor_record_offset":12988,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":596,"record_offset":14012,"parent_record_offset":13272,"ancestor_record_offset":12988,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":596,"record_offset":13636,"parent_record_offset":13608,"ancestor_record_offset":12988,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":596,"record_offset":13608,"parent_record_offset":13580,"ancestor_record_offset":12988,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":596,"record_offset":13428,"parent_record_offset":13404,"ancestor_record_offset":12988,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":596,"record_offset":14232,"parent_record_offset":14208,"ancestor_record_offset":12988,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":596,"record_offset":13752,"parent_record_offset":13728,"ancestor_record_offset":12988,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":596,"record_offset":13928,"parent_record_offset":13900,"ancestor_record_offset":12988,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":596,"record_offset":13956,"parent_record_offset":13928,"ancestor_record_offset":12988,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":596,"record_offset":14208,"parent_record_offset":14184,"ancestor_record_offset":12988,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":596,"record_offset":13872,"parent_record_offset":13844,"ancestor_record_offset":12988,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":596,"record_offset":13548,"parent_record_offset":13520,"ancestor_record_offset":12988,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":596,"record_offset":13580,"parent_record_offset":13548,"ancestor_record_offset":12988,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":596,"record_offset":13520,"parent_record_offset":13272,"ancestor_record_offset":12988,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":596,"record_offset":13380,"parent_record_offset":13272,"ancestor_record_offset":12988,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":596,"record_offset":14040,"parent_record_offset":13272,"ancestor_record_offset":12988,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":596,"record_offset":13464,"parent_record_offset":13272,"ancestor_record_offset":12988,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":596,"record_offset":13324,"parent_record_offset":13272,"ancestor_record_offset":12988,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":596,"record_offset":13816,"parent_record_offset":13788,"ancestor_record_offset":12988,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":596,"record_offset":13164,"parent_record_offset":13140,"ancestor_record_offset":12988,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":596,"record_offset":13788,"parent_record_offset":13680,"ancestor_record_offset":12988,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":596,"record_offset":13196,"parent_record_offset":13164,"ancestor_record_offset":12988,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":13728,"parent_record_offset":13704,"ancestor_record_offset":12988,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":14132,"parent_record_offset":13272,"ancestor_record_offset":12988,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":13704,"parent_record_offset":13680,"ancestor_record_offset":12988,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":13248,"parent_record_offset":13224,"ancestor_record_offset":12988,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":13224,"parent_record_offset":13164,"ancestor_record_offset":12988,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":13272,"parent_record_offset":13248,"ancestor_record_offset":12988,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":13352,"parent_record_offset":13272,"ancestor_record_offset":12988,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":13680,"parent_record_offset":13272,"ancestor_record_offset":12988,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::Mesh::normals_without_indices
// 记录形式: compiled_procedure; instances=1
// 源码行: 211
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":596,"record_offset":11708,"rva":12756016,"va":5381465136}
// 编译实例 RVAs: 0xc2a430
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cfc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7984288","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381465136,"raw_hex":"e8602b86ff","size":5,"soft_fail":false,"target_va":5373480896,"va":5381465179,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::vertex::VertexAttributeValues::as_f32x3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t2222302","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381465136,"raw_hex":"e8dee82100","size":5,"soft_fail":false,"target_va":5383687680,"va":5381465373,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":596,"record_offset":12900,"parent_record_offset":11708,"ancestor_record_offset":11708,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":596,"record_offset":11856,"parent_record_offset":11708,"ancestor_record_offset":11708,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":596,"record_offset":12560,"parent_record_offset":12532,"ancestor_record_offset":11708,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":596,"record_offset":12120,"parent_record_offset":12096,"ancestor_record_offset":11708,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":596,"record_offset":12204,"parent_record_offset":12180,"ancestor_record_offset":11708,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":596,"record_offset":12728,"parent_record_offset":11988,"ancestor_record_offset":11708,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":596,"record_offset":12352,"parent_record_offset":12324,"ancestor_record_offset":11708,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":596,"record_offset":12324,"parent_record_offset":12296,"ancestor_record_offset":11708,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":596,"record_offset":12144,"parent_record_offset":12120,"ancestor_record_offset":11708,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":596,"record_offset":12948,"parent_record_offset":12924,"ancestor_record_offset":11708,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":596,"record_offset":12468,"parent_record_offset":12444,"ancestor_record_offset":11708,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":596,"record_offset":12644,"parent_record_offset":12616,"ancestor_record_offset":11708,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":596,"record_offset":12672,"parent_record_offset":12644,"ancestor_record_offset":11708,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":596,"record_offset":12924,"parent_record_offset":12900,"ancestor_record_offset":11708,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":596,"record_offset":12588,"parent_record_offset":12560,"ancestor_record_offset":11708,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":596,"record_offset":12264,"parent_record_offset":12236,"ancestor_record_offset":11708,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":596,"record_offset":12296,"parent_record_offset":12264,"ancestor_record_offset":11708,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":596,"record_offset":12236,"parent_record_offset":11988,"ancestor_record_offset":11708,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":596,"record_offset":12096,"parent_record_offset":11988,"ancestor_record_offset":11708,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":596,"record_offset":12756,"parent_record_offset":11988,"ancestor_record_offset":11708,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":596,"record_offset":12180,"parent_record_offset":11988,"ancestor_record_offset":11708,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":596,"record_offset":12040,"parent_record_offset":11988,"ancestor_record_offset":11708,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":596,"record_offset":12532,"parent_record_offset":12504,"ancestor_record_offset":11708,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":596,"record_offset":11880,"parent_record_offset":11856,"ancestor_record_offset":11708,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":596,"record_offset":12504,"parent_record_offset":12396,"ancestor_record_offset":11708,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":596,"record_offset":11912,"parent_record_offset":11880,"ancestor_record_offset":11708,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":12444,"parent_record_offset":12420,"ancestor_record_offset":11708,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":12848,"parent_record_offset":11988,"ancestor_record_offset":11708,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":12420,"parent_record_offset":12396,"ancestor_record_offset":11708,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":11964,"parent_record_offset":11940,"ancestor_record_offset":11708,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":11940,"parent_record_offset":11880,"ancestor_record_offset":11708,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":11988,"parent_record_offset":11964,"ancestor_record_offset":11708,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":12068,"parent_record_offset":11988,"ancestor_record_offset":11708,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":12396,"parent_record_offset":11988,"ancestor_record_offset":11708,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::Mesh::uvs_without_indices
// 记录形式: compiled_procedure; instances=1
// 源码行: 218
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":596,"record_offset":10432,"rva":12755760,"va":5381464880}
// 编译实例 RVAs: 0xc2a330
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cfc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7984030","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381464880,"raw_hex":"e8622c86ff","size":5,"soft_fail":false,"target_va":5373480896,"va":5381464921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::vertex::VertexAttributeValues::as_f32x2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t2222526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381464880,"raw_hex":"e8bee92100","size":5,"soft_fail":false,"target_va":5383687632,"va":5381465101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":596,"record_offset":11620,"parent_record_offset":10432,"ancestor_record_offset":10432,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":596,"record_offset":10576,"parent_record_offset":10432,"ancestor_record_offset":10432,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":596,"record_offset":11280,"parent_record_offset":11252,"ancestor_record_offset":10432,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":596,"record_offset":10840,"parent_record_offset":10816,"ancestor_record_offset":10432,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":596,"record_offset":10924,"parent_record_offset":10900,"ancestor_record_offset":10432,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":596,"record_offset":11448,"parent_record_offset":10708,"ancestor_record_offset":10432,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":596,"record_offset":11072,"parent_record_offset":11044,"ancestor_record_offset":10432,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":596,"record_offset":11044,"parent_record_offset":11016,"ancestor_record_offset":10432,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":596,"record_offset":10864,"parent_record_offset":10840,"ancestor_record_offset":10432,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":596,"record_offset":11668,"parent_record_offset":11644,"ancestor_record_offset":10432,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":596,"record_offset":11188,"parent_record_offset":11164,"ancestor_record_offset":10432,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":596,"record_offset":11364,"parent_record_offset":11336,"ancestor_record_offset":10432,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":596,"record_offset":11392,"parent_record_offset":11364,"ancestor_record_offset":10432,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":596,"record_offset":11644,"parent_record_offset":11620,"ancestor_record_offset":10432,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":596,"record_offset":11308,"parent_record_offset":11280,"ancestor_record_offset":10432,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":596,"record_offset":10984,"parent_record_offset":10956,"ancestor_record_offset":10432,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":596,"record_offset":11016,"parent_record_offset":10984,"ancestor_record_offset":10432,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":596,"record_offset":10956,"parent_record_offset":10708,"ancestor_record_offset":10432,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":596,"record_offset":10816,"parent_record_offset":10708,"ancestor_record_offset":10432,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":596,"record_offset":11476,"parent_record_offset":10708,"ancestor_record_offset":10432,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":596,"record_offset":10900,"parent_record_offset":10708,"ancestor_record_offset":10432,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":596,"record_offset":10760,"parent_record_offset":10708,"ancestor_record_offset":10432,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":596,"record_offset":11252,"parent_record_offset":11224,"ancestor_record_offset":10432,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":596,"record_offset":10600,"parent_record_offset":10576,"ancestor_record_offset":10432,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":596,"record_offset":11224,"parent_record_offset":11116,"ancestor_record_offset":10432,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":596,"record_offset":10632,"parent_record_offset":10600,"ancestor_record_offset":10432,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":11164,"parent_record_offset":11140,"ancestor_record_offset":10432,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":11568,"parent_record_offset":10708,"ancestor_record_offset":10432,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":11140,"parent_record_offset":11116,"ancestor_record_offset":10432,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":10684,"parent_record_offset":10660,"ancestor_record_offset":10432,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":10660,"parent_record_offset":10600,"ancestor_record_offset":10432,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":10708,"parent_record_offset":10684,"ancestor_record_offset":10432,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":10788,"parent_record_offset":10708,"ancestor_record_offset":10432,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":11116,"parent_record_offset":10708,"ancestor_record_offset":10432,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::Mesh::retain_vertices
// 记录形式: compiled_procedure; instances=1
// 源码行: 228
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":596,"record_offset":7980,"rva":12754752,"va":5381463872}
// 编译实例 RVAs: 0xc29f40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141a34370
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14721900","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381463872,"raw_hex":"e86ca3e000","size":5,"soft_fail":false,"target_va":5396185968,"va":5381464063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t35592388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381463872,"raw_hex":"e8c4181f02","size":5,"soft_fail":false,"target_va":5417057072,"va":5381464679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35157657","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381463872,"raw_hex":"e899761802","size":5,"soft_fail":false,"target_va":5416622208,"va":5381464546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t35157196","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381463872,"raw_hex":"e8cc741802","size":5,"soft_fail":false,"target_va":5416621762,"va":5381464561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::retain
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":596,"record_offset":8128,"parent_record_offset":7980,"ancestor_record_offset":7980,"inlinee":11071,"source_line":2425,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":596,"record_offset":9660,"parent_record_offset":7980,"ancestor_record_offset":7980,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::values_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":596,"record_offset":8676,"parent_record_offset":7980,"ancestor_record_offset":7980,"inlinee":20111,"source_line":576,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$55::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":596,"record_offset":9028,"parent_record_offset":7980,"ancestor_record_offset":7980,"inlinee":21910,"source_line":2271,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":596,"record_offset":8240,"parent_record_offset":8168,"ancestor_record_offset":7980,"inlinee":4800,"source_line":2996,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::retain_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":596,"record_offset":8168,"parent_record_offset":8128,"ancestor_record_offset":7980,"inlinee":10770,"source_line":2451,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":596,"record_offset":8488,"parent_record_offset":8168,"ancestor_record_offset":7980,"inlinee":4802,"source_line":2161,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":596,"record_offset":8268,"parent_record_offset":8168,"ancestor_record_offset":7980,"inlinee":10772,"source_line":2429,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$1::retain_mut::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":596,"record_offset":8540,"parent_record_offset":8516,"ancestor_record_offset":7980,"inlinee":4928,"source_line":2481,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":596,"record_offset":8944,"parent_record_offset":8920,"ancestor_record_offset":7980,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::hint::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":596,"record_offset":8340,"parent_record_offset":8168,"ancestor_record_offset":7980,"inlinee":10761,"source_line":713,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":596,"record_offset":8364,"parent_record_offset":8340,"ancestor_record_offset":7980,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":596,"record_offset":9372,"parent_record_offset":9344,"ancestor_record_offset":7980,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":596,"record_offset":9344,"parent_record_offset":9316,"ancestor_record_offset":7980,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":596,"record_offset":8976,"parent_record_offset":8892,"ancestor_record_offset":7980,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":596,"record_offset":8604,"parent_record_offset":8540,"ancestor_record_offset":7980,"inlinee":4801,"source_line":623,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":596,"record_offset":8460,"parent_record_offset":8168,"ancestor_record_offset":7980,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":596,"record_offset":8516,"parent_record_offset":8168,"ancestor_record_offset":7980,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":596,"record_offset":8576,"parent_record_offset":8540,"ancestor_record_offset":7980,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":596,"record_offset":9468,"parent_record_offset":9444,"ancestor_record_offset":7980,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":596,"record_offset":9728,"parent_record_offset":9660,"ancestor_record_offset":7980,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":596,"record_offset":9700,"parent_record_offset":9660,"ancestor_record_offset":7980,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$1::retain_vertices::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":596,"record_offset":8300,"parent_record_offset":8268,"ancestor_record_offset":7980,"inlinee":37945,"source_line":229,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":596,"record_offset":9276,"parent_record_offset":9236,"ancestor_record_offset":7980,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":596,"record_offset":9316,"parent_record_offset":9276,"ancestor_record_offset":7980,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":596,"record_offset":9412,"parent_record_offset":9236,"ancestor_record_offset":7980,"inlinee":4664,"source_line":37,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":596,"record_offset":9236,"parent_record_offset":9188,"ancestor_record_offset":7980,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":596,"record_offset":8920,"parent_record_offset":8892,"ancestor_record_offset":7980,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":596,"record_offset":8748,"parent_record_offset":8712,"ancestor_record_offset":7980,"inlinee":20113,"source_line":799,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$48::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":596,"record_offset":9100,"parent_record_offset":9064,"ancestor_record_offset":7980,"inlinee":13411,"source_line":3267,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":9444,"parent_record_offset":9188,"ancestor_record_offset":7980,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":8892,"parent_record_offset":8820,"ancestor_record_offset":7980,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":9188,"parent_record_offset":9140,"ancestor_record_offset":7980,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":8784,"parent_record_offset":8748,"ancestor_record_offset":7980,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":8864,"parent_record_offset":8820,"ancestor_record_offset":7980,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":8820,"parent_record_offset":8784,"ancestor_record_offset":7980,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":9140,"parent_record_offset":9100,"ancestor_record_offset":7980,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":596,"record_offset":8712,"parent_record_offset":8676,"ancestor_record_offset":7980,"inlinee":20112,"source_line":673,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$41::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":596,"record_offset":9064,"parent_record_offset":9028,"ancestor_record_offset":7980,"inlinee":21911,"source_line":2113,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::impl$1::retain_vertices::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 229
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9439
// 首条位置: {"module":596,"inlinee":37945}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::impl$1::retain_vertices::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 232
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13D48
// 首条位置: {"module":1643,"inlinee":81224}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::Mesh::clear
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 254
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8184
// 首条位置: {"module":539,"inlinee":33156}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":588,"record_offset":112484,"parent_record_offset":112396,"ancestor_record_offset":112396,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":588,"record_offset":112428,"parent_record_offset":112396,"ancestor_record_offset":112396,"inlinee":21905,"source_line":829,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::clear
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":588,"record_offset":112452,"parent_record_offset":112428,"ancestor_record_offset":112396,"inlinee":19433,"source_line":1009,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::Mesh::collision_radius_xz
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 267
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9460
// 首条位置: {"module":596,"record_offset":9828,"rva":12755600,"va":5381464720}
// 编译实例 RVAs: 0xc2a290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35156893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381464720,"raw_hex":"e89d731802","size":5,"soft_fail":false,"target_va":5416621762,"va":5381464864,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::mesh::Mesh::positions
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1855","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381464720,"raw_hex":"e83f070000","size":5,"soft_fail":false,"target_va":5381466592,"va":5381464732,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::f32::impl$0::max
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":596,"record_offset":10400,"parent_record_offset":9828,"ancestor_record_offset":9828,"inlinee":5477,"source_line":991,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\map.rs
//     evidence={"module":596,"record_offset":9976,"parent_record_offset":9828,"ancestor_record_offset":9828,"inlinee":5118,"source_line":106,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] glam::f32::vec2::Vec2::length
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":596,"record_offset":10280,"parent_record_offset":9828,"ancestor_record_offset":9828,"inlinee":9813,"source_line":461,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::f32::math::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\f32.rs
//     evidence={"module":596,"record_offset":10360,"parent_record_offset":10336,"ancestor_record_offset":9828,"inlinee":9781,"source_line":2040,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::copied::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\copied.rs
//     evidence={"module":596,"record_offset":10008,"parent_record_offset":9976,"ancestor_record_offset":9828,"inlinee":7432,"source_line":51,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":596,"record_offset":10204,"parent_record_offset":10172,"ancestor_record_offset":9828,"inlinee":5124,"source_line":309,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::copied
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":596,"record_offset":10140,"parent_record_offset":10008,"ancestor_record_offset":9828,"inlinee":7434,"source_line":2119,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":596,"record_offset":10172,"parent_record_offset":9976,"ancestor_record_offset":9828,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":596,"record_offset":10108,"parent_record_offset":10040,"ancestor_record_offset":9828,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":596,"record_offset":10076,"parent_record_offset":10040,"ancestor_record_offset":9828,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":596,"record_offset":10040,"parent_record_offset":10008,"ancestor_record_offset":9828,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::mesh::impl$1::positions::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mesh.rs
//     evidence={"module":596,"record_offset":10236,"parent_record_offset":10204,"ancestor_record_offset":9828,"inlinee":36789,"source_line":196,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] glam::f32::vec2::Vec2::dot
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"glam","version":"0.30.10","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/glam-0.30.10/src/f32/vec2.rs
//     evidence={"module":596,"record_offset":10304,"parent_record_offset":10280,"ancestor_record_offset":9828,"inlinee":9814,"source_line":185,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] std::f32::impl$0::sqrt
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\num\f32.rs
//     evidence={"module":596,"record_offset":10336,"parent_record_offset":10304,"ancestor_record_offset":9828,"inlinee":9779,"source_line":384,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::Mesh::as_triangles
// 记录形式: compiled_procedure; instances=1
// 源码行: 277
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":596,"record_offset":6092,"rva":12754080,"va":5381463200}
// 编译实例 RVAs: 0xc29ca0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14048cfc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7982354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381463200,"raw_hex":"e8ee3286ff","size":5,"soft_fail":false,"target_va":5373480896,"va":5381463245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141608a50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mesh.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10349658","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381463200,"raw_hex":"e85aec9d00","size":5,"soft_fail":false,"target_va":5391813200,"va":5381463537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t35158031","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381463200,"raw_hex":"e80f781802","size":5,"soft_fail":false,"target_va":5416621600,"va":5381463564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::vertex::VertexAttributeValues::as_f32x3
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t2224234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381463200,"raw_hex":"e86af02100","size":5,"soft_fail":false,"target_va":5383687680,"va":5381463441,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":596,"record_offset":7396,"parent_record_offset":6092,"ancestor_record_offset":6092,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":596,"record_offset":7480,"parent_record_offset":6092,"ancestor_record_offset":6092,"inlinee":4507,"source_line":1013,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::chunks_exact
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":596,"record_offset":7280,"parent_record_offset":6092,"ancestor_record_offset":6092,"inlinee":27399,"source_line":1242,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":596,"record_offset":6236,"parent_record_offset":6092,"ancestor_record_offset":6092,"inlinee":7239,"source_line":997,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":596,"record_offset":7420,"parent_record_offset":7396,"ancestor_record_offset":6092,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":596,"record_offset":6940,"parent_record_offset":6912,"ancestor_record_offset":6092,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":596,"record_offset":6500,"parent_record_offset":6476,"ancestor_record_offset":6092,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":596,"record_offset":6584,"parent_record_offset":6560,"ancestor_record_offset":6092,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":596,"record_offset":7108,"parent_record_offset":6368,"ancestor_record_offset":6092,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":596,"record_offset":7444,"parent_record_offset":7420,"ancestor_record_offset":6092,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":596,"record_offset":6732,"parent_record_offset":6704,"ancestor_record_offset":6092,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":596,"record_offset":6704,"parent_record_offset":6676,"ancestor_record_offset":6092,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":596,"record_offset":7356,"parent_record_offset":7332,"ancestor_record_offset":6092,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":596,"record_offset":6524,"parent_record_offset":6500,"ancestor_record_offset":6092,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":596,"record_offset":6848,"parent_record_offset":6824,"ancestor_record_offset":6092,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":596,"record_offset":7024,"parent_record_offset":6996,"ancestor_record_offset":6092,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":596,"record_offset":7052,"parent_record_offset":7024,"ancestor_record_offset":6092,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::split_at_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":596,"record_offset":7332,"parent_record_offset":7304,"ancestor_record_offset":6092,"inlinee":11960,"source_line":2038,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::ChunksExact::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":596,"record_offset":7304,"parent_record_offset":7280,"ancestor_record_offset":6092,"inlinee":27401,"source_line":1851,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":596,"record_offset":6968,"parent_record_offset":6940,"ancestor_record_offset":6092,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":596,"record_offset":6644,"parent_record_offset":6616,"ancestor_record_offset":6092,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":596,"record_offset":6676,"parent_record_offset":6644,"ancestor_record_offset":6092,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":596,"record_offset":6616,"parent_record_offset":6368,"ancestor_record_offset":6092,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":596,"record_offset":6476,"parent_record_offset":6368,"ancestor_record_offset":6092,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":596,"record_offset":7136,"parent_record_offset":6368,"ancestor_record_offset":6092,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":596,"record_offset":6560,"parent_record_offset":6368,"ancestor_record_offset":6092,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":596,"record_offset":6420,"parent_record_offset":6368,"ancestor_record_offset":6092,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":596,"record_offset":6912,"parent_record_offset":6884,"ancestor_record_offset":6092,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":596,"record_offset":6260,"parent_record_offset":6236,"ancestor_record_offset":6092,"inlinee":4299,"source_line":1304,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":596,"record_offset":6884,"parent_record_offset":6776,"ancestor_record_offset":6092,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":596,"record_offset":6292,"parent_record_offset":6260,"ancestor_record_offset":6092,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":6824,"parent_record_offset":6800,"ancestor_record_offset":6092,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":7228,"parent_record_offset":6368,"ancestor_record_offset":6092,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":6800,"parent_record_offset":6776,"ancestor_record_offset":6092,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":6344,"parent_record_offset":6320,"ancestor_record_offset":6092,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":6320,"parent_record_offset":6260,"ancestor_record_offset":6092,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":6368,"parent_record_offset":6344,"ancestor_record_offset":6092,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":6448,"parent_record_offset":6368,"ancestor_record_offset":6092,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":596,"record_offset":6776,"parent_record_offset":6368,"ancestor_record_offset":6092,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00596.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::render::mesh::impl$1::as_triangles::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 286
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14362
// 首条位置: {"module":1682,"inlinee":82786}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: mesh.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::render::mesh::Mesh
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 24; 过程引用数: 110
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "constructor": "country_core::asset_libraries::Handle", "arguments": ["country_core::render::mesh::Mesh"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>", "constructor": "alloc::boxed::Box", "arguments": ["country_core::render::mesh::Mesh", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global> >", "constructor": "tuple$", "arguments": ["country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<alloc::string::String,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >", "constructor": "tuple$", "arguments": ["alloc::string::String", "country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<alloc::string::String,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<alloc::string::String,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >", "constructor": "tuple$", "arguments": ["std::path::PathBuf", "country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::vao_library::MeshVaos>", "constructor": "tuple$", "arguments": ["country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "country_core::asset_libraries::vao_library::MeshVaos"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::vao_library::MeshVaos>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::vao_library::MeshVaos>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::poison::mutex::MutexGuard<alloc::vec::Vec<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::alloc::Global> >", "constructor": "std::sync::poison::mutex::MutexGuard", "arguments": ["alloc::vec::Vec<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::poison::mutex::MutexGuard<alloc::vec::Vec<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::poison::mutex::MutexGuard<alloc::vec::Vec<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::set::HashSet<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "std::collections::hash::set::HashSet", "arguments": ["country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,std::collections::hash::set::HashSet<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,std::hash::random::RandomState,alloc::alloc::Global> >", "constructor": "lock_api::rwlock::RwLock", "arguments": ["parking_lot::raw_rwlock::RawRwLock", "std::collections::hash::set::HashSet<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,std::hash::random::RandomState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,std::collections::hash::set::HashSet<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,std::hash::random::RandomState,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,std::collections::hash::set::HashSet<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,std::hash::random::RandomState,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["country_core::render::mesh::Mesh", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::VacantEntry<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "std::collections::hash::map::VacantEntry", "arguments": ["tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >", "alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::map::VacantEntry<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::map::VacantEntry<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["std::path::PathBuf", "country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,std::collections::hash::map::HashMap<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,std::hash::random::RandomState,alloc::alloc::Global> >", "constructor": "lock_api::rwlock::RwLock", "arguments": ["parking_lot::raw_rwlock::RawRwLock", "std::collections::hash::map::HashMap<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,std::hash::random::RandomState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,std::collections::hash::map::HashMap<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,std::hash::random::RandomState,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,std::collections::hash::map::HashMap<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,std::hash::random::RandomState,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<alloc::string::String,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["alloc::string::String", "country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,std::collections::hash::map::HashMap<alloc::string::String,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,std::hash::random::RandomState,alloc::alloc::Global> >", "constructor": "lock_api::rwlock::RwLock", "arguments": ["parking_lot::raw_rwlock::RawRwLock", "std::collections::hash::map::HashMap<alloc::string::String,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,std::hash::random::RandomState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,std::collections::hash::map::HashMap<alloc::string::String,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,std::hash::random::RandomState,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,std::collections::hash::map::HashMap<alloc::string::String,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,std::hash::random::RandomState,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::render::mesh::Mesh>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::render::mesh::Mesh"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::render::mesh::Mesh,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::render::mesh::Mesh", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::render::mesh::Mesh,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::render::mesh::Mesh,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::ArcInner<country_core::render::mesh::Mesh>", "constructor": "alloc::sync::ArcInner", "arguments": ["country_core::render::mesh::Mesh"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::ArcInner<country_core::render::mesh::Mesh> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::ArcInner<country_core::render::mesh::Mesh>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::asset_libraries::Asset<country_core::render::mesh::Mesh>", "constructor": "country_core::asset_libraries::Asset", "arguments": ["country_core::render::mesh::Mesh"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::asset_libraries::Asset<country_core::render::mesh::Mesh> >", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::asset_libraries::Asset<country_core::render::mesh::Mesh>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::rwlock::RwLockReadGuard<parking_lot::raw_rwlock::RawRwLock,alloc::vec::Vec<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::alloc::Global> >", "constructor": "lock_api::rwlock::RwLockReadGuard", "arguments": ["parking_lot::raw_rwlock::RawRwLock", "alloc::vec::Vec<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::rwlock::RwLockReadGuard<parking_lot::raw_rwlock::RawRwLock,alloc::vec::Vec<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::rwlock::RwLockReadGuard<parking_lot::raw_rwlock::RawRwLock,alloc::vec::Vec<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,4>,alloc::alloc::Global> >", "constructor": "country_core::render::mesh::Mesh::set_attribute", "arguments": ["alloc::vec::Vec<array$<f32,4>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::mesh::Mesh::set_attribute<enum2$<rhapsody::vertex::VertexAttributeValues> >", "constructor": "country_core::render::mesh::Mesh::set_attribute", "arguments": ["enum2$<rhapsody::vertex::VertexAttributeValues>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<i32,alloc::alloc::Global> >", "constructor": "country_core::render::mesh::Mesh::set_attribute", "arguments": ["alloc::vec::Vec<i32,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,2>,alloc::alloc::Global> >", "constructor": "country_core::render::mesh::Mesh::set_attribute", "arguments": ["alloc::vec::Vec<array$<f32,2>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<f32,alloc::alloc::Global> >", "constructor": "country_core::render::mesh::Mesh::set_attribute", "arguments": ["alloc::vec::Vec<f32,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,3>,alloc::alloc::Global> >", "constructor": "country_core::render::mesh::Mesh::set_attribute", "arguments": ["alloc::vec::Vec<array$<f32,3>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::vertex::impl$8::encode_buffers::closure_env$0<country_core::render::mesh::Mesh>", "constructor": "rhapsody::vertex::impl$8::encode_buffers::closure_env$0", "arguments": ["country_core::render::mesh::Mesh"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::vertex::InterleavedVertexBufferLayout>,rhapsody::vertex::impl$8::encode_buffers::closure_env$0<country_core::render::mesh::Mesh> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<rhapsody::vertex::InterleavedVertexBufferLayout>", "rhapsody::vertex::impl$8::encode_buffers::closure_env$0<country_core::render::mesh::Mesh>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::impl$26::from_iter::closure_env$0<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::vertex::VertexBufferEncodeError>,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::vertex::InterleavedVertexBufferLayout>,rhapsody::vertex::impl$8::encode_buffers::closure_env$0<country_core::render::mesh::Mesh> > >", "constructor": "core::result::impl$26::from_iter::closure_env$0", "arguments": ["rhapsody::raw::buffer::Buffer", "enum2$<rhapsody::vertex::VertexBufferEncodeError>", "alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>", "core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::vertex::InterleavedVertexBufferLayout>,rhapsody::vertex::impl$8::encode_buffers::closure_env$0<country_core::render::mesh::Mesh> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::try_process<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::vertex::InterleavedVertexBufferLayout>,rhapsody::vertex::impl$8::encode_buffers::closure_env$0<country_core::render::mesh::Mesh> >,rhapsody::raw::buffer::Buffer,enum2$<core::result::Result<enum2$<core::convert::Infallible>,enum2$<rhapsody::vertex::VertexBufferEncodeError> > >,core::result::impl$26::from_iter::closure_env$0<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::vertex::VertexBufferEncodeError>,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::vertex::InterleavedVertexBufferLayout>,rhapsody::vertex::impl$8::encode_buffers::closure_env$0<country_core::render::mesh::Mesh> > >,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >", "constructor": "core::iter::adapters::try_process", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::vertex::InterleavedVertexBufferLayout>,rhapsody::vertex::impl$8::encode_buffers::closure_env$0<country_core::render::mesh::Mesh> >", "rhapsody::raw::buffer::Buffer", "enum2$<core::result::Result<enum2$<core::convert::Infallible>,enum2$<rhapsody::vertex::VertexBufferEncodeError> > >", "core::result::impl$26::from_iter::closure_env$0<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::vertex::VertexBufferEncodeError>,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::vertex::InterleavedVertexBufferLayout>,rhapsody::vertex::impl$8::encode_buffers::closure_env$0<country_core::render::mesh::Mesh> > >", "alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "frozen::FrozenMap::get<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >", "constructor": "frozen::FrozenMap::get", "arguments": ["country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>", "std::hash::random::RandomState", "country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "frozen::FrozenMap::insert<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState>", "constructor": "frozen::FrozenMap::insert", "arguments": ["country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<country_core::render::mesh::Mesh,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["country_core::render::mesh::Mesh", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >", "constructor": "tuple$", "arguments": ["tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >", "alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >", "alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1", "arguments": ["tuple$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState> >,tuple$<ptr_mut$<u8> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState> >", "tuple$<ptr_mut$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::equivalent_key::closure_env$0<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::vao_library::MeshVaos>", "constructor": "hashbrown::map::equivalent_key::closure_env$0", "arguments": ["country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "country_core::asset_libraries::vao_library::MeshVaos"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::remove_entry<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::vao_library::MeshVaos>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::vao_library::MeshVaos> >", "constructor": "hashbrown::raw::RawTable::remove_entry", "arguments": ["tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::vao_library::MeshVaos>", "alloc::alloc::Global", "hashbrown::map::equivalent_key::closure_env$0<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::vao_library::MeshVaos>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::equivalent_key::closure_env$0<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global> >", "constructor": "hashbrown::map::equivalent_key::closure_env$0", "arguments": ["country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::remove_entry<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global> > >", "constructor": "hashbrown::raw::RawTable::remove_entry", "arguments": ["tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global> >", "alloc::alloc::Global", "hashbrown::map::equivalent_key::closure_env$0<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,tuple$<> >", "constructor": "tuple$", "arguments": ["country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::equivalent_key::closure_env$0<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,tuple$<> >", "constructor": "hashbrown::map::equivalent_key::closure_env$0", "arguments": ["country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::remove_entry<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,tuple$<> >,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,tuple$<> > >", "constructor": "hashbrown::raw::RawTable::remove_entry", "arguments": ["tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,tuple$<> >", "alloc::alloc::Global", "hashbrown::map::equivalent_key::closure_env$0<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,tuple$<> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,tuple$<>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "tuple$<>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,tuple$<> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,tuple$<>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,tuple$<> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,tuple$<>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<alloc::string::String,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["alloc::string::String", "country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<alloc::string::String,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<alloc::string::String,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<alloc::string::String,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<alloc::string::String,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::vao_library::MeshVaos,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "country_core::asset_libraries::vao_library::MeshVaos", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::vao_library::MeshVaos>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::vao_library::MeshVaos,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::vao_library::MeshVaos>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::vao_library::MeshVaos,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<country_core::render::mesh::Mesh,anyhow::Error>", "constructor": "core::result::Result", "arguments": ["country_core::render::mesh::Mesh", "anyhow::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<country_core::render::mesh::Mesh,anyhow::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<country_core::render::mesh::Mesh,anyhow::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<country_core::render::mesh::Mesh,anyhow::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<country_core::render::mesh::Mesh,anyhow::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::component::clone::component_clone_via_clone<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >", "constructor": "bevy_ecs::component::clone::component_clone_via_clone", "arguments": ["country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >", "constructor": "core::slice::iter::Iter", "arguments": ["country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter::Filter<core::slice::iter::Iter<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,country_core::asset_libraries::mesh_library::impl$1::never_accessed_assets::closure_env$0>", "constructor": "core::iter::adapters::filter::Filter", "arguments": ["core::slice::iter::Iter<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >", "country_core::asset_libraries::mesh_library::impl$1::never_accessed_assets::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::next<country_core::asset_libraries::mesh_library::NeverAccessedMeshAsset,core::iter::adapters::filter::Filter<core::slice::iter::Iter<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,country_core::asset_libraries::mesh_library::impl$1::never_accessed_assets::closure_env$0>,country_core::asset_libraries::mesh_library::impl$1::never_accessed_assets::closure_env$1>", "constructor": "core::iter::adapters::map::impl$2::next", "arguments": ["country_core::asset_libraries::mesh_library::NeverAccessedMeshAsset", "core::iter::adapters::filter::Filter<core::slice::iter::Iter<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,country_core::asset_libraries::mesh_library::impl$1::never_accessed_assets::closure_env$0>", "country_core::asset_libraries::mesh_library::impl$1::never_accessed_assets::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::render::mesh::Mesh,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::render::mesh::Mesh", "core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,country_core::asset_libraries::mesh_library::impl$1::never_accessed_assets::closure_env$0>,country_core::asset_libraries::mesh_library::impl$1::never_accessed_assets::closure_env$1>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::filter::Filter<core::slice::iter::Iter<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,country_core::asset_libraries::mesh_library::impl$1::never_accessed_assets::closure_env$0>", "country_core::asset_libraries::mesh_library::impl$1::never_accessed_assets::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<country_core::asset_libraries::mesh_library::NeverAccessedMeshAsset,core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,country_core::asset_libraries::mesh_library::impl$1::never_accessed_assets::closure_env$0>,country_core::asset_libraries::mesh_library::impl$1::never_accessed_assets::closure_env$1> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["country_core::asset_libraries::mesh_library::NeverAccessedMeshAsset", "core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,country_core::asset_libraries::mesh_library::impl$1::never_accessed_assets::closure_env$0>,country_core::asset_libraries::mesh_library::impl$1::never_accessed_assets::closure_env$1>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::vertex::InterleavedVertexBufferLayout>,rhapsody::vertex::impl$8::encode_buffers::closure_env$0<country_core::render::mesh::Mesh> >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,enum2$<rhapsody::vertex::VertexBufferEncodeError> > > >", "constructor": "core::iter::adapters::GenericShunt", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::vertex::InterleavedVertexBufferLayout>,rhapsody::vertex::impl$8::encode_buffers::closure_env$0<country_core::render::mesh::Mesh> >", "enum2$<core::result::Result<enum2$<core::convert::Infallible>,enum2$<rhapsody::vertex::VertexBufferEncodeError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<rhapsody::raw::buffer::Buffer,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::vertex::InterleavedVertexBufferLayout>,rhapsody::vertex::impl$8::encode_buffers::closure_env$0<country_core::render::mesh::Mesh> >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,enum2$<rhapsody::vertex::VertexBufferEncodeError> > > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["rhapsody::raw::buffer::Buffer", "core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::vertex::InterleavedVertexBufferLayout>,rhapsody::vertex::impl$8::encode_buffers::closure_env$0<country_core::render::mesh::Mesh> >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,enum2$<rhapsody::vertex::VertexBufferEncodeError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef>", "constructor": "tuple$", "arguments": ["country_core::components::KeepOnLevelUnload", "country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines", "country_core::systems::debug::human_scale_ref::HumanScaleRef"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef> >", "constructor": "bevy_ecs::system::commands::impl$2::spawn::closure_env$0", "arguments": ["tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef> > >", "constructor": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0", "arguments": ["bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "constructor": "bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef> > >", "tuple$<>", "enum2$<bevy_ecs::world::error::EntityMutableFetchError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "constructor": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::components::drawable::WireframePass>", "constructor": "tuple$", "arguments": ["country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines", "country_core::components::drawable::WireframePass"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::components::drawable::WireframePass> >", "constructor": "bevy_ecs::system::commands::impl$2::spawn::closure_env$0", "arguments": ["tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::components::drawable::WireframePass>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::components::drawable::WireframePass> > >", "constructor": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0", "arguments": ["bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::components::drawable::WireframePass> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::components::drawable::WireframePass> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "constructor": "bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::components::drawable::WireframePass> > >", "tuple$<>", "enum2$<bevy_ecs::world::error::EntityMutableFetchError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::components::drawable::WireframePass> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "constructor": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::components::drawable::WireframePass> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::components::drawable::WireframePass> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::components::drawable::WireframePass> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>", "constructor": "tuple$", "arguments": ["country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >", "constructor": "bevy_ecs::system::commands::impl$2::spawn::closure_env$0", "arguments": ["tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> > >", "constructor": "bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0", "arguments": ["bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "constructor": "bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> > >", "tuple$<>", "enum2$<bevy_ecs::world::error::EntityMutableFetchError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "constructor": "bevy_ecs::world::command_queue::impl$5::push::closure_env$0", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >,tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["bevy_ecs::world::command_queue::impl$5::push::closure_env$0<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> > >", "tuple$<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2$<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World> > >,ref_mut$<usize> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::BundleInfo::write_components<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>,bevy_ecs::archetype::ArchetypeAfterBundleInsert,core::slice::iter::Iter<bevy_ecs::component::required::RequiredComponentConstructor> >", "constructor": "bevy_ecs::bundle::info::BundleInfo::write_components", "arguments": ["tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>", "bevy_ecs::archetype::ArchetypeAfterBundleInsert", "core::slice::iter::Iter<bevy_ecs::component::required::RequiredComponentConstructor>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::BundleInfo::write_components<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef>,bevy_ecs::archetype::ArchetypeAfterBundleInsert,core::slice::iter::Iter<bevy_ecs::component::required::RequiredComponentConstructor> >", "constructor": "bevy_ecs::bundle::info::BundleInfo::write_components", "arguments": ["tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef>", "bevy_ecs::archetype::ArchetypeAfterBundleInsert", "core::slice::iter::Iter<bevy_ecs::component::required::RequiredComponentConstructor>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::BundleInfo::write_components<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::components::drawable::WireframePass>,bevy_ecs::archetype::ArchetypeAfterBundleInsert,core::slice::iter::Iter<bevy_ecs::component::required::RequiredComponentConstructor> >", "constructor": "bevy_ecs::bundle::info::BundleInfo::write_components", "arguments": ["tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::components::drawable::WireframePass>", "bevy_ecs::archetype::ArchetypeAfterBundleInsert", "core::slice::iter::Iter<bevy_ecs::component::required::RequiredComponentConstructor>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::Commands::spawn<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef> >", "constructor": "bevy_ecs::system::commands::Commands::spawn", "arguments": ["tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::Commands::spawn<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::components::drawable::WireframePass> >", "constructor": "bevy_ecs::system::commands::Commands::spawn", "arguments": ["tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::components::drawable::WireframePass>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::Commands::spawn<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >", "constructor": "bevy_ecs::system::commands::Commands::spawn", "arguments": ["tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::components::drawable::WireframePass> > >", "constructor": "bevy_ecs::system::commands::entity_command::impl$0::apply", "arguments": ["tuple$<>", "bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::components::drawable::WireframePass> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef> > >", "constructor": "bevy_ecs::system::commands::entity_command::impl$0::apply", "arguments": ["tuple$<>", "bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::entity_command::impl$0::apply<tuple$<>,bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> > >", "constructor": "bevy_ecs::system::commands::entity_command::impl$0::apply", "arguments": ["tuple$<>", "bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::rustc_entry<tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >,alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::rustc_entry", "arguments": ["tuple$<alloc::string::String,enum2$<core::option::Option<enum2$<asset::prefab::config::MeshProcessorConfig> > > >", "alloc::sync::Arc<country_core::render::mesh::Mesh,alloc::alloc::Global>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >", "constructor": "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0", "arguments": ["tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::Entry::or_insert_with<core::any::TypeId,bevy_ecs::bundle::info::BundleId,bevy_platform::hash::NoOpHash,hashbrown::raw::alloc::inner::Global,bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> > >", "constructor": "hashbrown::map::Entry::or_insert_with", "arguments": ["core::any::TypeId", "bevy_ecs::bundle::info::BundleId", "bevy_platform::hash::NoOpHash", "hashbrown::raw::alloc::inner::Global", "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::components::drawable::WireframePass> >", "constructor": "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0", "arguments": ["tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::components::drawable::WireframePass>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::Entry::or_insert_with<core::any::TypeId,bevy_ecs::bundle::info::BundleId,bevy_platform::hash::NoOpHash,hashbrown::raw::alloc::inner::Global,bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::components::drawable::WireframePass> > >", "constructor": "hashbrown::map::Entry::or_insert_with", "arguments": ["core::any::TypeId", "bevy_ecs::bundle::info::BundleId", "bevy_platform::hash::NoOpHash", "hashbrown::raw::alloc::inner::Global", "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::components::drawable::WireframePass> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef> >", "constructor": "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0", "arguments": ["tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::Entry::or_insert_with<core::any::TypeId,bevy_ecs::bundle::info::BundleId,bevy_platform::hash::NoOpHash,hashbrown::raw::alloc::inner::Global,bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef> > >", "constructor": "hashbrown::map::Entry::or_insert_with", "arguments": ["core::any::TypeId", "bevy_ecs::bundle::info::BundleId", "bevy_platform::hash::NoOpHash", "hashbrown::raw::alloc::inner::Global", "bevy_ecs::bundle::info::impl$3::register_info::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<alloc::string::String,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["alloc::string::String", "country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "tuple$<>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<std::path::PathBuf,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["std::path::PathBuf", "country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::retain<alloc::string::String,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,std::hash::random::RandomState,alloc::alloc::Global,country_core::asset_libraries::mesh_library::impl$1::remove::closure_env$1>", "constructor": "hashbrown::map::HashMap::retain", "arguments": ["alloc::string::String", "country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "std::hash::random::RandomState", "alloc::alloc::Global", "country_core::asset_libraries::mesh_library::impl$1::remove::closure_env$1"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$80::fmt<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >", "constructor": "core::fmt::impl$80::fmt", "arguments": ["country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::components::drawable::WireframePass> >", "constructor": "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0", "arguments": ["tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::components::drawable::WireframePass>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impls::impl$15::component_ids<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::components::drawable::WireframePass,bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::components::drawable::WireframePass> > >", "constructor": "bevy_ecs::bundle::impls::impl$15::component_ids", "arguments": ["country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines", "country_core::components::drawable::WireframePass", "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::components::drawable::WireframePass> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::component::register::ComponentsRegistrator::register_component_checked<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >", "constructor": "bevy_ecs::component::register::ComponentsRegistrator::register_component_checked", "arguments": ["country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::render::mesh::Mesh,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0> >", "constructor": "alloc::vec::impl$21::extend_trusted::closure_env$0", "arguments": ["country_core::render::mesh::Mesh", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::render::mesh::Mesh,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::render::mesh::Mesh,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["country_core::render::mesh::Mesh", "alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::render::mesh::Mesh,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<country_core::render::mesh::Mesh,core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::render::mesh::Mesh,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::render::mesh::Mesh,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["country_core::render::mesh::Mesh", "core::ops::range::RangeInclusive<usize>", "country_core::startup::startup_sheep::startup_sheep::closure_env$0", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<country_core::render::mesh::Mesh,alloc::vec::impl$21::extend_trusted::closure_env$0<country_core::render::mesh::Mesh,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,country_core::startup::startup_sheep::startup_sheep::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::vertex::InterleavedVertexBufferLayout>,rhapsody::vertex::impl$8::encode_buffers::closure_env$0<country_core::render::mesh::Mesh> >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,enum2$<rhapsody::vertex::VertexBufferEncodeError> > >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<rhapsody::raw::buffer::Buffer,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > > (*)(rhapsody::raw::buffer::Buffer)>,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > > >", "constructor": "core::iter::adapters::impl$0::try_fold::closure_env$0", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::vertex::InterleavedVertexBufferLayout>,rhapsody::vertex::impl$8::encode_buffers::closure_env$0<country_core::render::mesh::Mesh> >", "enum2$<core::result::Result<enum2$<core::convert::Infallible>,enum2$<rhapsody::vertex::VertexBufferEncodeError> > >", "tuple$<>", "core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<rhapsody::raw::buffer::Buffer,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > > (*)(rhapsody::raw::buffer::Buffer)>", "enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::vertex::VertexBufferEncodeError> > >,core::slice::iter::Iter<rhapsody::vertex::InterleavedVertexBufferLayout>,rhapsody::vertex::impl$8::encode_buffers::closure_env$0<country_core::render::mesh::Mesh>,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::vertex::InterleavedVertexBufferLayout>,rhapsody::vertex::impl$8::encode_buffers::closure_env$0<country_core::render::mesh::Mesh> >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,enum2$<rhapsody::vertex::VertexBufferEncodeError> > >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<rhapsody::raw::buffer::Buffer,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > > (*)(rhapsody::raw::buffer::Buffer)>,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > >,tuple$<> > > >", "constructor": "core::iter::adapters::map::impl$2::try_fold", "arguments": ["enum2$<core::result::Result<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::vertex::VertexBufferEncodeError> > >", "core::slice::iter::Iter<rhapsody::vertex::InterleavedVertexBufferLayout>", "rhapsody::vertex::impl$8::encode_buffers::closure_env$0<country_core::render::mesh::Mesh>", "tuple$<>", "core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::vertex::InterleavedVertexBufferLayout>,rhapsody::vertex::impl$8::encode_buffers::closure_env$0<country_core::render::mesh::Mesh> >,enum2$<core::result::Result<enum2$<core::convert::Infallible>,enum2$<rhapsody::vertex::VertexBufferEncodeError> > >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<rhapsody::raw::buffer::Buffer,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > > (*)(rhapsody::raw::buffer::Buffer)>,enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > > >", "enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::buffer::Buffer,tuple$<> > >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::entity::clone_entities::ComponentCloneCtx::write_target_component<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >", "constructor": "bevy_ecs::entity::clone_entities::ComponentCloneCtx::write_target_component", "arguments": ["country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::cell::UnsafeCell<std::collections::hash::map::HashMap<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global> >", "constructor": "core::cell::UnsafeCell", "arguments": ["std::collections::hash::map::HashMap<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<core::cell::UnsafeCell<std::collections::hash::map::HashMap<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global> > >", "constructor": "ref_mut$", "arguments": ["core::cell::UnsafeCell<std::collections::hash::map::HashMap<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::poison::impl$1::fmt<ref_mut$<core::cell::UnsafeCell<std::collections::hash::map::HashMap<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global> > > >", "constructor": "std::sync::poison::impl$1::fmt", "arguments": ["ref_mut$<core::cell::UnsafeCell<std::collections::hash::map::HashMap<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,alloc::boxed::Box<country_core::render::mesh::Mesh,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::World::register_bundle_info<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef> >", "constructor": "bevy_ecs::world::World::register_bundle_info", "arguments": ["tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::World::register_bundle_info<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::components::drawable::WireframePass> >", "constructor": "bevy_ecs::world::World::register_bundle_info", "arguments": ["tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::components::drawable::WireframePass>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::world::World::register_bundle_info<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> >", "constructor": "bevy_ecs::world::World::register_bundle_info", "arguments": ["tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::vertex::InterleavedVertexBufferLayout::encode_into_buffer<country_core::render::mesh::Mesh>", "constructor": "rhapsody::vertex::InterleavedVertexBufferLayout::encode_into_buffer", "arguments": ["country_core::render::mesh::Mesh"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef> >", "constructor": "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0", "arguments": ["tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impls::impl$0::component_ids<country_core::systems::debug::human_scale_ref::HumanScaleRef,bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef> > >", "constructor": "bevy_ecs::bundle::impls::impl$0::component_ids", "arguments": ["country_core::systems::debug::human_scale_ref::HumanScaleRef", "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::bundle::impls::impl$19::component_ids<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef,bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef> > >", "constructor": "bevy_ecs::bundle::impls::impl$19::component_ids", "arguments": ["country_core::components::KeepOnLevelUnload", "country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>", "country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines", "country_core::systems::debug::human_scale_ref::HumanScaleRef", "bevy_ecs::bundle::info::impl$3::register_info::closure$0::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::components::drawable::WireframePass> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >,tuple$<> >", "constructor": "bevy_ecs::system::commands::command::impl$0::apply", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::components::drawable::WireframePass> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >,tuple$<> >", "constructor": "bevy_ecs::system::commands::command::impl$0::apply", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::components::KeepOnLevelUnload,country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines,country_core::systems::debug::human_scale_ref::HumanScaleRef> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::system::commands::command::impl$0::apply<bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >,tuple$<> >", "constructor": "bevy_ecs::system::commands::command::impl$0::apply", "arguments": ["bevy_ecs::error::command_handling::impl$0::handle_error::closure_env$0<bevy_ecs::error::command_handling::impl$3::with_entity::closure_env$0<bevy_ecs::system::commands::impl$2::spawn::closure_env$0<tuple$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh>,country_core::asset_libraries::shader_library::render_pipelines_impl::RenderPipelines> > >,tuple$<>,enum2$<bevy_ecs::world::error::EntityMutableFetchError> >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mesh.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mesh.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 51; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::audio::forest_audio::ForestAudioState>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::InteractableForestTree>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::audio::forest_audio::ForestAudioState>", "bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::InteractableForestTree>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::forest_audio::ForestAudioState>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::InteractableForestTree>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::image_baker::PendingImageBake,alloc::alloc::Global> > (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] tuple$<i64,std::time::Instant> (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] bool (*)(char)
// 候选参数类型: ["char"]
// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] f32 (*)(f32,f32)
// 候选参数类型: ["f32", "f32"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::Mesh::aabb_bounds
// 信息 ID: 85f945a82dfb1fd9c8bceb8e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_prefabs::build_prefab @ 0xa68b53 -> 0xc29ae0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_prefabs::build_prefab @ 0xa68cff -> 0xc29ae0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::collision_bbx @ 0xc29e41 -> 0xc29ae0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_clutter_packs::apply_built_clutter_packs @ 0x1adef15 -> 0xc29ae0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::Mesh::add_color
// 信息 ID: 30bb5267dc8822d07c262dbb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 20 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::load_decorators::load_decorators @ 0x87281c -> 0xc2a950 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::ui::signifiers::impl$2::build::closure$0; module=518, inline_record=95884; 不是独立调用边。
// [PDB 内联宿主] country_core::asset_libraries::mesh_library::AssetMeshLibrary::reload_mesh_src; module=530, inline_record=23884; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_mesh_into_library; module=540, inline_record=3068; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_mesh_into_library_replacing; module=540, inline_record=5892; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::startup_critters::startup_critters; module=544, inline_record=2992; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::startup_critters::startup_critters; module=544, inline_record=3896; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::startup_ivy::startup_ivy; module=558, inline_record=924; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::debug::debug; module=578, inline_record=18144; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::Mesh::arrayed_indices
// 信息 ID: 7b2625634e6a42a180d76423
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::load_decorators::load_decorators::closure$0 @ 0x876543 -> 0xc29ed0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::Mesh::as_triangles
// 信息 ID: 26792d0c56fbc8919c6bc3d7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::load_decorators::load_decorators::closure$3 @ 0x8762c0 -> 0xc29ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_decorators::closure$0 @ 0x87696b -> 0xc29ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys @ 0x87e896 -> 0xc29ca0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::Mesh::clear
// 信息 ID: 46a68bd05ed3c1bd7a353f8a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::clear_world_subset_atlas; module=588, inline_record=112396; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::Mesh::collision_bbx
// 信息 ID: e9ab8dccb2576c9d9a3e9026
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::load_decorators::load_decorators::closure$3 @ 0x8762d4 -> 0xc29e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_decorators::closure$0 @ 0x876982 -> 0xc29e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_collision_as_bbx @ 0x8770b5 -> 0xc29e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_collision_as_bbx @ 0x8772ba -> 0xc29e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure$0 @ 0x21a4225 -> 0xc29e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::try_trait::impl$2::wrap_mut_2::closure$0<tuple$<>,tuple$<>,i32,core::iter::adapters::map::map_fold::closure_env$0<i32,ref$<country_core::resources::clutter_meshes::ClutterMesh>,tuple$<>,system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ref$<country_core::resources::clutter_meshes::ClutterMesh>,alloc::vec::impl$21::extend_trusted::closure_env$0<ref$<country_core::resources::clutter_meshes::ClutterMesh>,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<i32>,system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure_env$1> > > > > @ 0x28d2bb1 -> 0xc29e20 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::startup::load_clutter_packs::apply_built_clutter_packs; module=1770, inline_record=52592; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::Mesh::collision_radius_xz
// 信息 ID: 38cfc5756026a13cc8e77416
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure$0 @ 0x21a420d -> 0xc2a290 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::try_trait::impl$2::wrap_mut_2::closure$0<tuple$<>,tuple$<>,i32,core::iter::adapters::map::map_fold::closure_env$0<i32,ref$<country_core::resources::clutter_meshes::ClutterMesh>,tuple$<>,system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure_env$1,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ref$<country_core::resources::clutter_meshes::ClutterMesh>,alloc::vec::impl$21::extend_trusted::closure_env$0<ref$<country_core::resources::clutter_meshes::ClutterMesh>,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<i32>,system_clutter::autoclutter::load_clutter_meshes::load_clutter_meshes::closure_env$1> > > > > @ 0x28d2b99 -> 0xc2a290 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::startup::load_clutter_packs::apply_built_clutter_packs; module=1770, inline_record=52096; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::Mesh::has_attribute
// 信息 ID: 84b1904ab13d5f57b1c1ec40
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::asset_libraries::mesh_library::AssetMeshLibrary::reload_mesh_src; module=530, inline_record=22804; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::mesh_atlas::MeshAtlas::add; module=539, inline_record=18868; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_mesh_into_library; module=540, inline_record=1988; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_mesh_into_library_replacing; module=540, inline_record=4812; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_mesh_atlases::load_grass2_lod0_meshes; module=544, inline_record=36324; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_mesh_atlases::load_grass2_lod0_meshes; module=544, inline_record=37400; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_mesh_atlases::load_and_prepare_nani_mesh; module=544, inline_record=67004; 不是独立调用边。
// [PDB 内联宿主] country_core::render::vao::build_vao; module=576, inline_record=12400; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::Mesh::has_color
// 信息 ID: 35fe91c5236b24066888b80b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::asset_libraries::mesh_library::AssetMeshLibrary::reload_mesh_src; module=530, inline_record=22780; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_mesh_into_library; module=540, inline_record=1964; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_mesh_into_library_replacing; module=540, inline_record=4788; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::Mesh::is_valid
// 信息 ID: 876ba4d4b3a6af70e1691bbf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::asset_libraries::vao_library::AssetVAOLibrary::rebuild_vao; module=536, inline_record=9020; 不是独立调用边。
// [PDB 内联宿主] country_core::asset_libraries::vao_library::AssetVAOLibrary::add; module=536, inline_record=16288; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::mesh_atlas::MeshAtlas::add; module=539, inline_record=18812; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap; module=539, inline_record=105336; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::reload_prefab_meshes_after_mod_swap; module=539, inline_record=107256; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_mesh_atlases::load_mesh_atlases; module=544, inline_record=6116; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_mesh_atlases::reload_mesh_atlases; module=544, inline_record=27308; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::Mesh::make_double_sided_by_duplicating_triangles
// 信息 ID: 6d6ba16350f512fa4cd771b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_mesh_atlases::load_and_prepare_nani_mesh @ 0xa8b81b -> 0xc2a640 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::Mesh::new
// 信息 ID: 307cf352f6eba69e1d53f927
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 20 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui::signifiers::SignifierEntityBuilder::build; module=518, inline_record=77784; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::load_json::load_json_str_as_mesh; module=521, inline_record=29752; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::startup_terrain::startup_glade_terrain; module=537, inline_record=29932; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_mesh_atlases::load_mesh_atlases; module=544, inline_record=7080; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_mesh_atlases::load_mesh_atlases; module=544, inline_record=8860; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_mesh_atlases::load_mesh_atlases; module=544, inline_record=9740; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_mesh_atlases::load_mesh_atlases; module=544, inline_record=11984; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_mesh_atlases::load_mesh_atlases; module=544, inline_record=13808; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::Mesh::normals_without_indices
// 信息 ID: c638310b15975f9cdfe58bbb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::load_decorators::load_decorators::closure$0 @ 0x876695 -> 0xc2a430 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms @ 0x8c1847 -> 0xc2a430 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::Mesh::positions
// 信息 ID: ad0ddc1ecbc12084b0853bc3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 7 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::prefab::create_prefab_rt @ 0x90ef20 -> 0xc2a9e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::mesh_follow_terrain::mesh_follow_terrain @ 0xaede97 -> 0xc2a9e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_walls::glade_startup_walls @ 0xba2d9b -> 0xc2a9e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::mesh::Mesh::collision_radius_xz @ 0xc2a29c -> 0xc2a9e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_window_cut_sdf::bake_layer @ 0xc6a412 -> 0xc2a9e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_clutter_packs::apply_built_clutter_packs @ 0x1adee5c -> 0xc2a9e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_color::ui_color_gates::ui_color_gates @ 0x28adbc4 -> 0xc2a9e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::Mesh::positions_without_indices
// 信息 ID: e8aea8969dfd55d4d2342c99
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::load_decorators::load_decorators::closure$3 @ 0x876114 -> 0xc2a540 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_decorators::closure$0 @ 0x876552 -> 0xc2a540 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::load_decorators::load_decorators::closure$1 @ 0x876da7 -> 0xc2a540 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::cottage_wall_window::flowerbed_xforms @ 0x8c1839 -> 0xc2a540 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_walls::glade_startup_walls @ 0xba2efa -> 0xc2a540 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::clearing::init_clearing @ 0xba8bc3 -> 0xc2a540 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::Mesh::retain_vertices
// 信息 ID: 594b63b12f7e8c6e8c61d913
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$2> @ 0xa2994e -> 0xc29f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$1> @ 0xa29f76 -> 0xc29f40 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::mesh_atlas::MeshAtlas::compact @ 0xa6623d -> 0xc29f40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,2>,alloc::alloc::Global> >
// 信息 ID: 9b7611770d0326e0e712add7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] array$ -> ["f32", "2"]; 字段布局 = unknown。
// [类型关系] alloc::vec::Vec -> ["array$<f32,2>", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] country_core::render::mesh::Mesh::set_attribute -> ["alloc::vec::Vec<array$<f32,2>,alloc::alloc::Global>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::geometry::cube::impl$5::from @ 0xc2e288 -> 0xac7970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::geometry::plane::impl$1::from @ 0xc2ebbd -> 0xac7970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::geometry::sphere::impl$2::try_from @ 0xc2f436 -> 0xac7970 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_clutter_packs::apply_built_clutter_packs @ 0x1adedcd -> 0xac7970 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::construct_gates::construct_wooden_gate_doors @ 0x1432a3a -> 0x24e1aa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,3>,alloc::alloc::Global> >
// 信息 ID: f494112717f39667a582d596
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] array$ -> ["f32", "3"]; 字段布局 = unknown。
// [类型关系] alloc::vec::Vec -> ["array$<f32,3>", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] country_core::render::mesh::Mesh::set_attribute -> ["alloc::vec::Vec<array$<f32,3>,alloc::alloc::Global>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 62 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::ui::signifiers::impl$2::build::closure$0 @ 0x9c345f -> 0xac8270 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ui::signifiers::impl$2::build::closure$0 @ 0x9c34f8 -> 0xac8270 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::mesh_library::AssetMeshLibrary::reload_mesh_src @ 0xa124a6 -> 0xac8270 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_prefabs::build_prefab @ 0xa6821d -> 0xac8270 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_prefabs::build_prefab @ 0xa67c4f -> 0xac8270 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_prefabs::build_prefab @ 0xa67c9a -> 0xac8270 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_prefabs::build_prefab @ 0xa67eff -> 0xac8270 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_mesh_into_library @ 0xa6f222 -> 0xac8270 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_mesh_into_library_replacing @ 0xa6f552 -> 0xac8270 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_critters::startup_critters @ 0xa84275 -> 0xac8270 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_critters::startup_critters @ 0xa84442 -> 0xac8270 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_mesh_atlases::load_mesh_atlases @ 0xa8563a -> 0xac8270 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_mesh_atlases::load_mesh_atlases @ 0xa8588a -> 0xac8270 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_mesh_atlases::load_mesh_atlases @ 0xa85a66 -> 0xac8270 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_mesh_atlases::load_grass2_lod0_meshes @ 0xa89976 -> 0xac8270 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_mesh_atlases::load_grass2_lod0_meshes @ 0xa899bd -> 0xac8270 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<array$<f32,4>,alloc::alloc::Global> >
// 信息 ID: b4bb5d40c64e76be068adb7c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] array$ -> ["f32", "4"]; 字段布局 = unknown。
// [类型关系] alloc::vec::Vec -> ["array$<f32,4>", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] country_core::render::mesh::Mesh::set_attribute -> ["alloc::vec::Vec<array$<f32,4>,alloc::alloc::Global>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<f32,alloc::alloc::Global> >
// 信息 ID: 237bdac52e7e95e3e67084e2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["f32", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] country_core::render::mesh::Mesh::set_attribute -> ["alloc::vec::Vec<f32,alloc::alloc::Global>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_prefabs::build_prefab @ 0xa683e5 -> 0xac7df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_prefabs::build_prefab @ 0xa68095 -> 0xac7df0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::vao::build_vao @ 0xb91c3e -> 0xac7df0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::Mesh::set_attribute<alloc::vec::Vec<i32,alloc::alloc::Global> >
// 信息 ID: ded3e90ad959c063f57a9018
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::Vec -> ["i32", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] country_core::render::mesh::Mesh::set_attribute -> ["alloc::vec::Vec<i32,alloc::alloc::Global>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_mesh_atlases::load_and_prepare_nani_mesh @ 0xa8bc98 -> 0xac74f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_clutter_packs::apply_built_clutter_packs @ 0x1adee30 -> 0xac74f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::utils::legacy_mod_mesh::upgrade_legacy_textured_clutter_mesh @ 0x1b2692a -> 0xac74f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::Mesh::set_attribute<enum2$<rhapsody::vertex::VertexAttributeValues> >
// 信息 ID: 31fca33441cc0500a85777d9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["rhapsody::vertex::VertexAttributeValues"]; 字段布局 = unknown。
// [类型关系] country_core::render::mesh::Mesh::set_attribute -> ["enum2$<rhapsody::vertex::VertexAttributeValues>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::utils::load_json::load_json_str_as_mesh @ 0x9daa04 -> 0xac7170 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::Mesh::set_indices
// 信息 ID: b24e445b1c3c7c4db52846a8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 10 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::load_json::load_json_str_as_mesh; module=521, inline_record=33724; 不是独立调用边。
// [PDB 内联宿主] country_core::geometry::cube::impl$5::from; module=596, inline_record=52632; 不是独立调用边。
// [PDB 内联宿主] country_core::geometry::plane::impl$1::from; module=596, inline_record=55916; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::geo::mesh_from_pts; module=600, inline_record=42804; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::geo::thick_line_mesh; module=600, inline_record=48708; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::geo::update_curve_mesh<core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec2::Vec2>,country_core::systems::wall::shapes::rectangle::signifier_edit_dims_line::signifier_edit_dims::closure_env$1> >; module=1700, inline_record=130832; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::geo::update_curve_mesh<alloc::vec::into_iter::IntoIter<glam::f32::vec3::Vec3,alloc::alloc::Global> >; module=1700, inline_record=133280; 不是独立调用边。
// [PDB 内联宿主] country_core::utils::geo::update_curve_mesh<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::vec3::Vec3> > >; module=1700, inline_record=136440; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::Mesh::set_source_file
// 信息 ID: 819a10323ef435fd1492c300
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::utils::load_json::load_json_str_as_mesh; module=521, inline_record=30156; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::Mesh::uvs_without_indices
// 信息 ID: 3d27fe012da5bfd9694e2177
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::load_decorators::load_decorators::closure$0 @ 0x8767d5 -> 0xc2a330 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::Mesh::with_color
// 信息 ID: 9d5ddb6dcd27b4fc167a00d4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 16 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::startup_critters::startup_critters; module=544, inline_record=2952; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::startup_critters::startup_critters; module=544, inline_record=3856; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::startup_ivy::startup_ivy; module=558, inline_record=888; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::debug::debug; module=578, inline_record=18104; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::debug::debug; module=578, inline_record=19692; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::debug::debug; module=578, inline_record=20756; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::debug::debug; module=578, inline_record=22100; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::spawn_brush_previews::spawn_brush_previews; module=1636, inline_record=84772; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::impl$0::find_attrib
// 信息 ID: f429ef947411f675f8129fd2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::vertex::InterleavedVertexBufferLayout::encode_into_buffer<country_core::render::mesh::Mesh>; module=1727, inline_record=162752; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::impl$0::vertex_count
// 信息 ID: faf55c89d29cfd31a9685bd6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 28 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::ui::signifiers::impl$2::build::closure$0; module=518, inline_record=95972; 不是独立调用边。
// [PDB 内联宿主] country_core::asset_libraries::mesh_library::AssetMeshLibrary::reload_mesh_src; module=530, inline_record=23912; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::mesh_atlas::MeshAtlas::add; module=539, inline_record=21692; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::mesh_atlas::MeshAtlas::compact; module=539, inline_record=31252; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::build_prefab; module=539, inline_record=45540; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::build_prefab; module=539, inline_record=60408; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_mesh_into_library; module=540, inline_record=3156; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_mesh_into_library_replacing; module=540, inline_record=5980; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::impl$1::arrayed_indices::closure$0
// 信息 ID: b11e5c7813600c39bcfc4bf8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::mesh::impl$1::arrayed_indices", "captures": "unknown"}
// [闭包环境类型引用] country_core::render::mesh::impl$1::arrayed_indices::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<array$<u32,3>,core::iter::adapters::map::Map<core::slice::iter::ChunksExact<u32>,country_core::render::mesh::impl$1::arrayed_indices::closure_env$0> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<array$<u32,3>,core::iter::adapters::map::Map<core::slice::iter::ChunksExact<u32>,country_core::render::mesh::impl$1::arrayed_indices::closure_env$0> >; module=1643, inline_record=455768; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::impl$1::as_triangles::closure$0
// 信息 ID: 9c69a7be89531a8586440b0f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::mesh::impl$1::as_triangles", "captures": "unknown"}
// [闭包环境类型引用] country_core::render::mesh::impl$1::as_triangles::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<utils::geometry::intersect::Triangle,core::iter::adapters::map::Map<core::slice::iter::ChunksExact<u32>,country_core::render::mesh::impl$1::as_triangles::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::mesh::impl$1::as_triangles::closure_env$0 <- core::iter::adapters::map::impl$2::fold<utils::geometry::intersect::Triangle,core::slice::iter::ChunksExact<u32>,country_core::render::mesh::impl$1::as_triangles::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::geometry::intersect::Triangle,alloc::vec::impl$21::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::ChunksExact<u32>,country_core::render::mesh::impl$1::as_triangles::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::mesh::impl$1::as_triangles::closure_env$0 <- core::iter::adapters::map::impl$2::fold<utils::geometry::intersect::Triangle,core::slice::iter::ChunksExact<u32>,country_core::render::mesh::impl$1::as_triangles::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::geometry::intersect::Triangle,alloc::vec::impl$21::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::ChunksExact<u32>,country_core::render::mesh::impl$1::as_triangles::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<utils::geometry::intersect::Triangle,core::slice::iter::ChunksExact<u32>,country_core::render::mesh::impl$1::as_triangles::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::geometry::intersect::Triangle,alloc::vec::impl$21::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::ChunksExact<u32>,country_core::render::mesh::impl$1::as_triangles::closure_env$0> > > >; module=1682, inline_record=18576; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::impl$1::positions::closure$0
// 信息 ID: 1263d5fd8ba7ba059f96d32f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::mesh::impl$1::positions", "captures": "unknown"}
// [闭包环境类型引用] country_core::render::mesh::impl$1::positions::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,3>,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<u32> >,country_core::render::mesh::impl$1::positions::closure_env$0>,country_core::systems::mesh_follow_terrain::mesh_follow_terrain::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::mesh::impl$1::positions::closure_env$0 <- core::iter::adapters::copied::impl$1::fold<core::slice::iter::Iter<u32>,u32,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<u32,glam::f32::vec3::Vec3,tuple$<>,country_core::render::mesh::impl$1::positions::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<u32> >,country_core::render::mesh::impl$1::positions::closure_env$0> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::render::mesh::impl$1::positions::closure_env$0 <- core::iter::adapters::copied::impl$1::fold<core::slice::iter::Iter<u32>,u32,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<u32,glam::f32::vec3::Vec3,tuple$<>,country_core::render::mesh::impl$1::positions::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<u32> >,country_core::render::mesh::impl$1::positions::closure_env$0> > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] country_core::startup::startup_walls::glade_startup_walls; module=578, inline_record=3776; 不是独立调用边。
// [PDB 内联宿主] country_core::render::mesh::Mesh::collision_radius_xz; module=596, inline_record=10236; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::startup_window_cut_sdf::bake_layer; module=605, inline_record=6748; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<array$<f32,3>,core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<u32> >,country_core::render::mesh::impl$1::positions::closure_env$0>,country_core::systems::mesh_follow_terrain::mesh_follow_terrain::closure_env$0> >; module=1643, inline_record=784172; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::copied::impl$1::fold<core::slice::iter::Iter<u32>,u32,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<u32,glam::f32::vec3::Vec3,tuple$<>,country_core::render::mesh::impl$1::positions::closure_env$0,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<glam::f32::vec3::Vec3,alloc::vec::impl$21::extend_trusted::closure_env$0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<u32> >,country_core::render::mesh::impl$1::positions::closure_env$0> > > > >; module=1687, inline_record=3272; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_clutter_packs::apply_built_clutter_packs; module=1770, inline_record=52392; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<utils::geometry::intersect::Triangle,core::ops::range::Range<usize>,system_color::ui_color_gates::ui_color_gates::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::geometry::intersect::Triangle,alloc::vec::impl$21::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_color::ui_color_gates::ui_color_gates::closure_env$0> > > >; module=3991, inline_record=12264; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<utils::geometry::intersect::Triangle,core::ops::range::Range<usize>,system_color::ui_color_gates::ui_color_gates::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<utils::geometry::intersect::Triangle,alloc::vec::impl$21::extend_trusted::closure_env$0<utils::geometry::intersect::Triangle,alloc::alloc::Global,core::iter::adapters::map::Map<core::ops::range::Range<usize>,system_color::ui_color_gates::ui_color_gates::closure_env$0> > > >; module=3991, inline_record=12544; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::impl$1::retain_vertices::closure$0
// 信息 ID: cfafbc2435b08304c47ae74c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::render::mesh::impl$1::retain_vertices", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::render::mesh::Mesh::retain_vertices; module=596, inline_record=8300; 不是独立调用边。
// [PDB 内联宿主] country_core::render::mesh::Mesh::retain_vertices; module=596, inline_record=8424; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::impl$1::retain_vertices::closure$1
// 信息 ID: dc0f32dcb525b590ae8b7866
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::render::mesh::impl$1::retain_vertices", "captures": "unknown"}
// [闭包环境类型引用] country_core::render::mesh::impl$1::retain_vertices::closure_env$1 <- rhapsody::vertex::VertexAttributeValues::retain_by_index<country_core::render::mesh::impl$1::retain_vertices::closure_env$1>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 12 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] rhapsody::vertex::VertexAttributeValues::retain_by_index<country_core::render::mesh::impl$1::retain_vertices::closure_env$1>; module=1727, inline_record=157580; 不是独立调用边。
// [PDB 内联宿主] rhapsody::vertex::VertexAttributeValues::retain_by_index<country_core::render::mesh::impl$1::retain_vertices::closure_env$1>; module=1727, inline_record=157724; 不是独立调用边。
// [PDB 内联宿主] rhapsody::vertex::VertexAttributeValues::retain_by_index<country_core::render::mesh::impl$1::retain_vertices::closure_env$1>; module=1727, inline_record=158176; 不是独立调用边。
// [PDB 内联宿主] rhapsody::vertex::VertexAttributeValues::retain_by_index<country_core::render::mesh::impl$1::retain_vertices::closure_env$1>; module=1727, inline_record=158324; 不是独立调用边。
// [PDB 内联宿主] rhapsody::vertex::VertexAttributeValues::retain_by_index<country_core::render::mesh::impl$1::retain_vertices::closure_env$1>; module=1727, inline_record=158800; 不是独立调用边。
// [PDB 内联宿主] rhapsody::vertex::VertexAttributeValues::retain_by_index<country_core::render::mesh::impl$1::retain_vertices::closure_env$1>; module=1727, inline_record=158948; 不是独立调用边。
// [PDB 内联宿主] rhapsody::vertex::VertexAttributeValues::retain_by_index<country_core::render::mesh::impl$1::retain_vertices::closure_env$1>; module=1727, inline_record=159396; 不是独立调用边。
// [PDB 内联宿主] rhapsody::vertex::VertexAttributeValues::retain_by_index<country_core::render::mesh::impl$1::retain_vertices::closure_env$1>; module=1727, inline_record=159544; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::render::mesh::impl$2::clone
// 信息 ID: 59408fd948dcd927e52bf33a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$2>; module=533, inline_record=93992; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_prefabs::filter_mesh_by_vertex_color<country_core::startup::load_prefabs::load_prefab_mesh::closure_env$1>; module=533, inline_record=98612; 不是独立调用边。
// [PDB 内联宿主] country_core::render::nani::mesh_atlas::MeshAtlas::add; module=539, inline_record=23552; 不是独立调用边。
// [PDB 内联宿主] country_core::render::vao::build_vao; module=576, inline_record=9768; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::register_meshes; module=588, inline_record=101352; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::render::mesh_atlas_library::MeshAtlasLibrary::register_meshes; module=588, inline_record=103424; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_clutter_packs::apply_built_clutter_packs; module=1770, inline_record=52644; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_clutter_packs::apply_built_clutter_packs; module=1770, inline_record=55024; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
