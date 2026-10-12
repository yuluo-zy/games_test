// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/raw/binding.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 24eeaa8236aba6f160356e78321d948ba0c808dd867acb21c73b1e4ed5138eb7
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: binding.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 63, 572, 576, 703, 1610, 1624, 1634, 1657
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use bumpalo; // namespace candidate
//   package identities: [{"name":"bumpalo","source":"registry+https://github.com/rust-lang/crates.io-index","version":"3.18.1"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use shader_pipeline; // namespace candidate
//   package identities: [{"name":"shader-pipeline","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::binding::impl$1::fmt
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8092
// 首条位置: {"module":576,"inlinee":32914}
// 编译实例 RVAs: 0x1621310
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field2_finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9021585","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391913744,"raw_hex":"e86f5776ff","size":5,"soft_fail":false,"target_va":5382892704,"va":5391914284,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_tuple_field1_finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9019885","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5391913744,"raw_hex":"e8135e76ff","size":5,"soft_fail":false,"target_va":5382894288,"va":5391914168,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::binding::impl$2::clone
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2683
// 首条位置: {"module":63,"inlinee":9859}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::definitions::impl$7164::clone
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":63,"record_offset":53724,"parent_record_offset":53668,"ancestor_record_offset":53668,"inlinee":9861,"source_line":1866,"archive":"_pdb/B/modules/00063.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":63,"record_offset":54188,"parent_record_offset":54164,"ancestor_record_offset":53668,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00063.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":63,"record_offset":54212,"parent_record_offset":54188,"ancestor_record_offset":53668,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00063.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":63,"record_offset":54164,"parent_record_offset":53988,"ancestor_record_offset":53668,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00063.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":63,"record_offset":53932,"parent_record_offset":53904,"ancestor_record_offset":53668,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00063.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":63,"record_offset":53988,"parent_record_offset":53960,"ancestor_record_offset":53668,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00063.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":63,"record_offset":53960,"parent_record_offset":53932,"ancestor_record_offset":53668,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00063.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":63,"record_offset":54024,"parent_record_offset":53988,"ancestor_record_offset":53668,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00063.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":63,"record_offset":53864,"parent_record_offset":53832,"ancestor_record_offset":53668,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00063.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":63,"record_offset":53832,"parent_record_offset":53764,"ancestor_record_offset":53668,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00063.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":63,"record_offset":53804,"parent_record_offset":53764,"ancestor_record_offset":53668,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00063.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":63,"record_offset":53904,"parent_record_offset":53864,"ancestor_record_offset":53668,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00063.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":63,"record_offset":53764,"parent_record_offset":53724,"ancestor_record_offset":53668,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00063.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":63,"record_offset":54048,"parent_record_offset":54024,"ancestor_record_offset":53668,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00063.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":63,"record_offset":54072,"parent_record_offset":54048,"ancestor_record_offset":53668,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00063.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":63,"record_offset":54288,"parent_record_offset":54264,"ancestor_record_offset":53668,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00063.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":63,"record_offset":54096,"parent_record_offset":54072,"ancestor_record_offset":53668,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00063.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":63,"record_offset":54120,"parent_record_offset":54096,"ancestor_record_offset":53668,"inlinee":4619,"source_line":2579,"archive":"_pdb/B/modules/00063.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":63,"record_offset":54264,"parent_record_offset":53864,"ancestor_record_offset":53668,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00063.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::bind_pipeline
// 记录形式: compiled_procedure; instances=1
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":7692,"rva":12509488,"va":5381218608}
// 编译实例 RVAs: 0xbee130
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::cmd_bind_pipeline
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":703,"record_offset":7828,"parent_record_offset":7692,"ancestor_record_offset":7692,"inlinee":39589,"source_line":1850,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1657,"record_offset":37580,"rva":23748624,"va":5392457744}
// 编译实例 RVAs: 0x16a6010
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e6620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17563388","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392457744,"raw_hex":"e80401f4fe","size":5,"soft_fail":false,"target_va":5374895648,"va":5392459031,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17376951","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392457744,"raw_hex":"e849d9f6fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5392458450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14088e400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14779122","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392457744,"raw_hex":"e80e7d1eff","size":5,"soft_fail":false,"target_va":5377680384,"va":5392459501,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b05f50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12190531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392457744,"raw_hex":"e8bdfc45ff","size":5,"soft_fail":false,"target_va":5380267856,"va":5392458382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b06020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12190483","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392457744,"raw_hex":"e8edfc45ff","size":5,"soft_fail":false,"target_va":5380268064,"va":5392458542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141479b60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2279328","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392457744,"raw_hex":"e86038ddff","size":5,"soft_fail":false,"target_va":5390179168,"va":5392458491,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::Bump::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-9813212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392457744,"raw_hex":"e824436aff","size":5,"soft_fail":false,"target_va":5382644656,"va":5392457863,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::collections::raw_vec::RawVec$LT$T$GT$::allocate_in
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1181894","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392457744,"raw_hex":"e8c6081200","size":5,"soft_fail":false,"target_va":5393639872,"va":5392457973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::collections::raw_vec::RawVec$LT$T$GT$::reserve_internal_or_panic
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1181876","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392457744,"raw_hex":"e8b4081200","size":5,"soft_fail":false,"target_va":5393640464,"va":5392458583,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19083308","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392457744,"raw_hex":"e82c302301","size":5,"soft_fail":false,"target_va":5411542296,"va":5392458983,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::shader::DescriptorSetState::allocate_descriptor_set
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11273841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392457744,"raw_hex":"e88ff953ff","size":5,"soft_fail":false,"target_va":5381184064,"va":5392457900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-17377149","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5392457744,"raw_hex":"e983d8f6fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5392458648,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*616(%rsi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5392457744,"raw_hex":"ff9668020000","size":6,"soft_fail":false,"va":5392458731,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::cmd_bind_descriptor_sets
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1657,"record_offset":43156,"parent_record_offset":37580,"ancestor_record_offset":37580,"inlinee":39591,"source_line":1761,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":42752,"parent_record_offset":37580,"ancestor_record_offset":37580,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1657,"record_offset":38416,"parent_record_offset":37580,"ancestor_record_offset":37580,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::update_descriptor_set
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/binding.rs
//     evidence={"module":1657,"record_offset":38472,"parent_record_offset":37580,"ancestor_record_offset":37580,"inlinee":75154,"source_line":96,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1657,"record_offset":39888,"parent_record_offset":39864,"ancestor_record_offset":37580,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1657,"record_offset":39864,"parent_record_offset":39776,"ancestor_record_offset":37580,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":39804,"parent_record_offset":39776,"ancestor_record_offset":37580,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":39776,"parent_record_offset":39752,"ancestor_record_offset":37580,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":39752,"parent_record_offset":39728,"ancestor_record_offset":37580,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1657,"record_offset":39408,"parent_record_offset":39380,"ancestor_record_offset":37580,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1657,"record_offset":39656,"parent_record_offset":39632,"ancestor_record_offset":37580,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1657,"record_offset":39704,"parent_record_offset":39680,"ancestor_record_offset":37580,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1657,"record_offset":39176,"parent_record_offset":39144,"ancestor_record_offset":37580,"inlinee":11832,"source_line":324,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::update_descriptor_sets
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1657,"record_offset":41064,"parent_record_offset":38472,"ancestor_record_offset":37580,"inlinee":75157,"source_line":1356,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::dealloc
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1657,"record_offset":41288,"parent_record_offset":41220,"ancestor_record_offset":37580,"inlinee":19984,"source_line":2228,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::is_last_allocation
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1657,"record_offset":41320,"parent_record_offset":41288,"ancestor_record_offset":37580,"inlinee":19985,"source_line":2221,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::ChunkFooter::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1657,"record_offset":43652,"parent_record_offset":43616,"ancestor_record_offset":37580,"inlinee":12170,"source_line":375,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::current_layout
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":41252,"parent_record_offset":41220,"ancestor_record_offset":37580,"inlinee":19983,"source_line":158,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::dealloc_buffer
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":41220,"parent_record_offset":41196,"ancestor_record_offset":37580,"inlinee":19982,"source_line":701,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::infallible_reserve_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":38880,"parent_record_offset":38856,"ancestor_record_offset":37580,"inlinee":19975,"source_line":607,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":38784,"parent_record_offset":38756,"ancestor_record_offset":37580,"inlinee":19972,"source_line":137,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::reserve
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":38856,"parent_record_offset":38832,"ancestor_record_offset":37580,"inlinee":19974,"source_line":429,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::with_capacity_in
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":38572,"parent_record_offset":38544,"ancestor_record_offset":37580,"inlinee":75156,"source_line":74,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::impl$5::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":41196,"parent_record_offset":41172,"ancestor_record_offset":37580,"inlinee":19981,"source_line":713,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":1657,"record_offset":38720,"parent_record_offset":38664,"ancestor_record_offset":37580,"inlinee":19969,"source_line":1451,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::push_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":1657,"record_offset":38756,"parent_record_offset":38720,"ancestor_record_offset":37580,"inlinee":19970,"source_line":1422,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::reserve
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":1657,"record_offset":38832,"parent_record_offset":38720,"ancestor_record_offset":37580,"inlinee":19973,"source_line":736,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::with_capacity_in
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":1657,"record_offset":38544,"parent_record_offset":38472,"ancestor_record_offset":37580,"inlinee":75155,"source_line":585,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::impl$17::extend
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":1657,"record_offset":38664,"parent_record_offset":38472,"ancestor_record_offset":37580,"inlinee":44844,"source_line":2311,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::dealloc_chunk_list
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1657,"record_offset":43616,"parent_record_offset":43592,"ancestor_record_offset":37580,"inlinee":12168,"source_line":395,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$6::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1657,"record_offset":43592,"parent_record_offset":43568,"ancestor_record_offset":37580,"inlinee":16119,"source_line":387,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1657,"record_offset":41344,"parent_record_offset":41320,"ancestor_record_offset":37580,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1657,"record_offset":41436,"parent_record_offset":41412,"ancestor_record_offset":37580,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1657,"record_offset":41412,"parent_record_offset":41288,"ancestor_record_offset":37580,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1657,"record_offset":39212,"parent_record_offset":39176,"ancestor_record_offset":37580,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1657,"record_offset":39020,"parent_record_offset":38984,"ancestor_record_offset":37580,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1657,"record_offset":38984,"parent_record_offset":38952,"ancestor_record_offset":37580,"inlinee":47224,"source_line":167,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::find::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1657,"record_offset":39084,"parent_record_offset":39020,"ancestor_record_offset":37580,"inlinee":47226,"source_line":171,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::find::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1657,"record_offset":39984,"parent_record_offset":39960,"ancestor_record_offset":37580,"inlinee":47229,"source_line":172,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1657,"record_offset":38952,"parent_record_offset":38920,"ancestor_record_offset":37580,"inlinee":4465,"source_line":97,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1657,"record_offset":38920,"parent_record_offset":38664,"ancestor_record_offset":37580,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1657,"record_offset":39144,"parent_record_offset":39112,"ancestor_record_offset":37580,"inlinee":5946,"source_line":115,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_try_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1657,"record_offset":39352,"parent_record_offset":39176,"ancestor_record_offset":37580,"inlinee":10987,"source_line":95,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1657,"record_offset":39112,"parent_record_offset":39084,"ancestor_record_offset":37580,"inlinee":5945,"source_line":2821,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1657,"record_offset":39508,"parent_record_offset":39352,"ancestor_record_offset":37580,"inlinee":8146,"source_line":2828,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1657,"record_offset":41460,"parent_record_offset":41436,"ancestor_record_offset":37580,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1657,"record_offset":39832,"parent_record_offset":39804,"ancestor_record_offset":37580,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::ControlFlow::break_value
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\control_flow.rs
//     evidence={"module":1657,"record_offset":40060,"parent_record_offset":40036,"ancestor_record_offset":37580,"inlinee":14462,"source_line":182,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::impl$0::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\control_flow.rs
//     evidence={"module":1657,"record_offset":39540,"parent_record_offset":39176,"ancestor_record_offset":37580,"inlinee":10993,"source_line":111,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1657,"record_offset":39056,"parent_record_offset":39020,"ancestor_record_offset":37580,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1657,"record_offset":40108,"parent_record_offset":38920,"ancestor_record_offset":37580,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1657,"record_offset":39960,"parent_record_offset":38984,"ancestor_record_offset":37580,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":39584,"parent_record_offset":39020,"ancestor_record_offset":37580,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":43680,"parent_record_offset":43652,"ancestor_record_offset":37580,"inlinee":7359,"source_line":2074,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1657,"record_offset":41384,"parent_record_offset":41288,"ancestor_record_offset":37580,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1657,"record_offset":39324,"parent_record_offset":39176,"ancestor_record_offset":37580,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1657,"record_offset":39268,"parent_record_offset":39176,"ancestor_record_offset":37580,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1657,"record_offset":39236,"parent_record_offset":39212,"ancestor_record_offset":37580,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":39292,"parent_record_offset":39268,"ancestor_record_offset":37580,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":41484,"parent_record_offset":41460,"ancestor_record_offset":37580,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1657,"record_offset":39436,"parent_record_offset":39408,"ancestor_record_offset":37580,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1657,"record_offset":38440,"parent_record_offset":38416,"ancestor_record_offset":37580,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1657,"record_offset":39464,"parent_record_offset":39436,"ancestor_record_offset":37580,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1657,"record_offset":41092,"parent_record_offset":38472,"ancestor_record_offset":37580,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1657,"record_offset":41116,"parent_record_offset":41092,"ancestor_record_offset":37580,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":1657,"record_offset":39380,"parent_record_offset":39352,"ancestor_record_offset":37580,"inlinee":74171,"source_line":540,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1657,"record_offset":31400,"rva":23746832,"va":5392455952}
// 编译实例 RVAs: 0x16a5910
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e6620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17561596","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392455952,"raw_hex":"e80408f4fe","size":5,"soft_fail":false,"target_va":5374895648,"va":5392457239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17375153","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392455952,"raw_hex":"e84fe0f6fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5392456652,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14088d550
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14781106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392455952,"raw_hex":"e84e751eff","size":5,"soft_fail":false,"target_va":5377676624,"va":5392457725,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b05f50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12188736","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392455952,"raw_hex":"e8c00346ff","size":5,"soft_fail":false,"target_va":5380267856,"va":5392456587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b06020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12188688","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392455952,"raw_hex":"e8f00346ff","size":5,"soft_fail":false,"target_va":5380268064,"va":5392456747,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14147a0b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2276173","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392455952,"raw_hex":"e8b344ddff","size":5,"soft_fail":false,"target_va":5390180528,"va":5392456696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::Bump::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-9811420","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392455952,"raw_hex":"e8244a6aff","size":5,"soft_fail":false,"target_va":5382644656,"va":5392456071,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::collections::raw_vec::RawVec$LT$T$GT$::allocate_in
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1183684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392455952,"raw_hex":"e8c40f1200","size":5,"soft_fail":false,"target_va":5393639872,"va":5392456183,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::collections::raw_vec::RawVec$LT$T$GT$::reserve_internal_or_panic
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1183671","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392455952,"raw_hex":"e8b70f1200","size":5,"soft_fail":false,"target_va":5393640464,"va":5392456788,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19085100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392455952,"raw_hex":"e82c372301","size":5,"soft_fail":false,"target_va":5411542296,"va":5392457191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::shader::DescriptorSetState::allocate_descriptor_set
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11272049","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392455952,"raw_hex":"e88f0054ff","size":5,"soft_fail":false,"target_va":5381184064,"va":5392456108,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-17375353","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5392455952,"raw_hex":"e987dff6fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5392456852,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*616(%rsi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5392455952,"raw_hex":"ff9668020000","size":6,"soft_fail":false,"va":5392456929,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::cmd_bind_descriptor_sets
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1657,"record_offset":36948,"parent_record_offset":31400,"ancestor_record_offset":31400,"inlinee":39591,"source_line":1761,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":36544,"parent_record_offset":31400,"ancestor_record_offset":31400,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1657,"record_offset":32208,"parent_record_offset":31400,"ancestor_record_offset":31400,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::update_descriptor_set
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/binding.rs
//     evidence={"module":1657,"record_offset":32264,"parent_record_offset":31400,"ancestor_record_offset":31400,"inlinee":75154,"source_line":96,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1657,"record_offset":33680,"parent_record_offset":33656,"ancestor_record_offset":31400,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1657,"record_offset":33656,"parent_record_offset":33568,"ancestor_record_offset":31400,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":33596,"parent_record_offset":33568,"ancestor_record_offset":31400,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":33568,"parent_record_offset":33544,"ancestor_record_offset":31400,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":33544,"parent_record_offset":33520,"ancestor_record_offset":31400,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1657,"record_offset":33200,"parent_record_offset":33172,"ancestor_record_offset":31400,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1657,"record_offset":33448,"parent_record_offset":33424,"ancestor_record_offset":31400,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1657,"record_offset":33496,"parent_record_offset":33472,"ancestor_record_offset":31400,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1657,"record_offset":32968,"parent_record_offset":32936,"ancestor_record_offset":31400,"inlinee":11832,"source_line":324,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::update_descriptor_sets
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1657,"record_offset":34856,"parent_record_offset":32264,"ancestor_record_offset":31400,"inlinee":75157,"source_line":1356,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::dealloc
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1657,"record_offset":35080,"parent_record_offset":35012,"ancestor_record_offset":31400,"inlinee":19984,"source_line":2228,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::is_last_allocation
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1657,"record_offset":35112,"parent_record_offset":35080,"ancestor_record_offset":31400,"inlinee":19985,"source_line":2221,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::ChunkFooter::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1657,"record_offset":37444,"parent_record_offset":37408,"ancestor_record_offset":31400,"inlinee":12170,"source_line":375,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::current_layout
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":35044,"parent_record_offset":35012,"ancestor_record_offset":31400,"inlinee":19983,"source_line":158,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::dealloc_buffer
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":35012,"parent_record_offset":34988,"ancestor_record_offset":31400,"inlinee":19982,"source_line":701,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::infallible_reserve_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":32672,"parent_record_offset":32648,"ancestor_record_offset":31400,"inlinee":19975,"source_line":607,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":32576,"parent_record_offset":32548,"ancestor_record_offset":31400,"inlinee":19972,"source_line":137,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::reserve
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":32648,"parent_record_offset":32624,"ancestor_record_offset":31400,"inlinee":19974,"source_line":429,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::with_capacity_in
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":32364,"parent_record_offset":32336,"ancestor_record_offset":31400,"inlinee":75156,"source_line":74,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::impl$5::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":34988,"parent_record_offset":34964,"ancestor_record_offset":31400,"inlinee":19981,"source_line":713,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":1657,"record_offset":32512,"parent_record_offset":32456,"ancestor_record_offset":31400,"inlinee":19969,"source_line":1451,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::push_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":1657,"record_offset":32548,"parent_record_offset":32512,"ancestor_record_offset":31400,"inlinee":19970,"source_line":1422,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::reserve
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":1657,"record_offset":32624,"parent_record_offset":32512,"ancestor_record_offset":31400,"inlinee":19973,"source_line":736,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::with_capacity_in
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":1657,"record_offset":32336,"parent_record_offset":32264,"ancestor_record_offset":31400,"inlinee":75155,"source_line":585,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::impl$17::extend
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":1657,"record_offset":32456,"parent_record_offset":32264,"ancestor_record_offset":31400,"inlinee":44844,"source_line":2311,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::dealloc_chunk_list
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1657,"record_offset":37408,"parent_record_offset":37384,"ancestor_record_offset":31400,"inlinee":12168,"source_line":395,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$6::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1657,"record_offset":37384,"parent_record_offset":37360,"ancestor_record_offset":31400,"inlinee":16119,"source_line":387,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1657,"record_offset":35136,"parent_record_offset":35112,"ancestor_record_offset":31400,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1657,"record_offset":35228,"parent_record_offset":35204,"ancestor_record_offset":31400,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1657,"record_offset":35204,"parent_record_offset":35080,"ancestor_record_offset":31400,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1657,"record_offset":33004,"parent_record_offset":32968,"ancestor_record_offset":31400,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1657,"record_offset":32812,"parent_record_offset":32776,"ancestor_record_offset":31400,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1657,"record_offset":32776,"parent_record_offset":32744,"ancestor_record_offset":31400,"inlinee":47224,"source_line":167,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::find::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1657,"record_offset":32876,"parent_record_offset":32812,"ancestor_record_offset":31400,"inlinee":47226,"source_line":171,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::find::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1657,"record_offset":33776,"parent_record_offset":33752,"ancestor_record_offset":31400,"inlinee":47229,"source_line":172,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1657,"record_offset":32744,"parent_record_offset":32712,"ancestor_record_offset":31400,"inlinee":4465,"source_line":97,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1657,"record_offset":32712,"parent_record_offset":32456,"ancestor_record_offset":31400,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1657,"record_offset":32936,"parent_record_offset":32904,"ancestor_record_offset":31400,"inlinee":5946,"source_line":115,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_try_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1657,"record_offset":33144,"parent_record_offset":32968,"ancestor_record_offset":31400,"inlinee":10987,"source_line":95,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1657,"record_offset":32904,"parent_record_offset":32876,"ancestor_record_offset":31400,"inlinee":5945,"source_line":2821,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1657,"record_offset":33300,"parent_record_offset":33144,"ancestor_record_offset":31400,"inlinee":8146,"source_line":2828,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1657,"record_offset":35252,"parent_record_offset":35228,"ancestor_record_offset":31400,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1657,"record_offset":33624,"parent_record_offset":33596,"ancestor_record_offset":31400,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::ControlFlow::break_value
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\control_flow.rs
//     evidence={"module":1657,"record_offset":33852,"parent_record_offset":33828,"ancestor_record_offset":31400,"inlinee":14462,"source_line":182,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::impl$0::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\control_flow.rs
//     evidence={"module":1657,"record_offset":33332,"parent_record_offset":32968,"ancestor_record_offset":31400,"inlinee":10993,"source_line":111,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1657,"record_offset":32848,"parent_record_offset":32812,"ancestor_record_offset":31400,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1657,"record_offset":33900,"parent_record_offset":32712,"ancestor_record_offset":31400,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1657,"record_offset":33752,"parent_record_offset":32776,"ancestor_record_offset":31400,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=40
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":33376,"parent_record_offset":32812,"ancestor_record_offset":31400,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":37472,"parent_record_offset":37444,"ancestor_record_offset":31400,"inlinee":7359,"source_line":2074,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1657,"record_offset":35176,"parent_record_offset":35080,"ancestor_record_offset":31400,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1657,"record_offset":33116,"parent_record_offset":32968,"ancestor_record_offset":31400,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1657,"record_offset":33060,"parent_record_offset":32968,"ancestor_record_offset":31400,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1657,"record_offset":33028,"parent_record_offset":33004,"ancestor_record_offset":31400,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":33084,"parent_record_offset":33060,"ancestor_record_offset":31400,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":35276,"parent_record_offset":35252,"ancestor_record_offset":31400,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1657,"record_offset":33228,"parent_record_offset":33200,"ancestor_record_offset":31400,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1657,"record_offset":32232,"parent_record_offset":32208,"ancestor_record_offset":31400,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1657,"record_offset":33256,"parent_record_offset":33228,"ancestor_record_offset":31400,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1657,"record_offset":34884,"parent_record_offset":32264,"ancestor_record_offset":31400,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1657,"record_offset":34908,"parent_record_offset":34884,"ancestor_record_offset":31400,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":1657,"record_offset":33172,"parent_record_offset":33144,"ancestor_record_offset":31400,"inlinee":74167,"source_line":585,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$4> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1657,"record_offset":26816,"rva":23745408,"va":5392454528}
// 编译实例 RVAs: 0x16a5380
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17778439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392454528,"raw_hex":"e8f9b8f0fe","size":5,"soft_fail":false,"target_va":5374676688,"va":5392455122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e6620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17560267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392454528,"raw_hex":"e8350df4fe","size":5,"soft_fail":false,"target_va":5374895648,"va":5392455910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17374100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392454528,"raw_hex":"e86ce4f6fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5392455599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b06020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12187212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392454528,"raw_hex":"e8b40946ff","size":5,"soft_fail":false,"target_va":5380268064,"va":5392455271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::Bump::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-9809977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392454528,"raw_hex":"e8c74f6aff","size":5,"soft_fail":false,"target_va":5382644656,"va":5392454628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::collections::raw_vec::RawVec$LT$T$GT$::allocate_in
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1185166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392454528,"raw_hex":"e88e151200","size":5,"soft_fail":false,"target_va":5393639872,"va":5392454701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::collections::raw_vec::RawVec$LT$T$GT$::reserve_internal_or_panic
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1185147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392454528,"raw_hex":"e87b151200","size":5,"soft_fail":false,"target_va":5393640464,"va":5392455312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::shader::DescriptorSetState::allocate_descriptor_set
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11270588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392454528,"raw_hex":"e8440654ff","size":5,"soft_fail":false,"target_va":5381184064,"va":5392454647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*616(%r14)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5392454528,"raw_hex":"41ff9668020000","size":7,"soft_fail":false,"va":5392455389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::cmd_bind_descriptor_sets
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1657,"record_offset":30768,"parent_record_offset":26816,"ancestor_record_offset":26816,"inlinee":39591,"source_line":1761,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":30796,"parent_record_offset":26816,"ancestor_record_offset":26816,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1657,"record_offset":27236,"parent_record_offset":26816,"ancestor_record_offset":26816,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::update_descriptor_set
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/binding.rs
//     evidence={"module":1657,"record_offset":27292,"parent_record_offset":26816,"ancestor_record_offset":26816,"inlinee":75154,"source_line":96,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1657,"record_offset":31356,"parent_record_offset":31228,"ancestor_record_offset":26816,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::update_descriptor_sets
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1657,"record_offset":29504,"parent_record_offset":27292,"ancestor_record_offset":26816,"inlinee":75157,"source_line":1356,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::dealloc
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1657,"record_offset":29728,"parent_record_offset":29660,"ancestor_record_offset":26816,"inlinee":19984,"source_line":2228,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::is_last_allocation
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1657,"record_offset":29760,"parent_record_offset":29728,"ancestor_record_offset":26816,"inlinee":19985,"source_line":2221,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::ChunkFooter::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1657,"record_offset":31264,"parent_record_offset":31228,"ancestor_record_offset":26816,"inlinee":12170,"source_line":375,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::current_layout
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":29692,"parent_record_offset":29660,"ancestor_record_offset":26816,"inlinee":19983,"source_line":158,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::dealloc_buffer
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":29660,"parent_record_offset":29636,"ancestor_record_offset":26816,"inlinee":19982,"source_line":701,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::infallible_reserve_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":29460,"parent_record_offset":29436,"ancestor_record_offset":26816,"inlinee":19975,"source_line":607,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":29364,"parent_record_offset":29328,"ancestor_record_offset":26816,"inlinee":19972,"source_line":137,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::reserve
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":29436,"parent_record_offset":29412,"ancestor_record_offset":26816,"inlinee":19974,"source_line":429,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::with_capacity_in
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":27376,"parent_record_offset":27348,"ancestor_record_offset":26816,"inlinee":75156,"source_line":74,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::impl$5::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":29636,"parent_record_offset":29612,"ancestor_record_offset":26816,"inlinee":19981,"source_line":713,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":1657,"record_offset":29288,"parent_record_offset":27468,"ancestor_record_offset":26816,"inlinee":19969,"source_line":1451,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::push_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":1657,"record_offset":29328,"parent_record_offset":29288,"ancestor_record_offset":26816,"inlinee":19970,"source_line":1422,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::reserve
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":1657,"record_offset":29412,"parent_record_offset":29288,"ancestor_record_offset":26816,"inlinee":19973,"source_line":736,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::with_capacity_in
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":1657,"record_offset":27348,"parent_record_offset":27292,"ancestor_record_offset":26816,"inlinee":75155,"source_line":585,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::impl$17::extend
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":1657,"record_offset":27468,"parent_record_offset":27292,"ancestor_record_offset":26816,"inlinee":44844,"source_line":2311,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::dealloc_chunk_list
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1657,"record_offset":31228,"parent_record_offset":31204,"ancestor_record_offset":26816,"inlinee":12168,"source_line":395,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$6::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1657,"record_offset":31204,"parent_record_offset":31180,"ancestor_record_offset":26816,"inlinee":16119,"source_line":387,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1657,"record_offset":29784,"parent_record_offset":29760,"ancestor_record_offset":26816,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1657,"record_offset":29876,"parent_record_offset":29852,"ancestor_record_offset":26816,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1657,"record_offset":29852,"parent_record_offset":29728,"ancestor_record_offset":26816,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1657,"record_offset":28860,"parent_record_offset":28836,"ancestor_record_offset":26816,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1657,"record_offset":28368,"parent_record_offset":28344,"ancestor_record_offset":26816,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1657,"record_offset":28452,"parent_record_offset":28428,"ancestor_record_offset":26816,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1657,"record_offset":28392,"parent_record_offset":28368,"ancestor_record_offset":26816,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1657,"record_offset":28928,"parent_record_offset":28284,"ancestor_record_offset":26816,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1657,"record_offset":27668,"parent_record_offset":27636,"ancestor_record_offset":26816,"inlinee":6091,"source_line":73,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1657,"record_offset":27972,"parent_record_offset":27700,"ancestor_record_offset":26816,"inlinee":6094,"source_line":85,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1657,"record_offset":27572,"parent_record_offset":27540,"ancestor_record_offset":26816,"inlinee":4465,"source_line":97,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1657,"record_offset":27540,"parent_record_offset":27468,"ancestor_record_offset":26816,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1657,"record_offset":27636,"parent_record_offset":27604,"ancestor_record_offset":26816,"inlinee":5946,"source_line":115,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_try_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1657,"record_offset":28012,"parent_record_offset":27972,"ancestor_record_offset":26816,"inlinee":10987,"source_line":95,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1657,"record_offset":27512,"parent_record_offset":27468,"ancestor_record_offset":26816,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1657,"record_offset":27604,"parent_record_offset":27572,"ancestor_record_offset":26816,"inlinee":5945,"source_line":2821,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1657,"record_offset":28044,"parent_record_offset":28012,"ancestor_record_offset":26816,"inlinee":8146,"source_line":2828,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1657,"record_offset":27700,"parent_record_offset":27668,"ancestor_record_offset":26816,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1657,"record_offset":29900,"parent_record_offset":29876,"ancestor_record_offset":26816,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1657,"record_offset":28600,"parent_record_offset":28572,"ancestor_record_offset":26816,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1657,"record_offset":28572,"parent_record_offset":28544,"ancestor_record_offset":26816,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::ControlFlow::break_value
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\control_flow.rs
//     evidence={"module":1657,"record_offset":29220,"parent_record_offset":27604,"ancestor_record_offset":26816,"inlinee":14462,"source_line":182,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1657,"record_offset":28076,"parent_record_offset":28044,"ancestor_record_offset":26816,"inlinee":4194,"source_line":293,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1657,"record_offset":29256,"parent_record_offset":27540,"ancestor_record_offset":26816,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":29588,"parent_record_offset":27292,"ancestor_record_offset":26816,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":31292,"parent_record_offset":31264,"ancestor_record_offset":26816,"inlinee":7359,"source_line":2074,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1657,"record_offset":29824,"parent_record_offset":29728,"ancestor_record_offset":26816,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1657,"record_offset":28720,"parent_record_offset":28696,"ancestor_record_offset":26816,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1657,"record_offset":27928,"parent_record_offset":27900,"ancestor_record_offset":26816,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1657,"record_offset":27828,"parent_record_offset":27752,"ancestor_record_offset":26816,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":29924,"parent_record_offset":29900,"ancestor_record_offset":26816,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1657,"record_offset":27260,"parent_record_offset":27236,"ancestor_record_offset":26816,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1657,"record_offset":27900,"parent_record_offset":27872,"ancestor_record_offset":26816,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1657,"record_offset":27752,"parent_record_offset":27700,"ancestor_record_offset":26816,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1657,"record_offset":27872,"parent_record_offset":27752,"ancestor_record_offset":26816,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1657,"record_offset":29532,"parent_record_offset":27292,"ancestor_record_offset":26816,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1657,"record_offset":29556,"parent_record_offset":29532,"ancestor_record_offset":26816,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1657,"record_offset":28512,"parent_record_offset":28484,"ancestor_record_offset":26816,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1657,"record_offset":28544,"parent_record_offset":28512,"ancestor_record_offset":26816,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1657,"record_offset":28484,"parent_record_offset":28284,"ancestor_record_offset":26816,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1657,"record_offset":28344,"parent_record_offset":28284,"ancestor_record_offset":26816,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1657,"record_offset":28956,"parent_record_offset":28284,"ancestor_record_offset":26816,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1657,"record_offset":28428,"parent_record_offset":28284,"ancestor_record_offset":26816,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1657,"record_offset":29108,"parent_record_offset":28284,"ancestor_record_offset":26816,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1657,"record_offset":28836,"parent_record_offset":28812,"ancestor_record_offset":26816,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1657,"record_offset":28160,"parent_record_offset":28132,"ancestor_record_offset":26816,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1657,"record_offset":28188,"parent_record_offset":28160,"ancestor_record_offset":26816,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1657,"record_offset":28812,"parent_record_offset":28644,"ancestor_record_offset":26816,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1657,"record_offset":29148,"parent_record_offset":28188,"ancestor_record_offset":26816,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":28752,"parent_record_offset":28668,"ancestor_record_offset":26816,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":28696,"parent_record_offset":28668,"ancestor_record_offset":26816,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":29048,"parent_record_offset":28284,"ancestor_record_offset":26816,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":28668,"parent_record_offset":28644,"ancestor_record_offset":26816,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":28256,"parent_record_offset":28228,"ancestor_record_offset":26816,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":28228,"parent_record_offset":28188,"ancestor_record_offset":26816,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":28284,"parent_record_offset":28256,"ancestor_record_offset":26816,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":29080,"parent_record_offset":28284,"ancestor_record_offset":26816,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":28644,"parent_record_offset":28284,"ancestor_record_offset":26816,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::binding::impl$0::bind_descriptor_set::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/binding.rs
//     evidence={"module":1657,"record_offset":28104,"parent_record_offset":28076,"ancestor_record_offset":26816,"inlinee":32916,"source_line":76,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$58::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1657,"record_offset":28884,"parent_record_offset":28860,"ancestor_record_offset":26816,"inlinee":11982,"source_line":69,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1657,"record_offset":28132,"parent_record_offset":28104,"ancestor_record_offset":26816,"inlinee":19952,"source_line":1125,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1657,"record_offset":22248,"rva":23745408,"va":5392454528}
// 编译实例 RVAs: 0x16a5380
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405b0ed0
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17778439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392454528,"raw_hex":"e8f9b8f0fe","size":5,"soft_fail":false,"target_va":5374676688,"va":5392455122,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e6620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17560267","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392454528,"raw_hex":"e8350df4fe","size":5,"soft_fail":false,"target_va":5374895648,"va":5392455910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17374100","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392454528,"raw_hex":"e86ce4f6fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5392455599,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b06020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12187212","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392454528,"raw_hex":"e8b40946ff","size":5,"soft_fail":false,"target_va":5380268064,"va":5392455271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::Bump::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-9809977","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392454528,"raw_hex":"e8c74f6aff","size":5,"soft_fail":false,"target_va":5382644656,"va":5392454628,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::collections::raw_vec::RawVec$LT$T$GT$::allocate_in
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1185166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392454528,"raw_hex":"e88e151200","size":5,"soft_fail":false,"target_va":5393639872,"va":5392454701,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::collections::raw_vec::RawVec$LT$T$GT$::reserve_internal_or_panic
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1185147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392454528,"raw_hex":"e87b151200","size":5,"soft_fail":false,"target_va":5393640464,"va":5392455312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::shader::DescriptorSetState::allocate_descriptor_set
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11270588","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392454528,"raw_hex":"e8440654ff","size":5,"soft_fail":false,"target_va":5381184064,"va":5392454647,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*616(%r14)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5392454528,"raw_hex":"41ff9668020000","size":7,"soft_fail":false,"va":5392455389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::cmd_bind_descriptor_sets
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1657,"record_offset":26184,"parent_record_offset":22248,"ancestor_record_offset":22248,"inlinee":39591,"source_line":1761,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":26212,"parent_record_offset":22248,"ancestor_record_offset":22248,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":1657,"record_offset":22652,"parent_record_offset":22248,"ancestor_record_offset":22248,"inlinee":4143,"source_line":594,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::update_descriptor_set
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/binding.rs
//     evidence={"module":1657,"record_offset":22708,"parent_record_offset":22248,"ancestor_record_offset":22248,"inlinee":75154,"source_line":96,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1657,"record_offset":26772,"parent_record_offset":26644,"ancestor_record_offset":22248,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::update_descriptor_sets
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1657,"record_offset":24920,"parent_record_offset":22708,"ancestor_record_offset":22248,"inlinee":75157,"source_line":1356,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::dealloc
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1657,"record_offset":25144,"parent_record_offset":25076,"ancestor_record_offset":22248,"inlinee":19984,"source_line":2228,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::is_last_allocation
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1657,"record_offset":25176,"parent_record_offset":25144,"ancestor_record_offset":22248,"inlinee":19985,"source_line":2221,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::ChunkFooter::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1657,"record_offset":26680,"parent_record_offset":26644,"ancestor_record_offset":22248,"inlinee":12170,"source_line":375,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::current_layout
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":25108,"parent_record_offset":25076,"ancestor_record_offset":22248,"inlinee":19983,"source_line":158,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::dealloc_buffer
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":25076,"parent_record_offset":25052,"ancestor_record_offset":22248,"inlinee":19982,"source_line":701,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::infallible_reserve_internal
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":24876,"parent_record_offset":24852,"ancestor_record_offset":22248,"inlinee":19975,"source_line":607,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":24780,"parent_record_offset":24744,"ancestor_record_offset":22248,"inlinee":19972,"source_line":137,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::reserve
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":24852,"parent_record_offset":24828,"ancestor_record_offset":22248,"inlinee":19974,"source_line":429,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::with_capacity_in
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":22792,"parent_record_offset":22764,"ancestor_record_offset":22248,"inlinee":75156,"source_line":74,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::impl$5::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":25052,"parent_record_offset":25028,"ancestor_record_offset":22248,"inlinee":19981,"source_line":713,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":1657,"record_offset":24704,"parent_record_offset":22884,"ancestor_record_offset":22248,"inlinee":19969,"source_line":1451,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::push_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":1657,"record_offset":24744,"parent_record_offset":24704,"ancestor_record_offset":22248,"inlinee":19970,"source_line":1422,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::reserve
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":1657,"record_offset":24828,"parent_record_offset":24704,"ancestor_record_offset":22248,"inlinee":19973,"source_line":736,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::with_capacity_in
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":1657,"record_offset":22764,"parent_record_offset":22708,"ancestor_record_offset":22248,"inlinee":75155,"source_line":585,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::impl$17::extend
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":1657,"record_offset":22884,"parent_record_offset":22708,"ancestor_record_offset":22248,"inlinee":44844,"source_line":2311,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::dealloc_chunk_list
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1657,"record_offset":26644,"parent_record_offset":26620,"ancestor_record_offset":22248,"inlinee":12168,"source_line":395,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$6::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1657,"record_offset":26620,"parent_record_offset":26596,"ancestor_record_offset":22248,"inlinee":16119,"source_line":387,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1657,"record_offset":25200,"parent_record_offset":25176,"ancestor_record_offset":22248,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1657,"record_offset":25292,"parent_record_offset":25268,"ancestor_record_offset":22248,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1657,"record_offset":25268,"parent_record_offset":25144,"ancestor_record_offset":22248,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1657,"record_offset":24276,"parent_record_offset":24252,"ancestor_record_offset":22248,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1657,"record_offset":23784,"parent_record_offset":23760,"ancestor_record_offset":22248,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1657,"record_offset":23868,"parent_record_offset":23844,"ancestor_record_offset":22248,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1657,"record_offset":23808,"parent_record_offset":23784,"ancestor_record_offset":22248,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1657,"record_offset":24344,"parent_record_offset":23700,"ancestor_record_offset":22248,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1657,"record_offset":23084,"parent_record_offset":23052,"ancestor_record_offset":22248,"inlinee":6091,"source_line":73,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1657,"record_offset":23388,"parent_record_offset":23116,"ancestor_record_offset":22248,"inlinee":6094,"source_line":85,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1657,"record_offset":22988,"parent_record_offset":22956,"ancestor_record_offset":22248,"inlinee":4465,"source_line":97,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1657,"record_offset":22956,"parent_record_offset":22884,"ancestor_record_offset":22248,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1657,"record_offset":23052,"parent_record_offset":23020,"ancestor_record_offset":22248,"inlinee":5946,"source_line":115,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_try_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1657,"record_offset":23428,"parent_record_offset":23388,"ancestor_record_offset":22248,"inlinee":10987,"source_line":95,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1657,"record_offset":22928,"parent_record_offset":22884,"ancestor_record_offset":22248,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1657,"record_offset":23020,"parent_record_offset":22988,"ancestor_record_offset":22248,"inlinee":5945,"source_line":2821,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1657,"record_offset":23460,"parent_record_offset":23428,"ancestor_record_offset":22248,"inlinee":8146,"source_line":2828,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1657,"record_offset":23116,"parent_record_offset":23084,"ancestor_record_offset":22248,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1657,"record_offset":25316,"parent_record_offset":25292,"ancestor_record_offset":22248,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1657,"record_offset":24016,"parent_record_offset":23988,"ancestor_record_offset":22248,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1657,"record_offset":23988,"parent_record_offset":23960,"ancestor_record_offset":22248,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::ControlFlow::break_value
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\control_flow.rs
//     evidence={"module":1657,"record_offset":24636,"parent_record_offset":23020,"ancestor_record_offset":22248,"inlinee":14462,"source_line":182,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1657,"record_offset":23492,"parent_record_offset":23460,"ancestor_record_offset":22248,"inlinee":4194,"source_line":293,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1657,"record_offset":24672,"parent_record_offset":22956,"ancestor_record_offset":22248,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":25004,"parent_record_offset":22708,"ancestor_record_offset":22248,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":26708,"parent_record_offset":26680,"ancestor_record_offset":22248,"inlinee":7359,"source_line":2074,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1657,"record_offset":25240,"parent_record_offset":25144,"ancestor_record_offset":22248,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1657,"record_offset":24136,"parent_record_offset":24112,"ancestor_record_offset":22248,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1657,"record_offset":23344,"parent_record_offset":23316,"ancestor_record_offset":22248,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1657,"record_offset":23244,"parent_record_offset":23168,"ancestor_record_offset":22248,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":25340,"parent_record_offset":25316,"ancestor_record_offset":22248,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1657,"record_offset":22676,"parent_record_offset":22652,"ancestor_record_offset":22248,"inlinee":4145,"source_line":226,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1657,"record_offset":23316,"parent_record_offset":23288,"ancestor_record_offset":22248,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1657,"record_offset":23168,"parent_record_offset":23116,"ancestor_record_offset":22248,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1657,"record_offset":23288,"parent_record_offset":23168,"ancestor_record_offset":22248,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1657,"record_offset":24948,"parent_record_offset":22708,"ancestor_record_offset":22248,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1657,"record_offset":24972,"parent_record_offset":24948,"ancestor_record_offset":22248,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1657,"record_offset":23928,"parent_record_offset":23900,"ancestor_record_offset":22248,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1657,"record_offset":23960,"parent_record_offset":23928,"ancestor_record_offset":22248,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1657,"record_offset":23900,"parent_record_offset":23700,"ancestor_record_offset":22248,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1657,"record_offset":23760,"parent_record_offset":23700,"ancestor_record_offset":22248,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1657,"record_offset":24372,"parent_record_offset":23700,"ancestor_record_offset":22248,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1657,"record_offset":23844,"parent_record_offset":23700,"ancestor_record_offset":22248,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1657,"record_offset":24524,"parent_record_offset":23700,"ancestor_record_offset":22248,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1657,"record_offset":24252,"parent_record_offset":24228,"ancestor_record_offset":22248,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1657,"record_offset":23576,"parent_record_offset":23548,"ancestor_record_offset":22248,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1657,"record_offset":23604,"parent_record_offset":23576,"ancestor_record_offset":22248,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1657,"record_offset":24228,"parent_record_offset":24060,"ancestor_record_offset":22248,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1657,"record_offset":24564,"parent_record_offset":23604,"ancestor_record_offset":22248,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":24168,"parent_record_offset":24084,"ancestor_record_offset":22248,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":24112,"parent_record_offset":24084,"ancestor_record_offset":22248,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":24464,"parent_record_offset":23700,"ancestor_record_offset":22248,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":24084,"parent_record_offset":24060,"ancestor_record_offset":22248,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":23672,"parent_record_offset":23644,"ancestor_record_offset":22248,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":23644,"parent_record_offset":23604,"ancestor_record_offset":22248,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":23700,"parent_record_offset":23672,"ancestor_record_offset":22248,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":24496,"parent_record_offset":23700,"ancestor_record_offset":22248,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":24060,"parent_record_offset":23700,"ancestor_record_offset":22248,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::binding::impl$0::bind_descriptor_set::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/binding.rs
//     evidence={"module":1657,"record_offset":23520,"parent_record_offset":23492,"ancestor_record_offset":22248,"inlinee":32916,"source_line":76,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$58::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1657,"record_offset":24300,"parent_record_offset":24276,"ancestor_record_offset":22248,"inlinee":11982,"source_line":69,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1657,"record_offset":23548,"parent_record_offset":23520,"ancestor_record_offset":22248,"inlinee":19952,"source_line":1125,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::binding::impl$0::bind_descriptor_set::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 76
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8094
// 首条位置: {"module":576,"inlinee":32916}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":576,"record_offset":1816,"parent_record_offset":1792,"ancestor_record_offset":1792,"inlinee":19952,"source_line":1125,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":576,"record_offset":2592,"parent_record_offset":2568,"ancestor_record_offset":1792,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":576,"record_offset":2104,"parent_record_offset":2080,"ancestor_record_offset":1792,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":576,"record_offset":2188,"parent_record_offset":2164,"ancestor_record_offset":1792,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":576,"record_offset":2128,"parent_record_offset":2104,"ancestor_record_offset":1792,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":576,"record_offset":2660,"parent_record_offset":1972,"ancestor_record_offset":1792,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":576,"record_offset":2336,"parent_record_offset":2308,"ancestor_record_offset":1792,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":576,"record_offset":2308,"parent_record_offset":2280,"ancestor_record_offset":1792,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":576,"record_offset":2452,"parent_record_offset":2428,"ancestor_record_offset":1792,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":576,"record_offset":2248,"parent_record_offset":2220,"ancestor_record_offset":1792,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1610,"record_offset":113180,"parent_record_offset":113152,"ancestor_record_offset":112764,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":576,"record_offset":2280,"parent_record_offset":2248,"ancestor_record_offset":1792,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1610,"record_offset":113212,"parent_record_offset":113180,"ancestor_record_offset":112764,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/bitmask.rs
//     evidence={"module":576,"record_offset":2220,"parent_record_offset":1972,"ancestor_record_offset":1792,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1610,"record_offset":113152,"parent_record_offset":112956,"ancestor_record_offset":112764,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":576,"record_offset":2080,"parent_record_offset":1972,"ancestor_record_offset":1792,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1610,"record_offset":113012,"parent_record_offset":112956,"ancestor_record_offset":112764,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":576,"record_offset":2688,"parent_record_offset":1972,"ancestor_record_offset":1792,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1610,"record_offset":113624,"parent_record_offset":112956,"ancestor_record_offset":112764,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/group/sse2.rs
//     evidence={"module":576,"record_offset":2164,"parent_record_offset":1972,"ancestor_record_offset":1792,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1610,"record_offset":113096,"parent_record_offset":112956,"ancestor_record_offset":112764,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/control/tag.rs
//     evidence={"module":576,"record_offset":2052,"parent_record_offset":1972,"ancestor_record_offset":1792,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1610,"record_offset":113776,"parent_record_offset":112956,"ancestor_record_offset":112764,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":576,"record_offset":2568,"parent_record_offset":2544,"ancestor_record_offset":1792,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":576,"record_offset":1840,"parent_record_offset":1816,"ancestor_record_offset":1792,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":576,"record_offset":1864,"parent_record_offset":1840,"ancestor_record_offset":1792,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1610,"record_offset":112860,"parent_record_offset":112828,"ancestor_record_offset":112764,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/map.rs
//     evidence={"module":576,"record_offset":2544,"parent_record_offset":2380,"ancestor_record_offset":1792,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1610,"record_offset":113480,"parent_record_offset":113312,"ancestor_record_offset":112764,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":576,"record_offset":1896,"parent_record_offset":1864,"ancestor_record_offset":1792,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":576,"record_offset":2484,"parent_record_offset":2400,"ancestor_record_offset":1792,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":576,"record_offset":2428,"parent_record_offset":2400,"ancestor_record_offset":1792,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1610,"record_offset":113364,"parent_record_offset":113336,"ancestor_record_offset":112764,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":576,"record_offset":2780,"parent_record_offset":1972,"ancestor_record_offset":1792,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1610,"record_offset":113716,"parent_record_offset":112956,"ancestor_record_offset":112764,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":576,"record_offset":2400,"parent_record_offset":2380,"ancestor_record_offset":1792,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1610,"record_offset":113336,"parent_record_offset":113312,"ancestor_record_offset":112764,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":576,"record_offset":1948,"parent_record_offset":1924,"ancestor_record_offset":1792,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1610,"record_offset":112928,"parent_record_offset":112900,"ancestor_record_offset":112764,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":576,"record_offset":1924,"parent_record_offset":1864,"ancestor_record_offset":1792,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1610,"record_offset":112900,"parent_record_offset":112860,"ancestor_record_offset":112764,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":576,"record_offset":1972,"parent_record_offset":1948,"ancestor_record_offset":1792,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1610,"record_offset":112956,"parent_record_offset":112928,"ancestor_record_offset":112764,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":576,"record_offset":2024,"parent_record_offset":1972,"ancestor_record_offset":1792,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1610,"record_offset":113748,"parent_record_offset":112956,"ancestor_record_offset":112764,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.15.2/src/raw/mod.rs
//     evidence={"module":576,"record_offset":2380,"parent_record_offset":1972,"ancestor_record_offset":1792,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1610,"record_offset":113312,"parent_record_offset":112956,"ancestor_record_offset":112764,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$58::eq
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":576,"record_offset":2616,"parent_record_offset":2592,"ancestor_record_offset":1792,"inlinee":11982,"source_line":69,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::update_descriptor_set
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 96
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x12592
// 首条位置: {"module":1657,"inlinee":75154}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::update_descriptor_sets
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1657,"record_offset":3992,"parent_record_offset":2112,"ancestor_record_offset":2112,"inlinee":75157,"source_line":1356,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] bumpalo::collections::vec::Vec::with_capacity_in
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":1657,"record_offset":2180,"parent_record_offset":2112,"ancestor_record_offset":2112,"inlinee":75155,"source_line":585,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] bumpalo::collections::vec::impl$17::extend
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":1657,"record_offset":2304,"parent_record_offset":2112,"ancestor_record_offset":2112,"inlinee":44844,"source_line":2311,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":4076,"parent_record_offset":2112,"ancestor_record_offset":2112,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1657,"record_offset":4020,"parent_record_offset":2112,"ancestor_record_offset":2112,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1657,"record_offset":33680,"parent_record_offset":33656,"ancestor_record_offset":32264,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1657,"record_offset":33656,"parent_record_offset":33568,"ancestor_record_offset":32264,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":3664,"parent_record_offset":3632,"ancestor_record_offset":2112,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":33596,"parent_record_offset":33568,"ancestor_record_offset":32264,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":33568,"parent_record_offset":33544,"ancestor_record_offset":32264,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":3712,"parent_record_offset":3688,"ancestor_record_offset":2112,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":3688,"parent_record_offset":3664,"ancestor_record_offset":2112,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":33544,"parent_record_offset":33520,"ancestor_record_offset":32264,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1657,"record_offset":3632,"parent_record_offset":3588,"ancestor_record_offset":2112,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1657,"record_offset":3936,"parent_record_offset":3872,"ancestor_record_offset":2112,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1657,"record_offset":3808,"parent_record_offset":3588,"ancestor_record_offset":2112,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":1657,"record_offset":2768,"parent_record_offset":2740,"ancestor_record_offset":2112,"inlinee":8249,"source_line":156,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":1657,"record_offset":2984,"parent_record_offset":2960,"ancestor_record_offset":2112,"inlinee":8251,"source_line":157,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":1657,"record_offset":2452,"parent_record_offset":2424,"ancestor_record_offset":2112,"inlinee":14070,"source_line":160,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$7::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":1657,"record_offset":3388,"parent_record_offset":3364,"ancestor_record_offset":2112,"inlinee":5300,"source_line":175,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$7::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":1657,"record_offset":3588,"parent_record_offset":3552,"ancestor_record_offset":2112,"inlinee":5302,"source_line":180,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1657,"record_offset":33200,"parent_record_offset":33172,"ancestor_record_offset":32264,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1657,"record_offset":33448,"parent_record_offset":33424,"ancestor_record_offset":32264,"inlinee":4899,"source_line":487,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1657,"record_offset":33496,"parent_record_offset":33472,"ancestor_record_offset":32264,"inlinee":4901,"source_line":491,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$5::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1657,"record_offset":32968,"parent_record_offset":32936,"ancestor_record_offset":32264,"inlinee":11832,"source_line":324,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::dealloc
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1657,"record_offset":4216,"parent_record_offset":4148,"ancestor_record_offset":2112,"inlinee":19984,"source_line":2228,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::is_last_allocation
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1657,"record_offset":4248,"parent_record_offset":4216,"ancestor_record_offset":2112,"inlinee":19985,"source_line":2221,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::current_layout
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":4180,"parent_record_offset":4148,"ancestor_record_offset":2112,"inlinee":19983,"source_line":158,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::dealloc_buffer
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":4148,"parent_record_offset":4124,"ancestor_record_offset":2112,"inlinee":19982,"source_line":701,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::infallible_reserve_internal
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":2672,"parent_record_offset":2648,"ancestor_record_offset":2112,"inlinee":19975,"source_line":607,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::ptr
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":3180,"parent_record_offset":3152,"ancestor_record_offset":2112,"inlinee":19972,"source_line":137,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::reserve
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":2648,"parent_record_offset":2624,"ancestor_record_offset":2112,"inlinee":19974,"source_line":429,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::with_capacity_in
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":2208,"parent_record_offset":2180,"ancestor_record_offset":2112,"inlinee":75156,"source_line":74,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::impl$5::drop
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":4124,"parent_record_offset":4100,"ancestor_record_offset":2112,"inlinee":19981,"source_line":713,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::push
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":1657,"record_offset":3116,"parent_record_offset":2304,"ancestor_record_offset":2112,"inlinee":19969,"source_line":1451,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::push_unchecked
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":1657,"record_offset":3152,"parent_record_offset":3116,"ancestor_record_offset":2112,"inlinee":19970,"source_line":1422,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::reserve
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":1657,"record_offset":2624,"parent_record_offset":2304,"ancestor_record_offset":2112,"inlinee":19973,"source_line":736,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1657,"record_offset":4272,"parent_record_offset":4248,"ancestor_record_offset":2112,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1657,"record_offset":4364,"parent_record_offset":4340,"ancestor_record_offset":2112,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1657,"record_offset":4340,"parent_record_offset":4216,"ancestor_record_offset":2112,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1657,"record_offset":33004,"parent_record_offset":32968,"ancestor_record_offset":32264,"inlinee":4974,"source_line":261,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":1657,"record_offset":24276,"parent_record_offset":24252,"ancestor_record_offset":22708,"inlinee":4448,"source_line":1921,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1657,"record_offset":23784,"parent_record_offset":23760,"ancestor_record_offset":22708,"inlinee":4484,"source_line":1227,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1657,"record_offset":23868,"parent_record_offset":23844,"ancestor_record_offset":22708,"inlinee":4486,"source_line":1433,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1657,"record_offset":3780,"parent_record_offset":3588,"ancestor_record_offset":2112,"inlinee":5303,"source_line":4520,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1657,"record_offset":23808,"parent_record_offset":23784,"ancestor_record_offset":22708,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1657,"record_offset":24344,"parent_record_offset":23700,"ancestor_record_offset":22708,"inlinee":4502,"source_line":1497,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1657,"record_offset":32812,"parent_record_offset":32776,"ancestor_record_offset":32264,"inlinee":5541,"source_line":332,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::find
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1657,"record_offset":32776,"parent_record_offset":32744,"ancestor_record_offset":32264,"inlinee":47224,"source_line":167,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::find::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1657,"record_offset":32876,"parent_record_offset":32812,"ancestor_record_offset":32264,"inlinee":47226,"source_line":171,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::find::closure$1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1657,"record_offset":33776,"parent_record_offset":33752,"ancestor_record_offset":32264,"inlinee":47229,"source_line":172,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1657,"record_offset":23084,"parent_record_offset":23052,"ancestor_record_offset":22708,"inlinee":6091,"source_line":73,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold::enumerate::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1657,"record_offset":23388,"parent_record_offset":23116,"ancestor_record_offset":22708,"inlinee":6094,"source_line":85,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$3::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1657,"record_offset":22988,"parent_record_offset":22956,"ancestor_record_offset":22708,"inlinee":4465,"source_line":97,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1657,"record_offset":2712,"parent_record_offset":2304,"ancestor_record_offset":2112,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1657,"record_offset":2396,"parent_record_offset":2304,"ancestor_record_offset":2112,"inlinee":8256,"source_line":111,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::try_fold
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1657,"record_offset":23052,"parent_record_offset":23020,"ancestor_record_offset":22708,"inlinee":5946,"source_line":115,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_try_fold::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1657,"record_offset":23428,"parent_record_offset":23388,"ancestor_record_offset":22708,"inlinee":10987,"source_line":95,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1657,"record_offset":2368,"parent_record_offset":2304,"ancestor_record_offset":2112,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1657,"record_offset":23020,"parent_record_offset":22988,"ancestor_record_offset":22708,"inlinee":5945,"source_line":2821,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1657,"record_offset":23460,"parent_record_offset":23428,"ancestor_record_offset":22708,"inlinee":8146,"source_line":2828,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1657,"record_offset":23116,"parent_record_offset":23084,"ancestor_record_offset":22708,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1657,"record_offset":3460,"parent_record_offset":3432,"ancestor_record_offset":2112,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1657,"record_offset":3432,"parent_record_offset":3388,"ancestor_record_offset":2112,"inlinee":5313,"source_line":793,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1657,"record_offset":33624,"parent_record_offset":33596,"ancestor_record_offset":32264,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1657,"record_offset":24016,"parent_record_offset":23988,"ancestor_record_offset":22708,"inlinee":4503,"source_line":449,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\nonzero.rs
//     evidence={"module":1657,"record_offset":23988,"parent_record_offset":23960,"ancestor_record_offset":22708,"inlinee":4493,"source_line":601,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::ControlFlow::break_value
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\control_flow.rs
//     evidence={"module":1657,"record_offset":24636,"parent_record_offset":23020,"ancestor_record_offset":22708,"inlinee":14462,"source_line":182,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::impl$0::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\control_flow.rs
//     evidence={"module":1657,"record_offset":33332,"parent_record_offset":32968,"ancestor_record_offset":32264,"inlinee":10993,"source_line":111,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":1657,"record_offset":23492,"parent_record_offset":23460,"ancestor_record_offset":22708,"inlinee":4194,"source_line":293,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1657,"record_offset":32848,"parent_record_offset":32812,"ancestor_record_offset":32264,"inlinee":4130,"source_line":728,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1657,"record_offset":2960,"parent_record_offset":2768,"ancestor_record_offset":2112,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1657,"record_offset":33752,"parent_record_offset":32776,"ancestor_record_offset":32264,"inlinee":4571,"source_line":1546,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1657,"record_offset":2568,"parent_record_offset":2540,"ancestor_record_offset":2112,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=74
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":3316,"parent_record_offset":2304,"ancestor_record_offset":2112,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1657,"record_offset":3752,"parent_record_offset":3588,"ancestor_record_offset":2112,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1657,"record_offset":24136,"parent_record_offset":24112,"ancestor_record_offset":22708,"inlinee":4723,"source_line":1129,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1657,"record_offset":2540,"parent_record_offset":2512,"ancestor_record_offset":2112,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1657,"record_offset":2920,"parent_record_offset":2896,"ancestor_record_offset":2112,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1657,"record_offset":3908,"parent_record_offset":3872,"ancestor_record_offset":2112,"inlinee":5530,"source_line":463,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1657,"record_offset":33060,"parent_record_offset":32968,"ancestor_record_offset":32264,"inlinee":4922,"source_line":945,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1657,"record_offset":2512,"parent_record_offset":2480,"ancestor_record_offset":2112,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1657,"record_offset":2840,"parent_record_offset":2796,"ancestor_record_offset":2112,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":3008,"parent_record_offset":2984,"ancestor_record_offset":2112,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":3488,"parent_record_offset":3460,"ancestor_record_offset":2112,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1657,"record_offset":33228,"parent_record_offset":33200,"ancestor_record_offset":32264,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1657,"record_offset":33256,"parent_record_offset":33228,"ancestor_record_offset":32264,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1657,"record_offset":2896,"parent_record_offset":2872,"ancestor_record_offset":2112,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1657,"record_offset":2796,"parent_record_offset":2768,"ancestor_record_offset":2112,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1657,"record_offset":2480,"parent_record_offset":2452,"ancestor_record_offset":2112,"inlinee":8257,"source_line":171,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1657,"record_offset":2872,"parent_record_offset":2796,"ancestor_record_offset":2112,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1657,"record_offset":4044,"parent_record_offset":4020,"ancestor_record_offset":2112,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":1657,"record_offset":39380,"parent_record_offset":39352,"ancestor_record_offset":38472,"inlinee":74171,"source_line":540,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure$3
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/draw_stream.rs
//     evidence={"module":1657,"record_offset":33172,"parent_record_offset":33144,"ancestor_record_offset":32264,"inlinee":74167,"source_line":585,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1657,"record_offset":23928,"parent_record_offset":23900,"ancestor_record_offset":22708,"inlinee":4490,"source_line":49,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1657,"record_offset":23960,"parent_record_offset":23928,"ancestor_record_offset":22708,"inlinee":4491,"source_line":74,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\bitmask.rs
//     evidence={"module":1657,"record_offset":23900,"parent_record_offset":23700,"ancestor_record_offset":22708,"inlinee":4488,"source_line":112,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1657,"record_offset":23760,"parent_record_offset":23700,"ancestor_record_offset":22708,"inlinee":4482,"source_line":51,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1657,"record_offset":24372,"parent_record_offset":23700,"ancestor_record_offset":22708,"inlinee":4504,"source_line":94,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\group\sse2.rs
//     evidence={"module":1657,"record_offset":23844,"parent_record_offset":23700,"ancestor_record_offset":22708,"inlinee":4485,"source_line":76,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\control\tag.rs
//     evidence={"module":1657,"record_offset":24524,"parent_record_offset":23700,"ancestor_record_offset":22708,"inlinee":4517,"source_line":36,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\lib.rs
//     evidence={"module":1657,"record_offset":24252,"parent_record_offset":24228,"ancestor_record_offset":22708,"inlinee":4499,"source_line":164,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1657,"record_offset":23576,"parent_record_offset":23548,"ancestor_record_offset":22708,"inlinee":4474,"source_line":1414,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1657,"record_offset":23604,"parent_record_offset":23576,"ancestor_record_offset":22708,"inlinee":4475,"source_line":1347,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1657,"record_offset":24228,"parent_record_offset":24060,"ancestor_record_offset":22708,"inlinee":4497,"source_line":224,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\map.rs
//     evidence={"module":1657,"record_offset":24564,"parent_record_offset":23604,"ancestor_record_offset":22708,"inlinee":4514,"source_line":253,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":24168,"parent_record_offset":24084,"ancestor_record_offset":22708,"inlinee":4722,"source_line":384,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":24112,"parent_record_offset":24084,"ancestor_record_offset":22708,"inlinee":5616,"source_line":261,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":24464,"parent_record_offset":23700,"ancestor_record_offset":22708,"inlinee":4506,"source_line":81,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":24084,"parent_record_offset":24060,"ancestor_record_offset":22708,"inlinee":5615,"source_line":734,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":23672,"parent_record_offset":23644,"ancestor_record_offset":22708,"inlinee":4478,"source_line":1156,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":23644,"parent_record_offset":23604,"ancestor_record_offset":22708,"inlinee":4477,"source_line":1179,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":23700,"parent_record_offset":23672,"ancestor_record_offset":22708,"inlinee":4480,"source_line":1862,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":24496,"parent_record_offset":23700,"ancestor_record_offset":22708,"inlinee":4515,"source_line":2301,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.15.2\src\raw\mod.rs
//     evidence={"module":1657,"record_offset":24060,"parent_record_offset":23700,"ancestor_record_offset":22708,"inlinee":4495,"source_line":1166,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::binding::impl$0::bind_descriptor_set::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/binding.rs
//     evidence={"module":1657,"record_offset":23520,"parent_record_offset":23492,"ancestor_record_offset":22708,"inlinee":32916,"source_line":76,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$58::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"shader-pipeline","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1657,"record_offset":24300,"parent_record_offset":24276,"ancestor_record_offset":22708,"inlinee":11982,"source_line":69,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\collections\hash\map.rs
//     evidence={"module":1657,"record_offset":23548,"parent_record_offset":23520,"ancestor_record_offset":22708,"inlinee":19952,"source_line":1125,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::binding::impl$0::update_descriptor_set::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 109
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8097
// 首条位置: {"module":576,"inlinee":32919}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::definitions::WriteDescriptorSet::buffer_info
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":576,"record_offset":5796,"parent_record_offset":3824,"ancestor_record_offset":3824,"inlinee":32921,"source_line":1993,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::WriteDescriptorSet::image_info
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":576,"record_offset":4508,"parent_record_offset":3824,"ancestor_record_offset":3824,"inlinee":32920,"source_line":1987,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::WriteDescriptorSet::push_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":576,"record_offset":6400,"parent_record_offset":3824,"ancestor_record_offset":3824,"inlinee":27928,"source_line":2009,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] bumpalo::Bump::alloc
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":576,"record_offset":3992,"parent_record_offset":3824,"ancestor_record_offset":3824,"inlinee":15485,"source_line":1028,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_direct] bumpalo::collections::vec::Vec::push
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":576,"record_offset":5088,"parent_record_offset":3824,"ancestor_record_offset":3824,"inlinee":19969,"source_line":1451,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::ptr_chain_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":576,"record_offset":6548,"parent_record_offset":6524,"ancestor_record_offset":3824,"inlinee":27900,"source_line":39,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_layout
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":576,"record_offset":4056,"parent_record_offset":4020,"ancestor_record_offset":3824,"inlinee":15487,"source_line":1864,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_with
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":576,"record_offset":4020,"parent_record_offset":3992,"ancestor_record_offset":3824,"inlinee":15486,"source_line":1071,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":576,"record_offset":4084,"parent_record_offset":4056,"ancestor_record_offset":3824,"inlinee":15488,"source_line":1879,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":576,"record_offset":4116,"parent_record_offset":4084,"ancestor_record_offset":3824,"inlinee":15489,"source_line":1888,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::infallible_reserve_internal
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":576,"record_offset":5168,"parent_record_offset":5144,"ancestor_record_offset":3824,"inlinee":19975,"source_line":607,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":576,"record_offset":5232,"parent_record_offset":5204,"ancestor_record_offset":3824,"inlinee":19972,"source_line":137,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::reserve
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":576,"record_offset":5144,"parent_record_offset":5120,"ancestor_record_offset":3824,"inlinee":19974,"source_line":429,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::push_unchecked
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":576,"record_offset":5204,"parent_record_offset":5088,"ancestor_record_offset":3824,"inlinee":19970,"source_line":1422,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::reserve
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":576,"record_offset":5120,"parent_record_offset":5088,"ancestor_record_offset":3824,"inlinee":19973,"source_line":736,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_with::inner_writer
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":576,"record_offset":4444,"parent_record_offset":4020,"ancestor_record_offset":3824,"inlinee":15492,"source_line":1076,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::round_mut_ptr_down_to
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":576,"record_offset":4208,"parent_record_offset":4116,"ancestor_record_offset":3824,"inlinee":15490,"source_line":450,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":576,"record_offset":4152,"parent_record_offset":4116,"ancestor_record_offset":3824,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":576,"record_offset":4316,"parent_record_offset":4292,"ancestor_record_offset":3824,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":576,"record_offset":4292,"parent_record_offset":4116,"ancestor_record_offset":3824,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\mod.rs
//     evidence={"module":576,"record_offset":6452,"parent_record_offset":6428,"ancestor_record_offset":3824,"inlinee":27895,"source_line":372,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":576,"record_offset":6476,"parent_record_offset":6452,"ancestor_record_offset":3824,"inlinee":27896,"source_line":57,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold::scan::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":576,"record_offset":6524,"parent_record_offset":6500,"ancestor_record_offset":3824,"inlinee":27898,"source_line":68,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":576,"record_offset":6428,"parent_record_offset":6400,"ancestor_record_offset":3824,"inlinee":11662,"source_line":249,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":576,"record_offset":6500,"parent_record_offset":6476,"ancestor_record_offset":3824,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":576,"record_offset":4340,"parent_record_offset":4316,"ancestor_record_offset":3824,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":576,"record_offset":4408,"parent_record_offset":4084,"ancestor_record_offset":3824,"inlinee":9461,"source_line":1252,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":576,"record_offset":4256,"parent_record_offset":4232,"ancestor_record_offset":3824,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":576,"record_offset":4232,"parent_record_offset":4208,"ancestor_record_offset":3824,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":576,"record_offset":4364,"parent_record_offset":4340,"ancestor_record_offset":3824,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00576.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::bind_persistent_descriptor_set
// 记录形式: compiled_procedure; instances=1
// 源码行: 183
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":7860,"rva":12509520,"va":5381218640}
// 编译实例 RVAs: 0xbee150
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1418aee50
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13372628","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381218640,"raw_hex":"e8d40ccc00","size":5,"soft_fail":false,"target_va":5394591312,"va":5381218679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30322970","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381218640,"raw_hex":"e81ab1ce01","size":5,"soft_fail":false,"target_va":5411541776,"va":5381218801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30323270","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381218640,"raw_hex":"e846b2ce01","size":5,"soft_fail":false,"target_va":5411542064,"va":5381218789,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*808(%rdi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381218640,"raw_hex":"ff9728030000","size":6,"soft_fail":false,"va":5381218753,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::cmd_bind_descriptor_sets
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":703,"record_offset":8092,"parent_record_offset":7860,"ancestor_record_offset":7860,"inlinee":39591,"source_line":1761,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":8024,"parent_record_offset":7860,"ancestor_record_offset":7860,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::command_buffer::CommandBuffer::raw
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":703,"record_offset":8056,"parent_record_offset":7860,"ancestor_record_offset":7860,"inlinee":39590,"source_line":179,"archive":"_pdb/B/modules/00703.jsonl.gz"}
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
// 完整结构及原始类型记录: binding.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::binding::DescriptorSetBinding
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 53
// [PDB 泛型关系，非字段关系] {"type": "enum2$<rhapsody::raw::binding::DescriptorSetBinding>", "constructor": "enum2$", "arguments": ["rhapsody::raw::binding::DescriptorSetBinding"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>", "constructor": "rhapsody::raw::managed::handle::GpuOwned", "arguments": ["enum2$<rhapsody::raw::binding::DescriptorSetBinding>", "rhapsody::raw::managed::handle::RawManagedHandle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >", "constructor": "tuple$", "arguments": ["shader_pipeline::reflection::BindingIndex", "rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> >", "constructor": "core::slice::iter::Iter", "arguments": ["enum2$<rhapsody::raw::binding::DescriptorSetBinding>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::cloned::impl$8::next_unchecked<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> >,enum2$<rhapsody::raw::binding::DescriptorSetBinding> >", "constructor": "core::iter::adapters::cloned::impl$8::next_unchecked", "arguments": ["core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> >", "enum2$<rhapsody::raw::binding::DescriptorSetBinding>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "array$<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,2>", "constructor": "array$", "arguments": ["enum2$<rhapsody::raw::binding::DescriptorSetBinding>", "2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::handle::GpuOwned<array$<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,2>,rhapsody::raw::managed::handle::RawManagedHandle>", "constructor": "rhapsody::raw::managed::handle::GpuOwned", "arguments": ["array$<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,2>", "rhapsody::raw::managed::handle::RawManagedHandle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::managed::handle::GpuOwned<array$<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,2>,rhapsody::raw::managed::handle::RawManagedHandle> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::managed::handle::GpuOwned<array$<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,2>,rhapsody::raw::managed::handle::RawManagedHandle>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::handle::impl$21::clone<array$<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,2>,rhapsody::raw::managed::handle::RawManagedHandle>", "constructor": "rhapsody::raw::managed::handle::impl$21::clone", "arguments": ["array$<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,2>", "rhapsody::raw::managed::handle::RawManagedHandle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::array::Guard<enum2$<rhapsody::raw::binding::DescriptorSetBinding> >", "constructor": "core::array::Guard", "arguments": ["enum2$<rhapsody::raw::binding::DescriptorSetBinding>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::array::Guard<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::array::Guard<enum2$<rhapsody::raw::binding::DescriptorSetBinding> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<array$<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,2> >", "constructor": "core::ptr::drop_in_place", "arguments": ["array$<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >", "constructor": "std::collections::hash::map::Iter", "arguments": ["shader_pipeline::reflection::BindingIndex", "rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4>", "constructor": "core::iter::adapters::map::Map", "arguments": ["std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >", "country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>", "core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >", "constructor": "core::slice::iter::Iter", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >", "country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>", "core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["enum2$<rhapsody::raw::binding::DescriptorSetBinding>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$74::fmt<enum2$<rhapsody::raw::binding::DescriptorSetBinding> >", "constructor": "core::fmt::impl$74::fmt", "arguments": ["enum2$<rhapsody::raw::binding::DescriptorSetBinding>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> >", "constructor": "ref$", "arguments": ["enum2$<rhapsody::raw::binding::DescriptorSetBinding>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "constructor": "tuple$", "arguments": ["shader_pipeline::reflection::BindingIndex", "shader_pipeline::reflection::BindingArrayElement", "ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > > >", "constructor": "ref$", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > > > >", "constructor": "tuple$", "arguments": ["ref$<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > >", "constructor": "rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0", "arguments": ["core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > > >", "constructor": "ref_mut$", "arguments": ["rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::impls::impl$3::call_mut<tuple$<ref$<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > > > >,ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > > > >", "constructor": "core::ops::function::impls::impl$3::call_mut", "arguments": ["tuple$<ref$<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > > > >", "ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > > >", "constructor": "tuple$", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "constructor": "core::iter::adapters::enumerate::Enumerate", "arguments": ["core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6> >", "constructor": "rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6>,rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6> > >", "constructor": "core::iter::adapters::filter::Filter", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6>", "rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::binding::impl$0::update_descriptor_set::closure_env$0<core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6>,rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6> > > >", "constructor": "rhapsody::raw::binding::impl$0::update_descriptor_set::closure_env$0", "arguments": ["core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6>,rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::impls::impl$4::call_once<tuple$<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > > >,rhapsody::raw::binding::impl$0::update_descriptor_set::closure_env$0<core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6>,rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6> > > > >", "constructor": "core::ops::function::impls::impl$4::call_once", "arguments": ["tuple$<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > > >", "rhapsody::raw::binding::impl$0::update_descriptor_set::closure_env$0<core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6>,rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >", "constructor": "ref$", "arguments": ["rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<shader_pipeline::reflection::BindingIndex>,ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >", "constructor": "tuple$", "arguments": ["ref$<shader_pipeline::reflection::BindingIndex>", "ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2>", "constructor": "core::iter::adapters::map::Map", "arguments": ["std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >", "country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>", "core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > >", "constructor": "hashbrown::map::impl$82::extend::closure_env$0", "arguments": ["shader_pipeline::reflection::BindingIndex", "alloc::string::String", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>", "hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<shader_pipeline::reflection::BindingIndex>,ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,tuple$<>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > >", "constructor": "core::iter::adapters::map::map_fold::closure_env$0", "arguments": ["tuple$<ref$<shader_pipeline::reflection::BindingIndex>,ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >", "tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>", "tuple$<>", "country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$44::fold::closure_env$0<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<shader_pipeline::reflection::BindingIndex>,ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,tuple$<>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > > >", "constructor": "hashbrown::map::impl$44::fold::closure_env$0", "arguments": ["shader_pipeline::reflection::BindingIndex", "rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>", "tuple$<>", "core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<shader_pipeline::reflection::BindingIndex>,ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,tuple$<>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawIterRange::fold_impl<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,hashbrown::map::impl$44::fold::closure_env$0<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<shader_pipeline::reflection::BindingIndex>,ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,tuple$<>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > > >,tuple$<> >", "constructor": "hashbrown::raw::RawIterRange::fold_impl", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >", "hashbrown::map::impl$44::fold::closure_env$0<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<shader_pipeline::reflection::BindingIndex>,ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,tuple$<>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > > >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["shader_pipeline::reflection::BindingIndex", "rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::rg::pass_api::RenderPassBinding> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["enum2$<rhapsody::raw::binding::DescriptorSetBinding>", "core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::rg::pass_api::RenderPassBinding> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > >", "constructor": "ref_mut$", "arguments": ["core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1,tuple$<>,ref_mut$<core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>", "core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >", "country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1", "tuple$<>", "ref_mut$<core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$82::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > > > >", "constructor": "core::iter::traits::iterator::Iterator::find::check::closure_env$0", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> >", "constructor": "core::ops::control_flow::ControlFlow", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > >", "constructor": "enum2$", "arguments": ["core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::try_fold<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > > >", "constructor": "core::iter::adapters::map::impl$2::try_fold", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >", "country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2", "tuple$<>", "core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > > > >", "enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > >", "constructor": "rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0", "arguments": ["core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > >", "constructor": "ref_mut$", "arguments": ["rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > >", "constructor": "core::iter::traits::iterator::Iterator::find::check::closure_env$0", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::try_fold<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > > >", "constructor": "core::iter::adapters::map::impl$2::try_fold", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >", "country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4", "tuple$<>", "core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > >", "enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$82::extend<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > >", "constructor": "hashbrown::map::impl$82::extend", "arguments": ["shader_pipeline::reflection::BindingIndex", "alloc::string::String", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["shader_pipeline::reflection::BindingIndex", "rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::handle::GpuOwnedInner<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>", "constructor": "rhapsody::raw::managed::handle::GpuOwnedInner", "arguments": ["enum2$<rhapsody::raw::binding::DescriptorSetBinding>", "rhapsody::raw::managed::handle::RawManagedHandle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$74::fmt<rhapsody::raw::managed::handle::GpuOwnedInner<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >", "constructor": "core::fmt::impl$74::fmt", "arguments": ["rhapsody::raw::managed::handle::GpuOwnedInner<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$74::fmt<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >", "constructor": "core::fmt::impl$74::fmt", "arguments": ["rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<rhapsody::raw::binding::DescriptorSetBinding> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<rhapsody::raw::binding::DescriptorSetBinding>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6> >", "constructor": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$4>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$4"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$4> >", "constructor": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$4>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > >", "constructor": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set", "arguments": ["core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > >", "constructor": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set", "arguments": ["core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::mem::manually_drop::ManuallyDrop<rhapsody::raw::managed::handle::GpuOwnedInner<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >", "constructor": "core::mem::manually_drop::ManuallyDrop", "arguments": ["rhapsody::raw::managed::handle::GpuOwnedInner<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$74::fmt<core::mem::manually_drop::ManuallyDrop<rhapsody::raw::managed::handle::GpuOwnedInner<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >", "constructor": "core::fmt::impl$74::fmt", "arguments": ["core::mem::manually_drop::ManuallyDrop<rhapsody::raw::managed::handle::GpuOwnedInner<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: binding.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: binding.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 5; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// [文件级接口候选，具体函数归属 unknown] core::cmp::Ordering (*)(ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >,ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >)
// 候选参数类型: ["ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >", "ref$<tuple$<usize,hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> > >"]
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>,bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>", "bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::roofs::dormer_tiles::DormerTiles>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::event::reader::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>", "bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>", "bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>", "bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>", "bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::AdjustWallHeightCmd>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::audio::building_audio::PlayBuildingAudio>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>
// [候选系统接口] component_query: bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >

// ------------------------------------------------------------------
// 函数: rhapsody::raw::binding::impl$0::bind_descriptor_set::closure$0
// 信息 ID: 035a999deb2f21700f510feb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::binding::impl$0::bind_descriptor_set", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0 <- core::ops::function::impls::impl$3::call_mut<tuple$<ref$<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > > > >,ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0 <- core::ops::function::impls::impl$4::call_once<tuple$<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > > >,rhapsody::raw::binding::impl$0::update_descriptor_set::closure_env$0<core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6>,rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6> > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<ref$<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > > > >,ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > > > >; module=576, inline_record=1792; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > > >; module=1610, inline_record=112764; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > > >; module=1610, inline_record=121824; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6> >; module=1657, inline_record=23520; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$4> >; module=1657, inline_record=28104; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::binding::impl$0::update_descriptor_set::closure$0
// 信息 ID: 9088486cc558d36c487c636c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::binding::impl$0::update_descriptor_set", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::binding::impl$0::update_descriptor_set::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::map::Map<core::iter::adapters::map::Map<alloc::vec::drain::Drain<country_core::asset_libraries::texture_library::AssetIndex,alloc::alloc::Global>,country_core::asset_libraries::texture_library::impl$1::update_bindless_texture_table::closure_env$0>,rhapsody::raw::binding::impl$0::update_descriptor_set::closure_env$0<core::iter::adapters::map::Map<alloc::vec::drain::Drain<country_core::asset_libraries::texture_library::AssetIndex,alloc::alloc::Global>,country_core::asset_libraries::texture_library::impl$1::update_bindless_texture_table::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::binding::impl$0::update_descriptor_set::closure_env$0 <- core::ops::function::impls::impl$4::call_once<tuple$<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > > >,rhapsody::raw::binding::impl$0::update_descriptor_set::closure_env$0<core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6>,rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6> > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > > >,rhapsody::raw::binding::impl$0::update_descriptor_set::closure_env$0<core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6>,rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6> > > > >; module=576, inline_record=3824; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::binding::impl$1::fmt
// 信息 ID: 6e954f6f8dc810be70558ae0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$74::fmt<enum2$<rhapsody::raw::binding::DescriptorSetBinding> >; module=576, inline_record=424; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::binding::impl$2::clone
// 信息 ID: fd7387d03413a2f7f8435a1b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::cloned::impl$8::next_unchecked<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> >,enum2$<rhapsody::raw::binding::DescriptorSetBinding> >; module=63, inline_record=53668; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::cloned::impl$8::next_unchecked<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> >,enum2$<rhapsody::raw::binding::DescriptorSetBinding> >; module=572, inline_record=3836; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > >
// 信息 ID: e431174eca8d54573b91b675
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["shader_pipeline::reflection::BindingIndex", "rhapsody::rg::binding::PassResourceSlot"]; 字段布局 = unknown。
// [类型关系] alloc::vec::into_iter::IntoIter -> ["tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>", "country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["rhapsody::raw::binding::DescriptorSetBinding"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::managed::handle::GpuOwned -> ["enum2$<rhapsody::raw::binding::DescriptorSetBinding>", "rhapsody::raw::managed::handle::RawManagedHandle"]; 字段布局 = unknown。
// [类型关系] tuple$ -> ["shader_pipeline::reflection::BindingIndex", "rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>"]; 字段布局 = unknown。
// [类型关系] core::slice::iter::Iter -> ["tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >", "country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::chain::Chain -> ["core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>", "core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2>"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::bind_descriptor_set -> ["core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0,alloc::alloc::Global> @ 0x169b104 -> 0x16a6010 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > >
// 信息 ID: 9d7404397ab0fe3e77b073a4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> ["shader_pipeline::reflection::BindingIndex", "rhapsody::rg::binding::PassResourceSlot"]; 字段布局 = unknown。
// [类型关系] alloc::vec::into_iter::IntoIter -> ["tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>", "country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["rhapsody::raw::binding::DescriptorSetBinding"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::managed::handle::GpuOwned -> ["enum2$<rhapsody::raw::binding::DescriptorSetBinding>", "rhapsody::raw::managed::handle::RawManagedHandle"]; 字段布局 = unknown。
// [类型关系] std::collections::hash::map::Iter -> ["shader_pipeline::reflection::BindingIndex", "rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >", "country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::chain::Chain -> ["core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>", "core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4>"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::bind_descriptor_set -> ["core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x1698553 -> 0x16a5910 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$4> >
// 信息 ID: bf07a0f453dea7379bc71530
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["rhapsody::raw::binding::DescriptorSetBinding"]; 字段布局 = unknown。
// [类型关系] core::slice::iter::Iter -> ["enum2$<rhapsody::raw::binding::DescriptorSetBinding>"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::enumerate::Enumerate -> ["core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> >"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$4"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::bind_descriptor_set -> ["core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$4>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x16985f3 -> 0x16a5380 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0,alloc::alloc::Global> @ 0x169b19d -> 0x16a5380 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6> >
// 信息 ID: 82aa92e72afb8449bbd3e2b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["rhapsody::raw::binding::DescriptorSetBinding"]; 字段布局 = unknown。
// [类型关系] core::slice::iter::Iter -> ["enum2$<rhapsody::raw::binding::DescriptorSetBinding>"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::enumerate::Enumerate -> ["core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> >"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::bind_descriptor_set -> ["core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x16985f3 -> 0x16a5380 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0,alloc::alloc::Global> @ 0x169b19d -> 0x16a5380 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::bind_persistent_descriptor_set
// 信息 ID: f81e28282bb84242ef5bf94b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x1698663 -> 0xbee150 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::bind_pipeline
// 信息 ID: 3f9ab18547dc6e038ec236cd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x1697fed -> 0xbee130 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$28::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0,alloc::alloc::Global> @ 0x169b049 -> 0xbee130 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::update_descriptor_set
// 信息 ID: 21490be387d0d9638772b828
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::update_persistent_descriptor_set<core::iter::adapters::map::Map<alloc::vec::drain::Drain<country_core::asset_libraries::texture_library::AssetIndex,alloc::alloc::Global>,country_core::asset_libraries::texture_library::impl$1::update_bindless_texture_table::closure_env$0> >; module=1657, inline_record=2112; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6> >; module=1657, inline_record=22708; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$4> >; module=1657, inline_record=27292; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > >; module=1657, inline_record=32264; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > >; module=1657, inline_record=38472; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
