// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/asset/src/texture.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 5254b8a561bbdc777f0e22b3c3cf0e6c5984587d80db8a02e585383218a57c6a
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: texture.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 502, 1651, 1666, 1670, 1671, 1676
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use asset; // namespace candidate
//   package identities: [{"name":"asset","source":"local_or_path","version":"0.1.0"}]
// use core; // namespace candidate
//   package identities: []
// use rkyv; // namespace candidate
//   package identities: [{"name":"rkyv","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.16"}]
// use turbosloth; // namespace candidate
//   package identities: [{"name":"turbosloth","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: asset::texture::impl$1::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13F67
// 首条位置: {"module":1651,"inlinee":81767}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1651,"record_offset":350948,"parent_record_offset":350920,"ancestor_record_offset":350920,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1651,"record_offset":351344,"parent_record_offset":351320,"ancestor_record_offset":350920,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1651,"record_offset":351368,"parent_record_offset":351344,"ancestor_record_offset":350920,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1651,"record_offset":351392,"parent_record_offset":351368,"ancestor_record_offset":350920,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1651,"record_offset":351320,"parent_record_offset":351164,"ancestor_record_offset":350920,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1651,"record_offset":351100,"parent_record_offset":351072,"ancestor_record_offset":350920,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1651,"record_offset":351164,"parent_record_offset":351128,"ancestor_record_offset":350920,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1651,"record_offset":351128,"parent_record_offset":351100,"ancestor_record_offset":350920,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1651,"record_offset":351208,"parent_record_offset":351164,"ancestor_record_offset":350920,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1651,"record_offset":351040,"parent_record_offset":350976,"ancestor_record_offset":350920,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1651,"record_offset":351012,"parent_record_offset":350976,"ancestor_record_offset":350920,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1651,"record_offset":351072,"parent_record_offset":351040,"ancestor_record_offset":350920,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1651,"record_offset":350976,"parent_record_offset":350948,"ancestor_record_offset":350920,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1651,"record_offset":351232,"parent_record_offset":351208,"ancestor_record_offset":350920,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1651,"record_offset":351280,"parent_record_offset":351256,"ancestor_record_offset":350920,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1651,"record_offset":351256,"parent_record_offset":351232,"ancestor_record_offset":350920,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1651,"record_offset":351448,"parent_record_offset":351040,"ancestor_record_offset":350920,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1651,"record_offset":351472,"parent_record_offset":351448,"ancestor_record_offset":350920,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::impl$2::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13F2B
// 首条位置: {"module":1651,"inlinee":81707}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1651,"record_offset":17796,"parent_record_offset":17632,"ancestor_record_offset":17632,"inlinee":14306,"source_line":3793,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_direct] asset::texture::impl$10::hash
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture.rs
//     evidence={"module":1651,"record_offset":17660,"parent_record_offset":17632,"ancestor_record_offset":17632,"inlinee":81709,"source_line":13,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":17704,"parent_record_offset":17680,"ancestor_record_offset":17632,"inlinee":14034,"source_line":429,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":17848,"parent_record_offset":17820,"ancestor_record_offset":17632,"inlinee":14307,"source_line":483,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":17728,"parent_record_offset":17704,"ancestor_record_offset":17632,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":17680,"parent_record_offset":17660,"ancestor_record_offset":17632,"inlinee":14033,"source_line":812,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":17820,"parent_record_offset":17796,"ancestor_record_offset":17632,"inlinee":14368,"source_line":933,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$9::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":17932,"parent_record_offset":17820,"ancestor_record_offset":17632,"inlinee":14369,"source_line":817,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1651,"record_offset":17752,"parent_record_offset":17728,"ancestor_record_offset":17632,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::impl$3::resolve
// 记录形式: compiled_procedure; instances=1
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1676,"record_offset":496,"rva":25139872,"va":5393848992}
// 编译实例 RVAs: 0x17f9aa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$rancor..Panic$u20$as$u20$rancor..Source$GT$::new
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t23019086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393848992,"raw_hex":"e84e3e5f01","size":5,"soft_fail":false,"target_va":5416868112,"va":5393849021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] asset::texture::_::impl$0::resolve
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture.rs
//     evidence={"module":1676,"record_offset":620,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":82645,"source_line":13,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_direct] rkyv::impls::alloc::vec::impl$0::resolve
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/alloc/vec.rs
//     evidence={"module":1676,"record_offset":756,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":42157,"source_line":19,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_direct] rkyv::place::Place::from_field_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/place.rs
//     evidence={"module":1676,"record_offset":728,"parent_record_offset":496,"ancestor_record_offset":496,"inlinee":42155,"source_line":47,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1676,"record_offset":784,"parent_record_offset":756,"ancestor_record_offset":496,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::ptr_try_from_impls::impl$17::try_from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\num.rs
//     evidence={"module":1676,"record_offset":1036,"parent_record_offset":1008,"ancestor_record_offset":496,"inlinee":42168,"source_line":319,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1676,"record_offset":976,"parent_record_offset":940,"ancestor_record_offset":496,"inlinee":42164,"source_line":2927,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1676,"record_offset":664,"parent_record_offset":640,"ancestor_record_offset":496,"inlinee":5339,"source_line":1413,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1676,"record_offset":688,"parent_record_offset":664,"ancestor_record_offset":496,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::impls::core::primitive::impl$0::resolve
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/core/primitive.rs
//     evidence={"module":1676,"record_offset":1200,"parent_record_offset":820,"ancestor_record_offset":496,"inlinee":42170,"source_line":269,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::place::Place::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/place.rs
//     evidence={"module":1676,"record_offset":1076,"parent_record_offset":908,"ancestor_record_offset":496,"inlinee":42147,"source_line":89,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::place::Place::write_unchecked
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/place.rs
//     evidence={"module":1676,"record_offset":640,"parent_record_offset":620,"ancestor_record_offset":496,"inlinee":42148,"source_line":79,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::try_emplace
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1676,"record_offset":908,"parent_record_offset":880,"ancestor_record_offset":496,"inlinee":42161,"source_line":173,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RelPtr::emplace
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1676,"record_offset":852,"parent_record_offset":820,"ancestor_record_offset":496,"inlinee":42159,"source_line":410,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RelPtr::try_emplace
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1676,"record_offset":880,"parent_record_offset":852,"ancestor_record_offset":496,"inlinee":42160,"source_line":395,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::impl$19::from_isize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1676,"record_offset":1008,"parent_record_offset":908,"ancestor_record_offset":496,"inlinee":42166,"source_line":70,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::signed_offset
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1676,"record_offset":940,"parent_record_offset":908,"ancestor_record_offset":496,"inlinee":42163,"source_line":144,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::resolve_from_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":1676,"record_offset":820,"parent_record_offset":784,"ancestor_record_offset":496,"inlinee":42158,"source_line":79,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::impl$5::check_bytes
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6FB3
// 首条位置: {"module":1666,"inlinee":28595}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] asset::texture::_::impl$1::check_bytes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/texture.rs
//     evidence={"module":1666,"record_offset":109660,"parent_record_offset":109636,"ancestor_record_offset":109636,"inlinee":28550,"source_line":13,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_direct] rkyv::vec::impl$19::check_bytes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":1666,"record_offset":109688,"parent_record_offset":109636,"ancestor_record_offset":109636,"inlinee":28271,"source_line":28,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::metadata::from_raw_parts
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\metadata.rs
//     evidence={"module":1666,"record_offset":109968,"parent_record_offset":109944,"ancestor_record_offset":109636,"inlinee":28281,"source_line":116,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1666,"record_offset":109844,"parent_record_offset":109820,"ancestor_record_offset":109636,"inlinee":4729,"source_line":1258,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1666,"record_offset":109904,"parent_record_offset":109792,"ancestor_record_offset":109636,"inlinee":5228,"source_line":465,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1666,"record_offset":109868,"parent_record_offset":109844,"ancestor_record_offset":109636,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::slice_from_raw_parts
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1666,"record_offset":109944,"parent_record_offset":109712,"ancestor_record_offset":109636,"inlinee":28280,"source_line":1174,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr_wrapping
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1666,"record_offset":109768,"parent_record_offset":109744,"ancestor_record_offset":109636,"inlinee":28277,"source_line":325,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr_wrapping_raw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1666,"record_offset":109792,"parent_record_offset":109768,"ancestor_record_offset":109636,"inlinee":28278,"source_line":236,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::offset_raw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1666,"record_offset":109820,"parent_record_offset":109792,"ancestor_record_offset":109636,"inlinee":28279,"source_line":205,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RelPtr::as_ptr_wrapping
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1666,"record_offset":109744,"parent_record_offset":109712,"ancestor_record_offset":109636,"inlinee":28275,"source_line":606,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::verify::impl$0::verify
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":1666,"record_offset":109712,"parent_record_offset":109688,"ancestor_record_offset":109636,"inlinee":28273,"source_line":304,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::impl$6::serialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6F7A
// 首条位置: {"module":502,"inlinee":28538}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rkyv::impls::alloc::vec::impl$1::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/alloc/vec.rs
//     evidence={"module":502,"record_offset":21924,"parent_record_offset":21892,"ancestor_record_offset":21892,"inlinee":28163,"source_line":27,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":502,"record_offset":22008,"parent_record_offset":21984,"ancestor_record_offset":21892,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":502,"record_offset":22056,"parent_record_offset":22032,"ancestor_record_offset":21892,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":502,"record_offset":22032,"parent_record_offset":22008,"ancestor_record_offset":21892,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":502,"record_offset":21984,"parent_record_offset":21960,"ancestor_record_offset":21892,"inlinee":4178,"source_line":1913,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":502,"record_offset":21960,"parent_record_offset":21924,"ancestor_record_offset":21892,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":502,"record_offset":22368,"parent_record_offset":22332,"ancestor_record_offset":21892,"inlinee":4931,"source_line":2493,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":502,"record_offset":22448,"parent_record_offset":22280,"ancestor_record_offset":21892,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::impls::core::impl$14::serialize_unsized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/core/mod.rs
//     evidence={"module":502,"record_offset":22132,"parent_record_offset":22100,"ancestor_record_offset":21892,"inlinee":28167,"source_line":288,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::ser::impl$2::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/ser/mod.rs
//     evidence={"module":502,"record_offset":22208,"parent_record_offset":22172,"ancestor_record_offset":21892,"inlinee":28175,"source_line":57,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::ser::writer::WriterExt::align
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/ser/writer/mod.rs
//     evidence={"module":502,"record_offset":22516,"parent_record_offset":22492,"ancestor_record_offset":21892,"inlinee":28170,"source_line":94,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::ser::writer::WriterExt::align_for
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/ser/writer/mod.rs
//     evidence={"module":502,"record_offset":22492,"parent_record_offset":22132,"ancestor_record_offset":21892,"inlinee":28169,"source_line":104,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::ser::writer::WriterExt::pad
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/ser/writer/mod.rs
//     evidence={"module":502,"record_offset":22540,"parent_record_offset":22516,"ancestor_record_offset":21892,"inlinee":28171,"source_line":85,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::ser::writer::alloc::impl$3::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/ser/writer/alloc.rs
//     evidence={"module":502,"record_offset":22244,"parent_record_offset":22208,"ancestor_record_offset":21892,"inlinee":28177,"source_line":29,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::ser::writer::impl$4::write
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/ser/writer/mod.rs
//     evidence={"module":502,"record_offset":22172,"parent_record_offset":22132,"ancestor_record_offset":21892,"inlinee":28173,"source_line":77,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::util::alloc::aligned_vec::AlignedVec::as_mut_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/util/alloc/aligned_vec.rs
//     evidence={"module":502,"record_offset":22400,"parent_record_offset":22280,"ancestor_record_offset":21892,"inlinee":28188,"source_line":295,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::util::alloc::aligned_vec::AlignedVec::extend_from_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/util/alloc/aligned_vec.rs
//     evidence={"module":502,"record_offset":22280,"parent_record_offset":22244,"ancestor_record_offset":21892,"inlinee":28179,"source_line":567,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::util::alloc::aligned_vec::AlignedVec::reserve
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/util/alloc/aligned_vec.rs
//     evidence={"module":502,"record_offset":22332,"parent_record_offset":22280,"ancestor_record_offset":21892,"inlinee":28180,"source_line":398,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::serialize_from_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":502,"record_offset":22100,"parent_record_offset":21924,"ancestor_record_offset":21892,"inlinee":28165,"source_line":90,"archive":"_pdb/C/modules/00502.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::impl$7::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6FA9
// 首条位置: {"module":1666,"inlinee":28585}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1666,"record_offset":111292,"parent_record_offset":110808,"ancestor_record_offset":110808,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_direct] rkyv::impls::alloc::vec::impl$2::deserialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/alloc/vec.rs
//     evidence={"module":1666,"record_offset":110840,"parent_record_offset":110808,"ancestor_record_offset":110808,"inlinee":28517,"source_line":45,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1666,"record_offset":110940,"parent_record_offset":110840,"ancestor_record_offset":110808,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1666,"record_offset":111124,"parent_record_offset":111092,"ancestor_record_offset":110808,"inlinee":4729,"source_line":1258,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1666,"record_offset":111228,"parent_record_offset":111204,"ancestor_record_offset":110808,"inlinee":5339,"source_line":1413,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1666,"record_offset":111148,"parent_record_offset":111124,"ancestor_record_offset":110808,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1666,"record_offset":111252,"parent_record_offset":111228,"ancestor_record_offset":110808,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::impls::core::impl$15::deserialize_unsized
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/impls/core/mod.rs
//     evidence={"module":1666,"record_offset":111204,"parent_record_offset":110840,"ancestor_record_offset":110808,"inlinee":28442,"source_line":335,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1666,"record_offset":111044,"parent_record_offset":111020,"ancestor_record_offset":110808,"inlinee":28522,"source_line":290,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::as_ptr_raw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1666,"record_offset":111068,"parent_record_offset":111044,"ancestor_record_offset":110808,"inlinee":28523,"source_line":219,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RawRelPtr::offset_raw
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1666,"record_offset":111092,"parent_record_offset":111068,"ancestor_record_offset":110808,"inlinee":28279,"source_line":205,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::rel_ptr::RelPtr::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/rel_ptr.rs
//     evidence={"module":1666,"record_offset":111020,"parent_record_offset":110996,"ancestor_record_offset":110808,"inlinee":28521,"source_line":575,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::as_ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":1666,"record_offset":110996,"parent_record_offset":110972,"ancestor_record_offset":110808,"inlinee":28520,"source_line":40,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::as_slice
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":1666,"record_offset":110884,"parent_record_offset":110840,"ancestor_record_offset":110808,"inlinee":28518,"source_line":55,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   [pdb_inline_transitive] rkyv::vec::ArchivedVec::len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/vec.rs
//     evidence={"module":1666,"record_offset":110908,"parent_record_offset":110884,"ancestor_record_offset":110808,"inlinee":28519,"source_line":45,"archive":"_pdb/C/modules/01666.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::impl$8::visit_asset_dependencies
// 记录形式: compiled_procedure; instances=1
// 源码行: 7
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1676,"record_offset":3284,"rva":25140688,"va":5393849808}
// 编译实例 RVAs: 0x17f9dd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25127663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393849808,"raw_hex":"e8119580fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5393849850,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] turbosloth::asset::impl$6::visit_asset_dependencies
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":1676,"record_offset":3416,"parent_record_offset":3284,"ancestor_record_offset":3284,"inlinee":42184,"source_line":149,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1676,"record_offset":3504,"parent_record_offset":3444,"ancestor_record_offset":3284,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1676,"record_offset":3476,"parent_record_offset":3444,"ancestor_record_offset":3284,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1676,"record_offset":3444,"parent_record_offset":3416,"ancestor_record_offset":3284,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::_::impl$0::resolve
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x142D5
// 首条位置: {"module":1676,"inlinee":82645}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rkyv::place::Place::write_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"rkyv","version":"0.8.16","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rkyv-0.8.16/src/place.rs
//     evidence={"module":1676,"record_offset":640,"parent_record_offset":620,"ancestor_record_offset":620,"inlinee":42148,"source_line":79,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1676,"record_offset":664,"parent_record_offset":640,"ancestor_record_offset":620,"inlinee":5339,"source_line":1413,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1676,"record_offset":688,"parent_record_offset":664,"ancestor_record_offset":620,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::_::impl$1::check_bytes
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6F86
// 首条位置: {"module":502,"inlinee":28550}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::impl$10::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 13
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13F2D
// 首条位置: {"module":1651,"inlinee":81709}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":17680,"parent_record_offset":17660,"ancestor_record_offset":17660,"inlinee":14033,"source_line":812,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":17704,"parent_record_offset":17680,"ancestor_record_offset":17660,"inlinee":14034,"source_line":429,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\hash\mod.rs
//     evidence={"module":1651,"record_offset":17728,"parent_record_offset":17704,"ancestor_record_offset":17660,"inlinee":14297,"source_line":392,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1651,"record_offset":17752,"parent_record_offset":17728,"ancestor_record_offset":17660,"inlinee":14298,"source_line":3925,"archive":"_pdb/C/modules/01651.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: asset::texture::impl$0::from_str
// 记录形式: compiled_procedure; instances=1
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1676,"record_offset":1356,"rva":25139936,"va":5393849056}
// 编译实例 RVAs: 0x17f9ae0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-18170469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393849056,"raw_hex":"e89bbdeafe","size":5,"soft_fail":false,"target_va":5375678688,"va":5393849152,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-18170981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393849056,"raw_hex":"e89bbbeafe","size":5,"soft_fail":false,"target_va":5375678704,"va":5393849680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-25126947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393849056,"raw_hex":"e8dd9780fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5393849134,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: texture.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t23207877","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393849056,"raw_hex":"e8c51f6201","size":5,"soft_fail":false,"target_va":5417057056,"va":5393849174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t22771668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393849056,"raw_hex":"e8d4775b01","size":5,"soft_fail":false,"target_va":5416620794,"va":5393849121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::__private::format_err
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t22920223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5393849056,"raw_hex":"e81fbc5d01","size":5,"soft_fail":false,"target_va":5416769856,"va":5393849628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::str::impl$5::to_ascii_lowercase
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1676,"record_offset":1480,"parent_record_offset":1356,"ancestor_record_offset":1356,"inlinee":18731,"source_line":591,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1676,"record_offset":2712,"parent_record_offset":1356,"ancestor_record_offset":1356,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1676,"record_offset":2260,"parent_record_offset":1356,"ancestor_record_offset":1356,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1676,"record_offset":1884,"parent_record_offset":1860,"ancestor_record_offset":1356,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1676,"record_offset":1908,"parent_record_offset":1884,"ancestor_record_offset":1356,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1676,"record_offset":2888,"parent_record_offset":2864,"ancestor_record_offset":1356,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1676,"record_offset":2912,"parent_record_offset":2888,"ancestor_record_offset":1356,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1676,"record_offset":1932,"parent_record_offset":1908,"ancestor_record_offset":1356,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1676,"record_offset":2936,"parent_record_offset":2912,"ancestor_record_offset":1356,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1676,"record_offset":1860,"parent_record_offset":1712,"ancestor_record_offset":1356,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1676,"record_offset":2864,"parent_record_offset":2808,"ancestor_record_offset":1356,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1676,"record_offset":1656,"parent_record_offset":1632,"ancestor_record_offset":1356,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1676,"record_offset":2836,"parent_record_offset":2808,"ancestor_record_offset":1356,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1676,"record_offset":2808,"parent_record_offset":2784,"ancestor_record_offset":1356,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1676,"record_offset":1712,"parent_record_offset":1680,"ancestor_record_offset":1356,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1676,"record_offset":1680,"parent_record_offset":1656,"ancestor_record_offset":1356,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1676,"record_offset":2784,"parent_record_offset":2760,"ancestor_record_offset":1356,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1676,"record_offset":1748,"parent_record_offset":1712,"ancestor_record_offset":1356,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1676,"record_offset":1556,"parent_record_offset":1532,"ancestor_record_offset":1356,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1676,"record_offset":1580,"parent_record_offset":1556,"ancestor_record_offset":1356,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1676,"record_offset":1604,"parent_record_offset":1580,"ancestor_record_offset":1356,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1676,"record_offset":1532,"parent_record_offset":1508,"ancestor_record_offset":1356,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1676,"record_offset":1508,"parent_record_offset":1480,"ancestor_record_offset":1356,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1676,"record_offset":1632,"parent_record_offset":1604,"ancestor_record_offset":1356,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1676,"record_offset":1772,"parent_record_offset":1748,"ancestor_record_offset":1356,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1676,"record_offset":2284,"parent_record_offset":2260,"ancestor_record_offset":1356,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1676,"record_offset":1820,"parent_record_offset":1796,"ancestor_record_offset":1356,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1676,"record_offset":1796,"parent_record_offset":1772,"ancestor_record_offset":1356,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::is_ascii_uppercase
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":1676,"record_offset":2204,"parent_record_offset":2172,"ancestor_record_offset":1356,"inlinee":18736,"source_line":816,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::make_ascii_lowercase
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":1676,"record_offset":2140,"parent_record_offset":2088,"ancestor_record_offset":1356,"inlinee":18734,"source_line":746,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::to_ascii_lowercase
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\mod.rs
//     evidence={"module":1676,"record_offset":2172,"parent_record_offset":2140,"ancestor_record_offset":1356,"inlinee":18735,"source_line":668,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1676,"record_offset":1988,"parent_record_offset":1604,"ancestor_record_offset":1356,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1676,"record_offset":2012,"parent_record_offset":1988,"ancestor_record_offset":1356,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1676,"record_offset":2736,"parent_record_offset":2712,"ancestor_record_offset":1356,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::ascii::impl$0::make_ascii_lowercase
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\ascii.rs
//     evidence={"module":1676,"record_offset":2088,"parent_record_offset":2064,"ancestor_record_offset":1356,"inlinee":18733,"source_line":195,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1676,"record_offset":2308,"parent_record_offset":2284,"ancestor_record_offset":1356,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1676,"record_offset":2336,"parent_record_offset":2308,"ancestor_record_offset":1356,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01676.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::make_ascii_lowercase
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":1676,"record_offset":2064,"parent_record_offset":1480,"ancestor_record_offset":1356,"inlinee":18732,"source_line":2894,"archive":"_pdb/C/modules/01676.jsonl.gz"}
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
// 完整结构及原始类型记录: texture.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] asset::texture::SourceImageAsset
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 53
// [PDB 泛型关系，非字段关系] {"type": "rkyv::impls::core::impl$5::serialize_unsized<asset::texture::SourceImageAsset,rancor::Strategy<rkyv::ser::Serializer<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rkyv::ser::allocator::alloc::ArenaHandle,rkyv::ser::sharing::alloc::Share>,rancor::Error> >", "constructor": "rkyv::impls::core::impl$5::serialize_unsized", "arguments": ["asset::texture::SourceImageAsset", "rancor::Strategy<rkyv::ser::Serializer<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rkyv::ser::allocator::alloc::ArenaHandle,rkyv::ser::sharing::alloc::Share>,rancor::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$3::to_archive_bytes<asset::texture::SourceImageAsset>", "constructor": "turbosloth::asset::impl$3::to_archive_bytes", "arguments": ["asset::texture::SourceImageAsset"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["asset::texture::SourceImageAsset", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError>", "constructor": "core::result::Result", "arguments": ["alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>", "turbosloth::asset::AssetBuildError"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >", "constructor": "enum2$", "arguments": ["core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_channel::Send<enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >", "constructor": "async_channel::Send", "arguments": ["enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<async_channel::Send<enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["async_channel::Send<enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_channel::Sender<enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >", "constructor": "async_channel::Sender", "arguments": ["enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<async_channel::Sender<enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["async_channel::Sender<enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_channel::Receiver<enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >", "constructor": "async_channel::Receiver", "arguments": ["enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<async_channel::Receiver<enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["async_channel::Receiver<enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_channel::Channel<enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >", "constructor": "async_channel::Channel", "arguments": ["enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::ArcInner<async_channel::Channel<enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > > >", "constructor": "alloc::sync::ArcInner", "arguments": ["async_channel::Channel<enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::sync::ArcInner<async_channel::Channel<enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::sync::ArcInner<async_channel::Channel<enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<asset::texture::SourceImageAsset,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["asset::texture::SourceImageAsset", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<asset::texture::SourceImageAsset,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<asset::texture::SourceImageAsset,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<asset::texture::SourceImageAsset,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["asset::texture::SourceImageAsset", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<asset::texture::SourceImageAsset,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<asset::texture::SourceImageAsset,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_channel::impl$30::poll_with_strategy<enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >,event_listener_strategy::NonBlocking>", "constructor": "async_channel::impl$30::poll_with_strategy", "arguments": ["enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >", "event_listener_strategy::NonBlocking"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_channel::impl$29::poll_with_strategy<enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >,event_listener_strategy::NonBlocking>", "constructor": "async_channel::impl$29::poll_with_strategy", "arguments": ["enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >", "event_listener_strategy::NonBlocking"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::AssetRef<asset::texture::SourceImageAsset>", "constructor": "turbosloth::asset::AssetRef", "arguments": ["asset::texture::SourceImageAsset"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<turbosloth::asset::AssetRef<asset::texture::SourceImageAsset>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["turbosloth::asset::AssetRef<asset::texture::SourceImageAsset>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::context::impl$0::with_context<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,std::io::error::Error,alloc::string::String,asset_pipe::texture::impl$0::load::async_fn$0::closure_env$0>", "constructor": "anyhow::context::impl$0::with_context", "arguments": ["alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>", "std::io::error::Error", "alloc::string::String", "asset_pipe::texture::impl$0::load::async_fn$0::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<turbosloth::asset::AssetRef<asset::texture::SourceImageAsset>,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["turbosloth::asset::AssetRef<asset::texture::SourceImageAsset>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<turbosloth::asset::AssetRef<asset::texture::SourceImageAsset>,alloc::alloc::Global>,clutter_pack_pipe::impl$11::run::async_fn$0::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["alloc::vec::into_iter::IntoIter<turbosloth::asset::AssetRef<asset::texture::SourceImageAsset>,alloc::alloc::Global>", "clutter_pack_pipe::impl$11::run::async_fn$0::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<turbosloth::asset::AssetRef<asset::texture::SourceImageAsset>,alloc::alloc::Global>,clutter_pack_pipe::impl$11::run::async_fn$0::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,std::io::error::Error> > >", "constructor": "core::iter::adapters::GenericShunt", "arguments": ["core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<turbosloth::asset::AssetRef<asset::texture::SourceImageAsset>,alloc::alloc::Global>,clutter_pack_pipe::impl$11::run::async_fn$0::closure_env$0>", "enum2$<core::result::Result<enum2$<core::convert::Infallible>,std::io::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_collect::impl$1::from_iter<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<turbosloth::asset::AssetRef<asset::texture::SourceImageAsset>,alloc::alloc::Global>,clutter_pack_pipe::impl$11::run::async_fn$0::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,std::io::error::Error> > > >", "constructor": "alloc::vec::in_place_collect::impl$1::from_iter", "arguments": ["alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>", "core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<turbosloth::asset::AssetRef<asset::texture::SourceImageAsset>,alloc::alloc::Global>,clutter_pack_pipe::impl$11::run::async_fn$0::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,std::io::error::Error> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rkyv::api::high::to_bytes_in::closure_env$0<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rancor::Error,asset::texture::SourceImageAsset>", "constructor": "rkyv::api::high::to_bytes_in::closure_env$0", "arguments": ["rkyv::util::alloc::aligned_vec::AlignedVec<16>", "rancor::Error", "asset::texture::SourceImageAsset"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rkyv::util::alloc::arena::detail::with_arena::closure_env$0<enum2$<core::result::Result<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rancor::Error> >,rkyv::api::high::to_bytes_in::closure_env$0<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rancor::Error,asset::texture::SourceImageAsset> >", "constructor": "rkyv::util::alloc::arena::detail::with_arena::closure_env$0", "arguments": ["enum2$<core::result::Result<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rancor::Error> >", "rkyv::api::high::to_bytes_in::closure_env$0<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rancor::Error,asset::texture::SourceImageAsset>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::thread::local::LocalKey::with<core::cell::Cell<enum2$<core::option::Option<rkyv::ser::allocator::alloc::Arena> > >,rkyv::util::alloc::arena::detail::with_arena::closure_env$0<enum2$<core::result::Result<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rancor::Error> >,rkyv::api::high::to_bytes_in::closure_env$0<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rancor::Error,asset::texture::SourceImageAsset> >,enum2$<core::result::Result<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rancor::Error> > >", "constructor": "std::thread::local::LocalKey::with", "arguments": ["core::cell::Cell<enum2$<core::option::Option<rkyv::ser::allocator::alloc::Arena> > >", "rkyv::util::alloc::arena::detail::with_arena::closure_env$0<enum2$<core::result::Result<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rancor::Error> >,rkyv::api::high::to_bytes_in::closure_env$0<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rancor::Error,asset::texture::SourceImageAsset> >", "enum2$<core::result::Result<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rancor::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::BuildContext::store_asset<asset::texture::SourceImageAsset>", "constructor": "turbosloth::asset::BuildContext::store_asset", "arguments": ["asset::texture::SourceImageAsset"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::BuildContext::load<asset::texture::SourceImageAsset>", "constructor": "turbosloth::asset::BuildContext::load", "arguments": ["asset::texture::SourceImageAsset"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::AssetRequest::build<asset::texture::SourceImageAsset,turbosloth::asset::BuildContext>", "constructor": "turbosloth::asset::AssetRequest::build", "arguments": ["asset::texture::SourceImageAsset", "turbosloth::asset::BuildContext"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$44::build::async_block$1<asset::texture::SourceImageAsset,turbosloth::asset::BuildContext>", "constructor": "turbosloth::asset::impl$44::build::async_block$1", "arguments": ["asset::texture::SourceImageAsset", "turbosloth::asset::BuildContext"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$44::build::async_block$2<asset::texture::SourceImageAsset,turbosloth::asset::BuildContext>", "constructor": "turbosloth::asset::impl$44::build::async_block$2", "arguments": ["asset::texture::SourceImageAsset", "turbosloth::asset::BuildContext"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$44::build::async_block$3<asset::texture::SourceImageAsset,turbosloth::asset::BuildContext>", "constructor": "turbosloth::asset::impl$44::build::async_block$3", "arguments": ["asset::texture::SourceImageAsset", "turbosloth::asset::BuildContext"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::downcast_build_artifact<asset::texture::SourceImageAsset>", "constructor": "turbosloth::asset::downcast_build_artifact", "arguments": ["asset::texture::SourceImageAsset"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<asset::texture::SourceImageAsset>", "constructor": "core::any::impl$0::type_id", "arguments": ["asset::texture::SourceImageAsset"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$44::build::async_block_env$2<asset::texture::SourceImageAsset,turbosloth::asset::BuildContext>", "constructor": "turbosloth::asset::impl$44::build::async_block_env$2", "arguments": ["asset::texture::SourceImageAsset", "turbosloth::asset::BuildContext"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<turbosloth::asset::impl$44::build::async_block_env$2<asset::texture::SourceImageAsset,turbosloth::asset::BuildContext> >", "constructor": "enum2$", "arguments": ["turbosloth::asset::impl$44::build::async_block_env$2<asset::texture::SourceImageAsset,turbosloth::asset::BuildContext>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<turbosloth::asset::impl$44::build::async_block_env$2<asset::texture::SourceImageAsset,turbosloth::asset::BuildContext> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<turbosloth::asset::impl$44::build::async_block_env$2<asset::texture::SourceImageAsset,turbosloth::asset::BuildContext> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$44::build::async_block_env$3<asset::texture::SourceImageAsset,turbosloth::asset::BuildContext>", "constructor": "turbosloth::asset::impl$44::build::async_block_env$3", "arguments": ["asset::texture::SourceImageAsset", "turbosloth::asset::BuildContext"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<turbosloth::asset::impl$44::build::async_block_env$3<asset::texture::SourceImageAsset,turbosloth::asset::BuildContext> >", "constructor": "enum2$", "arguments": ["turbosloth::asset::impl$44::build::async_block_env$3<asset::texture::SourceImageAsset,turbosloth::asset::BuildContext>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<turbosloth::asset::impl$44::build::async_block_env$3<asset::texture::SourceImageAsset,turbosloth::asset::BuildContext> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<turbosloth::asset::impl$44::build::async_block_env$3<asset::texture::SourceImageAsset,turbosloth::asset::BuildContext> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::StoreAssetRecipe<asset::texture::SourceImageAsset>", "constructor": "turbosloth::asset::StoreAssetRecipe", "arguments": ["asset::texture::SourceImageAsset"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$25::debug_name<asset::texture::SourceImageAsset,enum2$<core::convert::Infallible>,turbosloth::asset::StoreAssetRecipe<asset::texture::SourceImageAsset> >", "constructor": "turbosloth::asset::impl$25::debug_name", "arguments": ["asset::texture::SourceImageAsset", "enum2$<core::convert::Infallible>", "turbosloth::asset::StoreAssetRecipe<asset::texture::SourceImageAsset>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$25::clone_boxed<asset::texture::SourceImageAsset,enum2$<core::convert::Infallible>,turbosloth::asset::StoreAssetRecipe<asset::texture::SourceImageAsset> >", "constructor": "turbosloth::asset::impl$25::clone_boxed", "arguments": ["asset::texture::SourceImageAsset", "enum2$<core::convert::Infallible>", "turbosloth::asset::StoreAssetRecipe<asset::texture::SourceImageAsset>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$25::run_boxed<asset::texture::SourceImageAsset,enum2$<core::convert::Infallible>,turbosloth::asset::StoreAssetRecipe<asset::texture::SourceImageAsset> >", "constructor": "turbosloth::asset::impl$25::run_boxed", "arguments": ["asset::texture::SourceImageAsset", "enum2$<core::convert::Infallible>", "turbosloth::asset::StoreAssetRecipe<asset::texture::SourceImageAsset>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$24::run::async_block$0<asset::texture::SourceImageAsset,enum2$<core::convert::Infallible>,turbosloth::asset::StoreAssetRecipe<asset::texture::SourceImageAsset> >", "constructor": "turbosloth::asset::impl$24::run::async_block$0", "arguments": ["asset::texture::SourceImageAsset", "enum2$<core::convert::Infallible>", "turbosloth::asset::StoreAssetRecipe<asset::texture::SourceImageAsset>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<async_channel::Channel<enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["async_channel::Channel<enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<asset::texture::SourceImageAsset,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["asset::texture::SourceImageAsset", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::AssetRefResolver<asset::texture::SourceImageAsset>", "constructor": "turbosloth::asset::AssetRefResolver", "arguments": ["asset::texture::SourceImageAsset"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rkyv::impls::core::impl$14::serialize_unsized::closure_env$0<turbosloth::asset::AssetRef<asset::texture::SourceImageAsset>,rancor::Strategy<rkyv::ser::Serializer<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rkyv::ser::allocator::alloc::ArenaHandle,rkyv::ser::sharing::alloc::Share>,rancor::Error> >", "constructor": "rkyv::impls::core::impl$14::serialize_unsized::closure_env$0", "arguments": ["turbosloth::asset::AssetRef<asset::texture::SourceImageAsset>", "rancor::Strategy<rkyv::ser::Serializer<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rkyv::ser::allocator::alloc::ArenaHandle,rkyv::ser::sharing::alloc::Share>,rancor::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rkyv::util::ser_vec::SerVec::with_capacity<turbosloth::asset::AssetRefResolver<asset::texture::SourceImageAsset>,rancor::Strategy<rkyv::ser::Serializer<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rkyv::ser::allocator::alloc::ArenaHandle,rkyv::ser::sharing::alloc::Share>,rancor::Error>,enum2$<core::result::Result<usize,rancor::Error> >,rkyv::impls::core::impl$14::serialize_unsized::closure_env$0<turbosloth::asset::AssetRef<asset::texture::SourceImageAsset>,rancor::Strategy<rkyv::ser::Serializer<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rkyv::ser::allocator::alloc::ArenaHandle,rkyv::ser::sharing::alloc::Share>,rancor::Error> > >", "constructor": "rkyv::util::ser_vec::SerVec::with_capacity", "arguments": ["turbosloth::asset::AssetRefResolver<asset::texture::SourceImageAsset>", "rancor::Strategy<rkyv::ser::Serializer<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rkyv::ser::allocator::alloc::ArenaHandle,rkyv::ser::sharing::alloc::Share>,rancor::Error>", "enum2$<core::result::Result<usize,rancor::Error> >", "rkyv::impls::core::impl$14::serialize_unsized::closure_env$0<turbosloth::asset::AssetRef<asset::texture::SourceImageAsset>,rancor::Strategy<rkyv::ser::Serializer<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rkyv::ser::allocator::alloc::ArenaHandle,rkyv::ser::sharing::alloc::Share>,rancor::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rkyv::api::high::to_bytes_in_with_alloc<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rkyv::ser::allocator::alloc::ArenaHandle,rancor::Error,asset::texture::SourceImageAsset>", "constructor": "rkyv::api::high::to_bytes_in_with_alloc", "arguments": ["rkyv::util::alloc::aligned_vec::AlignedVec<16>", "rkyv::ser::allocator::alloc::ArenaHandle", "rancor::Error", "asset::texture::SourceImageAsset"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rkyv::api::high::checked::from_bytes<asset::texture::SourceImageAsset,rancor::Error>", "constructor": "rkyv::api::high::checked::from_bytes", "arguments": ["asset::texture::SourceImageAsset", "rancor::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::ArchivedAssetRef<asset::texture::SourceImageAsset>", "constructor": "turbosloth::asset::ArchivedAssetRef", "arguments": ["asset::texture::SourceImageAsset"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "slice2$<turbosloth::asset::ArchivedAssetRef<asset::texture::SourceImageAsset> >", "constructor": "slice2$", "arguments": ["turbosloth::asset::ArchivedAssetRef<asset::texture::SourceImageAsset>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rkyv::vec::verify::impl$0::verify::closure_env$0<turbosloth::asset::ArchivedAssetRef<asset::texture::SourceImageAsset>,rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error> >", "constructor": "rkyv::vec::verify::impl$0::verify::closure_env$0", "arguments": ["turbosloth::asset::ArchivedAssetRef<asset::texture::SourceImageAsset>", "rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rkyv::validation::archive::impl$1::in_subtree<rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error>,rancor::Error,slice2$<turbosloth::asset::ArchivedAssetRef<asset::texture::SourceImageAsset> >,tuple$<>,rkyv::vec::verify::impl$0::verify::closure_env$0<turbosloth::asset::ArchivedAssetRef<asset::texture::SourceImageAsset>,rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error> > >", "constructor": "rkyv::validation::archive::impl$1::in_subtree", "arguments": ["rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error>", "rancor::Error", "slice2$<turbosloth::asset::ArchivedAssetRef<asset::texture::SourceImageAsset> >", "tuple$<>", "rkyv::vec::verify::impl$0::verify::closure_env$0<turbosloth::asset::ArchivedAssetRef<asset::texture::SourceImageAsset>,rancor::Strategy<rkyv::validation::Validator<rkyv::validation::archive::validator::ArchiveValidator,rkyv::validation::shared::validator::SharedValidator>,rancor::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "concurrent_queue::bounded::impl$0::push::closure_env$0<enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >", "constructor": "concurrent_queue::bounded::impl$0::push::closure_env$0", "arguments": ["enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "concurrent_queue::bounded::Bounded::push_or_else<enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >,concurrent_queue::bounded::impl$0::push::closure_env$0<enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > > >", "constructor": "concurrent_queue::bounded::Bounded::push_or_else", "arguments": ["enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >", "concurrent_queue::bounded::impl$0::push::closure_env$0<enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$44::build::closure_env$0<asset::texture::SourceImageAsset,turbosloth::asset::BuildContext>", "constructor": "turbosloth::asset::impl$44::build::closure_env$0", "arguments": ["asset::texture::SourceImageAsset", "turbosloth::asset::BuildContext"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::store::AssetStore::get_or_insert_with<turbosloth::asset::impl$44::build::closure_env$0<asset::texture::SourceImageAsset,turbosloth::asset::BuildContext> >", "constructor": "turbosloth::store::AssetStore::get_or_insert_with", "arguments": ["turbosloth::asset::impl$44::build::closure_env$0<asset::texture::SourceImageAsset,turbosloth::asset::BuildContext>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::store::AssetStore::load<asset::texture::SourceImageAsset>", "constructor": "turbosloth::store::AssetStore::load", "arguments": ["asset::texture::SourceImageAsset"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::store::DiskStore::store<asset::texture::SourceImageAsset>", "constructor": "turbosloth::store::DiskStore::store", "arguments": ["asset::texture::SourceImageAsset"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::store::DiskStore::try_load<asset::texture::SourceImageAsset>", "constructor": "turbosloth::store::DiskStore::try_load", "arguments": ["asset::texture::SourceImageAsset"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "concurrent_queue::unbounded::Unbounded::push<enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >", "constructor": "concurrent_queue::unbounded::Unbounded::push", "arguments": ["enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<async_channel::Channel<enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["async_channel::Channel<enum2$<core::result::Result<alloc::sync::Arc<asset::texture::SourceImageAsset,alloc::alloc::Global>,turbosloth::asset::AssetBuildError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$24::run::async_block_env$0<asset::texture::SourceImageAsset,anyhow::Error,clutter_pack_pipe::ExtractRoughnessChannel>", "constructor": "turbosloth::asset::impl$24::run::async_block_env$0", "arguments": ["asset::texture::SourceImageAsset", "anyhow::Error", "clutter_pack_pipe::ExtractRoughnessChannel"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<turbosloth::asset::impl$24::run::async_block_env$0<asset::texture::SourceImageAsset,anyhow::Error,clutter_pack_pipe::ExtractRoughnessChannel> >", "constructor": "enum2$", "arguments": ["turbosloth::asset::impl$24::run::async_block_env$0<asset::texture::SourceImageAsset,anyhow::Error,clutter_pack_pipe::ExtractRoughnessChannel>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<turbosloth::asset::impl$24::run::async_block_env$0<asset::texture::SourceImageAsset,anyhow::Error,clutter_pack_pipe::ExtractRoughnessChannel> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<turbosloth::asset::impl$24::run::async_block_env$0<asset::texture::SourceImageAsset,anyhow::Error,clutter_pack_pipe::ExtractRoughnessChannel> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$25::debug_name<asset::texture::SourceImageAsset,anyhow::Error,clutter_pack_pipe::ExtractRoughnessChannel>", "constructor": "turbosloth::asset::impl$25::debug_name", "arguments": ["asset::texture::SourceImageAsset", "anyhow::Error", "clutter_pack_pipe::ExtractRoughnessChannel"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$25::clone_boxed<asset::texture::SourceImageAsset,anyhow::Error,clutter_pack_pipe::ExtractRoughnessChannel>", "constructor": "turbosloth::asset::impl$25::clone_boxed", "arguments": ["asset::texture::SourceImageAsset", "anyhow::Error", "clutter_pack_pipe::ExtractRoughnessChannel"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$25::version<asset::texture::SourceImageAsset,anyhow::Error,clutter_pack_pipe::ExtractRoughnessChannel>", "constructor": "turbosloth::asset::impl$25::version", "arguments": ["asset::texture::SourceImageAsset", "anyhow::Error", "clutter_pack_pipe::ExtractRoughnessChannel"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$25::run_boxed<asset::texture::SourceImageAsset,anyhow::Error,clutter_pack_pipe::ExtractRoughnessChannel>", "constructor": "turbosloth::asset::impl$25::run_boxed", "arguments": ["asset::texture::SourceImageAsset", "anyhow::Error", "clutter_pack_pipe::ExtractRoughnessChannel"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "turbosloth::asset::impl$24::run::async_block$0<asset::texture::SourceImageAsset,anyhow::Error,clutter_pack_pipe::ExtractRoughnessChannel>", "constructor": "turbosloth::asset::impl$24::run::async_block$0", "arguments": ["asset::texture::SourceImageAsset", "anyhow::Error", "clutter_pack_pipe::ExtractRoughnessChannel"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: texture.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: texture.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 22; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustClosed>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustClosed>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::gui::EscapeMenuJustClosed>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>,bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::FileDroppedOnWindow>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::UninstallModClutterCmd>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_publish::PublishModCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>", "bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::FileDroppedOnWindow>", "bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::UninstallModClutterCmd>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_publish::PublishModCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::mod_editor::ModEditor>
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::resources::window::FileDroppedOnWindow>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::InstallModClutterCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::startup::install_mod_clutter::UninstallModClutterCmd>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::steam::workshop_publish::PublishModCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::DeleteSessionCmd>)
// 候选参数类型: ["bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::DeleteSessionCmd>"]
// [候选系统接口] message_reader: bevy_ecs::message::message_reader::MessageReader<country_core::systems::save::save_game::DeleteSessionCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState> > >)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState> > >"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::session::SavedSessionList>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::save::save_game::AutoSaveState>
// [文件级接口候选，具体函数归属 unknown] void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >,bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>,bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource> > >", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>", "bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::gui::EguiContextResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::music::music_system::MusicAudioState>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::gui::DebugGui>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::terrain::animate_terrain_delete::TerrainDeleteAnimation>,bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>,bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_terrain_focus_mode::EditTerrainStrokeCache>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::highlight_active_terrain_stroke::AddTerrainHighlightCmd> >,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::systems::terrain::animate_terrain_delete::TerrainDeleteAnimation>", "bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>", "bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>", "bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_terrain_focus_mode::EditTerrainStrokeCache>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>", "bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>,bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::highlight_active_terrain_stroke::AddTerrainHighlightCmd> >", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>", "tuple$<bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::Res<country_core::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::terrain::animate_terrain_delete::TerrainDeleteAnimation>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_editing::TerrainEditingData>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::terrain::ui_terrain_focus_mode::EditTerrainStrokeCache>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::terrain::deferred_terrain_history::PendingTerrainHistory>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::SetTerrainStrokePtsCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::glade_border::GladeBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::DisplayOutOfBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::glade_border::FeedbackInputOutOfBorder>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::render_loop::compute_passes::highlight_active_terrain_stroke::AddTerrainHighlightCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::systems::terrain::edit_terrain_stroke::EditTerrainStrokeCmd>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::collision::world::RaycastWorld>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::terrain::terrain_hover_idx::HoveredTerrainStrokeId>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::render::RenderParams>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::render::RenderParams>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::RenderParams>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::system::system_param::Local<country_core::systems::ui::hover_intersection::HoverIntersectionCache>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::system::system_param::Local<country_core::systems::ui::hover_intersection::HoverIntersectionCache>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::read_mouse_depth_value::MouseDepthValue>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::ui::hover_intersection::HoverIntersectionCache>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: asset::texture::_::impl$0::resolve
// 信息 ID: aa5c397d6e441b01903fa658
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] asset::texture::impl$3::resolve; module=1676, inline_record=620; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::_::impl$1::check_bytes
// 信息 ID: 3570406bd28778db32b3d956
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rkyv::api::high::checked::from_bytes<asset::texture::SourceImageAsset,rancor::Error>; module=1666, inline_record=109660; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::impl$0::from_str
// 信息 ID: 8204c9158ab99b1db8493448
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] asset_pipe::texture::source_image_format @ 0xd97146 -> 0x17f9ae0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::impl$10::hash
// 信息 ID: 8c2204ce097d70d157490bb4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::asset::BuildContext::store_asset<asset::texture::SourceImageAsset>; module=1651, inline_record=17660; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::impl$1::clone
// 信息 ID: ad287d5db2ede3c20b411aea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::asset::impl$25::clone_boxed<asset::texture::SourceImageAsset,enum2$<core::convert::Infallible>,turbosloth::asset::StoreAssetRecipe<asset::texture::SourceImageAsset> >; module=1651, inline_record=350920; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::impl$2::hash
// 信息 ID: b2d6b53ea52a992cfaf26e05
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] turbosloth::asset::BuildContext::store_asset<asset::texture::SourceImageAsset>; module=1651, inline_record=17632; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::impl$3::resolve
// 信息 ID: cebe62bd9e3dc5838d022207
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rkyv::impls::core::impl$5::serialize_unsized<asset::texture::SourceImageAsset,rancor::Strategy<rkyv::ser::Serializer<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rkyv::ser::allocator::alloc::ArenaHandle,rkyv::ser::sharing::alloc::Share>,rancor::Error> > @ 0x9012f4 -> 0x17f9aa0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::impl$5::check_bytes
// 信息 ID: 386c6c192b65add409f06bb7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rkyv::api::high::checked::from_bytes<asset::texture::SourceImageAsset,rancor::Error>; module=1666, inline_record=109636; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::impl$6::serialize
// 信息 ID: 3ea60191d12ea3516f982d6b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rkyv::impls::core::impl$5::serialize_unsized<asset::texture::SourceImageAsset,rancor::Strategy<rkyv::ser::Serializer<rkyv::util::alloc::aligned_vec::AlignedVec<16>,rkyv::ser::allocator::alloc::ArenaHandle,rkyv::ser::sharing::alloc::Share>,rancor::Error> >; module=502, inline_record=21892; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::impl$7::deserialize
// 信息 ID: 506c606f1118648372598f99
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rkyv::api::high::checked::from_bytes<asset::texture::SourceImageAsset,rancor::Error>; module=1666, inline_record=110808; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: asset::texture::impl$8::visit_asset_dependencies
// 信息 ID: 621c92c2e13f36a1cec8c992
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] turbosloth::asset::impl$44::build::async_block$1<asset::texture::SourceImageAsset,turbosloth::asset::BuildContext> @ 0x172b1ea -> 0x17f9dd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
