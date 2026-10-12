// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: A
// Original source: crates/rhapsody/src/raw/buffer.rs
// PDB SHA-256: d5d4001583e31abf20b1563bbfdd9e4853629065aa2e461fe5809b205db908a5
// Source SHA-256: 6cc90c9ff7cf90947d6a53ac6335ba00307b99f6d4adbee38b1bc373da60b21d, d9129672af303a329a412665071e76ac7e392bfcaa307e91159781e586547b8d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: buffer.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/A/
// 编译单元: 134, 387, 400, 449, 451, 464, 485, 490, 557, 565, 567, 570, 575, 1611, 1612, 1617, 1624
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
// use gpu_allocator; // namespace candidate
//   package identities: []
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"PDB_registry_path_only","version":"0.4.9"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"PDB_registry_path_only","version":"0.12.1"}]
// use rhapsody; // namespace candidate
//   package identities: []
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::Buffer::device_address
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 38
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7793
// 首条位置: {"module":557,"record_offset":612,"rva":7531184,"va":5376240304}
// 编译实例 RVAs: 0x72eab0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26738371","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376240304,"raw_hex":"e8c3fe9701","size":5,"soft_fail":false,"target_va":5402978880,"va":5376240504,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26986027","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376240304,"raw_hex":"e82bc69b01","size":5,"soft_fail":false,"target_va":5403226480,"va":5376240448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*1280(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5376240304,"raw_hex":"ff9000050000","size":6,"soft_fail":false,"va":5376240397,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::get_buffer_device_address
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":557,"record_offset":944,"parent_record_offset":612,"ancestor_record_offset":612,"inlinee":30596,"source_line":710,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::BufferDeviceAddressInfo::buffer
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":557,"record_offset":916,"parent_record_offset":612,"ancestor_record_offset":612,"inlinee":30595,"source_line":27762,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":557,"record_offset":880,"parent_record_offset":612,"ancestor_record_offset":612,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":557,"record_offset":756,"parent_record_offset":612,"ancestor_record_offset":612,"inlinee":7658,"source_line":314,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":557,"record_offset":796,"parent_record_offset":756,"ancestor_record_offset":612,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":557,"record_offset":848,"parent_record_offset":756,"ancestor_record_offset":612,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":557,"record_offset":824,"parent_record_offset":756,"ancestor_record_offset":612,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::Buffer::mapped_slice_mut
// 记录形式: compiled_procedure; instances=1
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":557,"record_offset":976,"rva":7531392,"va":5376240512}
// 编译实例 RVAs: 0x72eb80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140733c20
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t20549","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376240512,"raw_hex":"e845500000","size":5,"soft_fail":false,"target_va":5376261152,"va":5376240598,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26738165","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376240512,"raw_hex":"e8f5fd9701","size":5,"soft_fail":false,"target_va":5402978912,"va":5376240742,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26738121","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376240512,"raw_hex":"e8c9fd9701","size":5,"soft_fail":false,"target_va":5402978880,"va":5376240754,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26985804","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376240512,"raw_hex":"e84cc59b01","size":5,"soft_fail":false,"target_va":5403226480,"va":5376240671,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26739527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376240512,"raw_hex":"e847039801","size":5,"soft_fail":false,"target_va":5402980304,"va":5376240772,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_allocator::vulkan::Allocation::mapped_ptr
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t150229","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376240512,"raw_hex":"e8d54a0200","size":5,"soft_fail":false,"target_va":5376390816,"va":5376240582,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":557,"record_offset":1244,"parent_record_offset":976,"ancestor_record_offset":976,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":557,"record_offset":1468,"parent_record_offset":976,"ancestor_record_offset":976,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::buffer::ManagedBuffer::mapped_slice_mut
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/buffer.rs
//     evidence={"module":557,"record_offset":1280,"parent_record_offset":976,"ancestor_record_offset":976,"inlinee":30598,"source_line":46,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":557,"record_offset":1120,"parent_record_offset":976,"ancestor_record_offset":976,"inlinee":7658,"source_line":314,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":557,"record_offset":1340,"parent_record_offset":1312,"ancestor_record_offset":976,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":557,"record_offset":1408,"parent_record_offset":1280,"ancestor_record_offset":976,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":557,"record_offset":1368,"parent_record_offset":1312,"ancestor_record_offset":976,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":557,"record_offset":1496,"parent_record_offset":1468,"ancestor_record_offset":976,"inlinee":5972,"source_line":424,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::buffer::ManagedBuffer::allocation
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/buffer.rs
//     evidence={"module":557,"record_offset":1312,"parent_record_offset":1280,"ancestor_record_offset":976,"inlinee":30599,"source_line":38,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::buffer::impl$1::mapped_slice_mut::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/buffer.rs
//     evidence={"module":557,"record_offset":1432,"parent_record_offset":1408,"ancestor_record_offset":976,"inlinee":30601,"source_line":48,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":557,"record_offset":1160,"parent_record_offset":1120,"ancestor_record_offset":976,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":557,"record_offset":1212,"parent_record_offset":1120,"ancestor_record_offset":976,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":557,"record_offset":1188,"parent_record_offset":1120,"ancestor_record_offset":976,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::impl$10::eq
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7A10
// 首条位置: {"module":575,"inlinee":31248}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::bitflags::impl$278::eq
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":575,"record_offset":2764,"parent_record_offset":2732,"ancestor_record_offset":2732,"inlinee":31250,"source_line":114,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   [pdb_inline_direct] gpu_allocator::impl$7::eq
//     owner=confirmed_vendored_dependency; count=3
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/gpu-allocator/src/lib.rs
//     evidence={"module":575,"record_offset":2792,"parent_record_offset":2732,"ancestor_record_offset":2732,"inlinee":31252,"source_line":229,"archive":"_pdb/A/modules/00575.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::impl$4::clone
// 记录形式: inline_instance, inlinee_declaration; instances=34
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x59DF
// 首条位置: {"module":400,"inlinee":23007}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::impl$6::fmt
// 记录形式: compiled_procedure; instances=2
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":134,"record_offset":15948,"rva":2508560,"va":5371217680}
// 编译实例 RVAs: 0x264710, 0x5058d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field4_finish
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t6366873","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371217680,"raw_hex":"e899266100","size":5,"soft_fail":false,"target_va":5377584768,"va":5371217890,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::impl$7::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 63
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1124F
// 首条位置: {"module":1611,"inlinee":70223}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::bitflags::impl$282::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":1611,"record_offset":15312,"parent_record_offset":15132,"ancestor_record_offset":15132,"inlinee":70225,"source_line":114,"archive":"_pdb/A/modules/01611.jsonl.gz"}
//   [pdb_inline_direct] core::hash::impls::impl$10::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1611,"record_offset":15684,"parent_record_offset":15132,"ancestor_record_offset":15132,"inlinee":15030,"source_line":806,"archive":"_pdb/A/modules/01611.jsonl.gz"}
//   [pdb_inline_direct] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1611,"record_offset":15168,"parent_record_offset":15132,"ancestor_record_offset":15132,"inlinee":6472,"source_line":806,"archive":"_pdb/A/modules/01611.jsonl.gz"}
//   [pdb_inline_direct] gpu_allocator::impl$9::hash
//     owner=confirmed_vendored_dependency; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/gpu-allocator/src/lib.rs
//     evidence={"module":1611,"record_offset":15484,"parent_record_offset":15132,"ancestor_record_offset":15132,"inlinee":70227,"source_line":229,"archive":"_pdb/A/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1611,"record_offset":15532,"parent_record_offset":15508,"ancestor_record_offset":15132,"inlinee":8460,"source_line":432,"archive":"_pdb/A/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u16
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1611,"record_offset":15708,"parent_record_offset":15684,"ancestor_record_offset":15132,"inlinee":15568,"source_line":371,"archive":"_pdb/A/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1611,"record_offset":15360,"parent_record_offset":15336,"ancestor_record_offset":15132,"inlinee":8446,"source_line":377,"archive":"_pdb/A/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1611,"record_offset":15192,"parent_record_offset":15168,"ancestor_record_offset":15132,"inlinee":8461,"source_line":395,"archive":"_pdb/A/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1611,"record_offset":15336,"parent_record_offset":15312,"ancestor_record_offset":15132,"inlinee":8445,"source_line":806,"archive":"_pdb/A/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\mod.rs
//     evidence={"module":1611,"record_offset":15508,"parent_record_offset":15484,"ancestor_record_offset":15132,"inlinee":8459,"source_line":806,"archive":"_pdb/A/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\hash\sip.rs
//     evidence={"module":1611,"record_offset":15272,"parent_record_offset":15248,"ancestor_record_offset":15132,"inlinee":8436,"source_line":236,"archive":"_pdb/A/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1611,"record_offset":15216,"parent_record_offset":15192,"ancestor_record_offset":15132,"inlinee":8462,"source_line":2859,"archive":"_pdb/A/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1611,"record_offset":15732,"parent_record_offset":15708,"ancestor_record_offset":15132,"inlinee":15569,"source_line":2859,"archive":"_pdb/A/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\num\uint_macros.rs
//     evidence={"module":1611,"record_offset":15384,"parent_record_offset":15360,"ancestor_record_offset":15132,"inlinee":8447,"source_line":2859,"archive":"_pdb/A/modules/01611.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\std\src\hash\random.rs
//     evidence={"module":1611,"record_offset":15248,"parent_record_offset":15192,"ancestor_record_offset":15132,"inlinee":8434,"source_line":132,"archive":"_pdb/A/modules/01611.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::BufferDesc::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 72
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":557,"record_offset":1540,"rva":7531664,"va":5376240784}
// 编译实例 RVAs: 0x72ec90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26738310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376240784,"raw_hex":"e886fe9701","size":5,"soft_fail":false,"target_va":5402979152,"va":5376240837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::BufferDesc::new_gpu_only
// 记录形式: compiled_procedure, inlinee_declaration; instances=2
// 源码行: 83
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7797
// 首条位置: {"module":557,"record_offset":1656,"rva":7531728,"va":5376240848}
// 编译实例 RVAs: 0x72ecd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26738246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376240848,"raw_hex":"e846fe9701","size":5,"soft_fail":false,"target_va":5402979152,"va":5376240901,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu
// 记录形式: compiled_procedure; instances=1
// 源码行: 94
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":557,"record_offset":1780,"rva":7531792,"va":5376240912}
// 编译实例 RVAs: 0x72ed10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26738182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376240912,"raw_hex":"e806fe9701","size":5,"soft_fail":false,"target_va":5402979152,"va":5376240965,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu_low_prio
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 105
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7799
// 首条位置: {"module":570,"inlinee":30617}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::BufferDesc::new_gpu_to_cpu
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 116
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x779A
// 首条位置: {"module":557,"record_offset":1908,"rva":7531856,"va":5376240976}
// 编译实例 RVAs: 0x72ed50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26738118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376240976,"raw_hex":"e8c6fd9701","size":5,"soft_fail":false,"target_va":5402979152,"va":5376241029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::BufferDesc::alignment
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 127
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x779B
// 首条位置: {"module":557,"record_offset":2036,"rva":7531920,"va":5376241040}
// 编译实例 RVAs: 0x72ed90
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26738993","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376241040,"raw_hex":"e831019801","size":5,"soft_fail":false,"target_va":5402980128,"va":5376241130,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$6::try_into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\mod.rs
//     evidence={"module":557,"record_offset":2168,"parent_record_offset":2036,"ancestor_record_offset":2036,"inlinee":4103,"source_line":797,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":557,"record_offset":2216,"parent_record_offset":2036,"ancestor_record_offset":2036,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::ptr_try_from_impls::impl$1::try_from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\convert\num.rs
//     evidence={"module":557,"record_offset":2188,"parent_record_offset":2168,"ancestor_record_offset":2036,"inlinee":30603,"source_line":248,"archive":"_pdb/A/modules/00557.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::create_buffer_impl
// 记录形式: compiled_procedure; instances=1
// 源码行: 134
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":570,"record_offset":7560,"rva":7572640,"va":5376281760}
// 编译实例 RVAs: 0x738ca0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da780
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3532703","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376281760,"raw_hex":"e86118caff","size":5,"soft_fail":false,"target_va":5372749696,"va":5376282394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3532394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376281760,"raw_hex":"e89619caff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376282101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14043f210
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3120669","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376281760,"raw_hex":"e8e361d0ff","size":5,"soft_fail":false,"target_va":5373162000,"va":5376282664,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14072c660
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-52767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376281760,"raw_hex":"e8e131ffff","size":5,"soft_fail":false,"target_va":5376231008,"va":5376283770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140869090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1245988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376281760,"raw_hex":"e824031300","size":5,"soft_fail":false,"target_va":5377527952,"va":5376281959,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141177310
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10740243","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376281760,"raw_hex":"e813e2a300","size":5,"soft_fail":false,"target_va":5387023120,"va":5376282872,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27310869","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376281760,"raw_hex":"e815bba001","size":5,"soft_fail":false,"target_va":5403593360,"va":5376282486,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::alloc::handle_alloc_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26694808","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376281760,"raw_hex":"e898549701","size":5,"soft_fail":false,"target_va":5402978144,"va":5376283331,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::capacity_overflow
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t1243803","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376281760,"raw_hex":"e89bfa1200","size":5,"soft_fail":false,"target_va":5377527120,"va":5376283312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t26697016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376281760,"raw_hex":"e8385d9701","size":5,"soft_fail":false,"target_va":5402980128,"va":5376283107,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26697109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376281760,"raw_hex":"e8955d9701","size":5,"soft_fail":false,"target_va":5402980304,"va":5376283190,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_allocator::vulkan::Allocation::mapped_slice_mut
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t108098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376281760,"raw_hex":"e842a60100","size":5,"soft_fail":false,"target_va":5376390832,"va":5376282729,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_allocator::vulkan::Allocation::memory
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2852394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376281760,"raw_hex":"e8d679d4ff","size":5,"soft_fail":false,"target_va":5373430256,"va":5376282645,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t26685943","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376281760,"raw_hex":"e8f7319701","size":5,"soft_fail":false,"target_va":5402968992,"va":5376283044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t26688166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376281760,"raw_hex":"e8a63a9701","size":5,"soft_fail":false,"target_va":5402970800,"va":5376282629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::allocator::GpuAllocator::allocate
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-58109","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376281760,"raw_hex":"e8031dffff","size":5,"soft_fail":false,"target_va":5376224192,"va":5376282296,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*264(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5376281760,"raw_hex":"ff9008010000","size":6,"soft_fail":false,"va":5376281908,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::ffi::c_str::CString::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\ffi\c_str.rs
//     evidence={"module":570,"record_offset":7884,"parent_record_offset":7560,"ancestor_record_offset":7560,"inlinee":9051,"source_line":263,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::bind_buffer_memory
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":570,"record_offset":9864,"parent_record_offset":7560,"ancestor_record_offset":7560,"inlinee":30914,"source_line":2702,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":570,"record_offset":7768,"parent_record_offset":7560,"ancestor_record_offset":7560,"inlinee":30908,"source_line":2202,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::get_buffer_memory_requirements
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":570,"record_offset":8480,"parent_record_offset":7560,"ancestor_record_offset":7560,"inlinee":30909,"source_line":2636,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::extensions_generated::ext::debug_utils::Device::set_debug_utils_object_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/extensions/ext/debug_utils.rs
//     evidence={"module":570,"record_offset":8188,"parent_record_offset":7560,"ancestor_record_offset":7560,"inlinee":11129,"source_line":11,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::BufferUsageFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":570,"record_offset":8540,"parent_record_offset":7560,"ancestor_record_offset":7560,"inlinee":30911,"source_line":32,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DebugUtilsObjectNameInfoEXT::object_handle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":570,"record_offset":8160,"parent_record_offset":7560,"ancestor_record_offset":7560,"inlinee":11127,"source_line":19604,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":9988,"parent_record_offset":7560,"ancestor_record_offset":7560,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":8308,"parent_record_offset":7560,"ancestor_record_offset":7560,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":8272,"parent_record_offset":7560,"ancestor_record_offset":7560,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":8804,"parent_record_offset":7560,"ancestor_record_offset":7560,"inlinee":4728,"source_line":826,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":7912,"parent_record_offset":7560,"ancestor_record_offset":7560,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":9660,"parent_record_offset":7560,"ancestor_record_offset":7560,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":10204,"parent_record_offset":7560,"ancestor_record_offset":7560,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":570,"record_offset":8568,"parent_record_offset":7560,"ancestor_record_offset":7560,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":570,"record_offset":8748,"parent_record_offset":7560,"ancestor_record_offset":7560,"inlinee":5303,"source_line":646,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::ManagedResources::create_buffer
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/buffer.rs
//     evidence={"module":570,"record_offset":10108,"parent_record_offset":7560,"ancestor_record_offset":7560,"inlinee":30443,"source_line":55,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":9208,"parent_record_offset":9184,"ancestor_record_offset":7560,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":9232,"parent_record_offset":9208,"ancestor_record_offset":7560,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":8108,"parent_record_offset":8084,"ancestor_record_offset":7560,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":9184,"parent_record_offset":9148,"ancestor_record_offset":7560,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":8084,"parent_record_offset":8028,"ancestor_record_offset":7560,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":570,"record_offset":8384,"parent_record_offset":8360,"ancestor_record_offset":7560,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::impl$2::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\ffi\c_str.rs
//     evidence={"module":570,"record_offset":8332,"parent_record_offset":8308,"ancestor_record_offset":7560,"inlinee":8650,"source_line":703,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":8056,"parent_record_offset":8028,"ancestor_record_offset":7560,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":9148,"parent_record_offset":9120,"ancestor_record_offset":7560,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":9120,"parent_record_offset":9092,"ancestor_record_offset":7560,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":9304,"parent_record_offset":9120,"ancestor_record_offset":7560,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":8028,"parent_record_offset":8000,"ancestor_record_offset":7560,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":570,"record_offset":9056,"parent_record_offset":9024,"ancestor_record_offset":7560,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":570,"record_offset":9024,"parent_record_offset":8992,"ancestor_record_offset":7560,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":570,"record_offset":8960,"parent_record_offset":8928,"ancestor_record_offset":7560,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":570,"record_offset":8992,"parent_record_offset":8960,"ancestor_record_offset":7560,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":570,"record_offset":8928,"parent_record_offset":8896,"ancestor_record_offset":7560,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":570,"record_offset":8896,"parent_record_offset":8852,"ancestor_record_offset":7560,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":9092,"parent_record_offset":9056,"ancestor_record_offset":7560,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":570,"record_offset":7796,"parent_record_offset":7768,"ancestor_record_offset":7560,"inlinee":11125,"source_line":24,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":570,"record_offset":7820,"parent_record_offset":7796,"ancestor_record_offset":7560,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":570,"record_offset":7844,"parent_record_offset":7820,"ancestor_record_offset":7560,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":8772,"parent_record_offset":8748,"ancestor_record_offset":7560,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":570,"record_offset":9368,"parent_record_offset":9344,"ancestor_record_offset":7560,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\maybe_uninit.rs
//     evidence={"module":570,"record_offset":8508,"parent_record_offset":8480,"ancestor_record_offset":7560,"inlinee":17599,"source_line":627,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":570,"record_offset":9344,"parent_record_offset":9056,"ancestor_record_offset":7560,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":7952,"parent_record_offset":7912,"ancestor_record_offset":7560,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":9260,"parent_record_offset":9232,"ancestor_record_offset":7560,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":8684,"parent_record_offset":8596,"ancestor_record_offset":7560,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":8708,"parent_record_offset":8684,"ancestor_record_offset":7560,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":10044,"parent_record_offset":10020,"ancestor_record_offset":7560,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":10068,"parent_record_offset":10044,"ancestor_record_offset":7560,"inlinee":5972,"source_line":424,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":9768,"parent_record_offset":9736,"ancestor_record_offset":7560,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":8628,"parent_record_offset":8596,"ancestor_record_offset":7560,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":9792,"parent_record_offset":9768,"ancestor_record_offset":7560,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":8652,"parent_record_offset":8628,"ancestor_record_offset":7560,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":570,"record_offset":9712,"parent_record_offset":9688,"ancestor_record_offset":7560,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":570,"record_offset":8596,"parent_record_offset":8568,"ancestor_record_offset":7560,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":570,"record_offset":9736,"parent_record_offset":9712,"ancestor_record_offset":7560,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::impl$3::create_buffer_impl::closure$0
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":570,"record_offset":8852,"parent_record_offset":8804,"ancestor_record_offset":7560,"inlinee":30913,"source_line":201,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::impl$3::create_buffer_impl::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":570,"record_offset":10020,"parent_record_offset":9988,"ancestor_record_offset":7560,"inlinee":30917,"source_line":217,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$4::create
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":570,"record_offset":10132,"parent_record_offset":10108,"ancestor_record_offset":7560,"inlinee":11133,"source_line":297,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$7::allocate
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":570,"record_offset":10168,"parent_record_offset":10132,"ancestor_record_offset":7560,"inlinee":30916,"source_line":378,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::impl$3::create_buffer_impl::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 201
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78C1
// 首条位置: {"module":570,"inlinee":30913}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\str.rs
//     evidence={"module":570,"record_offset":8896,"parent_record_offset":8852,"ancestor_record_offset":8852,"inlinee":4411,"source_line":210,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":9424,"parent_record_offset":8852,"ancestor_record_offset":8852,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":9208,"parent_record_offset":9184,"ancestor_record_offset":8852,"inlinee":4228,"source_line":178,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":9232,"parent_record_offset":9208,"ancestor_record_offset":8852,"inlinee":4229,"source_line":94,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":9600,"parent_record_offset":9576,"ancestor_record_offset":8852,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":9184,"parent_record_offset":9148,"ancestor_record_offset":8852,"inlinee":4226,"source_line":242,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":9576,"parent_record_offset":9520,"ancestor_record_offset":8852,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":9548,"parent_record_offset":9520,"ancestor_record_offset":8852,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":9148,"parent_record_offset":9120,"ancestor_record_offset":8852,"inlinee":4225,"source_line":199,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":9120,"parent_record_offset":9092,"ancestor_record_offset":8852,"inlinee":4224,"source_line":154,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::handle_reserve
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":9304,"parent_record_offset":9120,"ancestor_record_offset":8852,"inlinee":4253,"source_line":563,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":9520,"parent_record_offset":9496,"ancestor_record_offset":8852,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":570,"record_offset":9056,"parent_record_offset":9024,"ancestor_record_offset":8852,"inlinee":4420,"source_line":161,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":570,"record_offset":9024,"parent_record_offset":8992,"ancestor_record_offset":8852,"inlinee":4418,"source_line":110,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":570,"record_offset":8960,"parent_record_offset":8928,"ancestor_record_offset":8852,"inlinee":4415,"source_line":412,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":570,"record_offset":8992,"parent_record_offset":8960,"ancestor_record_offset":8852,"inlinee":4416,"source_line":436,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\slice.rs
//     evidence={"module":570,"record_offset":8928,"parent_record_offset":8896,"ancestor_record_offset":8852,"inlinee":4413,"source_line":822,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\vec\mod.rs
//     evidence={"module":570,"record_offset":9092,"parent_record_offset":9056,"ancestor_record_offset":8852,"inlinee":4223,"source_line":687,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":570,"record_offset":9368,"parent_record_offset":9344,"ancestor_record_offset":8852,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":570,"record_offset":9344,"parent_record_offset":9056,"ancestor_record_offset":8852,"inlinee":4421,"source_line":1331,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":9448,"parent_record_offset":9424,"ancestor_record_offset":8852,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_volatile
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":9260,"parent_record_offset":9232,"ancestor_record_offset":8852,"inlinee":4230,"source_line":1654,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::impl$3::create_buffer_impl::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 217
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78C5
// 首条位置: {"module":570,"inlinee":30917}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":10044,"parent_record_offset":10020,"ancestor_record_offset":10020,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":10068,"parent_record_offset":10044,"ancestor_record_offset":10020,"inlinee":5972,"source_line":424,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::impl$3::create_buffer_impl::closure$2
// 记录形式: inlinee_declaration; instances=1
// 源码行: 222
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11272
// 首条位置: {"module":1617,"inlinee":70258}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::create_mapped_buffer
// 记录形式: compiled_procedure; instances=1
// 源码行: 227
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":570,"record_offset":10804,"rva":7574672,"va":5376283792}
// 编译实例 RVAs: 0x739490
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-44283","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376283792,"raw_hex":"e80553ffff","size":5,"soft_fail":false,"target_va":5376239920,"va":5376284198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26694743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376283792,"raw_hex":"e857549701","size":5,"soft_fail":false,"target_va":5402978912,"va":5376284164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26695007","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376283792,"raw_hex":"e85f559701","size":5,"soft_fail":false,"target_va":5402979152,"va":5376284140,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::create_buffer_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-2166","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376283792,"raw_hex":"e88af7ffff","size":5,"soft_fail":false,"target_va":5376281760,"va":5376283921,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":570,"record_offset":11028,"parent_record_offset":10804,"ancestor_record_offset":10804,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":11120,"parent_record_offset":10804,"ancestor_record_offset":10804,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":11184,"parent_record_offset":10804,"ancestor_record_offset":10804,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":11056,"parent_record_offset":10804,"ancestor_record_offset":10804,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":11156,"parent_record_offset":10804,"ancestor_record_offset":10804,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::debug_utils
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":570,"record_offset":10972,"parent_record_offset":10804,"ancestor_record_offset":10804,"inlinee":30918,"source_line":604,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::ManagedRef::into_owner
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":570,"record_offset":11092,"parent_record_offset":10804,"ancestor_record_offset":10804,"inlinee":11060,"source_line":310,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":10996,"parent_record_offset":10972,"ancestor_record_offset":10804,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":11208,"parent_record_offset":11184,"ancestor_record_offset":10804,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::create_buffer
// 记录形式: compiled_procedure, inlinee_declaration; instances=2
// 源码行: 249
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7925
// 首条位置: {"module":570,"inlinee":31013}
// 编译实例 RVAs: 0x739640
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-45114","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376284224,"raw_hex":"e8c64fffff","size":5,"soft_fail":false,"target_va":5376239920,"va":5376285029,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14118c440
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10824428","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376284224,"raw_hex":"e8ec2aa500","size":5,"soft_fail":false,"target_va":5387109440,"va":5376285007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27308836","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376284224,"raw_hex":"e824b3a001","size":5,"soft_fail":false,"target_va":5403593360,"va":5376284519,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26693663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376284224,"raw_hex":"e81f509701","size":5,"soft_fail":false,"target_va":5402978912,"va":5376285244,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26693972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376284224,"raw_hex":"e854519701","size":5,"soft_fail":false,"target_va":5402979152,"va":5376285175,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26695107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376284224,"raw_hex":"e8c3559701","size":5,"soft_fail":false,"target_va":5402980304,"va":5376285192,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::create_buffer_impl
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-2616","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376284224,"raw_hex":"e8c8f5ffff","size":5,"soft_fail":false,"target_va":5376281760,"va":5376284371,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":570,"record_offset":11568,"parent_record_offset":11272,"ancestor_record_offset":11272,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::impl$286::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":570,"record_offset":11484,"parent_record_offset":11272,"ancestor_record_offset":11272,"inlinee":30634,"source_line":45,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":11968,"parent_record_offset":11272,"ancestor_record_offset":11272,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":11452,"parent_record_offset":11272,"ancestor_record_offset":11272,"inlinee":4711,"source_line":607,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":12156,"parent_record_offset":11272,"ancestor_record_offset":11272,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":11596,"parent_record_offset":11272,"ancestor_record_offset":11272,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":11784,"parent_record_offset":11272,"ancestor_record_offset":11272,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":570,"record_offset":11700,"parent_record_offset":11272,"ancestor_record_offset":11272,"inlinee":5235,"source_line":3626,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":11632,"parent_record_offset":11272,"ancestor_record_offset":11272,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu_low_prio
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":570,"record_offset":11812,"parent_record_offset":11272,"ancestor_record_offset":11272,"inlinee":30617,"source_line":105,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::debug_utils
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":570,"record_offset":11512,"parent_record_offset":11272,"ancestor_record_offset":11272,"inlinee":30918,"source_line":604,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::ManagedRef::into_owner
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":570,"record_offset":11756,"parent_record_offset":11272,"ancestor_record_offset":11272,"inlinee":11060,"source_line":310,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":570,"record_offset":11724,"parent_record_offset":11700,"ancestor_record_offset":11272,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":11536,"parent_record_offset":11512,"ancestor_record_offset":11272,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":12180,"parent_record_offset":12156,"ancestor_record_offset":11272,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":11660,"parent_record_offset":11632,"ancestor_record_offset":11272,"inlinee":5972,"source_line":424,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::impl$3::create_buffer::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 292
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11355
// 首条位置: {"module":1624,"inlinee":70485}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::cmd_copy_buffer
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1624,"record_offset":5604,"parent_record_offset":4976,"ancestor_record_offset":4976,"inlinee":67180,"source_line":1466,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::ManagedRef::get_tracked
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1624,"record_offset":5028,"parent_record_offset":4976,"ancestor_record_offset":4976,"inlinee":30923,"source_line":327,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":1624,"record_offset":5272,"parent_record_offset":5180,"ancestor_record_offset":4976,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU64::fetch_max
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1624,"record_offset":5216,"parent_record_offset":5180,"ancestor_record_offset":4976,"inlinee":30861,"source_line":2932,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_umax
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1624,"record_offset":5240,"parent_record_offset":5216,"ancestor_record_offset":4976,"inlinee":26109,"source_line":3538,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::CommandBufferSeqTracker::track
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":1624,"record_offset":5180,"parent_record_offset":5092,"ancestor_record_offset":4976,"inlinee":30860,"source_line":54,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$17::track
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1624,"record_offset":5064,"parent_record_offset":5028,"ancestor_record_offset":4976,"inlinee":7654,"source_line":250,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1624,"record_offset":5152,"parent_record_offset":5092,"ancestor_record_offset":4976,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$4::track
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1624,"record_offset":5092,"parent_record_offset":5064,"ancestor_record_offset":4976,"inlinee":30867,"source_line":94,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::ManagedResourceMeta::checksum
//     owner=private_source_ownership_candidate; count=4
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1624,"record_offset":5124,"parent_record_offset":5092,"ancestor_record_offset":4976,"inlinee":30448,"source_line":43,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::set_buffer_debug_name
// 记录形式: compiled_procedure; instances=1
// 源码行: 313
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":570,"record_offset":12776,"rva":7576288,"va":5376285408}
// 编译实例 RVAs: 0x739ae0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1403da790
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3535910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376285408,"raw_hex":"e8da0bcaff","size":5,"soft_fail":false,"target_va":5372749712,"va":5376285617,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140869090
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1242296","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376285408,"raw_hex":"e8b8f41200","size":5,"soft_fail":false,"target_va":5377527952,"va":5376285651,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26693043","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376285408,"raw_hex":"e8b34d9701","size":5,"soft_fail":false,"target_va":5402978880,"va":5376285832,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26940580","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376285408,"raw_hex":"e8a4149b01","size":5,"soft_fail":false,"target_va":5403226480,"va":5376285895,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26694118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376285408,"raw_hex":"e8e6519701","size":5,"soft_fail":false,"target_va":5402980128,"va":5376286005,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_allocator::vulkan::Allocator::rename_allocation
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t118743","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376285408,"raw_hex":"e8d7cf0100","size":5,"soft_fail":false,"target_va":5376404320,"va":5376285572,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t26683182","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376285408,"raw_hex":"e82e279701","size":5,"soft_fail":false,"target_va":5402968992,"va":5376285805,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t26684947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376285408,"raw_hex":"e8132e9701","size":5,"soft_fail":false,"target_va":5402970800,"va":5376285848,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*2112(%rsi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5376285408,"raw_hex":"ff9640080000","size":6,"soft_fail":false,"va":5376285737,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::ffi::c_str::CString::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\ffi\c_str.rs
//     evidence={"module":570,"record_offset":13996,"parent_record_offset":12776,"ancestor_record_offset":12776,"inlinee":9051,"source_line":263,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::extensions_generated::ext::debug_utils::Device::set_debug_utils_object_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/extensions/ext/debug_utils.rs
//     evidence={"module":570,"record_offset":14304,"parent_record_offset":12776,"ancestor_record_offset":12776,"inlinee":11129,"source_line":11,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DebugUtilsObjectNameInfoEXT::object_handle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":570,"record_offset":14276,"parent_record_offset":12776,"ancestor_record_offset":12776,"inlinee":11127,"source_line":19604,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":13152,"parent_record_offset":12776,"ancestor_record_offset":12776,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":13552,"parent_record_offset":12776,"ancestor_record_offset":12776,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":14388,"parent_record_offset":12776,"ancestor_record_offset":12776,"inlinee":5301,"source_line":1028,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":14024,"parent_record_offset":12776,"ancestor_record_offset":12776,"inlinee":5280,"source_line":1071,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":570,"record_offset":13188,"parent_record_offset":12776,"ancestor_record_offset":12776,"inlinee":5282,"source_line":213,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$13::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":570,"record_offset":13468,"parent_record_offset":12776,"ancestor_record_offset":12776,"inlinee":10631,"source_line":639,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::allocator::GpuAllocator::rename_allocation
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/allocator.rs
//     evidence={"module":570,"record_offset":13524,"parent_record_offset":12776,"ancestor_record_offset":12776,"inlinee":30568,"source_line":76,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::debug_utils
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":570,"record_offset":12968,"parent_record_offset":12776,"ancestor_record_offset":12776,"inlinee":30918,"source_line":604,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::buffer::ManagedBuffer::allocation
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/buffer.rs
//     evidence={"module":570,"record_offset":13368,"parent_record_offset":12776,"ancestor_record_offset":12776,"inlinee":30599,"source_line":38,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":570,"record_offset":13024,"parent_record_offset":12776,"ancestor_record_offset":12776,"inlinee":7658,"source_line":314,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":13756,"parent_record_offset":13732,"ancestor_record_offset":12776,"inlinee":4121,"source_line":118,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\alloc.rs
//     evidence={"module":570,"record_offset":13732,"parent_record_offset":13676,"ancestor_record_offset":12776,"inlinee":4119,"source_line":252,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\boxed.rs
//     evidence={"module":570,"record_offset":14500,"parent_record_offset":14476,"ancestor_record_offset":12776,"inlinee":4113,"source_line":1240,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::impl$2::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\ffi\c_str.rs
//     evidence={"module":570,"record_offset":14448,"parent_record_offset":14424,"ancestor_record_offset":12776,"inlinee":8650,"source_line":703,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":13704,"parent_record_offset":13676,"ancestor_record_offset":12776,"inlinee":4256,"source_line":279,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$4::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\raw_vec.rs
//     evidence={"module":570,"record_offset":13676,"parent_record_offset":13648,"ancestor_record_offset":12776,"inlinee":4255,"source_line":553,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":570,"record_offset":14328,"parent_record_offset":14304,"ancestor_record_offset":12776,"inlinee":11115,"source_line":11,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":570,"record_offset":14352,"parent_record_offset":14328,"ancestor_record_offset":12776,"inlinee":11116,"source_line":16,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":570,"record_offset":13492,"parent_record_offset":13468,"ancestor_record_offset":12776,"inlinee":5305,"source_line":2122,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":12992,"parent_record_offset":12968,"ancestor_record_offset":12776,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":13428,"parent_record_offset":13368,"ancestor_record_offset":12776,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":13576,"parent_record_offset":13552,"ancestor_record_offset":12776,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":13304,"parent_record_offset":13216,"ancestor_record_offset":12776,"inlinee":5288,"source_line":581,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":13328,"parent_record_offset":13304,"ancestor_record_offset":12776,"inlinee":4753,"source_line":538,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":13900,"parent_record_offset":13868,"ancestor_record_offset":12776,"inlinee":5297,"source_line":2580,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":13248,"parent_record_offset":13216,"ancestor_record_offset":12776,"inlinee":5286,"source_line":2629,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":13924,"parent_record_offset":13900,"ancestor_record_offset":12776,"inlinee":5298,"source_line":3369,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":570,"record_offset":13272,"parent_record_offset":13248,"ancestor_record_offset":12776,"inlinee":5287,"source_line":3404,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":570,"record_offset":13840,"parent_record_offset":13812,"ancestor_record_offset":12776,"inlinee":5295,"source_line":653,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":570,"record_offset":13216,"parent_record_offset":13188,"ancestor_record_offset":12776,"inlinee":5284,"source_line":66,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":570,"record_offset":13868,"parent_record_offset":13840,"ancestor_record_offset":12776,"inlinee":5296,"source_line":100,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":570,"record_offset":13068,"parent_record_offset":13024,"ancestor_record_offset":12776,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":570,"record_offset":13120,"parent_record_offset":13024,"ancestor_record_offset":12776,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":570,"record_offset":13096,"parent_record_offset":13024,"ancestor_record_offset":12776,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::update_buffer_bytes
// 记录形式: compiled_procedure; instances=1
// 源码行: 352
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":570,"record_offset":14956,"rva":7577168,"va":5376286288}
// 编译实例 RVAs: 0x739e50
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x14072e930
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-47289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376286288,"raw_hex":"e84747ffff","size":5,"soft_fail":false,"target_va":5376239920,"va":5376287204,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14118cd00
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10824668","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376286288,"raw_hex":"e8dc2ba500","size":5,"soft_fail":false,"target_va":5387111680,"va":5376287007,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142144a90
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t27306471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376286288,"raw_hex":"e8e7a9a001","size":5,"soft_fail":false,"target_va":5403593360,"va":5376286884,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26691314","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376286288,"raw_hex":"e8f2469701","size":5,"soft_fail":false,"target_va":5402978912,"va":5376287593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26691432","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376286288,"raw_hex":"e868479701","size":5,"soft_fail":false,"target_va":5402978880,"va":5376287443,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26939095","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376286288,"raw_hex":"e8d70e9b01","size":5,"soft_fail":false,"target_va":5403226480,"va":5376287380,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26692610","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376286288,"raw_hex":"e8024c9701","size":5,"soft_fail":false,"target_va":5402979152,"va":5376286537,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t26691394","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376286288,"raw_hex":"e842479701","size":5,"soft_fail":false,"target_va":5402978944,"va":5376287545,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t26693042","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376286288,"raw_hex":"e8b24d9701","size":5,"soft_fail":false,"target_va":5402980304,"va":5376287257,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t26693076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376286288,"raw_hex":"e8d44d9701","size":5,"soft_fail":false,"target_va":5402980320,"va":5376287239,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::create_buffer_impl
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4941","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376286288,"raw_hex":"e8b3ecffff","size":5,"soft_fail":false,"target_va":5376281760,"va":5376286696,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\alloc\src\sync.rs
//     evidence={"module":570,"record_offset":15700,"parent_record_offset":14956,"ancestor_record_offset":14956,"inlinee":4586,"source_line":2105,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::BufferUsageFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":570,"record_offset":15572,"parent_record_offset":14956,"ancestor_record_offset":14956,"inlinee":30911,"source_line":32,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\fmt\mod.rs
//     evidence={"module":570,"record_offset":16488,"parent_record_offset":14956,"ancestor_record_offset":14956,"inlinee":4333,"source_line":339,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":15764,"parent_record_offset":14956,"ancestor_record_offset":14956,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":15288,"parent_record_offset":14956,"ancestor_record_offset":14956,"inlinee":5205,"source_line":930,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":16376,"parent_record_offset":14956,"ancestor_record_offset":14956,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":15728,"parent_record_offset":14956,"ancestor_record_offset":14956,"inlinee":4959,"source_line":1950,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":570,"record_offset":16320,"parent_record_offset":14956,"ancestor_record_offset":14956,"inlinee":4109,"source_line":1962,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":570,"record_offset":16088,"parent_record_offset":14956,"ancestor_record_offset":14956,"inlinee":5235,"source_line":3626,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":15500,"parent_record_offset":14956,"ancestor_record_offset":14956,"inlinee":5452,"source_line":17,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":16020,"parent_record_offset":14956,"ancestor_record_offset":14956,"inlinee":4868,"source_line":28,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":570,"record_offset":15324,"parent_record_offset":14956,"ancestor_record_offset":14956,"inlinee":7698,"source_line":24,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":570,"record_offset":15436,"parent_record_offset":14956,"ancestor_record_offset":14956,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu_low_prio
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":570,"record_offset":15600,"parent_record_offset":14956,"ancestor_record_offset":14956,"inlinee":30617,"source_line":105,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::debug_utils
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":570,"record_offset":15640,"parent_record_offset":14956,"ancestor_record_offset":14956,"inlinee":30918,"source_line":604,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":570,"record_offset":15160,"parent_record_offset":14956,"ancestor_record_offset":14956,"inlinee":7658,"source_line":314,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\intrinsics.rs
//     evidence={"module":570,"record_offset":16112,"parent_record_offset":16088,"ancestor_record_offset":14956,"inlinee":4285,"source_line":2962,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":570,"record_offset":15668,"parent_record_offset":15640,"ancestor_record_offset":14956,"inlinee":4150,"source_line":682,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\const_ptr.rs
//     evidence={"module":570,"record_offset":15980,"parent_record_offset":15956,"ancestor_record_offset":14956,"inlinee":4375,"source_line":959,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":570,"record_offset":16400,"parent_record_offset":16376,"ancestor_record_offset":14956,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":570,"record_offset":15396,"parent_record_offset":15372,"ancestor_record_offset":14956,"inlinee":4181,"source_line":1044,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":570,"record_offset":15468,"parent_record_offset":15436,"ancestor_record_offset":14956,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\mod.rs
//     evidence={"module":570,"record_offset":15348,"parent_record_offset":15324,"ancestor_record_offset":14956,"inlinee":4177,"source_line":1034,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":15956,"parent_record_offset":15912,"ancestor_record_offset":14956,"inlinee":5475,"source_line":373,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":16216,"parent_record_offset":16192,"ancestor_record_offset":14956,"inlinee":5946,"source_line":395,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":15528,"parent_record_offset":15500,"ancestor_record_offset":14956,"inlinee":6002,"source_line":413,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":16048,"parent_record_offset":16020,"ancestor_record_offset":14956,"inlinee":5972,"source_line":424,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":16192,"parent_record_offset":16168,"ancestor_record_offset":14956,"inlinee":5945,"source_line":495,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$6::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\index.rs
//     evidence={"module":570,"record_offset":16168,"parent_record_offset":16144,"ancestor_record_offset":14956,"inlinee":5944,"source_line":510,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter.rs
//     evidence={"module":570,"record_offset":15372,"parent_record_offset":15348,"ancestor_record_offset":14956,"inlinee":4179,"source_line":89,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":570,"record_offset":15204,"parent_record_offset":15160,"ancestor_record_offset":14956,"inlinee":7648,"source_line":14,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":570,"record_offset":15256,"parent_record_offset":15160,"ancestor_record_offset":14956,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":570,"record_offset":15232,"parent_record_offset":15160,"ancestor_record_offset":14956,"inlinee":7650,"source_line":32,"archive":"_pdb/A/modules/00570.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::impl$3::update_buffer_bytes::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 401
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1624,"record_offset":972,"rva":18399024,"va":5387108144}
// 编译实例 RVAs: 0x118bf30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$bumpalo..Bump$u20$as$u20$core..ops..drop..Drop$GT$::drop
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t-10206266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387108144,"raw_hex":"e8c64364ff","size":5,"soft_fail":false,"target_va":5376902512,"va":5387108773,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407d0550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10205845","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387108144,"raw_hex":"e86b4564ff","size":5,"soft_fail":false,"target_va":5376902480,"va":5387108320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::Bump::alloc_layout_slow
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-10205751","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387108144,"raw_hex":"e8c94564ff","size":5,"soft_fail":false,"target_va":5376902656,"va":5387108402,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::oom
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t15862632","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387108144,"raw_hex":"e8680bf200","size":5,"soft_fail":false,"target_va":5402971536,"va":5387108899,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15870089","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387108144,"raw_hex":"e88928f200","size":5,"soft_fail":false,"target_va":5402978912,"va":5387108818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t15870045","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5387108144,"raw_hex":"e85d28f200","size":5,"soft_fail":false,"target_va":5402978944,"va":5387108894,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*%rax","direct":false,"event":"call","llvm_opcode":"CALL64r","range_va":5387108144,"raw_hex":"ffd0","size":2,"soft_fail":false,"va":5387108637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::cmd_copy_buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1624,"record_offset":2844,"parent_record_offset":972,"ancestor_record_offset":972,"inlinee":67180,"source_line":1466,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1624,"record_offset":1580,"parent_record_offset":972,"ancestor_record_offset":972,"inlinee":64630,"source_line":1351,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1624,"record_offset":2932,"parent_record_offset":972,"ancestor_record_offset":972,"inlinee":4111,"source_line":515,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::impl$3::update_buffer_bytes::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":1624,"record_offset":2780,"parent_record_offset":972,"ancestor_record_offset":972,"inlinee":70480,"source_line":406,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::ManagedRef::get_tracked
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1624,"record_offset":1344,"parent_record_offset":972,"ancestor_record_offset":972,"inlinee":30923,"source_line":327,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$17::track
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1624,"record_offset":1140,"parent_record_offset":972,"ancestor_record_offset":972,"inlinee":7654,"source_line":250,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::cmd_pipeline_barrier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1624,"record_offset":2812,"parent_record_offset":2780,"ancestor_record_offset":972,"inlinee":30903,"source_line":2370,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_layout
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1624,"record_offset":1772,"parent_record_offset":1616,"ancestor_record_offset":972,"inlinee":6155,"source_line":1396,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_with
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1624,"record_offset":1616,"parent_record_offset":1580,"ancestor_record_offset":972,"inlinee":30946,"source_line":1267,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1624,"record_offset":1804,"parent_record_offset":1772,"ancestor_record_offset":972,"inlinee":6156,"source_line":1411,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1624,"record_offset":1840,"parent_record_offset":1804,"ancestor_record_offset":972,"inlinee":6157,"source_line":1420,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$8::alloc_layout::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1624,"record_offset":2204,"parent_record_offset":2172,"ancestor_record_offset":972,"inlinee":6179,"source_line":1397,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$8::alloc_slice_fill_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1624,"record_offset":2348,"parent_record_offset":1616,"ancestor_record_offset":972,"inlinee":64632,"source_line":1357,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$8::alloc_slice_fill_with::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.12.0","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bumpalo-3.12.0/src/lib.rs
//     evidence={"module":1624,"record_offset":1740,"parent_record_offset":1672,"ancestor_record_offset":972,"inlinee":64636,"source_line":1271,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1624,"record_offset":1672,"parent_record_offset":1616,"ancestor_record_offset":972,"inlinee":4817,"source_line":433,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\alloc\layout.rs
//     evidence={"module":1624,"record_offset":1712,"parent_record_offset":1672,"ancestor_record_offset":972,"inlinee":4819,"source_line":438,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1624,"record_offset":1884,"parent_record_offset":1840,"ancestor_record_offset":972,"inlinee":5551,"source_line":518,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1624,"record_offset":2076,"parent_record_offset":2052,"ancestor_record_offset":972,"inlinee":4790,"source_line":479,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cell.rs
//     evidence={"module":1624,"record_offset":2052,"parent_record_offset":1840,"ancestor_record_offset":972,"inlinee":4789,"source_line":410,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$54::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\cmp.rs
//     evidence={"module":1624,"record_offset":2304,"parent_record_offset":2272,"ancestor_record_offset":972,"inlinee":4237,"source_line":1553,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\adapters\map.rs
//     evidence={"module":1624,"record_offset":2380,"parent_record_offset":2348,"ancestor_record_offset":972,"inlinee":4423,"source_line":107,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1624,"record_offset":2272,"parent_record_offset":2240,"ancestor_record_offset":972,"inlinee":4235,"source_line":753,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1624,"record_offset":2240,"parent_record_offset":1616,"ancestor_record_offset":972,"inlinee":4233,"source_line":844,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1624,"record_offset":2660,"parent_record_offset":2636,"ancestor_record_offset":972,"inlinee":8391,"source_line":849,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1624,"record_offset":2636,"parent_record_offset":2600,"ancestor_record_offset":972,"inlinee":6044,"source_line":116,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\mem\mod.rs
//     evidence={"module":1624,"record_offset":2100,"parent_record_offset":2076,"ancestor_record_offset":972,"inlinee":4135,"source_line":912,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ops\function.rs
//     evidence={"module":1624,"record_offset":2568,"parent_record_offset":2536,"ancestor_record_offset":972,"inlinee":4428,"source_line":304,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1624,"record_offset":2708,"parent_record_offset":2348,"ancestor_record_offset":972,"inlinee":5324,"source_line":893,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\option.rs
//     evidence={"module":1624,"record_offset":2536,"parent_record_offset":2380,"ancestor_record_offset":972,"inlinee":4133,"source_line":1068,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1624,"record_offset":1964,"parent_record_offset":1940,"ancestor_record_offset":972,"inlinee":6159,"source_line":563,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1624,"record_offset":1940,"parent_record_offset":1840,"ancestor_record_offset":972,"inlinee":6158,"source_line":1299,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1624,"record_offset":2500,"parent_record_offset":2476,"ancestor_record_offset":972,"inlinee":4426,"source_line":618,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$12::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\non_null.rs
//     evidence={"module":1624,"record_offset":2448,"parent_record_offset":2408,"ancestor_record_offset":972,"inlinee":4187,"source_line":1797,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\ptr\mod.rs
//     evidence={"module":1624,"record_offset":2124,"parent_record_offset":2100,"ancestor_record_offset":972,"inlinee":4137,"source_line":1471,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\result.rs
//     evidence={"module":1624,"record_offset":2172,"parent_record_offset":1772,"ancestor_record_offset":972,"inlinee":6160,"source_line":1428,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1624,"record_offset":2476,"parent_record_offset":2408,"ancestor_record_offset":972,"inlinee":5091,"source_line":101,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$181::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\slice\iter\macros.rs
//     evidence={"module":1624,"record_offset":2408,"parent_record_offset":2380,"ancestor_record_offset":972,"inlinee":4185,"source_line":156,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU64::fetch_max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1624,"record_offset":1276,"parent_record_offset":1248,"ancestor_record_offset":972,"inlinee":30861,"source_line":2932,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_umax
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\sync\atomic.rs
//     evidence={"module":1624,"record_offset":1300,"parent_record_offset":1276,"ancestor_record_offset":972,"inlinee":26109,"source_line":3538,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::impl$3::update_buffer_bytes::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":1624,"record_offset":2600,"parent_record_offset":2568,"ancestor_record_offset":972,"inlinee":70479,"source_line":428,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::CommandBufferSeqTracker::track
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":1624,"record_offset":1248,"parent_record_offset":1164,"ancestor_record_offset":972,"inlinee":30860,"source_line":54,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$17::track
//     owner=private_source_ownership_candidate; count=1
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1624,"record_offset":1372,"parent_record_offset":1344,"ancestor_record_offset":972,"inlinee":7654,"source_line":250,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1624,"record_offset":1220,"parent_record_offset":1164,"ancestor_record_offset":972,"inlinee":7652,"source_line":23,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$4::track
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1624,"record_offset":1164,"parent_record_offset":1140,"ancestor_record_offset":972,"inlinee":30867,"source_line":94,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::ManagedResourceMeta::checksum
//     owner=private_source_ownership_candidate; count=2
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":1624,"record_offset":1192,"parent_record_offset":1164,"ancestor_record_offset":972,"inlinee":30448,"source_line":43,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::impl$3::update_buffer_bytes::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 406
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x11350
// 首条位置: {"module":1624,"inlinee":70480}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::cmd_pipeline_barrier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":1624,"record_offset":2812,"parent_record_offset":2780,"ancestor_record_offset":2780,"inlinee":30903,"source_line":2370,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::impl$3::update_buffer_bytes::closure$0::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 428
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1134F
// 首条位置: {"module":1624,"inlinee":70479}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1624,"record_offset":2636,"parent_record_offset":2600,"ancestor_record_offset":2600,"inlinee":6044,"source_line":116,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6\library\core\src\iter\range.rs
//     evidence={"module":1624,"record_offset":2660,"parent_record_offset":2636,"ancestor_record_offset":2600,"inlinee":8391,"source_line":849,"archive":"_pdb/A/modules/01624.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::get_vk_buffer
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=8
// 源码行: 450
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7833
// 首条位置: {"module":565,"inlinee":30771}
// 编译实例 RVAs: 0x73a3b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x141177460
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10735776","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5376287664,"raw_hex":"e8a0d0a300","size":5,"soft_fail":false,"target_va":5387023456,"va":5376287675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
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
// 完整结构及原始类型记录: buffer.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::buffer::Buffer
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 12
// [PDB 泛型关系，非字段关系] {"type": "anyhow::context::impl$0::with_context<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::raw::error::BackendError>,alloc::string::String,rhapsody::rg::temporal::impl$12::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > >", "constructor": "anyhow::context::impl$0::with_context", "arguments": ["rhapsody::raw::buffer::Buffer", "enum2$<rhapsody::raw::error::BackendError>", "alloc::string::String", "rhapsody::rg::temporal::impl$12::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::reserve_for_push<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::reserve_for_push", "arguments": ["rhapsody::raw::buffer::Buffer", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["rhapsody::raw::buffer::Buffer", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >", "constructor": "tuple$", "arguments": ["rhapsody::raw::buffer::BufferDesc", "alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["rhapsody::raw::buffer::BufferDesc", "alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::clear<tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTable::clear", "arguments": ["tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["rhapsody::raw::buffer::BufferDesc", "alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::buffer::BufferDesc
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >", "constructor": "tuple$", "arguments": ["rhapsody::raw::buffer::BufferDesc", "alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["rhapsody::raw::buffer::BufferDesc", "alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::clear<tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTable::clear", "arguments": ["tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<rhapsody::raw::buffer::BufferDesc>", "constructor": "ref$", "arguments": ["rhapsody::raw::buffer::BufferDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<rhapsody::raw::buffer::BufferDesc> >", "constructor": "core::hash::BuildHasher::hash_one", "arguments": ["std::hash::random::RandomState", "ref$<rhapsody::raw::buffer::BufferDesc>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["rhapsody::raw::buffer::BufferDesc", "alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: buffer.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: buffer.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/A/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 2; 完整候选见附档及 _pdb/A/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>,bevy_ecs::event::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>", "bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>", "bevy_ecs::event::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::audio::roof_audio::RoofAudioState>
// [候选系统接口] event_reader: bevy_ecs::event::EventReader<country_core::resources::roofs::roof_sounds::PlayRoofSound>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::Buffer::device_address
// 信息 ID: 7f8fb0c2186f1dd84a07ecd7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists @ 0x60f627 -> 0x72eab0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::raw::dynamic_constants::DynamicConstants::new; module=565, inline_record=3716; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::dynamic_constants::DynamicConstants::allocate; module=565, inline_record=4348; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::Buffer::mapped_slice_mut
// 信息 ID: 80d387a2ddb51e788d1fdb1d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 20 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::dynamic_constants::DynamicConstants::new @ 0x733f86 -> 0x72eb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::dynamic_constants::DynamicConstants::allocate @ 0x734220 -> 0x72eb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::last_known_frame_duration_ns @ 0x736bdc -> 0x72eb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$4::new_storage_buffer<country_core::resources::terrain_rock_mask::TerrainRockMaskSsbo> @ 0xf8684c -> 0x72eb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$4::new_storage_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo> @ 0xf86c97 -> 0x72eb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$4::new_storage_buffer<country_core::render::normals_debug::NormalDebugSsbo> @ 0xf8702b -> 0x72eb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$4::new_storage_buffer<country_core::systems::audio::stairs_audio::StairsCounterSsbo> @ 0xf8738f -> 0x72eb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$4::new_storage_buffer<country_core::resources::world_raster_defs::PathRasterStagingSsbo> @ 0xf877d7 -> 0x72eb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$4::new_storage_buffer<country_core::systems::render_loop::resources::critters::FireflyVisibilitySsbo> @ 0xf87b8c -> 0x72eb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$4::new_storage_buffer<country_core::resources::world_raster_defs::WaterRasterStagingSsbo> @ 0xf87fd7 -> 0x72eb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$4::new_storage_buffer<country_core::resources::compute_mouse_depth_value::MouseDepthValueSsbo> @ 0xf88383 -> 0x72eb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$4::new_storage_buffer<country_core::systems::render_loop::resources::critters::ButterflySsbo> @ 0xf88b0a -> 0x72eb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$4::new_storage_buffer<country_core::resources::water_mask::WaterMaskSsbo> @ 0xf88f7c -> 0x72eb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$4::new_storage_buffer<country_core::resources::garden_mask::GardenMaskSsbo> @ 0xf893ec -> 0x72eb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$4::new_storage_buffer<country_core::resources::terrain_height::TerrainHeightSsbo> @ 0xf8985c -> 0x72eb80 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$4::new_storage_buffer<country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::MouseTerrainRaycastSsbo> @ 0xf89cc6 -> 0x72eb80 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::BufferDesc::alignment
// 信息 ID: cbb2afd6d53f704eda0f19a4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 86 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::geometry::instanced_wall::BrickDrawListSsbo> @ 0xf7d35f -> 0x72ed90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo> @ 0xf7d520 -> 0x72ed90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo> @ 0xf7d7cf -> 0x72ed90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo> @ 0xf7d9fd -> 0x72ed90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::geometry::instanced_wall::BrickNextVisibilityBitmaskSsbo> @ 0xf7dd3d -> 0x72ed90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer> @ 0xf7e09c -> 0x72ed90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::resources::rocky_terrain::RockyTerrainTriangleSSBO> @ 0xf7e3ef -> 0x72ed90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::stairs::StairsTransformSsbo> @ 0xf7e61f -> 0x72ed90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::resources::render::spatial_resolve_offsets::SpatialResolveOffsetsSsbo> @ 0xf7e885 -> 0x72ed90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::grass::GrassBladeDataSsbo> @ 0xf7ec0f -> 0x72ed90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo> @ 0xf7ee05 -> 0x72ed90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo> @ 0xf7f09f -> 0x72ed90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::indirect_params::DispatchIndirectBuffer> @ 0xf7f2e3 -> 0x72ed90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo> @ 0xf7f61c -> 0x72ed90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::render::nani::buffers::NaniDrawListSsbo> @ 0xf7f816 -> 0x72ed90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::resources::render::lantern::LanternSsbo> @ 0xf7fa74 -> 0x72ed90 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::raw::dynamic_constants::DynamicConstants::allocate_buffer; module=565, inline_record=5296; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::BufferDesc::new
// 信息 ID: 9a67510459a07c790bf68d55
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 88 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::painterly::painterly_pass @ 0x5eed57 -> 0x72ec90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::vermeer::vermeer_render_from_target @ 0x63d8b9 -> 0x72ec90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::geometry::instanced_wall::BrickDrawListSsbo> @ 0xf7d34a -> 0x72ec90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo> @ 0xf7d50b -> 0x72ec90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo> @ 0xf7d7ba -> 0x72ec90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo> @ 0xf7d9d9 -> 0x72ec90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::geometry::instanced_wall::BrickNextVisibilityBitmaskSsbo> @ 0xf7dd19 -> 0x72ec90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer> @ 0xf7e078 -> 0x72ec90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::resources::rocky_terrain::RockyTerrainTriangleSSBO> @ 0xf7e3da -> 0x72ec90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::stairs::StairsTransformSsbo> @ 0xf7e60a -> 0x72ec90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::resources::render::spatial_resolve_offsets::SpatialResolveOffsetsSsbo> @ 0xf7e85e -> 0x72ec90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::grass::GrassBladeDataSsbo> @ 0xf7ebfa -> 0x72ec90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo> @ 0xf7edf0 -> 0x72ec90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::render::nani::buffers::NaniInstanceIndirectionSsbo> @ 0xf7f08a -> 0x72ec90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::indirect_params::DispatchIndirectBuffer> @ 0xf7f2bf -> 0x72ec90 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskMeanSsbo> @ 0xf7f607 -> 0x72ec90 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu
// 信息 ID: dce6081829d5e0b66997ca0a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x5d2e4 -> 0x72ed10 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x5d38b -> 0x72ed10 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_screenshot::create_graphics_resources @ 0x3c1f22 -> 0x72ed10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu_low_prio
// 信息 ID: c8993785437a0009e8bddac4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_buffer; module=570, inline_record=11812; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::update_buffer_bytes; module=570, inline_record=15600; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::update_texture_impl; module=570, inline_record=81764; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::BufferDesc::new_gpu_only
// 信息 ID: 2d8c0cab5b7aebd557140479
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 25 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::generate_mips::generate_mips<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0xdc56c -> 0x72ecd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::generate_mips::generate_mips<country_core::resources::texture_presets::TexturePreset> @ 0x5ada0a -> 0x72ecd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$4::call_once<tuple$<tuple$<ref$<alloc::string::String>,ref$<enum2$<country_core::render::mesh::VertexAttributeValues> > > >,country_core::render::vao::build_vao::closure_env$0> @ 0x5daf99 -> 0x72ecd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::vao::build_vao @ 0x5dc823 -> 0x72ecd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass @ 0x608bf8 -> 0x72ecd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass @ 0x608d73 -> 0x72ecd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass @ 0x609da2 -> 0x72ecd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass @ 0x609e16 -> 0x72ecd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass @ 0x609eea -> 0x72ecd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass @ 0x609f67 -> 0x72ecd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists @ 0x60ef64 -> 0x72ecd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists @ 0x60efd0 -> 0x72ecd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::generate_hzb::generate_hzb @ 0x6561e8 -> 0x72ecd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::fill_sorted_instance_list<country_core::render::nani::gpu_world::RenderSublistsIter> @ 0x690fe1 -> 0x72ecd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::prefix_sum_unsorted_instances_per_atlas_entry<country_core::render::nani::gpu_world::RenderSublistsIter> @ 0x691344 -> 0x72ecd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::grass::grass_compute @ 0x691a50 -> 0x72ecd0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::BufferDesc::new_gpu_to_cpu
// 信息 ID: d0d4cd54826e8a0661caf6bc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x15b976 -> 0x72ed50 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::rg::executor::GraphExecutor::new; module=567, inline_record=5368; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create; module=570, inline_record=44540; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$10::eq
// 信息 ID: 45b0827fe73ef655f7ceb737
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::resource_pool::ResourcePool::get_buffer; module=575, inline_record=2732; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::resource_pool::ResourcePool::insert_buffer; module=575, inline_record=4056; 不是独立调用边。
// [PDB 内联宿主] hashbrown::map::HashMap::insert<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>; module=1612, inline_record=2516; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$3::create_buffer::closure$0
// 信息 ID: 071dd3b8dcced075d68bc463
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::buffer::impl$3::create_buffer", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::buffer::impl$3::create_buffer::closure_env$0 <- rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$3::create_buffer::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$3::create_buffer::closure_env$0>; module=1624, inline_record=4976; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$3::create_buffer::closure_env$0>; module=1624, inline_record=5988; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$3::create_buffer_impl::closure$0
// 信息 ID: 76f2b06bd831a71bb54c4d72
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::buffer::impl$3::create_buffer_impl", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_buffer_impl; module=570, inline_record=8852; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$3::create_buffer_impl::closure$1
// 信息 ID: 03ff5dcf8af689c12d632b6e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::buffer::impl$3::create_buffer_impl", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_buffer_impl; module=570, inline_record=10020; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$3::create_buffer_impl::closure$2
// 信息 ID: 5f10fca045b5a03b395a0e3b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "rhapsody::raw::buffer::impl$3::create_buffer_impl", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$3::update_buffer_bytes::closure$0
// 信息 ID: a5051144edb41ddeaacbf0ae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::buffer::impl$3::update_buffer_bytes", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::buffer::impl$3::update_buffer_bytes::closure_env$0 <- rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$3::update_buffer_bytes::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$3::update_buffer_bytes::closure_env$0> @ 0x118cd8a -> 0x118bf30 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$3::update_buffer_bytes::closure_env$0> @ 0x118ce77 -> 0x118bf30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$3::update_buffer_bytes::closure$0::closure$0
// 信息 ID: f6f223a28be282178c643b8a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::buffer::impl$3::update_buffer_bytes", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::buffer::impl$3::update_buffer_bytes::closure$0; module=1624, inline_record=2780; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::buffer::impl$3::update_buffer_bytes::closure$0; module=1624, inline_record=2872; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$3::update_buffer_bytes::closure$0::closure$1
// 信息 ID: cd0d8a4231f7efb5e989e6a5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::buffer::impl$3::update_buffer_bytes", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::buffer::impl$3::update_buffer_bytes::closure$0; module=1624, inline_record=2600; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$4::clone
// 信息 ID: 63581c678f1507ceff2aebfe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 28 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::calc_dof; module=400, inline_record=51324; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::calc_dof; module=400, inline_record=51424; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::calc_dof; module=400, inline_record=51524; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass; module=449, inline_record=14540; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass; module=449, inline_record=15140; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass; module=449, inline_record=17716; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass; module=449, inline_record=17812; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass; module=449, inline_record=18244; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$6::fmt
// 信息 ID: bd024af38f99d33305351d0c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$7::hash
// 信息 ID: ac58730e5488162490945a73
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<rhapsody::raw::buffer::BufferDesc> >; module=1611, inline_record=15132; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_buffer
// 信息 ID: 4a3155febdb84c654806ab39
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 92 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::temporal::impl$12::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0xda7ac -> 0x739640 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$12::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0x5ab1ac -> 0x739640 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$4::call_once<tuple$<tuple$<ref$<alloc::string::String>,ref$<enum2$<country_core::render::mesh::VertexAttributeValues> > > >,country_core::render::vao::build_vao::closure_env$0> @ 0x5dafc5 -> 0x739640 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::vao::build_vao @ 0x5dc84f -> 0x739640 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::dynamic_constants::DynamicConstants::allocate_buffer @ 0x73447c -> 0x739640 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::new @ 0x73620c -> 0x739640 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::geometry::instanced_wall::BrickDrawListSsbo> @ 0xf7d387 -> 0x739640 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo> @ 0xf7d548 -> 0x739640 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::compute_garden::FlowerSsbo> @ 0xf7d7f7 -> 0x739640 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo> @ 0xf7da33 -> 0x739640 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::geometry::instanced_wall::BrickNextVisibilityBitmaskSsbo> @ 0xf7dd73 -> 0x739640 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer> @ 0xf7e0da -> 0x739640 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::resources::rocky_terrain::RockyTerrainTriangleSSBO> @ 0xf7e417 -> 0x739640 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::stairs::StairsTransformSsbo> @ 0xf7e647 -> 0x739640 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::resources::render::spatial_resolve_offsets::SpatialResolveOffsetsSsbo> @ 0xf7e8be -> 0x739640 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::grass::GrassBladeDataSsbo> @ 0xf7ec37 -> 0x739640 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_buffer_impl
// 信息 ID: f7775755e11285a6697d2b62
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 6 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_mapped_buffer @ 0x739511 -> 0x738ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer @ 0x7396d3 -> 0x738ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer @ 0x739897 -> 0x738ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_buffer_bytes @ 0x739fe8 -> 0x738ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0x73ebdd -> 0x738ca0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_texture_impl @ 0x744643 -> 0x738ca0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_mapped_buffer
// 信息 ID: fd62d41ea43d98ae167814f1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x5d305 -> 0x739490 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0x5d3b3 -> 0x739490 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x15b993 -> 0x739490 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_screenshot::create_graphics_resources @ 0x3c1f40 -> 0x739490 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::get_vk_buffer
// 信息 ID: c9fc31596b2587722f2f499a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 8 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::bind_external_storage_buffer @ 0x53f54c -> 0x73a3b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_external_storage_buffer @ 0x6ab912 -> 0x73a3b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::fill_unsorted_instance_indirection::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0xdc92ad -> 0x73a3b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::allocate_draw_lists::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0xdc998e -> 0x73a3b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::group_draw_cmd_prefix_sum::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0xdc9b6e -> 0x73a3b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::fill_sorted_instance_list::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0xdca1ef -> 0x73a3b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_draw_lists::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0xdca824 -> 0x73a3b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::group_draw_cmd_prefix_sum::closure_env$1<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0xdcaa0e -> 0x73a3b0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_buffer_binding; module=565, inline_record=2488; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_and_storage_buffer_binding; module=565, inline_record=2872; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_buffer_binding_and_data; module=565, inline_record=3256; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::executor::GraphExecutor::end_top_level_timestamp_query; module=567, inline_record=12956; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::set_buffer_debug_name
// 信息 ID: 00a7d3259b33fb257faedfc3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::update_buffer_bytes
// 信息 ID: 67fe379ad0841745a655b13a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 33 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::app_systems::walls_on_glade_unload @ 0x3bde75 -> 0x739e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::app_systems::walls_on_glade_unload @ 0x3be061 -> 0x739e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::app_systems::walls_on_glade_unload @ 0x3be246 -> 0x739e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::rt::upload_rt_world @ 0x5ffe13 -> 0x739e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::rt::upload_rt_world @ 0x5fff0f -> 0x739e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::rt::upload_rt_world @ 0x600015 -> 0x739e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings @ 0x613238 -> 0x739e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf @ 0x6546e3 -> 0x739e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf @ 0x654917 -> 0x739e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments @ 0x65519d -> 0x739e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::clear_canvas::clear_canvas @ 0x67da54 -> 0x739e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::clear_canvas::clear_canvas @ 0x67dba3 -> 0x739e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::clear_canvas::clear_canvas @ 0x67dcef -> 0x739e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::clearing::init_clearing @ 0x6c9207 -> 0x739e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_text::TextRenderer::consume<toolbar::text::flush_text_render::closure_env$0> @ 0xa5c907 -> 0x739e50 (direct_call); 同地址别名未裁定。
// [静态直接引用] tg_text::TextRenderer::consume<toolbar::text::flush_text_render::closure_env$0> @ 0xa5c973 -> 0x739e50 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
