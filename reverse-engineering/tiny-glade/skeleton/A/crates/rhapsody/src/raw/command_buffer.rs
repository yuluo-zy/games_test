// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/raw/command_buffer.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: ff1b50a8a0da015c1f4f00a7e8aaf4395f71674ebead54efc86c9b7f9a45d463
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: command_buffer.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 68, 139, 549, 556, 568, 569, 570, 571, 1464, 1591, 1592, 1599, 1611, 1612, 1617, 1624, 1633
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"PDB_registry_path_only","version":"0.38.0+1.3.281"}]
// use core; // namespace candidate
//   package identities: []
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"PDB_registry_path_only","version":"0.4.9"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"PDB_registry_path_only","version":"0.4.21"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::command_buffer::impl$11::fmt
// 记录形式: compiled_procedure; instances=2
// 源码行: 14
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":139,"record_offset":3772,"rva":2529344,"va":5371238464}
// 编译实例 RVAs: 0x269840, 0x745870
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field2_finish
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6345385","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371238464,"raw_hex":"e8a9d26000","size":5,"soft_fail":false,"target_va":5377584208,"va":5371238818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x140876b30
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: command_buffer.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t6345405","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5371238464,"raw_hex":"e9bdd26000","size":5,"soft_fail":false,"target_va":5377583920,"va":5371238510,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::command_buffer::impl$17::eq
// 记录形式: inline_instance, inlinee_declaration; instances=13
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x111A4
// 首条位置: {"module":1592,"inlinee":70052}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::command_buffer::impl$21::fmt
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x204C
// 首条位置: {"module":68,"inlinee":8268}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::command_buffer::impl$22::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 31
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11245
// 首条位置: {"module":1611,"inlinee":70213}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::hash::impls::impl$12::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1611,"record_offset":612,"parent_record_offset":588,"ancestor_record_offset":588,"inlinee":5838,"source_line":806,"archive":"_pdb/A/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u64
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1611,"record_offset":636,"parent_record_offset":612,"ancestor_record_offset":588,"inlinee":8431,"source_line":383,"archive":"_pdb/A/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":1611,"record_offset":716,"parent_record_offset":692,"ancestor_record_offset":588,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1611,"record_offset":660,"parent_record_offset":636,"ancestor_record_offset":588,"inlinee":8432,"source_line":2859,"archive":"_pdb/A/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1611,"record_offset":692,"parent_record_offset":636,"ancestor_record_offset":588,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/01611.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::command_buffer::CommandBufferSeqTracker::track
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=23
// 源码行: 54
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x788C
// 首条位置: {"module":568,"inlinee":30860}
// 编译实例 RVAs: 0x7451e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26646567","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376332256,"raw_hex":"e827989601","size":5,"soft_fail":false,"target_va":5402978944,"va":5376332372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=13
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":568,"record_offset":1224,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00568.jsonl.gz"}
//   [pdb_inline_direct] core::sync::atomic::AtomicU64::fetch_max
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":568,"record_offset":1168,"parent_record_offset":1136,"ancestor_record_offset":1136,"inlinee":30861,"source_line":2932,"archive":"_pdb/A/modules/00568.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_umax
//     owner=rust_standard_library; count=16
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":568,"record_offset":1192,"parent_record_offset":1168,"ancestor_record_offset":1136,"inlinee":26109,"source_line":3538,"archive":"_pdb/A/modules/00568.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::command_buffer::CommandBufferSeqTracker::current
// 记录形式: inline_instance, inlinee_declaration; instances=9
// 源码行: 68
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x76EA
// 首条位置: {"module":549,"inlinee":30442}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::sync::atomic::AtomicU64::load
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":549,"record_offset":1624,"parent_record_offset":1600,"ancestor_record_offset":1600,"inlinee":19665,"source_line":2413,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":549,"record_offset":1648,"parent_record_offset":1624,"ancestor_record_offset":1600,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00549.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::command_buffer::CbGraphicsState::record_default
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 96
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x79A4
// 首条位置: {"module":571,"inlinee":31140}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::cmd_set_stencil_reference
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":571,"record_offset":2948,"parent_record_offset":2924,"ancestor_record_offset":2924,"inlinee":31128,"source_line":2048,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::command_buffer::impl$4::drop
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=14
// 源码行: 125
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7772
// 首条位置: {"module":556,"inlinee":30578}
// 编译实例 RVAs: 0x72e7f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26739506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376239600,"raw_hex":"e832039801","size":5,"soft_fail":false,"target_va":5402979152,"va":5376239641,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::command_buffer::CommandBuffer::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 135
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":571,"record_offset":720,"rva":7623264,"va":5376332384}
// 编译实例 RVAs: 0x745260
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: command_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3582860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376332384,"raw_hex":"e87454c9ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5376332551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: command_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3582929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376332384,"raw_hex":"e82f54c9ff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376332636,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072c7a0
//     owner=unresolved_namespace; count=1
//     target_aliases=9 candidates; complete list: command_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-101446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376332384,"raw_hex":"e8ba73feff","size":5,"soft_fail":false,"target_va":5376231328,"va":5376332769,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26645381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376332384,"raw_hex":"e885939601","size":5,"soft_fail":false,"target_va":5402978144,"va":5376332758,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26647311","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376332384,"raw_hex":"e80f9b9601","size":5,"soft_fail":false,"target_va":5402980128,"va":5376332812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t*656(%rdi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5376332384,"raw_hex":"ff9790020000","size":6,"soft_fail":false,"va":5376332469,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::allocate_command_buffers
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":571,"record_offset":1052,"parent_record_offset":720,"ancestor_record_offset":720,"inlinee":31134,"source_line":2541,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_command_pool
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":571,"record_offset":908,"parent_record_offset":720,"ancestor_record_offset":720,"inlinee":31131,"source_line":2556,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::CommandBufferAllocateInfo::level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":571,"record_offset":1024,"parent_record_offset":720,"ancestor_record_offset":720,"inlinee":31133,"source_line":5565,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::CommandPoolCreateInfo::queue_family_index
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":571,"record_offset":880,"parent_record_offset":720,"ancestor_record_offset":720,"inlinee":31130,"source_line":5522,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":1672,"parent_record_offset":720,"ancestor_record_offset":720,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":571,"record_offset":1840,"parent_record_offset":720,"ancestor_record_offset":720,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::ManagedResources::owning_device
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":571,"record_offset":1812,"parent_record_offset":720,"ancestor_record_offset":720,"inlinee":30450,"source_line":173,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":1240,"parent_record_offset":1216,"ancestor_record_offset":720,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":1264,"parent_record_offset":1240,"ancestor_record_offset":720,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":1624,"parent_record_offset":1600,"ancestor_record_offset":720,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":1216,"parent_record_offset":1188,"ancestor_record_offset":720,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":1600,"parent_record_offset":1576,"ancestor_record_offset":720,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":571,"record_offset":1188,"parent_record_offset":1160,"ancestor_record_offset":720,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":571,"record_offset":1160,"parent_record_offset":1132,"ancestor_record_offset":720,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":571,"record_offset":1336,"parent_record_offset":1160,"ancestor_record_offset":720,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":571,"record_offset":1576,"parent_record_offset":1552,"ancestor_record_offset":720,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":1104,"parent_record_offset":1052,"ancestor_record_offset":720,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":1132,"parent_record_offset":1104,"ancestor_record_offset":720,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":571,"record_offset":936,"parent_record_offset":908,"ancestor_record_offset":720,"inlinee":11125,"source_line":24,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":571,"record_offset":960,"parent_record_offset":936,"ancestor_record_offset":720,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":571,"record_offset":984,"parent_record_offset":960,"ancestor_record_offset":720,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::set_vec_len_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":571,"record_offset":1376,"parent_record_offset":1052,"ancestor_record_offset":720,"inlinee":31060,"source_line":29,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":1492,"parent_record_offset":1460,"ancestor_record_offset":720,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":1292,"parent_record_offset":1264,"ancestor_record_offset":720,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":571,"record_offset":1460,"parent_record_offset":1376,"ancestor_record_offset":720,"inlinee":4723,"source_line":744,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::command_buffer::CommandBuffer::raw
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 177
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78BA
// 首条位置: {"module":570,"inlinee":30906}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::command_buffer::CommandBuffer::begin
// 记录形式: compiled_procedure; instances=1
// 源码行: 182
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":571,"record_offset":1900,"rva":7623776,"va":5376332896}
// 编译实例 RVAs: 0x745460
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26645798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376332896,"raw_hex":"e826959601","size":5,"soft_fail":false,"target_va":5402979152,"va":5376333349,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26646102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376332896,"raw_hex":"e856969601","size":5,"soft_fail":false,"target_va":5402979344,"va":5376333237,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26646841","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376332896,"raw_hex":"e839999601","size":5,"soft_fail":false,"target_va":5402980128,"va":5376333282,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t26635895","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376332896,"raw_hex":"e8776e9601","size":5,"soft_fail":false,"target_va":5402968992,"va":5376333092,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t26637576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376332896,"raw_hex":"e808759601","size":5,"soft_fail":false,"target_va":5402970800,"va":5376333219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::disposal::CommandBufferDisposalBuckets::insert
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t10789844","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376332896,"raw_hex":"e8d4a3a400","size":5,"soft_fail":false,"target_va":5387123040,"va":5376333191,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*712(%r15)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5376332896,"raw_hex":"41ff97c8020000","size":7,"soft_fail":false,"va":5376332959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::begin_command_buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":571,"record_offset":2216,"parent_record_offset":1900,"ancestor_record_offset":1900,"inlinee":31138,"source_line":2413,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::reset_command_buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":571,"record_offset":2068,"parent_record_offset":1900,"ancestor_record_offset":1900,"inlinee":31135,"source_line":1621,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::CommandBufferBeginInfo::flags
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":571,"record_offset":2188,"parent_record_offset":1900,"ancestor_record_offset":1900,"inlinee":31137,"source_line":5694,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":2740,"parent_record_offset":1900,"ancestor_record_offset":1900,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":571,"record_offset":2152,"parent_record_offset":1900,"ancestor_record_offset":1900,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":571,"record_offset":2336,"parent_record_offset":1900,"ancestor_record_offset":1900,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::command_buffer::CbGraphicsState::record_default
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":571,"record_offset":2924,"parent_record_offset":1900,"ancestor_record_offset":1900,"inlinee":31140,"source_line":96,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::Queue::allocate_command_buffer_seq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":571,"record_offset":2656,"parent_record_offset":1900,"ancestor_record_offset":1900,"inlinee":31080,"source_line":133,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::submit::SubmissionEngine::queue
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/submit.rs
//     evidence={"module":571,"record_offset":2516,"parent_record_offset":1900,"ancestor_record_offset":1900,"inlinee":30506,"source_line":132,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":2556,"parent_record_offset":2516,"ancestor_record_offset":1900,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::cmd_set_stencil_reference
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":571,"record_offset":2948,"parent_record_offset":2924,"ancestor_record_offset":1900,"inlinee":31128,"source_line":2048,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":571,"record_offset":2092,"parent_record_offset":2068,"ancestor_record_offset":1900,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":571,"record_offset":2116,"parent_record_offset":2092,"ancestor_record_offset":1900,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":571,"record_offset":2452,"parent_record_offset":2364,"ancestor_record_offset":1900,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":571,"record_offset":2476,"parent_record_offset":2452,"ancestor_record_offset":1900,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":571,"record_offset":2584,"parent_record_offset":2556,"ancestor_record_offset":1900,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":571,"record_offset":2612,"parent_record_offset":2584,"ancestor_record_offset":1900,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU64::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":571,"record_offset":2680,"parent_record_offset":2656,"ancestor_record_offset":1900,"inlinee":15668,"source_line":2665,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":571,"record_offset":2828,"parent_record_offset":2796,"ancestor_record_offset":1900,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":571,"record_offset":2396,"parent_record_offset":2364,"ancestor_record_offset":1900,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":571,"record_offset":2704,"parent_record_offset":2680,"ancestor_record_offset":1900,"inlinee":4353,"source_line":3336,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":571,"record_offset":2852,"parent_record_offset":2828,"ancestor_record_offset":1900,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":571,"record_offset":2420,"parent_record_offset":2396,"ancestor_record_offset":1900,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":571,"record_offset":2768,"parent_record_offset":2740,"ancestor_record_offset":1900,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":571,"record_offset":2364,"parent_record_offset":2336,"ancestor_record_offset":1900,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":571,"record_offset":2796,"parent_record_offset":2768,"ancestor_record_offset":1900,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::command_buffer::CommandBuffer::end
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 221
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7914
// 首条位置: {"module":570,"inlinee":30996}
// 编译实例 RVAs: 0x745680
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26645381","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376333440,"raw_hex":"e885939601","size":5,"soft_fail":false,"target_va":5402978944,"va":5376333558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26646520","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376333440,"raw_hex":"e8f8979601","size":5,"soft_fail":false,"target_va":5402980128,"va":5376333603,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*704(%rdx)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5376333440,"raw_hex":"ff92c0020000","size":6,"soft_fail":false,"va":5376333457,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::end_command_buffer
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":570,"record_offset":48724,"parent_record_offset":48680,"ancestor_record_offset":48680,"inlinee":30997,"source_line":2423,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":570,"record_offset":48844,"parent_record_offset":48680,"ancestor_record_offset":48680,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":48808,"parent_record_offset":48680,"ancestor_record_offset":48680,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":570,"record_offset":48748,"parent_record_offset":48724,"ancestor_record_offset":48680,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":570,"record_offset":48772,"parent_record_offset":48748,"ancestor_record_offset":48680,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::command_buffer::CommandBuffer::set_stencil_reference
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 241
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x10666
// 首条位置: {"module":1464,"inlinee":67174}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::cmd_set_stencil_reference
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1464,"record_offset":4896,"parent_record_offset":4864,"ancestor_record_offset":4864,"inlinee":31128,"source_line":2048,"archive":"_pdb/A/modules/01464.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::command_buffer::CommandBufferPool::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 262
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":571,"record_offset":3460,"rva":7624496,"va":5376333616}
// 编译实例 RVAs: 0x745730
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: command_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3583989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376333616,"raw_hex":"e80b50c9ff","size":5,"soft_fail":false,"target_va":5372749696,"va":5376333680,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26644213","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376333616,"raw_hex":"e8f58e9601","size":5,"soft_fail":false,"target_va":5402978144,"va":5376333926,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1193202","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376333616,"raw_hex":"e8f2341200","size":5,"soft_fail":false,"target_va":5377527120,"va":5376333913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":3600,"parent_record_offset":3460,"ancestor_record_offset":3460,"inlinee":4249,"source_line":1999,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":3916,"parent_record_offset":3892,"ancestor_record_offset":3460,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":3940,"parent_record_offset":3916,"ancestor_record_offset":3460,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":571,"record_offset":3892,"parent_record_offset":3800,"ancestor_record_offset":3460,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":571,"record_offset":3800,"parent_record_offset":3772,"ancestor_record_offset":3460,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":571,"record_offset":3772,"parent_record_offset":3744,"ancestor_record_offset":3460,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":571,"record_offset":4012,"parent_record_offset":3772,"ancestor_record_offset":3460,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::extend_trusted
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":4080,"parent_record_offset":4056,"ancestor_record_offset":3460,"inlinee":5431,"source_line":3007,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":3716,"parent_record_offset":3684,"ancestor_record_offset":3460,"inlinee":6967,"source_line":480,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":3744,"parent_record_offset":3716,"ancestor_record_offset":3460,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$14::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":3628,"parent_record_offset":3600,"ancestor_record_offset":3460,"inlinee":4251,"source_line":2893,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$19::extend_trusted::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":571,"record_offset":4320,"parent_record_offset":4292,"ancestor_record_offset":3460,"inlinee":5435,"source_line":3020,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::set_len_on_drop::SetLenOnDrop::increment_len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\set_len_on_drop.rs
//     evidence={"module":571,"record_offset":4392,"parent_record_offset":4320,"ancestor_record_offset":3460,"inlinee":5440,"source_line":18,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_extend::impl$1::spec_extend
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_extend.rs
//     evidence={"module":571,"record_offset":4056,"parent_record_offset":3684,"ancestor_record_offset":3460,"inlinee":5430,"source_line":25,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter::impl$0::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter.rs
//     evidence={"module":571,"record_offset":3656,"parent_record_offset":3628,"ancestor_record_offset":3460,"inlinee":7317,"source_line":32,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::spec_from_iter_nested::impl$1::from_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\spec_from_iter_nested.rs
//     evidence={"module":571,"record_offset":3684,"parent_record_offset":3656,"ancestor_record_offset":3460,"inlinee":7319,"source_line":52,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":571,"record_offset":3836,"parent_record_offset":3800,"ancestor_record_offset":3460,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":571,"record_offset":3860,"parent_record_offset":3836,"ancestor_record_offset":3460,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":571,"record_offset":4128,"parent_record_offset":4104,"ancestor_record_offset":3460,"inlinee":4667,"source_line":125,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::map_fold::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":571,"record_offset":4264,"parent_record_offset":4152,"ancestor_record_offset":3460,"inlinee":4670,"source_line":89,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":571,"record_offset":4224,"parent_record_offset":4192,"ancestor_record_offset":3460,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":571,"record_offset":4192,"parent_record_offset":4152,"ancestor_record_offset":3460,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::fold
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":4152,"parent_record_offset":4128,"ancestor_record_offset":3460,"inlinee":5097,"source_line":2580,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":4104,"parent_record_offset":4080,"ancestor_record_offset":3460,"inlinee":5220,"source_line":807,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::for_each::call::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\iterator.rs
//     evidence={"module":571,"record_offset":4292,"parent_record_offset":4264,"ancestor_record_offset":3460,"inlinee":5224,"source_line":814,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":3968,"parent_record_offset":3940,"ancestor_record_offset":3460,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":571,"record_offset":4356,"parent_record_offset":4320,"ancestor_record_offset":3460,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00571.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::command_buffer::CommandBufferPool::recycle
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 268
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78D5
// 首条位置: {"module":570,"inlinee":30933}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":29228,"parent_record_offset":29056,"ancestor_record_offset":29056,"inlinee":4191,"source_line":1967,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":29096,"parent_record_offset":29056,"ancestor_record_offset":29056,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":29304,"parent_record_offset":29272,"ancestor_record_offset":29056,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":29272,"parent_record_offset":29228,"ancestor_record_offset":29056,"inlinee":4192,"source_line":1362,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":29336,"parent_record_offset":29228,"ancestor_record_offset":29056,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":29136,"parent_record_offset":29096,"ancestor_record_offset":29056,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":29176,"parent_record_offset":29136,"ancestor_record_offset":29056,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::create_command_buffer
// 记录形式: compiled_procedure; instances=1
// 源码行: 274
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":570,"record_offset":16760,"rva":7578592,"va":5376287712}
// 编译实例 RVAs: 0x73a3e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: command_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3538601","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376287712,"raw_hex":"e85701caff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376288308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072b3c0
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: command_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-62513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376287712,"raw_hex":"e8cf0bffff","size":5,"soft_fail":false,"target_va":5376226240,"va":5376288748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140869090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: command_buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1239787","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376287712,"raw_hex":"e8ebea1200","size":5,"soft_fail":false,"target_va":5377527952,"va":5376288160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1075375","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376287712,"raw_hex":"e8af681000","size":5,"soft_fail":false,"target_va":5377364336,"va":5376288956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26690893","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376287712,"raw_hex":"e84d459701","size":5,"soft_fail":false,"target_va":5402979344,"va":5376288446,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26691604","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376287712,"raw_hex":"e814489701","size":5,"soft_fail":false,"target_va":5402980128,"va":5376288519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] log::__private_api::log_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1065824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376287712,"raw_hex":"e860431000","size":5,"soft_fail":false,"target_va":5377353776,"va":5376287947,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t26680580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376287712,"raw_hex":"e8041d9701","size":5,"soft_fail":false,"target_va":5402968992,"va":5376288407,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t26682366","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376287712,"raw_hex":"e8fe239701","size":5,"soft_fail":false,"target_va":5402970800,"va":5376288429,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::command_buffer::CommandBuffer::new
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t44378","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376287712,"raw_hex":"e85aad0000","size":5,"soft_fail":false,"target_va":5376332384,"va":5376288001,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*2112(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5376287712,"raw_hex":"ff9040080000","size":6,"soft_fail":false,"va":5376288259,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::ffi::c_str::CString::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\ffi\c_str.rs
//     evidence={"module":570,"record_offset":17832,"parent_record_offset":16760,"ancestor_record_offset":16760,"inlinee":9051,"source_line":263,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::pop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":17232,"parent_record_offset":16760,"ancestor_record_offset":16760,"inlinee":11505,"source_line":2050,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$13::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":17124,"parent_record_offset":16760,"ancestor_record_offset":16760,"inlinee":4866,"source_line":2840,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::extensions_generated::ext::debug_utils::Device::set_debug_utils_object_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/extensions/ext/debug_utils.rs
//     evidence={"module":570,"record_offset":18136,"parent_record_offset":16760,"ancestor_record_offset":16760,"inlinee":11129,"source_line":11,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DebugUtilsObjectNameInfoEXT::object_handle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":570,"record_offset":18108,"parent_record_offset":16760,"ancestor_record_offset":16760,"inlinee":11127,"source_line":19604,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":17332,"parent_record_offset":16760,"ancestor_record_offset":16760,"inlinee":4138,"source_line":972,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":18256,"parent_record_offset":16760,"ancestor_record_offset":16760,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":18220,"parent_record_offset":16760,"ancestor_record_offset":16760,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":17860,"parent_record_offset":16760,"ancestor_record_offset":16760,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":570,"record_offset":16944,"parent_record_offset":16760,"ancestor_record_offset":16760,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::debug_utils
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":570,"record_offset":17768,"parent_record_offset":16760,"ancestor_record_offset":16760,"inlinee":30918,"source_line":604,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":18056,"parent_record_offset":18032,"ancestor_record_offset":16760,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":18032,"parent_record_offset":17976,"ancestor_record_offset":16760,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":570,"record_offset":18332,"parent_record_offset":18308,"ancestor_record_offset":16760,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::impl$2::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\ffi\c_str.rs
//     evidence={"module":570,"record_offset":18280,"parent_record_offset":18256,"ancestor_record_offset":16760,"inlinee":8650,"source_line":703,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":18004,"parent_record_offset":17976,"ancestor_record_offset":16760,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":17296,"parent_record_offset":17264,"ancestor_record_offset":16760,"inlinee":4194,"source_line":257,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":17976,"parent_record_offset":17948,"ancestor_record_offset":16760,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":17264,"parent_record_offset":17232,"ancestor_record_offset":16760,"inlinee":5998,"source_line":1302,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":17592,"parent_record_offset":17376,"ancestor_record_offset":16760,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":570,"record_offset":18160,"parent_record_offset":18136,"ancestor_record_offset":16760,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":570,"record_offset":18184,"parent_record_offset":18160,"ancestor_record_offset":16760,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":570,"record_offset":17508,"parent_record_offset":17376,"ancestor_record_offset":16760,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":570,"record_offset":17536,"parent_record_offset":17376,"ancestor_record_offset":16760,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":17796,"parent_record_offset":17768,"ancestor_record_offset":16760,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":17728,"parent_record_offset":17688,"ancestor_record_offset":16760,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":17060,"parent_record_offset":16972,"ancestor_record_offset":16760,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":17084,"parent_record_offset":17060,"ancestor_record_offset":16760,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":17688,"parent_record_offset":17376,"ancestor_record_offset":16760,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":17620,"parent_record_offset":17592,"ancestor_record_offset":16760,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":17156,"parent_record_offset":17124,"ancestor_record_offset":16760,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":17648,"parent_record_offset":17620,"ancestor_record_offset":16760,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":17188,"parent_record_offset":17156,"ancestor_record_offset":16760,"inlinee":4869,"source_line":268,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":18516,"parent_record_offset":18484,"ancestor_record_offset":16760,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":17004,"parent_record_offset":16972,"ancestor_record_offset":16760,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":17448,"parent_record_offset":17424,"ancestor_record_offset":16760,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":18540,"parent_record_offset":18516,"ancestor_record_offset":16760,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":17028,"parent_record_offset":17004,"ancestor_record_offset":16760,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":17472,"parent_record_offset":17448,"ancestor_record_offset":16760,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":570,"record_offset":18456,"parent_record_offset":18428,"ancestor_record_offset":16760,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":570,"record_offset":17564,"parent_record_offset":17376,"ancestor_record_offset":16760,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":570,"record_offset":17424,"parent_record_offset":17376,"ancestor_record_offset":16760,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":570,"record_offset":16972,"parent_record_offset":16944,"ancestor_record_offset":16760,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":570,"record_offset":18484,"parent_record_offset":18456,"ancestor_record_offset":16760,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::impl$7::create_command_buffer::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":570,"record_offset":17376,"parent_record_offset":17332,"ancestor_record_offset":16760,"inlinee":30920,"source_line":282,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::command_buffer::impl$7::create_command_buffer::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 282
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78C8
// 首条位置: {"module":570,"inlinee":30920}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$12::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":17592,"parent_record_offset":17376,"ancestor_record_offset":17376,"inlinee":5450,"source_line":2828,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":570,"record_offset":17508,"parent_record_offset":17376,"ancestor_record_offset":17376,"inlinee":4715,"source_line":1188,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_const
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":570,"record_offset":17536,"parent_record_offset":17376,"ancestor_record_offset":17376,"inlinee":4716,"source_line":329,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":17688,"parent_record_offset":17376,"ancestor_record_offset":17376,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/__private_api.rs
//     evidence={"module":570,"record_offset":17564,"parent_record_offset":17376,"ancestor_record_offset":17376,"inlinee":4718,"source_line":64,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.21","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/log-0.4.21/src/lib.rs
//     evidence={"module":570,"record_offset":17424,"parent_record_offset":17376,"ancestor_record_offset":17376,"inlinee":4713,"source_line":1269,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":17728,"parent_record_offset":17688,"ancestor_record_offset":17376,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":17620,"parent_record_offset":17592,"ancestor_record_offset":17376,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":17648,"parent_record_offset":17620,"ancestor_record_offset":17376,"inlinee":5453,"source_line":262,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":17448,"parent_record_offset":17424,"ancestor_record_offset":17376,"inlinee":4714,"source_line":2413,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":17472,"parent_record_offset":17448,"ancestor_record_offset":17376,"inlinee":4391,"source_line":3303,"archive":"_pdb/A/modules/00570.jsonl.gz"}
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
// 完整结构及原始类型记录: command_buffer.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::command_buffer::CbGraphicsState
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 0
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::command_buffer::CommandBuffer
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 49
// [PDB 泛型关系，非字段关系] {"type": "core::slice::iter::Iter<rhapsody::raw::command_buffer::CommandBuffer>", "constructor": "core::slice::iter::Iter", "arguments": ["rhapsody::raw::command_buffer::CommandBuffer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::command_buffer::CommandBuffer>,rhapsody::raw::submit::impl$2::queue_submit::closure$1::closure_env$0<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<rhapsody::raw::command_buffer::CommandBuffer>", "rhapsody::raw::submit::impl$2::queue_submit::closure$1::closure_env$0<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::command_buffer::CommandBuffer>,rhapsody::raw::submit::impl$2::queue_submit::closure$1::closure_env$0<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$1<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > >", "constructor": "core::iter::adapters::flatten::FlatMap", "arguments": ["core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>", "core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::command_buffer::CommandBuffer>,rhapsody::raw::submit::impl$2::queue_submit::closure$1::closure_env$0<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > >", "rhapsody::raw::submit::impl$2::queue_submit::closure_env$1<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<ash::vk::definitions::CommandBuffer,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::command_buffer::CommandBuffer>,rhapsody::raw::submit::impl$2::queue_submit::closure$1::closure_env$0<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$1<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["ash::vk::definitions::CommandBuffer", "core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<rhapsody::raw::submit::SubmitInfo>,core::iter::adapters::map::Map<core::slice::iter::Iter<rhapsody::raw::command_buffer::CommandBuffer>,rhapsody::raw::submit::impl$2::queue_submit::closure$1::closure_env$0<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > >,rhapsody::raw::submit::impl$2::queue_submit::closure_env$1<alloc::vec::Vec<rhapsody::raw::submit::SubmitInfo,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>", "constructor": "rhapsody::raw::device::TimelineReclaim", "arguments": ["rhapsody::raw::command_buffer::CommandBuffer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::VecDeque::grow<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>,alloc::alloc::Global>", "constructor": "alloc::collections::vec_deque::VecDeque::grow", "arguments": ["rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::into_iter::IntoIter<rhapsody::raw::command_buffer::CommandBuffer,alloc::alloc::Global>", "constructor": "alloc::vec::into_iter::IntoIter", "arguments": ["rhapsody::raw::command_buffer::CommandBuffer", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<rhapsody::raw::command_buffer::CommandBuffer,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::into_iter::IntoIter<rhapsody::raw::command_buffer::CommandBuffer,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::VecDeque<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>,alloc::alloc::Global>", "constructor": "alloc::collections::vec_deque::VecDeque", "arguments": ["rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::collections::vec_deque::VecDeque<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::collections::vec_deque::VecDeque<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::option::Option<rhapsody::raw::command_buffer::CommandBuffer>", "constructor": "core::option::Option", "arguments": ["rhapsody::raw::command_buffer::CommandBuffer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::option::Option<rhapsody::raw::command_buffer::CommandBuffer> >", "constructor": "enum2$", "arguments": ["core::option::Option<rhapsody::raw::command_buffer::CommandBuffer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,enum2$<core::option::Option<rhapsody::raw::command_buffer::CommandBuffer> > >", "constructor": "lock_api::mutex::Mutex", "arguments": ["parking_lot::raw_mutex::RawMutex", "enum2$<core::option::Option<rhapsody::raw::command_buffer::CommandBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,enum2$<core::option::Option<rhapsody::raw::command_buffer::CommandBuffer> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,enum2$<core::option::Option<rhapsody::raw::command_buffer::CommandBuffer> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::collections::vec_deque::impl$1::drop::Dropper<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer> >", "constructor": "alloc::collections::vec_deque::impl$1::drop::Dropper", "arguments": ["rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::collections::vec_deque::impl$1::drop::Dropper<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::collections::vec_deque::impl$1::drop::Dropper<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::command_buffer::CommandBuffer>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::command_buffer::CommandBuffer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<rhapsody::raw::command_buffer::CommandBuffer,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["rhapsody::raw::command_buffer::CommandBuffer", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::raw::command_buffer::CommandBuffer,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<rhapsody::raw::command_buffer::CommandBuffer,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "slice2$<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer> >", "constructor": "slice2$", "arguments": ["rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<slice2$<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["slice2$<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<rhapsody::raw::command_buffer::CommandBuffer,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["rhapsody::raw::command_buffer::CommandBuffer", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::command_buffer::CommandBufferPool
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::command_buffer::CommandBufferPool>", "constructor": "lock_api::mutex::Mutex", "arguments": ["parking_lot::raw_mutex::RawMutex", "rhapsody::raw::command_buffer::CommandBufferPool"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::command_buffer::CommandBufferPool> >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::command_buffer::CommandBufferPool>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::command_buffer::CommandBufferResources
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 8
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::command_buffer::CommandBufferResources>", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::command_buffer::CommandBufferResources"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::command_buffer::CommandBufferSeq
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 2; 过程引用数: 13
// [PDB 泛型关系，非字段关系] {"type": "core::fmt::impl$51::fmt<rhapsody::raw::command_buffer::CommandBufferSeq>", "constructor": "core::fmt::impl$51::fmt", "arguments": ["rhapsody::raw::command_buffer::CommandBufferSeq"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> >", "constructor": "tuple$", "arguments": ["rhapsody::raw::command_buffer::CommandBufferSeq", "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::equivalent_key::closure_env$0<rhapsody::raw::command_buffer::CommandBufferSeq,rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> >", "constructor": "hashbrown::map::equivalent_key::closure_env$0", "arguments": ["rhapsody::raw::command_buffer::CommandBufferSeq", "rhapsody::raw::command_buffer::CommandBufferSeq", "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::remove_entry<tuple$<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> >,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<rhapsody::raw::command_buffer::CommandBufferSeq,rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> > >", "constructor": "hashbrown::raw::RawTable::remove_entry", "arguments": ["tuple$<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> >", "alloc::alloc::Global", "hashbrown::map::equivalent_key::closure_env$0<rhapsody::raw::command_buffer::CommandBufferSeq,rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["rhapsody::raw::command_buffer::CommandBufferSeq", "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<rhapsody::raw::command_buffer::CommandBufferSeq>", "constructor": "ref$", "arguments": ["rhapsody::raw::command_buffer::CommandBufferSeq"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<rhapsody::raw::command_buffer::CommandBufferSeq> >", "constructor": "core::hash::BuildHasher::hash_one", "arguments": ["std::hash::random::RandomState", "ref$<rhapsody::raw::command_buffer::CommandBufferSeq>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["rhapsody::raw::command_buffer::CommandBufferSeq", "lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::command_buffer::CommandBufferSeqTracker
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 1
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: command_buffer.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: command_buffer.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 0; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz


// ------------------------------------------------------------------
// 函数: rhapsody::raw::command_buffer::CbGraphicsState::record_default
// 信息 ID: 3e98cc4aedeaaf7fb22c194c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::command_buffer::CommandBuffer::begin; module=571, inline_record=2924; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::command_buffer::CommandBuffer::begin
// 信息 ID: fe2f12e13705c1a16c485022
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::execute<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2df000 -> 0x745460 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$3::create_buffer::closure_env$0> @ 0x118c4d8 -> 0x745460 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::texture::impl$9::update_texture_impl::closure_env$2> @ 0x118ca8a -> 0x745460 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$3::update_buffer_bytes::closure_env$0> @ 0x118cd7a -> 0x745460 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::command_buffer::CommandBuffer::end
// 信息 ID: a0323cc7f4462fc308e93f96
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::execute<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2dfd0a -> 0x745680 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::get_setup_cb_submit_info; module=570, inline_record=48680; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::command_buffer::CommandBuffer::new
// 信息 ID: d7456b4ff7e4d3aee7a82197
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_command_buffer @ 0x73a501 -> 0x745260 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::command_buffer::CommandBuffer::raw
// 信息 ID: 70c8d2b442e7347419ada914
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::bind_persistent_descriptor_set; module=570, inline_record=7492; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::record_crash_marker; module=570, inline_record=19468; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::begin_label; module=570, inline_record=24540; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::end_label; module=570, inline_record=25140; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::command_buffer::CommandBuffer::set_stencil_reference
// 信息 ID: 916362ddea8d8f8a366494dc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::pass_api::RenderPassApi::set_stencil_ref; module=1464, inline_record=4864; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::command_buffer::CommandBufferPool::new
// 信息 ID: b0bee1c294a6d613a7646439
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0x73f042 -> 0x745730 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::command_buffer::CommandBufferPool::recycle
// 信息 ID: c489b66489c56ce8dd50494c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::Queue::reclaim_command_buffer_resources; module=570, inline_record=29056; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::command_buffer::CommandBufferSeqTracker::current
// 信息 ID: 462df192994e75f242a083e3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste; module=549, inline_record=1600; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste; module=549, inline_record=1812; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste; module=549, inline_record=1960; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste; module=549, inline_record=2108; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste; module=549, inline_record=2256; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste; module=549, inline_record=2404; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::ManagedResources::take_out_dropped_waste; module=549, inline_record=2552; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::command_buffer::CommandBufferSeqTracker::track
// 信息 ID: 5167fc8978a65f4e9007880e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 15 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources> @ 0x74832 -> 0x7451e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources> @ 0x748b4 -> 0x7451e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources> @ 0x7515a -> 0x7451e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::compiled::CompiledRenderGraph::record_batch<country_core::render::pass_builder::pass_resources::PassResources> @ 0x751d1 -> 0x7451e0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::raw::managed::handle::impl$4::track; module=568, inline_record=1136; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::swapchain::Swapchain::acquire_next_texture; module=569, inline_record=4448; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::record_crash_marker; module=570, inline_record=19640; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::pass_api::RenderPassApi::begin_render_pass; module=1464, inline_record=3764; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::AccessManagedResource::get_maybe_tracked<rhapsody::raw::managed::resources::ManagedResources,rhapsody::raw::managed::buffer::ManagedBuffer>; module=1591, inline_record=2820; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$4::get_tracked<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet>; module=1591, inline_record=3376; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$4::get_tracked<rhapsody::raw::managed::buffer::ManagedBuffer>; module=1591, inline_record=3776; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::managed::resources::impl$4::get_tracked<rhapsody::raw::managed::render_pass::ManagedRenderPass>; module=1591, inline_record=4184; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::command_buffer::impl$11::fmt
// 信息 ID: b01f689f3d29af28b489b548
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::command_buffer::impl$17::eq
// 信息 ID: 839eb1d0e9212c81cd740671
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 9 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::framebuffer::ManagedFramebuffer> >; module=1592, inline_record=1608; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::descriptor_set::ManagedPersistentDescriptorSet> >; module=1592, inline_record=4248; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::sampler::ManagedSampler> >; module=1592, inline_record=6864; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::render_pass::ManagedRenderPass> >; module=1592, inline_record=9488; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::buffer::ManagedBuffer> >; module=1592, inline_record=12100; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::texture::ManagedTexture> >; module=1592, inline_record=14716; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::disposal::CommandBufferDisposalBuckets::dispose_cb_resource<rhapsody::raw::managed::resources::impl$3::throw_out_managed_resource::closure_env$0<rhapsody::raw::managed::swapchain::ManagedSwapchain> >; module=1592, inline_record=17336; 不是独立调用边。
// [PDB 内联宿主] hashbrown::raw::RawTable::remove_entry<tuple$<rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> >,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<rhapsody::raw::command_buffer::CommandBufferSeq,rhapsody::raw::command_buffer::CommandBufferSeq,lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,rhapsody::raw::disposal::DisposalBucket> > >; module=1599, inline_record=6832; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::command_buffer::impl$21::fmt
// 信息 ID: 0bf49ff1c64320200d0f1152
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::fmt::impl$51::fmt<rhapsody::raw::command_buffer::CommandBufferSeq>; module=68, inline_record=10028; 不是独立调用边。
// [PDB 内联宿主] core::fmt::impl$51::fmt<rhapsody::raw::command_buffer::CommandBufferSeq>; module=1633, inline_record=404; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::command_buffer::impl$22::hash
// 信息 ID: 59727c375699b02e316f9fc8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<rhapsody::raw::command_buffer::CommandBufferSeq> >; module=1611, inline_record=588; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::command_buffer::impl$4::drop
// 信息 ID: efa4c9eda683cdc2e10722b3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 14 条；内联宿主 10 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<rhapsody::raw::command_buffer::CommandBuffer,alloc::alloc::Global> > @ 0x35dfa9 -> 0x72e7f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer> > | core::ptr::drop_in_place<rhapsody::raw::command_buffer::CommandBuffer> | core::ptr::drop_in_place<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer> > | core::ptr::drop_in_place<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer> > | 其余 4 个同地址别名见共享索引 @ 0x360127 -> 0x72e7f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<alloc::collections::vec_deque::VecDeque<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>,alloc::alloc::Global> > | core::ptr::drop_in_place<alloc::collections::vec_deque::VecDeque<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>,alloc::alloc::Global> > | core::ptr::drop_in_place<alloc::collections::vec_deque::VecDeque<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>,alloc::alloc::Global> > | core::ptr::drop_in_place<alloc::collections::vec_deque::VecDeque<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>,alloc::alloc::Global> > | 其余 3 个同地址别名见共享索引 @ 0x36804d -> 0x72e7f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<alloc::collections::vec_deque::VecDeque<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>,alloc::alloc::Global> > | core::ptr::drop_in_place<alloc::collections::vec_deque::VecDeque<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>,alloc::alloc::Global> > | core::ptr::drop_in_place<alloc::collections::vec_deque::VecDeque<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>,alloc::alloc::Global> > | core::ptr::drop_in_place<alloc::collections::vec_deque::VecDeque<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer>,alloc::alloc::Global> > | 其余 3 个同地址别名见共享索引 @ 0x3680ad -> 0x72e7f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,enum2$<core::option::Option<rhapsody::raw::command_buffer::CommandBuffer> > > > | core::ptr::drop_in_place<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,enum2$<core::option::Option<rhapsody::raw::command_buffer::CommandBuffer> > > > | core::ptr::drop_in_place<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,enum2$<core::option::Option<rhapsody::raw::command_buffer::CommandBuffer> > > > | core::ptr::drop_in_place<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,enum2$<core::option::Option<rhapsody::raw::command_buffer::CommandBuffer> > > > | 其余 3 个同地址别名见共享索引 @ 0x3683f8 -> 0x72e7f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<alloc::collections::vec_deque::impl$1::drop::Dropper<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer> > > | core::ptr::drop_in_place<alloc::collections::vec_deque::impl$1::drop::Dropper<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer> > > | core::ptr::drop_in_place<alloc::collections::vec_deque::impl$1::drop::Dropper<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer> > > | core::ptr::drop_in_place<alloc::collections::vec_deque::impl$1::drop::Dropper<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer> > > | 其余 3 个同地址别名见共享索引 @ 0x36d094 -> 0x72e7f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::device::GraphicsDevice> @ 0x37c74c -> 0x72e7f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::raw::command_buffer::CommandBuffer,alloc::alloc::Global> > | core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::raw::command_buffer::CommandBuffer,alloc::alloc::Global> > | core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::raw::command_buffer::CommandBuffer,alloc::alloc::Global> > | core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::raw::command_buffer::CommandBuffer,alloc::alloc::Global> > | 其余 3 个同地址别名见共享索引 @ 0x390059 -> 0x72e7f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::device::GraphicsDevice> @ 0x6e500c -> 0x72e7f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::device::GraphicsDevice> @ 0x8cd9fc -> 0x72e7f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::device::GraphicsDevice> @ 0x9a5adc -> 0x72e7f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::device::GraphicsDevice> @ 0xa62b1c -> 0x72e7f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::device::GraphicsDevice> @ 0xd0296c -> 0x72e7f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ptr::drop_in_place<rhapsody::raw::device::GraphicsDevice> @ 0x155e31c -> 0x72e7f0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] core::ptr::drop_in_place<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer> >; module=556, inline_record=3720; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<slice2$<rhapsody::raw::device::TimelineReclaim<rhapsody::raw::command_buffer::CommandBuffer> > >; module=556, inline_record=4484; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,enum2$<core::option::Option<rhapsody::raw::command_buffer::CommandBuffer> > > >; module=556, inline_record=18408; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<rhapsody::raw::command_buffer::CommandBuffer>; module=556, inline_record=45212; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::raw::command_buffer::CommandBuffer,alloc::alloc::Global> >; module=556, inline_record=72248; 不是独立调用边。
// [PDB 内联宿主] core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::raw::command_buffer::CommandBuffer,alloc::alloc::Global> >; module=556, inline_record=72568; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::get_setup_cb_submit_info; module=570, inline_record=48396; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$3::create_buffer::closure_env$0>; module=1624, inline_record=5700; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::command_buffer::impl$7::create_command_buffer::closure$0
// 信息 ID: 3f376f41834f00561e741bd8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::command_buffer::impl$7::create_command_buffer", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_command_buffer; module=570, inline_record=17376; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_command_buffer
// 信息 ID: 8e8057413ec62dec6acad4f2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::execute<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2def6e -> 0x73a3e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::execute<country_core::render::pass_builder::pass_resources::PassResources> @ 0x2ded53 -> 0x73a3e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$3::create_buffer::closure_env$0> @ 0x118c4bb -> 0x73a3e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::texture::impl$9::update_texture_impl::closure_env$2> @ 0x118ca76 -> 0x73a3e0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$3::update_buffer_bytes::closure_env$0> @ 0x118cd66 -> 0x73a3e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
