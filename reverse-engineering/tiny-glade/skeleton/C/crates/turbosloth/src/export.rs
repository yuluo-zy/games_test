// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/turbosloth/src/export.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: a20fdddba507960de02c3001fe9551e173c35f32052fb4bca5d4b822d00d11cf
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: export.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 2150, 3627, 4407
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use asset; // namespace candidate
//   package identities: [{"name":"asset","source":"local_or_path","version":"0.1.0"}]
// use core; // namespace candidate
//   package identities: []
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
// use turbosloth; // namespace candidate
//   package identities: [{"name":"turbosloth","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: turbosloth::export::to_json_value<asset::foliage::Grass2RuntimeConfigAsset>
// 记录形式: compiled_procedure; instances=1
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2150,"record_offset":912,"rva":31413184,"va":5400122304}
// 编译实例 RVAs: 0x1df53c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$::end
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16894953","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400122304,"raw_hex":"e81734fefe","size":5,"soft_fail":false,"target_va":5383227792,"va":5400122740,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-24443893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400122304,"raw_hex":"e80b048bfe","size":5,"soft_fail":false,"target_va":5375678688,"va":5400122576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-24444204","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400122304,"raw_hex":"e8d4028bfe","size":5,"soft_fail":false,"target_va":5375678704,"va":5400122903,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31400374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400122304,"raw_hex":"e84ade20fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5400122561,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009ae10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30779565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400122304,"raw_hex":"e853572afe","size":5,"soft_fail":false,"target_va":5369343504,"va":5400123064,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098f690
//     owner=unresolved_namespace; count=4
//     target_aliases=3 candidates; complete list: export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21388983","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400122304,"raw_hex":"e849a1b9fe","size":5,"soft_fail":false,"target_va":5378733712,"va":5400122690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409942b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21369821","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400122304,"raw_hex":"e823ecb9fe","size":5,"soft_fail":false,"target_va":5378753200,"va":5400123016,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14271c990
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9598164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400122304,"raw_hex":"e8d4749200","size":5,"soft_fail":false,"target_va":5409720720,"va":5400122551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1427286d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9646822","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400122304,"raw_hex":"e8e6329300","size":5,"soft_fail":false,"target_va":5409769168,"va":5400122341,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272ca80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9664004","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400122304,"raw_hex":"e804769300","size":5,"soft_fail":false,"target_va":5409786496,"va":5400122487,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272db30
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9668234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400122304,"raw_hex":"e88a869300","size":5,"soft_fail":false,"target_va":5409790768,"va":5400122529,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16497831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400122304,"raw_hex":"e8a7bcfb00","size":5,"soft_fail":false,"target_va":5416620769,"va":5400122933,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2150,"record_offset":1564,"parent_record_offset":912,"ancestor_record_offset":912,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2150,"record_offset":1952,"parent_record_offset":912,"ancestor_record_offset":912,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2150,"record_offset":1152,"parent_record_offset":912,"ancestor_record_offset":912,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::to_value
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":2150,"record_offset":1180,"parent_record_offset":912,"ancestor_record_offset":912,"inlinee":8744,"source_line":995,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::asset::impl$3::from_archive_bytes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":2150,"record_offset":1088,"parent_record_offset":912,"ancestor_record_offset":912,"inlinee":28445,"source_line":92,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2150,"record_offset":1456,"parent_record_offset":1432,"ancestor_record_offset":912,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2150,"record_offset":1480,"parent_record_offset":1456,"ancestor_record_offset":912,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2150,"record_offset":1844,"parent_record_offset":1820,"ancestor_record_offset":912,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2150,"record_offset":1868,"parent_record_offset":1844,"ancestor_record_offset":912,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2150,"record_offset":1504,"parent_record_offset":1480,"ancestor_record_offset":912,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2150,"record_offset":1892,"parent_record_offset":1868,"ancestor_record_offset":912,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2150,"record_offset":1432,"parent_record_offset":1400,"ancestor_record_offset":912,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2150,"record_offset":1820,"parent_record_offset":1732,"ancestor_record_offset":912,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2150,"record_offset":1400,"parent_record_offset":1360,"ancestor_record_offset":912,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::convert::impl$16::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed\convert.rs
//     evidence={"module":2150,"record_offset":1328,"parent_record_offset":1296,"ancestor_record_offset":912,"inlinee":6824,"source_line":556,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2150,"record_offset":1360,"parent_record_offset":1328,"ancestor_record_offset":912,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2150,"record_offset":1760,"parent_record_offset":1732,"ancestor_record_offset":912,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2150,"record_offset":1732,"parent_record_offset":1708,"ancestor_record_offset":912,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":2150,"record_offset":1708,"parent_record_offset":1684,"ancestor_record_offset":912,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":2150,"record_offset":1620,"parent_record_offset":1592,"ancestor_record_offset":912,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] asset::foliage::_::impl$0::serialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/foliage.rs
//     evidence={"module":2150,"record_offset":1252,"parent_record_offset":1216,"ancestor_record_offset":912,"inlinee":98374,"source_line":26,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":2150,"record_offset":1788,"parent_record_offset":1760,"ancestor_record_offset":912,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":2150,"record_offset":1592,"parent_record_offset":1564,"ancestor_record_offset":912,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2150,"record_offset":1116,"parent_record_offset":1088,"ancestor_record_offset":912,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$109::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":2150,"record_offset":1216,"parent_record_offset":1180,"ancestor_record_offset":912,"inlinee":8449,"source_line":468,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::export::to_json_value<asset::mesh::MeshData>
// 记录形式: compiled_procedure; instances=1
// 源码行: 48
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":2150,"record_offset":260,"rva":31412624,"va":5400121744}
// 编译实例 RVAs: 0x1df5190
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$serde_json..value..ser..SerializeMap$u20$as$u20$serde_core..ser..SerializeStruct$GT$::end
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-16894292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400121744,"raw_hex":"e8ac36fefe","size":5,"soft_fail":false,"target_va":5383227792,"va":5400122079,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-24443303","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400121744,"raw_hex":"e859068bfe","size":5,"soft_fail":false,"target_va":5375678688,"va":5400121986,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-31399784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400121744,"raw_hex":"e898e020fe","size":5,"soft_fail":false,"target_va":5368722192,"va":5400121971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009ae10
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-30778780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400121744,"raw_hex":"e8645a2afe","size":5,"soft_fail":false,"target_va":5369343504,"va":5400122279,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098cd80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-21398830","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400121744,"raw_hex":"e8d27ab9fe","size":5,"soft_fail":false,"target_va":5378723200,"va":5400122025,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141df3790
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400121744,"raw_hex":"e822e5ffff","size":5,"soft_fail":false,"target_va":5400115088,"va":5400121961,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142728450
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9646746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400121744,"raw_hex":"e89a329300","size":5,"soft_fail":false,"target_va":5409768528,"va":5400121777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272d410
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9667035","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400121744,"raw_hex":"e8db819300","size":5,"soft_fail":false,"target_va":5409788944,"va":5400121904,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14272d670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: export.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9667604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400121744,"raw_hex":"e814849300","size":5,"soft_fail":false,"target_va":5409789552,"va":5400121943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t16498620","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5400121744,"raw_hex":"e8bcbffb00","size":5,"soft_fail":false,"target_va":5416620769,"va":5400122144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2150,"record_offset":876,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2150,"record_offset":476,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::to_value
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":2150,"record_offset":504,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":8744,"source_line":995,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::asset::impl$3::from_archive_bytes
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/asset.rs
//     evidence={"module":2150,"record_offset":412,"parent_record_offset":260,"ancestor_record_offset":260,"inlinee":28445,"source_line":92,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2150,"record_offset":768,"parent_record_offset":744,"ancestor_record_offset":260,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2150,"record_offset":792,"parent_record_offset":768,"ancestor_record_offset":260,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2150,"record_offset":816,"parent_record_offset":792,"ancestor_record_offset":260,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":2150,"record_offset":744,"parent_record_offset":712,"ancestor_record_offset":260,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::box_new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2150,"record_offset":712,"parent_record_offset":672,"ancestor_record_offset":260,"inlinee":4234,"source_line":247,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::convert::impl$16::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed\convert.rs
//     evidence={"module":2150,"record_offset":640,"parent_record_offset":608,"ancestor_record_offset":260,"inlinee":6824,"source_line":556,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":2150,"record_offset":672,"parent_record_offset":640,"ancestor_record_offset":260,"inlinee":4232,"source_line":284,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] asset::mesh::_::impl$0::serialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"asset","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/asset/src/mesh.rs
//     evidence={"module":2150,"record_offset":568,"parent_record_offset":536,"ancestor_record_offset":260,"inlinee":98367,"source_line":10,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":2150,"record_offset":440,"parent_record_offset":412,"ancestor_record_offset":260,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::ser::impls::impl$109::serialize
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/ser/impls.rs
//     evidence={"module":2150,"record_offset":536,"parent_record_offset":504,"ancestor_record_offset":260,"inlinee":8449,"source_line":468,"archive":"_pdb/C/modules/02150.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::export::exports
// 记录形式: inlinee_declaration; instances=1
// 源码行: 55
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FAA2
// 首条位置: {"module":3627,"inlinee":129698}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::export::exports::closure$0
// 记录形式: inlinee_declaration; instances=1
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2200F
// 首条位置: {"module":4407,"inlinee":139279}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::export::parse_path
// 记录形式: inlinee_declaration; instances=1
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA94
// 首条位置: {"module":3627,"inlinee":129684}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::export::parse_path::closure$0
// 记录形式: inlinee_declaration; instances=1
// 源码行: 96
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA96
// 首条位置: {"module":3627,"inlinee":129686}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::export::parse_path::closure$1
// 记录形式: inlinee_declaration; instances=1
// 源码行: 100
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA97
// 首条位置: {"module":3627,"inlinee":129687}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::export::as_asset_ref
// 记录形式: inlinee_declaration; instances=1
// 源码行: 112
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA9A
// 首条位置: {"module":3627,"inlinee":129690}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::export::value_kind
// 记录形式: inlinee_declaration; instances=1
// 源码行: 120
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA9F
// 首条位置: {"module":3627,"inlinee":129695}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::export::index_value
// 记录形式: inlinee_declaration; instances=1
// 源码行: 131
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA9B
// 首条位置: {"module":3627,"inlinee":129691}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::export::index_value::closure$0
// 记录形式: inlinee_declaration; instances=1
// 源码行: 136
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA9E
// 首条位置: {"module":3627,"inlinee":129694}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::export::jsonify::closure$0
// 记录形式: inlinee_declaration; instances=1
// 源码行: 150
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FAA1
// 首条位置: {"module":3627,"inlinee":129697}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::export::jsonify::closure$1
// 记录形式: inlinee_declaration; instances=1
// 源码行: 156
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FAA3
// 首条位置: {"module":3627,"inlinee":129699}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::export::jsonify::closure$2
// 记录形式: inlinee_declaration; instances=1
// 源码行: 164
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FAA4
// 首条位置: {"module":3627,"inlinee":129700}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::export::export_asset::closure$0
// 记录形式: inlinee_declaration; instances=1
// 源码行: 186
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FA99
// 首条位置: {"module":3627,"inlinee":129689}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: turbosloth::export::export_asset::closure$1
// 记录形式: inlinee_declaration; instances=1
// 源码行: 207
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1FAA5
// 首条位置: {"module":3627,"inlinee":129701}
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
// 完整结构及原始类型记录: export.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] turbosloth::export::AssetExport
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "ref$<turbosloth::export::AssetExport>", "constructor": "ref$", "arguments": ["turbosloth::export::AssetExport"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<ref$<str$>,ref$<turbosloth::export::AssetExport>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["ref$<str$>", "ref$<turbosloth::export::AssetExport>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::map::HashMap<ref$<str$>,ref$<turbosloth::export::AssetExport>,std::hash::random::RandomState,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::map::HashMap<ref$<str$>,ref$<turbosloth::export::AssetExport>,std::hash::random::RandomState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<str$>,ref$<turbosloth::export::AssetExport> >", "constructor": "tuple$", "arguments": ["ref$<str$>", "ref$<turbosloth::export::AssetExport>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<ref$<str$>,ref$<turbosloth::export::AssetExport>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["ref$<str$>", "ref$<turbosloth::export::AssetExport>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<ref$<str$>,ref$<turbosloth::export::AssetExport> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<ref$<str$>,ref$<turbosloth::export::AssetExport>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<ref$<str$>,ref$<turbosloth::export::AssetExport> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<ref$<str$>,ref$<turbosloth::export::AssetExport>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "inventory::impl$1::submit<turbosloth::export::AssetExport>", "constructor": "inventory::impl$1::submit", "arguments": ["turbosloth::export::AssetExport"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: export.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: export.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: turbosloth::export::as_asset_ref
// 信息 ID: 2649529b2e4403f07ee25e0f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::export::export_asset::closure$0
// 信息 ID: 8e503a28fbf1b428965a7544
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "turbosloth::export::export_asset", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::export::export_asset::closure$1
// 信息 ID: 6e5c985bfb97af2d9cca802d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "turbosloth::export::export_asset", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::export::exports
// 信息 ID: 7ae3c62a9ebb1b2b416e9ed2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::export::exports::closure$0
// 信息 ID: 44efa5b69dfd7d3391e8312b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "turbosloth::export::exports", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::export::index_value
// 信息 ID: d74cd1014ce2f4db603cb307
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::export::index_value::closure$0
// 信息 ID: 994f4476b3883c2bb49b0f26
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "turbosloth::export::index_value", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::export::jsonify::closure$0
// 信息 ID: 356cddb43f579166bbc780a6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "turbosloth::export::jsonify", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::export::jsonify::closure$1
// 信息 ID: 38ea1ccef0279537aef3d353
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "turbosloth::export::jsonify", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::export::jsonify::closure$2
// 信息 ID: d3869f33f04068bdd85d84cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "turbosloth::export::jsonify", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::export::parse_path
// 信息 ID: ec644fafe413101bb06c394b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::export::parse_path::closure$0
// 信息 ID: 65c82f068c3d626f164f3a35
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "turbosloth::export::parse_path", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::export::parse_path::closure$1
// 信息 ID: edb2a7b48e9dbf45a79f0a8b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "turbosloth::export::parse_path", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::export::to_json_value<asset::foliage::Grass2RuntimeConfigAsset>
// 信息 ID: 35d9653aba27cae01554a672
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] turbosloth::export::to_json_value -> ["asset::foliage::Grass2RuntimeConfigAsset"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::export::to_json_value<asset::mesh::MeshData>
// 信息 ID: 473267127f41ddc945855677
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] turbosloth::export::to_json_value -> ["asset::mesh::MeshData"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: turbosloth::export::value_kind
// 信息 ID: 7a6f97613748b49521a6252b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
