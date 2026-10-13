// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/usdl/src/sdl.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: daacc315cb051f32a1dc0af12776538de32b03be87b43224017ba26aabfec9cf
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: sdl.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 281, 285, 287
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use usdl; // namespace candidate
//   package identities: [{"name":"usdl","source":"local_or_path","version":"0.1.0"}]
// use usdl_sys; // namespace candidate
//   package identities: [{"name":"usdl-sys","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: usdl::sdl::get_error
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":281,"record_offset":2420,"rva":7119952,"va":5375829072}
// 编译实例 RVAs: 0x6ca450
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String>::from_utf8_lossy
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t8956008","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375829072,"raw_hex":"e868a88800","size":5,"soft_fail":false,"target_va":5384785136,"va":5375829123,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-150493","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375829072,"raw_hex":"e823b4fdff","size":5,"soft_fail":false,"target_va":5375678688,"va":5375829176,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=2
//     target_aliases=1699 candidates; complete list: sdl.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7106974","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375829072,"raw_hex":"e8628e93ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5375829161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: sdl.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41227753","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375829072,"raw_hex":"e8e9157502","size":5,"soft_fail":false,"target_va":5417057056,"va":5375829298,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bf30
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: sdl.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t41228989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375829072,"raw_hex":"e8bd1a7502","size":5,"soft_fail":false,"target_va":5417058096,"va":5375829102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t40791633","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375829072,"raw_hex":"e8516e6e02","size":5,"soft_fail":false,"target_va":5416620794,"va":5375829156,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl_sys::uSDL_GetError
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9661485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375829072,"raw_hex":"e82d6c9300","size":5,"soft_fail":false,"target_va":5385490576,"va":5375829086,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::borrow::Cow::into_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\borrow.rs
//     evidence={"module":281,"record_offset":2724,"parent_record_offset":2420,"ancestor_record_offset":2420,"inlinee":5422,"source_line":331,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":281,"record_offset":3356,"parent_record_offset":2420,"ancestor_record_offset":2420,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_direct] core::ffi::c_str::CStr::from_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ffi\c_str.rs
//     evidence={"module":281,"record_offset":2604,"parent_record_offset":2420,"ancestor_record_offset":2420,"inlinee":8370,"source_line":253,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::const_ptr::impl$0::is_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":281,"record_offset":2540,"parent_record_offset":2420,"ancestor_record_offset":2420,"inlinee":19030,"source_line":22,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":281,"record_offset":2696,"parent_record_offset":2420,"ancestor_record_offset":2420,"inlinee":12073,"source_line":136,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":281,"record_offset":3172,"parent_record_offset":3148,"ancestor_record_offset":2420,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":281,"record_offset":3196,"parent_record_offset":3172,"ancestor_record_offset":2420,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":281,"record_offset":3220,"parent_record_offset":3196,"ancestor_record_offset":2420,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":281,"record_offset":3148,"parent_record_offset":3000,"ancestor_record_offset":2420,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":281,"record_offset":2940,"parent_record_offset":2912,"ancestor_record_offset":2420,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":281,"record_offset":3000,"parent_record_offset":2968,"ancestor_record_offset":2420,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":281,"record_offset":2968,"parent_record_offset":2940,"ancestor_record_offset":2420,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":281,"record_offset":3036,"parent_record_offset":3000,"ancestor_record_offset":2420,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":281,"record_offset":2824,"parent_record_offset":2788,"ancestor_record_offset":2420,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":281,"record_offset":2852,"parent_record_offset":2824,"ancestor_record_offset":2420,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":281,"record_offset":2880,"parent_record_offset":2852,"ancestor_record_offset":2420,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":281,"record_offset":2788,"parent_record_offset":2760,"ancestor_record_offset":2420,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":281,"record_offset":2760,"parent_record_offset":2724,"ancestor_record_offset":2420,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":281,"record_offset":2912,"parent_record_offset":2880,"ancestor_record_offset":2420,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":281,"record_offset":3060,"parent_record_offset":3036,"ancestor_record_offset":2420,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::strlen
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ffi\c_str.rs
//     evidence={"module":281,"record_offset":2628,"parent_record_offset":2604,"ancestor_record_offset":2420,"inlinee":8372,"source_line":737,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::ffi::c_str::strlen::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":281,"record_offset":2656,"parent_record_offset":2628,"ancestor_record_offset":2420,"inlinee":8374,"source_line":2434,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":281,"record_offset":3108,"parent_record_offset":3084,"ancestor_record_offset":2420,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":281,"record_offset":3084,"parent_record_offset":3060,"ancestor_record_offset":2420,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":281,"record_offset":3276,"parent_record_offset":2880,"ancestor_record_offset":2420,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::is_null::runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":281,"record_offset":2568,"parent_record_offset":2540,"ancestor_record_offset":2420,"inlinee":19032,"source_line":2434,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":281,"record_offset":3300,"parent_record_offset":3276,"ancestor_record_offset":2420,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::sdl::init
// 记录形式: compiled_procedure; instances=1
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":281,"record_offset":1896,"rva":7119712,"va":5375828832}
// 编译实例 RVAs: 0x6ca360
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] usdl::sdl::get_error
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375828832,"raw_hex":"e8a4000000","size":5,"soft_fail":false,"target_va":5375829072,"va":5375828903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl_sys::uSDL_Init
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9661930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375828832,"raw_hex":"e8ea6d9300","size":5,"soft_fail":false,"target_va":5385490816,"va":5375828881,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl::sdl::Sdl::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/sdl.rs
//     evidence={"module":281,"record_offset":1996,"parent_record_offset":1896,"ancestor_record_offset":1896,"inlinee":19026,"source_line":33,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":281,"record_offset":2032,"parent_record_offset":1996,"ancestor_record_offset":1896,"inlinee":4151,"source_line":3134,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":281,"record_offset":2084,"parent_record_offset":1996,"ancestor_record_offset":1896,"inlinee":19022,"source_line":2870,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":281,"record_offset":2052,"parent_record_offset":2032,"ancestor_record_offset":1896,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":281,"record_offset":2108,"parent_record_offset":2084,"ancestor_record_offset":1896,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::sdl::load_default_library
// 记录形式: compiled_procedure; instances=1
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":281,"record_offset":260,"rva":7119232,"va":5375828352}
// 编译实例 RVAs: 0x6ca180
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_external_jump_candidate] usdl_sys::backend::load_default_library
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tjmp\t10571","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5375828352,"raw_hex":"e94b290000","size":5,"soft_fail":false,"target_va":5375838928,"va":5375828352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::sdl::Sdl::new
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4A52
// 首条位置: {"module":281,"inlinee":19026}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":281,"record_offset":2032,"parent_record_offset":1996,"ancestor_record_offset":1996,"inlinee":4151,"source_line":3134,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":281,"record_offset":2084,"parent_record_offset":1996,"ancestor_record_offset":1996,"inlinee":19022,"source_line":2870,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":281,"record_offset":2052,"parent_record_offset":2032,"ancestor_record_offset":1996,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":281,"record_offset":2108,"parent_record_offset":2084,"ancestor_record_offset":1996,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::sdl::Sdl::video
// 记录形式: compiled_procedure; instances=1
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":281,"record_offset":1292,"rva":7119488,"va":5375828608}
// 编译实例 RVAs: 0x6ca280
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40793463","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375828608,"raw_hex":"e877756e02","size":5,"soft_fail":false,"target_va":5416622240,"va":5375828772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl::sdl::get_error
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t351","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375828608,"raw_hex":"e85f010000","size":5,"soft_fail":false,"target_va":5375829072,"va":5375828716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl_sys::uSDL_InitSubSystem
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9661959","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375828608,"raw_hex":"e8076e9300","size":5,"soft_fail":false,"target_va":5385490656,"va":5375828692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl_sys::uSDL_Quit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9662145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375828608,"raw_hex":"e8c16e9300","size":5,"soft_fail":false,"target_va":5385490896,"va":5375828746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl::sdl::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/sdl.rs
//     evidence={"module":281,"record_offset":1404,"parent_record_offset":1292,"ancestor_record_offset":1292,"inlinee":19019,"source_line":78,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_direct] usdl::video::VideoSubsystem::init
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/video.rs
//     evidence={"module":281,"record_offset":1492,"parent_record_offset":1292,"ancestor_record_offset":1292,"inlinee":19021,"source_line":26,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":281,"record_offset":1656,"parent_record_offset":1492,"ancestor_record_offset":1292,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":281,"record_offset":1432,"parent_record_offset":1404,"ancestor_record_offset":1292,"inlinee":4151,"source_line":3134,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":281,"record_offset":1708,"parent_record_offset":1680,"ancestor_record_offset":1292,"inlinee":4762,"source_line":3165,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":281,"record_offset":1600,"parent_record_offset":1492,"ancestor_record_offset":1292,"inlinee":19022,"source_line":2870,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":281,"record_offset":1456,"parent_record_offset":1432,"ancestor_record_offset":1292,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":281,"record_offset":1624,"parent_record_offset":1600,"ancestor_record_offset":1292,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":281,"record_offset":1732,"parent_record_offset":1708,"ancestor_record_offset":1292,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] usdl::sdl::impl$1::drop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/sdl.rs
//     evidence={"module":281,"record_offset":1680,"parent_record_offset":1656,"ancestor_record_offset":1292,"inlinee":19024,"source_line":69,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::sdl::Sdl::event_pump
// 记录形式: compiled_procedure; instances=1
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":281,"record_offset":368,"rva":7119248,"va":5375828368}
// 编译实例 RVAs: 0x6ca190
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-149720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375828368,"raw_hex":"e828b7fdff","size":5,"soft_fail":false,"target_va":5375678688,"va":5375828403,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: sdl.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7106201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375828368,"raw_hex":"e8679193ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5375828388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40792254","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375828368,"raw_hex":"e8be706e02","size":5,"soft_fail":false,"target_va":5416620794,"va":5375828535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40793715","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375828368,"raw_hex":"e873766e02","size":5,"soft_fail":false,"target_va":5416622240,"va":5375828520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":281,"record_offset":544,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicBool::swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":281,"record_offset":488,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":4686,"source_line":788,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_direct] usdl::sdl::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/sdl.rs
//     evidence={"module":281,"record_offset":1004,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":19019,"source_line":78,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":281,"record_offset":824,"parent_record_offset":800,"ancestor_record_offset":368,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":281,"record_offset":848,"parent_record_offset":824,"ancestor_record_offset":368,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":281,"record_offset":872,"parent_record_offset":848,"ancestor_record_offset":368,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":281,"record_offset":800,"parent_record_offset":772,"ancestor_record_offset":368,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":281,"record_offset":716,"parent_record_offset":688,"ancestor_record_offset":368,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":281,"record_offset":772,"parent_record_offset":744,"ancestor_record_offset":368,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":281,"record_offset":744,"parent_record_offset":716,"ancestor_record_offset":368,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":281,"record_offset":600,"parent_record_offset":572,"ancestor_record_offset":368,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":281,"record_offset":628,"parent_record_offset":600,"ancestor_record_offset":368,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":281,"record_offset":656,"parent_record_offset":628,"ancestor_record_offset":368,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":281,"record_offset":572,"parent_record_offset":544,"ancestor_record_offset":368,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":281,"record_offset":688,"parent_record_offset":656,"ancestor_record_offset":368,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":281,"record_offset":928,"parent_record_offset":656,"ancestor_record_offset":368,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":281,"record_offset":952,"parent_record_offset":928,"ancestor_record_offset":368,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":281,"record_offset":1032,"parent_record_offset":1004,"ancestor_record_offset":368,"inlinee":4151,"source_line":3134,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":281,"record_offset":1056,"parent_record_offset":1032,"ancestor_record_offset":368,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_swap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":281,"record_offset":512,"parent_record_offset":488,"ancestor_record_offset":368,"inlinee":4687,"source_line":3902,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::sdl::Sdl::mouse
// 记录形式: compiled_procedure; instances=1
// 源码行: 59
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":281,"record_offset":1096,"rva":7119424,"va":5375828544}
// 编译实例 RVAs: 0x6ca240
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t40793646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375828544,"raw_hex":"e82e766e02","size":5,"soft_fail":false,"target_va":5416622240,"va":5375828589,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl::sdl::impl$2::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/sdl.rs
//     evidence={"module":281,"record_offset":1204,"parent_record_offset":1096,"ancestor_record_offset":1096,"inlinee":19019,"source_line":78,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU32::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":281,"record_offset":1232,"parent_record_offset":1204,"ancestor_record_offset":1096,"inlinee":4151,"source_line":3134,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":281,"record_offset":1252,"parent_record_offset":1232,"ancestor_record_offset":1096,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::sdl::impl$1::drop
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 69
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4A50
// 首条位置: {"module":281,"inlinee":19024}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::fetch_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":281,"record_offset":1708,"parent_record_offset":1680,"ancestor_record_offset":1680,"inlinee":4762,"source_line":3165,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":281,"record_offset":1732,"parent_record_offset":1708,"ancestor_record_offset":1680,"inlinee":4670,"source_line":3936,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::sdl::impl$2::clone
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 78
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x4A4B
// 首条位置: {"module":281,"inlinee":19019}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU32::fetch_add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":281,"record_offset":1032,"parent_record_offset":1004,"ancestor_record_offset":1004,"inlinee":4151,"source_line":3134,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":281,"record_offset":1056,"parent_record_offset":1032,"ancestor_record_offset":1004,"inlinee":4153,"source_line":3919,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::sdl::EventPump::poll_iter
// 记录形式: compiled_procedure; instances=1
// 源码行: 96
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":281,"record_offset":2312,"rva":7119936,"va":5375829056}
// 编译实例 RVAs: 0x6ca440
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::sdl::EventPump::wait_event_timeout
// 记录形式: compiled_procedure; instances=1
// 源码行: 100
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":281,"record_offset":2148,"rva":7119808,"va":5375828928}
// 编译实例 RVAs: 0x6ca3c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ca5f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: sdl.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t461","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375828928,"raw_hex":"e8cd010000","size":5,"soft_fail":false,"target_va":5375829488,"va":5375829022,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140957040
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: sdl.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t2673763","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375828928,"raw_hex":"e863cc2800","size":5,"soft_fail":false,"target_va":5378502720,"va":5375828952,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] usdl_sys::event::uSDL_WaitEventTimeout
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"usdl-sys","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t9662217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5375828928,"raw_hex":"e8096f9300","size":5,"soft_fail":false,"target_va":5385491184,"va":5375828962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] usdl::event::wait_event_timeout
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"usdl","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/usdl/src/event.rs
//     evidence={"module":281,"record_offset":2272,"parent_record_offset":2148,"ancestor_record_offset":2148,"inlinee":19029,"source_line":168,"archive":"_pdb/C/modules/00281.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: usdl::sdl::impl$4::drop
// 记录形式: compiled_procedure; instances=1
// 源码行: 106
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":287,"record_offset":676,"rva":7124896,"va":5375834016}
// 编译实例 RVAs: 0x6cb7a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicBool::store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":287,"record_offset":784,"parent_record_offset":676,"ancestor_record_offset":676,"inlinee":4773,"source_line":755,"archive":"_pdb/C/modules/00287.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_store
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":287,"record_offset":808,"parent_record_offset":784,"ancestor_record_offset":676,"inlinee":4774,"source_line":3871,"archive":"_pdb/C/modules/00287.jsonl.gz"}
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
// 完整结构及原始类型记录: sdl.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] usdl::sdl::EventPump
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<usdl::sdl::EventPump>", "constructor": "core::ptr::drop_in_place", "arguments": ["usdl::sdl::EventPump"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] usdl::sdl::Sdl
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<usdl::sdl::Sdl>", "constructor": "core::ptr::drop_in_place", "arguments": ["usdl::sdl::Sdl"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: sdl.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: sdl.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)
// 候选参数类型: ["country_core::systems::collision::world::SubworldColliderHandle", "ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >"]

// ------------------------------------------------------------------
// 函数: usdl::sdl::EventPump::poll_iter
// 信息 ID: 65398f8ad4212ae3cf9c91b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 137943db23b17086ce8b9d61: bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)
// 引用方: 原程序直接转移 206 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::load_language_library::load_language_library @ 0xf2741 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x15092b -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x150952 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x150bcc -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x150cd4 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x150cfe -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x14fde0 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_mod_editor::do_gui_mod_editor @ 0x150e75 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_debug::do_gui_debug @ 0x1879d5 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_debug::do_gui_debug @ 0x1875c2 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::gui::gui_debug::do_gui_debug @ 0x1878ea -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x1c3be3 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x1c3bf9 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x1c3cb6 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x22453b -> 0x6ca440 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x224c02 -> 0x6ca440 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::sdl::EventPump::wait_event_timeout
// 信息 ID: b1cd56d871b33df0e9c7754a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_event_loop::process_event_batch<tiny_glade::sdl_event_loop::run_event_loop::closure_env$0> @ 0x52e0ca -> 0x6ca3c0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_iced::gui_runner_sdl::run_gui<tg_gpu_selector::gpu_selection::Controls> @ 0x52fbca -> 0x6ca3c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::sdl::Sdl::event_pump
// 信息 ID: 1ffb44b0b6a1d04a2fb52430
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_setup::init_sdl @ 0x3fa8ed -> 0x6ca190 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::sdl::Sdl::mouse
// 信息 ID: 9130d6643eaf424905b1eeec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_event_loop::run_event_loop @ 0x5135d2 -> 0x6ca240 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::sdl_event_loop::run_event_loop @ 0x5135db -> 0x6ca240 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::sdl::Sdl::new
// 信息 ID: bd0c6ada9096fd9b76d14207
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] usdl::sdl::init; module=281, inline_record=1996; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::sdl::Sdl::video
// 信息 ID: 5606254d70967be20a107fac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_setup::init_sdl @ 0x3fa903 -> 0x6ca280 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::sdl::get_error
// 信息 ID: c1dc9c28049c9052ed6734e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 14 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] usdl::sdl::Sdl::video @ 0x6ca2ec -> 0x6ca450 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl::sdl::init @ 0x6ca3a7 -> 0x6ca450 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl::mouse::Cursor::from_system @ 0x6caa94 -> 0x6ca450 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl::video::WindowBuilder::build @ 0x6cac0e -> 0x6ca450 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl::video::VideoSubsystem::display_mode @ 0x6cad52 -> 0x6ca450 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl::video::VideoSubsystem::display_name @ 0x6caddc -> 0x6ca450 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl::video::VideoSubsystem::display_bounds @ 0x6cae84 -> 0x6ca450 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl::video::VideoSubsystem::num_display_modes @ 0x6caef0 -> 0x6ca450 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl::video::VideoSubsystem::num_video_displays @ 0x6caf2d -> 0x6ca450 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl::video::VideoSubsystem::current_display_mode @ 0x6caf6d -> 0x6ca450 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl::video::VideoSubsystem::desktop_display_mode @ 0x6cb02d -> 0x6ca450 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl::video::Window::display_index @ 0x6cb270 -> 0x6ca450 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl::video::Window::set_fullscreen @ 0x6cb3da -> 0x6ca450 (direct_call); 同地址别名未裁定。
// [静态直接引用] usdl::video::Window::set_display_mode @ 0x6cb47d -> 0x6ca450 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::sdl::impl$1::drop
// 信息 ID: 4ca64b472f5bb8bf798df70c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] usdl::sdl::Sdl::video; module=281, inline_record=1680; 不是独立调用边。
// [PDB 内联宿主] usdl::sdl::Sdl::video; module=281, inline_record=1796; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<usdl::sdl::Sdl>; module=287, inline_record=384; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::sdl::impl$2::clone
// 信息 ID: 2a0e6d7c299355f97582addc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] usdl::sdl::Sdl::event_pump; module=281, inline_record=1004; 不是独立调用边。
// [PDB 内联宿主] usdl::sdl::Sdl::mouse; module=281, inline_record=1204; 不是独立调用边。
// [PDB 内联宿主] usdl::sdl::Sdl::video; module=281, inline_record=1404; 不是独立调用边。
// [PDB 内联宿主] usdl::video::WindowBuilder::build; module=285, inline_record=744; 不是独立调用边。
// [PDB 内联宿主] usdl::video::VideoSubsystem::window; module=285, inline_record=4804; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::sdl::impl$4::drop
// 信息 ID: 7948b5cca8f3ead6007aa8d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<usdl::sdl::EventPump> @ 0x999e6 -> 0x6cb7a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<tiny_glade::sdl_setup::SdlWindowBackend> @ 0xac10a -> 0x6cb7a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::sdl_setup::init_sdl @ 0x3fa9b2 -> 0x6cb7a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::sdl::init
// 信息 ID: 2d8b782496a1978daf21bbea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::sdl_setup::init_sdl @ 0x3fa8c7 -> 0x6ca360 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: usdl::sdl::load_default_library
// 信息 ID: af2f9a73023132f0bc38cc7c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game @ 0x42c952 -> 0x6ca180 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
