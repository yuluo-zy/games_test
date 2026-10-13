// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/resources/gpu_lut.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 5c1f659a8b8852b0389ed9d0b6d4b14152292e291646db5f296d4d37b40c1787
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: gpu_lut.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 550, 560
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.14"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.5"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.12"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// use rhapsody_core; // namespace candidate
//   package identities: [{"name":"rhapsody-core","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::resources::gpu_lut::impl$0::build
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":550,"record_offset":61896,"rva":11298880,"va":5380008000}
// 编译实例 RVAs: 0xac6840
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140996c40
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: gpu_lut.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1244476","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380008000,"raw_hex":"e8c402edff","size":5,"soft_fail":false,"target_va":5378763840,"va":5380008311,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140998280
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: gpu_lut.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1238885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380008000,"raw_hex":"e89b18edff","size":5,"soft_fail":false,"target_va":5378769536,"va":5380008416,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418680d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gpu_lut.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14292860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380008000,"raw_hex":"e87c17da00","size":5,"soft_fail":false,"target_va":5394301136,"va":5380008271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f320
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: gpu_lut.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36342399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380008000,"raw_hex":"e87f8a2a02","size":5,"soft_fail":false,"target_va":5416350496,"va":5380008092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":550,"record_offset":62036,"parent_record_offset":61896,"ancestor_record_offset":61896,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":550,"record_offset":62092,"parent_record_offset":61896,"ancestor_record_offset":61896,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::gpu_lut::GpuLutDesc::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gpu_lut.rs
//     evidence={"module":550,"record_offset":62064,"parent_record_offset":61896,"ancestor_record_offset":61896,"inlinee":34282,"source_line":76,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":550,"record_offset":62184,"parent_record_offset":62160,"ancestor_record_offset":61896,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":550,"record_offset":62232,"parent_record_offset":62208,"ancestor_record_offset":61896,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":550,"record_offset":62208,"parent_record_offset":62184,"ancestor_record_offset":61896,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":550,"record_offset":62160,"parent_record_offset":62120,"ancestor_record_offset":61896,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":550,"record_offset":62120,"parent_record_offset":62092,"ancestor_record_offset":61896,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":550,"record_offset":62272,"parent_record_offset":62120,"ancestor_record_offset":61896,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::gpu_lut::impl$0::build::closure$0
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8964
// 首条位置: {"module":560,"inlinee":35172}
// 编译实例 RVAs: 0xb0c960
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140395170
//     owner=unresolved_namespace; count=1
//     target_aliases=49 candidates; complete list: gpu_lut.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7829808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380295008,"raw_hex":"e8d08688ff","size":5,"soft_fail":false,"target_va":5372465520,"va":5380295323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141770e40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gpu_lut.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12993628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380295008,"raw_hex":"e85c44c600","size":5,"soft_fail":false,"target_va":5393288768,"va":5380295135,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14177f390
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gpu_lut.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13052284","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380295008,"raw_hex":"e87c29c700","size":5,"soft_fail":false,"target_va":5393347472,"va":5380295183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: gpu_lut.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36761845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380295008,"raw_hex":"e8f5f03002","size":5,"soft_fail":false,"target_va":5417057056,"va":5380295206,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_rg_preset_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1015778","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380295008,"raw_hex":"e81e80f0ff","size":5,"soft_fail":false,"target_va":5379279280,"va":5380295053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t36279656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380295008,"raw_hex":"e868952902","size":5,"soft_fail":false,"target_va":5416574944,"va":5380295283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3690120","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380295008,"raw_hex":"e8884e3800","size":5,"soft_fail":false,"target_va":5383985360,"va":5380295235,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3243981","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380295008,"raw_hex":"e8cd7f3100","size":5,"soft_fail":false,"target_va":5383539152,"va":5380295166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*40(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5380295008,"raw_hex":"ff5028","size":3,"soft_fail":false,"va":5380295221,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":560,"record_offset":16624,"parent_record_offset":16456,"ancestor_record_offset":16456,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::gpu_lut::GpuLutApi::compute
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gpu_lut.rs
//     evidence={"module":560,"record_offset":16496,"parent_record_offset":16456,"ancestor_record_offset":16456,"inlinee":35169,"source_line":57,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$33::call
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":560,"record_offset":16592,"parent_record_offset":16496,"ancestor_record_offset":16456,"inlinee":8038,"source_line":2253,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":560,"record_offset":16804,"parent_record_offset":16680,"ancestor_record_offset":16456,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":560,"record_offset":16828,"parent_record_offset":16804,"ancestor_record_offset":16456,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$55::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":560,"record_offset":16652,"parent_record_offset":16624,"ancestor_record_offset":16456,"inlinee":6102,"source_line":3102,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":560,"record_offset":16716,"parent_record_offset":16680,"ancestor_record_offset":16456,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":560,"record_offset":16680,"parent_record_offset":16652,"ancestor_record_offset":16456,"inlinee":6103,"source_line":94,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":560,"record_offset":16744,"parent_record_offset":16716,"ancestor_record_offset":16456,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":560,"record_offset":16524,"parent_record_offset":16496,"ancestor_record_offset":16456,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody_core::shader::compute_brdf_lut::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/rhapsody-core-d39e02cd2b2655fe/out/shabriri.rs
//     evidence={"module":560,"record_offset":16560,"parent_record_offset":16524,"ancestor_record_offset":16456,"inlinee":35174,"source_line":687,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$1,tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeNode> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":560,"record_offset":14228,"rva":11578560,"va":5380287680}
// 编译实例 RVAs: 0xb0acc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: gpu_lut.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36769349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380287680,"raw_hex":"e8450e3102","size":5,"soft_fail":false,"target_va":5417057056,"va":5380287702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3248921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380287680,"raw_hex":"e819933100","size":5,"soft_fail":false,"target_va":5383536672,"va":5380287746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":560,"record_offset":14488,"parent_record_offset":14228,"ancestor_record_offset":14228,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::gpu_lut::impl$0::build::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gpu_lut.rs
//     evidence={"module":560,"record_offset":14508,"parent_record_offset":14488,"ancestor_record_offset":14228,"inlinee":35170,"source_line":28,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":560,"record_offset":14528,"parent_record_offset":14508,"ancestor_record_offset":14228,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::gpu_lut::impl$0::build::closure$1
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 28
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8962
// 首条位置: {"module":560,"inlinee":35170}
// 编译实例 RVAs: 0xb0acc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: gpu_lut.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36769349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380287680,"raw_hex":"e8450e3102","size":5,"soft_fail":false,"target_va":5417057056,"va":5380287702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3248921","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380287680,"raw_hex":"e819933100","size":5,"soft_fail":false,"target_va":5383536672,"va":5380287746,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":560,"record_offset":14528,"parent_record_offset":14508,"ancestor_record_offset":14508,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::gpu_lut::impl$0::build::closure$2
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 34
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x895F
// 首条位置: {"module":560,"inlinee":35167}
// 编译实例 RVAs: 0xb0cab0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140395170
//     owner=unresolved_namespace; count=1
//     target_aliases=49 candidates; complete list: gpu_lut.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7830144","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380295344,"raw_hex":"e8808588ff","size":5,"soft_fail":false,"target_va":5372465520,"va":5380295659,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141770e40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: gpu_lut.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12993292","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380295344,"raw_hex":"e80c43c600","size":5,"soft_fail":false,"target_va":5393288768,"va":5380295471,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14177a260
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: gpu_lut.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13031164","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380295344,"raw_hex":"e8fcd6c600","size":5,"soft_fail":false,"target_va":5393326688,"va":5380295519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: gpu_lut.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36761509","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380295344,"raw_hex":"e8a5ef3002","size":5,"soft_fail":false,"target_va":5417057056,"va":5380295542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::get_rg_preset_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-1016114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380295344,"raw_hex":"e8ce7ef0ff","size":5,"soft_fail":false,"target_va":5379279280,"va":5380295389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t36279320","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380295344,"raw_hex":"e818942902","size":5,"soft_fail":false,"target_va":5416574944,"va":5380295619,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3689784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380295344,"raw_hex":"e8384d3800","size":5,"soft_fail":false,"target_va":5383985360,"va":5380295571,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3243645","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380295344,"raw_hex":"e87d7e3100","size":5,"soft_fail":false,"target_va":5383539152,"va":5380295502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*40(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5380295344,"raw_hex":"ff5028","size":3,"soft_fail":false,"va":5380295557,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":560,"record_offset":13968,"parent_record_offset":13800,"ancestor_record_offset":13800,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::gpu_lut::GpuLutApi::compute
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gpu_lut.rs
//     evidence={"module":560,"record_offset":13840,"parent_record_offset":13800,"ancestor_record_offset":13800,"inlinee":35169,"source_line":57,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$33::call
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":560,"record_offset":13936,"parent_record_offset":13840,"ancestor_record_offset":13800,"inlinee":8038,"source_line":2253,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":560,"record_offset":14148,"parent_record_offset":14024,"ancestor_record_offset":13800,"inlinee":6105,"source_line":3033,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":560,"record_offset":14172,"parent_record_offset":14148,"ancestor_record_offset":13800,"inlinee":4968,"source_line":3955,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::lut::water_noise::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":560,"record_offset":13904,"parent_record_offset":13868,"ancestor_record_offset":13800,"inlinee":33208,"source_line":19719,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::rwlock::impl$55::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.14","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.14/src/rwlock.rs
//     evidence={"module":560,"record_offset":13996,"parent_record_offset":13968,"ancestor_record_offset":13800,"inlinee":6102,"source_line":3102,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::RawRwLock::deadlock_release
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":560,"record_offset":14060,"parent_record_offset":14024,"ancestor_record_offset":13800,"inlinee":6104,"source_line":1154,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_rwlock::impl$0::unlock_exclusive
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.5","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.5/src/raw_rwlock.rs
//     evidence={"module":560,"record_offset":14024,"parent_record_offset":13996,"ancestor_record_offset":13800,"inlinee":6103,"source_line":94,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot_core","version":"0.9.12","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.12/src/parking_lot.rs
//     evidence={"module":560,"record_offset":14088,"parent_record_offset":14060,"ancestor_record_offset":13800,"inlinee":4966,"source_line":1128,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":560,"record_offset":13868,"parent_record_offset":13840,"ancestor_record_offset":13800,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$3,tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeNode> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":560,"record_offset":14840,"rva":11578896,"va":5380288016}
// 编译实例 RVAs: 0xb0ae10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: gpu_lut.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36769013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380288016,"raw_hex":"e8f50c3102","size":5,"soft_fail":false,"target_va":5417057056,"va":5380288038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3248585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380288016,"raw_hex":"e8c9913100","size":5,"soft_fail":false,"target_va":5383536672,"va":5380288082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":560,"record_offset":15100,"parent_record_offset":14840,"ancestor_record_offset":14840,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::gpu_lut::impl$0::build::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/gpu_lut.rs
//     evidence={"module":560,"record_offset":15120,"parent_record_offset":15100,"ancestor_record_offset":14840,"inlinee":35171,"source_line":41,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":560,"record_offset":15140,"parent_record_offset":15120,"ancestor_record_offset":14840,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::gpu_lut::impl$0::build::closure$3
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 41
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8963
// 首条位置: {"module":560,"inlinee":35171}
// 编译实例 RVAs: 0xb0ae10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: gpu_lut.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36769013","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380288016,"raw_hex":"e8f50c3102","size":5,"soft_fail":false,"target_va":5417057056,"va":5380288038,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_impl
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     evidence={"assembly":"\tcallq\t3248585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5380288016,"raw_hex":"e8c9913100","size":5,"soft_fail":false,"target_va":5383536672,"va":5380288082,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::rg::node_builder::compute_pass_builder::ComputeNode::dispatch_threads
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":560,"record_offset":15140,"parent_record_offset":15120,"ancestor_record_offset":15120,"inlinee":13027,"source_line":224,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::gpu_lut::GpuLutApi::compute
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 57
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8961
// 首条位置: {"module":560,"inlinee":35169}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::boxed::impl$33::call
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":560,"record_offset":13936,"parent_record_offset":13840,"ancestor_record_offset":13840,"inlinee":8038,"source_line":2253,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::rg::api::GraphApi::compute
//     owner=git_dependency_definition; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/.cargo/git/checkouts/rhapsody2-06daace556675b61/48efa87/src/rg/node_builder/compute_pass_builder.rs
//     evidence={"module":560,"record_offset":13868,"parent_record_offset":13840,"ancestor_record_offset":13840,"inlinee":13022,"source_line":836,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] country_core::shader::lut::water_noise::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/country-core-6440573039d4527c/out/shabriri.rs
//     evidence={"module":560,"record_offset":13904,"parent_record_offset":13868,"ancestor_record_offset":13840,"inlinee":33208,"source_line":19719,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody_core::shader::compute_brdf_lut::impl$3::to_shader_permutation_key
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody-core","version":"0.1.0","source":"git+ssh://git@github.com/PounceLight/rhapsody2.git?rev=48efa87114d22e484592a0e9efe866faafa6a876#48efa87114d22e484592a0e9efe866faafa6a876"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/target/x86_64-pc-windows-msvc/release/build/rhapsody-core-d39e02cd2b2655fe/out/shabriri.rs
//     evidence={"module":560,"record_offset":16560,"parent_record_offset":16524,"ancestor_record_offset":16496,"inlinee":35174,"source_line":687,"archive":"_pdb/C/modules/00560.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::gpu_lut::GpuLutDesc::new
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x85EA
// 首条位置: {"module":550,"inlinee":34282}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::gpu_lut::GpuLutTracker::render_luts
// 记录形式: compiled_procedure; instances=1
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":550,"record_offset":54188,"rva":11296560,"va":5380005680}
// 编译实例 RVAs: 0xac5f30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*40(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5380005680,"raw_hex":"ff5028","size":3,"soft_fail":false,"va":5380005785,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::slice::iter::impl$179::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":550,"record_offset":54332,"parent_record_offset":54188,"ancestor_record_offset":54188,"inlinee":5064,"source_line":157,"archive":"_pdb/C/modules/00550.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":550,"record_offset":54360,"parent_record_offset":54332,"ancestor_record_offset":54188,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00550.jsonl.gz"}
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
// 完整结构及原始类型记录: gpu_lut.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::gpu_lut::GpuLutApi
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<country_core::resources::gpu_lut::GpuLutApi>", "constructor": "ref_mut$", "arguments": ["country_core::resources::gpu_lut::GpuLutApi"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref_mut$<country_core::resources::gpu_lut::GpuLutApi>,ref_mut$<country_core::asset_libraries::texture_library::AssetTextureLibrary> >", "constructor": "tuple$", "arguments": ["ref_mut$<country_core::resources::gpu_lut::GpuLutApi>", "ref_mut$<country_core::asset_libraries::texture_library::AssetTextureLibrary>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$2,tuple$<ref_mut$<country_core::resources::gpu_lut::GpuLutApi>,ref_mut$<country_core::asset_libraries::texture_library::AssetTextureLibrary> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::resources::gpu_lut::impl$0::build::closure_env$2", "tuple$<ref_mut$<country_core::resources::gpu_lut::GpuLutApi>,ref_mut$<country_core::asset_libraries::texture_library::AssetTextureLibrary> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$0,tuple$<ref_mut$<country_core::resources::gpu_lut::GpuLutApi>,ref_mut$<country_core::asset_libraries::texture_library::AssetTextureLibrary> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["country_core::resources::gpu_lut::impl$0::build::closure_env$0", "tuple$<ref_mut$<country_core::resources::gpu_lut::GpuLutApi>,ref_mut$<country_core::asset_libraries::texture_library::AssetTextureLibrary> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::gpu_lut::GpuLutDesc
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 2
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::gpu_lut::GpuLutDesc>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::gpu_lut::GpuLutDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::resources::gpu_lut::GpuLutDesc,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::resources::gpu_lut::GpuLutDesc", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::resources::gpu_lut::GpuLutDesc,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::resources::gpu_lut::GpuLutDesc,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::gpu_lut::GpuLutPlugin
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 6
// [PDB 泛型关系，非字段关系] {"type": "core::any::impl$0::type_id<country_core::resources::gpu_lut::GpuLutPlugin>", "constructor": "core::any::impl$0::type_id", "arguments": ["country_core::resources::gpu_lut::GpuLutPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any_mut<country_core::resources::gpu_lut::GpuLutPlugin>", "constructor": "downcast_rs::impl$0::as_any_mut", "arguments": ["country_core::resources::gpu_lut::GpuLutPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::into_any_rc<country_core::resources::gpu_lut::GpuLutPlugin>", "constructor": "downcast_rs::impl$0::into_any_rc", "arguments": ["country_core::resources::gpu_lut::GpuLutPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "downcast_rs::impl$0::as_any<country_core::resources::gpu_lut::GpuLutPlugin>", "constructor": "downcast_rs::impl$0::as_any", "arguments": ["country_core::resources::gpu_lut::GpuLutPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::Plugin::name<country_core::resources::gpu_lut::GpuLutPlugin>", "constructor": "bevy_app::plugin::Plugin::name", "arguments": ["country_core::resources::gpu_lut::GpuLutPlugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::plugin::sealed::impl$16::add_to_app<bevy_app::plugin::sealed::PluginMarker,tiny_glade::decorator_plugin::DecoratorPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::mod_editor::ModEditorPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::backend::icc::IccProfilePlugin,bevy_app::plugin::sealed::PluginMarker,system_critters::CritterPlugin,bevy_app::plugin::sealed::PluginMarker,system_critters::sheep::SheepPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::resources::gpu_lut::GpuLutPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::components::drawable::PrevTransformPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::render::fsr_plugin::plugin_impl::FsrPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::dlss_plugin::plugin_impl::DlssPlugin,bevy_app::plugin::sealed::PluginMarker,tiny_glade::rollercoaster::RollercoasterPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::startup::load_glade_tod_and_weather::SnowPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::egui_input::EguiInputPlugin,bevy_app::plugin::sealed::PluginMarker,country_core::systems::grass2::Grass2Plugin>", "constructor": "bevy_app::plugin::sealed::impl$16::add_to_app", "arguments": ["bevy_app::plugin::sealed::PluginMarker", "tiny_glade::decorator_plugin::DecoratorPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::mod_editor::ModEditorPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::backend::icc::IccProfilePlugin", "bevy_app::plugin::sealed::PluginMarker", "system_critters::CritterPlugin", "bevy_app::plugin::sealed::PluginMarker", "system_critters::sheep::SheepPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::theme_fetcher_plugin::ThemeFetcherPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::resources::gpu_lut::GpuLutPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::components::drawable::PrevTransformPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::render::fsr_plugin::plugin_impl::FsrPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::dlss_plugin::plugin_impl::DlssPlugin", "bevy_app::plugin::sealed::PluginMarker", "tiny_glade::rollercoaster::RollercoasterPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::startup::load_glade_tod_and_weather::SnowPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::egui_input::EguiInputPlugin", "bevy_app::plugin::sealed::PluginMarker", "country_core::systems::grass2::Grass2Plugin"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::resources::gpu_lut::GpuLutTracker
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::resources::gpu_lut::GpuLutTracker>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::resources::gpu_lut::GpuLutTracker"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_app::app::App::insert_resource<country_core::resources::gpu_lut::GpuLutTracker>", "constructor": "bevy_app::app::App::insert_resource", "arguments": ["country_core::resources::gpu_lut::GpuLutTracker"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_ecs::component::info::ComponentDescriptor::drop_ptr<country_core::resources::gpu_lut::GpuLutTracker>", "constructor": "bevy_ecs::component::info::ComponentDescriptor::drop_ptr", "arguments": ["country_core::resources::gpu_lut::GpuLutTracker"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: gpu_lut.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: gpu_lut.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::result::Result<array$<f32,2>,anyhow::Error> > (*)(ref$<enum2$<serde_json::value::Value> >)
// 候选参数类型: ["ref$<enum2$<serde_json::value::Value> >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::result::Result<array$<f32,3>,anyhow::Error> > (*)(ref$<enum2$<serde_json::value::Value> >)
// 候选参数类型: ["ref$<enum2$<serde_json::value::Value> >"]

// ------------------------------------------------------------------
// 函数: core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$1,tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeNode> >
// 信息 ID: f24069b564dbb976d0d4cd9e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["rhapsody::rg::node_builder::compute_pass_builder::ComputeNode"]; 字段布局 = unknown。
// [类型关系] core::ops::function::FnOnce::call_once -> ["country_core::resources::gpu_lut::impl$0::build::closure_env$1", "tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeNode>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$3,tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeNode> >
// 信息 ID: 4245ba94d32894e39e6b9ae5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["rhapsody::rg::node_builder::compute_pass_builder::ComputeNode"]; 字段布局 = unknown。
// [类型关系] core::ops::function::FnOnce::call_once -> ["country_core::resources::gpu_lut::impl$0::build::closure_env$3", "tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeNode>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::gpu_lut::GpuLutApi::compute
// 信息 ID: 700f8bf92ca916a1030d2e9c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$2,tuple$<ref_mut$<country_core::resources::gpu_lut::GpuLutApi>,ref_mut$<country_core::asset_libraries::texture_library::AssetTextureLibrary> > >; module=560, inline_record=13840; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$0,tuple$<ref_mut$<country_core::resources::gpu_lut::GpuLutApi>,ref_mut$<country_core::asset_libraries::texture_library::AssetTextureLibrary> > >; module=560, inline_record=16496; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::gpu_lut::impl$0::build::closure$0; module=560, inline_record=44464; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::gpu_lut::impl$0::build::closure$2; module=560, inline_record=44992; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::gpu_lut::GpuLutDesc::new
// 信息 ID: 98db9d6b2cfb43f07f27d5ed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::resources::gpu_lut::impl$0::build; module=550, inline_record=62064; 不是独立调用边。
// [PDB 内联宿主] country_core::resources::gpu_lut::impl$0::build; module=550, inline_record=62308; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::gpu_lut::GpuLutTracker::render_luts
// 信息 ID: 9f417d3c0ddc2fb65d378c21
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compile::compile<rhapsody::rg::resource::Handle<rhapsody::raw::texture::Texture>,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x39948d -> 0xac5f30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::gpu_lut::impl$0::build
// 信息 ID: a2965dfec2072bdbf1e6cd86
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::gpu_lut::impl$0::build::closure$0
// 信息 ID: 7304a641901d2bbc8824dfb4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::resources::gpu_lut::impl$0::build", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::gpu_lut::impl$0::build::closure_env$0 <- core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$0,tuple$<ref_mut$<country_core::resources::gpu_lut::GpuLutApi>,ref_mut$<country_core::asset_libraries::texture_library::AssetTextureLibrary> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$0,tuple$<ref_mut$<country_core::resources::gpu_lut::GpuLutApi>,ref_mut$<country_core::asset_libraries::texture_library::AssetTextureLibrary> > >; module=560, inline_record=16456; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::gpu_lut::impl$0::build::closure$1
// 信息 ID: 348b70d45650070079d917e1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::resources::gpu_lut::impl$0::build", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::gpu_lut::impl$0::build::closure_env$1 <- core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$1,tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeNode> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$1,tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeNode> >; module=560, inline_record=14508; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::gpu_lut::impl$0::build::closure$2
// 信息 ID: 3a4775e99c241f19daa9299e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::resources::gpu_lut::impl$0::build", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::gpu_lut::impl$0::build::closure_env$2 <- core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$2,tuple$<ref_mut$<country_core::resources::gpu_lut::GpuLutApi>,ref_mut$<country_core::asset_libraries::texture_library::AssetTextureLibrary> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$2,tuple$<ref_mut$<country_core::resources::gpu_lut::GpuLutApi>,ref_mut$<country_core::asset_libraries::texture_library::AssetTextureLibrary> > >; module=560, inline_record=13800; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::gpu_lut::impl$0::build::closure$3
// 信息 ID: fdf3418116376bf3ca5cb875
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 3, "lexical_owner": "country_core::resources::gpu_lut::impl$0::build", "captures": "unknown"}
// [闭包环境类型引用] country_core::resources::gpu_lut::impl$0::build::closure_env$3 <- core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$3,tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeNode> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::resources::gpu_lut::impl$0::build::closure_env$3,tuple$<rhapsody::rg::node_builder::compute_pass_builder::ComputeNode> >; module=560, inline_record=15120; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
