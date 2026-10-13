// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/raw/shader.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: b2f41422e4410842eb5099f749582e562d360dc658b1a2f1e7589496c2af62eb
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: shader.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 39, 388, 393, 394, 402, 412, 415, 433, 441, 449, 462, 482, 485, 487, 554, 570, 1321, 1329, 1332, 1370, 1385, 1407, 1594, 1595, 1612, 3007
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use arrayvec; // namespace candidate
//   package identities: [{"name":"arrayvec","source":"PDB_registry_path_only","version":"0.7.4"}]
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"PDB_registry_path_only","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"PDB_registry_path_only","version":"0.4.9"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use modular_bitfield; // namespace candidate
//   package identities: [{"name":"modular-bitfield","source":"PDB_registry_path_only","version":"0.11.2"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::create_descriptor_set_layout<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<shader_pipeline::reflection::Binding> >,rhapsody::raw::descriptor_set::impl$0::create_persistent_descriptor_set::closure_env$0> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1594,"record_offset":368,"rva":18336224,"va":5387045344}
// 编译实例 RVAs: 0x117c9e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14295803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387045344,"raw_hex":"e805dd25ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5387045494,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14296832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387045344,"raw_hex":"e800d925ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5387046539,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140742f40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10722770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387045344,"raw_hex":"e82e625cff","size":5,"soft_fail":false,"target_va":5376323392,"va":5387046157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16547808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387045344,"raw_hex":"e8e07ffc00","size":5,"soft_fail":false,"target_va":5403593392,"va":5387045579,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t15931423","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387045344,"raw_hex":"e81f18f300","size":5,"soft_fail":false,"target_va":5402978144,"va":5387046716,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-15739094","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387045344,"raw_hex":"e82ad70fff","size":5,"soft_fail":false,"target_va":5371306912,"va":5387046001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9519653","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387045344,"raw_hex":"e8dbbd6eff","size":5,"soft_fail":false,"target_va":5377527120,"va":5387046768,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t15932764","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387045344,"raw_hex":"e85c1df300","size":5,"soft_fail":false,"target_va":5402979344,"va":5387046575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15932240","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387045344,"raw_hex":"e8501bf300","size":5,"soft_fail":false,"target_va":5402978944,"va":5387046699,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15933362","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387045344,"raw_hex":"e8b21ff300","size":5,"soft_fail":false,"target_va":5402980128,"va":5387046761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*552(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5387045344,"raw_hex":"ff9028020000","size":6,"soft_fail":false,"va":5387046470,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1594,"record_offset":1904,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1594,"record_offset":948,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1594,"record_offset":1348,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":4220,"source_line":2685,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1594,"record_offset":2212,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_descriptor_set_layout
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1594,"record_offset":3788,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":30534,"source_line":1561,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::impl$1034::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1594,"record_offset":2620,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":70094,"source_line":45,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DescriptorSetLayoutBindingFlagsCreateInfo::binding_flags
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1594,"record_offset":3556,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":70098,"source_line":21154,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DescriptorSetLayoutCreateInfo::push_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1594,"record_offset":3584,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":70100,"source_line":3592,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1594,"record_offset":4112,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1594,"record_offset":2036,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$4::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1594,"record_offset":792,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":24559,"source_line":175,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1594,"record_offset":3904,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1594,"record_offset":4140,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::ends_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1594,"record_offset":2308,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":12644,"source_line":1213,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::max_bindless_descriptor_count
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":1594,"record_offset":2156,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":31055,"source_line":612,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::temp_list::TempList::add
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/temp_list.rs
//     evidence={"module":1594,"record_offset":2920,"parent_record_offset":368,"ancestor_record_offset":368,"inlinee":23787,"source_line":30,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1594,"record_offset":1208,"parent_record_offset":1184,"ancestor_record_offset":368,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1594,"record_offset":1232,"parent_record_offset":1208,"ancestor_record_offset":368,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1594,"record_offset":4068,"parent_record_offset":4044,"ancestor_record_offset":368,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1594,"record_offset":3244,"parent_record_offset":3216,"ancestor_record_offset":368,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1594,"record_offset":1184,"parent_record_offset":1076,"ancestor_record_offset":368,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1594,"record_offset":4044,"parent_record_offset":3952,"ancestor_record_offset":368,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1594,"record_offset":3216,"parent_record_offset":2920,"ancestor_record_offset":368,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1594,"record_offset":3980,"parent_record_offset":3952,"ancestor_record_offset":368,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1594,"record_offset":1972,"parent_record_offset":1940,"ancestor_record_offset":368,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1594,"record_offset":1076,"parent_record_offset":1036,"ancestor_record_offset":368,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1594,"record_offset":1036,"parent_record_offset":992,"ancestor_record_offset":368,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1594,"record_offset":1304,"parent_record_offset":1036,"ancestor_record_offset":368,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1594,"record_offset":3952,"parent_record_offset":3928,"ancestor_record_offset":368,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1594,"record_offset":1940,"parent_record_offset":1904,"ancestor_record_offset":368,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1594,"record_offset":1712,"parent_record_offset":1376,"ancestor_record_offset":368,"inlinee":4231,"source_line":2627,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1594,"record_offset":992,"parent_record_offset":948,"ancestor_record_offset":368,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$0::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1594,"record_offset":1376,"parent_record_offset":1348,"ancestor_record_offset":368,"inlinee":4222,"source_line":14,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1594,"record_offset":3148,"parent_record_offset":3120,"ancestor_record_offset":368,"inlinee":23748,"source_line":689,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::try_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1594,"record_offset":2972,"parent_record_offset":2920,"ancestor_record_offset":368,"inlinee":23745,"source_line":204,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1594,"record_offset":3080,"parent_record_offset":3024,"ancestor_record_offset":368,"inlinee":10360,"source_line":715,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$3::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1594,"record_offset":3120,"parent_record_offset":2920,"ancestor_record_offset":368,"inlinee":23747,"source_line":732,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1594,"record_offset":3176,"parent_record_offset":3148,"ancestor_record_offset":368,"inlinee":23749,"source_line":17,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::push_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1594,"record_offset":3024,"parent_record_offset":2996,"ancestor_record_offset":368,"inlinee":15312,"source_line":54,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::try_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1594,"record_offset":2996,"parent_record_offset":2972,"ancestor_record_offset":368,"inlinee":15309,"source_line":43,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$1033::bitor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1594,"record_offset":2644,"parent_record_offset":2620,"ancestor_record_offset":368,"inlinee":70096,"source_line":39,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1594,"record_offset":3816,"parent_record_offset":3788,"ancestor_record_offset":368,"inlinee":11125,"source_line":24,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1594,"record_offset":3840,"parent_record_offset":3816,"ancestor_record_offset":368,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1594,"record_offset":3864,"parent_record_offset":3840,"ancestor_record_offset":368,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::ptr_chain_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":1594,"record_offset":3732,"parent_record_offset":3708,"ancestor_record_offset":368,"inlinee":6177,"source_line":39,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1594,"record_offset":1128,"parent_record_offset":1076,"ancestor_record_offset":368,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1594,"record_offset":1152,"parent_record_offset":1128,"ancestor_record_offset":368,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1594,"record_offset":1800,"parent_record_offset":1772,"ancestor_record_offset":368,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1594,"record_offset":2392,"parent_record_offset":2364,"ancestor_record_offset":368,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1594,"record_offset":2060,"parent_record_offset":2036,"ancestor_record_offset":368,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$3::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1594,"record_offset":820,"parent_record_offset":792,"ancestor_record_offset":368,"inlinee":60082,"source_line":221,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\mod.rs
//     evidence={"module":1594,"record_offset":3636,"parent_record_offset":3612,"ancestor_record_offset":368,"inlinee":6172,"source_line":372,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\scan.rs
//     evidence={"module":1594,"record_offset":3660,"parent_record_offset":3636,"ancestor_record_offset":368,"inlinee":6173,"source_line":56,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold::scan::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\scan.rs
//     evidence={"module":1594,"record_offset":3708,"parent_record_offset":3684,"ancestor_record_offset":368,"inlinee":6175,"source_line":67,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1594,"record_offset":1772,"parent_record_offset":1744,"ancestor_record_offset":368,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1594,"record_offset":1744,"parent_record_offset":1712,"ancestor_record_offset":368,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1594,"record_offset":3612,"parent_record_offset":3584,"ancestor_record_offset":368,"inlinee":6170,"source_line":258,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1594,"record_offset":3684,"parent_record_offset":3660,"ancestor_record_offset":368,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1594,"record_offset":3400,"parent_record_offset":2920,"ancestor_record_offset":368,"inlinee":8892,"source_line":728,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap_simple
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1594,"record_offset":3424,"parent_record_offset":3400,"ancestor_record_offset":368,"inlinee":8891,"source_line":761,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1594,"record_offset":2180,"parent_record_offset":2156,"ancestor_record_offset":368,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1594,"record_offset":4008,"parent_record_offset":3980,"ancestor_record_offset":368,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1594,"record_offset":904,"parent_record_offset":880,"ancestor_record_offset":368,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1594,"record_offset":3928,"parent_record_offset":3904,"ancestor_record_offset":368,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1594,"record_offset":880,"parent_record_offset":848,"ancestor_record_offset":368,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1594,"record_offset":2116,"parent_record_offset":2084,"ancestor_record_offset":368,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1594,"record_offset":3452,"parent_record_offset":3424,"ancestor_record_offset":368,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1594,"record_offset":1260,"parent_record_offset":1232,"ancestor_record_offset":368,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1594,"record_offset":1836,"parent_record_offset":1712,"ancestor_record_offset":368,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1594,"record_offset":2416,"parent_record_offset":2392,"ancestor_record_offset":368,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1594,"record_offset":2440,"parent_record_offset":2416,"ancestor_record_offset":368,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::ends_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1594,"record_offset":2364,"parent_record_offset":2336,"ancestor_record_offset":368,"inlinee":5504,"source_line":2585,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1594,"record_offset":2240,"parent_record_offset":2212,"ancestor_record_offset":368,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1594,"record_offset":2268,"parent_record_offset":2240,"ancestor_record_offset":368,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$180::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1594,"record_offset":848,"parent_record_offset":820,"ancestor_record_offset":368,"inlinee":7906,"source_line":141,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1594,"record_offset":2084,"parent_record_offset":2060,"ancestor_record_offset":368,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_suffix_of
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1594,"record_offset":2336,"parent_record_offset":2308,"ancestor_record_offset":368,"inlinee":12627,"source_line":1005,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::create_descriptor_set_layout<core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,rhapsody::raw::shader::create_descriptor_set_layouts::closure_env$0> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1594,"record_offset":4656,"rva":18337856,"va":5387046976}
// 编译实例 RVAs: 0x117d040
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14297394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387046976,"raw_hex":"e8ced625ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5387047085,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14298523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387046976,"raw_hex":"e865d225ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5387048230,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140742f40
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10724466","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387046976,"raw_hex":"e88e5b5cff","size":5,"soft_fail":false,"target_va":5376323392,"va":5387047853,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16546217","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387046976,"raw_hex":"e8a979fc00","size":5,"soft_fail":false,"target_va":5403593392,"va":5387047170,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t15929728","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387046976,"raw_hex":"e88011f300","size":5,"soft_fail":false,"target_va":5402978144,"va":5387048411,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-15740784","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387046976,"raw_hex":"e890d00fff","size":5,"soft_fail":false,"target_va":5371306912,"va":5387047691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-9521348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387046976,"raw_hex":"e83cb76eff","size":5,"soft_fail":false,"target_va":5377527120,"va":5387048463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t15931069","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387046976,"raw_hex":"e8bd16f300","size":5,"soft_fail":false,"target_va":5402979344,"va":5387048270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15930545","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387046976,"raw_hex":"e8b114f300","size":5,"soft_fail":false,"target_va":5402978944,"va":5387048394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15931667","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387046976,"raw_hex":"e81319f300","size":5,"soft_fail":false,"target_va":5402980128,"va":5387048456,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*552(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5387046976,"raw_hex":"ff9028020000","size":6,"soft_fail":false,"va":5387048166,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1594,"record_offset":6036,"parent_record_offset":4656,"ancestor_record_offset":4656,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1594,"record_offset":5096,"parent_record_offset":4656,"ancestor_record_offset":4656,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1594,"record_offset":5480,"parent_record_offset":4656,"ancestor_record_offset":4656,"inlinee":4220,"source_line":2685,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1594,"record_offset":6740,"parent_record_offset":4656,"ancestor_record_offset":4656,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_descriptor_set_layout
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1594,"record_offset":8316,"parent_record_offset":4656,"ancestor_record_offset":4656,"inlinee":30534,"source_line":1561,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::impl$1034::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1594,"record_offset":7148,"parent_record_offset":4656,"ancestor_record_offset":4656,"inlinee":70094,"source_line":45,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DescriptorSetLayoutBindingFlagsCreateInfo::binding_flags
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1594,"record_offset":8084,"parent_record_offset":4656,"ancestor_record_offset":4656,"inlinee":70098,"source_line":21154,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DescriptorSetLayoutCreateInfo::push_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1594,"record_offset":8112,"parent_record_offset":4656,"ancestor_record_offset":4656,"inlinee":70100,"source_line":3592,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1594,"record_offset":8640,"parent_record_offset":4656,"ancestor_record_offset":4656,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1594,"record_offset":6172,"parent_record_offset":4656,"ancestor_record_offset":4656,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::map::impl$4::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1594,"record_offset":5068,"parent_record_offset":4656,"ancestor_record_offset":4656,"inlinee":24559,"source_line":175,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1594,"record_offset":8432,"parent_record_offset":4656,"ancestor_record_offset":4656,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1594,"record_offset":8668,"parent_record_offset":4656,"ancestor_record_offset":4656,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::ends_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\mod.rs
//     evidence={"module":1594,"record_offset":6836,"parent_record_offset":4656,"ancestor_record_offset":4656,"inlinee":12644,"source_line":1213,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::max_bindless_descriptor_count
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":1594,"record_offset":6684,"parent_record_offset":4656,"ancestor_record_offset":4656,"inlinee":31055,"source_line":612,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::temp_list::TempList::add
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/temp_list.rs
//     evidence={"module":1594,"record_offset":7448,"parent_record_offset":4656,"ancestor_record_offset":4656,"inlinee":23787,"source_line":30,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1594,"record_offset":5340,"parent_record_offset":5316,"ancestor_record_offset":4656,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1594,"record_offset":5364,"parent_record_offset":5340,"ancestor_record_offset":4656,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1594,"record_offset":8596,"parent_record_offset":8572,"ancestor_record_offset":4656,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1594,"record_offset":7772,"parent_record_offset":7744,"ancestor_record_offset":4656,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1594,"record_offset":5316,"parent_record_offset":5212,"ancestor_record_offset":4656,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1594,"record_offset":8572,"parent_record_offset":8480,"ancestor_record_offset":4656,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1594,"record_offset":7744,"parent_record_offset":7448,"ancestor_record_offset":4656,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1594,"record_offset":8508,"parent_record_offset":8480,"ancestor_record_offset":4656,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1594,"record_offset":6108,"parent_record_offset":6076,"ancestor_record_offset":4656,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1594,"record_offset":5212,"parent_record_offset":5176,"ancestor_record_offset":4656,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1594,"record_offset":5176,"parent_record_offset":5136,"ancestor_record_offset":4656,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1594,"record_offset":5436,"parent_record_offset":5176,"ancestor_record_offset":4656,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1594,"record_offset":8480,"parent_record_offset":8456,"ancestor_record_offset":4656,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1594,"record_offset":6076,"parent_record_offset":6036,"ancestor_record_offset":4656,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1594,"record_offset":5844,"parent_record_offset":5508,"ancestor_record_offset":4656,"inlinee":4231,"source_line":2627,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1594,"record_offset":5136,"parent_record_offset":5096,"ancestor_record_offset":4656,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$0::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1594,"record_offset":5508,"parent_record_offset":5480,"ancestor_record_offset":4656,"inlinee":4222,"source_line":14,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1594,"record_offset":7676,"parent_record_offset":7648,"ancestor_record_offset":4656,"inlinee":23748,"source_line":689,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::try_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1594,"record_offset":7500,"parent_record_offset":7448,"ancestor_record_offset":4656,"inlinee":23745,"source_line":204,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$2::set_len
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1594,"record_offset":7608,"parent_record_offset":7552,"ancestor_record_offset":4656,"inlinee":10360,"source_line":715,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$3::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1594,"record_offset":7648,"parent_record_offset":7448,"ancestor_record_offset":4656,"inlinee":23747,"source_line":732,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::as_slice
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1594,"record_offset":7704,"parent_record_offset":7676,"ancestor_record_offset":4656,"inlinee":23749,"source_line":17,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::push_unchecked
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1594,"record_offset":7552,"parent_record_offset":7524,"ancestor_record_offset":4656,"inlinee":15312,"source_line":54,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec_impl::ArrayVecImpl::try_push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec_impl.rs
//     evidence={"module":1594,"record_offset":7524,"parent_record_offset":7500,"ancestor_record_offset":4656,"inlinee":15309,"source_line":43,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::bitflags::impl$1033::bitor
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1594,"record_offset":7172,"parent_record_offset":7148,"ancestor_record_offset":4656,"inlinee":70096,"source_line":39,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1594,"record_offset":8344,"parent_record_offset":8316,"ancestor_record_offset":4656,"inlinee":11125,"source_line":24,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1594,"record_offset":8368,"parent_record_offset":8344,"ancestor_record_offset":4656,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1594,"record_offset":8392,"parent_record_offset":8368,"ancestor_record_offset":4656,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::ptr_chain_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":1594,"record_offset":8260,"parent_record_offset":8236,"ancestor_record_offset":4656,"inlinee":6177,"source_line":39,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1594,"record_offset":5260,"parent_record_offset":5212,"ancestor_record_offset":4656,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1594,"record_offset":5284,"parent_record_offset":5260,"ancestor_record_offset":4656,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1594,"record_offset":5932,"parent_record_offset":5904,"ancestor_record_offset":4656,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1594,"record_offset":6920,"parent_record_offset":6892,"ancestor_record_offset":4656,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1594,"record_offset":6416,"parent_record_offset":6384,"ancestor_record_offset":4656,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1594,"record_offset":6480,"parent_record_offset":6444,"ancestor_record_offset":4656,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\mod.rs
//     evidence={"module":1594,"record_offset":8164,"parent_record_offset":8140,"ancestor_record_offset":4656,"inlinee":6172,"source_line":372,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\scan.rs
//     evidence={"module":1594,"record_offset":8188,"parent_record_offset":8164,"ancestor_record_offset":4656,"inlinee":6173,"source_line":56,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold::scan::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\scan.rs
//     evidence={"module":1594,"record_offset":8236,"parent_record_offset":8212,"ancestor_record_offset":4656,"inlinee":6175,"source_line":67,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1594,"record_offset":5904,"parent_record_offset":5876,"ancestor_record_offset":4656,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1594,"record_offset":5876,"parent_record_offset":5844,"ancestor_record_offset":4656,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1594,"record_offset":8140,"parent_record_offset":8112,"ancestor_record_offset":4656,"inlinee":6170,"source_line":258,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1594,"record_offset":8212,"parent_record_offset":8188,"ancestor_record_offset":4656,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1594,"record_offset":7928,"parent_record_offset":7448,"ancestor_record_offset":4656,"inlinee":8892,"source_line":728,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::swap_simple
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1594,"record_offset":7952,"parent_record_offset":7928,"ancestor_record_offset":4656,"inlinee":8891,"source_line":761,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::saturating_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1594,"record_offset":6708,"parent_record_offset":6684,"ancestor_record_offset":4656,"inlinee":5655,"source_line":1547,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1594,"record_offset":8536,"parent_record_offset":8508,"ancestor_record_offset":4656,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1594,"record_offset":6644,"parent_record_offset":6172,"ancestor_record_offset":4656,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1594,"record_offset":6600,"parent_record_offset":6280,"ancestor_record_offset":4656,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1594,"record_offset":8456,"parent_record_offset":8432,"ancestor_record_offset":4656,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1594,"record_offset":6564,"parent_record_offset":6540,"ancestor_record_offset":4656,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1594,"record_offset":6540,"parent_record_offset":6516,"ancestor_record_offset":4656,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1594,"record_offset":7980,"parent_record_offset":7952,"ancestor_record_offset":4656,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1594,"record_offset":5392,"parent_record_offset":5364,"ancestor_record_offset":4656,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1594,"record_offset":5968,"parent_record_offset":5844,"ancestor_record_offset":4656,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1594,"record_offset":6944,"parent_record_offset":6920,"ancestor_record_offset":4656,"inlinee":4506,"source_line":15,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\cmp.rs
//     evidence={"module":1594,"record_offset":6968,"parent_record_offset":6944,"ancestor_record_offset":4656,"inlinee":4508,"source_line":83,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::ends_with
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1594,"record_offset":6892,"parent_record_offset":6864,"ancestor_record_offset":4656,"inlinee":5504,"source_line":2585,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1594,"record_offset":6768,"parent_record_offset":6740,"ancestor_record_offset":4656,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1594,"record_offset":6796,"parent_record_offset":6768,"ancestor_record_offset":4656,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_suffix_of
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\str\pattern.rs
//     evidence={"module":1594,"record_offset":6864,"parent_record_offset":6836,"ancestor_record_offset":4656,"inlinee":12627,"source_line":1005,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1594,"record_offset":6224,"parent_record_offset":6196,"ancestor_record_offset":4656,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1594,"record_offset":6516,"parent_record_offset":6280,"ancestor_record_offset":4656,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1594,"record_offset":6280,"parent_record_offset":6252,"ancestor_record_offset":4656,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1594,"record_offset":6348,"parent_record_offset":6320,"ancestor_record_offset":4656,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1594,"record_offset":6320,"parent_record_offset":6280,"ancestor_record_offset":4656,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1594,"record_offset":6252,"parent_record_offset":6224,"ancestor_record_offset":4656,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1594,"record_offset":6384,"parent_record_offset":6280,"ancestor_record_offset":4656,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1594,"record_offset":6444,"parent_record_offset":6384,"ancestor_record_offset":4656,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1594,"record_offset":6196,"parent_record_offset":6172,"ancestor_record_offset":4656,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/01594.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::DescriptorPoolDiscardPile::reclaim_descriptor_pools
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 33
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78D3
// 首条位置: {"module":570,"inlinee":30931}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$18::extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":27740,"parent_record_offset":27540,"ancestor_record_offset":27540,"inlinee":4432,"source_line":2961,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":28312,"parent_record_offset":27540,"ancestor_record_offset":27540,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":570,"record_offset":27568,"parent_record_offset":27540,"ancestor_record_offset":27540,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":27892,"parent_record_offset":27860,"ancestor_record_offset":27540,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":27980,"parent_record_offset":27956,"ancestor_record_offset":27540,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":27860,"parent_record_offset":27832,"ancestor_record_offset":27540,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":27956,"parent_record_offset":27796,"ancestor_record_offset":27540,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":27796,"parent_record_offset":27768,"ancestor_record_offset":27540,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":27832,"parent_record_offset":27796,"ancestor_record_offset":27540,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":28088,"parent_record_offset":28064,"ancestor_record_offset":27540,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":570,"record_offset":28144,"parent_record_offset":28088,"ancestor_record_offset":27540,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":570,"record_offset":27928,"parent_record_offset":27796,"ancestor_record_offset":27540,"inlinee":5454,"source_line":13,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::impl$1::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":570,"record_offset":28252,"parent_record_offset":28228,"ancestor_record_offset":27540,"inlinee":5437,"source_line":30,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":570,"record_offset":27768,"parent_record_offset":27740,"ancestor_record_offset":27540,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":570,"record_offset":28036,"parent_record_offset":28012,"ancestor_record_offset":27540,"inlinee":5097,"source_line":2580,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":570,"record_offset":28012,"parent_record_offset":27796,"ancestor_record_offset":27540,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":570,"record_offset":28064,"parent_record_offset":28036,"ancestor_record_offset":27540,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":28180,"parent_record_offset":28036,"ancestor_record_offset":27540,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":28116,"parent_record_offset":28088,"ancestor_record_offset":27540,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":27676,"parent_record_offset":27592,"ancestor_record_offset":27540,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":27700,"parent_record_offset":27676,"ancestor_record_offset":27540,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":28392,"parent_record_offset":28360,"ancestor_record_offset":27540,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":27620,"parent_record_offset":27592,"ancestor_record_offset":27540,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":28416,"parent_record_offset":28392,"ancestor_record_offset":27540,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":27644,"parent_record_offset":27620,"ancestor_record_offset":27540,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":570,"record_offset":28336,"parent_record_offset":28312,"ancestor_record_offset":27540,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":570,"record_offset":27592,"parent_record_offset":27568,"ancestor_record_offset":27540,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":570,"record_offset":28360,"parent_record_offset":28336,"ancestor_record_offset":27540,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::DescriptorSetState::allocate_descriptor_set
// 记录形式: compiled_procedure; instances=1
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":554,"record_offset":372,"rva":7510800,"va":5376219920}
// 编译实例 RVAs: 0x729b10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1402cf9b0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4564575","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376219920,"raw_hex":"e8a159baff","size":5,"soft_fail":false,"target_va":5371656624,"va":5376221194,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27372827","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376219920,"raw_hex":"e81bada101","size":5,"soft_fail":false,"target_va":5403593360,"va":5376220528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve::do_reserve_and_handle
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26699296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376219920,"raw_hex":"e820669701","size":5,"soft_fail":false,"target_va":5402919776,"va":5376220475,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4915375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376219920,"raw_hex":"e851ffb4ff","size":5,"soft_fail":false,"target_va":5371304880,"va":5376220250,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26757950","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376219920,"raw_hex":"e83e4b9801","size":5,"soft_fail":false,"target_va":5402978880,"va":5376220925,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26757934","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376219920,"raw_hex":"e82e4b9801","size":5,"soft_fail":false,"target_va":5402978944,"va":5376221005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26759215","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376219920,"raw_hex":"e82f509801","size":5,"soft_fail":false,"target_va":5402980128,"va":5376220908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1133336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376219920,"raw_hex":"e8184b1100","size":5,"soft_fail":false,"target_va":5377353776,"va":5376220435,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t26748686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376219920,"raw_hex":"e80e279801","size":5,"soft_fail":false,"target_va":5402968992,"va":5376220301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t26750225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376219920,"raw_hex":"e8112d9801","size":5,"soft_fail":false,"target_va":5402970800,"va":5376220570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*-72(%rbp)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5376219920,"raw_hex":"ff55b8","size":3,"soft_fail":false,"va":5376220155,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$28::clone
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":554,"record_offset":1332,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":5316,"source_line":2063,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":554,"record_offset":1420,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::append
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":1816,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":9608,"source_line":2080,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":1620,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":7328,"source_line":2218,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":1232,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":11505,"source_line":2050,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":852,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$16::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":2352,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":5021,"source_line":2942,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_descriptor_pool
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":554,"record_offset":2796,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":30531,"source_line":1584,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::reset_descriptor_pool
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":554,"record_offset":2588,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":30525,"source_line":1601,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DescriptorPoolCreateInfo::max_sets
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":554,"record_offset":2740,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":30529,"source_line":3667,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DescriptorPoolCreateInfo::pool_sizes
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":554,"record_offset":2768,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":30530,"source_line":3672,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DescriptorSetAllocateInfo::set_layouts
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":554,"record_offset":1204,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":30524,"source_line":3734,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$174::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":554,"record_offset":2712,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":30527,"source_line":3644,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":554,"record_offset":1732,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":554,"record_offset":1760,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":554,"record_offset":3216,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":554,"record_offset":2148,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":554,"record_offset":2676,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":554,"record_offset":1176,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":9339,"source_line":302,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":554,"record_offset":2464,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":554,"record_offset":616,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":554,"record_offset":796,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":5303,"source_line":646,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":554,"record_offset":1788,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":554,"record_offset":1648,"parent_record_offset":372,"ancestor_record_offset":372,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":1996,"parent_record_offset":1968,"ancestor_record_offset":372,"inlinee":4277,"source_line":270,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::needs_to_grow
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":1968,"parent_record_offset":1936,"ancestor_record_offset":372,"inlinee":4276,"source_line":414,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":932,"parent_record_offset":900,"ancestor_record_offset":372,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":1936,"parent_record_offset":1912,"ancestor_record_offset":372,"inlinee":4275,"source_line":318,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":554,"record_offset":1040,"parent_record_offset":1016,"ancestor_record_offset":372,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::append_elements
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":1876,"parent_record_offset":1816,"ancestor_record_offset":372,"inlinee":4273,"source_line":2090,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":900,"parent_record_offset":852,"ancestor_record_offset":372,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":1264,"parent_record_offset":1232,"ancestor_record_offset":372,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":1844,"parent_record_offset":1816,"ancestor_record_offset":372,"inlinee":5202,"source_line":1227,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":2048,"parent_record_offset":1876,"ancestor_record_offset":372,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":1912,"parent_record_offset":1876,"ancestor_record_offset":372,"inlinee":4274,"source_line":944,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":2116,"parent_record_offset":1816,"ancestor_record_offset":372,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":554,"record_offset":2824,"parent_record_offset":2796,"ancestor_record_offset":372,"inlinee":11125,"source_line":24,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":554,"record_offset":2616,"parent_record_offset":2588,"ancestor_record_offset":372,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":554,"record_offset":2640,"parent_record_offset":2616,"ancestor_record_offset":372,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":554,"record_offset":820,"parent_record_offset":796,"ancestor_record_offset":372,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":554,"record_offset":992,"parent_record_offset":852,"ancestor_record_offset":372,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":554,"record_offset":2076,"parent_record_offset":1876,"ancestor_record_offset":372,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":554,"record_offset":2552,"parent_record_offset":2528,"ancestor_record_offset":372,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":554,"record_offset":2500,"parent_record_offset":2464,"ancestor_record_offset":372,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":554,"record_offset":964,"parent_record_offset":852,"ancestor_record_offset":372,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":554,"record_offset":732,"parent_record_offset":644,"ancestor_record_offset":372,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":554,"record_offset":756,"parent_record_offset":732,"ancestor_record_offset":372,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":554,"record_offset":2376,"parent_record_offset":2352,"ancestor_record_offset":372,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":554,"record_offset":2400,"parent_record_offset":2376,"ancestor_record_offset":372,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":554,"record_offset":2528,"parent_record_offset":2464,"ancestor_record_offset":372,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":554,"record_offset":2228,"parent_record_offset":2196,"ancestor_record_offset":372,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":554,"record_offset":676,"parent_record_offset":644,"ancestor_record_offset":372,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":554,"record_offset":1360,"parent_record_offset":1332,"ancestor_record_offset":372,"inlinee":5317,"source_line":2665,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":554,"record_offset":1076,"parent_record_offset":1040,"ancestor_record_offset":372,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":554,"record_offset":1672,"parent_record_offset":1648,"ancestor_record_offset":372,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":554,"record_offset":1384,"parent_record_offset":1360,"ancestor_record_offset":372,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":554,"record_offset":2252,"parent_record_offset":2228,"ancestor_record_offset":372,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":554,"record_offset":700,"parent_record_offset":676,"ancestor_record_offset":372,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":554,"record_offset":1696,"parent_record_offset":1672,"ancestor_record_offset":372,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":554,"record_offset":1100,"parent_record_offset":1076,"ancestor_record_offset":372,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":554,"record_offset":1132,"parent_record_offset":1040,"ancestor_record_offset":372,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":554,"record_offset":2172,"parent_record_offset":2148,"ancestor_record_offset":372,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":554,"record_offset":644,"parent_record_offset":616,"ancestor_record_offset":372,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":554,"record_offset":2196,"parent_record_offset":2172,"ancestor_record_offset":372,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::impl$2::deref
// 记录形式: compiled_procedure; instances=1
// 源码行: 145
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":554,"record_offset":3664,"rva":4440720,"va":5373149840}
// 编译实例 RVAs: 0x43c290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::impl$3::deref
// 记录形式: compiled_procedure; instances=1
// 源码行: 145
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":554,"record_offset":3776,"rva":4440720,"va":5373149840}
// 编译实例 RVAs: 0x43c290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::create_descriptor_set_layouts
// 记录形式: compiled_procedure; instances=1
// 源码行: 163
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":554,"record_offset":3888,"rva":7512192,"va":5376221312}
// 编译实例 RVAs: 0x72a080
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403bc080
//     owner=unresolved_namespace; count=1
//     target_aliases=16 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3597079","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376221312,"raw_hex":"e8e91cc9ff","size":5,"soft_fail":false,"target_va":5372625024,"va":5376222098,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3471690","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376221312,"raw_hex":"e8b606cbff","size":5,"soft_fail":false,"target_va":5372749696,"va":5376221381,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3472263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376221312,"raw_hex":"e87904cbff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376221970,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d8ca0
//     owner=unresolved_namespace; count=2
//     target_aliases=19 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-334106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376221312,"raw_hex":"e8e6e6faff","size":5,"soft_fail":false,"target_va":5375888544,"va":5376222645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406de6b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-310890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376221312,"raw_hex":"e89641fbff","size":5,"soft_fail":false,"target_va":5375911600,"va":5376222485,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072b5a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t4511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376221312,"raw_hex":"e89f110000","size":5,"soft_fail":false,"target_va":5376226720,"va":5376222204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117d040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10825190","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376221312,"raw_hex":"e8e62da500","size":5,"soft_fail":false,"target_va":5387046976,"va":5376221781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14117d6d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10826791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376221312,"raw_hex":"e82734a500","size":5,"soft_fail":false,"target_va":5387048656,"va":5376221860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26756606","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376221312,"raw_hex":"e8fe459801","size":5,"soft_fail":false,"target_va":5402978144,"va":5376221533,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t-4916773","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376221312,"raw_hex":"e8dbf9b4ff","size":5,"soft_fail":false,"target_va":5371305056,"va":5376221824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26756832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376221312,"raw_hex":"e8e0469801","size":5,"soft_fail":false,"target_va":5402979152,"va":5376222315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26757849","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376221312,"raw_hex":"e8d94a9801","size":5,"soft_fail":false,"target_va":5402980128,"va":5376222274,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::hash::random::RandomState::new::KEYS::__getit::__KEY$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1144143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376221312,"raw_hex":"e84f751100","size":5,"soft_fail":false,"target_va":5377366224,"va":5376222076,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*552(%rbx)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5376221312,"raw_hex":"ff9328020000","size":6,"soft_fail":false,"va":5376222023,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":5076,"parent_record_offset":3888,"ancestor_record_offset":3888,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":4320,"parent_record_offset":3888,"ancestor_record_offset":3888,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_descriptor_set_layout
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":554,"record_offset":6508,"parent_record_offset":3888,"ancestor_record_offset":3888,"inlinee":30534,"source_line":1561,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$168::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":554,"record_offset":6480,"parent_record_offset":3888,"ancestor_record_offset":3888,"inlinee":30533,"source_line":3560,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":554,"record_offset":5220,"parent_record_offset":3888,"ancestor_record_offset":3888,"inlinee":5443,"source_line":46,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":554,"record_offset":6104,"parent_record_offset":3888,"ancestor_record_offset":3888,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":554,"record_offset":5836,"parent_record_offset":3888,"ancestor_record_offset":3888,"inlinee":4244,"source_line":761,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":554,"record_offset":6272,"parent_record_offset":3888,"ancestor_record_offset":3888,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":554,"record_offset":7264,"parent_record_offset":3888,"ancestor_record_offset":3888,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":554,"record_offset":4992,"parent_record_offset":3888,"ancestor_record_offset":3888,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::temp_list::TempList::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/temp_list.rs
//     evidence={"module":554,"record_offset":4076,"parent_record_offset":3888,"ancestor_record_offset":3888,"inlinee":23788,"source_line":26,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":554,"record_offset":5376,"parent_record_offset":3888,"ancestor_record_offset":3888,"inlinee":21777,"source_line":588,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":554,"record_offset":5432,"parent_record_offset":3888,"ancestor_record_offset":3888,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$8::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":554,"record_offset":6744,"parent_record_offset":3888,"ancestor_record_offset":3888,"inlinee":5535,"source_line":1322,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":4216,"parent_record_offset":4192,"ancestor_record_offset":3888,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":4240,"parent_record_offset":4216,"ancestor_record_offset":3888,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":6436,"parent_record_offset":6412,"ancestor_record_offset":3888,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":4160,"parent_record_offset":4132,"ancestor_record_offset":3888,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":4192,"parent_record_offset":4160,"ancestor_record_offset":3888,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":554,"record_offset":6412,"parent_record_offset":6320,"ancestor_record_offset":3888,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":554,"record_offset":4132,"parent_record_offset":4104,"ancestor_record_offset":3888,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":554,"record_offset":4104,"parent_record_offset":4076,"ancestor_record_offset":3888,"inlinee":18352,"source_line":1259,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":6348,"parent_record_offset":6320,"ancestor_record_offset":3888,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":5156,"parent_record_offset":5124,"ancestor_record_offset":3888,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":4412,"parent_record_offset":4384,"ancestor_record_offset":3888,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":4384,"parent_record_offset":4352,"ancestor_record_offset":3888,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":4624,"parent_record_offset":4384,"ancestor_record_offset":3888,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":554,"record_offset":6320,"parent_record_offset":6296,"ancestor_record_offset":3888,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":5124,"parent_record_offset":5076,"ancestor_record_offset":3888,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":4352,"parent_record_offset":4320,"ancestor_record_offset":3888,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":554,"record_offset":6536,"parent_record_offset":6508,"ancestor_record_offset":3888,"inlinee":11125,"source_line":24,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":554,"record_offset":6560,"parent_record_offset":6536,"ancestor_record_offset":3888,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":554,"record_offset":6584,"parent_record_offset":6560,"ancestor_record_offset":3888,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":554,"record_offset":4448,"parent_record_offset":4412,"ancestor_record_offset":3888,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":554,"record_offset":4472,"parent_record_offset":4448,"ancestor_record_offset":3888,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":554,"record_offset":7016,"parent_record_offset":6988,"ancestor_record_offset":3888,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":554,"record_offset":7068,"parent_record_offset":7044,"ancestor_record_offset":3888,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":554,"record_offset":7044,"parent_record_offset":6988,"ancestor_record_offset":3888,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":554,"record_offset":5692,"parent_record_offset":5664,"ancestor_record_offset":3888,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":554,"record_offset":5752,"parent_record_offset":5720,"ancestor_record_offset":3888,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::Map::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":554,"record_offset":5864,"parent_record_offset":5836,"ancestor_record_offset":3888,"inlinee":4246,"source_line":69,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":554,"record_offset":7092,"parent_record_offset":7068,"ancestor_record_offset":3888,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":554,"record_offset":6376,"parent_record_offset":6348,"ancestor_record_offset":3888,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":554,"record_offset":6944,"parent_record_offset":6920,"ancestor_record_offset":3888,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":554,"record_offset":5636,"parent_record_offset":5604,"ancestor_record_offset":3888,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":554,"record_offset":6296,"parent_record_offset":6272,"ancestor_record_offset":3888,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":554,"record_offset":5040,"parent_record_offset":5016,"ancestor_record_offset":3888,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":554,"record_offset":5336,"parent_record_offset":5312,"ancestor_record_offset":3888,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":554,"record_offset":5284,"parent_record_offset":5248,"ancestor_record_offset":3888,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":554,"record_offset":4264,"parent_record_offset":4240,"ancestor_record_offset":3888,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":554,"record_offset":5188,"parent_record_offset":5076,"ancestor_record_offset":3888,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":554,"record_offset":5016,"parent_record_offset":4992,"ancestor_record_offset":3888,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":554,"record_offset":5312,"parent_record_offset":5248,"ancestor_record_offset":3888,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":554,"record_offset":5248,"parent_record_offset":5220,"ancestor_record_offset":3888,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":554,"record_offset":5400,"parent_record_offset":5376,"ancestor_record_offset":3888,"inlinee":21321,"source_line":847,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":554,"record_offset":5460,"parent_record_offset":5432,"ancestor_record_offset":3888,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":554,"record_offset":5604,"parent_record_offset":5516,"ancestor_record_offset":3888,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":554,"record_offset":5488,"parent_record_offset":5460,"ancestor_record_offset":3888,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::buckets
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":554,"record_offset":5576,"parent_record_offset":5516,"ancestor_record_offset":3888,"inlinee":6423,"source_line":2806,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":554,"record_offset":5548,"parent_record_offset":5516,"ancestor_record_offset":3888,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":554,"record_offset":5516,"parent_record_offset":5488,"ancestor_record_offset":3888,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":554,"record_offset":5664,"parent_record_offset":5604,"ancestor_record_offset":3888,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":554,"record_offset":5720,"parent_record_offset":5664,"ancestor_record_offset":3888,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::with_hasher
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":554,"record_offset":7176,"parent_record_offset":6744,"ancestor_record_offset":3888,"inlinee":5553,"source_line":283,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":554,"record_offset":6792,"parent_record_offset":6768,"ancestor_record_offset":3888,"inlinee":5539,"source_line":55,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__getit
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":554,"record_offset":6868,"parent_record_offset":6840,"ancestor_record_offset":3888,"inlinee":5544,"source_line":85,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":554,"record_offset":6988,"parent_record_offset":6840,"ancestor_record_offset":3888,"inlinee":5550,"source_line":71,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":554,"record_offset":6768,"parent_record_offset":6744,"ancestor_record_offset":3888,"inlinee":5537,"source_line":151,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::fast_local::Key::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\fast_local.rs
//     evidence={"module":554,"record_offset":6892,"parent_record_offset":6868,"ancestor_record_offset":3888,"inlinee":5546,"source_line":165,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::lazy::LazyKeyInner::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\sys\thread_local\mod.rs
//     evidence={"module":554,"record_offset":6920,"parent_record_offset":6892,"ancestor_record_offset":3888,"inlinee":5548,"source_line":41,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":554,"record_offset":6840,"parent_record_offset":6816,"ancestor_record_offset":3888,"inlinee":5542,"source_line":278,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\thread\local.rs
//     evidence={"module":554,"record_offset":6816,"parent_record_offset":6792,"ancestor_record_offset":3888,"inlinee":5541,"source_line":256,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::create_descriptor_set_layouts::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 210
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x111D9
// 首条位置: {"module":1595,"inlinee":70105}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: core::ptr::drop_in_place<ash::vk::enums::Result>
// 记录形式: compiled_procedure; instances=1
// 源码行: 373
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":554,"record_offset":248,"rva":11552,"va":5368720672}
// 编译实例 RVAs: 0x2d20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::ComputePipelineDesc::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 373
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":554,"record_offset":7780,"rva":11552,"va":5368720672}
// 编译实例 RVAs: 0x2d20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::create_compute_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 377
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1332,"record_offset":15232,"rva":15318480,"va":5384027600}
// 编译实例 RVAs: 0xe9bdd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11278654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384027600,"raw_hex":"e8c2e653ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5384028345,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=7
//     target_aliases=1 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11278724","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384027600,"raw_hex":"e87ce653ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5384028431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406de6b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8117980","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384027600,"raw_hex":"e8242184ff","size":5,"soft_fail":false,"target_va":5375911600,"va":5384029575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406ecc20
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8059171","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384027600,"raw_hex":"e8dd0685ff","size":5,"soft_fail":false,"target_va":5375970336,"va":5384029502,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140869090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6500015","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384027600,"raw_hex":"e851d19cff","size":5,"soft_fail":false,"target_va":5377527952,"va":5384027962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ea0ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17584","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384027600,"raw_hex":"e8b0440000","size":5,"soft_fail":false,"target_va":5384047280,"va":5384029691,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18949200","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384027600,"raw_hex":"e850242101","size":5,"soft_fail":false,"target_va":5402978144,"va":5384028939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18949792","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384027600,"raw_hex":"e8a0262101","size":5,"soft_fail":false,"target_va":5402978912,"va":5384029115,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18950433","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384027600,"raw_hex":"e821292101","size":5,"soft_fail":false,"target_va":5402979152,"va":5384028714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18950682","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384027600,"raw_hex":"e81a2a2101","size":5,"soft_fail":false,"target_va":5402979344,"va":5384028657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18949850","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384027600,"raw_hex":"e8da262101","size":5,"soft_fail":false,"target_va":5402978944,"va":5384029089,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t18951348","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384027600,"raw_hex":"e8b42c2101","size":5,"soft_fail":false,"target_va":5402980128,"va":5384028775,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::shader::create_descriptor_set_layouts
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7806385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384027600,"raw_hex":"e84fe288ff","size":5,"soft_fail":false,"target_va":5376221312,"va":5384027692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::shader::create_descriptor_set_states
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7805720","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384027600,"raw_hex":"e8e8e488ff","size":5,"soft_fail":false,"target_va":5376222800,"va":5384028515,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5384027600,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5384027901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::ffi::c_str::CString::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\ffi\c_str.rs
//     evidence={"module":1332,"record_offset":15856,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":9051,"source_line":263,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1332,"record_offset":15760,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_compute_pipelines
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1332,"record_offset":16460,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":62188,"source_line":2178,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_pipeline_layout
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1332,"record_offset":15644,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":62149,"source_line":2219,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_shader_module
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1332,"record_offset":16260,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":61764,"source_line":2668,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::ComputePipelineCreateInfo::stage
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1332,"record_offset":16432,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":62187,"source_line":3955,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PipelineLayoutCreateInfo::set_layouts
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1332,"record_offset":15616,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":62148,"source_line":5320,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::ShaderModuleCreateInfo::code
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1332,"record_offset":16228,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":61763,"source_line":3467,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$189::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1332,"record_offset":16376,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":61766,"source_line":3843,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$194::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1332,"record_offset":16404,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":62185,"source_line":3931,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$276::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1332,"record_offset":15588,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":62146,"source_line":5291,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1332,"record_offset":17064,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1332,"record_offset":16924,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1332,"record_offset":15884,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1332,"record_offset":17328,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":6160,"source_line":1428,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1332,"record_offset":15532,"parent_record_offset":15232,"ancestor_record_offset":15232,"inlinee":21777,"source_line":588,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1332,"record_offset":16644,"parent_record_offset":16620,"ancestor_record_offset":15232,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1332,"record_offset":16668,"parent_record_offset":16644,"ancestor_record_offset":15232,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1332,"record_offset":16080,"parent_record_offset":16056,"ancestor_record_offset":15232,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1332,"record_offset":16620,"parent_record_offset":16592,"ancestor_record_offset":15232,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1332,"record_offset":16056,"parent_record_offset":16000,"ancestor_record_offset":15232,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1332,"record_offset":17176,"parent_record_offset":17152,"ancestor_record_offset":15232,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::impl$2::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\ffi\c_str.rs
//     evidence={"module":1332,"record_offset":17124,"parent_record_offset":17100,"ancestor_record_offset":15232,"inlinee":8650,"source_line":703,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1332,"record_offset":16028,"parent_record_offset":16000,"ancestor_record_offset":15232,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1332,"record_offset":16592,"parent_record_offset":16564,"ancestor_record_offset":15232,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1332,"record_offset":16564,"parent_record_offset":16536,"ancestor_record_offset":15232,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1332,"record_offset":16740,"parent_record_offset":16564,"ancestor_record_offset":15232,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1332,"record_offset":16000,"parent_record_offset":15972,"ancestor_record_offset":15232,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1332,"record_offset":18076,"parent_record_offset":18052,"ancestor_record_offset":15232,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1332,"record_offset":16508,"parent_record_offset":16460,"ancestor_record_offset":15232,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1332,"record_offset":16536,"parent_record_offset":16508,"ancestor_record_offset":15232,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1332,"record_offset":15672,"parent_record_offset":15644,"ancestor_record_offset":15232,"inlinee":11125,"source_line":24,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1332,"record_offset":15696,"parent_record_offset":15672,"ancestor_record_offset":15232,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1332,"record_offset":15720,"parent_record_offset":15696,"ancestor_record_offset":15232,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1332,"record_offset":17388,"parent_record_offset":17360,"ancestor_record_offset":15232,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1332,"record_offset":17556,"parent_record_offset":17524,"ancestor_record_offset":15232,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1332,"record_offset":15924,"parent_record_offset":15884,"ancestor_record_offset":15232,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1332,"record_offset":16696,"parent_record_offset":16668,"ancestor_record_offset":15232,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1332,"record_offset":15788,"parent_record_offset":15760,"ancestor_record_offset":15232,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1332,"record_offset":15816,"parent_record_offset":15788,"ancestor_record_offset":15232,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::fetch_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1332,"record_offset":18112,"parent_record_offset":18076,"ancestor_record_offset":15232,"inlinee":4708,"source_line":2695,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1332,"record_offset":18136,"parent_record_offset":18112,"ancestor_record_offset":15232,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1332,"record_offset":18168,"parent_record_offset":18076,"ancestor_record_offset":15232,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1332,"record_offset":15556,"parent_record_offset":15532,"ancestor_record_offset":15232,"inlinee":21321,"source_line":847,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::impl$5::create_compute_pipeline::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1332,"record_offset":17360,"parent_record_offset":17328,"ancestor_record_offset":15232,"inlinee":62190,"source_line":442,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::impl$5::create_compute_pipeline::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 442
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF2EE
// 首条位置: {"module":1332,"inlinee":62190}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1332,"record_offset":17388,"parent_record_offset":17360,"ancestor_record_offset":17360,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1332,"record_offset":17416,"parent_record_offset":17360,"ancestor_record_offset":17360,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1332,"record_offset":17616,"parent_record_offset":17592,"ancestor_record_offset":17360,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1332,"record_offset":17592,"parent_record_offset":17492,"ancestor_record_offset":17360,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1332,"record_offset":17524,"parent_record_offset":17492,"ancestor_record_offset":17360,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1332,"record_offset":17492,"parent_record_offset":17464,"ancestor_record_offset":17360,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1332,"record_offset":17556,"parent_record_offset":17524,"ancestor_record_offset":17360,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1332,"record_offset":17440,"parent_record_offset":17416,"ancestor_record_offset":17360,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::create_descriptor_set_states
// 记录形式: compiled_procedure; instances=1
// 源码行: 462
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":554,"record_offset":7904,"rva":7513680,"va":5376222800}
// 编译实例 RVAs: 0x72a650
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14117aff0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10815786","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376222800,"raw_hex":"e82a09a500","size":5,"soft_fail":false,"target_va":5387038704,"va":5376222913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":8048,"parent_record_offset":7904,"ancestor_record_offset":7904,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":554,"record_offset":8268,"parent_record_offset":7904,"ancestor_record_offset":7904,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":554,"record_offset":8128,"parent_record_offset":7904,"ancestor_record_offset":7904,"inlinee":4239,"source_line":599,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":8292,"parent_record_offset":8268,"ancestor_record_offset":7904,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":554,"record_offset":8152,"parent_record_offset":8128,"ancestor_record_offset":7904,"inlinee":4248,"source_line":2920,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":554,"record_offset":8208,"parent_record_offset":8128,"ancestor_record_offset":7904,"inlinee":4241,"source_line":23,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\zip.rs
//     evidence={"module":554,"record_offset":8232,"parent_record_offset":8208,"ancestor_record_offset":7904,"inlinee":11139,"source_line":154,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":554,"record_offset":8316,"parent_record_offset":8292,"ancestor_record_offset":7904,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\manually_drop.rs
//     evidence={"module":554,"record_offset":8072,"parent_record_offset":8048,"ancestor_record_offset":7904,"inlinee":4174,"source_line":70,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":554,"record_offset":8096,"parent_record_offset":8048,"ancestor_record_offset":7904,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00554.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::create_descriptor_set_states::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 469
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18183
// 首条位置: {"module":3007,"inlinee":98691}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::sync::impl$52::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3007,"record_offset":10932,"parent_record_offset":9980,"ancestor_record_offset":9980,"inlinee":5311,"source_line":3285,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3007,"record_offset":10072,"parent_record_offset":9980,"ancestor_record_offset":9980,"inlinee":4175,"source_line":422,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3007,"record_offset":11180,"parent_record_offset":9980,"ancestor_record_offset":9980,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3007,"record_offset":11460,"parent_record_offset":9980,"ancestor_record_offset":9980,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$189::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3007,"record_offset":11308,"parent_record_offset":9980,"ancestor_record_offset":9980,"inlinee":13747,"source_line":296,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3007,"record_offset":10100,"parent_record_offset":9980,"ancestor_record_offset":9980,"inlinee":22371,"source_line":423,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$53::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3007,"record_offset":10428,"parent_record_offset":9980,"ancestor_record_offset":9980,"inlinee":22373,"source_line":2392,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3007,"record_offset":11072,"parent_record_offset":11048,"ancestor_record_offset":9980,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3007,"record_offset":11096,"parent_record_offset":11072,"ancestor_record_offset":9980,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3007,"record_offset":11636,"parent_record_offset":11612,"ancestor_record_offset":9980,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3007,"record_offset":11016,"parent_record_offset":10988,"ancestor_record_offset":9980,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3007,"record_offset":11048,"parent_record_offset":11016,"ancestor_record_offset":9980,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":3007,"record_offset":11612,"parent_record_offset":11512,"ancestor_record_offset":9980,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":3007,"record_offset":10988,"parent_record_offset":10960,"ancestor_record_offset":9980,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3007,"record_offset":11544,"parent_record_offset":11512,"ancestor_record_offset":9980,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3007,"record_offset":11272,"parent_record_offset":11248,"ancestor_record_offset":9980,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":3007,"record_offset":11512,"parent_record_offset":11484,"ancestor_record_offset":9980,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":3007,"record_offset":10960,"parent_record_offset":10932,"ancestor_record_offset":9980,"inlinee":5312,"source_line":392,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":3007,"record_offset":11248,"parent_record_offset":11180,"ancestor_record_offset":9980,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::impl$329::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":3007,"record_offset":11424,"parent_record_offset":11400,"ancestor_record_offset":9980,"inlinee":31066,"source_line":174,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3007,"record_offset":10280,"parent_record_offset":10252,"ancestor_record_offset":9980,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":3007,"record_offset":10340,"parent_record_offset":10308,"ancestor_record_offset":9980,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":3007,"record_offset":11576,"parent_record_offset":11544,"ancestor_record_offset":9980,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":3007,"record_offset":10376,"parent_record_offset":10224,"ancestor_record_offset":9980,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3007,"record_offset":11484,"parent_record_offset":11460,"ancestor_record_offset":9980,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3007,"record_offset":10848,"parent_record_offset":10824,"ancestor_record_offset":9980,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":3007,"record_offset":10824,"parent_record_offset":10800,"ancestor_record_offset":9980,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":3007,"record_offset":11368,"parent_record_offset":11336,"ancestor_record_offset":9980,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3007,"record_offset":11124,"parent_record_offset":11096,"ancestor_record_offset":9980,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":3007,"record_offset":11220,"parent_record_offset":11180,"ancestor_record_offset":9980,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$189::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":3007,"record_offset":11336,"parent_record_offset":11308,"ancestor_record_offset":9980,"inlinee":6288,"source_line":156,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3007,"record_offset":10148,"parent_record_offset":10124,"ancestor_record_offset":9980,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":3007,"record_offset":10492,"parent_record_offset":10460,"ancestor_record_offset":9980,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3007,"record_offset":10800,"parent_record_offset":10564,"ancestor_record_offset":9980,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3007,"record_offset":10224,"parent_record_offset":10196,"ancestor_record_offset":9980,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3007,"record_offset":10564,"parent_record_offset":10524,"ancestor_record_offset":9980,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3007,"record_offset":10172,"parent_record_offset":10148,"ancestor_record_offset":9980,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3007,"record_offset":10196,"parent_record_offset":10172,"ancestor_record_offset":9980,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3007,"record_offset":10632,"parent_record_offset":10604,"ancestor_record_offset":9980,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":3007,"record_offset":10604,"parent_record_offset":10564,"ancestor_record_offset":9980,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":3007,"record_offset":10524,"parent_record_offset":10492,"ancestor_record_offset":9980,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3007,"record_offset":10252,"parent_record_offset":10224,"ancestor_record_offset":9980,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":3007,"record_offset":10308,"parent_record_offset":10252,"ancestor_record_offset":9980,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::create_descriptor_set_states::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":3007,"record_offset":11400,"parent_record_offset":11308,"ancestor_record_offset":9980,"inlinee":98693,"source_line":475,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3007,"record_offset":10124,"parent_record_offset":10100,"ancestor_record_offset":9980,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":3007,"record_offset":10460,"parent_record_offset":10428,"ancestor_record_offset":9980,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::create_descriptor_set_states::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 475
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x18185
// 首条位置: {"module":3007,"inlinee":98693}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::enums::impl$329::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":3007,"record_offset":11424,"parent_record_offset":11400,"ancestor_record_offset":11400,"inlinee":31066,"source_line":174,"archive":"_pdb/A/modules/03007.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::impl$118::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 610
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF7C2
// 首条位置: {"module":1370,"inlinee":63426}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":1370,"record_offset":40736,"parent_record_offset":40712,"ancestor_record_offset":40712,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":1370,"record_offset":40760,"parent_record_offset":40736,"ancestor_record_offset":40712,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::impl$120::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 610
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF260
// 首条位置: {"module":1329,"inlinee":62048}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::array::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\mod.rs
//     evidence={"module":1329,"record_offset":33832,"parent_record_offset":33808,"ancestor_record_offset":33808,"inlinee":15040,"source_line":307,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1329,"record_offset":33884,"parent_record_offset":33856,"ancestor_record_offset":33808,"inlinee":15043,"source_line":486,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1329,"record_offset":33908,"parent_record_offset":33884,"ancestor_record_offset":33808,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1329,"record_offset":33856,"parent_record_offset":33832,"ancestor_record_offset":33808,"inlinee":15042,"source_line":932,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$9::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1329,"record_offset":33972,"parent_record_offset":33856,"ancestor_record_offset":33808,"inlinee":15045,"source_line":811,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1329,"record_offset":33932,"parent_record_offset":33908,"ancestor_record_offset":33808,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_topology
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 612
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x57FC
// 首条位置: {"module":394,"inlinee":22524}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_topology_checked
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":394,"record_offset":804,"parent_record_offset":784,"ancestor_record_offset":784,"inlinee":22525,"source_line":612,"archive":"_pdb/A/modules/00394.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":394,"record_offset":824,"parent_record_offset":804,"ancestor_record_offset":784,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00394.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_topology_checked
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 612
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x57FD
// 首条位置: {"module":394,"inlinee":22525}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":394,"record_offset":824,"parent_record_offset":804,"ancestor_record_offset":804,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00394.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::topology
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 612
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF2CB
// 首条位置: {"module":1332,"inlinee":62155}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1332,"record_offset":10648,"parent_record_offset":10536,"ancestor_record_offset":10536,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::topology_or_err
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1332,"record_offset":10564,"parent_record_offset":10536,"ancestor_record_offset":10536,"inlinee":62156,"source_line":612,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::read_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":1332,"record_offset":10588,"parent_record_offset":10564,"ancestor_record_offset":10536,"inlinee":62157,"source_line":23,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::push_pop::impl$14::push_bits
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/push_pop.rs
//     evidence={"module":1332,"record_offset":10612,"parent_record_offset":10588,"ancestor_record_offset":10536,"inlinee":62159,"source_line":85,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::topology_or_err
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 612
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF2CC
// 首条位置: {"module":1332,"inlinee":62156}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] modular_bitfield::private::proc::read_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":1332,"record_offset":10588,"parent_record_offset":10564,"ancestor_record_offset":10564,"inlinee":62157,"source_line":23,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::push_pop::impl$14::push_bits
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/push_pop.rs
//     evidence={"module":1332,"record_offset":10612,"parent_record_offset":10588,"ancestor_record_offset":10564,"inlinee":62159,"source_line":85,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::cull_mode
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 613
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF2D5
// 首条位置: {"module":1332,"inlinee":62165}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::cull_mode_or_err
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1332,"record_offset":10796,"parent_record_offset":10772,"ancestor_record_offset":10772,"inlinee":62166,"source_line":613,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::read_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":1332,"record_offset":10820,"parent_record_offset":10796,"ancestor_record_offset":10772,"inlinee":62157,"source_line":23,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::push_pop::impl$14::push_bits
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/push_pop.rs
//     evidence={"module":1332,"record_offset":10844,"parent_record_offset":10820,"ancestor_record_offset":10772,"inlinee":62159,"source_line":85,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::cull_mode_or_err
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 613
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF2D6
// 首条位置: {"module":1332,"inlinee":62166}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] modular_bitfield::private::proc::read_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":1332,"record_offset":10820,"parent_record_offset":10796,"ancestor_record_offset":10796,"inlinee":62157,"source_line":23,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::push_pop::impl$14::push_bits
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/push_pop.rs
//     evidence={"module":1332,"record_offset":10844,"parent_record_offset":10820,"ancestor_record_offset":10796,"inlinee":62159,"source_line":85,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 613
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x55D1
// 首条位置: {"module":388,"inlinee":21969}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode_checked
//     owner=private_source_ownership_candidate; count=11
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":388,"record_offset":24188,"parent_record_offset":24164,"ancestor_record_offset":24164,"inlinee":21970,"source_line":613,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":388,"record_offset":24212,"parent_record_offset":24188,"ancestor_record_offset":24164,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode_checked
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 613
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x55D2
// 首条位置: {"module":388,"inlinee":21970}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":388,"record_offset":24212,"parent_record_offset":24188,"ancestor_record_offset":24188,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::depth_compare_mode
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 615
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF2D7
// 首条位置: {"module":1332,"inlinee":62167}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1332,"record_offset":10996,"parent_record_offset":10884,"ancestor_record_offset":10884,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::depth_compare_mode_or_err
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1332,"record_offset":10912,"parent_record_offset":10884,"ancestor_record_offset":10884,"inlinee":62168,"source_line":615,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::read_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":1332,"record_offset":10936,"parent_record_offset":10912,"ancestor_record_offset":10884,"inlinee":62157,"source_line":23,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::push_pop::impl$14::push_bits
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/push_pop.rs
//     evidence={"module":1332,"record_offset":10960,"parent_record_offset":10936,"ancestor_record_offset":10884,"inlinee":62159,"source_line":85,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::depth_compare_mode_or_err
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 615
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF2D8
// 首条位置: {"module":1332,"inlinee":62168}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] modular_bitfield::private::proc::read_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":1332,"record_offset":10936,"parent_record_offset":10912,"ancestor_record_offset":10912,"inlinee":62157,"source_line":23,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::push_pop::impl$14::push_bits
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/push_pop.rs
//     evidence={"module":1332,"record_offset":10960,"parent_record_offset":10936,"ancestor_record_offset":10912,"inlinee":62159,"source_line":85,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_depth_compare_mode
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 615
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x64C3
// 首条位置: {"module":449,"inlinee":25795}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_compare_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":449,"record_offset":13016,"parent_record_offset":12992,"ancestor_record_offset":12992,"inlinee":25796,"source_line":615,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":449,"record_offset":13040,"parent_record_offset":13016,"ancestor_record_offset":12992,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_depth_compare_mode_checked
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 615
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x64C4
// 首条位置: {"module":449,"inlinee":25796}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":449,"record_offset":13040,"parent_record_offset":13016,"ancestor_record_offset":13016,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00449.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_depth_write
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 616
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x633D
// 首条位置: {"module":441,"inlinee":25405}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_write_checked
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":441,"record_offset":18984,"parent_record_offset":18960,"ancestor_record_offset":18960,"inlinee":25406,"source_line":616,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":441,"record_offset":19008,"parent_record_offset":18984,"ancestor_record_offset":18960,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_depth_write_checked
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 616
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x633E
// 首条位置: {"module":441,"inlinee":25406}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":441,"record_offset":19008,"parent_record_offset":18984,"ancestor_record_offset":18984,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable
// 记录形式: inline_instance, inlinee_declaration; instances=29
// 源码行: 617
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CA8
// 首条位置: {"module":39,"inlinee":7336}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable_checked
//     owner=private_source_ownership_candidate; count=20
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":39,"record_offset":22340,"parent_record_offset":22316,"ancestor_record_offset":22316,"inlinee":7337,"source_line":617,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":39,"record_offset":22364,"parent_record_offset":22340,"ancestor_record_offset":22316,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable_checked
// 记录形式: inline_instance, inlinee_declaration; instances=29
// 源码行: 617
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CA9
// 首条位置: {"module":39,"inlinee":7337}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":39,"record_offset":22364,"parent_record_offset":22340,"ancestor_record_offset":22340,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_test_enable
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 619
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x55E8
// 首条位置: {"module":388,"inlinee":21992}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_test_enable_checked
//     owner=private_source_ownership_candidate; count=9
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":388,"record_offset":32724,"parent_record_offset":32700,"ancestor_record_offset":32700,"inlinee":21993,"source_line":619,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":388,"record_offset":32748,"parent_record_offset":32724,"ancestor_record_offset":32700,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_test_enable_checked
// 记录形式: inline_instance, inlinee_declaration; instances=11
// 源码行: 619
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x55E9
// 首条位置: {"module":388,"inlinee":21993}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":388,"record_offset":32748,"parent_record_offset":32724,"ancestor_record_offset":32724,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::stencil_test_enable
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 619
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF2D9
// 首条位置: {"module":1332,"inlinee":62169}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1385,"record_offset":38160,"parent_record_offset":38136,"ancestor_record_offset":38136,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::stencil_test_enable_or_err
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1332,"record_offset":11060,"parent_record_offset":11036,"ancestor_record_offset":11036,"inlinee":62170,"source_line":619,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::read_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":1332,"record_offset":11084,"parent_record_offset":11060,"ancestor_record_offset":11036,"inlinee":62157,"source_line":23,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::push_pop::impl$14::push_bits
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/push_pop.rs
//     evidence={"module":1332,"record_offset":11108,"parent_record_offset":11084,"ancestor_record_offset":11036,"inlinee":62159,"source_line":85,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::stencil_test_enable_or_err
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 619
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF2DA
// 首条位置: {"module":1332,"inlinee":62170}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] modular_bitfield::private::proc::read_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":1332,"record_offset":11084,"parent_record_offset":11060,"ancestor_record_offset":11060,"inlinee":62157,"source_line":23,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::push_pop::impl$14::push_bits
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/push_pop.rs
//     evidence={"module":1332,"record_offset":11108,"parent_record_offset":11084,"ancestor_record_offset":11060,"inlinee":62159,"source_line":85,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 621
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x55EA
// 首条位置: {"module":388,"inlinee":21994}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op_checked
//     owner=private_source_ownership_candidate; count=8
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":388,"record_offset":32808,"parent_record_offset":32784,"ancestor_record_offset":32784,"inlinee":21995,"source_line":621,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":388,"record_offset":32832,"parent_record_offset":32808,"ancestor_record_offset":32784,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op_checked
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 621
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x55EB
// 首条位置: {"module":388,"inlinee":21995}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":388,"record_offset":32832,"parent_record_offset":32808,"ancestor_record_offset":32808,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00388.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_compare_op
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 623
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6340
// 首条位置: {"module":441,"inlinee":25408}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_compare_op_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":441,"record_offset":19960,"parent_record_offset":19936,"ancestor_record_offset":19936,"inlinee":25409,"source_line":623,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":441,"record_offset":19984,"parent_record_offset":19960,"ancestor_record_offset":19936,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_compare_op_checked
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 623
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6341
// 首条位置: {"module":441,"inlinee":25409}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":441,"record_offset":19984,"parent_record_offset":19960,"ancestor_record_offset":19960,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_compare_mask
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 624
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6342
// 首条位置: {"module":441,"inlinee":25410}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_stencil_compare_mask_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":441,"record_offset":20044,"parent_record_offset":20020,"ancestor_record_offset":20020,"inlinee":25411,"source_line":624,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::checked_shr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":441,"record_offset":20124,"parent_record_offset":20100,"ancestor_record_offset":20020,"inlinee":25414,"source_line":1326,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":441,"record_offset":20068,"parent_record_offset":20044,"ancestor_record_offset":20020,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::push_pop::impl$2::pop_bits
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/push_pop.rs
//     evidence={"module":441,"record_offset":20100,"parent_record_offset":20068,"ancestor_record_offset":20020,"inlinee":25413,"source_line":24,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_compare_mask_checked
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 624
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6343
// 首条位置: {"module":441,"inlinee":25411}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":441,"record_offset":20068,"parent_record_offset":20044,"ancestor_record_offset":20044,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$6::checked_shr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":441,"record_offset":20124,"parent_record_offset":20100,"ancestor_record_offset":20044,"inlinee":25414,"source_line":1326,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::push_pop::impl$2::pop_bits
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/push_pop.rs
//     evidence={"module":441,"record_offset":20100,"parent_record_offset":20068,"ancestor_record_offset":20044,"inlinee":25413,"source_line":24,"archive":"_pdb/A/modules/00441.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
// 记录形式: inline_instance, inlinee_declaration; instances=34
// 源码行: 626
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CAC
// 首条位置: {"module":39,"inlinee":7340}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
//     owner=private_source_ownership_candidate; count=23
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":39,"record_offset":22424,"parent_record_offset":22400,"ancestor_record_offset":22400,"inlinee":7341,"source_line":626,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=23
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":39,"record_offset":22448,"parent_record_offset":22424,"ancestor_record_offset":22400,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
// 记录形式: inline_instance, inlinee_declaration; instances=34
// 源码行: 626
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1CAD
// 首条位置: {"module":39,"inlinee":7341}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=23
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":39,"record_offset":22448,"parent_record_offset":22424,"ancestor_record_offset":22424,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00039.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::polygon_mode
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 627
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xFCCA
// 首条位置: {"module":1385,"inlinee":64714}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1385,"record_offset":38284,"parent_record_offset":38260,"ancestor_record_offset":38260,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01385.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_polygon_mode
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 627
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B28
// 首条位置: {"module":487,"inlinee":27432}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::set_polygon_mode_checked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":487,"record_offset":28664,"parent_record_offset":28640,"ancestor_record_offset":28640,"inlinee":27433,"source_line":627,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":487,"record_offset":28688,"parent_record_offset":28664,"ancestor_record_offset":28640,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineOpts::set_polygon_mode_checked
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 627
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6B29
// 首条位置: {"module":487,"inlinee":27433}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] modular_bitfield::private::proc::write_specifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":487,"record_offset":28688,"parent_record_offset":28664,"ancestor_record_offset":28664,"inlinee":7339,"source_line":64,"archive":"_pdb/A/modules/00487.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::impl$7::default
// 记录形式: compiled_procedure; instances=1
// 源码行: 646
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":554,"record_offset":8356,"rva":7513824,"va":5376222944}
// 编译实例 RVAs: 0x72a6e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::impl$123::eq
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 649
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF7C0
// 首条位置: {"module":1370,"inlinee":63424}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$12::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1370,"record_offset":40684,"parent_record_offset":40656,"ancestor_record_offset":40656,"inlinee":62068,"source_line":194,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::impl$118::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1370,"record_offset":40712,"parent_record_offset":40656,"ancestor_record_offset":40656,"inlinee":63426,"source_line":610,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":1370,"record_offset":40736,"parent_record_offset":40712,"ancestor_record_offset":40656,"inlinee":4535,"source_line":10,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_transitive] core::array::equality::impl$9::spec_eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\equality.rs
//     evidence={"module":1370,"record_offset":40760,"parent_record_offset":40736,"ancestor_record_offset":40656,"inlinee":6198,"source_line":147,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::impl$125::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 649
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF254
// 首条位置: {"module":1329,"inlinee":62036}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$13::hash
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1329,"record_offset":33264,"parent_record_offset":33236,"ancestor_record_offset":33236,"inlinee":62038,"source_line":200,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::impl$120::hash
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1329,"record_offset":33808,"parent_record_offset":33236,"ancestor_record_offset":33236,"inlinee":62048,"source_line":610,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::array::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\array\mod.rs
//     evidence={"module":1329,"record_offset":33832,"parent_record_offset":33808,"ancestor_record_offset":33236,"inlinee":15040,"source_line":307,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_length_prefix
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1329,"record_offset":33884,"parent_record_offset":33856,"ancestor_record_offset":33236,"inlinee":15043,"source_line":486,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u16
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1329,"record_offset":33676,"parent_record_offset":33652,"ancestor_record_offset":33236,"inlinee":15568,"source_line":371,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1329,"record_offset":33368,"parent_record_offset":33344,"ancestor_record_offset":33236,"inlinee":8446,"source_line":377,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u8
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1329,"record_offset":33536,"parent_record_offset":33512,"ancestor_record_offset":33236,"inlinee":8453,"source_line":365,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1329,"record_offset":33908,"parent_record_offset":33884,"ancestor_record_offset":33236,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$10::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1329,"record_offset":33652,"parent_record_offset":33628,"ancestor_record_offset":33236,"inlinee":15030,"source_line":806,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1329,"record_offset":33344,"parent_record_offset":33320,"ancestor_record_offset":33236,"inlinee":8445,"source_line":806,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$4::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1329,"record_offset":33856,"parent_record_offset":33832,"ancestor_record_offset":33236,"inlinee":15042,"source_line":932,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$9::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1329,"record_offset":33512,"parent_record_offset":33492,"ancestor_record_offset":33236,"inlinee":17875,"source_line":806,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$9::hash_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1329,"record_offset":33972,"parent_record_offset":33856,"ancestor_record_offset":33236,"inlinee":15045,"source_line":811,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":1329,"record_offset":33448,"parent_record_offset":33424,"ancestor_record_offset":33236,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1329,"record_offset":33932,"parent_record_offset":33908,"ancestor_record_offset":33236,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1329,"record_offset":33700,"parent_record_offset":33676,"ancestor_record_offset":33236,"inlinee":15569,"source_line":2859,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1329,"record_offset":33392,"parent_record_offset":33368,"ancestor_record_offset":33236,"inlinee":8447,"source_line":2859,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$37::hash
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1329,"record_offset":33628,"parent_record_offset":33288,"ancestor_record_offset":33236,"inlinee":62046,"source_line":14,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$45::hash
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1329,"record_offset":33320,"parent_record_offset":33288,"ancestor_record_offset":33236,"inlinee":62042,"source_line":23,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$52::hash
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1329,"record_offset":33492,"parent_record_offset":33288,"ancestor_record_offset":33236,"inlinee":62044,"source_line":32,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$9::hash
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1329,"record_offset":33288,"parent_record_offset":33264,"ancestor_record_offset":33236,"inlinee":62040,"source_line":171,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1329,"record_offset":33424,"parent_record_offset":33368,"ancestor_record_offset":33236,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RasterPipelineDesc::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 656
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":554,"record_offset":8472,"rva":7513840,"va":5376222960}
// 编译实例 RVAs: 0x72a6f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::impl$130::eq
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 670
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x57CD
// 首条位置: {"module":393,"inlinee":22477}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::enums::impl$266::eq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1370,"record_offset":35280,"parent_record_offset":35224,"ancestor_record_offset":35224,"inlinee":22479,"source_line":36,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::enums::impl$275::eq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1370,"record_offset":35308,"parent_record_offset":35224,"ancestor_record_offset":35224,"inlinee":22481,"source_line":55,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::enums::impl$527::eq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1370,"record_offset":35252,"parent_record_offset":35224,"ancestor_record_offset":35224,"inlinee":8319,"source_line":649,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::impl$24::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":1370,"record_offset":35732,"parent_record_offset":35616,"ancestor_record_offset":35616,"inlinee":8321,"source_line":49,"archive":"_pdb/A/modules/01370.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::impl$132::hash
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 670
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x57B9
// 首条位置: {"module":393,"inlinee":22457}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::enums::impl$270::hash
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1329,"record_offset":46456,"parent_record_offset":46220,"ancestor_record_offset":46220,"inlinee":22461,"source_line":36,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::enums::impl$279::hash
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1329,"record_offset":46656,"parent_record_offset":46220,"ancestor_record_offset":46220,"inlinee":22463,"source_line":55,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::enums::impl$531::hash
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/enums.rs
//     evidence={"module":1329,"record_offset":46256,"parent_record_offset":46220,"ancestor_record_offset":46220,"inlinee":22459,"source_line":649,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::texture::impl$22::hash
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/texture.rs
//     evidence={"module":1329,"record_offset":46856,"parent_record_offset":46220,"ancestor_record_offset":46220,"inlinee":22465,"source_line":49,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_i32
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1329,"record_offset":46304,"parent_record_offset":46280,"ancestor_record_offset":46220,"inlinee":8469,"source_line":414,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1329,"record_offset":46904,"parent_record_offset":46880,"ancestor_record_offset":46220,"inlinee":8460,"source_line":432,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1329,"record_offset":46328,"parent_record_offset":46304,"ancestor_record_offset":46220,"inlinee":8446,"source_line":377,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1329,"record_offset":46928,"parent_record_offset":46904,"ancestor_record_offset":46220,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$16::hash
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1329,"record_offset":46280,"parent_record_offset":46256,"ancestor_record_offset":46220,"inlinee":8468,"source_line":806,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1329,"record_offset":46880,"parent_record_offset":46856,"ancestor_record_offset":46220,"inlinee":8459,"source_line":806,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":1329,"record_offset":46408,"parent_record_offset":46384,"ancestor_record_offset":46220,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1329,"record_offset":46952,"parent_record_offset":46928,"ancestor_record_offset":46220,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1329,"record_offset":46352,"parent_record_offset":46328,"ancestor_record_offset":46220,"inlinee":8447,"source_line":2859,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1329,"record_offset":46384,"parent_record_offset":46328,"ancestor_record_offset":46220,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RenderPassAttachmentDesc::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 679
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":554,"record_offset":8596,"rva":7513872,"va":5376222992}
// 编译实例 RVAs: 0x72a710
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RenderPassAttachmentDesc::clear_input
// 记录形式: compiled_procedure; instances=1
// 源码行: 693
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":554,"record_offset":8724,"rva":7513904,"va":5376223024}
// 编译实例 RVAs: 0x72a730
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::RenderPassAttachmentDesc::sample_count
// 记录形式: compiled_procedure; instances=1
// 源码行: 703
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":554,"record_offset":8860,"rva":7513936,"va":5376223056}
// 编译实例 RVAs: 0x72a750
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 751
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1332,"record_offset":7184,"rva":15313408,"va":5384022528}
// 编译实例 RVAs: 0xe9aa00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11273237","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384022528,"raw_hex":"e8ebfb53ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5384022928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=12
//     target_aliases=1 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11275831","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384022528,"raw_hex":"e8c9f153ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5384025538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406d9f30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8132551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384022528,"raw_hex":"e839e883ff","size":5,"soft_fail":false,"target_va":5375893296,"va":5384025842,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406de6b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8115832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384022528,"raw_hex":"e8882984ff","size":5,"soft_fail":false,"target_va":5375911600,"va":5384027427,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406f09b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8040582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384022528,"raw_hex":"e87a4f85ff","size":5,"soft_fail":false,"target_va":5375986096,"va":5384026673,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d80fc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1154125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384022528,"raw_hex":"e8b363eeff","size":5,"soft_fail":false,"target_va":5382868928,"va":5384023048,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140ea0ab0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19698","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384022528,"raw_hex":"e8f24c0000","size":5,"soft_fail":false,"target_va":5384047280,"va":5384027577,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9940
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18931922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384022528,"raw_hex":"e8d2e02001","size":5,"soft_fail":false,"target_va":5402958144,"va":5384026217,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1420a9b00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18932430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384022528,"raw_hex":"e8cee22001","size":5,"soft_fail":false,"target_va":5402958592,"va":5384026157,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: shader.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t19570387","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384022528,"raw_hex":"e8d39e2a01","size":5,"soft_fail":false,"target_va":5403593360,"va":5384022968,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t18952042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384022528,"raw_hex":"e86a2f2101","size":5,"soft_fail":false,"target_va":5402978144,"va":5384026097,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::RawVec$LT$T$C$A$GT$::reserve_for_push
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t-4479481","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384022528,"raw_hex":"e807a6bbff","size":5,"soft_fail":false,"target_va":5379544224,"va":5384023700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t-6499260","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384022528,"raw_hex":"e844d49cff","size":5,"soft_fail":false,"target_va":5377527120,"va":5384026375,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18953011","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384022528,"raw_hex":"e833332101","size":5,"soft_fail":false,"target_va":5402978912,"va":5384025896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18952634","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384022528,"raw_hex":"e8ba312101","size":5,"soft_fail":false,"target_va":5402978880,"va":5384026241,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t18952930","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384022528,"raw_hex":"e8e2322101","size":5,"soft_fail":false,"target_va":5402978944,"va":5384026009,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t18954046","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384022528,"raw_hex":"e83e372101","size":5,"soft_fail":false,"target_va":5402980128,"va":5384026077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::shader::create_descriptor_set_layouts
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7801302","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384022528,"raw_hex":"e82af688ff","size":5,"soft_fail":false,"target_va":5376221312,"va":5384022609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::shader::create_descriptor_set_states
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7802838","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384022528,"raw_hex":"e82af088ff","size":5,"soft_fail":false,"target_va":5376222800,"va":5384025633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::shader::get_vk_blend_mode
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7801526","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384022528,"raw_hex":"e84af588ff","size":5,"soft_fail":false,"target_va":5376223104,"va":5384024625,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::shader::map_vertex_attribute_format
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-7800395","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384022528,"raw_hex":"e8b5f988ff","size":5,"soft_fail":false,"target_va":5376223200,"va":5384023590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] shader_pipeline::reflection_ty::ScalarType::nbyte
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t9995007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384022528,"raw_hex":"e8ff829800","size":5,"soft_fail":false,"target_va":5394018576,"va":5384023564,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] shader_pipeline::reflection_ty::Type::nbyte
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t9996586","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384022528,"raw_hex":"e82a899800","size":5,"soft_fail":false,"target_va":5394020240,"va":5384023649,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5384022528,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5384022873,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1332,"record_offset":9348,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1332,"record_offset":8268,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1332,"record_offset":11148,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":4220,"source_line":2685,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_graphics_pipelines
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1332,"record_offset":12000,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":62183,"source_line":2150,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_pipeline_layout
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1332,"record_offset":7612,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":62149,"source_line":2219,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::GraphicsPipelineCreateInfo::stages
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1332,"record_offset":11972,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":62182,"source_line":5033,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PipelineColorBlendStateCreateInfo::attachments
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1332,"record_offset":11860,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":62174,"source_line":4746,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PipelineDynamicStateCreateInfo::dynamic_states
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1332,"record_offset":11916,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":62178,"source_line":4812,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PipelineLayoutCreateInfo::set_layouts
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1332,"record_offset":7584,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":62148,"source_line":5320,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PipelineVertexInputStateCreateInfo::vertex_attribute_descriptions
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1332,"record_offset":10480,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":62153,"source_line":4194,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PipelineVertexInputStateCreateInfo::vertex_binding_descriptions
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1332,"record_offset":10508,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":62154,"source_line":4185,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PipelineViewportStateCreateInfo::scissor_count
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1332,"record_offset":10744,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":62164,"source_line":4381,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PipelineViewportStateCreateInfo::viewport_count
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1332,"record_offset":10716,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":62163,"source_line":4370,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$216::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1332,"record_offset":10452,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":62151,"source_line":4161,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$231::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1332,"record_offset":10688,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":62161,"source_line":4346,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$247::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1332,"record_offset":11832,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":62172,"source_line":4711,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$252::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1332,"record_offset":11888,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":62176,"source_line":4791,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$263::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1332,"record_offset":11944,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":62180,"source_line":4997,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$276::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1332,"record_offset":7556,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":62146,"source_line":5291,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1332,"record_offset":13516,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1332,"record_offset":8184,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1332,"record_offset":10112,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1332,"record_offset":10416,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1332,"record_offset":12464,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1332,"record_offset":13572,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1332,"record_offset":13544,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1332,"record_offset":8088,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1332,"record_offset":10284,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":7658,"source_line":314,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::cull_mode
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1332,"record_offset":10772,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":62165,"source_line":613,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::depth_compare_mode
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1332,"record_offset":10884,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":62167,"source_line":615,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::stencil_test_enable
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1332,"record_offset":11036,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":62169,"source_line":619,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::shader::RasterPipelineOpts::topology
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1332,"record_offset":10536,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":62155,"source_line":612,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::temp_list::TempList::new
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/temp_list.rs
//     evidence={"module":1332,"record_offset":7728,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":23788,"source_line":26,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::values
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1332,"record_offset":8972,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":22371,"source_line":423,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::impl$53::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1332,"record_offset":9488,"parent_record_offset":7184,"ancestor_record_offset":7184,"inlinee":22373,"source_line":2392,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1332,"record_offset":7980,"parent_record_offset":7956,"ancestor_record_offset":7184,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1332,"record_offset":8004,"parent_record_offset":7980,"ancestor_record_offset":7184,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1332,"record_offset":12416,"parent_record_offset":12392,"ancestor_record_offset":7184,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::exchange_malloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1332,"record_offset":7924,"parent_record_offset":7888,"ancestor_record_offset":7184,"inlinee":4331,"source_line":330,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1332,"record_offset":7956,"parent_record_offset":7924,"ancestor_record_offset":7184,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1332,"record_offset":12392,"parent_record_offset":12368,"ancestor_record_offset":7184,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$0::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1332,"record_offset":7888,"parent_record_offset":7764,"ancestor_record_offset":7184,"inlinee":4330,"source_line":217,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":1332,"record_offset":7764,"parent_record_offset":7728,"ancestor_record_offset":7184,"inlinee":18352,"source_line":1259,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1332,"record_offset":12680,"parent_record_offset":12652,"ancestor_record_offset":7184,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1332,"record_offset":9424,"parent_record_offset":9392,"ancestor_record_offset":7184,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1332,"record_offset":8368,"parent_record_offset":8340,"ancestor_record_offset":7184,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1332,"record_offset":8340,"parent_record_offset":8304,"ancestor_record_offset":7184,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1332,"record_offset":8580,"parent_record_offset":8340,"ancestor_record_offset":7184,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1332,"record_offset":12368,"parent_record_offset":12344,"ancestor_record_offset":7184,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::Arc::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1332,"record_offset":15044,"parent_record_offset":15008,"ancestor_record_offset":7184,"inlinee":4296,"source_line":1790,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$33::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":1332,"record_offset":15008,"parent_record_offset":14984,"ancestor_record_offset":7184,"inlinee":4294,"source_line":2420,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1332,"record_offset":9392,"parent_record_offset":9348,"ancestor_record_offset":7184,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_with
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1332,"record_offset":11580,"parent_record_offset":11188,"ancestor_record_offset":7184,"inlinee":4231,"source_line":2627,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1332,"record_offset":12048,"parent_record_offset":12000,"ancestor_record_offset":7184,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1332,"record_offset":8304,"parent_record_offset":8268,"ancestor_record_offset":7184,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1332,"record_offset":8208,"parent_record_offset":8184,"ancestor_record_offset":7184,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_elem::impl$0::from_elem
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_elem.rs
//     evidence={"module":1332,"record_offset":11188,"parent_record_offset":11148,"ancestor_record_offset":7184,"inlinee":4222,"source_line":14,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::ArrayVec::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1332,"record_offset":7852,"parent_record_offset":7828,"ancestor_record_offset":7184,"inlinee":23754,"source_line":82,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] arrayvec::arrayvec::impl$36::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"arrayvec","version":"0.7.4","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/arrayvec-0.7.4/src/arrayvec.rs
//     evidence={"module":1332,"record_offset":7828,"parent_record_offset":7800,"ancestor_record_offset":7184,"inlinee":23753,"source_line":1208,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1332,"record_offset":7640,"parent_record_offset":7612,"ancestor_record_offset":7184,"inlinee":11125,"source_line":24,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1332,"record_offset":7664,"parent_record_offset":7640,"ancestor_record_offset":7184,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1332,"record_offset":7688,"parent_record_offset":7664,"ancestor_record_offset":7184,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1332,"record_offset":8404,"parent_record_offset":8368,"ancestor_record_offset":7184,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1332,"record_offset":8428,"parent_record_offset":8404,"ancestor_record_offset":7184,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1332,"record_offset":11692,"parent_record_offset":11660,"ancestor_record_offset":7184,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1332,"record_offset":9200,"parent_record_offset":9172,"ancestor_record_offset":7184,"inlinee":4898,"source_line":1225,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1332,"record_offset":9260,"parent_record_offset":9228,"ancestor_record_offset":7184,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1332,"record_offset":11660,"parent_record_offset":11628,"ancestor_record_offset":7184,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1332,"record_offset":11628,"parent_record_offset":11580,"ancestor_record_offset":7184,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1332,"record_offset":8232,"parent_record_offset":8208,"ancestor_record_offset":7184,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1332,"record_offset":12708,"parent_record_offset":12680,"ancestor_record_offset":7184,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1332,"record_offset":9724,"parent_record_offset":9700,"ancestor_record_offset":7184,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1332,"record_offset":9296,"parent_record_offset":9144,"ancestor_record_offset":7184,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1332,"record_offset":12320,"parent_record_offset":12000,"ancestor_record_offset":7184,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1332,"record_offset":8144,"parent_record_offset":8116,"ancestor_record_offset":7184,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1332,"record_offset":9944,"parent_record_offset":9920,"ancestor_record_offset":7184,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1332,"record_offset":9920,"parent_record_offset":9896,"ancestor_record_offset":7184,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1332,"record_offset":15076,"parent_record_offset":15044,"ancestor_record_offset":7184,"inlinee":4298,"source_line":398,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1332,"record_offset":8032,"parent_record_offset":8004,"ancestor_record_offset":7184,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1332,"record_offset":9456,"parent_record_offset":9348,"ancestor_record_offset":7184,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1332,"record_offset":10648,"parent_record_offset":10536,"ancestor_record_offset":7184,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":1332,"record_offset":8116,"parent_record_offset":8088,"ancestor_record_offset":7184,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1332,"record_offset":15104,"parent_record_offset":15044,"ancestor_record_offset":7184,"inlinee":4300,"source_line":3353,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::fence
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1332,"record_offset":15136,"parent_record_offset":15008,"ancestor_record_offset":7184,"inlinee":4301,"source_line":3646,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1332,"record_offset":9028,"parent_record_offset":9000,"ancestor_record_offset":7184,"inlinee":8929,"source_line":756,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$60::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1332,"record_offset":9536,"parent_record_offset":9512,"ancestor_record_offset":7184,"inlinee":8644,"source_line":4727,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":9896,"parent_record_offset":9588,"ancestor_record_offset":7184,"inlinee":8358,"source_line":561,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":9144,"parent_record_offset":9084,"ancestor_record_offset":7184,"inlinee":6425,"source_line":3760,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":9588,"parent_record_offset":9560,"ancestor_record_offset":7184,"inlinee":8357,"source_line":3827,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":9056,"parent_record_offset":9028,"ancestor_record_offset":7184,"inlinee":6420,"source_line":1596,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":9116,"parent_record_offset":9084,"ancestor_record_offset":7184,"inlinee":6422,"source_line":2597,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":9084,"parent_record_offset":9056,"ancestor_record_offset":7184,"inlinee":6421,"source_line":2297,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1332,"record_offset":9668,"parent_record_offset":9636,"ancestor_record_offset":7184,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1332,"record_offset":9700,"parent_record_offset":9668,"ancestor_record_offset":7184,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1332,"record_offset":9636,"parent_record_offset":9588,"ancestor_record_offset":7184,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$28::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":9560,"parent_record_offset":9536,"ancestor_record_offset":7184,"inlinee":8355,"source_line":4127,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load_aligned
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1332,"record_offset":9172,"parent_record_offset":9144,"ancestor_record_offset":7184,"inlinee":6426,"source_line":60,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty_or_deleted
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1332,"record_offset":9228,"parent_record_offset":9172,"ancestor_record_offset":7184,"inlinee":6427,"source_line":104,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::proc::read_specifier
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/proc.rs
//     evidence={"module":1332,"record_offset":10588,"parent_record_offset":10564,"ancestor_record_offset":7184,"inlinee":62157,"source_line":23,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] modular_bitfield::private::push_pop::impl$14::push_bits
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"modular-bitfield","version":"0.11.2","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/modular-bitfield-0.11.2/src/private/push_pop.rs
//     evidence={"module":1332,"record_offset":10612,"parent_record_offset":10588,"ancestor_record_offset":7184,"inlinee":62159,"source_line":85,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1332,"record_offset":10328,"parent_record_offset":10284,"ancestor_record_offset":7184,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1332,"record_offset":10384,"parent_record_offset":10284,"ancestor_record_offset":7184,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1332,"record_offset":10356,"parent_record_offset":10284,"ancestor_record_offset":7184,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::cull_mode_or_err
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1332,"record_offset":10796,"parent_record_offset":10772,"ancestor_record_offset":7184,"inlinee":62166,"source_line":613,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::depth_compare_mode_or_err
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1332,"record_offset":10912,"parent_record_offset":10884,"ancestor_record_offset":7184,"inlinee":62168,"source_line":615,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::stencil_test_enable_or_err
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1332,"record_offset":11060,"parent_record_offset":11036,"ancestor_record_offset":7184,"inlinee":62170,"source_line":619,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::shader::RasterPipelineOpts::topology_or_err
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/shader.rs
//     evidence={"module":1332,"record_offset":10564,"parent_record_offset":10536,"ancestor_record_offset":7184,"inlinee":62156,"source_line":612,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::temp_list::impl$0::default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/temp_list.rs
//     evidence={"module":1332,"record_offset":7800,"parent_record_offset":7764,"ancestor_record_offset":7184,"inlinee":23751,"source_line":9,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1332,"record_offset":9000,"parent_record_offset":8972,"ancestor_record_offset":7184,"inlinee":8928,"source_line":518,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$39::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1332,"record_offset":9512,"parent_record_offset":9488,"ancestor_record_offset":7184,"inlinee":8642,"source_line":2233,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::impl$12::create_raster_pipeline::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 789
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF141
// 首条位置: {"module":1321,"inlinee":61761}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::ffi::c_str::CString::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\ffi\c_str.rs
//     evidence={"module":1321,"record_offset":29312,"parent_record_offset":29068,"ancestor_record_offset":29068,"inlinee":9051,"source_line":263,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_shader_module
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1321,"record_offset":29164,"parent_record_offset":29068,"ancestor_record_offset":29068,"inlinee":61764,"source_line":2668,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::PipelineShaderStageCreateInfo::stage
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1321,"record_offset":29588,"parent_record_offset":29068,"ancestor_record_offset":29068,"inlinee":61768,"source_line":3867,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::ShaderModuleCreateInfo::code
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1321,"record_offset":29132,"parent_record_offset":29068,"ancestor_record_offset":29068,"inlinee":61763,"source_line":3467,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::impl$189::default
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1321,"record_offset":29284,"parent_record_offset":29068,"ancestor_record_offset":29068,"inlinee":61766,"source_line":3843,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":29616,"parent_record_offset":29068,"ancestor_record_offset":29068,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1321,"record_offset":29340,"parent_record_offset":29068,"ancestor_record_offset":29068,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":29536,"parent_record_offset":29512,"ancestor_record_offset":29068,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1321,"record_offset":29512,"parent_record_offset":29456,"ancestor_record_offset":29068,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":29484,"parent_record_offset":29456,"ancestor_record_offset":29068,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1321,"record_offset":29456,"parent_record_offset":29428,"ancestor_record_offset":29068,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1321,"record_offset":29196,"parent_record_offset":29164,"ancestor_record_offset":29068,"inlinee":11125,"source_line":24,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1321,"record_offset":29220,"parent_record_offset":29196,"ancestor_record_offset":29068,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":1321,"record_offset":29244,"parent_record_offset":29220,"ancestor_record_offset":29068,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1321,"record_offset":29380,"parent_record_offset":29340,"ancestor_record_offset":29068,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::get_vk_blend_mode
// 记录形式: compiled_procedure; instances=1
// 源码行: 968
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":554,"record_offset":9000,"rva":7513984,"va":5376223104}
// 编译实例 RVAs: 0x72a780
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::shader::map_vertex_attribute_format
// 记录形式: compiled_procedure; instances=1
// 源码行: 1026
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":554,"record_offset":9116,"rva":7514080,"va":5376223200}
// 编译实例 RVAs: 0x72a7e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t26755565","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376223200,"raw_hex":"e8ed419801","size":5,"soft_fail":false,"target_va":5402978944,"va":5376223374,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":554,"record_offset":9252,"parent_record_offset":9116,"ancestor_record_offset":9116,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00554.jsonl.gz"}
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
// 完整结构及原始类型记录: shader.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::shader::ComputePipelineDesc
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 1
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::shader::DescriptorPoolDiscardPile
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 12
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc<rhapsody::raw::shader::DescriptorPoolDiscardPile,alloc::alloc::Global>", "constructor": "alloc::sync::Arc", "arguments": ["rhapsody::raw::shader::DescriptorPoolDiscardPile", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ash::vk::definitions::DescriptorPool,alloc::sync::Arc<rhapsody::raw::shader::DescriptorPoolDiscardPile,alloc::alloc::Global> >", "constructor": "tuple$", "arguments": ["ash::vk::definitions::DescriptorPool", "alloc::sync::Arc<rhapsody::raw::shader::DescriptorPoolDiscardPile,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<ash::vk::definitions::DescriptorPool,alloc::sync::Arc<rhapsody::raw::shader::DescriptorPoolDiscardPile,alloc::alloc::Global> >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<ash::vk::definitions::DescriptorPool,alloc::sync::Arc<rhapsody::raw::shader::DescriptorPoolDiscardPile,alloc::alloc::Global> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tuple$<ash::vk::definitions::DescriptorPool,alloc::sync::Arc<rhapsody::raw::shader::DescriptorPoolDiscardPile,alloc::alloc::Global> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tuple$<ash::vk::definitions::DescriptorPool,alloc::sync::Arc<rhapsody::raw::shader::DescriptorPoolDiscardPile,alloc::alloc::Global> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<tuple$<ash::vk::definitions::DescriptorPool,alloc::sync::Arc<rhapsody::raw::shader::DescriptorPoolDiscardPile,alloc::alloc::Global> >,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["tuple$<ash::vk::definitions::DescriptorPool,alloc::sync::Arc<rhapsody::raw::shader::DescriptorPoolDiscardPile,alloc::alloc::Global> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<tuple$<ash::vk::definitions::DescriptorPool,alloc::sync::Arc<rhapsody::raw::shader::DescriptorPoolDiscardPile,alloc::alloc::Global> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<tuple$<ash::vk::definitions::DescriptorPool,alloc::sync::Arc<rhapsody::raw::shader::DescriptorPoolDiscardPile,alloc::alloc::Global> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<rhapsody::raw::shader::DescriptorPoolDiscardPile,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["rhapsody::raw::shader::DescriptorPoolDiscardPile", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::shader::DescriptorSetInfo
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 1
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::shader::DescriptorSetInfo>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::shader::DescriptorSetInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::shader::DescriptorSetState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::shader::DescriptorSetState>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::shader::DescriptorSetState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<rhapsody::raw::shader::DescriptorSetState,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["rhapsody::raw::shader::DescriptorSetState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::raw::shader::DescriptorSetState,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<rhapsody::raw::shader::DescriptorSetState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle<rhapsody::raw::shader::DescriptorSetState,alloc::alloc::Global>", "constructor": "alloc::raw_vec::impl$2::reserve::do_reserve_and_handle", "arguments": ["rhapsody::raw::shader::DescriptorSetState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::in_place_collect::impl$1::from_iter<rhapsody::raw::shader::DescriptorSetState,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0> >", "constructor": "alloc::vec::in_place_collect::impl$1::from_iter", "arguments": ["rhapsody::raw::shader::DescriptorSetState", "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0> >", "constructor": "alloc::vec::impl$19::extend_trusted::closure_env$0", "arguments": ["rhapsody::raw::shader::DescriptorSetState", "alloc::alloc::Global", "core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0> > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["rhapsody::raw::shader::DescriptorSetState", "alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<rhapsody::raw::shader::DescriptorSetState,core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0> > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["rhapsody::raw::shader::DescriptorSetState", "core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >", "rhapsody::raw::shader::create_descriptor_set_states::closure_env$0", "tuple$<>", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::shader::RasterPipeline
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "tuple$<rhapsody::raw::shader::RasterPipelineDesc,rhapsody::raw::shader::RasterPipeline>", "constructor": "tuple$", "arguments": ["rhapsody::raw::shader::RasterPipelineDesc", "rhapsody::raw::shader::RasterPipeline"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<rhapsody::raw::shader::RasterPipelineDesc,rhapsody::raw::shader::RasterPipeline>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<rhapsody::raw::shader::RasterPipelineDesc,rhapsody::raw::shader::RasterPipeline>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<rhapsody::raw::shader::RasterPipelineDesc,rhapsody::raw::shader::RasterPipeline> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<rhapsody::raw::shader::RasterPipelineDesc,rhapsody::raw::shader::RasterPipeline>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::shader::RasterPipelineDesc,rhapsody::raw::shader::RasterPipeline,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["rhapsody::raw::shader::RasterPipelineDesc", "rhapsody::raw::shader::RasterPipeline", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<rhapsody::raw::shader::RasterPipelineDesc,rhapsody::raw::shader::RasterPipeline>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::shader::RasterPipelineDesc,rhapsody::raw::shader::RasterPipeline,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<rhapsody::raw::shader::RasterPipelineDesc,rhapsody::raw::shader::RasterPipeline>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::shader::RasterPipelineDesc,rhapsody::raw::shader::RasterPipeline,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::rustc_entry<rhapsody::raw::shader::RasterPipelineDesc,rhapsody::raw::shader::RasterPipeline,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::rustc_entry", "arguments": ["rhapsody::raw::shader::RasterPipelineDesc", "rhapsody::raw::shader::RasterPipeline", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::shader::RasterPipelineDesc
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "tuple$<rhapsody::raw::shader::RasterPipelineDesc,rhapsody::raw::shader::RasterPipeline>", "constructor": "tuple$", "arguments": ["rhapsody::raw::shader::RasterPipelineDesc", "rhapsody::raw::shader::RasterPipeline"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<rhapsody::raw::shader::RasterPipelineDesc,rhapsody::raw::shader::RasterPipeline>,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<rhapsody::raw::shader::RasterPipelineDesc,rhapsody::raw::shader::RasterPipeline>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<rhapsody::raw::shader::RasterPipelineDesc,rhapsody::raw::shader::RasterPipeline> >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<rhapsody::raw::shader::RasterPipelineDesc,rhapsody::raw::shader::RasterPipeline>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::shader::RasterPipelineDesc,rhapsody::raw::shader::RasterPipeline,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["rhapsody::raw::shader::RasterPipelineDesc", "rhapsody::raw::shader::RasterPipeline", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<rhapsody::raw::shader::RasterPipelineDesc,rhapsody::raw::shader::RasterPipeline>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::shader::RasterPipelineDesc,rhapsody::raw::shader::RasterPipeline,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<rhapsody::raw::shader::RasterPipelineDesc,rhapsody::raw::shader::RasterPipeline>", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::shader::RasterPipelineDesc,rhapsody::raw::shader::RasterPipeline,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<rhapsody::raw::shader::RasterPipelineDesc>", "constructor": "ref$", "arguments": ["rhapsody::raw::shader::RasterPipelineDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<rhapsody::raw::shader::RasterPipelineDesc> >", "constructor": "core::hash::BuildHasher::hash_one", "arguments": ["std::hash::random::RandomState", "ref$<rhapsody::raw::shader::RasterPipelineDesc>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::rustc_entry<rhapsody::raw::shader::RasterPipelineDesc,rhapsody::raw::shader::RasterPipeline,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::rustc_entry", "arguments": ["rhapsody::raw::shader::RasterPipelineDesc", "rhapsody::raw::shader::RasterPipeline", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::shader::RasterPipelineOpts
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 33; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::shader::RenderPassAttachmentDesc
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 5
// [PDB 泛型关系，非字段关系] {"type": "arrayvec::arrayvec::impl$27::hash<rhapsody::raw::shader::RenderPassAttachmentDesc,8,std::hash::random::DefaultHasher>", "constructor": "arrayvec::arrayvec::impl$27::hash", "arguments": ["rhapsody::raw::shader::RenderPassAttachmentDesc", "8", "std::hash::random::DefaultHasher"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<rhapsody::raw::shader::RenderPassAttachmentDesc>", "constructor": "core::slice::iter::Iter", "arguments": ["rhapsody::raw::shader::RenderPassAttachmentDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::shader::RenderPassAttachmentDesc>,rhapsody::raw::render_pass::impl$1::create_render_pass::closure_env$0>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<rhapsody::raw::shader::RenderPassAttachmentDesc>", "rhapsody::raw::render_pass::impl$1::create_render_pass::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::shader::RenderPassAttachmentDesc>,rhapsody::raw::render_pass::impl$1::create_render_pass::closure_env$0>,core::option::IntoIter<ash::vk::definitions::AttachmentDescription> >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::shader::RenderPassAttachmentDesc>,rhapsody::raw::render_pass::impl$1::create_render_pass::closure_env$0>", "core::option::IntoIter<ash::vk::definitions::AttachmentDescription>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::AttachmentDescription,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::shader::RenderPassAttachmentDesc>,rhapsody::raw::render_pass::impl$1::create_render_pass::closure_env$0>,core::option::IntoIter<ash::vk::definitions::AttachmentDescription> > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["ash::vk::definitions::AttachmentDescription", "core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::shader::RenderPassAttachmentDesc>,rhapsody::raw::render_pass::impl$1::create_render_pass::closure_env$0>,core::option::IntoIter<ash::vk::definitions::AttachmentDescription> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::shader::ShaderPipelineCommon
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::shader::ShaderPipelineCommon>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::shader::ShaderPipelineCommon"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: shader.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: shader.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 121; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio>,bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio>", "bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAudioGrid>
// [文件级接口候选，具体函数归属 unknown] bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio> > >)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::audio::water_ambience_audio::WaterAmbienceAudio>
// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::event::EventReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::event::EventReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventReader<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafRng>,bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>)
// 候选参数类型: ["bevy_ecs::event::EventReader<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>", "bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>", "bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafRng>", "bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>", "bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>"]
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::systems::ivy::ivy_leaf_spawner::IvySegmentDoneGrowingMsg>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_leaf_storage::IvyLeafRng>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::startup::load_glade_settings::GladeSettings>
// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >)
// 候选参数类型: ["bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >", "bevy_ecs::system::commands::Commands", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >", "enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >"]
// [候选系统接口] event_writer: bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: core::ptr::drop_in_place<ash::vk::enums::Result>
// 信息 ID: b0fdce207184fa181dda6545
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 03d77369acb10f9e8a8c49c5: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>)
// [泛型宿主/闭包/直接引用方参数候选] 076821a8e8d882e6d45dbb01: void (*)(bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::WindowClutterCandidates>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>)
// [泛型宿主/闭包/直接引用方参数候选] 0c4383e19e8e41ddd102d7ce: void (*)(bevy_ecs::event::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// [泛型宿主/闭包/直接引用方参数候选] 0f7c39de2bbbbcade72fdc5b: void (*)(bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::rt::RtBvhBuilderGlueResource>,bevy_ecs::change_detection::Res<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::With<country_core::resources::critters::Critter> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterControl>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>,tuple$<bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >,bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SaveGameCmd> >,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 121a48a843a3661860cc0c60: void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// [泛型宿主/闭包/直接引用方参数候选] 188ee5116e6a2f635b4a8b5b: void (*)(bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::BirdHaven>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::system::system_param::Local<system_critters::wren::WrenGen>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::Wren> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::critters::BirdSpots>,tuple$<> >,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 1ac3b09f94008d55466c958a: void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)
// [泛型宿主/闭包/直接引用方参数候选] 1bdb6576ac14098c1ed7fdf1: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>)
// 其余候选接口见完整附档，共 115 条。
// [类型关系] core::ptr::drop_in_place -> ["ash::vk::enums::Result"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 514 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)> @ 0x12d50 -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)> @ 0x12dc0 -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x5d93c -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64cf3 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64d55 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::asset_libraries::color_library::ColorLibrarySsbo> @ 0x6525e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo> @ 0x6535e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::PathRasterStagingSsbo> @ 0x6545e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo> @ 0x6555e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> @ 0x664ec -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> @ 0x6655c -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >),void (*)(bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >)> @ 0x8b872 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >)> @ 0x8bb13 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x8bd82 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands)> @ 0x8bee1 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref_mut$<country_core::components::DashedLineLength> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref_mut$<country_core::components::DashedLineLength> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x8c110 -> 0x2d20 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_compute_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >
// 信息 ID: 8d80907fb989033b958d3461
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::sync::Arc -> ["shader_pipeline::compiled::CompiledShaderPipeline", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::create_compute_pipeline -> ["alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::shader::ComputeShaderProgram::new @ 0x650241 -> 0xe9bdd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >
// 信息 ID: 65ec4560a0ac98337c01738f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] alloc::sync::Arc -> ["shader_pipeline::compiled::CompiledShaderPipeline", "alloc::alloc::Global"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline -> ["alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::shader::RasterShaderProgramVariant::get_or_generate @ 0x650be3 -> 0xe9aa00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::ComputePipelineDesc::new
// 信息 ID: 099c7d1d1360b809dd4b0049
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 03d77369acb10f9e8a8c49c5: void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::event::EventWriter<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>)
// [泛型宿主/闭包/直接引用方参数候选] 076821a8e8d882e6d45dbb01: void (*)(bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::WindowClutterCandidates>,bevy_ecs::change_detection::Res<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<system_decorator::blueprint::PrevDecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>)
// [泛型宿主/闭包/直接引用方参数候选] 0c4383e19e8e41ddd102d7ce: void (*)(bevy_ecs::event::EventReader<country_core::resources::critters::sheep::CmdCheckForSheepInsideBuilding>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::critters::sheep::Sheep>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::critters::sheep::SheepAudio> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::event::EventWriter<country_core::systems::audio::sheep_audio::PlaySheepReactSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)
// [泛型宿主/闭包/直接引用方参数候选] 0f7c39de2bbbbcade72fdc5b: void (*)(bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::rt::RtBvhBuilderGlueResource>,bevy_ecs::change_detection::Res<country_core::resources::rt::RtWorld>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::system::query::Query<bevy_ecs::entity::Entity,bevy_ecs::query::filter::With<country_core::resources::critters::Critter> >,bevy_ecs::change_detection::ResMut<country_core::resources::critters::CritterControl>,bevy_ecs::change_detection::ResMut<country_core::resources::ivy::ivy_storage::IvyStorage>,bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > >,bevy_ecs::change_detection::ResMut<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::render::RenderTemporalState>,tuple$<bevy_ecs::event::EventWriter<enum2$<country_core::systems::backend::input::input_record_replay::InputReplayCommand> >,bevy_ecs::event::EventWriter<country_core::systems::save::save_game::SaveGameCmd> >,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 121a48a843a3661860cc0c60: void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::Res<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<system_decorator::blueprint::DecoratorBlueprints>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)
// [泛型宿主/闭包/直接引用方参数候选] 188ee5116e6a2f635b4a8b5b: void (*)(bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::system::query::Query<tuple$<ref$<country_core::resources::tree_storage::BirdHaven>,ref$<country_core::systems::prefab::PrefabInstanceData> >,tuple$<> >,bevy_ecs::system::system_param::Local<system_critters::wren::WrenGen>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<utils::components::transform::Transform>,ref_mut$<country_core::resources::critters::Wren> >,tuple$<> >,bevy_ecs::system::query::Query<ref_mut$<country_core::resources::critters::BirdSpots>,tuple$<> >,bevy_ecs::system::commands::Commands)
// [泛型宿主/闭包/直接引用方参数候选] 1ac3b09f94008d55466c958a: void (*)(bevy_ecs::event::EventReader<country_core::resources::roofs::commands::DeleteRoofCmd>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::resources::roofs::roof_shape::Roof> >,tuple$<> >,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::roofs::roof_sounds::PlayRoofSound>)
// [泛型宿主/闭包/直接引用方参数候选] 1bdb6576ac14098c1ed7fdf1: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventReader<country_core::resources::history::tree_edit::SpawnPreplacedTrees>,bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>,bevy_ecs::event::EventWriter<country_core::systems::prefab::SpawnPrefabCmd>)
// 其余候选接口见完整附档，共 115 条。
// 引用方: 原程序直接转移 514 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)> @ 0x12d50 -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>),void (*)(bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::public_walls::WallEntities>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::system::query::Query<tuple$<ref_mut$<country_core::geometry::instanced_wall::InstancedWall>,ref_mut$<country_core::resources::walls::wall_trigger_volume::WallTriggerVolume> >,tuple$<> >,bevy_ecs::system::query::Query<ref$<country_core::resources::roofs::roof_shape::Roof>,tuple$<> >,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,bevy_ecs::event::EventReader<country_core::systems::wall::wall_state::wall_roof_hashmap::RecalculateWallBricks>,bevy_ecs::event::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::resources::walls::plaster::RecalculatePlasterCmd>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_storage::DecoratorStorage>,country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources>)> @ 0x12dc0 -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x5d93c -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64cf3 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0x64d55 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::asset_libraries::color_library::ColorLibrarySsbo> @ 0x6525e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo> @ 0x6535e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::PathRasterStagingSsbo> @ 0x6545e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::pass_resource_access::read_access_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo> @ 0x6555e -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::apply_deferred<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> @ 0x664ec -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::queue_deferred<void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>),void (*)(country_core::resources::world_raster_defs::AllWorldRasterWrite,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::event::EventReader<country_core::resources::world_raster::UploadWorldRasterBuffersToStagingCmd>)> @ 0x6655c -> 0x2d20 (direct_jump_outside_procedure); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >),void (*)(bevy_ecs::event::EventWriter<bevy_state::state::transitions::StateTransitionEvent<country_core::app_loading_state::AppLoadingState> >,bevy_ecs::system::commands::Commands,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> > > >,enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<enum2$<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState> > > > >)> @ 0x8b872 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >),void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::animate_wall_bulldoze::AnimateWallBulldozeBuffer>,bevy_ecs::change_detection::Res<country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast>,country_core::resources::walls::decorator_archivist::DecoratorArchivalResources,bevy_ecs::change_detection::Res<country_core::resources::color_id::WallColorIds>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::inner_walls::InnerWalls>,bevy_ecs::change_detection::ResMut<country_core::systems::ui::resources::ui_state::UiState>,bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<country_core::resources::editing_highlight::EditingHighlight>,bevy_ecs::change_detection::Res<country_core::systems::wall::eraser::EraserSizeMultiplier>,tuple$<bevy_ecs::change_detection::ResMut<country_core::systems::wall::eraser::archivist::WallEraserArchivist>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_storage::DecoratorStorage>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::decorator_archivist::DecoratorArchivist> >,bevy_ecs::change_detection::Res<country_core::resources::color_id::HalftimberColorIds>,tuple$<bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_fmod::PlayWallAudio>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetWallColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::color_id::SetHalftimberColorIdCmd>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed>,bevy_ecs::event::EventWriter<country_core::resources::onboarding::NotifyHintSystemSomethingWasErased>,bevy_ecs::event::EventWriter<country_core::resources::events::ReanchorDecorators>,bevy_ecs::event::EventWriter<country_core::systems::ivy::ivy_pruner::RemoveIvyCmd> >)> @ 0x8bb13 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>),void (*)(bevy_ecs::event::EventReader<system_wall_constructor::construct_gates::systems::ConstructGateCmd>,bevy_ecs::change_detection::ResMut<country_core::resources::walls::wooden_gates::Gates>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::change_detection::Res<country_core::resources::terrain_height::TerrainHeights>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>)> @ 0x8bd82 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands),void (*)(bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> > >,bevy_ecs::query::filter::Without<country_core::asset_libraries::Handle<country_core::render::vao::Vao> > >,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::vao_library::AssetVAOLibrary>,bevy_ecs::system::commands::Commands)> @ 0x8bee1 -> 0x2d20 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$7::update_archetype_component_access<void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref_mut$<country_core::components::DashedLineLength> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),void (*)(bevy_ecs::event::EventReader<country_core::systems::wall::snapping::telegraph_walls_are_merged::PlayLineAcrossWall>,bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ecs::system::query::Query<tuple$<bevy_ecs::entity::Entity,ref_mut$<country_core::systems::wall::snapping::telegraph_walls_are_merged::LineAcrossWallSignifier>,ref$<country_core::asset_libraries::Handle<country_core::render::mesh::Mesh> >,ref_mut$<country_core::components::DashedLineLength> >,tuple$<> >,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::event::EventWriter<country_core::systems::audio::wall_audio_ui::PlayWallUiSound>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)> @ 0x8c110 -> 0x2d20 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::DescriptorPoolDiscardPile::reclaim_descriptor_pools
// 信息 ID: 5e1fb78f231e1eae659511d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::Queue::reclaim_command_buffer_resources; module=570, inline_record=27540; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::DescriptorSetState::allocate_descriptor_set
// 信息 ID: 619a112588bb0f5befbf38ab
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > > @ 0xe9c6b7 -> 0x729b10 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$4> > | rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6> > @ 0xe9d18d -> 0x729b10 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > @ 0xe9d8d5 -> 0x729b10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineDesc::new
// 信息 ID: 97b7b92cc837d6c51cc518d3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x10026c2 -> 0x72a6f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::cull_mode
// 信息 ID: fd007691e079854a0fd5be09
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >; module=1332, inline_record=10772; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::cull_mode_or_err
// 信息 ID: 45b63c0902adfd1aec6a12f6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >; module=1332, inline_record=10796; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::depth_compare_mode
// 信息 ID: 5178f9aff53dfca7fe5e0111
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >; module=1332, inline_record=10884; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::depth_compare_mode_or_err
// 信息 ID: f3b79be5c900088eb3a8d108
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >; module=1332, inline_record=10912; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::polygon_mode
// 信息 ID: 3aad49cb42965f33d4dc03da
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; module=1385, inline_record=38260; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode
// 信息 ID: e4770459aea7830da0b78421
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 23 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::egui_backend::EguiBackend::draw; module=39, inline_record=22400; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$38,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=394, inline_record=33016; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$49,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=394, inline_record=126132; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery; module=402, inline_record=51668; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::logo_overlay::render_logo_overlay; module=412, inline_record=24960; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass; module=415, inline_record=41460; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::session_selection::render_session_selection_previews; module=433, inline_record=28740; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::session_selection::render_session_selection_previews; module=433, inline_record=29980; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_blend_mode_checked
// 信息 ID: 4a2279b8c15b2ce98d0fdc47
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 23 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::egui_backend::EguiBackend::draw; module=39, inline_record=22424; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$38,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=394, inline_record=33040; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$49,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=394, inline_record=126152; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery; module=402, inline_record=51692; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::logo_overlay::render_logo_overlay; module=412, inline_record=24984; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass; module=415, inline_record=41484; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::session_selection::render_session_selection_previews; module=433, inline_record=28764; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::session_selection::render_session_selection_previews; module=433, inline_record=30004; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode
// 信息 ID: e814f876a49d630fffe01931
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_shader_nani; module=388, inline_record=24164; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$16,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=394, inline_record=45028; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$32,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=394, inline_record=55636; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$23,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=394, inline_record=70576; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$28,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=394, inline_record=92840; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$0,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=394, inline_record=116528; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$54,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=394, inline_record=125520; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_shaders::clearing_shaders::tree_trunk; module=485, inline_record=23064; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_cull_mode_checked
// 信息 ID: 35fc64c0b258d94b5ea4c50a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 11 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_shaders::instanced_roof::draw_instanced_roof_shader_nani; module=388, inline_record=24188; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$16,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=394, inline_record=45048; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$32,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=394, inline_record=55656; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$23,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=394, inline_record=70596; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$28,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=394, inline_record=92864; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$0,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=394, inline_record=116552; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$54,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=394, inline_record=125544; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::draw_shaders::clearing_shaders::tree_trunk; module=485, inline_record=23084; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_depth_compare_mode
// 信息 ID: c6ef1300c915ecd223ff7250
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::shadow_pass::shadow_pass; module=449, inline_record=12992; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_depth_compare_mode_checked
// 信息 ID: 529f5835f6dd97db2d8ff0b8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::shadow_pass::shadow_pass; module=449, inline_record=13016; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable
// 信息 ID: 71b6d331c2c4bd3d2f8647fe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 20 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::egui_backend::EguiBackend::draw; module=39, inline_record=22316; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery; module=402, inline_record=51584; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::feedback_report_bg::render_feedback_report_bg; module=412, inline_record=22020; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::logo_overlay::render_logo_overlay; module=412, inline_record=24876; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::menu_background::render_menu_bg; module=412, inline_record=31976; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass; module=415, inline_record=41376; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::session_selection::render_session_selection_previews; module=433, inline_record=27528; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::session_selection::render_session_selection_previews; module=433, inline_record=28656; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_depth_test_enable_checked
// 信息 ID: 6fb6ff752ba932c892491a97
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 20 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] tiny_glade::egui_backend::EguiBackend::draw; module=39, inline_record=22340; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery; module=402, inline_record=51608; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::feedback_report_bg::render_feedback_report_bg; module=412, inline_record=22044; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::logo_overlay::render_logo_overlay; module=412, inline_record=24900; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::menu_background::render_menu_bg; module=412, inline_record=32000; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass; module=415, inline_record=41400; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::session_selection::render_session_selection_previews; module=433, inline_record=27552; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::session_selection::render_session_selection_previews; module=433, inline_record=28680; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_depth_write
// 信息 ID: 1350158f03902c5575ce69f0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster; module=441, inline_record=18960; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass; module=449, inline_record=14188; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::transparency_pass::pre_taa_transparency_pass; module=449, inline_record=23940; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::transparency_pass::post_taa_transparency_pass; module=449, inline_record=25188; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_depth_write_checked
// 信息 ID: e4df5b66d211c3ec87669087
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster; module=441, inline_record=18984; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::transparency_pass::refraction_copy_pass; module=449, inline_record=14212; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::transparency_pass::pre_taa_transparency_pass; module=449, inline_record=23964; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::transparency_pass::post_taa_transparency_pass; module=449, inline_record=25212; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_polygon_mode
// 信息 ID: 171e45345d8103c9155353ac
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::wireframe_pass::wireframe_pass; module=487, inline_record=28640; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_polygon_mode_checked
// 信息 ID: 684ea8ce7f33f0e8e4a06776
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::wireframe_pass::wireframe_pass; module=487, inline_record=28664; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_compare_mask
// 信息 ID: de42bc54ccc4040ef0cb92b9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster; module=441, inline_record=20020; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_compare_mask_checked
// 信息 ID: ee0cba9b5a7bea27259b814a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster; module=441, inline_record=20044; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_compare_op
// 信息 ID: 8fb64dc927aff0ec8f54dff0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster; module=441, inline_record=19936; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_compare_op_checked
// 信息 ID: 2a5d32da8f3cf8d2a117a9ca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::deferred_raster::deferred_raster; module=441, inline_record=19960; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op
// 信息 ID: b15b48e236f138b067b31ee4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=388, inline_record=32784; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=388, inline_record=33948; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=388, inline_record=35096; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=388, inline_record=39092; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=388, inline_record=40256; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=388, inline_record=41404; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass; module=388, inline_record=42812; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass; module=388, inline_record=43988; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_pass_op_checked
// 信息 ID: 20be9c6c2ffd08244ea3ce12
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 8 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=388, inline_record=32808; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=388, inline_record=33972; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=388, inline_record=35120; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=388, inline_record=39116; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=388, inline_record=40280; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=388, inline_record=41428; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass; module=388, inline_record=42836; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass; module=388, inline_record=44012; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_test_enable
// 信息 ID: e9a9b5913a05d160aa9f7801
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=388, inline_record=32700; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=388, inline_record=33864; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=388, inline_record=35012; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=388, inline_record=39008; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=388, inline_record=40172; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=388, inline_record=41320; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass; module=388, inline_record=42728; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass; module=388, inline_record=43904; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_stencil_test_enable_checked
// 信息 ID: d86326399b2a526bf7994de3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=388, inline_record=32724; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=388, inline_record=33888; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass; module=388, inline_record=35036; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=388, inline_record=39032; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=388, inline_record=40196; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2; module=388, inline_record=41344; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass; module=388, inline_record=42752; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass; module=388, inline_record=43928; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_topology
// 信息 ID: 97cc217d4a43c63c58ec8f9e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$51,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=394, inline_record=784; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$50,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=394, inline_record=9620; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::set_topology_checked
// 信息 ID: a220167f1793b7a623701dba
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$51,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=394, inline_record=804; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env$50,tuple$<country_core::render::pass_builder::draw_builder::DrawBuilder,ref$<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell> >; module=394, inline_record=9640; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::stencil_test_enable
// 信息 ID: 97a3c6dbee95321b6bc36d00
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >; module=1332, inline_record=11036; 不是独立调用边。
// [PDB 内联宿主] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; module=1385, inline_record=38136; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::stencil_test_enable_or_err
// 信息 ID: 8e28755668b6e58f3210c94b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >; module=1332, inline_record=11060; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::topology
// 信息 ID: a8b5a2f99b3bfee9a035fd65
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >; module=1332, inline_record=10536; 不是独立调用边。
// [PDB 内联宿主] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global>; module=1385, inline_record=38192; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RasterPipelineOpts::topology_or_err
// 信息 ID: 9a48ed57c8371a9f186d9745
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >; module=1332, inline_record=10564; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RenderPassAttachmentDesc::clear_input
// 信息 ID: de89e0467fe89b5d86eb0a2d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::draw_stream::RasterPass::render @ 0x6613bb -> 0x72a730 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RenderPassAttachmentDesc::new
// 信息 ID: 379dc63c843b993a88c2e022
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::draw_stream::RasterPass::render @ 0x661235 -> 0x72a710 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_stream::RasterPass::render @ 0x661358 -> 0x72a710 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::RenderPassAttachmentDesc::sample_count
// 信息 ID: 463d813bf3a742a8d1bf58c3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::draw_stream::RasterPass::render @ 0x661250 -> 0x72a750 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_stream::RasterPass::render @ 0x661381 -> 0x72a750 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::create_descriptor_set_layout<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<shader_pipeline::reflection::Binding> >,rhapsody::raw::descriptor_set::impl$0::create_persistent_descriptor_set::closure_env$0> >
// 信息 ID: 44a91b83fd93a85ed9764ae9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] core::slice::iter::Iter -> ["shader_pipeline::reflection::Binding"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::enumerate::Enumerate -> ["core::slice::iter::Iter<shader_pipeline::reflection::Binding>"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<shader_pipeline::reflection::Binding> >", "rhapsody::raw::descriptor_set::impl$0::create_persistent_descriptor_set::closure_env$0"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::shader::create_descriptor_set_layout -> ["core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<shader_pipeline::reflection::Binding> >,rhapsody::raw::descriptor_set::impl$0::create_persistent_descriptor_set::closure_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_persistent_descriptor_set @ 0x744a42 -> 0x117c9e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::create_descriptor_set_layout<core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,rhapsody::raw::shader::create_descriptor_set_layouts::closure_env$0> >
// 信息 ID: d8e65322e6d18f6466c165ec
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] std::collections::hash::map::Iter -> ["shader_pipeline::reflection::BindingIndex", "shader_pipeline::reflection::Binding"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>", "rhapsody::raw::shader::create_descriptor_set_layouts::closure_env$0"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::shader::create_descriptor_set_layout -> ["core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::Binding>,rhapsody::raw::shader::create_descriptor_set_layouts::closure_env$0>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::shader::create_descriptor_set_layouts @ 0x72a255 -> 0x117d040 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::create_descriptor_set_layouts
// 信息 ID: cd9a3151f981904b0a0829ca
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> > @ 0xe9aa51 -> 0x72a080 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_compute_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> > @ 0xe9be2c -> 0x72a080 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::create_descriptor_set_layouts::closure$1
// 信息 ID: 1afa2266dc84a4d137aeb54d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::shader::create_descriptor_set_layouts", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::shader::create_descriptor_set_layouts::closure_env$1 <- std::collections::hash::map::impl$79::from_iter<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::DescriptorSetLayoutBinding>,rhapsody::raw::shader::create_descriptor_set_layouts::closure_env$1> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] std::collections::hash::map::impl$79::from_iter<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState,core::iter::adapters::map::Map<core::slice::iter::Iter<ash::vk::definitions::DescriptorSetLayoutBinding>,rhapsody::raw::shader::create_descriptor_set_layouts::closure_env$1> >; module=1595, inline_record=1496; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::create_descriptor_set_states
// 信息 ID: 5da567cc25db8808ebbca1da
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> > @ 0xe9b621 -> 0x72a650 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_compute_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> > @ 0xe9c163 -> 0x72a650 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::create_descriptor_set_states::closure$0
// 信息 ID: 3a98fa5e11ffb659b6a3a46b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::shader::create_descriptor_set_states", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::shader::create_descriptor_set_states::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::shader::create_descriptor_set_states::closure_env$0 <- alloc::vec::in_place_collect::impl$1::from_iter<rhapsody::raw::shader::DescriptorSetState,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0> >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::shader::create_descriptor_set_states::closure_env$0 <- core::iter::adapters::map::impl$2::fold<rhapsody::raw::shader::DescriptorSetState,core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::shader::create_descriptor_set_states::closure_env$0 <- core::iter::adapters::map::impl$2::fold<rhapsody::raw::shader::DescriptorSetState,core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<rhapsody::raw::shader::DescriptorSetState,core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0> > > >; module=3007, inline_record=9980; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::create_descriptor_set_states::closure$0::closure$0
// 信息 ID: 87103e4556e4d228e5640d30
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::shader::create_descriptor_set_states", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<rhapsody::raw::shader::DescriptorSetState,core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::vec::impl$19::extend_trusted::closure_env$0<rhapsody::raw::shader::DescriptorSetState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<std::collections::hash::map::HashMap<shader_pipeline::reflection::BindingIndex,tuple$<ash::vk::enums::DescriptorType,usize>,std::hash::random::RandomState>,alloc::alloc::Global>,alloc::vec::into_iter::IntoIter<ash::vk::definitions::DescriptorSetLayout,alloc::alloc::Global> >,rhapsody::raw::shader::create_descriptor_set_states::closure_env$0> > > >; module=3007, inline_record=11400; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::get_vk_blend_mode
// 信息 ID: fd13d22e1a4e4ea1400449ea
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> > @ 0xe9b231 -> 0x72a780 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::impl$118::eq
// 信息 ID: 14f1978a9a9cc08746b97f2e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] hashbrown::map::HashMap::rustc_entry<rhapsody::raw::shader::RasterPipelineDesc,rhapsody::raw::shader::RasterPipeline,std::hash::random::RandomState,alloc::alloc::Global>; module=1370, inline_record=40712; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::impl$120::hash
// 信息 ID: 3bb7725bb852c163003b7f07
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<rhapsody::raw::shader::RasterPipelineDesc> >; module=1329, inline_record=33808; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::impl$123::eq
// 信息 ID: 6e6bbe0c28a69816752cc17f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] hashbrown::map::HashMap::rustc_entry<rhapsody::raw::shader::RasterPipelineDesc,rhapsody::raw::shader::RasterPipeline,std::hash::random::RandomState,alloc::alloc::Global>; module=1370, inline_record=40656; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::impl$125::hash
// 信息 ID: 1308cee446c2632247d8f168
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<rhapsody::raw::shader::RasterPipelineDesc> >; module=1329, inline_record=33236; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::impl$12::create_raster_pipeline::closure$0
// 信息 ID: a01c161d529f79dd9d1dff81
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::shader::impl$12::create_raster_pipeline", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::shader::impl$12::create_raster_pipeline::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::PipelineShaderStageCreateInfo,core::iter::adapters::map::Map<core::slice::iter::Iter<shader_pipeline::compiled::CompiledShaderStage>,rhapsody::raw::shader::impl$12::create_raster_pipeline::closure_env$0<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::shader::impl$12::create_raster_pipeline::closure_env$0 <- core::iter::adapters::map::impl$2::fold<ash::vk::definitions::PipelineShaderStageCreateInfo,core::slice::iter::Iter<shader_pipeline::compiled::CompiledShaderStage>,rhapsody::raw::shader::impl$12::create_raster_pipeline::closure_env$0<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::PipelineShaderStageCreateInfo,alloc::vec::impl$19::extend_trusted::closure_env$0<ash::vk::definitions::PipelineShaderStageCreateInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<shader_pipeline::compiled::CompiledShaderStage>,rhapsody::raw::shader::impl$12::create_raster_pipeline::closure_env$0<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::shader::impl$12::create_raster_pipeline::closure_env$0 <- core::iter::adapters::map::impl$2::fold<ash::vk::definitions::PipelineShaderStageCreateInfo,core::slice::iter::Iter<shader_pipeline::compiled::CompiledShaderStage>,rhapsody::raw::shader::impl$12::create_raster_pipeline::closure_env$0<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::PipelineShaderStageCreateInfo,alloc::vec::impl$19::extend_trusted::closure_env$0<ash::vk::definitions::PipelineShaderStageCreateInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<shader_pipeline::compiled::CompiledShaderStage>,rhapsody::raw::shader::impl$12::create_raster_pipeline::closure_env$0<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::fold<ash::vk::definitions::PipelineShaderStageCreateInfo,core::slice::iter::Iter<shader_pipeline::compiled::CompiledShaderStage>,rhapsody::raw::shader::impl$12::create_raster_pipeline::closure_env$0<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >,tuple$<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<ash::vk::definitions::PipelineShaderStageCreateInfo,alloc::vec::impl$19::extend_trusted::closure_env$0<ash::vk::definitions::PipelineShaderStageCreateInfo,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<shader_pipeline::compiled::CompiledShaderStage>,rhapsody::raw::shader::impl$12::create_raster_pipeline::closure_env$0<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> > > > > >; module=1321, inline_record=29068; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::impl$130::eq
// 信息 ID: bbb2cae74c6937afde90a0ef
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] hashbrown::map::HashMap::rustc_entry<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass,std::hash::random::RandomState,alloc::alloc::Global>; module=1370, inline_record=35224; 不是独立调用边。
// [PDB 内联宿主] hashbrown::map::HashMap::rustc_entry<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass,std::hash::random::RandomState,alloc::alloc::Global>; module=1370, inline_record=35616; 不是独立调用边。
// [PDB 内联宿主] hashbrown::map::HashMap::rustc_entry<rhapsody::raw::render_pass::RenderPassDesc,rhapsody::raw::render_pass::RenderPass,std::hash::random::RandomState,alloc::alloc::Global>; module=1370, inline_record=36044; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::impl$132::hash
// 信息 ID: e70de086339e41c12395bf5d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<rhapsody::raw::render_pass::RenderPassDesc> >; module=1329, inline_record=46220; 不是独立调用边。
// [PDB 内联宿主] arrayvec::arrayvec::impl$27::hash<rhapsody::raw::shader::RenderPassAttachmentDesc,8,std::hash::random::DefaultHasher>; module=1407, inline_record=25156; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::impl$2::deref
// 信息 ID: e7e70e411b0e1878c209932b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 179 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1d9d9 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1dbad -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1dfc0 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1e1c2 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e5b0 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e804 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1ebf7 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1edab -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f187 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f33b -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f712 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f86a -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fbb3 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fdac -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x2018b -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x2030b -> 0x43c290 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::impl$3::deref
// 信息 ID: fbfe3b020098ffc0f8476f79
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 179 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1d9d9 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory> > @ 0x1dbad -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1dfc0 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<enum2$<tiny_glade::gui::settings::FpsCap> > > @ 0x1e1c2 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e5b0 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::settings::DisplayMode> > @ 0x1e804 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1ebf7 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment> > @ 0x1edab -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f187 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::game_config::ScreenshotFormat> > @ 0x1f33b -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f712 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableFloatWithStep> @ 0x1f86a -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fbb3 -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution> > @ 0x1fdac -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x2018b -> 0x43c290 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::widgets::tweaker::tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String> > @ 0x2030b -> 0x43c290 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::impl$5::create_compute_pipeline::closure$0
// 信息 ID: 2cbdf9820a488b5ce0a7047a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::shader::impl$5::create_compute_pipeline", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_compute_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> >; module=1332, inline_record=17360; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::impl$7::default
// 信息 ID: 206ea19459073d54ca42b46d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 37 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x50b5df -> 0x72a6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x50b8a1 -> 0x72a6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass @ 0x50bb4f -> 0x72a6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x50c3af -> 0x72a6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x50c671 -> 0x72a6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::main_pass_oc2 @ 0x50c91f -> 0x72a6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass @ 0x50ceef -> 0x72a6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::main_pass::late_forward_and_deferred_pass @ 0x50d1af -> 0x72a6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery @ 0x555b80 -> 0x72a6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::feedback_report_bg::render_feedback_report_bg @ 0x579610 -> 0x72a6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::logo_overlay::render_logo_overlay @ 0x579ac7 -> 0x72a6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::menu_background::render_menu_bg @ 0x57a3f5 -> 0x72a6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::object_grass_push::object_grass_push_pass @ 0x58385c -> 0x72a6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::session_selection::render_session_selection_previews @ 0x5cd05f -> 0x72a6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::session_selection::render_session_selection_previews @ 0x5cd4a2 -> 0x72a6e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::session_selection::render_session_selection_previews @ 0x5cd865 -> 0x72a6e0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::shader::map_vertex_attribute_format
// 信息 ID: 8699a9335328f89ebd11d3b4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> > @ 0xe9ae26 -> 0x72a7e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_raster_pipeline<alloc::sync::Arc<shader_pipeline::compiled::CompiledShaderPipeline,alloc::alloc::Global> > @ 0xe9ae50 -> 0x72a7e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
