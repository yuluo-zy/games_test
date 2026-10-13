// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/raw/descriptor_set.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: dca8feefd9fd74040e841d4174e7cbceb36d05f649a88cda93a9aed60db0e710
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: descriptor_set.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 197, 473, 570, 1332
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"PDB_registry_path_only","version":"0.38.0+1.3.281"}]
// use bumpalo; // namespace candidate
//   package identities: [{"name":"bumpalo","source":"PDB_registry_path_only","version":"3.12.0"}]
// use core; // namespace candidate
//   package identities: []
// use rhapsody; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::descriptor_set::impl$1::clone
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 11
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x316B
// 首条位置: {"module":197,"inlinee":12651}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":197,"record_offset":6056,"parent_record_offset":6032,"ancestor_record_offset":6032,"inlinee":7406,"source_line":256,"archive":"_pdb/A/modules/00197.jsonl.gz"}
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
// 首条位置: {"module":570,"record_offset":83044,"rva":7621040,"va":5376330160}
// 编译实例 RVAs: 0x7449b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: descriptor_set.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3580533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376330160,"raw_hex":"e88b5dc9ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5376330224,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: descriptor_set.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3580979","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376330160,"raw_hex":"e8cd5bc9ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376330686,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072b5a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: descriptor_set.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-104096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376330160,"raw_hex":"e86069feff","size":5,"soft_fail":false,"target_va":5376226720,"va":5376330811,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1411770a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: descriptor_set.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10691780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376330160,"raw_hex":"e8c424a300","size":5,"soft_fail":false,"target_va":5387022496,"va":5376330711,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117c9e0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: descriptor_set.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10715033","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376330160,"raw_hex":"e8997fa300","size":5,"soft_fail":false,"target_va":5387045344,"va":5376330306,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26647142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376330160,"raw_hex":"e8669a9601","size":5,"soft_fail":false,"target_va":5402978144,"va":5376330997,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-5023870","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376330160,"raw_hex":"e88257b3ff","size":5,"soft_fail":false,"target_va":5371307104,"va":5376330969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26649083","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376330160,"raw_hex":"e8fba19601","size":5,"soft_fail":false,"target_va":5402980128,"va":5376331040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*568(%rdi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5376330160,"raw_hex":"ff9738020000","size":6,"soft_fail":false,"va":5376330448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":83680,"parent_record_offset":83044,"ancestor_record_offset":83044,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":85888,"parent_record_offset":83044,"ancestor_record_offset":83044,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":83708,"parent_record_offset":83044,"ancestor_record_offset":83044,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::allocate_descriptor_sets
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":570,"record_offset":84412,"parent_record_offset":83044,"ancestor_record_offset":83044,"inlinee":31059,"source_line":1546,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_descriptor_pool
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":570,"record_offset":84004,"parent_record_offset":83044,"ancestor_record_offset":83044,"inlinee":30531,"source_line":1584,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DescriptorPoolCreateInfo::flags
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":570,"record_offset":83976,"parent_record_offset":83044,"ancestor_record_offset":83044,"inlinee":31054,"source_line":3662,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DescriptorSetAllocateInfo::push_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":570,"record_offset":84208,"parent_record_offset":83044,"ancestor_record_offset":83044,"inlinee":31058,"source_line":3744,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DescriptorSetVariableDescriptorCountAllocateInfo::descriptor_counts
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":570,"record_offset":84180,"parent_record_offset":83044,"ancestor_record_offset":83044,"inlinee":31057,"source_line":21196,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":570,"record_offset":83624,"parent_record_offset":83044,"ancestor_record_offset":83044,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":85172,"parent_record_offset":83044,"ancestor_record_offset":83044,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":86164,"parent_record_offset":83044,"ancestor_record_offset":83044,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":570,"record_offset":83540,"parent_record_offset":83044,"ancestor_record_offset":83044,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":570,"record_offset":83820,"parent_record_offset":83044,"ancestor_record_offset":83044,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$189::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":570,"record_offset":86012,"parent_record_offset":83044,"ancestor_record_offset":83044,"inlinee":13747,"source_line":296,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::max_bindless_descriptor_count
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":570,"record_offset":84124,"parent_record_offset":83044,"ancestor_record_offset":83044,"inlinee":31055,"source_line":612,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::ManagedResources::create_persistent_descriptor_set
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/descriptor_set.rs
//     evidence={"module":570,"record_offset":85312,"parent_record_offset":83044,"ancestor_record_offset":83044,"inlinee":30444,"source_line":34,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::temp_list::impl$1::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/temp_list.rs
//     evidence={"module":570,"record_offset":83264,"parent_record_offset":83044,"ancestor_record_offset":83044,"inlinee":31053,"source_line":20,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":83432,"parent_record_offset":83408,"ancestor_record_offset":83044,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":83456,"parent_record_offset":83432,"ancestor_record_offset":83044,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":84972,"parent_record_offset":84948,"ancestor_record_offset":83044,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":83376,"parent_record_offset":83348,"ancestor_record_offset":83044,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":83408,"parent_record_offset":83376,"ancestor_record_offset":83044,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":84948,"parent_record_offset":84924,"ancestor_record_offset":83044,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":570,"record_offset":83348,"parent_record_offset":83320,"ancestor_record_offset":83044,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":570,"record_offset":83320,"parent_record_offset":83292,"ancestor_record_offset":83044,"inlinee":18352,"source_line":1259,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":85488,"parent_record_offset":85456,"ancestor_record_offset":83044,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":85976,"parent_record_offset":85952,"ancestor_record_offset":83044,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":84548,"parent_record_offset":84520,"ancestor_record_offset":83044,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":84520,"parent_record_offset":84492,"ancestor_record_offset":83044,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":84696,"parent_record_offset":84520,"ancestor_record_offset":83044,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":84924,"parent_record_offset":84900,"ancestor_record_offset":83044,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":85952,"parent_record_offset":85888,"ancestor_record_offset":83044,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":84464,"parent_record_offset":84412,"ancestor_record_offset":83044,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":84492,"parent_record_offset":84464,"ancestor_record_offset":83044,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":570,"record_offset":84036,"parent_record_offset":84004,"ancestor_record_offset":83044,"inlinee":11125,"source_line":24,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":570,"record_offset":84060,"parent_record_offset":84036,"ancestor_record_offset":83044,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":570,"record_offset":84084,"parent_record_offset":84060,"ancestor_record_offset":83044,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::set_vec_len_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":570,"record_offset":84736,"parent_record_offset":84412,"ancestor_record_offset":83044,"inlinee":31060,"source_line":29,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$329::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":570,"record_offset":86128,"parent_record_offset":86104,"ancestor_record_offset":83044,"inlinee":31066,"source_line":174,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::ptr_chain_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":570,"record_offset":84356,"parent_record_offset":84332,"ancestor_record_offset":83044,"inlinee":6177,"source_line":39,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":570,"record_offset":83648,"parent_record_offset":83624,"ancestor_record_offset":83044,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\mod.rs
//     evidence={"module":570,"record_offset":84260,"parent_record_offset":84236,"ancestor_record_offset":83044,"inlinee":6172,"source_line":372,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\scan.rs
//     evidence={"module":570,"record_offset":84284,"parent_record_offset":84260,"ancestor_record_offset":83044,"inlinee":6173,"source_line":56,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold::scan::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\scan.rs
//     evidence={"module":570,"record_offset":84332,"parent_record_offset":84308,"ancestor_record_offset":83044,"inlinee":6175,"source_line":67,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":570,"record_offset":84236,"parent_record_offset":84208,"ancestor_record_offset":83044,"inlinee":6170,"source_line":258,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":570,"record_offset":84308,"parent_record_offset":84284,"ancestor_record_offset":83044,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":570,"record_offset":84148,"parent_record_offset":84124,"ancestor_record_offset":83044,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":570,"record_offset":85520,"parent_record_offset":85488,"ancestor_record_offset":83044,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":84852,"parent_record_offset":84820,"ancestor_record_offset":83044,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":83588,"parent_record_offset":83564,"ancestor_record_offset":83044,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":570,"record_offset":83940,"parent_record_offset":83916,"ancestor_record_offset":83044,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":570,"record_offset":83880,"parent_record_offset":83820,"ancestor_record_offset":83044,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":83480,"parent_record_offset":83456,"ancestor_record_offset":83044,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":85924,"parent_record_offset":85888,"ancestor_record_offset":83044,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":84820,"parent_record_offset":84736,"ancestor_record_offset":83044,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":570,"record_offset":83732,"parent_record_offset":83708,"ancestor_record_offset":83044,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":570,"record_offset":83564,"parent_record_offset":83540,"ancestor_record_offset":83044,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":570,"record_offset":83916,"parent_record_offset":83820,"ancestor_record_offset":83044,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":570,"record_offset":86040,"parent_record_offset":86012,"ancestor_record_offset":83044,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::descriptor_set::impl$0::create_persistent_descriptor_set::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/descriptor_set.rs
//     evidence={"module":570,"record_offset":86104,"parent_record_offset":86012,"ancestor_record_offset":83044,"inlinee":31064,"source_line":43,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$11::allocate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":570,"record_offset":85368,"parent_record_offset":85336,"ancestor_record_offset":83044,"inlinee":31062,"source_line":418,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$4::create
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":570,"record_offset":85336,"parent_record_offset":85312,"ancestor_record_offset":83044,"inlinee":11133,"source_line":297,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::temp_list::TempList::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/temp_list.rs
//     evidence={"module":570,"record_offset":83292,"parent_record_offset":83264,"ancestor_record_offset":83044,"inlinee":23788,"source_line":26,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::descriptor_set::impl$0::create_persistent_descriptor_set::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7958
// 首条位置: {"module":570,"inlinee":31064}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::enums::impl$329::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":570,"record_offset":86128,"parent_record_offset":86104,"ancestor_record_offset":86104,"inlinee":31066,"source_line":174,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::update_persistent_descriptor_set<core::iter::adapters::map::Map<alloc::vec::drain::Drain<country_core::asset_libraries::texture_library::AssetIndex,alloc::alloc::Global>,country_core::asset_libraries::texture_library::impl$1::update_bindless_texture_table::closure_env$0> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 90
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1332,"record_offset":1548,"rva":15311216,"va":5384020336}
// 编译实例 RVAs: 0xe9a170
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$bumpalo..Bump$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-7119096","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384020336,"raw_hex":"e8085f93ff","size":5,"soft_fail":false,"target_va":5376902512,"va":5384021603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140544f00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: descriptor_set.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9786840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384020336,"raw_hex":"e828aa6aff","size":5,"soft_fail":false,"target_va":5374234368,"va":5384021203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e14a0
//     owner=unresolved_namespace; count=4
//     target_aliases=6 candidates; complete list: descriptor_set.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8098502","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384020336,"raw_hex":"e83a6d84ff","size":5,"soft_fail":false,"target_va":5375923360,"va":5384021857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140744e80
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: descriptor_set.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7689002","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384020336,"raw_hex":"e8d6ac8aff","size":5,"soft_fail":false,"target_va":5376331392,"va":5384020389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407d0550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: descriptor_set.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7117948","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384020336,"raw_hex":"e8846393ff","size":5,"soft_fail":false,"target_va":5376902480,"va":5384020423,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e923b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: descriptor_set.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-33109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384020336,"raw_hex":"e8ab7effff","size":5,"soft_fail":false,"target_va":5383988144,"va":5384021248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14102e6c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: descriptor_set.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1655533","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384020336,"raw_hex":"e8ed421900","size":5,"soft_fail":false,"target_va":5385676480,"va":5384020942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144aa0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: descriptor_set.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19571929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384020336,"raw_hex":"e8d9a42a01","size":5,"soft_fail":false,"target_va":5403593376,"va":5384021442,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::Bump::alloc_layout_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-7117904","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384020336,"raw_hex":"e8b06393ff","size":5,"soft_fail":false,"target_va":5376902656,"va":5384020555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::alloc::handle_alloc_error
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t7977687","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384020336,"raw_hex":"e8d7ba7900","size":5,"soft_fail":false,"target_va":5391998272,"va":5384020580,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::collections::raw_vec::capacity_overflow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t7976724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384020336,"raw_hex":"e814b77900","size":5,"soft_fail":false,"target_va":5391998384,"va":5384021655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18957234","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384020336,"raw_hex":"e8b2432101","size":5,"soft_fail":false,"target_va":5402978880,"va":5384021641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18958416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384020336,"raw_hex":"e850482101","size":5,"soft_fail":false,"target_va":5402980128,"va":5384021707,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5384020336,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5384021496,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$4::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1332,"record_offset":1992,"parent_record_offset":1548,"ancestor_record_offset":1548,"inlinee":24559,"source_line":175,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1332,"record_offset":1956,"parent_record_offset":1548,"ancestor_record_offset":1548,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1332,"record_offset":5792,"parent_record_offset":1548,"ancestor_record_offset":1548,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::update_descriptor_set
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/binding.rs
//     evidence={"module":1332,"record_offset":2076,"parent_record_offset":1548,"ancestor_record_offset":1548,"inlinee":62135,"source_line":96,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1332,"record_offset":4384,"parent_record_offset":4340,"ancestor_record_offset":1548,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1332,"record_offset":4496,"parent_record_offset":4340,"ancestor_record_offset":1548,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":1332,"record_offset":3604,"parent_record_offset":3576,"ancestor_record_offset":1548,"inlinee":6077,"source_line":156,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":1332,"record_offset":3796,"parent_record_offset":3772,"ancestor_record_offset":1548,"inlinee":7344,"source_line":157,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":1332,"record_offset":2040,"parent_record_offset":2016,"ancestor_record_offset":1548,"inlinee":9113,"source_line":160,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$7::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":1332,"record_offset":4140,"parent_record_offset":4116,"ancestor_record_offset":1548,"inlinee":5995,"source_line":175,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$7::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":1332,"record_offset":4340,"parent_record_offset":4312,"ancestor_record_offset":1548,"inlinee":5997,"source_line":180,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::update_descriptor_sets
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1332,"record_offset":4612,"parent_record_offset":2076,"ancestor_record_offset":1548,"inlinee":62141,"source_line":1356,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::dealloc
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1332,"record_offset":4780,"parent_record_offset":4712,"ancestor_record_offset":1548,"inlinee":17342,"source_line":1707,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::is_last_allocation
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1332,"record_offset":4812,"parent_record_offset":4780,"ancestor_record_offset":1548,"inlinee":17343,"source_line":1700,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1332,"record_offset":2276,"parent_record_offset":2248,"ancestor_record_offset":1548,"inlinee":6156,"source_line":1411,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1332,"record_offset":2312,"parent_record_offset":2276,"ancestor_record_offset":1548,"inlinee":6157,"source_line":1420,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::allocate_in
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":2208,"parent_record_offset":2180,"ancestor_record_offset":1548,"inlinee":62138,"source_line":85,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::current_layout
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":4744,"parent_record_offset":4712,"ancestor_record_offset":1548,"inlinee":17341,"source_line":158,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::dealloc_buffer
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":4712,"parent_record_offset":4688,"ancestor_record_offset":1548,"inlinee":17340,"source_line":650,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":3972,"parent_record_offset":3928,"ancestor_record_offset":1548,"inlinee":17334,"source_line":137,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::with_capacity_in
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":2180,"parent_record_offset":2148,"ancestor_record_offset":1548,"inlinee":62137,"source_line":74,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::impl$0::allocate_in::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":3220,"parent_record_offset":3196,"ancestor_record_offset":1548,"inlinee":62143,"source_line":91,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::impl$5::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":4688,"parent_record_offset":4664,"ancestor_record_offset":1548,"inlinee":17339,"source_line":662,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1332,"record_offset":3928,"parent_record_offset":3264,"ancestor_record_offset":1548,"inlinee":17332,"source_line":1423,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::reserve
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1332,"record_offset":3520,"parent_record_offset":3264,"ancestor_record_offset":1548,"inlinee":17335,"source_line":735,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::with_capacity_in
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1332,"record_offset":2148,"parent_record_offset":2076,"ancestor_record_offset":1548,"inlinee":62136,"source_line":584,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::impl$11::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1332,"record_offset":4552,"parent_record_offset":2076,"ancestor_record_offset":1548,"inlinee":27046,"source_line":1943,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::impl$16::extend
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1332,"record_offset":3264,"parent_record_offset":2076,"ancestor_record_offset":1548,"inlinee":62140,"source_line":2023,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$13::alloc
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1332,"record_offset":2248,"parent_record_offset":2208,"ancestor_record_offset":1548,"inlinee":25380,"source_line":1864,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::from_size_align
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1332,"record_offset":2792,"parent_record_offset":2668,"ancestor_record_offset":1548,"inlinee":49215,"source_line":68,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1332,"record_offset":2488,"parent_record_offset":2464,"ancestor_record_offset":1548,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1332,"record_offset":2464,"parent_record_offset":2312,"ancestor_record_offset":1548,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1332,"record_offset":4468,"parent_record_offset":4340,"ancestor_record_offset":1548,"inlinee":5999,"source_line":3064,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1332,"record_offset":3548,"parent_record_offset":3264,"ancestor_record_offset":1548,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1332,"record_offset":3352,"parent_record_offset":3264,"ancestor_record_offset":1548,"inlinee":6217,"source_line":112,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1332,"record_offset":3324,"parent_record_offset":3264,"ancestor_record_offset":1548,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1332,"record_offset":2016,"parent_record_offset":1992,"ancestor_record_offset":1548,"inlinee":6044,"source_line":116,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1332,"record_offset":2512,"parent_record_offset":2488,"ancestor_record_offset":1548,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1332,"record_offset":4188,"parent_record_offset":4140,"ancestor_record_offset":1548,"inlinee":5326,"source_line":845,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1332,"record_offset":2724,"parent_record_offset":2668,"ancestor_record_offset":1548,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1332,"record_offset":2756,"parent_record_offset":2724,"ancestor_record_offset":1548,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1332,"record_offset":3772,"parent_record_offset":3604,"ancestor_record_offset":1548,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1332,"record_offset":3196,"parent_record_offset":2668,"ancestor_record_offset":1548,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1332,"record_offset":4412,"parent_record_offset":4340,"ancestor_record_offset":1548,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1332,"record_offset":3472,"parent_record_offset":3448,"ancestor_record_offset":1548,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1332,"record_offset":4068,"parent_record_offset":3264,"ancestor_record_offset":1548,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1332,"record_offset":4440,"parent_record_offset":4340,"ancestor_record_offset":1548,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1332,"record_offset":2376,"parent_record_offset":2352,"ancestor_record_offset":1548,"inlinee":6159,"source_line":563,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1332,"record_offset":2352,"parent_record_offset":2312,"ancestor_record_offset":1548,"inlinee":6158,"source_line":1299,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1332,"record_offset":3736,"parent_record_offset":3712,"ancestor_record_offset":1548,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1332,"record_offset":3448,"parent_record_offset":3424,"ancestor_record_offset":1548,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1332,"record_offset":3680,"parent_record_offset":3632,"ancestor_record_offset":1548,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1332,"record_offset":3820,"parent_record_offset":3796,"ancestor_record_offset":1548,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1332,"record_offset":2536,"parent_record_offset":2512,"ancestor_record_offset":1548,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1332,"record_offset":2820,"parent_record_offset":2668,"ancestor_record_offset":1548,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1332,"record_offset":3712,"parent_record_offset":3632,"ancestor_record_offset":1548,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1332,"record_offset":3632,"parent_record_offset":3604,"ancestor_record_offset":1548,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1332,"record_offset":3424,"parent_record_offset":3400,"ancestor_record_offset":1548,"inlinee":6061,"source_line":171,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::get_vk_descriptor_set
// 记录形式: compiled_procedure; instances=1
// 源码行: 107
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":570,"record_offset":86708,"rva":7622272,"va":5376331392}
// 编译实例 RVAs: 0x744e80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26894980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376331392,"raw_hex":"e884629a01","size":5,"soft_fail":false,"target_va":5403226480,"va":5376331495,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":570,"record_offset":86852,"parent_record_offset":86708,"ancestor_record_offset":86708,"inlinee":7658,"source_line":314,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":570,"record_offset":86888,"parent_record_offset":86852,"ancestor_record_offset":86708,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":570,"record_offset":86936,"parent_record_offset":86852,"ancestor_record_offset":86708,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":570,"record_offset":86912,"parent_record_offset":86852,"ancestor_record_offset":86708,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/00570.jsonl.gz"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::descriptor_set::impl$0::create_persistent_descriptor_set::closure$1
// 信息 ID: c460f350e98c1482a494d5f1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::descriptor_set::impl$0::create_persistent_descriptor_set", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_persistent_descriptor_set; module=570, inline_record=86104; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::descriptor_set::impl$1::clone
// 信息 ID: 8d5734ecd219e91c42aee141
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::render_loop::compile_frame_render_graph; module=197, inline_record=6032; 不是独立调用边。
// [PDB 内联宿主] tiny_glade::render_loop::render; module=197, inline_record=52640; 不是独立调用边。
// [PDB 内联宿主] country_core::render::pass_builder::draw_stream::RasterPass::new_impl; module=473, inline_record=2684; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_persistent_descriptor_set
// 信息 ID: 6ade44d7a1aec217b715e437
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::new @ 0x561739 -> 0x7449b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::get_vk_descriptor_set
// 信息 ID: b37a3ca85f66ced1b0a001ed
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_persistent_descriptor_set<core::iter::adapters::map::Map<alloc::vec::drain::Drain<country_core::asset_libraries::texture_library::AssetIndex,alloc::alloc::Global>,country_core::asset_libraries::texture_library::impl$1::update_bindless_texture_table::closure_env$0> > @ 0xe9a1a5 -> 0x744e80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::update_persistent_descriptor_set<core::iter::adapters::map::Map<alloc::vec::drain::Drain<country_core::asset_libraries::texture_library::AssetIndex,alloc::alloc::Global>,country_core::asset_libraries::texture_library::impl$1::update_bindless_texture_table::closure_env$0> >
// 信息 ID: 5663122a7dddbf0870e442d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::vec::drain::Drain -> ["country_core::asset_libraries::texture_library::AssetIndex", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["alloc::vec::drain::Drain<country_core::asset_libraries::texture_library::AssetIndex,alloc::alloc::Global>", "country_core::asset_libraries::texture_library::impl$1::update_bindless_texture_table::closure_env$0"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::update_persistent_descriptor_set -> ["core::iter::adapters::map::Map<alloc::vec::drain::Drain<country_core::asset_libraries::texture_library::AssetIndex,alloc::alloc::Global>,country_core::asset_libraries::texture_library::impl$1::update_bindless_texture_table::closure_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::asset_libraries::texture_library::AssetTextureLibrary::bindless_textures_dset @ 0x562be8 -> 0xe9a170 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
