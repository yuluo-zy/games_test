// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/clutter-pack-pipe/src/mod_icons.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 094fee21ceef3333d3f41325860b3f1f5826f4a91d6182ccd7859f0f6383afd2
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: mod_icons.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 576, 655, 1253, 1647, 1651, 1749, 1779
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use asset_pipe; // namespace candidate
//   package identities: [{"name":"asset-pipe","source":"local_or_path","version":"0.1.0"}]
// use clutter_pack_pipe; // namespace candidate
//   package identities: [{"name":"clutter-pack-pipe","source":"local_or_path","version":"0.1.0"}]
// use core; // namespace candidate
//   package identities: []
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use std; // namespace candidate
//   package identities: []
// use turbosloth; // namespace candidate
//   package identities: [{"name":"turbosloth","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: clutter_pack_pipe::mod_icons::mod_clutter_icon_package_dir
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":655,"record_offset":388,"rva":13539472,"va":5382248592}
// 编译实例 RVAs: 0xce9890
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t2407581","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382248592,"raw_hex":"e89dbc2400","size":5,"soft_fail":false,"target_va":5384656240,"va":5382248654,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-6569980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382248592,"raw_hex":"e804c09bff","size":5,"soft_fail":false,"target_va":5375678704,"va":5382248679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] asset_pipe::cache_root
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"asset-pipe","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t17126001","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5382248592,"raw_hex":"e871520501","size":5,"soft_fail":false,"target_va":5399374624,"va":5382248618,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":655,"record_offset":564,"parent_record_offset":388,"ancestor_record_offset":388,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00655.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":655,"record_offset":536,"parent_record_offset":388,"ancestor_record_offset":388,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":655,"record_offset":784,"parent_record_offset":760,"ancestor_record_offset":388,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":655,"record_offset":808,"parent_record_offset":784,"ancestor_record_offset":388,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":655,"record_offset":832,"parent_record_offset":808,"ancestor_record_offset":388,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":655,"record_offset":760,"parent_record_offset":708,"ancestor_record_offset":388,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":655,"record_offset":732,"parent_record_offset":708,"ancestor_record_offset":388,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":655,"record_offset":708,"parent_record_offset":684,"ancestor_record_offset":388,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00655.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":655,"record_offset":684,"parent_record_offset":664,"ancestor_record_offset":388,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00655.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":655,"record_offset":584,"parent_record_offset":564,"ancestor_record_offset":388,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00655.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: clutter_pack_pipe::mod_icons::impl$1::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15122
// 首条位置: {"module":1779,"inlinee":86306}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1779,"record_offset":44060,"parent_record_offset":43296,"ancestor_record_offset":43296,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1779,"record_offset":44744,"parent_record_offset":43296,"ancestor_record_offset":43296,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1779,"record_offset":43344,"parent_record_offset":43296,"ancestor_record_offset":43296,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1779,"record_offset":43872,"parent_record_offset":43848,"ancestor_record_offset":43296,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1779,"record_offset":43896,"parent_record_offset":43872,"ancestor_record_offset":43296,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1779,"record_offset":44992,"parent_record_offset":44968,"ancestor_record_offset":43296,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1779,"record_offset":45016,"parent_record_offset":44992,"ancestor_record_offset":43296,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1779,"record_offset":43920,"parent_record_offset":43896,"ancestor_record_offset":43296,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1779,"record_offset":45040,"parent_record_offset":45016,"ancestor_record_offset":43296,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1779,"record_offset":43848,"parent_record_offset":43692,"ancestor_record_offset":43296,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1779,"record_offset":44968,"parent_record_offset":44912,"ancestor_record_offset":43296,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1779,"record_offset":43628,"parent_record_offset":43600,"ancestor_record_offset":43296,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1779,"record_offset":44940,"parent_record_offset":44912,"ancestor_record_offset":43296,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1779,"record_offset":44912,"parent_record_offset":44888,"ancestor_record_offset":43296,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1779,"record_offset":43692,"parent_record_offset":43656,"ancestor_record_offset":43296,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1779,"record_offset":43656,"parent_record_offset":43628,"ancestor_record_offset":43296,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1779,"record_offset":44888,"parent_record_offset":44864,"ancestor_record_offset":43296,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1779,"record_offset":43736,"parent_record_offset":43692,"ancestor_record_offset":43296,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1779,"record_offset":43564,"parent_record_offset":43500,"ancestor_record_offset":43296,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1779,"record_offset":43536,"parent_record_offset":43500,"ancestor_record_offset":43296,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1779,"record_offset":43600,"parent_record_offset":43564,"ancestor_record_offset":43296,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1779,"record_offset":43472,"parent_record_offset":43440,"ancestor_record_offset":43296,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1779,"record_offset":43500,"parent_record_offset":43472,"ancestor_record_offset":43296,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":1779,"record_offset":43440,"parent_record_offset":43408,"ancestor_record_offset":43296,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1779,"record_offset":43760,"parent_record_offset":43736,"ancestor_record_offset":43296,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1779,"record_offset":43808,"parent_record_offset":43784,"ancestor_record_offset":43296,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1779,"record_offset":43784,"parent_record_offset":43760,"ancestor_record_offset":43296,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1779,"record_offset":43976,"parent_record_offset":43564,"ancestor_record_offset":43296,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1779,"record_offset":44000,"parent_record_offset":43976,"ancestor_record_offset":43296,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1779,"record_offset":44768,"parent_record_offset":44744,"ancestor_record_offset":43296,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1779,"record_offset":43376,"parent_record_offset":43344,"ancestor_record_offset":43296,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1779,"record_offset":44088,"parent_record_offset":44060,"ancestor_record_offset":43296,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1779,"record_offset":43408,"parent_record_offset":43376,"ancestor_record_offset":43296,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: clutter_pack_pipe::mod_icons::impl$2::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13F41
// 首条位置: {"module":1651,"inlinee":81729}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::impl$56::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1651,"record_offset":48160,"parent_record_offset":48132,"ancestor_record_offset":48132,"inlinee":14031,"source_line":594,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":48212,"parent_record_offset":48188,"ancestor_record_offset":48132,"inlinee":14034,"source_line":429,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":48236,"parent_record_offset":48212,"ancestor_record_offset":48132,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":48188,"parent_record_offset":48160,"ancestor_record_offset":48132,"inlinee":14033,"source_line":812,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1651,"record_offset":48260,"parent_record_offset":48236,"ancestor_record_offset":48132,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: clutter_pack_pipe::mod_icons::impl$0::run
// 记录形式: compiled_procedure; instances=1
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":655,"record_offset":1396,"rva":13539648,"va":5382248768}
// 编译实例 RVAs: 0xce9940
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: clutter_pack_pipe::mod_icons::impl$0::run::async_fn$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 58
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15124
// 首条位置: {"module":1779,"inlinee":86308}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::box_assume_init_into_vec_unsafe
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1779,"record_offset":54508,"parent_record_offset":46796,"ancestor_record_offset":46796,"inlinee":4459,"source_line":262,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_direct] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1779,"record_offset":54204,"parent_record_offset":46796,"ancestor_record_offset":46796,"inlinee":4443,"source_line":311,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_direct] anyhow::context::impl$0::with_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1779,"record_offset":48552,"parent_record_offset":46796,"ancestor_record_offset":46796,"inlinee":17362,"source_line":58,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1779,"record_offset":52648,"parent_record_offset":46796,"ancestor_record_offset":46796,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_direct] core::future::future::impl$1::poll
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\future\future.rs
//     evidence={"module":1779,"record_offset":47100,"parent_record_offset":46796,"ancestor_record_offset":46796,"inlinee":5392,"source_line":132,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1779,"record_offset":54388,"parent_record_offset":46796,"ancestor_record_offset":46796,"inlinee":5727,"source_line":1390,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1779,"record_offset":52704,"parent_record_offset":46796,"ancestor_record_offset":46796,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_none
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1779,"record_offset":51896,"parent_record_offset":46796,"ancestor_record_offset":46796,"inlinee":4877,"source_line":684,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1779,"record_offset":54476,"parent_record_offset":46796,"ancestor_record_offset":46796,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1779,"record_offset":52732,"parent_record_offset":46796,"ancestor_record_offset":46796,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_direct] core::option::impl$6::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1779,"record_offset":50916,"parent_record_offset":46796,"ancestor_record_offset":46796,"inlinee":5739,"source_line":2261,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=34
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1779,"record_offset":47184,"parent_record_offset":46796,"ancestor_record_offset":46796,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1779,"record_offset":47876,"parent_record_offset":46796,"ancestor_record_offset":46796,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1779,"record_offset":47908,"parent_record_offset":46796,"ancestor_record_offset":46796,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1779,"record_offset":53808,"parent_record_offset":46796,"ancestor_record_offset":46796,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1779,"record_offset":52564,"parent_record_offset":46796,"ancestor_record_offset":46796,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1779,"record_offset":51412,"parent_record_offset":51388,"ancestor_record_offset":46796,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1779,"record_offset":51436,"parent_record_offset":51412,"ancestor_record_offset":46796,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1779,"record_offset":47516,"parent_record_offset":47492,"ancestor_record_offset":46796,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1779,"record_offset":47540,"parent_record_offset":47516,"ancestor_record_offset":46796,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1779,"record_offset":51460,"parent_record_offset":51436,"ancestor_record_offset":46796,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1779,"record_offset":47564,"parent_record_offset":47540,"ancestor_record_offset":46796,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1779,"record_offset":51388,"parent_record_offset":51236,"ancestor_record_offset":46796,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1779,"record_offset":47492,"parent_record_offset":47360,"ancestor_record_offset":46796,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1779,"record_offset":54232,"parent_record_offset":54204,"ancestor_record_offset":46796,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1779,"record_offset":47360,"parent_record_offset":47332,"ancestor_record_offset":46796,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1779,"record_offset":48620,"parent_record_offset":48596,"ancestor_record_offset":46796,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1779,"record_offset":48668,"parent_record_offset":48644,"ancestor_record_offset":46796,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1779,"record_offset":51180,"parent_record_offset":51156,"ancestor_record_offset":46796,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=22
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1779,"record_offset":49688,"parent_record_offset":49660,"ancestor_record_offset":46796,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1779,"record_offset":49660,"parent_record_offset":49636,"ancestor_record_offset":46796,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1779,"record_offset":51236,"parent_record_offset":51204,"ancestor_record_offset":46796,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1779,"record_offset":51204,"parent_record_offset":51180,"ancestor_record_offset":46796,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1779,"record_offset":49636,"parent_record_offset":49612,"ancestor_record_offset":46796,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1779,"record_offset":51276,"parent_record_offset":51236,"ancestor_record_offset":46796,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::into_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1779,"record_offset":54532,"parent_record_offset":54508,"ancestor_record_offset":46796,"inlinee":4460,"source_line":480,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1779,"record_offset":52880,"parent_record_offset":52836,"ancestor_record_offset":46796,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1779,"record_offset":52908,"parent_record_offset":52880,"ancestor_record_offset":46796,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1779,"record_offset":51128,"parent_record_offset":51068,"ancestor_record_offset":46796,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1779,"record_offset":52836,"parent_record_offset":52808,"ancestor_record_offset":46796,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1779,"record_offset":52808,"parent_record_offset":52780,"ancestor_record_offset":46796,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1779,"record_offset":53416,"parent_record_offset":53364,"ancestor_record_offset":46796,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1779,"record_offset":53328,"parent_record_offset":53296,"ancestor_record_offset":46796,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$40::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1779,"record_offset":53364,"parent_record_offset":53328,"ancestor_record_offset":46796,"inlinee":6797,"source_line":2910,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1779,"record_offset":48908,"parent_record_offset":48868,"ancestor_record_offset":46796,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1779,"record_offset":48868,"parent_record_offset":48836,"ancestor_record_offset":46796,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1779,"record_offset":51100,"parent_record_offset":51068,"ancestor_record_offset":46796,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::from_raw_parts_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1779,"record_offset":54556,"parent_record_offset":54532,"ancestor_record_offset":46796,"inlinee":4461,"source_line":1181,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1779,"record_offset":51156,"parent_record_offset":51128,"ancestor_record_offset":46796,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1779,"record_offset":51044,"parent_record_offset":51020,"ancestor_record_offset":46796,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1779,"record_offset":51068,"parent_record_offset":51044,"ancestor_record_offset":46796,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":1779,"record_offset":51020,"parent_record_offset":50996,"ancestor_record_offset":46796,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Error::context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs
//     evidence={"module":1779,"record_offset":48732,"parent_record_offset":48708,"ancestor_record_offset":46796,"inlinee":6285,"source_line":398,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::context::ext::impl$1::ext_context
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/context.rs
//     evidence={"module":1779,"record_offset":48708,"parent_record_offset":48552,"ancestor_record_offset":46796,"inlinee":6338,"source_line":33,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] clutter_pack_pipe::mod_icons::impl$0::run::async_fn$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"clutter-pack-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/clutter-pack-pipe/src/mod_icons.rs
//     evidence={"module":1779,"record_offset":48596,"parent_record_offset":48552,"ancestor_record_offset":46796,"inlinee":70360,"source_line":61,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] clutter_pack_pipe::mod_icons::impl$0::run::async_fn$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"clutter-pack-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/clutter-pack-pipe/src/mod_icons.rs
//     evidence={"module":1779,"record_offset":52780,"parent_record_offset":52732,"ancestor_record_offset":46796,"inlinee":86309,"source_line":87,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] clutter_pack_pipe::mod_icons::impl$0::run::async_fn$0::closure$2
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"clutter-pack-pipe","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/clutter-pack-pipe/src/mod_icons.rs
//     evidence={"module":1779,"record_offset":53296,"parent_record_offset":52732,"ancestor_record_offset":46796,"inlinee":86310,"source_line":88,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1779,"record_offset":47392,"parent_record_offset":47360,"ancestor_record_offset":46796,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1779,"record_offset":51300,"parent_record_offset":51276,"ancestor_record_offset":46796,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":1779,"record_offset":53444,"parent_record_offset":53364,"ancestor_record_offset":46796,"inlinee":6799,"source_line":576,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1779,"record_offset":51348,"parent_record_offset":51324,"ancestor_record_offset":46796,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1779,"record_offset":47456,"parent_record_offset":47420,"ancestor_record_offset":46796,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1779,"record_offset":47420,"parent_record_offset":47392,"ancestor_record_offset":46796,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1779,"record_offset":51324,"parent_record_offset":51300,"ancestor_record_offset":46796,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=13
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1779,"record_offset":51736,"parent_record_offset":51704,"ancestor_record_offset":46796,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1779,"record_offset":54440,"parent_record_offset":54412,"ancestor_record_offset":46796,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1779,"record_offset":54412,"parent_record_offset":54388,"ancestor_record_offset":46796,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1779,"record_offset":51920,"parent_record_offset":51896,"ancestor_record_offset":46796,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1779,"record_offset":52672,"parent_record_offset":52648,"ancestor_record_offset":46796,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1779,"record_offset":48644,"parent_record_offset":48620,"ancestor_record_offset":46796,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::pin::Pin::as_deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\pin.rs
//     evidence={"module":1779,"record_offset":47124,"parent_record_offset":47100,"ancestor_record_offset":46796,"inlinee":5375,"source_line":1428,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::pin::Pin::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\pin.rs
//     evidence={"module":1779,"record_offset":47148,"parent_record_offset":47124,"ancestor_record_offset":46796,"inlinee":5376,"source_line":1409,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1779,"record_offset":51516,"parent_record_offset":51128,"ancestor_record_offset":46796,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1779,"record_offset":51540,"parent_record_offset":51516,"ancestor_record_offset":46796,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=84
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1779,"record_offset":47220,"parent_record_offset":47184,"ancestor_record_offset":46796,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1779,"record_offset":48960,"parent_record_offset":48908,"ancestor_record_offset":46796,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1779,"record_offset":53472,"parent_record_offset":53364,"ancestor_record_offset":46796,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1779,"record_offset":49152,"parent_record_offset":49116,"ancestor_record_offset":46796,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1779,"record_offset":52588,"parent_record_offset":52564,"ancestor_record_offset":46796,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1779,"record_offset":52612,"parent_record_offset":52588,"ancestor_record_offset":46796,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1779,"record_offset":48992,"parent_record_offset":48908,"ancestor_record_offset":46796,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1779,"record_offset":49028,"parent_record_offset":48868,"ancestor_record_offset":46796,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1779,"record_offset":53856,"parent_record_offset":53832,"ancestor_record_offset":46796,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1779,"record_offset":53832,"parent_record_offset":53808,"ancestor_record_offset":46796,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1779,"record_offset":53880,"parent_record_offset":53856,"ancestor_record_offset":46796,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1779,"record_offset":50972,"parent_record_offset":50948,"ancestor_record_offset":46796,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1779,"record_offset":50948,"parent_record_offset":50916,"ancestor_record_offset":46796,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1779,"record_offset":50996,"parent_record_offset":50972,"ancestor_record_offset":46796,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: core::ptr::drop_in_place<enum2$<clutter_pack_pipe::mod_icons::impl$0::run::async_fn_env$0> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1779,"record_offset":30444,"rva":28475648,"va":5397184768}
// 编译实例 RVAs: 0x1b28100
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=23
//     evidence={"assembly":"\tcallq\t-21506142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397184768,"raw_hex":"e8a2d7b7fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5397184841,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14076dcf0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_icons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-20687612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397184768,"raw_hex":"e80455c4fe","size":5,"soft_fail":false,"target_va":5376498928,"va":5397186535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990020
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod_icons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-18449486","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397184768,"raw_hex":"e8b27be6fe","size":5,"soft_fail":false,"target_va":5378736160,"va":5397185641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-4102761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5397184768,"raw_hex":"e89765c1ff","size":5,"soft_fail":false,"target_va":5393082512,"va":5397185268,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t-21507012","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5397184768,"raw_hex":"e93cd4b7fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5397185711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140990020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_icons.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-18448733","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5397184768,"raw_hex":"e9a37ee6fe","size":5,"soft_fail":false,"target_va":5378736160,"va":5397184888,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5397184768,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5397184982,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=23
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1779,"record_offset":30620,"parent_record_offset":30444,"ancestor_record_offset":30444,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1779,"record_offset":30860,"parent_record_offset":30836,"ancestor_record_offset":30444,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1779,"record_offset":30884,"parent_record_offset":30860,"ancestor_record_offset":30444,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1779,"record_offset":30908,"parent_record_offset":30884,"ancestor_record_offset":30444,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1779,"record_offset":30836,"parent_record_offset":30784,"ancestor_record_offset":30444,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1779,"record_offset":31524,"parent_record_offset":31496,"ancestor_record_offset":30444,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=17
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1779,"record_offset":30808,"parent_record_offset":30784,"ancestor_record_offset":30444,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1779,"record_offset":30784,"parent_record_offset":30760,"ancestor_record_offset":30444,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1779,"record_offset":30760,"parent_record_offset":30740,"ancestor_record_offset":30444,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1779,"record_offset":35760,"parent_record_offset":35724,"ancestor_record_offset":30444,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1779,"record_offset":33384,"parent_record_offset":33356,"ancestor_record_offset":30444,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1779,"record_offset":31556,"parent_record_offset":31524,"ancestor_record_offset":30444,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1779,"record_offset":31620,"parent_record_offset":31584,"ancestor_record_offset":30444,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1779,"record_offset":31584,"parent_record_offset":31556,"ancestor_record_offset":30444,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1779,"record_offset":34048,"parent_record_offset":34016,"ancestor_record_offset":30444,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=65
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1779,"record_offset":30640,"parent_record_offset":30620,"ancestor_record_offset":30444,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1779,"record_offset":35792,"parent_record_offset":35760,"ancestor_record_offset":30444,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1779,"record_offset":33420,"parent_record_offset":33384,"ancestor_record_offset":30444,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1779,"record_offset":33444,"parent_record_offset":33420,"ancestor_record_offset":30444,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1779,"record_offset":33476,"parent_record_offset":33384,"ancestor_record_offset":30444,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: clutter_pack_pipe::mod_icons::impl$0::run::async_fn$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x112D8
// 首条位置: {"module":1253,"inlinee":70360}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1779,"record_offset":48620,"parent_record_offset":48596,"ancestor_record_offset":48596,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1779,"record_offset":48668,"parent_record_offset":48644,"ancestor_record_offset":48596,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1779,"record_offset":48644,"parent_record_offset":48620,"ancestor_record_offset":48596,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: clutter_pack_pipe::mod_icons::impl$0::run::async_fn$0::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 87
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15125
// 首条位置: {"module":1779,"inlinee":86309}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1779,"record_offset":52808,"parent_record_offset":52780,"ancestor_record_offset":52780,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1779,"record_offset":53112,"parent_record_offset":53088,"ancestor_record_offset":52780,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1779,"record_offset":53136,"parent_record_offset":53112,"ancestor_record_offset":52780,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1779,"record_offset":53160,"parent_record_offset":53136,"ancestor_record_offset":52780,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1779,"record_offset":53088,"parent_record_offset":53060,"ancestor_record_offset":52780,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1779,"record_offset":53000,"parent_record_offset":52972,"ancestor_record_offset":52780,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1779,"record_offset":53060,"parent_record_offset":53028,"ancestor_record_offset":52780,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1779,"record_offset":53028,"parent_record_offset":53000,"ancestor_record_offset":52780,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1779,"record_offset":52880,"parent_record_offset":52836,"ancestor_record_offset":52780,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1779,"record_offset":52908,"parent_record_offset":52880,"ancestor_record_offset":52780,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1779,"record_offset":52936,"parent_record_offset":52908,"ancestor_record_offset":52780,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1779,"record_offset":52836,"parent_record_offset":52808,"ancestor_record_offset":52780,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1779,"record_offset":52972,"parent_record_offset":52936,"ancestor_record_offset":52780,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1779,"record_offset":53216,"parent_record_offset":52936,"ancestor_record_offset":52780,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1779,"record_offset":53240,"parent_record_offset":53216,"ancestor_record_offset":52780,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: clutter_pack_pipe::mod_icons::impl$0::run::async_fn$0::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 88
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x15126
// 首条位置: {"module":1779,"inlinee":86310}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::string::impl$39::to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1779,"record_offset":53328,"parent_record_offset":53296,"ancestor_record_offset":53296,"inlinee":4514,"source_line":2893,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1779,"record_offset":53684,"parent_record_offset":53660,"ancestor_record_offset":53296,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1779,"record_offset":53708,"parent_record_offset":53684,"ancestor_record_offset":53296,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1779,"record_offset":53732,"parent_record_offset":53708,"ancestor_record_offset":53296,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1779,"record_offset":53660,"parent_record_offset":53604,"ancestor_record_offset":53296,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1779,"record_offset":53632,"parent_record_offset":53604,"ancestor_record_offset":53296,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1779,"record_offset":53604,"parent_record_offset":53580,"ancestor_record_offset":53296,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1779,"record_offset":53580,"parent_record_offset":53556,"ancestor_record_offset":53296,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::String::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1779,"record_offset":53416,"parent_record_offset":53364,"ancestor_record_offset":53296,"inlinee":6798,"source_line":437,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$40::spec_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1779,"record_offset":53364,"parent_record_offset":53328,"ancestor_record_offset":53296,"inlinee":6797,"source_line":2910,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Formatter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\fmt\mod.rs
//     evidence={"module":1779,"record_offset":53444,"parent_record_offset":53364,"ancestor_record_offset":53296,"inlinee":6799,"source_line":576,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1779,"record_offset":53508,"parent_record_offset":53364,"ancestor_record_offset":53296,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1779,"record_offset":53472,"parent_record_offset":53364,"ancestor_record_offset":53296,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/01779.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: clutter_pack_pipe::mod_icons::build_mod_clutter_icons
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 123
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8F03
// 首条位置: {"module":576,"inlinee":36611}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: clutter_pack_pipe::mod_icons::build_mod_clutter_icons::async_fn$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 128
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13EC8
// 首条位置: {"module":1647,"inlinee":81608}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1647,"record_offset":109140,"parent_record_offset":104880,"ancestor_record_offset":104880,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$32::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1647,"record_offset":105216,"parent_record_offset":104880,"ancestor_record_offset":104880,"inlinee":5906,"source_line":2376,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1647,"record_offset":109784,"parent_record_offset":104880,"ancestor_record_offset":104880,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1647,"record_offset":107748,"parent_record_offset":104880,"ancestor_record_offset":104880,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1647,"record_offset":107492,"parent_record_offset":104880,"ancestor_record_offset":104880,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1647,"record_offset":105728,"parent_record_offset":104880,"ancestor_record_offset":104880,"inlinee":5351,"source_line":523,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1647,"record_offset":107776,"parent_record_offset":104880,"ancestor_record_offset":104880,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$18::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1647,"record_offset":106132,"parent_record_offset":104880,"ancestor_record_offset":104880,"inlinee":8387,"source_line":3913,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1647,"record_offset":107832,"parent_record_offset":104880,"ancestor_record_offset":104880,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1647,"record_offset":107296,"parent_record_offset":104880,"ancestor_record_offset":104880,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] core::future::future::impl$1::poll
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\future\future.rs
//     evidence={"module":1647,"record_offset":108028,"parent_record_offset":104880,"ancestor_record_offset":104880,"inlinee":5392,"source_line":132,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] core::future::into_future::impl$0::into_future
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\future\into_future.rs
//     evidence={"module":1647,"record_offset":108000,"parent_record_offset":104880,"ancestor_record_offset":104880,"inlinee":20661,"source_line":142,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] core::hint::must_use
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hint.rs
//     evidence={"module":1647,"record_offset":109224,"parent_record_offset":104880,"ancestor_record_offset":104880,"inlinee":7244,"source_line":613,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::as_deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1647,"record_offset":106332,"parent_record_offset":104880,"ancestor_record_offset":104880,"inlinee":5727,"source_line":1390,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1647,"record_offset":106396,"parent_record_offset":104880,"ancestor_record_offset":104880,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::transpose
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1647,"record_offset":106456,"parent_record_offset":104880,"ancestor_record_offset":104880,"inlinee":10033,"source_line":2222,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1647,"record_offset":107464,"parent_record_offset":104880,"ancestor_record_offset":104880,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1647,"record_offset":109728,"parent_record_offset":104880,"ancestor_record_offset":104880,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1647,"record_offset":109812,"parent_record_offset":104880,"ancestor_record_offset":104880,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1647,"record_offset":106244,"parent_record_offset":104880,"ancestor_record_offset":104880,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1647,"record_offset":107352,"parent_record_offset":104880,"ancestor_record_offset":104880,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1647,"record_offset":107212,"parent_record_offset":104880,"ancestor_record_offset":104880,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":1647,"record_offset":106496,"parent_record_offset":104880,"ancestor_record_offset":104880,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::asset::BuildContext::with_spawner
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":1647,"record_offset":105304,"parent_record_offset":104880,"ancestor_record_offset":104880,"inlinee":81575,"source_line":668,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::asset::BuiltAsset::asset_ref
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":1647,"record_offset":109756,"parent_record_offset":104880,"ancestor_record_offset":104880,"inlinee":81565,"source_line":555,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::asset::impl$33::deref
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":1647,"record_offset":109252,"parent_record_offset":104880,"ancestor_record_offset":104880,"inlinee":81567,"source_line":574,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1647,"record_offset":105468,"parent_record_offset":105444,"ancestor_record_offset":104880,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1647,"record_offset":105492,"parent_record_offset":105468,"ancestor_record_offset":104880,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1647,"record_offset":108324,"parent_record_offset":108300,"ancestor_record_offset":104880,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1647,"record_offset":108348,"parent_record_offset":108324,"ancestor_record_offset":104880,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1647,"record_offset":105516,"parent_record_offset":105492,"ancestor_record_offset":104880,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1647,"record_offset":108372,"parent_record_offset":108348,"ancestor_record_offset":104880,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1647,"record_offset":105444,"parent_record_offset":105412,"ancestor_record_offset":104880,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1647,"record_offset":108300,"parent_record_offset":108168,"ancestor_record_offset":104880,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1647,"record_offset":105412,"parent_record_offset":105376,"ancestor_record_offset":104880,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1647,"record_offset":105376,"parent_record_offset":105348,"ancestor_record_offset":104880,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1647,"record_offset":108168,"parent_record_offset":108140,"ancestor_record_offset":104880,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1647,"record_offset":109188,"parent_record_offset":109164,"ancestor_record_offset":104880,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1647,"record_offset":107596,"parent_record_offset":107564,"ancestor_record_offset":104880,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1647,"record_offset":105792,"parent_record_offset":105760,"ancestor_record_offset":104880,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1647,"record_offset":110444,"parent_record_offset":110412,"ancestor_record_offset":104880,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1647,"record_offset":110412,"parent_record_offset":110384,"ancestor_record_offset":104880,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1647,"record_offset":107644,"parent_record_offset":107620,"ancestor_record_offset":104880,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1647,"record_offset":107620,"parent_record_offset":107596,"ancestor_record_offset":104880,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1647,"record_offset":105852,"parent_record_offset":105820,"ancestor_record_offset":104880,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1647,"record_offset":105820,"parent_record_offset":105792,"ancestor_record_offset":104880,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1647,"record_offset":110384,"parent_record_offset":110356,"ancestor_record_offset":104880,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1647,"record_offset":105888,"parent_record_offset":105852,"ancestor_record_offset":104880,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1647,"record_offset":106728,"parent_record_offset":106664,"ancestor_record_offset":104880,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1647,"record_offset":109008,"parent_record_offset":108972,"ancestor_record_offset":104880,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1647,"record_offset":105348,"parent_record_offset":105304,"ancestor_record_offset":104880,"inlinee":6896,"source_line":419,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1647,"record_offset":105596,"parent_record_offset":105572,"ancestor_record_offset":104880,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1647,"record_offset":107564,"parent_record_offset":107520,"ancestor_record_offset":104880,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1647,"record_offset":106700,"parent_record_offset":106664,"ancestor_record_offset":104880,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1647,"record_offset":107520,"parent_record_offset":107492,"ancestor_record_offset":104880,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1647,"record_offset":105760,"parent_record_offset":105728,"ancestor_record_offset":104880,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1647,"record_offset":106636,"parent_record_offset":106604,"ancestor_record_offset":104880,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1647,"record_offset":110196,"parent_record_offset":110172,"ancestor_record_offset":104880,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1647,"record_offset":106664,"parent_record_offset":106636,"ancestor_record_offset":104880,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":1647,"record_offset":106604,"parent_record_offset":106568,"ancestor_record_offset":104880,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1647,"record_offset":108200,"parent_record_offset":108168,"ancestor_record_offset":104880,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1647,"record_offset":105912,"parent_record_offset":105888,"ancestor_record_offset":104880,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1647,"record_offset":106960,"parent_record_offset":106936,"ancestor_record_offset":104880,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1647,"record_offset":108264,"parent_record_offset":108228,"ancestor_record_offset":104880,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1647,"record_offset":108228,"parent_record_offset":108200,"ancestor_record_offset":104880,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1647,"record_offset":105936,"parent_record_offset":105912,"ancestor_record_offset":104880,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1647,"record_offset":105960,"parent_record_offset":105936,"ancestor_record_offset":104880,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1647,"record_offset":111012,"parent_record_offset":110984,"ancestor_record_offset":104880,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":1647,"record_offset":106424,"parent_record_offset":106396,"ancestor_record_offset":104880,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1647,"record_offset":106360,"parent_record_offset":106332,"ancestor_record_offset":104880,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1647,"record_offset":107320,"parent_record_offset":107296,"ancestor_record_offset":104880,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1647,"record_offset":109164,"parent_record_offset":109140,"ancestor_record_offset":104880,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::pin::Pin::as_deref_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\pin.rs
//     evidence={"module":1647,"record_offset":108052,"parent_record_offset":108028,"ancestor_record_offset":104880,"inlinee":5375,"source_line":1428,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::pin::Pin::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\pin.rs
//     evidence={"module":1647,"record_offset":108076,"parent_record_offset":108052,"ancestor_record_offset":104880,"inlinee":5376,"source_line":1409,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1647,"record_offset":107128,"parent_record_offset":106728,"ancestor_record_offset":104880,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1647,"record_offset":107152,"parent_record_offset":107128,"ancestor_record_offset":104880,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=27
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1647,"record_offset":105572,"parent_record_offset":105304,"ancestor_record_offset":104880,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1647,"record_offset":106204,"parent_record_offset":106180,"ancestor_record_offset":104880,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1647,"record_offset":106300,"parent_record_offset":106244,"ancestor_record_offset":104880,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1647,"record_offset":109040,"parent_record_offset":109008,"ancestor_record_offset":104880,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1647,"record_offset":107940,"parent_record_offset":107832,"ancestor_record_offset":104880,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1647,"record_offset":106272,"parent_record_offset":106244,"ancestor_record_offset":104880,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1647,"record_offset":107964,"parent_record_offset":107940,"ancestor_record_offset":104880,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1647,"record_offset":107684,"parent_record_offset":107520,"ancestor_record_offset":104880,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1647,"record_offset":106156,"parent_record_offset":106132,"ancestor_record_offset":104880,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1647,"record_offset":106180,"parent_record_offset":106156,"ancestor_record_offset":104880,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1647,"record_offset":105244,"parent_record_offset":105216,"ancestor_record_offset":104880,"inlinee":5322,"source_line":3134,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1647,"record_offset":105632,"parent_record_offset":105596,"ancestor_record_offset":104880,"inlinee":4732,"source_line":3165,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1647,"record_offset":107236,"parent_record_offset":107212,"ancestor_record_offset":104880,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1647,"record_offset":105268,"parent_record_offset":105244,"ancestor_record_offset":104880,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1647,"record_offset":107260,"parent_record_offset":107236,"ancestor_record_offset":104880,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1647,"record_offset":105656,"parent_record_offset":105632,"ancestor_record_offset":104880,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1647,"record_offset":105688,"parent_record_offset":105596,"ancestor_record_offset":104880,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1647,"record_offset":107400,"parent_record_offset":107376,"ancestor_record_offset":104880,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":1647,"record_offset":107376,"parent_record_offset":107352,"ancestor_record_offset":104880,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":1647,"record_offset":107424,"parent_record_offset":107400,"ancestor_record_offset":104880,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":1647,"record_offset":106532,"parent_record_offset":106496,"ancestor_record_offset":104880,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":1647,"record_offset":106568,"parent_record_offset":106532,"ancestor_record_offset":104880,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/01647.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: core::ptr::drop_in_place<enum2$<clutter_pack_pipe::mod_icons::build_mod_clutter_icons::async_fn_env$0<country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons::closure_env$1> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 153
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1749,"record_offset":34944,"rva":27814464,"va":5396523584}
// 编译实例 RVAs: 0x1a86a40
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-20845013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396523584,"raw_hex":"e82beec1fe","size":5,"soft_fail":false,"target_va":5375678704,"va":5396523712,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096c730
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod_icons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17933309","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396523584,"raw_hex":"e8035ceefe","size":5,"soft_fail":false,"target_va":5378590512,"va":5396523816,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140972c40
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: mod_icons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17907379","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396523584,"raw_hex":"e84dc1eefe","size":5,"soft_fail":false,"target_va":5378616384,"va":5396523758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098c1c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod_icons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17803541","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396523584,"raw_hex":"e8eb56f0fe","size":5,"soft_fail":false,"target_va":5378720192,"va":5396523728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990020
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: mod_icons.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17787626","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396523584,"raw_hex":"e81695f0fe","size":5,"soft_fail":false,"target_va":5378736160,"va":5396523781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-12886344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396523584,"raw_hex":"e8b85e3bff","size":5,"soft_fail":false,"target_va":5383637632,"va":5396523971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<enum2$<asset_pipe::texture::build_texture_assets::async_fn_env$0> >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-3680207","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5396523584,"raw_hex":"e831d8c7ff","size":5,"soft_fail":false,"target_va":5392843440,"va":5396523642,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x14096c730
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: mod_icons.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-17933300","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5396523584,"raw_hex":"e90c5ceefe","size":5,"soft_fail":false,"target_va":5378590512,"va":5396523807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alloc::sync::Arc$LT$T$C$A$GT$::drop_slow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tjmp\t-12886218","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5396523584,"raw_hex":"e9365f3bff","size":5,"soft_fail":false,"target_va":5383637632,"va":5396523845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5396523584,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5396523688,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1749,"record_offset":35224,"parent_record_offset":34944,"ancestor_record_offset":34944,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1749,"record_offset":35428,"parent_record_offset":35404,"ancestor_record_offset":34944,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1749,"record_offset":35452,"parent_record_offset":35428,"ancestor_record_offset":34944,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1749,"record_offset":35476,"parent_record_offset":35452,"ancestor_record_offset":34944,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1749,"record_offset":35404,"parent_record_offset":35280,"ancestor_record_offset":34944,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":1749,"record_offset":35280,"parent_record_offset":35252,"ancestor_record_offset":34944,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1749,"record_offset":35840,"parent_record_offset":35804,"ancestor_record_offset":34944,"inlinee":4668,"source_line":2098,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$42::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1749,"record_offset":35804,"parent_record_offset":35776,"ancestor_record_offset":34944,"inlinee":4666,"source_line":2807,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::for_value_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1749,"record_offset":35308,"parent_record_offset":35280,"ancestor_record_offset":34944,"inlinee":4689,"source_line":253,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::align_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1749,"record_offset":35368,"parent_record_offset":35336,"ancestor_record_offset":34944,"inlinee":4691,"source_line":566,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val_raw
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1749,"record_offset":35336,"parent_record_offset":35308,"ancestor_record_offset":34944,"inlinee":4690,"source_line":427,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1749,"record_offset":35252,"parent_record_offset":35224,"ancestor_record_offset":34944,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1749,"record_offset":35872,"parent_record_offset":35840,"ancestor_record_offset":34944,"inlinee":4669,"source_line":440,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1749,"record_offset":35900,"parent_record_offset":35840,"ancestor_record_offset":34944,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":1749,"record_offset":35932,"parent_record_offset":35804,"ancestor_record_offset":34944,"inlinee":4671,"source_line":4369,"archive":"_pdb/C/modules/01749.jsonl.gz"}
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
// 完整结构及原始类型记录: mod_icons.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] clutter_pack_pipe::mod_icons::BakeModClutterIcon
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<clutter_pack_pipe::mod_icons::BakeModClutterIcon>", "constructor": "core::ptr::drop_in_place", "arguments": ["clutter_pack_pipe::mod_icons::BakeModClutterIcon"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::ArcInner<clutter_pack_pipe::mod_icons::BakeModClutterIcon>", "constructor": "alloc::sync::ArcInner", "arguments": ["clutter_pack_pipe::mod_icons::BakeModClutterIcon"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::ArcInner<clutter_pack_pipe::mod_icons::BakeModClutterIcon> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::ArcInner<clutter_pack_pipe::mod_icons::BakeModClutterIcon>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::type_name<clutter_pack_pipe::mod_icons::BakeModClutterIcon>", "constructor": "core::any::type_name", "arguments": ["clutter_pack_pipe::mod_icons::BakeModClutterIcon"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::BuildContext::require<clutter_pack_pipe::mod_icons::BakeModClutterIcon>", "constructor": "turbosloth::asset::BuildContext::require", "arguments": ["clutter_pack_pipe::mod_icons::BakeModClutterIcon"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$24::run::async_block_env$0<clutter_pack_pipe::ClutterIconImage,anyhow::Error,clutter_pack_pipe::mod_icons::BakeModClutterIcon>", "constructor": "turbosloth::asset::impl$24::run::async_block_env$0", "arguments": ["clutter_pack_pipe::ClutterIconImage", "anyhow::Error", "clutter_pack_pipe::mod_icons::BakeModClutterIcon"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<turbosloth::asset::impl$24::run::async_block_env$0<clutter_pack_pipe::ClutterIconImage,anyhow::Error,clutter_pack_pipe::mod_icons::BakeModClutterIcon> >", "constructor": "enum2$", "arguments": ["turbosloth::asset::impl$24::run::async_block_env$0<clutter_pack_pipe::ClutterIconImage,anyhow::Error,clutter_pack_pipe::mod_icons::BakeModClutterIcon>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<turbosloth::asset::impl$24::run::async_block_env$0<clutter_pack_pipe::ClutterIconImage,anyhow::Error,clutter_pack_pipe::mod_icons::BakeModClutterIcon> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<turbosloth::asset::impl$24::run::async_block_env$0<clutter_pack_pipe::ClutterIconImage,anyhow::Error,clutter_pack_pipe::mod_icons::BakeModClutterIcon> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$25::debug_name<clutter_pack_pipe::ClutterIconImage,anyhow::Error,clutter_pack_pipe::mod_icons::BakeModClutterIcon>", "constructor": "turbosloth::asset::impl$25::debug_name", "arguments": ["clutter_pack_pipe::ClutterIconImage", "anyhow::Error", "clutter_pack_pipe::mod_icons::BakeModClutterIcon"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$25::clone_boxed<clutter_pack_pipe::ClutterIconImage,anyhow::Error,clutter_pack_pipe::mod_icons::BakeModClutterIcon>", "constructor": "turbosloth::asset::impl$25::clone_boxed", "arguments": ["clutter_pack_pipe::ClutterIconImage", "anyhow::Error", "clutter_pack_pipe::mod_icons::BakeModClutterIcon"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$25::version_registry_exempt<clutter_pack_pipe::ClutterIconImage,anyhow::Error,clutter_pack_pipe::mod_icons::BakeModClutterIcon>", "constructor": "turbosloth::asset::impl$25::version_registry_exempt", "arguments": ["clutter_pack_pipe::ClutterIconImage", "anyhow::Error", "clutter_pack_pipe::mod_icons::BakeModClutterIcon"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$25::version<clutter_pack_pipe::ClutterIconImage,anyhow::Error,clutter_pack_pipe::mod_icons::BakeModClutterIcon>", "constructor": "turbosloth::asset::impl$25::version", "arguments": ["clutter_pack_pipe::ClutterIconImage", "anyhow::Error", "clutter_pack_pipe::mod_icons::BakeModClutterIcon"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$25::run_boxed<clutter_pack_pipe::ClutterIconImage,anyhow::Error,clutter_pack_pipe::mod_icons::BakeModClutterIcon>", "constructor": "turbosloth::asset::impl$25::run_boxed", "arguments": ["clutter_pack_pipe::ClutterIconImage", "anyhow::Error", "clutter_pack_pipe::mod_icons::BakeModClutterIcon"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$24::run::async_block$0<clutter_pack_pipe::ClutterIconImage,anyhow::Error,clutter_pack_pipe::mod_icons::BakeModClutterIcon>", "constructor": "turbosloth::asset::impl$24::run::async_block$0", "arguments": ["clutter_pack_pipe::ClutterIconImage", "anyhow::Error", "clutter_pack_pipe::mod_icons::BakeModClutterIcon"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] clutter_pack_pipe::mod_icons::ModClutterIconBuildRequest
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<clutter_pack_pipe::mod_icons::ModClutterIconBuildRequest,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["clutter_pack_pipe::mod_icons::ModClutterIconBuildRequest", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<clutter_pack_pipe::mod_icons::ModClutterIconBuildRequest,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<clutter_pack_pipe::mod_icons::ModClutterIconBuildRequest,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<clutter_pack_pipe::mod_icons::ModClutterIconBuildRequest>", "constructor": "ref$", "arguments": ["clutter_pack_pipe::mod_icons::ModClutterIconBuildRequest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<clutter_pack_pipe::mod_icons::ModClutterIconBuildRequest>,core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,enum2$<core::result::Result<turbosloth::asset::BuiltAsset<clutter_pack_pipe::ClutterIconImage>,turbosloth::asset::AssetBuildError> > > >,core::marker::Send>,alloc::alloc::Global> > >", "constructor": "tuple$", "arguments": ["ref$<clutter_pack_pipe::mod_icons::ModClutterIconBuildRequest>", "core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,enum2$<core::result::Result<turbosloth::asset::BuiltAsset<clutter_pack_pipe::ClutterIconImage>,turbosloth::asset::AssetBuildError> > > >,core::marker::Send>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<ref$<clutter_pack_pipe::mod_icons::ModClutterIconBuildRequest>,core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,enum2$<core::result::Result<turbosloth::asset::BuiltAsset<clutter_pack_pipe::ClutterIconImage>,turbosloth::asset::AssetBuildError> > > >,core::marker::Send>,alloc::alloc::Global> > >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<ref$<clutter_pack_pipe::mod_icons::ModClutterIconBuildRequest>,core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,enum2$<core::result::Result<turbosloth::asset::BuiltAsset<clutter_pack_pipe::ClutterIconImage>,turbosloth::asset::AssetBuildError> > > >,core::marker::Send>,alloc::alloc::Global> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tuple$<ref$<clutter_pack_pipe::mod_icons::ModClutterIconBuildRequest>,core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,enum2$<core::result::Result<turbosloth::asset::BuiltAsset<clutter_pack_pipe::ClutterIconImage>,turbosloth::asset::AssetBuildError> > > >,core::marker::Send>,alloc::alloc::Global> > >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tuple$<ref$<clutter_pack_pipe::mod_icons::ModClutterIconBuildRequest>,core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,enum2$<core::result::Result<turbosloth::asset::BuiltAsset<clutter_pack_pipe::ClutterIconImage>,turbosloth::asset::AssetBuildError> > > >,core::marker::Send>,alloc::alloc::Global> > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<tuple$<ref$<clutter_pack_pipe::mod_icons::ModClutterIconBuildRequest>,core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,enum2$<core::result::Result<turbosloth::asset::BuiltAsset<clutter_pack_pipe::ClutterIconImage>,turbosloth::asset::AssetBuildError> > > >,core::marker::Send>,alloc::alloc::Global> > >,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["tuple$<ref$<clutter_pack_pipe::mod_icons::ModClutterIconBuildRequest>,core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,enum2$<core::result::Result<turbosloth::asset::BuiltAsset<clutter_pack_pipe::ClutterIconImage>,turbosloth::asset::AssetBuildError> > > >,core::marker::Send>,alloc::alloc::Global> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<tuple$<ref$<clutter_pack_pipe::mod_icons::ModClutterIconBuildRequest>,core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,enum2$<core::result::Result<turbosloth::asset::BuiltAsset<clutter_pack_pipe::ClutterIconImage>,turbosloth::asset::AssetBuildError> > > >,core::marker::Send>,alloc::alloc::Global> > >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<tuple$<ref$<clutter_pack_pipe::mod_icons::ModClutterIconBuildRequest>,core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,enum2$<core::result::Result<turbosloth::asset::BuiltAsset<clutter_pack_pipe::ClutterIconImage>,turbosloth::asset::AssetBuildError> > > >,core::marker::Send>,alloc::alloc::Global> > >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<clutter_pack_pipe::mod_icons::ModClutterIconBuildRequest>", "constructor": "core::ptr::drop_in_place", "arguments": ["clutter_pack_pipe::mod_icons::ModClutterIconBuildRequest"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: mod_icons.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: mod_icons.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >)
// 候选参数类型: ["core::pin::Pin<alloc::boxed::Box<dyn$<core::future::future::Future<assoc$<Output,tuple$<> > >,core::marker::Send>,alloc::alloc::Global> >"]

// ------------------------------------------------------------------
// 函数: clutter_pack_pipe::mod_icons::build_mod_clutter_icons
// 信息 ID: d007641ce9cd6ca6a7450ab7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons; module=576, inline_record=27720; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: clutter_pack_pipe::mod_icons::build_mod_clutter_icons::async_fn$0
// 信息 ID: 025593bcd91c2209302c0f39
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] std::thread::local::LocalKey::with<core::cell::RefCell<tuple$<parking::Parker,core::task::wake::Waker> >,futures_lite::future::block_on::closure_env$0<enum2$<core::result::Result<tuple$<>,anyhow::Error> >,enum2$<clutter_pack_pipe::mod_icons::build_mod_clutter_icons::async_fn_env$0<country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons::closure_env$1> > >,enum2$<core::result::Result<tuple$<>,anyhow::Error> > >; module=1647, inline_record=104880; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: clutter_pack_pipe::mod_icons::impl$0::run
// 信息 ID: 7439158e3b454f681747f576
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] turbosloth::asset::impl$24::run::async_block$0<clutter_pack_pipe::ClutterIconImage,anyhow::Error,clutter_pack_pipe::mod_icons::BakeModClutterIcon> @ 0x1b28e79 -> 0xce9940 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: clutter_pack_pipe::mod_icons::impl$0::run::async_fn$0
// 信息 ID: 2c989ea88fbb80578b1b6330
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::asset::impl$24::run::async_block$0<clutter_pack_pipe::ClutterIconImage,anyhow::Error,clutter_pack_pipe::mod_icons::BakeModClutterIcon>; module=1779, inline_record=46796; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: clutter_pack_pipe::mod_icons::impl$0::run::async_fn$0::closure$0
// 信息 ID: 247c3164177ca8b7baa966a0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "clutter_pack_pipe::mod_icons::impl$0::run::async_fn$0", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::asset::impl$24::run::async_block$0<clutter_pack_pipe::ClutterIconImage,anyhow::Error,clutter_pack_pipe::mod_icons::BakeModClutterIcon>; module=1779, inline_record=48596; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: clutter_pack_pipe::mod_icons::impl$0::run::async_fn$0::closure$1
// 信息 ID: 3284dd3f4de5778c073e7f38
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "clutter_pack_pipe::mod_icons::impl$0::run::async_fn$0", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::asset::impl$24::run::async_block$0<clutter_pack_pipe::ClutterIconImage,anyhow::Error,clutter_pack_pipe::mod_icons::BakeModClutterIcon>; module=1779, inline_record=52780; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: clutter_pack_pipe::mod_icons::impl$0::run::async_fn$0::closure$2
// 信息 ID: bf1da47a57ebe653a7ce1cbb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "clutter_pack_pipe::mod_icons::impl$0::run::async_fn$0", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::asset::impl$24::run::async_block$0<clutter_pack_pipe::ClutterIconImage,anyhow::Error,clutter_pack_pipe::mod_icons::BakeModClutterIcon>; module=1779, inline_record=53296; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: clutter_pack_pipe::mod_icons::impl$1::clone
// 信息 ID: 62adf58a55a5b8b1aab08452
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::asset::impl$25::clone_boxed<clutter_pack_pipe::ClutterIconImage,anyhow::Error,clutter_pack_pipe::mod_icons::BakeModClutterIcon>; module=1779, inline_record=43296; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: clutter_pack_pipe::mod_icons::impl$2::hash
// 信息 ID: b9d7d05f6233f211e0a75070
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::asset::BuildContext::require<clutter_pack_pipe::mod_icons::BakeModClutterIcon>; module=1651, inline_record=48132; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: clutter_pack_pipe::mod_icons::mod_clutter_icon_package_dir
// 信息 ID: 19723d1a85a8e99a6fc5dd03
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons @ 0xb93510 -> 0xce9890 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::thread::local::LocalKey::with<core::cell::RefCell<tuple$<parking::Parker,core::task::wake::Waker> >,futures_lite::future::block_on::closure_env$0<enum2$<core::result::Result<tuple$<>,anyhow::Error> >,enum2$<clutter_pack_pipe::mod_icons::build_mod_clutter_icons::async_fn_env$0<country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons::closure_env$1> > >,enum2$<core::result::Result<tuple$<>,anyhow::Error> > > @ 0x16ff040 -> 0xce9890 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: core::ptr::drop_in_place<enum2$<clutter_pack_pipe::mod_icons::build_mod_clutter_icons::async_fn_env$0<country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons::closure_env$1> > >
// 信息 ID: ae253ee2af0fc90f4d341d98
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] clutter_pack_pipe::mod_icons::build_mod_clutter_icons::async_fn_env$0 -> ["country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons::closure_env$1"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["clutter_pack_pipe::mod_icons::build_mod_clutter_icons::async_fn_env$0<country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons::closure_env$1>"]; 字段布局 = unknown。
// [类型关系] core::ptr::drop_in_place -> ["enum2$<clutter_pack_pipe::mod_icons::build_mod_clutter_icons::async_fn_env$0<country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons::closure_env$1> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons @ 0xb9316a -> 0x1a86a40 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: core::ptr::drop_in_place<enum2$<clutter_pack_pipe::mod_icons::impl$0::run::async_fn_env$0> >
// 信息 ID: f4f511d2975b4d52d008649a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["clutter_pack_pipe::mod_icons::impl$0::run::async_fn_env$0"]; 字段布局 = unknown。
// [类型关系] core::ptr::drop_in_place -> ["enum2$<clutter_pack_pipe::mod_icons::impl$0::run::async_fn_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<enum2$<turbosloth::asset::impl$24::run::async_block_env$0<clutter_pack_pipe::ClutterIconImage,anyhow::Error,clutter_pack_pipe::mod_icons::BakeModClutterIcon> > > @ 0x1b2881e -> 0x1b28100 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] turbosloth::asset::impl$24::run::async_block$0<clutter_pack_pipe::ClutterIconImage,anyhow::Error,clutter_pack_pipe::mod_icons::BakeModClutterIcon> @ 0x1b29c7a -> 0x1b28100 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
