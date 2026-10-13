// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/raw/descriptor_set.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: f611f273354b67c997fd9bd04370c8ae9b67be95cd504c47bdbabea5b1ced99d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: descriptor_set.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 66, 482, 703, 1657
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
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::descriptor_set::impl$1::clone
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2710
// 首条位置: {"module":66,"inlinee":10000}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":66,"record_offset":6940,"parent_record_offset":6916,"ancestor_record_offset":6916,"inlinee":9614,"source_line":256,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::create_persistent_descriptor_set
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":142552,"rva":12578672,"va":5381287792}
// 编译实例 RVAs: 0xbfef70
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: descriptor_set.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6206374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381287792,"raw_hex":"e85a4ca1ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5381287857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=6
//     target_aliases=1 candidates; complete list: descriptor_set.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6206820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381287792,"raw_hex":"e89c4aa1ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381288319,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bde2a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: descriptor_set.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-135020","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381287792,"raw_hex":"e894f0fdff","size":5,"soft_fail":false,"target_va":5381153440,"va":5381288455,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418aead0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: descriptor_set.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13302063","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381287792,"raw_hex":"e82ff9ca00","size":5,"soft_fail":false,"target_va":5394590416,"va":5381288348,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418c2150
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: descriptor_set.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13381957","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381287792,"raw_hex":"e84531cc00","size":5,"soft_fail":false,"target_va":5394669904,"va":5381287942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30252343","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381287792,"raw_hex":"e8379dcd01","size":5,"soft_fail":false,"target_va":5411540992,"va":5381288644,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6931521","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381287792,"raw_hex":"e8bf3b96ff","size":5,"soft_fail":false,"target_va":5374357104,"va":5381288620,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30252248","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381287792,"raw_hex":"e8d89ccd01","size":5,"soft_fail":false,"target_va":5411540963,"va":5381288710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30254189","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381287792,"raw_hex":"e86da4cd01","size":5,"soft_fail":false,"target_va":5411542880,"va":5381288686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*576(%rdi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381287792,"raw_hex":"ff9740020000","size":6,"soft_fail":false,"va":5381288094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":143272,"parent_record_offset":142552,"ancestor_record_offset":142552,"inlinee":4597,"source_line":422,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":145436,"parent_record_offset":142552,"ancestor_record_offset":142552,"inlinee":4634,"source_line":2417,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":143300,"parent_record_offset":142552,"ancestor_record_offset":142552,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::allocate_descriptor_sets
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":703,"record_offset":144060,"parent_record_offset":142552,"ancestor_record_offset":142552,"inlinee":39776,"source_line":1546,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_descriptor_pool
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":703,"record_offset":143620,"parent_record_offset":142552,"ancestor_record_offset":142552,"inlinee":39393,"source_line":1584,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DescriptorPoolCreateInfo::flags
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":143592,"parent_record_offset":142552,"ancestor_record_offset":142552,"inlinee":39771,"source_line":3662,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DescriptorSetAllocateInfo::push_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":143856,"parent_record_offset":142552,"ancestor_record_offset":142552,"inlinee":39775,"source_line":3744,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DescriptorSetVariableDescriptorCountAllocateInfo::descriptor_counts
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":143828,"parent_record_offset":142552,"ancestor_record_offset":142552,"inlinee":39774,"source_line":21196,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":703,"record_offset":143216,"parent_record_offset":142552,"ancestor_record_offset":142552,"inlinee":5102,"source_line":745,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":144652,"parent_record_offset":142552,"ancestor_record_offset":142552,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":145764,"parent_record_offset":142552,"ancestor_record_offset":142552,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":703,"record_offset":143132,"parent_record_offset":142552,"ancestor_record_offset":142552,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":143412,"parent_record_offset":142552,"ancestor_record_offset":142552,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$190::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":145616,"parent_record_offset":142552,"ancestor_record_offset":142552,"inlinee":9926,"source_line":296,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::max_bindless_descriptor_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":703,"record_offset":143740,"parent_record_offset":142552,"ancestor_record_offset":142552,"inlinee":39772,"source_line":690,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::ManagedResources::create_persistent_descriptor_set
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/descriptor_set.rs
//     evidence={"module":703,"record_offset":144820,"parent_record_offset":142552,"ancestor_record_offset":142552,"inlinee":39777,"source_line":34,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::temp_list::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/temp_list.rs
//     evidence={"module":703,"record_offset":142772,"parent_record_offset":142552,"ancestor_record_offset":142552,"inlinee":39770,"source_line":20,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":142964,"parent_record_offset":142940,"ancestor_record_offset":142552,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":142988,"parent_record_offset":142964,"ancestor_record_offset":142552,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":144588,"parent_record_offset":144564,"ancestor_record_offset":142552,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":142940,"parent_record_offset":142916,"ancestor_record_offset":142552,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":144564,"parent_record_offset":144540,"ancestor_record_offset":142552,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new_uninit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":703,"record_offset":142856,"parent_record_offset":142828,"ancestor_record_offset":142552,"inlinee":7958,"source_line":293,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":703,"record_offset":142884,"parent_record_offset":142856,"ancestor_record_offset":142552,"inlinee":6163,"source_line":480,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$1::try_new_uninit_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":703,"record_offset":142916,"parent_record_offset":142884,"ancestor_record_offset":142552,"inlinee":6164,"source_line":513,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":703,"record_offset":143064,"parent_record_offset":142828,"ancestor_record_offset":142552,"inlinee":7960,"source_line":972,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":703,"record_offset":142828,"parent_record_offset":142800,"ancestor_record_offset":142552,"inlinee":21073,"source_line":1695,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":145524,"parent_record_offset":145500,"ancestor_record_offset":142552,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":144160,"parent_record_offset":144132,"ancestor_record_offset":142552,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":145012,"parent_record_offset":144984,"ancestor_record_offset":142552,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":144540,"parent_record_offset":144516,"ancestor_record_offset":142552,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":145572,"parent_record_offset":145548,"ancestor_record_offset":142552,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":145548,"parent_record_offset":145524,"ancestor_record_offset":142552,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":144220,"parent_record_offset":144188,"ancestor_record_offset":142552,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":144188,"parent_record_offset":144160,"ancestor_record_offset":142552,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":144516,"parent_record_offset":144492,"ancestor_record_offset":142552,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":145500,"parent_record_offset":145436,"ancestor_record_offset":142552,"inlinee":4635,"source_line":1731,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":144104,"parent_record_offset":144060,"ancestor_record_offset":142552,"inlinee":4632,"source_line":482,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":144132,"parent_record_offset":144104,"ancestor_record_offset":142552,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":703,"record_offset":143652,"parent_record_offset":143620,"ancestor_record_offset":142552,"inlinee":15819,"source_line":24,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":703,"record_offset":143676,"parent_record_offset":143652,"ancestor_record_offset":142552,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":703,"record_offset":143700,"parent_record_offset":143676,"ancestor_record_offset":142552,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::set_vec_len_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":703,"record_offset":144352,"parent_record_offset":144060,"ancestor_record_offset":142552,"inlinee":39558,"source_line":29,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$329::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":703,"record_offset":145728,"parent_record_offset":145704,"ancestor_record_offset":142552,"inlinee":39783,"source_line":174,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::ptr_chain_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":703,"record_offset":144004,"parent_record_offset":143980,"ancestor_record_offset":142552,"inlinee":27900,"source_line":39,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":143796,"parent_record_offset":143740,"ancestor_record_offset":142552,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":703,"record_offset":143240,"parent_record_offset":143216,"ancestor_record_offset":142552,"inlinee":5104,"source_line":68,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\mod.rs
//     evidence={"module":703,"record_offset":143908,"parent_record_offset":143884,"ancestor_record_offset":142552,"inlinee":27895,"source_line":372,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":703,"record_offset":143932,"parent_record_offset":143908,"ancestor_record_offset":142552,"inlinee":27896,"source_line":57,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold::scan::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\scan.rs
//     evidence={"module":703,"record_offset":143980,"parent_record_offset":143956,"ancestor_record_offset":142552,"inlinee":27898,"source_line":68,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":703,"record_offset":143884,"parent_record_offset":143856,"ancestor_record_offset":142552,"inlinee":11662,"source_line":249,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":703,"record_offset":143956,"parent_record_offset":143932,"ancestor_record_offset":142552,"inlinee":6092,"source_line":2362,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":703,"record_offset":143088,"parent_record_offset":143064,"ancestor_record_offset":142552,"inlinee":7961,"source_line":510,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":703,"record_offset":143768,"parent_record_offset":143740,"ancestor_record_offset":142552,"inlinee":8040,"source_line":1859,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":703,"record_offset":145044,"parent_record_offset":145012,"ancestor_record_offset":142552,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":144444,"parent_record_offset":144380,"ancestor_record_offset":142552,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":703,"record_offset":143180,"parent_record_offset":143156,"ancestor_record_offset":142552,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":703,"record_offset":143552,"parent_record_offset":143528,"ancestor_record_offset":142552,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":703,"record_offset":143468,"parent_record_offset":143412,"ancestor_record_offset":142552,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":143012,"parent_record_offset":142988,"ancestor_record_offset":142552,"inlinee":4282,"source_line":1740,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":145472,"parent_record_offset":145436,"ancestor_record_offset":142552,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":703,"record_offset":143324,"parent_record_offset":143300,"ancestor_record_offset":142552,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":703,"record_offset":143156,"parent_record_offset":143132,"ancestor_record_offset":142552,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":143528,"parent_record_offset":143504,"ancestor_record_offset":142552,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":143504,"parent_record_offset":143412,"ancestor_record_offset":142552,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":145644,"parent_record_offset":145616,"ancestor_record_offset":142552,"inlinee":5335,"source_line":156,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::descriptor_set::impl$0::create_persistent_descriptor_set::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/descriptor_set.rs
//     evidence={"module":703,"record_offset":145704,"parent_record_offset":145616,"ancestor_record_offset":142552,"inlinee":39781,"source_line":43,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$11::allocate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":703,"record_offset":144876,"parent_record_offset":144844,"ancestor_record_offset":142552,"inlinee":39779,"source_line":419,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$4::create
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":703,"record_offset":144844,"parent_record_offset":144820,"ancestor_record_offset":142552,"inlinee":15367,"source_line":298,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::temp_list::TempList::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/temp_list.rs
//     evidence={"module":703,"record_offset":142800,"parent_record_offset":142772,"ancestor_record_offset":142552,"inlinee":28564,"source_line":26,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::descriptor_set::impl$0::create_persistent_descriptor_set::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9B65
// 首条位置: {"module":703,"inlinee":39781}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::enums::impl$329::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":703,"record_offset":145728,"parent_record_offset":145704,"ancestor_record_offset":145704,"inlinee":39783,"source_line":174,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::update_persistent_descriptor_set<core::iter::adapters::map::Map<alloc::vec::drain::Drain<country_core::asset_libraries::texture_library::AssetIndex,alloc::alloc::Global>,country_core::asset_libraries::texture_library::impl$1::update_bindless_texture_table::closure_env$0> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 93
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1657,"record_offset":1468,"rva":23733024,"va":5392442144}
// 编译实例 RVAs: 0x16a2320
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e6620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: descriptor_set.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17547978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392442144,"raw_hex":"e8363df4fe","size":5,"soft_fail":false,"target_va":5374895648,"va":5392443621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: descriptor_set.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-17361700","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392442144,"raw_hex":"e8dc14f7fe","size":5,"soft_fail":false,"target_va":5375081504,"va":5392443199,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140879c00
//     owner=unresolved_namespace; count=4
//     target_aliases=20 candidates; complete list: descriptor_set.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14846888","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392442144,"raw_hex":"e858741dff","size":5,"soft_fail":false,"target_va":5377596416,"va":5392443299,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a96740
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: descriptor_set.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12631678","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392442144,"raw_hex":"e882413fff","size":5,"soft_fail":false,"target_va":5379811136,"va":5392442809,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b06020
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: descriptor_set.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-12174785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392442144,"raw_hex":"e83f3a46ff","size":5,"soft_fail":false,"target_va":5380268064,"va":5392442844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bff400
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: descriptor_set.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11153265","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392442144,"raw_hex":"e88fd055ff","size":5,"soft_fail":false,"target_va":5381288960,"va":5392442220,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: descriptor_set.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19404179","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392442144,"raw_hex":"e893152801","size":5,"soft_fail":false,"target_va":5411847184,"va":5392443000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::Bump::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-9797599","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392442144,"raw_hex":"e821806aff","size":5,"soft_fail":false,"target_va":5382644656,"va":5392442250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::collections::raw_vec::RawVec$LT$T$GT$::allocate_in
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1197518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392442144,"raw_hex":"e8ce451200","size":5,"soft_fail":false,"target_va":5393639872,"va":5392442349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::collections::raw_vec::RawVec$LT$T$GT$::reserve_internal_or_panic
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1197928","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392442144,"raw_hex":"e868471200","size":5,"soft_fail":false,"target_va":5393640464,"va":5392442531,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19098525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392442144,"raw_hex":"e89d6b2301","size":5,"soft_fail":false,"target_va":5411541776,"va":5392443246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::leek::update_persistent_descriptor_set
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11270431","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392442144,"raw_hex":"e8e10654ff","size":5,"soft_fail":false,"target_va":5381171776,"va":5392442202,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*616(%rsi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5392442144,"raw_hex":"ff9668020000","size":6,"soft_fail":false,"va":5392443043,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$4::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1657,"record_offset":1912,"parent_record_offset":1468,"ancestor_record_offset":1468,"inlinee":30977,"source_line":174,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1657,"record_offset":1876,"parent_record_offset":1468,"ancestor_record_offset":1468,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":5256,"parent_record_offset":1468,"ancestor_record_offset":1468,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::update_descriptor_set
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/binding.rs
//     evidence={"module":1657,"record_offset":2112,"parent_record_offset":1468,"ancestor_record_offset":1468,"inlinee":75154,"source_line":96,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1657,"record_offset":5816,"parent_record_offset":5688,"ancestor_record_offset":1468,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":3664,"parent_record_offset":3632,"ancestor_record_offset":1468,"inlinee":4158,"source_line":281,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":3712,"parent_record_offset":3688,"ancestor_record_offset":1468,"inlinee":4161,"source_line":507,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1657,"record_offset":3688,"parent_record_offset":3664,"ancestor_record_offset":1468,"inlinee":4160,"source_line":502,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1657,"record_offset":3632,"parent_record_offset":3588,"ancestor_record_offset":1468,"inlinee":4156,"source_line":1668,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1657,"record_offset":3936,"parent_record_offset":3872,"ancestor_record_offset":1468,"inlinee":4352,"source_line":2680,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1657,"record_offset":3808,"parent_record_offset":3588,"ancestor_record_offset":1468,"inlinee":5297,"source_line":1895,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":1657,"record_offset":2768,"parent_record_offset":2740,"ancestor_record_offset":1468,"inlinee":8249,"source_line":156,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":1657,"record_offset":2984,"parent_record_offset":2960,"ancestor_record_offset":1468,"inlinee":8251,"source_line":157,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":1657,"record_offset":1960,"parent_record_offset":1936,"ancestor_record_offset":1468,"inlinee":14070,"source_line":160,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$7::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":1657,"record_offset":3388,"parent_record_offset":3364,"ancestor_record_offset":1468,"inlinee":5300,"source_line":175,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$7::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\drain.rs
//     evidence={"module":1657,"record_offset":3588,"parent_record_offset":3552,"ancestor_record_offset":1468,"inlinee":5302,"source_line":180,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::update_descriptor_sets
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1657,"record_offset":3992,"parent_record_offset":2112,"ancestor_record_offset":1468,"inlinee":75157,"source_line":1356,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::dealloc
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1657,"record_offset":4216,"parent_record_offset":4148,"ancestor_record_offset":1468,"inlinee":19984,"source_line":2228,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::is_last_allocation
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1657,"record_offset":4248,"parent_record_offset":4216,"ancestor_record_offset":1468,"inlinee":19985,"source_line":2221,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::ChunkFooter::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1657,"record_offset":5724,"parent_record_offset":5688,"ancestor_record_offset":1468,"inlinee":12170,"source_line":375,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::current_layout
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":4180,"parent_record_offset":4148,"ancestor_record_offset":1468,"inlinee":19983,"source_line":158,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::dealloc_buffer
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":4148,"parent_record_offset":4124,"ancestor_record_offset":1468,"inlinee":19982,"source_line":701,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::infallible_reserve_internal
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":2672,"parent_record_offset":2648,"ancestor_record_offset":1468,"inlinee":19975,"source_line":607,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::ptr
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":3180,"parent_record_offset":3152,"ancestor_record_offset":1468,"inlinee":19972,"source_line":137,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::reserve
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":2648,"parent_record_offset":2624,"ancestor_record_offset":1468,"inlinee":19974,"source_line":429,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::with_capacity_in
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":2208,"parent_record_offset":2180,"ancestor_record_offset":1468,"inlinee":75156,"source_line":74,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::impl$5::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/raw_vec.rs
//     evidence={"module":1657,"record_offset":4124,"parent_record_offset":4100,"ancestor_record_offset":1468,"inlinee":19981,"source_line":713,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":1657,"record_offset":3116,"parent_record_offset":2304,"ancestor_record_offset":1468,"inlinee":19969,"source_line":1451,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::push_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":1657,"record_offset":3152,"parent_record_offset":3116,"ancestor_record_offset":1468,"inlinee":19970,"source_line":1422,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::reserve
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":1657,"record_offset":2624,"parent_record_offset":2304,"ancestor_record_offset":1468,"inlinee":19973,"source_line":736,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::with_capacity_in
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":1657,"record_offset":2180,"parent_record_offset":2112,"ancestor_record_offset":1468,"inlinee":75155,"source_line":585,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::impl$17::extend
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/collections/vec.rs
//     evidence={"module":1657,"record_offset":2304,"parent_record_offset":2112,"ancestor_record_offset":1468,"inlinee":44844,"source_line":2311,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::dealloc_chunk_list
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1657,"record_offset":5688,"parent_record_offset":5664,"ancestor_record_offset":1468,"inlinee":12168,"source_line":395,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$6::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":1657,"record_offset":5664,"parent_record_offset":5640,"ancestor_record_offset":1468,"inlinee":16119,"source_line":387,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1657,"record_offset":4272,"parent_record_offset":4248,"ancestor_record_offset":1468,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1657,"record_offset":4364,"parent_record_offset":4340,"ancestor_record_offset":1468,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":1657,"record_offset":4340,"parent_record_offset":4216,"ancestor_record_offset":1468,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1657,"record_offset":3780,"parent_record_offset":3588,"ancestor_record_offset":1468,"inlinee":5303,"source_line":4520,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1657,"record_offset":2712,"parent_record_offset":2304,"ancestor_record_offset":1468,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":1657,"record_offset":2396,"parent_record_offset":2304,"ancestor_record_offset":1468,"inlinee":8256,"source_line":111,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":1657,"record_offset":2368,"parent_record_offset":2304,"ancestor_record_offset":1468,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1657,"record_offset":1936,"parent_record_offset":1912,"ancestor_record_offset":1468,"inlinee":4933,"source_line":116,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1657,"record_offset":3460,"parent_record_offset":3432,"ancestor_record_offset":1468,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":1657,"record_offset":3432,"parent_record_offset":3388,"ancestor_record_offset":1468,"inlinee":5313,"source_line":793,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1657,"record_offset":2960,"parent_record_offset":2768,"ancestor_record_offset":1468,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1657,"record_offset":2060,"parent_record_offset":2036,"ancestor_record_offset":1468,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":3316,"parent_record_offset":2304,"ancestor_record_offset":1468,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":5752,"parent_record_offset":5724,"ancestor_record_offset":1468,"inlinee":7359,"source_line":2074,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1657,"record_offset":3752,"parent_record_offset":3588,"ancestor_record_offset":1468,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1657,"record_offset":2036,"parent_record_offset":2012,"ancestor_record_offset":1468,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1657,"record_offset":2920,"parent_record_offset":2896,"ancestor_record_offset":1468,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1657,"record_offset":3908,"parent_record_offset":3872,"ancestor_record_offset":1468,"inlinee":5530,"source_line":463,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1657,"record_offset":2012,"parent_record_offset":1984,"ancestor_record_offset":1468,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":1657,"record_offset":2840,"parent_record_offset":2796,"ancestor_record_offset":1468,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":3008,"parent_record_offset":2984,"ancestor_record_offset":1468,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":3488,"parent_record_offset":3460,"ancestor_record_offset":1468,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1657,"record_offset":2896,"parent_record_offset":2872,"ancestor_record_offset":1468,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1657,"record_offset":2796,"parent_record_offset":2768,"ancestor_record_offset":1468,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1657,"record_offset":1984,"parent_record_offset":1960,"ancestor_record_offset":1468,"inlinee":8257,"source_line":171,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":1657,"record_offset":2872,"parent_record_offset":2796,"ancestor_record_offset":1468,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1657,"record_offset":4020,"parent_record_offset":2112,"ancestor_record_offset":1468,"inlinee":6812,"source_line":2956,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":1657,"record_offset":4044,"parent_record_offset":4020,"ancestor_record_offset":1468,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::get_vk_descriptor_set
// 记录形式: compiled_procedure; instances=1
// 源码行: 111
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":146348,"rva":12579840,"va":5381288960}
// 编译实例 RVAs: 0xbff400
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30192554","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381288960,"raw_hex":"e8aab3cc01","size":5,"soft_fail":false,"target_va":5411481622,"va":5381289063,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":703,"record_offset":146492,"parent_record_offset":146348,"ancestor_record_offset":146348,"inlinee":11960,"source_line":315,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":703,"record_offset":146528,"parent_record_offset":146492,"ancestor_record_offset":146348,"inlinee":11948,"source_line":14,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":703,"record_offset":146576,"parent_record_offset":146492,"ancestor_record_offset":146348,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":703,"record_offset":146552,"parent_record_offset":146492,"ancestor_record_offset":146348,"inlinee":11950,"source_line":32,"archive":"_pdb/B/modules/00703.jsonl.gz"}
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
// 完整结构及原始类型记录: descriptor_set.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// 本文件没有可确认归属的结构定义记录或命名类型；是否定义数据结构及其字段 = unknown。
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: descriptor_set.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: descriptor_set.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::descriptor_set::impl$0::create_persistent_descriptor_set::closure$1
// 信息 ID: 7b2f10b807bed068506aa032
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::descriptor_set::impl$0::create_persistent_descriptor_set", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_persistent_descriptor_set; module=703, inline_record=145704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::descriptor_set::impl$1::clone
// 信息 ID: 4638d77615878730023bad12
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0>; module=66, inline_record=6916; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0>; module=66, inline_record=29324; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::state::RenderGraphState::compile<tuple$<>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::execute_presentation_render_graph::closure_env$0>; module=66, inline_record=43452; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_stream::RasterPass::new_impl; module=482, inline_record=11804; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_persistent_descriptor_set
// 信息 ID: 72a34f4ca17ac139da4e44cf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::new @ 0x93bb99 -> 0xbfef70 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::get_vk_descriptor_set
// 信息 ID: 9039cea270a1ceafe1485092
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_persistent_descriptor_set<core::iter::adapters::map::Map<alloc::vec::drain::Drain<country_core::asset_libraries::texture_library::AssetIndex,alloc::alloc::Global>,country_core::asset_libraries::texture_library::impl$1::update_bindless_texture_table::closure_env$0> > @ 0x16a236c -> 0xbff400 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::update_persistent_descriptor_set<core::iter::adapters::map::Map<alloc::vec::drain::Drain<country_core::asset_libraries::texture_library::AssetIndex,alloc::alloc::Global>,country_core::asset_libraries::texture_library::impl$1::update_bindless_texture_table::closure_env$0> >
// 信息 ID: 1ac06d42af63e8c923a103fe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::drain::Drain -> ["country_core::asset_libraries::texture_library::AssetIndex", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["alloc::vec::drain::Drain<country_core::asset_libraries::texture_library::AssetIndex,alloc::alloc::Global>", "country_core::asset_libraries::texture_library::impl$1::update_bindless_texture_table::closure_env$0"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::update_persistent_descriptor_set -> ["core::iter::adapters::map::Map<alloc::vec::drain::Drain<country_core::asset_libraries::texture_library::AssetIndex,alloc::alloc::Global>,country_core::asset_libraries::texture_library::impl$1::update_bindless_texture_table::closure_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::bindless_textures_dset @ 0x93d136 -> 0x16a2320 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
