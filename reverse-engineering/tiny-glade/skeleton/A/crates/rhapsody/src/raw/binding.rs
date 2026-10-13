// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/raw/binding.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 338c68317d0a682e89f4b6d784b3cb77d3e6c6a4365b5ee7172b9a3ba35161b4
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: binding.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 167, 466, 570, 1321, 1329, 1332
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
// use country_core; // namespace candidate
//   package identities: []
// use rhapsody; // namespace candidate
//   package identities: []
// use shader_pipeline; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::binding::impl$1::fmt
// 记录形式: compiled_procedure; instances=1
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1329,"record_offset":49688,"rva":15285600,"va":5383994720}
// 编译实例 RVAs: 0xe93d60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field2_finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6411057","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383994720,"raw_hex":"e8cf2c9eff","size":5,"soft_fail":false,"target_va":5377584208,"va":5383995260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_tuple_field1_finish
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t-6409261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5383994720,"raw_hex":"e8d3339eff","size":5,"soft_fail":false,"target_va":5377585888,"va":5383995144,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::binding::impl$2::clone
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 17
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2EC4
// 首条位置: {"module":167,"inlinee":11972}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":167,"record_offset":5872,"parent_record_offset":5824,"ancestor_record_offset":5824,"inlinee":4740,"source_line":2780,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":167,"record_offset":6184,"parent_record_offset":6160,"ancestor_record_offset":5824,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":167,"record_offset":6208,"parent_record_offset":6184,"ancestor_record_offset":5824,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":167,"record_offset":6160,"parent_record_offset":6060,"ancestor_record_offset":5824,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":167,"record_offset":6060,"parent_record_offset":6032,"ancestor_record_offset":5824,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":167,"record_offset":6032,"parent_record_offset":6004,"ancestor_record_offset":5824,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":167,"record_offset":5968,"parent_record_offset":5936,"ancestor_record_offset":5824,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":167,"record_offset":5936,"parent_record_offset":5904,"ancestor_record_offset":5824,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":167,"record_offset":5904,"parent_record_offset":5872,"ancestor_record_offset":5824,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":167,"record_offset":6004,"parent_record_offset":5968,"ancestor_record_offset":5824,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":167,"record_offset":6104,"parent_record_offset":6060,"ancestor_record_offset":5824,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":167,"record_offset":6128,"parent_record_offset":6104,"ancestor_record_offset":5824,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":167,"record_offset":6312,"parent_record_offset":6288,"ancestor_record_offset":5824,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":167,"record_offset":6288,"parent_record_offset":5968,"ancestor_record_offset":5824,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00167.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":167,"record_offset":6236,"parent_record_offset":6208,"ancestor_record_offset":5824,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00167.jsonl.gz"}
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
// 首条位置: {"module":570,"record_offset":7128,"rva":7572432,"va":5376281552}
// 编译实例 RVAs: 0x738bd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::cmd_bind_pipeline
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":570,"record_offset":7264,"parent_record_offset":7128,"ancestor_record_offset":7128,"inlinee":30904,"source_line":1850,"archive":"_pdb/A/modules/00570.jsonl.gz"}
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
// 首条位置: {"module":1332,"record_offset":18208,"rva":15320592,"va":5384029712}
// 编译实例 RVAs: 0xe9c610
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$bumpalo..Bump$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-7128943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384029712,"raw_hex":"e8913893ff","size":5,"soft_fail":false,"target_va":5376902512,"va":5384031450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dce30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14417098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384029712,"raw_hex":"e8360324ff","size":5,"soft_fail":false,"target_va":5369613872,"va":5384030965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11281111","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384029712,"raw_hex":"e829dd53ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5384030818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e8ee0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8077869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384029712,"raw_hex":"e8d3bd84ff","size":5,"soft_fail":false,"target_va":5375954656,"va":5384032520,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407d0550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7127371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384029712,"raw_hex":"e8b53e93ff","size":5,"soft_fail":false,"target_va":5376902480,"va":5384029846,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de8520
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-738437","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384029712,"raw_hex":"e87bbbf4ff","size":5,"soft_fail":false,"target_va":5383292192,"va":5384030624,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e923b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-42538","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384029712,"raw_hex":"e8d659ffff","size":5,"soft_fail":false,"target_va":5383988144,"va":5384030677,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14102e6c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1646085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384029712,"raw_hex":"e8051e1900","size":5,"soft_fail":false,"target_va":5385676480,"va":5384030390,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::Bump::alloc_layout_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-7127396","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384029712,"raw_hex":"e89c3e93ff","size":5,"soft_fail":false,"target_va":5376902656,"va":5384030047,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::alloc::handle_alloc_error
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t7968192","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384029712,"raw_hex":"e8c0957900","size":5,"soft_fail":false,"target_va":5391998272,"va":5384030075,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::collections::raw_vec::capacity_overflow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t7966890","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384029712,"raw_hex":"e8aa907900","size":5,"soft_fail":false,"target_va":5391998384,"va":5384031489,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18948568","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384029712,"raw_hex":"e8d8212101","size":5,"soft_fail":false,"target_va":5402980128,"va":5384031555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::shader::DescriptorSetState::allocate_descriptor_set
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7809964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384029712,"raw_hex":"e854d488ff","size":5,"soft_fail":false,"target_va":5376219920,"va":5384029879,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-11280440","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5384029712,"raw_hex":"e9c8df53ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5384030147,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5384029712,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5384031219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::cmd_bind_descriptor_sets
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1332,"record_offset":26260,"parent_record_offset":18208,"ancestor_record_offset":18208,"inlinee":30907,"source_line":1761,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1332,"record_offset":25884,"parent_record_offset":18208,"ancestor_record_offset":18208,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1332,"record_offset":19044,"parent_record_offset":18208,"ancestor_record_offset":18208,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::update_descriptor_set
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/binding.rs
//     evidence={"module":1332,"record_offset":19100,"parent_record_offset":18208,"ancestor_record_offset":18208,"inlinee":62135,"source_line":96,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1332,"record_offset":21296,"parent_record_offset":21272,"ancestor_record_offset":18208,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1332,"record_offset":21272,"parent_record_offset":21180,"ancestor_record_offset":18208,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1332,"record_offset":21208,"parent_record_offset":21180,"ancestor_record_offset":18208,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1332,"record_offset":21180,"parent_record_offset":21156,"ancestor_record_offset":18208,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1332,"record_offset":21084,"parent_record_offset":21060,"ancestor_record_offset":18208,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1332,"record_offset":21132,"parent_record_offset":21108,"ancestor_record_offset":18208,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::update_descriptor_sets
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1332,"record_offset":24300,"parent_record_offset":19100,"ancestor_record_offset":18208,"inlinee":62141,"source_line":1356,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::dealloc
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1332,"record_offset":24472,"parent_record_offset":24400,"ancestor_record_offset":18208,"inlinee":17342,"source_line":1707,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::is_last_allocation
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1332,"record_offset":24504,"parent_record_offset":24472,"ancestor_record_offset":18208,"inlinee":17343,"source_line":1700,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1332,"record_offset":19488,"parent_record_offset":19460,"ancestor_record_offset":18208,"inlinee":6156,"source_line":1411,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1332,"record_offset":19524,"parent_record_offset":19488,"ancestor_record_offset":18208,"inlinee":6157,"source_line":1420,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::allocate_in
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":19268,"parent_record_offset":19228,"ancestor_record_offset":18208,"inlinee":62138,"source_line":85,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::current_layout
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":24436,"parent_record_offset":24400,"ancestor_record_offset":18208,"inlinee":17341,"source_line":158,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::dealloc_buffer
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":24400,"parent_record_offset":24376,"ancestor_record_offset":18208,"inlinee":17340,"source_line":650,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":20672,"parent_record_offset":20628,"ancestor_record_offset":18208,"inlinee":17334,"source_line":137,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::with_capacity_in
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":19228,"parent_record_offset":19184,"ancestor_record_offset":18208,"inlinee":62137,"source_line":74,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::impl$0::allocate_in::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":19824,"parent_record_offset":19800,"ancestor_record_offset":18208,"inlinee":62143,"source_line":91,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::impl$5::drop
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":24376,"parent_record_offset":24352,"ancestor_record_offset":18208,"inlinee":17339,"source_line":662,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1332,"record_offset":20628,"parent_record_offset":20544,"ancestor_record_offset":18208,"inlinee":17332,"source_line":1423,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::reserve
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1332,"record_offset":20600,"parent_record_offset":20544,"ancestor_record_offset":18208,"inlinee":17335,"source_line":735,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::with_capacity_in
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1332,"record_offset":19184,"parent_record_offset":19100,"ancestor_record_offset":18208,"inlinee":62136,"source_line":584,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::impl$11::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1332,"record_offset":24240,"parent_record_offset":19100,"ancestor_record_offset":18208,"inlinee":27046,"source_line":1943,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::impl$16::extend
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1332,"record_offset":20544,"parent_record_offset":19100,"ancestor_record_offset":18208,"inlinee":62140,"source_line":2023,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$13::alloc
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1332,"record_offset":19460,"parent_record_offset":19268,"ancestor_record_offset":18208,"inlinee":25380,"source_line":1864,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::from_size_align
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1332,"record_offset":19396,"parent_record_offset":19268,"ancestor_record_offset":18208,"inlinee":49215,"source_line":68,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1332,"record_offset":19700,"parent_record_offset":19676,"ancestor_record_offset":18208,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1332,"record_offset":19676,"parent_record_offset":19524,"ancestor_record_offset":18208,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1332,"record_offset":22596,"parent_record_offset":22572,"ancestor_record_offset":18208,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1332,"record_offset":22064,"parent_record_offset":22040,"ancestor_record_offset":18208,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1332,"record_offset":22148,"parent_record_offset":22124,"ancestor_record_offset":18208,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1332,"record_offset":22088,"parent_record_offset":22064,"ancestor_record_offset":18208,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1332,"record_offset":20868,"parent_record_offset":20832,"ancestor_record_offset":18208,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1332,"record_offset":20832,"parent_record_offset":20800,"ancestor_record_offset":18208,"inlinee":62191,"source_line":134,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::find::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1332,"record_offset":20928,"parent_record_offset":20868,"ancestor_record_offset":18208,"inlinee":62193,"source_line":138,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::find::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1332,"record_offset":21388,"parent_record_offset":21364,"ancestor_record_offset":18208,"inlinee":62194,"source_line":139,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1332,"record_offset":20800,"parent_record_offset":20768,"ancestor_record_offset":18208,"inlinee":8575,"source_line":59,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1332,"record_offset":20768,"parent_record_offset":20544,"ancestor_record_offset":18208,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1332,"record_offset":21464,"parent_record_offset":21440,"ancestor_record_offset":18208,"inlinee":6221,"source_line":116,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_try_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1332,"record_offset":21692,"parent_record_offset":21488,"ancestor_record_offset":18208,"inlinee":6183,"source_line":96,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1332,"record_offset":20952,"parent_record_offset":20928,"ancestor_record_offset":18208,"inlinee":6348,"source_line":2866,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1332,"record_offset":21720,"parent_record_offset":21692,"ancestor_record_offset":18208,"inlinee":6192,"source_line":2873,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1332,"record_offset":21488,"parent_record_offset":21464,"ancestor_record_offset":18208,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1332,"record_offset":19724,"parent_record_offset":19700,"ancestor_record_offset":18208,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1332,"record_offset":19328,"parent_record_offset":19268,"ancestor_record_offset":18208,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1332,"record_offset":19360,"parent_record_offset":19328,"ancestor_record_offset":18208,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1332,"record_offset":21236,"parent_record_offset":21208,"ancestor_record_offset":18208,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1332,"record_offset":22260,"parent_record_offset":22236,"ancestor_record_offset":18208,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::ControlFlow::break_value
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\control_flow.rs
//     evidence={"module":1332,"record_offset":20976,"parent_record_offset":20952,"ancestor_record_offset":18208,"inlinee":33084,"source_line":180,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::impl$0::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\control_flow.rs
//     evidence={"module":1332,"record_offset":22856,"parent_record_offset":21488,"ancestor_record_offset":18208,"inlinee":21161,"source_line":110,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1332,"record_offset":21748,"parent_record_offset":21720,"ancestor_record_offset":18208,"inlinee":4399,"source_line":293,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1332,"record_offset":20900,"parent_record_offset":20868,"ancestor_record_offset":18208,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1332,"record_offset":22912,"parent_record_offset":20768,"ancestor_record_offset":18208,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1332,"record_offset":21364,"parent_record_offset":20832,"ancestor_record_offset":18208,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1332,"record_offset":19800,"parent_record_offset":19268,"ancestor_record_offset":18208,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=49
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1332,"record_offset":21012,"parent_record_offset":20868,"ancestor_record_offset":18208,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1332,"record_offset":24560,"parent_record_offset":24472,"ancestor_record_offset":18208,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1332,"record_offset":22508,"parent_record_offset":22484,"ancestor_record_offset":18208,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1332,"record_offset":22400,"parent_record_offset":22376,"ancestor_record_offset":18208,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1332,"record_offset":19588,"parent_record_offset":19564,"ancestor_record_offset":18208,"inlinee":6159,"source_line":563,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1332,"record_offset":19564,"parent_record_offset":19524,"ancestor_record_offset":18208,"inlinee":6158,"source_line":1299,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1332,"record_offset":21652,"parent_record_offset":21624,"ancestor_record_offset":18208,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1332,"record_offset":21588,"parent_record_offset":21528,"ancestor_record_offset":18208,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1332,"record_offset":19748,"parent_record_offset":19724,"ancestor_record_offset":18208,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1332,"record_offset":19424,"parent_record_offset":19268,"ancestor_record_offset":18208,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1332,"record_offset":19068,"parent_record_offset":19044,"ancestor_record_offset":18208,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1332,"record_offset":21624,"parent_record_offset":21528,"ancestor_record_offset":18208,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1332,"record_offset":21528,"parent_record_offset":21488,"ancestor_record_offset":18208,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":1332,"record_offset":22824,"parent_record_offset":21692,"ancestor_record_offset":18208,"inlinee":61786,"source_line":527,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1332,"record_offset":22572,"parent_record_offset":22548,"ancestor_record_offset":18208,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1332,"record_offset":21832,"parent_record_offset":21804,"ancestor_record_offset":18208,"inlinee":8704,"source_line":1418,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1332,"record_offset":21860,"parent_record_offset":21832,"ancestor_record_offset":18208,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1332,"record_offset":22548,"parent_record_offset":22328,"ancestor_record_offset":18208,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1332,"record_offset":21892,"parent_record_offset":21860,"ancestor_record_offset":18208,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":22460,"parent_record_offset":22436,"ancestor_record_offset":18208,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":22436,"parent_record_offset":22328,"ancestor_record_offset":18208,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":22376,"parent_record_offset":22352,"ancestor_record_offset":18208,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":22756,"parent_record_offset":21968,"ancestor_record_offset":18208,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":22352,"parent_record_offset":22328,"ancestor_record_offset":18208,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":21944,"parent_record_offset":21920,"ancestor_record_offset":18208,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":21920,"parent_record_offset":21860,"ancestor_record_offset":18208,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":21968,"parent_record_offset":21944,"ancestor_record_offset":18208,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1332,"record_offset":22208,"parent_record_offset":22180,"ancestor_record_offset":18208,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1332,"record_offset":22236,"parent_record_offset":22208,"ancestor_record_offset":18208,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1332,"record_offset":22296,"parent_record_offset":22180,"ancestor_record_offset":18208,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1332,"record_offset":22180,"parent_record_offset":21968,"ancestor_record_offset":18208,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":22012,"parent_record_offset":21968,"ancestor_record_offset":18208,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":22328,"parent_record_offset":21968,"ancestor_record_offset":18208,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1332,"record_offset":22040,"parent_record_offset":21968,"ancestor_record_offset":18208,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1332,"record_offset":22124,"parent_record_offset":21968,"ancestor_record_offset":18208,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1332,"record_offset":22664,"parent_record_offset":21968,"ancestor_record_offset":18208,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::binding::impl$0::bind_descriptor_set::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/binding.rs
//     evidence={"module":1332,"record_offset":21776,"parent_record_offset":21748,"ancestor_record_offset":18208,"inlinee":61785,"source_line":77,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$58::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1332,"record_offset":22620,"parent_record_offset":22596,"ancestor_record_offset":18208,"inlinee":11856,"source_line":69,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1332,"record_offset":21804,"parent_record_offset":21776,"ancestor_record_offset":18208,"inlinee":17300,"source_line":1042,"archive":"_pdb/A/modules/01332.jsonl.gz"}
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
// 首条位置: {"module":1332,"record_offset":38764,"rva":15325232,"va":5384034352}
// 编译实例 RVAs: 0xe9d830
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$bumpalo..Bump$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-7133340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384034352,"raw_hex":"e8642793ff","size":5,"soft_fail":false,"target_va":5376902512,"va":5384035847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=5
//     target_aliases=1 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11285654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384034352,"raw_hex":"e86acb53ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5384035361,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406e8790
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8084018","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384034352,"raw_hex":"e8cea584ff","size":5,"soft_fail":false,"target_va":5375952784,"va":5384036797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407d0550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7132009","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384034352,"raw_hex":"e8972c93ff","size":5,"soft_fail":false,"target_va":5376902480,"va":5384034484,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de7040
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-748470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384034352,"raw_hex":"e84a94f4ff","size":5,"soft_fail":false,"target_va":5383286848,"va":5384035313,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140de7cc0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-745361","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384034352,"raw_hex":"e86fa0f4ff","size":5,"soft_fail":false,"target_va":5383290048,"va":5384035404,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e923b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-47306","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384034352,"raw_hex":"e83647ffff","size":5,"soft_fail":false,"target_va":5383988144,"va":5384035445,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14102e6c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1641438","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384034352,"raw_hex":"e8de0b1900","size":5,"soft_fail":false,"target_va":5385676480,"va":5384035037,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::Bump::alloc_layout_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-7132051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384034352,"raw_hex":"e86d2c93ff","size":5,"soft_fail":false,"target_va":5376902656,"va":5384034702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::alloc::handle_alloc_error
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t7963537","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384034352,"raw_hex":"e891837900","size":5,"soft_fail":false,"target_va":5391998272,"va":5384034730,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::collections::raw_vec::capacity_overflow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t7962499","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384034352,"raw_hex":"e8837f7900","size":5,"soft_fail":false,"target_va":5391998384,"va":5384035880,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18944180","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384034352,"raw_hex":"e8b4102101","size":5,"soft_fail":false,"target_va":5402980128,"va":5384035943,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::shader::DescriptorSetState::allocate_descriptor_set
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7814602","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384034352,"raw_hex":"e836c288ff","size":5,"soft_fail":false,"target_va":5376219920,"va":5384034517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-11285087","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5384034352,"raw_hex":"e9a1cd53ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5384034794,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5384034352,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5384035609,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::cmd_bind_descriptor_sets
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1332,"record_offset":45364,"parent_record_offset":38764,"ancestor_record_offset":38764,"inlinee":30907,"source_line":1761,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1332,"record_offset":44988,"parent_record_offset":38764,"ancestor_record_offset":38764,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1332,"record_offset":39572,"parent_record_offset":38764,"ancestor_record_offset":38764,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::update_descriptor_set
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/binding.rs
//     evidence={"module":1332,"record_offset":39628,"parent_record_offset":38764,"ancestor_record_offset":38764,"inlinee":62135,"source_line":96,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1332,"record_offset":41800,"parent_record_offset":41776,"ancestor_record_offset":38764,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1332,"record_offset":41776,"parent_record_offset":41684,"ancestor_record_offset":38764,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1332,"record_offset":41712,"parent_record_offset":41684,"ancestor_record_offset":38764,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1332,"record_offset":41684,"parent_record_offset":41660,"ancestor_record_offset":38764,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1332,"record_offset":41588,"parent_record_offset":41564,"ancestor_record_offset":38764,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1332,"record_offset":41636,"parent_record_offset":41612,"ancestor_record_offset":38764,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::update_descriptor_sets
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1332,"record_offset":43404,"parent_record_offset":39628,"ancestor_record_offset":38764,"inlinee":62141,"source_line":1356,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::dealloc
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1332,"record_offset":43576,"parent_record_offset":43504,"ancestor_record_offset":38764,"inlinee":17342,"source_line":1707,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::is_last_allocation
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1332,"record_offset":43608,"parent_record_offset":43576,"ancestor_record_offset":38764,"inlinee":17343,"source_line":1700,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1332,"record_offset":40016,"parent_record_offset":39988,"ancestor_record_offset":38764,"inlinee":6156,"source_line":1411,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1332,"record_offset":40052,"parent_record_offset":40016,"ancestor_record_offset":38764,"inlinee":6157,"source_line":1420,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::allocate_in
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":39796,"parent_record_offset":39756,"ancestor_record_offset":38764,"inlinee":62138,"source_line":85,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::current_layout
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":43540,"parent_record_offset":43504,"ancestor_record_offset":38764,"inlinee":17341,"source_line":158,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::dealloc_buffer
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":43504,"parent_record_offset":43480,"ancestor_record_offset":38764,"inlinee":17340,"source_line":650,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":41200,"parent_record_offset":41156,"ancestor_record_offset":38764,"inlinee":17334,"source_line":137,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::with_capacity_in
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":39756,"parent_record_offset":39712,"ancestor_record_offset":38764,"inlinee":62137,"source_line":74,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::impl$0::allocate_in::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":40352,"parent_record_offset":40328,"ancestor_record_offset":38764,"inlinee":62143,"source_line":91,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::impl$5::drop
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":43480,"parent_record_offset":43456,"ancestor_record_offset":38764,"inlinee":17339,"source_line":662,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1332,"record_offset":41156,"parent_record_offset":41072,"ancestor_record_offset":38764,"inlinee":17332,"source_line":1423,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::reserve
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1332,"record_offset":41128,"parent_record_offset":41072,"ancestor_record_offset":38764,"inlinee":17335,"source_line":735,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::with_capacity_in
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1332,"record_offset":39712,"parent_record_offset":39628,"ancestor_record_offset":38764,"inlinee":62136,"source_line":584,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::impl$11::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1332,"record_offset":43344,"parent_record_offset":39628,"ancestor_record_offset":38764,"inlinee":27046,"source_line":1943,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::impl$16::extend
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1332,"record_offset":41072,"parent_record_offset":39628,"ancestor_record_offset":38764,"inlinee":62140,"source_line":2023,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$13::alloc
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1332,"record_offset":39988,"parent_record_offset":39796,"ancestor_record_offset":38764,"inlinee":25380,"source_line":1864,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::from_size_align
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1332,"record_offset":39924,"parent_record_offset":39796,"ancestor_record_offset":38764,"inlinee":49215,"source_line":68,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1332,"record_offset":40228,"parent_record_offset":40204,"ancestor_record_offset":38764,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1332,"record_offset":40204,"parent_record_offset":40052,"ancestor_record_offset":38764,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1332,"record_offset":41376,"parent_record_offset":41344,"ancestor_record_offset":38764,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1332,"record_offset":41344,"parent_record_offset":41320,"ancestor_record_offset":38764,"inlinee":62191,"source_line":134,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::find::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1332,"record_offset":41432,"parent_record_offset":41376,"ancestor_record_offset":38764,"inlinee":62193,"source_line":138,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::find::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1332,"record_offset":41892,"parent_record_offset":41868,"ancestor_record_offset":38764,"inlinee":62194,"source_line":139,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1332,"record_offset":41320,"parent_record_offset":41296,"ancestor_record_offset":38764,"inlinee":8575,"source_line":59,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1332,"record_offset":41296,"parent_record_offset":41072,"ancestor_record_offset":38764,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1332,"record_offset":41456,"parent_record_offset":41432,"ancestor_record_offset":38764,"inlinee":6348,"source_line":2866,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1332,"record_offset":40252,"parent_record_offset":40228,"ancestor_record_offset":38764,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1332,"record_offset":39856,"parent_record_offset":39796,"ancestor_record_offset":38764,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1332,"record_offset":39888,"parent_record_offset":39856,"ancestor_record_offset":38764,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1332,"record_offset":41740,"parent_record_offset":41712,"ancestor_record_offset":38764,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::ControlFlow::break_value
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\control_flow.rs
//     evidence={"module":1332,"record_offset":41480,"parent_record_offset":41456,"ancestor_record_offset":38764,"inlinee":33084,"source_line":180,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1332,"record_offset":41404,"parent_record_offset":41376,"ancestor_record_offset":38764,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1332,"record_offset":42016,"parent_record_offset":41296,"ancestor_record_offset":38764,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1332,"record_offset":41868,"parent_record_offset":41344,"ancestor_record_offset":38764,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1332,"record_offset":40328,"parent_record_offset":39796,"ancestor_record_offset":38764,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=49
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1332,"record_offset":41516,"parent_record_offset":41376,"ancestor_record_offset":38764,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1332,"record_offset":43664,"parent_record_offset":43576,"ancestor_record_offset":38764,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1332,"record_offset":40116,"parent_record_offset":40092,"ancestor_record_offset":38764,"inlinee":6159,"source_line":563,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1332,"record_offset":40092,"parent_record_offset":40052,"ancestor_record_offset":38764,"inlinee":6158,"source_line":1299,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1332,"record_offset":43632,"parent_record_offset":43608,"ancestor_record_offset":38764,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1332,"record_offset":40276,"parent_record_offset":40252,"ancestor_record_offset":38764,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1332,"record_offset":39952,"parent_record_offset":39796,"ancestor_record_offset":38764,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1332,"record_offset":39596,"parent_record_offset":39572,"ancestor_record_offset":38764,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01332.jsonl.gz"}
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
// 首条位置: {"module":1332,"record_offset":27092,"rva":15323408,"va":5384032528}
// 编译实例 RVAs: 0xe9d110
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$bumpalo..Bump$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-7131312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384032528,"raw_hex":"e8502f93ff","size":5,"soft_fail":false,"target_va":5376902512,"va":5384033819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dce30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14419381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384032528,"raw_hex":"e84bfa23ff","size":5,"soft_fail":false,"target_va":5369613872,"va":5384033248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407d0550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7130152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384032528,"raw_hex":"e8d83393ff","size":5,"soft_fail":false,"target_va":5376902480,"va":5384032627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e923b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-45349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384032528,"raw_hex":"e8db4effff","size":5,"soft_fail":false,"target_va":5383988144,"va":5384033488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14102e6c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1643435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384032528,"raw_hex":"e8ab131900","size":5,"soft_fail":false,"target_va":5385676480,"va":5384033040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::Bump::alloc_layout_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-7130128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384032528,"raw_hex":"e8f03393ff","size":5,"soft_fail":false,"target_va":5376902656,"va":5384032779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::alloc::handle_alloc_error
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t7965470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384032528,"raw_hex":"e81e8b7900","size":5,"soft_fail":false,"target_va":5391998272,"va":5384032797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::collections::raw_vec::capacity_overflow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t7964528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384032528,"raw_hex":"e870877900","size":5,"soft_fail":false,"target_va":5391998384,"va":5384033851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18946223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384032528,"raw_hex":"e8af182101","size":5,"soft_fail":false,"target_va":5402980128,"va":5384033900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::shader::DescriptorSetState::allocate_descriptor_set
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7812738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384032528,"raw_hex":"e87ec988ff","size":5,"soft_fail":false,"target_va":5376219920,"va":5384032653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5384032528,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5384033610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::cmd_bind_descriptor_sets
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1332,"record_offset":32104,"parent_record_offset":27092,"ancestor_record_offset":27092,"inlinee":30907,"source_line":1761,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1332,"record_offset":32132,"parent_record_offset":27092,"ancestor_record_offset":27092,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1332,"record_offset":27512,"parent_record_offset":27092,"ancestor_record_offset":27092,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::update_descriptor_set
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/binding.rs
//     evidence={"module":1332,"record_offset":27568,"parent_record_offset":27092,"ancestor_record_offset":27092,"inlinee":62135,"source_line":96,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::update_descriptor_sets
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1332,"record_offset":30924,"parent_record_offset":27568,"ancestor_record_offset":27092,"inlinee":62141,"source_line":1356,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::dealloc
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1332,"record_offset":31092,"parent_record_offset":31024,"ancestor_record_offset":27092,"inlinee":17342,"source_line":1707,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::is_last_allocation
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1332,"record_offset":31124,"parent_record_offset":31092,"ancestor_record_offset":27092,"inlinee":17343,"source_line":1700,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1332,"record_offset":27936,"parent_record_offset":27908,"ancestor_record_offset":27092,"inlinee":6156,"source_line":1411,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1332,"record_offset":27972,"parent_record_offset":27936,"ancestor_record_offset":27092,"inlinee":6157,"source_line":1420,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::allocate_in
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":27712,"parent_record_offset":27676,"ancestor_record_offset":27092,"inlinee":62138,"source_line":85,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::current_layout
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":31056,"parent_record_offset":31024,"ancestor_record_offset":27092,"inlinee":17341,"source_line":158,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::dealloc_buffer
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":31024,"parent_record_offset":31000,"ancestor_record_offset":27092,"inlinee":17340,"source_line":650,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":29136,"parent_record_offset":29092,"ancestor_record_offset":27092,"inlinee":17334,"source_line":137,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::with_capacity_in
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":27676,"parent_record_offset":27636,"ancestor_record_offset":27092,"inlinee":62137,"source_line":74,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::impl$0::allocate_in::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":28272,"parent_record_offset":28248,"ancestor_record_offset":27092,"inlinee":62143,"source_line":91,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::impl$5::drop
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":31000,"parent_record_offset":30976,"ancestor_record_offset":27092,"inlinee":17339,"source_line":662,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1332,"record_offset":29092,"parent_record_offset":28988,"ancestor_record_offset":27092,"inlinee":17332,"source_line":1423,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::reserve
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1332,"record_offset":29064,"parent_record_offset":28988,"ancestor_record_offset":27092,"inlinee":17335,"source_line":735,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::with_capacity_in
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1332,"record_offset":27636,"parent_record_offset":27568,"ancestor_record_offset":27092,"inlinee":62136,"source_line":584,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::impl$11::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1332,"record_offset":30864,"parent_record_offset":27568,"ancestor_record_offset":27092,"inlinee":27046,"source_line":1943,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::impl$16::extend
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1332,"record_offset":28988,"parent_record_offset":27568,"ancestor_record_offset":27092,"inlinee":62140,"source_line":2023,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$13::alloc
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1332,"record_offset":27908,"parent_record_offset":27712,"ancestor_record_offset":27092,"inlinee":25380,"source_line":1864,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::from_size_align
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1332,"record_offset":27844,"parent_record_offset":27712,"ancestor_record_offset":27092,"inlinee":49215,"source_line":68,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1332,"record_offset":28148,"parent_record_offset":28124,"ancestor_record_offset":27092,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1332,"record_offset":28124,"parent_record_offset":27972,"ancestor_record_offset":27092,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1332,"record_offset":30544,"parent_record_offset":30520,"ancestor_record_offset":27092,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1332,"record_offset":30012,"parent_record_offset":29988,"ancestor_record_offset":27092,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1332,"record_offset":30096,"parent_record_offset":30072,"ancestor_record_offset":27092,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1332,"record_offset":30036,"parent_record_offset":30012,"ancestor_record_offset":27092,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1332,"record_offset":29344,"parent_record_offset":29316,"ancestor_record_offset":27092,"inlinee":8626,"source_line":73,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1332,"record_offset":29588,"parent_record_offset":29372,"ancestor_record_offset":27092,"inlinee":8628,"source_line":85,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1332,"record_offset":29260,"parent_record_offset":29232,"ancestor_record_offset":27092,"inlinee":8575,"source_line":59,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1332,"record_offset":29232,"parent_record_offset":28988,"ancestor_record_offset":27092,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1332,"record_offset":29316,"parent_record_offset":29288,"ancestor_record_offset":27092,"inlinee":6221,"source_line":116,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_try_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1332,"record_offset":29628,"parent_record_offset":29588,"ancestor_record_offset":27092,"inlinee":6183,"source_line":96,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1332,"record_offset":29036,"parent_record_offset":28988,"ancestor_record_offset":27092,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1332,"record_offset":29288,"parent_record_offset":29260,"ancestor_record_offset":27092,"inlinee":6348,"source_line":2866,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1332,"record_offset":29660,"parent_record_offset":29628,"ancestor_record_offset":27092,"inlinee":6192,"source_line":2873,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1332,"record_offset":29372,"parent_record_offset":29344,"ancestor_record_offset":27092,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1332,"record_offset":28172,"parent_record_offset":28148,"ancestor_record_offset":27092,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1332,"record_offset":27776,"parent_record_offset":27712,"ancestor_record_offset":27092,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1332,"record_offset":27808,"parent_record_offset":27776,"ancestor_record_offset":27092,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1332,"record_offset":30208,"parent_record_offset":30184,"ancestor_record_offset":27092,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::ControlFlow::break_value
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\control_flow.rs
//     evidence={"module":1332,"record_offset":30792,"parent_record_offset":29288,"ancestor_record_offset":27092,"inlinee":33084,"source_line":180,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1332,"record_offset":29692,"parent_record_offset":29660,"ancestor_record_offset":27092,"inlinee":4399,"source_line":293,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1332,"record_offset":30828,"parent_record_offset":29232,"ancestor_record_offset":27092,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1332,"record_offset":28248,"parent_record_offset":27712,"ancestor_record_offset":27092,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1332,"record_offset":30952,"parent_record_offset":27568,"ancestor_record_offset":27092,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1332,"record_offset":31180,"parent_record_offset":31092,"ancestor_record_offset":27092,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1332,"record_offset":30456,"parent_record_offset":30432,"ancestor_record_offset":27092,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1332,"record_offset":30348,"parent_record_offset":30324,"ancestor_record_offset":27092,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1332,"record_offset":28036,"parent_record_offset":28012,"ancestor_record_offset":27092,"inlinee":6159,"source_line":563,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1332,"record_offset":28012,"parent_record_offset":27972,"ancestor_record_offset":27092,"inlinee":6158,"source_line":1299,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1332,"record_offset":29548,"parent_record_offset":29520,"ancestor_record_offset":27092,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1332,"record_offset":29480,"parent_record_offset":29416,"ancestor_record_offset":27092,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1332,"record_offset":28196,"parent_record_offset":28172,"ancestor_record_offset":27092,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1332,"record_offset":27872,"parent_record_offset":27712,"ancestor_record_offset":27092,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1332,"record_offset":27536,"parent_record_offset":27512,"ancestor_record_offset":27092,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1332,"record_offset":29520,"parent_record_offset":29416,"ancestor_record_offset":27092,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1332,"record_offset":29416,"parent_record_offset":29372,"ancestor_record_offset":27092,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1332,"record_offset":30520,"parent_record_offset":30496,"ancestor_record_offset":27092,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1332,"record_offset":29776,"parent_record_offset":29748,"ancestor_record_offset":27092,"inlinee":8704,"source_line":1418,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1332,"record_offset":29804,"parent_record_offset":29776,"ancestor_record_offset":27092,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1332,"record_offset":30496,"parent_record_offset":30276,"ancestor_record_offset":27092,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1332,"record_offset":29840,"parent_record_offset":29804,"ancestor_record_offset":27092,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":30408,"parent_record_offset":30384,"ancestor_record_offset":27092,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":30384,"parent_record_offset":30276,"ancestor_record_offset":27092,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":30324,"parent_record_offset":30300,"ancestor_record_offset":27092,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":30704,"parent_record_offset":29916,"ancestor_record_offset":27092,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":30300,"parent_record_offset":30276,"ancestor_record_offset":27092,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":29892,"parent_record_offset":29868,"ancestor_record_offset":27092,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":29868,"parent_record_offset":29804,"ancestor_record_offset":27092,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":29916,"parent_record_offset":29892,"ancestor_record_offset":27092,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1332,"record_offset":30156,"parent_record_offset":30128,"ancestor_record_offset":27092,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1332,"record_offset":30184,"parent_record_offset":30156,"ancestor_record_offset":27092,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1332,"record_offset":30244,"parent_record_offset":30128,"ancestor_record_offset":27092,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1332,"record_offset":30128,"parent_record_offset":29916,"ancestor_record_offset":27092,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":29960,"parent_record_offset":29916,"ancestor_record_offset":27092,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":30276,"parent_record_offset":29916,"ancestor_record_offset":27092,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1332,"record_offset":29988,"parent_record_offset":29916,"ancestor_record_offset":27092,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1332,"record_offset":30072,"parent_record_offset":29916,"ancestor_record_offset":27092,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1332,"record_offset":30612,"parent_record_offset":29916,"ancestor_record_offset":27092,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::binding::impl$0::bind_descriptor_set::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/binding.rs
//     evidence={"module":1332,"record_offset":29720,"parent_record_offset":29692,"ancestor_record_offset":27092,"inlinee":61785,"source_line":77,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$58::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1332,"record_offset":30568,"parent_record_offset":30544,"ancestor_record_offset":27092,"inlinee":11856,"source_line":69,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1332,"record_offset":29748,"parent_record_offset":29720,"ancestor_record_offset":27092,"inlinee":17300,"source_line":1042,"archive":"_pdb/A/modules/01332.jsonl.gz"}
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
// 首条位置: {"module":1332,"record_offset":32936,"rva":15323408,"va":5384032528}
// 编译实例 RVAs: 0xe9d110
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$bumpalo..Bump$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-7131312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384032528,"raw_hex":"e8502f93ff","size":5,"soft_fail":false,"target_va":5376902512,"va":5384033819,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400dce30
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-14419381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384032528,"raw_hex":"e84bfa23ff","size":5,"soft_fail":false,"target_va":5369613872,"va":5384033248,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407d0550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7130152","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384032528,"raw_hex":"e8d83393ff","size":5,"soft_fail":false,"target_va":5376902480,"va":5384032627,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140e923b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-45349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384032528,"raw_hex":"e8db4effff","size":5,"soft_fail":false,"target_va":5383988144,"va":5384033488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14102e6c0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1643435","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384032528,"raw_hex":"e8ab131900","size":5,"soft_fail":false,"target_va":5385676480,"va":5384033040,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::Bump::alloc_layout_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-7130128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384032528,"raw_hex":"e8f03393ff","size":5,"soft_fail":false,"target_va":5376902656,"va":5384032779,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::alloc::handle_alloc_error
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t7965470","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384032528,"raw_hex":"e81e8b7900","size":5,"soft_fail":false,"target_va":5391998272,"va":5384032797,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::collections::raw_vec::capacity_overflow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t7964528","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384032528,"raw_hex":"e870877900","size":5,"soft_fail":false,"target_va":5391998384,"va":5384033851,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t18946223","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384032528,"raw_hex":"e8af182101","size":5,"soft_fail":false,"target_va":5402980128,"va":5384033900,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::shader::DescriptorSetState::allocate_descriptor_set
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-7812738","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5384032528,"raw_hex":"e87ec988ff","size":5,"soft_fail":false,"target_va":5376219920,"va":5384032653,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5384032528,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5384033610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::cmd_bind_descriptor_sets
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1332,"record_offset":37932,"parent_record_offset":32936,"ancestor_record_offset":32936,"inlinee":30907,"source_line":1761,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1332,"record_offset":37960,"parent_record_offset":32936,"ancestor_record_offset":32936,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":1332,"record_offset":33340,"parent_record_offset":32936,"ancestor_record_offset":32936,"inlinee":4364,"source_line":617,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::update_descriptor_set
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/binding.rs
//     evidence={"module":1332,"record_offset":33396,"parent_record_offset":32936,"ancestor_record_offset":32936,"inlinee":62135,"source_line":96,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::update_descriptor_sets
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1332,"record_offset":36752,"parent_record_offset":33396,"ancestor_record_offset":32936,"inlinee":62141,"source_line":1356,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::dealloc
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1332,"record_offset":36920,"parent_record_offset":36852,"ancestor_record_offset":32936,"inlinee":17342,"source_line":1707,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::is_last_allocation
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1332,"record_offset":36952,"parent_record_offset":36920,"ancestor_record_offset":32936,"inlinee":17343,"source_line":1700,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1332,"record_offset":33764,"parent_record_offset":33736,"ancestor_record_offset":32936,"inlinee":6156,"source_line":1411,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1332,"record_offset":33800,"parent_record_offset":33764,"ancestor_record_offset":32936,"inlinee":6157,"source_line":1420,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::allocate_in
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":33540,"parent_record_offset":33504,"ancestor_record_offset":32936,"inlinee":62138,"source_line":85,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::current_layout
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":36884,"parent_record_offset":36852,"ancestor_record_offset":32936,"inlinee":17341,"source_line":158,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::dealloc_buffer
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":36852,"parent_record_offset":36828,"ancestor_record_offset":32936,"inlinee":17340,"source_line":650,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":34964,"parent_record_offset":34920,"ancestor_record_offset":32936,"inlinee":17334,"source_line":137,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::with_capacity_in
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":33504,"parent_record_offset":33464,"ancestor_record_offset":32936,"inlinee":62137,"source_line":74,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::impl$0::allocate_in::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":34100,"parent_record_offset":34076,"ancestor_record_offset":32936,"inlinee":62143,"source_line":91,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::impl$5::drop
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":36828,"parent_record_offset":36804,"ancestor_record_offset":32936,"inlinee":17339,"source_line":662,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::push
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1332,"record_offset":34920,"parent_record_offset":34816,"ancestor_record_offset":32936,"inlinee":17332,"source_line":1423,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::reserve
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1332,"record_offset":34892,"parent_record_offset":34816,"ancestor_record_offset":32936,"inlinee":17335,"source_line":735,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::with_capacity_in
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1332,"record_offset":33464,"parent_record_offset":33396,"ancestor_record_offset":32936,"inlinee":62136,"source_line":584,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::impl$11::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1332,"record_offset":36692,"parent_record_offset":33396,"ancestor_record_offset":32936,"inlinee":27046,"source_line":1943,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::impl$16::extend
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1332,"record_offset":34816,"parent_record_offset":33396,"ancestor_record_offset":32936,"inlinee":62140,"source_line":2023,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$13::alloc
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1332,"record_offset":33736,"parent_record_offset":33540,"ancestor_record_offset":32936,"inlinee":25380,"source_line":1864,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::from_size_align
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1332,"record_offset":33672,"parent_record_offset":33540,"ancestor_record_offset":32936,"inlinee":49215,"source_line":68,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1332,"record_offset":33976,"parent_record_offset":33952,"ancestor_record_offset":32936,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1332,"record_offset":33952,"parent_record_offset":33800,"ancestor_record_offset":32936,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1332,"record_offset":36372,"parent_record_offset":36348,"ancestor_record_offset":32936,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1332,"record_offset":35840,"parent_record_offset":35816,"ancestor_record_offset":32936,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1332,"record_offset":35924,"parent_record_offset":35900,"ancestor_record_offset":32936,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1332,"record_offset":35864,"parent_record_offset":35840,"ancestor_record_offset":32936,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1332,"record_offset":35172,"parent_record_offset":35144,"ancestor_record_offset":32936,"inlinee":8626,"source_line":73,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold::enumerate::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1332,"record_offset":35416,"parent_record_offset":35200,"ancestor_record_offset":32936,"inlinee":8628,"source_line":85,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1332,"record_offset":35088,"parent_record_offset":35060,"ancestor_record_offset":32936,"inlinee":8575,"source_line":59,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1332,"record_offset":35060,"parent_record_offset":34816,"ancestor_record_offset":32936,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1332,"record_offset":35144,"parent_record_offset":35116,"ancestor_record_offset":32936,"inlinee":6221,"source_line":116,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_try_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1332,"record_offset":35456,"parent_record_offset":35416,"ancestor_record_offset":32936,"inlinee":6183,"source_line":96,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1332,"record_offset":34864,"parent_record_offset":34816,"ancestor_record_offset":32936,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1332,"record_offset":35116,"parent_record_offset":35088,"ancestor_record_offset":32936,"inlinee":6348,"source_line":2866,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1332,"record_offset":35488,"parent_record_offset":35456,"ancestor_record_offset":32936,"inlinee":6192,"source_line":2873,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1332,"record_offset":35200,"parent_record_offset":35172,"ancestor_record_offset":32936,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1332,"record_offset":34000,"parent_record_offset":33976,"ancestor_record_offset":32936,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1332,"record_offset":33604,"parent_record_offset":33540,"ancestor_record_offset":32936,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1332,"record_offset":33636,"parent_record_offset":33604,"ancestor_record_offset":32936,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1332,"record_offset":36036,"parent_record_offset":36012,"ancestor_record_offset":32936,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::ControlFlow::break_value
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\control_flow.rs
//     evidence={"module":1332,"record_offset":36620,"parent_record_offset":35116,"ancestor_record_offset":32936,"inlinee":33084,"source_line":180,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1332,"record_offset":35520,"parent_record_offset":35488,"ancestor_record_offset":32936,"inlinee":4399,"source_line":293,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1332,"record_offset":36656,"parent_record_offset":35060,"ancestor_record_offset":32936,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1332,"record_offset":34076,"parent_record_offset":33540,"ancestor_record_offset":32936,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1332,"record_offset":36780,"parent_record_offset":33396,"ancestor_record_offset":32936,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1332,"record_offset":37008,"parent_record_offset":36920,"ancestor_record_offset":32936,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1332,"record_offset":36284,"parent_record_offset":36260,"ancestor_record_offset":32936,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1332,"record_offset":36176,"parent_record_offset":36152,"ancestor_record_offset":32936,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1332,"record_offset":33864,"parent_record_offset":33840,"ancestor_record_offset":32936,"inlinee":6159,"source_line":563,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1332,"record_offset":33840,"parent_record_offset":33800,"ancestor_record_offset":32936,"inlinee":6158,"source_line":1299,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1332,"record_offset":35376,"parent_record_offset":35348,"ancestor_record_offset":32936,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1332,"record_offset":35308,"parent_record_offset":35244,"ancestor_record_offset":32936,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1332,"record_offset":34024,"parent_record_offset":34000,"ancestor_record_offset":32936,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1332,"record_offset":33700,"parent_record_offset":33540,"ancestor_record_offset":32936,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":1332,"record_offset":33364,"parent_record_offset":33340,"ancestor_record_offset":32936,"inlinee":4366,"source_line":220,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1332,"record_offset":35348,"parent_record_offset":35244,"ancestor_record_offset":32936,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1332,"record_offset":35244,"parent_record_offset":35200,"ancestor_record_offset":32936,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1332,"record_offset":36348,"parent_record_offset":36324,"ancestor_record_offset":32936,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1332,"record_offset":35604,"parent_record_offset":35576,"ancestor_record_offset":32936,"inlinee":8704,"source_line":1418,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1332,"record_offset":35632,"parent_record_offset":35604,"ancestor_record_offset":32936,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1332,"record_offset":36324,"parent_record_offset":36104,"ancestor_record_offset":32936,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1332,"record_offset":35668,"parent_record_offset":35632,"ancestor_record_offset":32936,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":36236,"parent_record_offset":36212,"ancestor_record_offset":32936,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":36212,"parent_record_offset":36104,"ancestor_record_offset":32936,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":36152,"parent_record_offset":36128,"ancestor_record_offset":32936,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":36532,"parent_record_offset":35744,"ancestor_record_offset":32936,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":36128,"parent_record_offset":36104,"ancestor_record_offset":32936,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":35720,"parent_record_offset":35696,"ancestor_record_offset":32936,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":35696,"parent_record_offset":35632,"ancestor_record_offset":32936,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":35744,"parent_record_offset":35720,"ancestor_record_offset":32936,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1332,"record_offset":35984,"parent_record_offset":35956,"ancestor_record_offset":32936,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1332,"record_offset":36012,"parent_record_offset":35984,"ancestor_record_offset":32936,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1332,"record_offset":36072,"parent_record_offset":35956,"ancestor_record_offset":32936,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1332,"record_offset":35956,"parent_record_offset":35744,"ancestor_record_offset":32936,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":35788,"parent_record_offset":35744,"ancestor_record_offset":32936,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":36104,"parent_record_offset":35744,"ancestor_record_offset":32936,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1332,"record_offset":35816,"parent_record_offset":35744,"ancestor_record_offset":32936,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=2
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1332,"record_offset":35900,"parent_record_offset":35744,"ancestor_record_offset":32936,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=1
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1332,"record_offset":36440,"parent_record_offset":35744,"ancestor_record_offset":32936,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::binding::impl$0::bind_descriptor_set::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/binding.rs
//     evidence={"module":1332,"record_offset":35548,"parent_record_offset":35520,"ancestor_record_offset":32936,"inlinee":61785,"source_line":77,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$58::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1332,"record_offset":36396,"parent_record_offset":36372,"ancestor_record_offset":32936,"inlinee":11856,"source_line":69,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1332,"record_offset":35576,"parent_record_offset":35548,"ancestor_record_offset":32936,"inlinee":17300,"source_line":1042,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::binding::impl$0::bind_descriptor_set::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=13
// 源码行: 77
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF159
// 首条位置: {"module":1321,"inlinee":61785}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1321,"record_offset":71960,"parent_record_offset":71936,"ancestor_record_offset":71936,"inlinee":17300,"source_line":1042,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1321,"record_offset":72740,"parent_record_offset":72716,"ancestor_record_offset":71936,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1321,"record_offset":72208,"parent_record_offset":72184,"ancestor_record_offset":71936,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1321,"record_offset":72292,"parent_record_offset":72268,"ancestor_record_offset":71936,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1321,"record_offset":72232,"parent_record_offset":72208,"ancestor_record_offset":71936,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1321,"record_offset":72404,"parent_record_offset":72380,"ancestor_record_offset":71936,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1321,"record_offset":72652,"parent_record_offset":72628,"ancestor_record_offset":71936,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1321,"record_offset":72544,"parent_record_offset":72520,"ancestor_record_offset":71936,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1321,"record_offset":72716,"parent_record_offset":72692,"ancestor_record_offset":71936,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1321,"record_offset":71984,"parent_record_offset":71960,"ancestor_record_offset":71936,"inlinee":8704,"source_line":1418,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1321,"record_offset":72008,"parent_record_offset":71984,"ancestor_record_offset":71936,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1321,"record_offset":72692,"parent_record_offset":72472,"ancestor_record_offset":71936,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1321,"record_offset":72036,"parent_record_offset":72008,"ancestor_record_offset":71936,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1321,"record_offset":72604,"parent_record_offset":72580,"ancestor_record_offset":71936,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1321,"record_offset":72580,"parent_record_offset":72472,"ancestor_record_offset":71936,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1321,"record_offset":72520,"parent_record_offset":72496,"ancestor_record_offset":71936,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1321,"record_offset":72900,"parent_record_offset":72112,"ancestor_record_offset":71936,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1321,"record_offset":72496,"parent_record_offset":72472,"ancestor_record_offset":71936,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1321,"record_offset":72088,"parent_record_offset":72064,"ancestor_record_offset":71936,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1321,"record_offset":72064,"parent_record_offset":72008,"ancestor_record_offset":71936,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1321,"record_offset":72112,"parent_record_offset":72088,"ancestor_record_offset":71936,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1321,"record_offset":72352,"parent_record_offset":72324,"ancestor_record_offset":71936,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1321,"record_offset":72380,"parent_record_offset":72352,"ancestor_record_offset":71936,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1321,"record_offset":72440,"parent_record_offset":72324,"ancestor_record_offset":71936,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1321,"record_offset":72324,"parent_record_offset":72112,"ancestor_record_offset":71936,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1321,"record_offset":72156,"parent_record_offset":72112,"ancestor_record_offset":71936,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1321,"record_offset":72472,"parent_record_offset":72112,"ancestor_record_offset":71936,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1321,"record_offset":72184,"parent_record_offset":72112,"ancestor_record_offset":71936,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=12
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1321,"record_offset":72268,"parent_record_offset":72112,"ancestor_record_offset":71936,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1321,"record_offset":72808,"parent_record_offset":72112,"ancestor_record_offset":71936,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$58::eq
//     owner=private_source_ownership_candidate; count=6
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1321,"record_offset":72764,"parent_record_offset":72740,"ancestor_record_offset":71936,"inlinee":11856,"source_line":69,"archive":"_pdb/A/modules/01321.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::update_descriptor_set
// 记录形式: inline_instance, inlinee_declaration; instances=10
// 源码行: 96
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF2B7
// 首条位置: {"module":1332,"inlinee":62135}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::update_descriptor_sets
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1332,"record_offset":4612,"parent_record_offset":2076,"ancestor_record_offset":2076,"inlinee":62141,"source_line":1356,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] bumpalo::collections::vec::Vec::with_capacity_in
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1332,"record_offset":2148,"parent_record_offset":2076,"ancestor_record_offset":2076,"inlinee":62136,"source_line":584,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] bumpalo::collections::vec::impl$11::deref
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1332,"record_offset":4552,"parent_record_offset":2076,"ancestor_record_offset":2076,"inlinee":27046,"source_line":1943,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] bumpalo::collections::vec::impl$16::extend
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1332,"record_offset":3264,"parent_record_offset":2076,"ancestor_record_offset":2076,"inlinee":62140,"source_line":2023,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=17
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1332,"record_offset":4640,"parent_record_offset":2076,"ancestor_record_offset":2076,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1332,"record_offset":21296,"parent_record_offset":21272,"ancestor_record_offset":19100,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":1332,"record_offset":21272,"parent_record_offset":21180,"ancestor_record_offset":19100,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1332,"record_offset":21208,"parent_record_offset":21180,"ancestor_record_offset":19100,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":1332,"record_offset":21180,"parent_record_offset":21156,"ancestor_record_offset":19100,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1332,"record_offset":4384,"parent_record_offset":4340,"ancestor_record_offset":2076,"inlinee":4292,"source_line":2202,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::set_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":1332,"record_offset":4496,"parent_record_offset":4340,"ancestor_record_offset":2076,"inlinee":5993,"source_line":1455,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":1332,"record_offset":3604,"parent_record_offset":3576,"ancestor_record_offset":2076,"inlinee":6077,"source_line":156,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::next::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":1332,"record_offset":3796,"parent_record_offset":3772,"ancestor_record_offset":2076,"inlinee":7344,"source_line":157,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$5::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":1332,"record_offset":3400,"parent_record_offset":3376,"ancestor_record_offset":2076,"inlinee":9113,"source_line":160,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$7::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":1332,"record_offset":4140,"parent_record_offset":4116,"ancestor_record_offset":2076,"inlinee":5995,"source_line":175,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::drain::impl$7::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\drain.rs
//     evidence={"module":1332,"record_offset":4340,"parent_record_offset":4312,"ancestor_record_offset":2076,"inlinee":5997,"source_line":180,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1332,"record_offset":21084,"parent_record_offset":21060,"ancestor_record_offset":19100,"inlinee":4436,"source_line":427,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$15::drop::impl$0::drop
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\into_iter.rs
//     evidence={"module":1332,"record_offset":21132,"parent_record_offset":21108,"ancestor_record_offset":19100,"inlinee":4438,"source_line":431,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::dealloc
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1332,"record_offset":4780,"parent_record_offset":4712,"ancestor_record_offset":2076,"inlinee":17342,"source_line":1707,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::is_last_allocation
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1332,"record_offset":4812,"parent_record_offset":4780,"ancestor_record_offset":2076,"inlinee":17343,"source_line":1700,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1332,"record_offset":2276,"parent_record_offset":2248,"ancestor_record_offset":2076,"inlinee":6156,"source_line":1411,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1332,"record_offset":2312,"parent_record_offset":2276,"ancestor_record_offset":2076,"inlinee":6157,"source_line":1420,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::allocate_in
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":2208,"parent_record_offset":2180,"ancestor_record_offset":2076,"inlinee":62138,"source_line":85,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::current_layout
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":4744,"parent_record_offset":4712,"ancestor_record_offset":2076,"inlinee":17341,"source_line":158,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::dealloc_buffer
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":4712,"parent_record_offset":4688,"ancestor_record_offset":2076,"inlinee":17340,"source_line":650,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::ptr
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":3972,"parent_record_offset":3928,"ancestor_record_offset":2076,"inlinee":17334,"source_line":137,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::with_capacity_in
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":2180,"parent_record_offset":2148,"ancestor_record_offset":2076,"inlinee":62137,"source_line":74,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::impl$0::allocate_in::closure$0
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":3220,"parent_record_offset":3196,"ancestor_record_offset":2076,"inlinee":62143,"source_line":91,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::impl$5::drop
//     owner=registry_dependency_definition; count=15
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1332,"record_offset":4688,"parent_record_offset":4664,"ancestor_record_offset":2076,"inlinee":17339,"source_line":662,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::push
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1332,"record_offset":3928,"parent_record_offset":3264,"ancestor_record_offset":2076,"inlinee":17332,"source_line":1423,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::reserve
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1332,"record_offset":3520,"parent_record_offset":3264,"ancestor_record_offset":2076,"inlinee":17335,"source_line":735,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$13::alloc
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1332,"record_offset":2248,"parent_record_offset":2208,"ancestor_record_offset":2076,"inlinee":25380,"source_line":1864,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::from_size_align
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1332,"record_offset":2792,"parent_record_offset":2668,"ancestor_record_offset":2076,"inlinee":49215,"source_line":68,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1332,"record_offset":2488,"parent_record_offset":2464,"ancestor_record_offset":2076,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1332,"record_offset":2464,"parent_record_offset":2312,"ancestor_record_offset":2076,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1332,"record_offset":22596,"parent_record_offset":22572,"ancestor_record_offset":19100,"inlinee":4504,"source_line":1654,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1332,"record_offset":22064,"parent_record_offset":22040,"ancestor_record_offset":19100,"inlinee":4833,"source_line":1238,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":1332,"record_offset":22148,"parent_record_offset":22124,"ancestor_record_offset":19100,"inlinee":4835,"source_line":1442,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1332,"record_offset":4468,"parent_record_offset":4340,"ancestor_record_offset":2076,"inlinee":5999,"source_line":3064,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":1332,"record_offset":22088,"parent_record_offset":22064,"ancestor_record_offset":19100,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::and_then_or_clear
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1332,"record_offset":20868,"parent_record_offset":20832,"ancestor_record_offset":19100,"inlinee":8622,"source_line":299,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::find
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1332,"record_offset":20832,"parent_record_offset":20800,"ancestor_record_offset":19100,"inlinee":62191,"source_line":134,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::find::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1332,"record_offset":20928,"parent_record_offset":20868,"ancestor_record_offset":19100,"inlinee":62193,"source_line":138,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::chain::impl$1::find::closure$1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\chain.rs
//     evidence={"module":1332,"record_offset":21388,"parent_record_offset":21364,"ancestor_record_offset":19100,"inlinee":62194,"source_line":139,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1332,"record_offset":29344,"parent_record_offset":29316,"ancestor_record_offset":27568,"inlinee":8626,"source_line":73,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::try_fold::enumerate::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":1332,"record_offset":29588,"parent_record_offset":29372,"ancestor_record_offset":27568,"inlinee":8628,"source_line":85,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::filter::impl$2::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\filter.rs
//     evidence={"module":1332,"record_offset":20800,"parent_record_offset":20768,"ancestor_record_offset":19100,"inlinee":8575,"source_line":59,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1332,"record_offset":3548,"parent_record_offset":3264,"ancestor_record_offset":2076,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::size_hint
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1332,"record_offset":3352,"parent_record_offset":3264,"ancestor_record_offset":2076,"inlinee":6217,"source_line":112,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::try_fold
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1332,"record_offset":21464,"parent_record_offset":21440,"ancestor_record_offset":19100,"inlinee":6221,"source_line":116,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_try_fold::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1332,"record_offset":21692,"parent_record_offset":21488,"ancestor_record_offset":19100,"inlinee":6183,"source_line":96,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\collect.rs
//     evidence={"module":1332,"record_offset":3324,"parent_record_offset":3264,"ancestor_record_offset":2076,"inlinee":7553,"source_line":321,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1332,"record_offset":20952,"parent_record_offset":20928,"ancestor_record_offset":19100,"inlinee":6348,"source_line":2866,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::find::check::closure$0
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1332,"record_offset":21720,"parent_record_offset":21692,"ancestor_record_offset":19100,"inlinee":6192,"source_line":2873,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1332,"record_offset":21488,"parent_record_offset":21464,"ancestor_record_offset":19100,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=26
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1332,"record_offset":2512,"parent_record_offset":2488,"ancestor_record_offset":2076,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::take
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1332,"record_offset":4188,"parent_record_offset":4140,"ancestor_record_offset":2076,"inlinee":5326,"source_line":845,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1332,"record_offset":2724,"parent_record_offset":2668,"ancestor_record_offset":2076,"inlinee":5978,"source_line":686,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1332,"record_offset":2756,"parent_record_offset":2724,"ancestor_record_offset":2076,"inlinee":5979,"source_line":2173,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1332,"record_offset":21236,"parent_record_offset":21208,"ancestor_record_offset":19100,"inlinee":4440,"source_line":746,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\nonzero.rs
//     evidence={"module":1332,"record_offset":22260,"parent_record_offset":22236,"ancestor_record_offset":19100,"inlinee":4842,"source_line":504,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::ControlFlow::break_value
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\control_flow.rs
//     evidence={"module":1332,"record_offset":20976,"parent_record_offset":20952,"ancestor_record_offset":19100,"inlinee":33084,"source_line":180,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::control_flow::impl$0::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\control_flow.rs
//     evidence={"module":1332,"record_offset":22856,"parent_record_offset":21488,"ancestor_record_offset":19100,"inlinee":21161,"source_line":110,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$3::call_mut
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1332,"record_offset":21748,"parent_record_offset":21720,"ancestor_record_offset":19100,"inlinee":4399,"source_line":293,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_mut
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1332,"record_offset":20900,"parent_record_offset":20868,"ancestor_record_offset":19100,"inlinee":4215,"source_line":704,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1332,"record_offset":3772,"parent_record_offset":3604,"ancestor_record_offset":2076,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::or_else
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1332,"record_offset":21364,"parent_record_offset":20832,"ancestor_record_offset":19100,"inlinee":5273,"source_line":1507,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1332,"record_offset":3196,"parent_record_offset":2668,"ancestor_record_offset":2076,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1332,"record_offset":4412,"parent_record_offset":4340,"ancestor_record_offset":2076,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1332,"record_offset":3472,"parent_record_offset":3448,"ancestor_record_offset":2076,"inlinee":6047,"source_line":809,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=89
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1332,"record_offset":4068,"parent_record_offset":3264,"ancestor_record_offset":2076,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1332,"record_offset":4440,"parent_record_offset":4340,"ancestor_record_offset":2076,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1332,"record_offset":22508,"parent_record_offset":22484,"ancestor_record_offset":19100,"inlinee":4291,"source_line":481,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1332,"record_offset":22400,"parent_record_offset":22376,"ancestor_record_offset":19100,"inlinee":4849,"source_line":1130,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1332,"record_offset":2376,"parent_record_offset":2352,"ancestor_record_offset":2076,"inlinee":6159,"source_line":563,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=20
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1332,"record_offset":2352,"parent_record_offset":2312,"ancestor_record_offset":2076,"inlinee":6158,"source_line":1299,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1332,"record_offset":3736,"parent_record_offset":3712,"ancestor_record_offset":2076,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1332,"record_offset":3448,"parent_record_offset":3424,"ancestor_record_offset":2076,"inlinee":6046,"source_line":953,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=19
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1332,"record_offset":3680,"parent_record_offset":3632,"ancestor_record_offset":2076,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1332,"record_offset":3820,"parent_record_offset":3796,"ancestor_record_offset":2076,"inlinee":4136,"source_line":1246,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=31
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1332,"record_offset":2536,"parent_record_offset":2512,"ancestor_record_offset":2076,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1332,"record_offset":2820,"parent_record_offset":2668,"ancestor_record_offset":2076,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1332,"record_offset":3712,"parent_record_offset":3632,"ancestor_record_offset":2076,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1332,"record_offset":3632,"parent_record_offset":3604,"ancestor_record_offset":2076,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1332,"record_offset":3424,"parent_record_offset":3400,"ancestor_record_offset":2076,"inlinee":6061,"source_line":171,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure$2
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/pass_builder/compute_pass_builder.rs
//     evidence={"module":1332,"record_offset":22824,"parent_record_offset":21692,"ancestor_record_offset":19100,"inlinee":61786,"source_line":527,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\lib.rs
//     evidence={"module":1332,"record_offset":22572,"parent_record_offset":22548,"ancestor_record_offset":19100,"inlinee":4854,"source_line":170,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1332,"record_offset":21832,"parent_record_offset":21804,"ancestor_record_offset":19100,"inlinee":8704,"source_line":1418,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::get_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1332,"record_offset":21860,"parent_record_offset":21832,"ancestor_record_offset":19100,"inlinee":5562,"source_line":1351,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1332,"record_offset":22548,"parent_record_offset":22328,"ancestor_record_offset":19100,"inlinee":4852,"source_line":229,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\map.rs
//     evidence={"module":1332,"record_offset":21892,"parent_record_offset":21860,"ancestor_record_offset":19100,"inlinee":5617,"source_line":257,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ptr
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":22460,"parent_record_offset":22436,"ancestor_record_offset":19100,"inlinee":5640,"source_line":513,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::as_ref
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":22436,"parent_record_offset":22328,"ancestor_record_offset":19100,"inlinee":5639,"source_line":682,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":22376,"parent_record_offset":22352,"ancestor_record_offset":19100,"inlinee":5638,"source_line":355,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":22756,"parent_record_offset":21968,"ancestor_record_offset":19100,"inlinee":5645,"source_line":175,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":22352,"parent_record_offset":22328,"ancestor_record_offset":19100,"inlinee":5636,"source_line":997,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":21944,"parent_record_offset":21920,"ancestor_record_offset":19100,"inlinee":5620,"source_line":1456,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":21920,"parent_record_offset":21860,"ancestor_record_offset":19100,"inlinee":7330,"source_line":1479,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":21968,"parent_record_offset":21944,"ancestor_record_offset":19100,"inlinee":5622,"source_line":2168,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::lowest_set_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1332,"record_offset":22208,"parent_record_offset":22180,"ancestor_record_offset":19100,"inlinee":5631,"source_line":49,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::nonzero_trailing_zeros
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1332,"record_offset":22236,"parent_record_offset":22208,"ancestor_record_offset":19100,"inlinee":5632,"source_line":74,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::BitMask::remove_lowest_bit
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1332,"record_offset":22296,"parent_record_offset":22180,"ancestor_record_offset":19100,"inlinee":5633,"source_line":37,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::bitmask::impl$3::next
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\bitmask.rs
//     evidence={"module":1332,"record_offset":22180,"parent_record_offset":21968,"ancestor_record_offset":19100,"inlinee":5629,"source_line":128,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::h2
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":22012,"parent_record_offset":21968,"ancestor_record_offset":19100,"inlinee":5624,"source_line":150,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\mod.rs
//     evidence={"module":1332,"record_offset":22328,"parent_record_offset":21968,"ancestor_record_offset":19100,"inlinee":5635,"source_line":1466,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::load
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1332,"record_offset":22040,"parent_record_offset":21968,"ancestor_record_offset":19100,"inlinee":5626,"source_line":52,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_byte
//     owner=unresolved_namespace; count=6
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1332,"record_offset":22124,"parent_record_offset":21968,"ancestor_record_offset":19100,"inlinee":5627,"source_line":79,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::sse2::Group::match_empty
//     owner=unresolved_namespace; count=3
//     definition_source=/rust/deps\hashbrown-0.14.3\src\raw\sse2.rs
//     evidence={"module":1332,"record_offset":22664,"parent_record_offset":21968,"ancestor_record_offset":19100,"inlinee":5643,"source_line":97,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::binding::impl$0::bind_descriptor_set::closure$0
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/binding.rs
//     evidence={"module":1332,"record_offset":21776,"parent_record_offset":21748,"ancestor_record_offset":19100,"inlinee":61785,"source_line":77,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] shader_pipeline::reflection::impl$58::eq
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/crates/shader-pipeline/src/reflection.rs
//     evidence={"module":1332,"record_offset":22620,"parent_record_offset":22596,"ancestor_record_offset":19100,"inlinee":11856,"source_line":69,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::HashMap::contains_key
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\collections\hash\map.rs
//     evidence={"module":1332,"record_offset":21804,"parent_record_offset":21776,"ancestor_record_offset":19100,"inlinee":17300,"source_line":1042,"archive":"_pdb/A/modules/01332.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::binding::impl$0::update_descriptor_set::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 109
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0xF242
// 首条位置: {"module":1329,"inlinee":62018}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::definitions::WriteDescriptorSet::buffer_info
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1329,"record_offset":9004,"parent_record_offset":4096,"ancestor_record_offset":4096,"inlinee":62020,"source_line":1993,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::WriteDescriptorSet::image_info
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1329,"record_offset":6036,"parent_record_offset":4096,"ancestor_record_offset":4096,"inlinee":62019,"source_line":1987,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::WriteDescriptorSet::push_next
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":1329,"record_offset":9032,"parent_record_offset":4096,"ancestor_record_offset":4096,"inlinee":60374,"source_line":2009,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_direct] bumpalo::Bump::alloc
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1329,"record_offset":4256,"parent_record_offset":4096,"ancestor_record_offset":4096,"inlinee":6153,"source_line":805,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_direct] bumpalo::collections::vec::Vec::push
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1329,"record_offset":5360,"parent_record_offset":4096,"ancestor_record_offset":4096,"inlinee":17332,"source_line":1423,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::ptr_chain_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk.rs
//     evidence={"module":1329,"record_offset":9180,"parent_record_offset":9156,"ancestor_record_offset":4096,"inlinee":6177,"source_line":39,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_layout
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1329,"record_offset":4316,"parent_record_offset":4284,"ancestor_record_offset":4096,"inlinee":6155,"source_line":1396,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_with
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1329,"record_offset":4284,"parent_record_offset":4256,"ancestor_record_offset":4096,"inlinee":6154,"source_line":850,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1329,"record_offset":4344,"parent_record_offset":4316,"ancestor_record_offset":4096,"inlinee":6156,"source_line":1411,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1329,"record_offset":4380,"parent_record_offset":4344,"ancestor_record_offset":4096,"inlinee":6157,"source_line":1420,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::raw_vec::RawVec::ptr
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/raw_vec.rs
//     evidence={"module":1329,"record_offset":5424,"parent_record_offset":5360,"ancestor_record_offset":4096,"inlinee":17334,"source_line":137,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::collections::vec::Vec::reserve
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/collections/vec.rs
//     evidence={"module":1329,"record_offset":5396,"parent_record_offset":5360,"ancestor_record_offset":4096,"inlinee":17335,"source_line":735,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$8::alloc_with::inner_writer
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1329,"record_offset":4744,"parent_record_offset":4284,"ancestor_record_offset":4096,"inlinee":6162,"source_line":855,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1329,"record_offset":4424,"parent_record_offset":4380,"ancestor_record_offset":4096,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1329,"record_offset":4616,"parent_record_offset":4592,"ancestor_record_offset":4096,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1329,"record_offset":4592,"parent_record_offset":4380,"ancestor_record_offset":4096,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\mod.rs
//     evidence={"module":1329,"record_offset":9084,"parent_record_offset":9060,"ancestor_record_offset":4096,"inlinee":6172,"source_line":372,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\scan.rs
//     evidence={"module":1329,"record_offset":9108,"parent_record_offset":9084,"ancestor_record_offset":4096,"inlinee":6173,"source_line":56,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::scan::impl$2::try_fold::scan::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\scan.rs
//     evidence={"module":1329,"record_offset":9156,"parent_record_offset":9132,"ancestor_record_offset":4096,"inlinee":6175,"source_line":67,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::last
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1329,"record_offset":9060,"parent_record_offset":9032,"ancestor_record_offset":4096,"inlinee":6170,"source_line":258,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":1329,"record_offset":9132,"parent_record_offset":9108,"ancestor_record_offset":4096,"inlinee":4926,"source_line":2401,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1329,"record_offset":4640,"parent_record_offset":4616,"ancestor_record_offset":4096,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1329,"record_offset":4504,"parent_record_offset":4480,"ancestor_record_offset":4096,"inlinee":6159,"source_line":563,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1329,"record_offset":4480,"parent_record_offset":4380,"ancestor_record_offset":4096,"inlinee":6158,"source_line":1299,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=18
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1329,"record_offset":4664,"parent_record_offset":4640,"ancestor_record_offset":4096,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=8
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1329,"record_offset":4712,"parent_record_offset":4316,"ancestor_record_offset":4096,"inlinee":6160,"source_line":1428,"archive":"_pdb/A/modules/01329.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::bind_persistent_descriptor_set
// 记录形式: compiled_procedure; instances=1
// 源码行: 180
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":570,"record_offset":7296,"rva":7572464,"va":5376281584}
// 编译实例 RVAs: 0x738bf0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1411775f0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: binding.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10742228","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376281584,"raw_hex":"e8d4e9a300","size":5,"soft_fail":false,"target_va":5387023856,"va":5376281623,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26697151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376281584,"raw_hex":"e8bf5d9701","size":5,"soft_fail":false,"target_va":5402978880,"va":5376281724,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26697399","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376281584,"raw_hex":"e8b75e9701","size":5,"soft_fail":false,"target_va":5402979152,"va":5376281748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*800(%rdi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5376281584,"raw_hex":"ff9720030000","size":6,"soft_fail":false,"va":5376281700,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::cmd_bind_descriptor_sets
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":570,"record_offset":7528,"parent_record_offset":7296,"ancestor_record_offset":7296,"inlinee":30907,"source_line":1761,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":7460,"parent_record_offset":7296,"ancestor_record_offset":7296,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::command_buffer::CommandBuffer::raw
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":570,"record_offset":7492,"parent_record_offset":7296,"ancestor_record_offset":7296,"inlinee":30906,"source_line":177,"archive":"_pdb/A/modules/00570.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 52
// [PDB 泛型关系，非字段关系] {"type": "enum2$<rhapsody::raw::binding::DescriptorSetBinding>", "constructor": "enum2$", "arguments": ["rhapsody::raw::binding::DescriptorSetBinding"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>", "constructor": "rhapsody::raw::managed::handle::GpuOwned", "arguments": ["enum2$<rhapsody::raw::binding::DescriptorSetBinding>", "rhapsody::raw::managed::handle::RawManagedHandle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >", "constructor": "tuple$", "arguments": ["shader_pipeline::reflection::BindingIndex", "rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "array$<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,2>", "constructor": "array$", "arguments": ["enum2$<rhapsody::raw::binding::DescriptorSetBinding>", "2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::handle::impl$21::clone<array$<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,2>,rhapsody::raw::managed::handle::RawManagedHandle>", "constructor": "rhapsody::raw::managed::handle::impl$21::clone", "arguments": ["array$<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,2>", "rhapsody::raw::managed::handle::RawManagedHandle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["shader_pipeline::reflection::BindingIndex", "rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> >", "constructor": "core::slice::iter::Iter", "arguments": ["enum2$<rhapsody::raw::binding::DescriptorSetBinding>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::cloned::impl$8::next_unchecked<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> >,enum2$<rhapsody::raw::binding::DescriptorSetBinding> >", "constructor": "core::iter::adapters::cloned::impl$8::next_unchecked", "arguments": ["core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> >", "enum2$<rhapsody::raw::binding::DescriptorSetBinding>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::handle::GpuOwnedInner<array$<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,2>,rhapsody::raw::managed::handle::RawManagedHandle>", "constructor": "rhapsody::raw::managed::handle::GpuOwnedInner", "arguments": ["array$<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,2>", "rhapsody::raw::managed::handle::RawManagedHandle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::managed::handle::GpuOwnedInner<array$<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,2>,rhapsody::raw::managed::handle::RawManagedHandle> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::managed::handle::GpuOwnedInner<array$<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,2>,rhapsody::raw::managed::handle::RawManagedHandle>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<rhapsody::raw::binding::DescriptorSetBinding> >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<rhapsody::raw::binding::DescriptorSetBinding>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<array$<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,2> >", "constructor": "core::ptr::drop_in_place", "arguments": ["array$<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::array::Guard<enum2$<rhapsody::raw::binding::DescriptorSetBinding> >", "constructor": "core::array::Guard", "arguments": ["enum2$<rhapsody::raw::binding::DescriptorSetBinding>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::array::Guard<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::array::Guard<enum2$<rhapsody::raw::binding::DescriptorSetBinding> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::handle::GpuOwnedInner<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>", "constructor": "rhapsody::raw::managed::handle::GpuOwnedInner", "arguments": ["enum2$<rhapsody::raw::binding::DescriptorSetBinding>", "rhapsody::raw::managed::handle::RawManagedHandle"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::mem::manually_drop::ManuallyDrop<rhapsody::raw::managed::handle::GpuOwnedInner<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >", "constructor": "core::mem::manually_drop::ManuallyDrop", "arguments": ["rhapsody::raw::managed::handle::GpuOwnedInner<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$51::fmt<core::mem::manually_drop::ManuallyDrop<rhapsody::raw::managed::handle::GpuOwnedInner<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >", "constructor": "core::fmt::impl$51::fmt", "arguments": ["core::mem::manually_drop::ManuallyDrop<rhapsody::raw::managed::handle::GpuOwnedInner<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<rhapsody::raw::managed::handle::GpuOwnedInner<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >", "constructor": "ref$", "arguments": ["rhapsody::raw::managed::handle::GpuOwnedInner<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<ref$<rhapsody::raw::managed::handle::GpuOwnedInner<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["ref$<rhapsody::raw::managed::handle::GpuOwnedInner<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::rg::pass_api::RenderPassBinding> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$0> >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["enum2$<rhapsody::raw::binding::DescriptorSetBinding>", "core::iter::adapters::map::Map<core::slice::iter::Iter<enum2$<rhapsody::rg::pass_api::RenderPassBinding> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$0>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2>", "constructor": "core::iter::adapters::map::Map", "arguments": ["std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >", "country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> >", "constructor": "core::iter::adapters::chain::Chain", "arguments": ["core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>", "core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$94::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > >", "constructor": "hashbrown::map::impl$94::extend::closure_env$0", "arguments": ["shader_pipeline::reflection::BindingIndex", "alloc::string::String", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$94::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > >", "constructor": "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>", "hashbrown::map::impl$94::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$94::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > >", "constructor": "ref_mut$", "arguments": ["core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$94::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::fold<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1,tuple$<>,ref_mut$<core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$94::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > > >", "constructor": "core::iter::adapters::map::impl$2::fold", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>", "core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >", "country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1", "tuple$<>", "ref_mut$<core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$94::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> >", "constructor": "ref$", "arguments": ["enum2$<rhapsody::raw::binding::DescriptorSetBinding>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "constructor": "tuple$", "arguments": ["shader_pipeline::reflection::BindingIndex", "shader_pipeline::reflection::BindingArrayElement", "ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > >", "constructor": "rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0", "arguments": ["core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > >", "constructor": "ref_mut$", "arguments": ["rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > >", "constructor": "ref_mut$", "arguments": ["ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > > >", "constructor": "core::iter::traits::iterator::Iterator::find::check::closure_env$0", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "ref_mut$<ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> >", "constructor": "core::ops::control_flow::ControlFlow", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > >", "constructor": "enum2$", "arguments": ["core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::try_fold<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > > >", "constructor": "core::iter::adapters::map::impl$2::try_fold", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>", "country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3", "tuple$<>", "core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > > >", "enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > >", "constructor": "core::iter::traits::iterator::Iterator::find::check::closure_env$0", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::try_fold<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > > >", "constructor": "core::iter::adapters::map::impl$2::try_fold", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >", "country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4", "tuple$<>", "core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > >", "enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > >", "constructor": "rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0", "arguments": ["core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > > >", "constructor": "ref_mut$", "arguments": ["rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref_mut$<ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > > > >", "constructor": "ref_mut$", "arguments": ["ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > > > > >", "constructor": "core::iter::traits::iterator::Iterator::find::check::closure_env$0", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "ref_mut$<ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::impl$2::try_fold<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > > > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > > >", "constructor": "core::iter::adapters::map::impl$2::try_fold", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>", "country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1", "tuple$<>", "core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > > > > >", "enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >", "constructor": "ref$", "arguments": ["rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<ref$<shader_pipeline::reflection::BindingIndex>,ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >", "constructor": "tuple$", "arguments": ["ref$<shader_pipeline::reflection::BindingIndex>", "ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<shader_pipeline::reflection::BindingIndex>,ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,tuple$<>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$94::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > >", "constructor": "core::iter::adapters::map::map_fold::closure_env$0", "arguments": ["tuple$<ref$<shader_pipeline::reflection::BindingIndex>,ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >", "tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>", "tuple$<>", "country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2", "core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$94::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$60::fold::closure_env$0<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<shader_pipeline::reflection::BindingIndex>,ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,tuple$<>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$94::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > > >", "constructor": "hashbrown::map::impl$60::fold::closure_env$0", "arguments": ["shader_pipeline::reflection::BindingIndex", "rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>", "tuple$<>", "core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<shader_pipeline::reflection::BindingIndex>,ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,tuple$<>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$94::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawIterRange::fold_impl<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,hashbrown::map::impl$60::fold::closure_env$0<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<shader_pipeline::reflection::BindingIndex>,ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,tuple$<>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$94::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > > >,tuple$<> >", "constructor": "hashbrown::raw::RawIterRange::fold_impl", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >", "hashbrown::map::impl$60::fold::closure_env$0<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>,tuple$<>,core::iter::adapters::map::map_fold::closure_env$0<tuple$<ref$<shader_pipeline::reflection::BindingIndex>,ref$<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,tuple$<>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2,core::iter::traits::iterator::Iterator::for_each::call::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,alloc::string::String>,hashbrown::map::impl$94::extend::closure_env$0<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > > > > >", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$51::fmt<rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >", "constructor": "core::fmt::impl$51::fmt", "arguments": ["rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$51::fmt<enum2$<rhapsody::raw::binding::DescriptorSetBinding> >", "constructor": "core::fmt::impl$51::fmt", "arguments": ["enum2$<rhapsody::raw::binding::DescriptorSetBinding>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$51::fmt<rhapsody::raw::managed::handle::GpuOwnedInner<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >", "constructor": "core::fmt::impl$51::fmt", "arguments": ["rhapsody::raw::managed::handle::GpuOwnedInner<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > > >", "constructor": "tuple$", "arguments": ["tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::filter::Filter<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> >,rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > >", "constructor": "core::iter::adapters::filter::Filter", "arguments": ["core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> >", "rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::binding::impl$0::update_descriptor_set::closure_env$0<core::iter::adapters::filter::Filter<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> >,rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > >", "constructor": "rhapsody::raw::binding::impl$0::update_descriptor_set::closure_env$0", "arguments": ["core::iter::adapters::filter::Filter<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> >,rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::impls::impl$4::call_once<tuple$<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > > >,rhapsody::raw::binding::impl$0::update_descriptor_set::closure_env$0<core::iter::adapters::filter::Filter<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> >,rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > > >", "constructor": "core::ops::function::impls::impl$4::call_once", "arguments": ["tuple$<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > > >", "rhapsody::raw::binding::impl$0::update_descriptor_set::closure_env$0<core::iter::adapters::filter::Filter<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> >,rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<core::mem::manually_drop::ManuallyDrop<rhapsody::raw::managed::handle::GpuOwnedInner<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >", "constructor": "ref$", "arguments": ["core::mem::manually_drop::ManuallyDrop<rhapsody::raw::managed::handle::GpuOwnedInner<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<ref$<core::mem::manually_drop::ManuallyDrop<rhapsody::raw::managed::handle::GpuOwnedInner<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["ref$<core::mem::manually_drop::ManuallyDrop<rhapsody::raw::managed::handle::GpuOwnedInner<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > >", "constructor": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set", "arguments": ["core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "constructor": "core::iter::adapters::enumerate::Enumerate", "arguments": ["core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$4>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$4"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$4> >", "constructor": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$4>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6>", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6> >", "constructor": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set", "arguments": ["core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > >", "constructor": "rhapsody::raw::device::GraphicsDevice::bind_descriptor_set", "arguments": ["core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::impl$94::extend<shader_pipeline::reflection::BindingIndex,alloc::string::String,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> > >", "constructor": "hashbrown::map::impl$94::extend", "arguments": ["shader_pipeline::reflection::BindingIndex", "alloc::string::String", "std::hash::random::RandomState", "alloc::alloc::Global", "core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$1>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$2> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["shader_pipeline::reflection::BindingIndex", "rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 1; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// 候选参数类型: ["country_core::resources::audio_resources::AudioInstance"]

// ------------------------------------------------------------------
// 函数: rhapsody::raw::binding::impl$0::bind_descriptor_set::closure$0
// 信息 ID: d8a84805f3b70bae6be57a66
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::binding::impl$0::bind_descriptor_set", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > > > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0 <- core::ops::function::impls::impl$4::call_once<tuple$<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > > >,rhapsody::raw::binding::impl$0::update_descriptor_set::closure_env$0<core::iter::adapters::filter::Filter<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> >,rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 6 条；闭包环境类型引用 4 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > > >; module=1321, inline_record=71936; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > > >; module=1321, inline_record=93340; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1,tuple$<>,core::iter::traits::iterator::Iterator::find::check::closure_env$0<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,ref_mut$<ref_mut$<rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > > > > >,enum2$<core::ops::control_flow::ControlFlow<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,tuple$<> > > >; module=1321, inline_record=104848; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > >; module=1332, inline_record=21776; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$4> >; module=1332, inline_record=29720; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6> >; module=1332, inline_record=35548; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::binding::impl$0::update_descriptor_set::closure$0
// 信息 ID: b073bebacea63710d63e0acd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::binding::impl$0::update_descriptor_set", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::binding::impl$0::update_descriptor_set::closure_env$0 <- core::ptr::drop_in_place<core::iter::adapters::map::Map<core::iter::adapters::map::Map<alloc::vec::drain::Drain<country_core::asset_libraries::texture_library::AssetIndex,alloc::alloc::Global>,country_core::asset_libraries::texture_library::impl$1::update_bindless_texture_table::closure_env$0>,rhapsody::raw::binding::impl$0::update_descriptor_set::closure_env$0<core::iter::adapters::map::Map<alloc::vec::drain::Drain<country_core::asset_libraries::texture_library::AssetIndex,alloc::alloc::Global>,country_core::asset_libraries::texture_library::impl$1::update_bindless_texture_table::closure_env$0> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::binding::impl$0::update_descriptor_set::closure_env$0 <- core::ops::function::impls::impl$4::call_once<tuple$<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > > >,rhapsody::raw::binding::impl$0::update_descriptor_set::closure_env$0<core::iter::adapters::filter::Filter<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> >,rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] core::ops::function::impls::impl$4::call_once<tuple$<tuple$<shader_pipeline::reflection::BindingIndex,shader_pipeline::reflection::BindingArrayElement,ref$<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > > >,rhapsody::raw::binding::impl$0::update_descriptor_set::closure_env$0<core::iter::adapters::filter::Filter<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> >,rhapsody::raw::binding::impl$0::bind_descriptor_set::closure_env$0<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > > > > >; module=1329, inline_record=4096; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::binding::impl$1::fmt
// 信息 ID: 281c0769c848cf0df7007304
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::fmt::impl$51::fmt<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > @ 0xe922c3 -> 0xe93d60 (direct_jump_outside_procedure); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::binding::impl$2::clone
// 信息 ID: 794f040df16d96f845c77ccf
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::iter::adapters::cloned::impl$8::next_unchecked<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> >,enum2$<rhapsody::raw::binding::DescriptorSetBinding> >; module=167, inline_record=5824; 不是独立调用边。
// [PDB 内联宿主] core::iter::adapters::cloned::impl$8::next_unchecked<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> >,enum2$<rhapsody::raw::binding::DescriptorSetBinding> >; module=466, inline_record=6272; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > >
// 信息 ID: 7b5b037bc28f229c0deeea23
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
// [静态直接引用] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0,alloc::alloc::Global> @ 0x1000f2f -> 0xe9c610 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > >
// 信息 ID: 0b22a9d39d35d458fb6166b6
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
// [静态直接引用] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x1002ff2 -> 0xe9d830 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$4> >
// 信息 ID: c1b8a928431266c1fa8d6e9b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["rhapsody::raw::binding::DescriptorSetBinding"]; 字段布局 = unknown。
// [类型关系] core::slice::iter::Iter -> ["enum2$<rhapsody::raw::binding::DescriptorSetBinding>"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::enumerate::Enumerate -> ["core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> >"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$4"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::bind_descriptor_set -> ["core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$4>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0,alloc::alloc::Global> @ 0x1000fe6 -> 0xe9d110 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x1003092 -> 0xe9d110 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6> >
// 信息 ID: ef606612e0d312d467342e2f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] enum2$ -> ["rhapsody::raw::binding::DescriptorSetBinding"]; 字段布局 = unknown。
// [类型关系] core::slice::iter::Iter -> ["enum2$<rhapsody::raw::binding::DescriptorSetBinding>"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::enumerate::Enumerate -> ["core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> >"]; 字段布局 = unknown。
// [类型关系] core::iter::adapters::map::Map -> ["core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >", "country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::bind_descriptor_set -> ["core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0,alloc::alloc::Global> @ 0x1000fe6 -> 0xe9d110 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x1003092 -> 0xe9d110 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::bind_persistent_descriptor_set
// 信息 ID: a9fbe34c5eb2ae7ecc427ebb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x1003102 -> 0x738bf0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::bind_pipeline
// 信息 ID: e1ac9f22d18f3c81b7ec93d1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0,alloc::alloc::Global> @ 0x1000e4f -> 0x738bd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] alloc::boxed::impl$47::call_once<tuple$<ref_mut$<rhapsody::rg::pass_api::RenderPassApi>,ref_mut$<country_core::render::pass_builder::pass_resources::PassResources> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure_env$0,alloc::alloc::Global> @ 0x1002731 -> 0x738bd0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::update_descriptor_set
// 信息 ID: e716a90b0818a59f5077dfc7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::update_persistent_descriptor_set<core::iter::adapters::map::Map<alloc::vec::drain::Drain<country_core::asset_libraries::texture_library::AssetIndex,alloc::alloc::Global>,country_core::asset_libraries::texture_library::impl$1::update_bindless_texture_table::closure_env$0> >; module=1332, inline_record=2076; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$1>,core::iter::adapters::map::Map<core::slice::iter::Iter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$2> > >; module=1332, inline_record=19100; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure$0::closure_env$4> >; module=1332, inline_record=27568; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<enum2$<rhapsody::raw::binding::DescriptorSetBinding> > >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$6> >; module=1332, inline_record=33396; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::bind_descriptor_set<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<tuple$<shader_pipeline::reflection::BindingIndex,rhapsody::rg::binding::PassResourceSlot>,alloc::alloc::Global>,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$3>,core::iter::adapters::map::Map<std::collections::hash::map::Iter<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2$<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle> >,country_core::render::pass_builder::draw_stream::impl$11::render::closure$0::closure_env$4> > >; module=1332, inline_record=39628; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
