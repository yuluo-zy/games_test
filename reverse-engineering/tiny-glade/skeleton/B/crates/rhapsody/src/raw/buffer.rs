// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/raw/buffer.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 032c586104bd9d0508e75fb0cdc7c842ea1aa7abd275b1385d433f91667f6a8b
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: buffer.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 125, 245, 521, 526, 529, 534, 541, 549, 550, 677, 678, 695, 699, 703, 705, 707, 1629, 1657, 1741, 1917, 1947, 1951, 3023, 4796
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
// use gpu_allocator; // namespace candidate
//   package identities: [{"name":"gpu-allocator","source":"local_or_path","version":"0.26.0"}]
// use lock_api; // namespace candidate
//   package identities: [{"name":"lock_api","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.9"}]
// use parking_lot; // namespace candidate
//   package identities: [{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.11.2"},{"name":"parking_lot","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.1"}]
// use parking_lot_core; // namespace candidate
//   package identities: [{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.8.6"},{"name":"parking_lot_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.9.3"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::impl$7::clone
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 20
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9846
// 首条位置: {"module":677,"inlinee":38982}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] rhapsody::raw::buffer::impl$8::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":677,"record_offset":2868,"parent_record_offset":2716,"ancestor_record_offset":2716,"inlinee":29807,"source_line":66,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":677,"record_offset":2744,"parent_record_offset":2716,"ancestor_record_offset":2716,"inlinee":9614,"source_line":256,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":677,"record_offset":2804,"parent_record_offset":2768,"ancestor_record_offset":2716,"inlinee":38986,"source_line":2956,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":677,"record_offset":2828,"parent_record_offset":2804,"ancestor_record_offset":2716,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":677,"record_offset":2768,"parent_record_offset":2744,"ancestor_record_offset":2716,"inlinee":38984,"source_line":109,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::impl$0::as_managed_handle
// 记录形式: compiled_procedure; instances=1
// 源码行: 29
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":677,"record_offset":364,"rva":6806912,"va":5375516032}
// 编译实例 RVAs: 0x67dd80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::Buffer::device_address
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 39
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9850
// 首条位置: {"module":677,"record_offset":488,"rva":12405760,"va":5381114880}
// 编译实例 RVAs: 0xbd4c00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30426691","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381114880,"raw_hex":"e84346d001","size":5,"soft_fail":false,"target_va":5411541776,"va":5381115080,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30366593","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381114880,"raw_hex":"e8815bcf01","size":5,"soft_fail":false,"target_va":5411481622,"va":5381115024,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*1288(%rax)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381114880,"raw_hex":"ff9008050000","size":6,"soft_fail":false,"va":5381114973,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::get_buffer_device_address
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":677,"record_offset":820,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":38973,"source_line":710,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::BufferDeviceAddressInfo::buffer
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":677,"record_offset":792,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":38972,"source_line":27762,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":677,"record_offset":756,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":677,"record_offset":632,"parent_record_offset":488,"ancestor_record_offset":488,"inlinee":11960,"source_line":315,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":677,"record_offset":672,"parent_record_offset":632,"ancestor_record_offset":488,"inlinee":11948,"source_line":14,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":677,"record_offset":724,"parent_record_offset":632,"ancestor_record_offset":488,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":677,"record_offset":700,"parent_record_offset":632,"ancestor_record_offset":488,"inlinee":11950,"source_line":32,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::Buffer::mapped_slice_mut
// 记录形式: compiled_procedure; instances=1
// 源码行: 52
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":677,"record_offset":852,"rva":12405968,"va":5381115088}
// 编译实例 RVAs: 0xbd4cd0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140c02280
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t185685","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381115088,"raw_hex":"e855d50200","size":5,"soft_fail":false,"target_va":5381300864,"va":5381115174,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30426471","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381115088,"raw_hex":"e86745d001","size":5,"soft_fail":false,"target_va":5411541808,"va":5381115332,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30426427","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381115088,"raw_hex":"e83b45d001","size":5,"soft_fail":false,"target_va":5411541776,"va":5381115344,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30366374","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381115088,"raw_hex":"e8a65acf01","size":5,"soft_fail":false,"target_va":5411481622,"va":5381115243,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30427791","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381115088,"raw_hex":"e88f4ad001","size":5,"soft_fail":false,"target_va":5411543104,"va":5381115308,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_allocator::vulkan::Allocation::mapped_ptr
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"gpu-allocator","version":"0.26.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t247445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381115088,"raw_hex":"e895c60300","size":5,"soft_fail":false,"target_va":5381362608,"va":5381115158,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":677,"record_offset":1120,"parent_record_offset":852,"ancestor_record_offset":852,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":677,"record_offset":1344,"parent_record_offset":852,"ancestor_record_offset":852,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::buffer::ManagedBuffer::mapped_slice_mut
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/buffer.rs
//     evidence={"module":677,"record_offset":1156,"parent_record_offset":852,"ancestor_record_offset":852,"inlinee":38975,"source_line":47,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":677,"record_offset":996,"parent_record_offset":852,"ancestor_record_offset":852,"inlinee":11960,"source_line":315,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":677,"record_offset":1216,"parent_record_offset":1188,"ancestor_record_offset":852,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":677,"record_offset":1284,"parent_record_offset":1156,"ancestor_record_offset":852,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":677,"record_offset":1244,"parent_record_offset":1188,"ancestor_record_offset":852,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":677,"record_offset":1372,"parent_record_offset":1344,"ancestor_record_offset":852,"inlinee":5308,"source_line":444,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::buffer::ManagedBuffer::allocation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/buffer.rs
//     evidence={"module":677,"record_offset":1188,"parent_record_offset":1156,"ancestor_record_offset":852,"inlinee":38976,"source_line":39,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::buffer::impl$1::mapped_slice_mut::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/buffer.rs
//     evidence={"module":677,"record_offset":1308,"parent_record_offset":1284,"ancestor_record_offset":852,"inlinee":38978,"source_line":50,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":677,"record_offset":1036,"parent_record_offset":996,"ancestor_record_offset":852,"inlinee":11948,"source_line":14,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":677,"record_offset":1088,"parent_record_offset":996,"ancestor_record_offset":852,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":677,"record_offset":1064,"parent_record_offset":996,"ancestor_record_offset":852,"inlinee":11950,"source_line":32,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::impl$10::fmt
// 记录形式: compiled_procedure; instances=2
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":125,"record_offset":22844,"rva":3220576,"va":5371929696}
// 编译实例 RVAs: 0x312460, 0x15a6090
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::fmt::Formatter::debug_struct_field4_finish
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t10963321","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5371929696,"raw_hex":"e87949a700","size":5,"soft_fail":false,"target_va":5382893232,"va":5371929906,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::impl$11::hash
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1531D
// 首条位置: {"module":1947,"inlinee":86813}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::bitflags::impl$282::hash
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":1947,"record_offset":11228,"parent_record_offset":11052,"ancestor_record_offset":11052,"inlinee":86815,"source_line":114,"archive":"_pdb/B/modules/01947.jsonl.gz"}
//   [pdb_inline_direct] core::hash::impls::impl$10::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1947,"record_offset":11592,"parent_record_offset":11052,"ancestor_record_offset":11052,"inlinee":17376,"source_line":811,"archive":"_pdb/B/modules/01947.jsonl.gz"}
//   [pdb_inline_direct] core::hash::impls::impl$13::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1947,"record_offset":11088,"parent_record_offset":11052,"ancestor_record_offset":11052,"inlinee":7683,"source_line":811,"archive":"_pdb/B/modules/01947.jsonl.gz"}
//   [pdb_inline_direct] gpu_allocator::impl$9::hash
//     owner=confirmed_vendored_dependency; count=1
//     packages=[{"name":"gpu-allocator","version":"0.26.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/gpu-allocator/src/lib.rs
//     evidence={"module":1947,"record_offset":11396,"parent_record_offset":11052,"ancestor_record_offset":11052,"inlinee":86817,"source_line":223,"archive":"_pdb/B/modules/01947.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_isize
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1947,"record_offset":11444,"parent_record_offset":11420,"ancestor_record_offset":11052,"inlinee":10581,"source_line":430,"archive":"_pdb/B/modules/01947.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u16
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1947,"record_offset":11616,"parent_record_offset":11592,"ancestor_record_offset":11052,"inlinee":56116,"source_line":369,"archive":"_pdb/B/modules/01947.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_u32
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1947,"record_offset":11276,"parent_record_offset":11252,"ancestor_record_offset":11052,"inlinee":8974,"source_line":375,"archive":"_pdb/B/modules/01947.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::Hasher::write_usize
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1947,"record_offset":11112,"parent_record_offset":11088,"ancestor_record_offset":11052,"inlinee":15756,"source_line":393,"archive":"_pdb/B/modules/01947.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$11::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1947,"record_offset":11252,"parent_record_offset":11228,"ancestor_record_offset":11052,"inlinee":8973,"source_line":811,"archive":"_pdb/B/modules/01947.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::impls::impl$18::hash
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\mod.rs
//     evidence={"module":1947,"record_offset":11420,"parent_record_offset":11396,"ancestor_record_offset":11052,"inlinee":12339,"source_line":811,"archive":"_pdb/B/modules/01947.jsonl.gz"}
//   [pdb_inline_transitive] core::hash::sip::impl$4::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\hash\sip.rs
//     evidence={"module":1947,"record_offset":11188,"parent_record_offset":11164,"ancestor_record_offset":11052,"inlinee":8969,"source_line":231,"archive":"_pdb/B/modules/01947.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::to_ne_bytes
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1947,"record_offset":11136,"parent_record_offset":11112,"ancestor_record_offset":11052,"inlinee":15757,"source_line":3479,"archive":"_pdb/B/modules/01947.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$7::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1947,"record_offset":11640,"parent_record_offset":11616,"ancestor_record_offset":11052,"inlinee":56117,"source_line":3479,"archive":"_pdb/B/modules/01947.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$8::to_ne_bytes
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":1947,"record_offset":11300,"parent_record_offset":11276,"ancestor_record_offset":11052,"inlinee":8975,"source_line":3479,"archive":"_pdb/B/modules/01947.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$4::write
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\std\src\hash\random.rs
//     evidence={"module":1947,"record_offset":11164,"parent_record_offset":11112,"ancestor_record_offset":11052,"inlinee":8977,"source_line":132,"archive":"_pdb/B/modules/01947.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::impl$14::eq
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9A07
// 首条位置: {"module":695,"inlinee":39431}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::vk::bitflags::impl$278::eq
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/bitflags.rs
//     evidence={"module":695,"record_offset":5580,"parent_record_offset":5548,"ancestor_record_offset":5548,"inlinee":39433,"source_line":114,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   [pdb_inline_direct] gpu_allocator::impl$7::eq
//     owner=confirmed_vendored_dependency; count=4
//     packages=[{"name":"gpu-allocator","version":"0.26.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/gpu-allocator/src/lib.rs
//     evidence={"module":695,"record_offset":5608,"parent_record_offset":5548,"ancestor_record_offset":5548,"inlinee":39435,"source_line":223,"archive":"_pdb/B/modules/00695.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::impl$8::clone
// 记录形式: inline_instance, inlinee_declaration; instances=42
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x746F
// 首条位置: {"module":521,"inlinee":29807}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::BufferDesc::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":677,"record_offset":1412,"rva":12406240,"va":5381115360}
// 编译实例 RVAs: 0xbd4de0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30426646","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381115360,"raw_hex":"e81646d001","size":5,"soft_fail":false,"target_va":5411542064,"va":5381115413,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::BufferDesc::new_gpu_only
// 记录形式: compiled_procedure, inlinee_declaration; instances=2
// 源码行: 86
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9854
// 首条位置: {"module":677,"record_offset":1528,"rva":12406304,"va":5381115424}
// 编译实例 RVAs: 0xbd4e20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30426582","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381115424,"raw_hex":"e8d645d001","size":5,"soft_fail":false,"target_va":5411542064,"va":5381115477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu
// 记录形式: compiled_procedure; instances=1
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":677,"record_offset":1652,"rva":12406368,"va":5381115488}
// 编译实例 RVAs: 0xbd4e60
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30426518","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381115488,"raw_hex":"e89645d001","size":5,"soft_fail":false,"target_va":5411542064,"va":5381115541,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu_low_prio
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=6
// 源码行: 108
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9856
// 首条位置: {"module":677,"record_offset":1780,"rva":12406432,"va":5381115552}
// 编译实例 RVAs: 0xbd4ea0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30426454","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381115552,"raw_hex":"e85645d001","size":5,"soft_fail":false,"target_va":5411542064,"va":5381115605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::BufferDesc::new_gpu_to_cpu
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 119
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9857
// 首条位置: {"module":677,"record_offset":1916,"rva":12406496,"va":5381115616}
// 编译实例 RVAs: 0xbd4ee0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30426390","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381115616,"raw_hex":"e81645d001","size":5,"soft_fail":false,"target_va":5411542064,"va":5381115669,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::BufferDesc::alignment
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 130
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9858
// 首条位置: {"module":677,"record_offset":2044,"rva":12406560,"va":5381115680}
// 编译实例 RVAs: 0xbd4f20
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30427105","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381115680,"raw_hex":"e8e147d001","size":5,"soft_fail":false,"target_va":5411542880,"va":5381115770,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::convert::impl$6::try_into
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":677,"record_offset":2176,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":9618,"source_line":799,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":677,"record_offset":2224,"parent_record_offset":2044,"ancestor_record_offset":2044,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::ptr_try_from_impls::impl$1::try_from
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\num.rs
//     evidence={"module":677,"record_offset":2196,"parent_record_offset":2176,"ancestor_record_offset":2044,"inlinee":38980,"source_line":255,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::impl$3::byte_count
// 记录形式: compiled_procedure; instances=1
// 源码行: 143
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":677,"record_offset":2260,"rva":6842544,"va":5375551664}
// 编译实例 RVAs: 0x6868b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::impl$3::copy_to_slice
// 记录形式: compiled_procedure; instances=1
// 源码行: 146
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":677,"record_offset":2380,"rva":12406656,"va":5381115776}
// 编译实例 RVAs: 0xbd4f80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::slice::_$LT$impl$u20$$u5b$T$u5d$$GT$::copy_from_slice::len_mismatch_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30427439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381115776,"raw_hex":"e82f49d001","size":5,"soft_fail":false,"target_va":5411543264,"va":5381115820,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t30731361","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5381115776,"raw_hex":"e961ecd401","size":5,"soft_fail":false,"target_va":5411847168,"va":5381115802,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":677,"record_offset":2508,"parent_record_offset":2380,"ancestor_record_offset":2380,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":677,"record_offset":2540,"parent_record_offset":2508,"ancestor_record_offset":2380,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00677.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::impl$4::byte_count
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=5
// 源码行: 152
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9869
// 首条位置: {"module":678,"record_offset":256,"rva":12407200,"va":5381116320}
// 编译实例 RVAs: 0xbd51a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::map_or
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":678,"record_offset":384,"parent_record_offset":256,"ancestor_record_offset":256,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/00678.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::impl$4::copy_to_slice
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 156
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x986A
// 首条位置: {"module":678,"record_offset":416,"rva":12407216,"va":5381116336}
// 编译实例 RVAs: 0xbd51b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::slice::_$LT$impl$u20$$u5b$T$u5d$$GT$::copy_from_slice::len_mismatch_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30426865","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381116336,"raw_hex":"e8f146d001","size":5,"soft_fail":false,"target_va":5411543264,"va":5381116394,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t30730796","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5381116336,"raw_hex":"e92cead401","size":5,"soft_fail":false,"target_va":5411847168,"va":5381116367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":678,"record_offset":544,"parent_record_offset":416,"ancestor_record_offset":416,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/00678.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":678,"record_offset":576,"parent_record_offset":544,"ancestor_record_offset":416,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00678.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::create_buffer_impl
// 记录形式: compiled_procedure; instances=1
// 源码行: 172
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":8124,"rva":12509696,"va":5381218816}
// 编译实例 RVAs: 0xbee200
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11062840","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381218816,"raw_hex":"e8c83157ff","size":5,"soft_fail":false,"target_va":5370158128,"va":5381220963,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6137966","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381218816,"raw_hex":"e89257a2ff","size":5,"soft_fail":false,"target_va":5375081488,"va":5381219449,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6137650","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381218816,"raw_hex":"e8ce58a2ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381219149,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1406868b0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5668118","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381218816,"raw_hex":"e8ea82a9ff","size":5,"soft_fail":false,"target_va":5375551664,"va":5381219777,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bdf610
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-62498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381218816,"raw_hex":"e8de0bffff","size":5,"soft_fail":false,"target_va":5381158416,"va":5381220909,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d797d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1619201","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381218816,"raw_hex":"e801b51800","size":5,"soft_fail":false,"target_va":5382838224,"va":5381219018,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418ae8a0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13369860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381218816,"raw_hex":"e80402cc00","size":5,"soft_fail":false,"target_va":5394589856,"va":5381219991,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30627612","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381218816,"raw_hex":"e81c57d301","size":5,"soft_fail":false,"target_va":5411847168,"va":5381219551,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30320416","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381218816,"raw_hex":"e820a7ce01","size":5,"soft_fail":false,"target_va":5411540963,"va":5381220542,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=4
//     evidence={"assembly":"\tcallq\t30322557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381218816,"raw_hex":"e87dafce01","size":5,"soft_fail":false,"target_va":5411542880,"va":5381220318,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30322847","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381218816,"raw_hex":"e89fb0ce01","size":5,"soft_fail":false,"target_va":5411543104,"va":5381220252,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_allocator::vulkan::Allocation::mapped_slice_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"gpu-allocator","version":"0.26.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t142811","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381218816,"raw_hex":"e8db2d0200","size":5,"soft_fail":false,"target_va":5381362640,"va":5381219824,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_allocator::vulkan::Allocation::memory
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"gpu-allocator","version":"0.26.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-5363968","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381218816,"raw_hex":"e80027aeff","size":5,"soft_fail":false,"target_va":5375855792,"va":5381219755,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30272160","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381218816,"raw_hex":"e8a0eacd01","size":5,"soft_fail":false,"target_va":5411492368,"va":5381220203,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30274570","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381218816,"raw_hex":"e80af4cd01","size":5,"soft_fail":false,"target_va":5411494800,"va":5381220225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t604344","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381218816,"raw_hex":"e8b8380900","size":5,"soft_fail":false,"target_va":5381823600,"va":5381219251,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t604297","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381218816,"raw_hex":"e889380900","size":5,"soft_fail":false,"target_va":5381824016,"va":5381219714,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::allocator::GpuAllocator::allocate
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t128576","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381218816,"raw_hex":"e840f60100","size":5,"soft_fail":false,"target_va":5381347904,"va":5381219323,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=4
//     evidence={"assembly":"\tcallq\t*264(%r13)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381218816,"raw_hex":"41ff9508010000","size":7,"soft_fail":false,"va":5381218956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::ffi::c_str::CString::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\ffi\c_str.rs
//     evidence={"module":703,"record_offset":8452,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":7851,"source_line":257,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::bind_buffer_memory
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":703,"record_offset":10604,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":39597,"source_line":2702,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::create_buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":703,"record_offset":8336,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":39592,"source_line":2202,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::device::Device::get_buffer_memory_requirements
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":703,"record_offset":9076,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":39593,"source_line":2636,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::extensions_generated::ext::debug_utils::Device::set_debug_utils_object_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/extensions/ext/debug_utils.rs
//     evidence={"module":703,"record_offset":8784,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":15825,"source_line":11,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::BufferUsageFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":703,"record_offset":9132,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":39594,"source_line":32,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DebugUtilsObjectNameInfoEXT::object_handle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":8756,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":15823,"source_line":19604,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":703,"record_offset":9104,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":10728,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":8904,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":8868,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":9372,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":4235,"source_line":853,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":8480,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":10376,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":10944,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":703,"record_offset":9160,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$14::deref_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":703,"record_offset":9316,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":10006,"source_line":646,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::ManagedResources::create_buffer
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/buffer.rs
//     evidence={"module":703,"record_offset":10848,"parent_record_offset":8124,"ancestor_record_offset":8124,"inlinee":39598,"source_line":58,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":9956,"parent_record_offset":9932,"ancestor_record_offset":8124,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":9980,"parent_record_offset":9956,"ancestor_record_offset":8124,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":8700,"parent_record_offset":8676,"ancestor_record_offset":8124,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":9932,"parent_record_offset":9756,"ancestor_record_offset":8124,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":8676,"parent_record_offset":8620,"ancestor_record_offset":8124,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":703,"record_offset":8980,"parent_record_offset":8956,"ancestor_record_offset":8124,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::impl$2::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\ffi\c_str.rs
//     evidence={"module":703,"record_offset":8928,"parent_record_offset":8904,"ancestor_record_offset":8124,"inlinee":7853,"source_line":696,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":9696,"parent_record_offset":9668,"ancestor_record_offset":8124,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":8648,"parent_record_offset":8620,"ancestor_record_offset":8124,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":8620,"parent_record_offset":8596,"ancestor_record_offset":8124,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":9756,"parent_record_offset":9724,"ancestor_record_offset":8124,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":9724,"parent_record_offset":9696,"ancestor_record_offset":8124,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":8596,"parent_record_offset":8572,"ancestor_record_offset":8124,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":9792,"parent_record_offset":9756,"ancestor_record_offset":8124,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":703,"record_offset":9632,"parent_record_offset":9600,"ancestor_record_offset":8124,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":703,"record_offset":9600,"parent_record_offset":9568,"ancestor_record_offset":8124,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":703,"record_offset":9536,"parent_record_offset":9504,"ancestor_record_offset":8124,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":703,"record_offset":9568,"parent_record_offset":9536,"ancestor_record_offset":8124,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":703,"record_offset":9504,"parent_record_offset":9472,"ancestor_record_offset":8124,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":703,"record_offset":9472,"parent_record_offset":9420,"ancestor_record_offset":8124,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":9668,"parent_record_offset":9632,"ancestor_record_offset":8124,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::assume_init_on_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":703,"record_offset":8364,"parent_record_offset":8336,"ancestor_record_offset":8124,"inlinee":15819,"source_line":24,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":703,"record_offset":8388,"parent_record_offset":8364,"ancestor_record_offset":8124,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":703,"record_offset":8412,"parent_record_offset":8388,"ancestor_record_offset":8124,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":703,"record_offset":9816,"parent_record_offset":9792,"ancestor_record_offset":8124,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":703,"record_offset":9840,"parent_record_offset":9816,"ancestor_record_offset":8124,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":9340,"parent_record_offset":9316,"ancestor_record_offset":8124,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":703,"record_offset":10056,"parent_record_offset":10032,"ancestor_record_offset":8124,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":703,"record_offset":9888,"parent_record_offset":9864,"ancestor_record_offset":8124,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":703,"record_offset":9864,"parent_record_offset":9840,"ancestor_record_offset":8124,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":703,"record_offset":10032,"parent_record_offset":9632,"ancestor_record_offset":8124,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":8524,"parent_record_offset":8480,"ancestor_record_offset":8124,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":10784,"parent_record_offset":10760,"ancestor_record_offset":8124,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":10808,"parent_record_offset":10784,"ancestor_record_offset":8124,"inlinee":5308,"source_line":444,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":10536,"parent_record_offset":10472,"ancestor_record_offset":8124,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":9224,"parent_record_offset":9188,"ancestor_record_offset":8124,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":10560,"parent_record_offset":10536,"ancestor_record_offset":8124,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":9248,"parent_record_offset":9224,"ancestor_record_offset":8124,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":703,"record_offset":10444,"parent_record_offset":10416,"ancestor_record_offset":8124,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":703,"record_offset":9188,"parent_record_offset":9160,"ancestor_record_offset":8124,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":703,"record_offset":10472,"parent_record_offset":10444,"ancestor_record_offset":8124,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":703,"record_offset":9280,"parent_record_offset":9188,"ancestor_record_offset":8124,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":703,"record_offset":10508,"parent_record_offset":10472,"ancestor_record_offset":8124,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::impl$6::create_buffer_impl::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":703,"record_offset":9420,"parent_record_offset":9372,"ancestor_record_offset":8124,"inlinee":39596,"source_line":239,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::impl$6::create_buffer_impl::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":703,"record_offset":10760,"parent_record_offset":10728,"ancestor_record_offset":8124,"inlinee":39601,"source_line":255,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$4::create
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":703,"record_offset":10872,"parent_record_offset":10848,"ancestor_record_offset":8124,"inlinee":15367,"source_line":298,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$7::allocate
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":703,"record_offset":10908,"parent_record_offset":10872,"ancestor_record_offset":8124,"inlinee":39600,"source_line":379,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::impl$6::create_buffer_impl::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 239
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9AAC
// 首条位置: {"module":703,"inlinee":39596}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":703,"record_offset":9472,"parent_record_offset":9420,"ancestor_record_offset":9420,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":10112,"parent_record_offset":9420,"ancestor_record_offset":9420,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":9956,"parent_record_offset":9932,"ancestor_record_offset":9420,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":9980,"parent_record_offset":9956,"ancestor_record_offset":9420,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":10312,"parent_record_offset":10288,"ancestor_record_offset":9420,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":9932,"parent_record_offset":9756,"ancestor_record_offset":9420,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":10288,"parent_record_offset":10232,"ancestor_record_offset":9420,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":9696,"parent_record_offset":9668,"ancestor_record_offset":9420,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":10260,"parent_record_offset":10232,"ancestor_record_offset":9420,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":10232,"parent_record_offset":10208,"ancestor_record_offset":9420,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":9756,"parent_record_offset":9724,"ancestor_record_offset":9420,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":9724,"parent_record_offset":9696,"ancestor_record_offset":9420,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":10208,"parent_record_offset":10184,"ancestor_record_offset":9420,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":9792,"parent_record_offset":9756,"ancestor_record_offset":9420,"inlinee":4258,"source_line":816,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":703,"record_offset":9632,"parent_record_offset":9600,"ancestor_record_offset":9420,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":703,"record_offset":9600,"parent_record_offset":9568,"ancestor_record_offset":9420,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":703,"record_offset":9536,"parent_record_offset":9504,"ancestor_record_offset":9420,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":703,"record_offset":9568,"parent_record_offset":9536,"ancestor_record_offset":9420,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":703,"record_offset":9504,"parent_record_offset":9472,"ancestor_record_offset":9420,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":703,"record_offset":9668,"parent_record_offset":9632,"ancestor_record_offset":9420,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":703,"record_offset":9816,"parent_record_offset":9792,"ancestor_record_offset":9420,"inlinee":4260,"source_line":373,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":703,"record_offset":9840,"parent_record_offset":9816,"ancestor_record_offset":9420,"inlinee":4261,"source_line":461,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":703,"record_offset":10056,"parent_record_offset":10032,"ancestor_record_offset":9420,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":703,"record_offset":9888,"parent_record_offset":9864,"ancestor_record_offset":9420,"inlinee":4265,"source_line":1520,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":703,"record_offset":9864,"parent_record_offset":9840,"ancestor_record_offset":9420,"inlinee":4263,"source_line":870,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":703,"record_offset":10032,"parent_record_offset":9632,"ancestor_record_offset":9420,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":10136,"parent_record_offset":10112,"ancestor_record_offset":9420,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::impl$6::create_buffer_impl::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 255
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9AB1
// 首条位置: {"module":703,"inlinee":39601}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":10784,"parent_record_offset":10760,"ancestor_record_offset":10760,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":10808,"parent_record_offset":10784,"ancestor_record_offset":10760,"inlinee":5308,"source_line":444,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::impl$6::create_buffer_impl::closure$2
// 记录形式: inlinee_declaration; instances=1
// 源码行: 260
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x1532C
// 首条位置: {"module":1951,"inlinee":86828}
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
// 源码行: 265
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":11364,"rva":12511872,"va":5381220992}
// 编译实例 RVAs: 0xbeea80
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-51051","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381220992,"raw_hex":"e89538ffff","size":5,"soft_fail":false,"target_va":5381170352,"va":5381221398,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bee200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-2310","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381220992,"raw_hex":"e8faf6ffff","size":5,"soft_fail":false,"target_va":5381218816,"va":5381221121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30320439","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381220992,"raw_hex":"e837a7ce01","size":5,"soft_fail":false,"target_va":5411541808,"va":5381221364,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30320719","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381220992,"raw_hex":"e84fa8ce01","size":5,"soft_fail":false,"target_va":5411542064,"va":5381221340,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":703,"record_offset":11588,"parent_record_offset":11364,"ancestor_record_offset":11364,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":11680,"parent_record_offset":11364,"ancestor_record_offset":11364,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":11744,"parent_record_offset":11364,"ancestor_record_offset":11364,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":11616,"parent_record_offset":11364,"ancestor_record_offset":11364,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":11716,"parent_record_offset":11364,"ancestor_record_offset":11364,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::debug_utils
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":703,"record_offset":11532,"parent_record_offset":11364,"ancestor_record_offset":11364,"inlinee":39602,"source_line":682,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::ManagedRef::into_owner
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":703,"record_offset":11652,"parent_record_offset":11364,"ancestor_record_offset":11364,"inlinee":13553,"source_line":310,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":11556,"parent_record_offset":11532,"ancestor_record_offset":11364,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":11768,"parent_record_offset":11744,"ancestor_record_offset":11364,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > >
// 记录形式: compiled_procedure; instances=5
// 源码行: 287
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":245,"record_offset":26292,"rva":6001584,"va":5374710704}
// 编译实例 RVAs: 0x5b93b0, 0xc03460, 0x16a2dc0, 0x1fdca60, 0x288e990
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd51a0
//     owner=unresolved_namespace; count=20
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11328627","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392444864,"raw_hex":"e88d2353ff","size":5,"soft_fail":false,"target_va":5381116320,"va":5392444942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd51b0
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11328883","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392444864,"raw_hex":"e88d2253ff","size":5,"soft_fail":false,"target_va":5381116336,"va":5392445214,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=25
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11275282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392444864,"raw_hex":"e8eef353ff","size":5,"soft_fail":false,"target_va":5381170352,"va":5392445629,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bee200
//     owner=unresolved_namespace; count=10
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11226219","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392444864,"raw_hex":"e895b354ff","size":5,"soft_fail":false,"target_va":5381218816,"va":5392445030,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c04840
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11135087","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392444864,"raw_hex":"e8911756ff","size":5,"soft_fail":false,"target_va":5381310528,"va":5392445610,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30541397","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381305440,"raw_hex":"e85506d201","size":5,"soft_fail":false,"target_va":5411847168,"va":5381305766,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t19095956","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392444864,"raw_hex":"e894612301","size":5,"soft_fail":false,"target_va":5411541808,"va":5392445847,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=5
//     evidence={"assembly":"\tcallq\t19096278","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392444864,"raw_hex":"e8d6622301","size":5,"soft_fail":false,"target_va":5411542064,"va":5392445781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t19097298","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392444864,"raw_hex":"e8d2662301","size":5,"soft_fail":false,"target_va":5411543104,"va":5392445801,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu_low_prio
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11329796","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392444864,"raw_hex":"e8fc1e53ff","size":5,"soft_fail":false,"target_va":5381115552,"va":5392445343,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::debug_utils
//     owner=package_namespace_candidate; count=8
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11183305","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392444864,"raw_hex":"e8375b55ff","size":5,"soft_fail":false,"target_va":5381261664,"va":5392444964,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":245,"record_offset":26540,"parent_record_offset":26292,"ancestor_record_offset":26292,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::impl$286::bitor_assign
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":245,"record_offset":26512,"parent_record_offset":26292,"ancestor_record_offset":26292,"inlinee":15804,"source_line":45,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":245,"record_offset":26792,"parent_record_offset":26292,"ancestor_record_offset":26292,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=25
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":245,"record_offset":26932,"parent_record_offset":26292,"ancestor_record_offset":26292,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":245,"record_offset":26568,"parent_record_offset":26292,"ancestor_record_offset":26292,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":245,"record_offset":26700,"parent_record_offset":26292,"ancestor_record_offset":26292,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":245,"record_offset":26604,"parent_record_offset":26292,"ancestor_record_offset":26292,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu_low_prio
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":707,"record_offset":6136,"parent_record_offset":5488,"ancestor_record_offset":5488,"inlinee":38998,"source_line":108,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::impl$4::byte_count
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":707,"record_offset":5724,"parent_record_offset":5488,"ancestor_record_offset":5488,"inlinee":39017,"source_line":152,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::impl$4::copy_to_slice
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":707,"record_offset":5996,"parent_record_offset":5488,"ancestor_record_offset":5488,"inlinee":39018,"source_line":156,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::debug_utils
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":707,"record_offset":5808,"parent_record_offset":5488,"ancestor_record_offset":5488,"inlinee":39602,"source_line":682,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::ManagedRef::into_owner
//     owner=private_source_ownership_candidate; count=5
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":245,"record_offset":26672,"parent_record_offset":26292,"ancestor_record_offset":26292,"inlinee":13553,"source_line":310,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":707,"record_offset":6044,"parent_record_offset":6020,"ancestor_record_offset":5488,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":707,"record_offset":5832,"parent_record_offset":5808,"ancestor_record_offset":5488,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":707,"record_offset":5748,"parent_record_offset":5724,"ancestor_record_offset":5488,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=75
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":245,"record_offset":26956,"parent_record_offset":26932,"ancestor_record_offset":26292,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":707,"record_offset":6020,"parent_record_offset":5996,"ancestor_record_offset":5488,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=10
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":245,"record_offset":26632,"parent_record_offset":26604,"ancestor_record_offset":26292,"inlinee":5308,"source_line":444,"archive":"_pdb/B/modules/00245.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::create_buffer<ref$<slice2$<u8> > >
// 记录形式: compiled_procedure; instances=1
// 源码行: 287
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1657,"record_offset":5860,"rva":23734624,"va":5392443744}
// 编译实例 RVAs: 0x16a2960
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1406868b0
//     owner=unresolved_namespace; count=5
//     target_aliases=5 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-16892163","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392443744,"raw_hex":"e8fd3efefe","size":5,"soft_fail":false,"target_va":5375551664,"va":5392443822,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bd4f80
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11328323","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392443744,"raw_hex":"e8bd2453ff","size":5,"soft_fail":false,"target_va":5381115776,"va":5392444094,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=5
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11274162","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392443744,"raw_hex":"e84ef853ff","size":5,"soft_fail":false,"target_va":5381170352,"va":5392444509,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bee200
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11225099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392443744,"raw_hex":"e8f5b754ff","size":5,"soft_fail":false,"target_va":5381218816,"va":5392443910,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416a3220
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1489","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392443744,"raw_hex":"e8d1050000","size":5,"soft_fail":false,"target_va":5392445984,"va":5392444490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19097076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392443744,"raw_hex":"e8f4652301","size":5,"soft_fail":false,"target_va":5411541808,"va":5392444727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t19097398","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392443744,"raw_hex":"e836672301","size":5,"soft_fail":false,"target_va":5411542064,"va":5392444661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t19098418","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392443744,"raw_hex":"e8326b2301","size":5,"soft_fail":false,"target_va":5411543104,"va":5392444681,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu_low_prio
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11328676","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392443744,"raw_hex":"e85c2353ff","size":5,"soft_fail":false,"target_va":5381115552,"va":5392444223,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::device::GraphicsDevice::debug_utils
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-11182185","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5392443744,"raw_hex":"e8975f55ff","size":5,"soft_fail":false,"target_va":5381261664,"va":5392443844,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":1657,"record_offset":6076,"parent_record_offset":5860,"ancestor_record_offset":5860,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::impl$286::bitor_assign
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":1657,"record_offset":6048,"parent_record_offset":5860,"ancestor_record_offset":5860,"inlinee":15804,"source_line":45,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1657,"record_offset":6328,"parent_record_offset":5860,"ancestor_record_offset":5860,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":6468,"parent_record_offset":5860,"ancestor_record_offset":5860,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1657,"record_offset":6104,"parent_record_offset":5860,"ancestor_record_offset":5860,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":1657,"record_offset":6236,"parent_record_offset":5860,"ancestor_record_offset":5860,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1657,"record_offset":6140,"parent_record_offset":5860,"ancestor_record_offset":5860,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::ManagedRef::into_owner
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":1657,"record_offset":6208,"parent_record_offset":5860,"ancestor_record_offset":5860,"inlinee":13553,"source_line":310,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1657,"record_offset":6492,"parent_record_offset":6468,"ancestor_record_offset":5860,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1657,"record_offset":6168,"parent_record_offset":6140,"ancestor_record_offset":5860,"inlinee":5308,"source_line":444,"archive":"_pdb/B/modules/01657.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::create_buffer<tuple$<> >
// 记录形式: compiled_procedure; instances=1
// 源码行: 287
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":707,"record_offset":7128,"rva":12597440,"va":5381306560}
// 编译实例 RVAs: 0xc038c0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140bee200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-87864","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381306560,"raw_hex":"e8c8a8feff","size":5,"soft_fail":false,"target_va":5381218816,"va":5381306675,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":707,"record_offset":7348,"parent_record_offset":7128,"ancestor_record_offset":7128,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":707,"record_offset":7376,"parent_record_offset":7128,"ancestor_record_offset":7128,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":707,"record_offset":7432,"parent_record_offset":7128,"ancestor_record_offset":7128,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::debug_utils
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":707,"record_offset":7292,"parent_record_offset":7128,"ancestor_record_offset":7128,"inlinee":39602,"source_line":682,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::ManagedRef::into_owner
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":707,"record_offset":7404,"parent_record_offset":7128,"ancestor_record_offset":7128,"inlinee":13553,"source_line":310,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":707,"record_offset":7316,"parent_record_offset":7292,"ancestor_record_offset":7128,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::impl$6::create_buffer::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=7
// 源码行: 330
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C0B
// 首条位置: {"module":707,"inlinee":39947}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::cmd_copy_buffer
//     owner=registry_dependency_definition; count=4
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":707,"record_offset":15204,"parent_record_offset":14516,"ancestor_record_offset":14516,"inlinee":39940,"source_line":1466,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::impl$4::byte_count
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":707,"record_offset":15148,"parent_record_offset":14516,"ancestor_record_offset":14516,"inlinee":39017,"source_line":152,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::ManagedRef::get_tracked
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":707,"record_offset":14572,"parent_record_offset":14516,"ancestor_record_offset":14516,"inlinee":39610,"source_line":327,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":707,"record_offset":14816,"parent_record_offset":14724,"ancestor_record_offset":14516,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":707,"record_offset":15172,"parent_record_offset":15148,"ancestor_record_offset":14516,"inlinee":5577,"source_line":1173,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU64::fetch_max
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":14760,"parent_record_offset":14724,"ancestor_record_offset":14516,"inlinee":39124,"source_line":3344,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_umax
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":14784,"parent_record_offset":14760,"ancestor_record_offset":14516,"inlinee":29644,"source_line":3962,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::CommandBufferSeqTracker::track
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":707,"record_offset":14724,"parent_record_offset":14636,"ancestor_record_offset":14516,"inlinee":39123,"source_line":54,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$17::track
//     owner=private_source_ownership_candidate; count=8
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":707,"record_offset":14608,"parent_record_offset":14572,"ancestor_record_offset":14516,"inlinee":8709,"source_line":250,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":707,"record_offset":14696,"parent_record_offset":14636,"ancestor_record_offset":14516,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$4::track
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":707,"record_offset":14636,"parent_record_offset":14608,"ancestor_record_offset":14516,"inlinee":39132,"source_line":96,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::ManagedResourceMeta::checksum
//     owner=private_source_ownership_candidate; count=4
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":707,"record_offset":14668,"parent_record_offset":14636,"ancestor_record_offset":14516,"inlinee":39121,"source_line":43,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::set_buffer_debug_name
// 记录形式: compiled_procedure; instances=1
// 源码行: 351
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":11832,"rva":12512304,"va":5381221424}
// 编译实例 RVAs: 0xbeec30
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140161c30
//     owner=unresolved_namespace; count=1
//     target_aliases=47 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11064147","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381221424,"raw_hex":"e8ad2c57ff","size":5,"soft_fail":false,"target_va":5370158128,"va":5381222270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6140151","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381221424,"raw_hex":"e8094fa2ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381221650,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140d797d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1616527","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381221424,"raw_hex":"e88faa1800","size":5,"soft_fail":false,"target_va":5382838224,"va":5381221692,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30319826","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381221424,"raw_hex":"e8d2a4ce01","size":5,"soft_fail":false,"target_va":5411541776,"va":5381221945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30259760","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381221424,"raw_hex":"e830bacd01","size":5,"soft_fail":false,"target_va":5411481622,"va":5381221857,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30320860","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381221424,"raw_hex":"e8dca8ce01","size":5,"soft_fail":false,"target_va":5411542880,"va":5381222015,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] gpu_allocator::vulkan::Allocator::rename_allocation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"gpu-allocator","version":"0.26.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t155926","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381221424,"raw_hex":"e816610200","size":5,"soft_fail":false,"target_va":5381377536,"va":5381221605,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::lock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30270450","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381221424,"raw_hex":"e8f2e3cd01","size":5,"soft_fail":false,"target_va":5411492368,"va":5381221913,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot::raw_mutex::RawMutex::unlock_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot","version":"0.11.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30272867","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381221424,"raw_hex":"e863edcd01","size":5,"soft_fail":false,"target_va":5411494800,"va":5381221928,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::acquire_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t602029","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381221424,"raw_hex":"e8ad2f0900","size":5,"soft_fail":false,"target_va":5381823600,"va":5381221566,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] parking_lot_core::parking_lot::deadlock_impl::release_resource
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"parking_lot_core","version":"0.8.6","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t602349","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381221424,"raw_hex":"e8ed300900","size":5,"soft_fail":false,"target_va":5381824016,"va":5381221662,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*2160(%rsi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381221424,"raw_hex":"ff9670080000","size":6,"soft_fail":false,"va":5381221762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::ffi::c_str::CString::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\ffi\c_str.rs
//     evidence={"module":703,"record_offset":13084,"parent_record_offset":11832,"ancestor_record_offset":11832,"inlinee":7851,"source_line":257,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":703,"record_offset":12248,"parent_record_offset":11832,"ancestor_record_offset":11832,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::extensions_generated::ext::debug_utils::Device::set_debug_utils_object_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/extensions/ext/debug_utils.rs
//     evidence={"module":703,"record_offset":13412,"parent_record_offset":11832,"ancestor_record_offset":11832,"inlinee":15825,"source_line":11,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::definitions::DebugUtilsObjectNameInfoEXT::object_handle
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":703,"record_offset":13384,"parent_record_offset":11832,"ancestor_record_offset":11832,"inlinee":15823,"source_line":19604,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":12212,"parent_record_offset":11832,"ancestor_record_offset":11832,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":12612,"parent_record_offset":11832,"ancestor_record_offset":11832,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":13496,"parent_record_offset":11832,"ancestor_record_offset":11832,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":13112,"parent_record_offset":11832,"ancestor_record_offset":11832,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::Mutex::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":703,"record_offset":12276,"parent_record_offset":11832,"ancestor_record_offset":11832,"inlinee":10002,"source_line":213,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] lock_api::mutex::impl$13::deref
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":703,"record_offset":12528,"parent_record_offset":11832,"ancestor_record_offset":11832,"inlinee":15806,"source_line":639,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::allocator::GpuAllocator::rename_allocation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/allocator.rs
//     evidence={"module":703,"record_offset":12584,"parent_record_offset":11832,"ancestor_record_offset":11832,"inlinee":39604,"source_line":87,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::debug_utils
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":703,"record_offset":12028,"parent_record_offset":11832,"ancestor_record_offset":11832,"inlinee":39602,"source_line":682,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::buffer::ManagedBuffer::allocation
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/buffer.rs
//     evidence={"module":703,"record_offset":12432,"parent_record_offset":11832,"ancestor_record_offset":11832,"inlinee":38976,"source_line":39,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":703,"record_offset":12084,"parent_record_offset":11832,"ancestor_record_offset":11832,"inlinee":11960,"source_line":315,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":12836,"parent_record_offset":12812,"ancestor_record_offset":11832,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":703,"record_offset":12812,"parent_record_offset":12756,"ancestor_record_offset":11832,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":703,"record_offset":13608,"parent_record_offset":13584,"ancestor_record_offset":11832,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::impl$2::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\ffi\c_str.rs
//     evidence={"module":703,"record_offset":13556,"parent_record_offset":13532,"ancestor_record_offset":11832,"inlinee":7853,"source_line":696,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":12784,"parent_record_offset":12756,"ancestor_record_offset":11832,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":12756,"parent_record_offset":12732,"ancestor_record_offset":11832,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":703,"record_offset":12732,"parent_record_offset":12708,"ancestor_record_offset":11832,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":703,"record_offset":13436,"parent_record_offset":13412,"ancestor_record_offset":11832,"inlinee":15820,"source_line":11,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::enums::Result::result_with_success
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/prelude.rs
//     evidence={"module":703,"record_offset":13460,"parent_record_offset":13436,"ancestor_record_offset":11832,"inlinee":15821,"source_line":16,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":703,"record_offset":12552,"parent_record_offset":12528,"ancestor_record_offset":11832,"inlinee":4439,"source_line":2194,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":12052,"parent_record_offset":12028,"ancestor_record_offset":11832,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":12488,"parent_record_offset":12432,"ancestor_record_offset":11832,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":12636,"parent_record_offset":12612,"ancestor_record_offset":11832,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":13016,"parent_record_offset":12952,"ancestor_record_offset":11832,"inlinee":4945,"source_line":2871,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU8::compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":12340,"parent_record_offset":12304,"ancestor_record_offset":11832,"inlinee":10004,"source_line":2920,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":13040,"parent_record_offset":13016,"ancestor_record_offset":11832,"inlinee":4946,"source_line":3793,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_compare_exchange_weak
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":703,"record_offset":12364,"parent_record_offset":12340,"ancestor_record_offset":11832,"inlinee":5825,"source_line":3828,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] lock_api::mutex::impl$15::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"lock_api","version":"0.4.9","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/lock_api-0.4.9/src/mutex.rs
//     evidence={"module":703,"record_offset":12924,"parent_record_offset":12896,"ancestor_record_offset":11832,"inlinee":9996,"source_line":653,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::lock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":703,"record_offset":12304,"parent_record_offset":12276,"ancestor_record_offset":11832,"inlinee":10003,"source_line":66,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot::raw_mutex::impl$0::unlock
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot","version":"0.12.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot-0.12.1/src/raw_mutex.rs
//     evidence={"module":703,"record_offset":12952,"parent_record_offset":12924,"ancestor_record_offset":11832,"inlinee":4941,"source_line":100,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::acquire_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":703,"record_offset":12396,"parent_record_offset":12304,"ancestor_record_offset":11832,"inlinee":5827,"source_line":1104,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] parking_lot_core::parking_lot::deadlock::release_resource
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"parking_lot_core","version":"0.9.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/parking_lot_core-0.9.3/src/parking_lot.rs
//     evidence={"module":703,"record_offset":12988,"parent_record_offset":12952,"ancestor_record_offset":11832,"inlinee":4943,"source_line":1120,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":703,"record_offset":12128,"parent_record_offset":12084,"ancestor_record_offset":11832,"inlinee":11948,"source_line":14,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":703,"record_offset":12180,"parent_record_offset":12084,"ancestor_record_offset":11832,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":703,"record_offset":12156,"parent_record_offset":12084,"ancestor_record_offset":11832,"inlinee":11950,"source_line":32,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::update_buffer_bytes
// 记录形式: compiled_procedure; instances=1
// 源码行: 390
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":703,"record_offset":13880,"rva":12513184,"va":5381222304}
// 编译实例 RVAs: 0xbeefa0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-52761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381222304,"raw_hex":"e8e731ffff","size":5,"soft_fail":false,"target_va":5381170352,"va":5381223108,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140bee200
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3879","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381222304,"raw_hex":"e8d9f0ffff","size":5,"soft_fail":false,"target_va":5381218816,"va":5381222690,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c04280
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t86076","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381222304,"raw_hex":"e83c500100","size":5,"soft_fail":false,"target_va":5381309056,"va":5381222975,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142923c00
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30624295","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381222304,"raw_hex":"e8274ad301","size":5,"soft_fail":false,"target_va":5411847168,"va":5381222868,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30318342","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381222304,"raw_hex":"e8069fce01","size":5,"soft_fail":false,"target_va":5411541808,"va":5381223461,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30318334","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381222304,"raw_hex":"e8fe9ece01","size":5,"soft_fail":false,"target_va":5411541776,"va":5381223437,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30258340","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381222304,"raw_hex":"e8a4b4cd01","size":5,"soft_fail":false,"target_va":5411481622,"va":5381223277,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30319506","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381222304,"raw_hex":"e892a3ce01","size":5,"soft_fail":false,"target_va":5411542064,"va":5381222553,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30318498","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381222304,"raw_hex":"e8a29fce01","size":5,"soft_fail":false,"target_va":5411541904,"va":5381223401,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t30319938","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381222304,"raw_hex":"e842a5ce01","size":5,"soft_fail":false,"target_va":5411543104,"va":5381223161,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30319972","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381222304,"raw_hex":"e864a5ce01","size":5,"soft_fail":false,"target_va":5411543120,"va":5381223143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::sync::impl$29::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\sync.rs
//     evidence={"module":703,"record_offset":14644,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":5345,"source_line":2205,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] ash::vk::bitflags::BufferUsageFlags::contains
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/macros.rs
//     evidence={"module":703,"record_offset":14520,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":39594,"source_line":32,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::fmt::Arguments::new_v1
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\fmt\mod.rs
//     evidence={"module":703,"record_offset":15484,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":4329,"source_line":608,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":14708,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":14212,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":15372,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":14672,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":703,"record_offset":15316,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":703,"record_offset":15056,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":14420,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":14988,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":703,"record_offset":14360,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$1::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":703,"record_offset":14248,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":10157,"source_line":25,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu_low_prio
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":703,"record_offset":14548,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":38998,"source_line":108,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::debug_utils
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":703,"record_offset":14584,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":39602,"source_line":682,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":703,"record_offset":14084,"parent_record_offset":13880,"ancestor_record_offset":13880,"inlinee":11960,"source_line":315,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":703,"record_offset":15080,"parent_record_offset":15056,"ancestor_record_offset":13880,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":703,"record_offset":14484,"parent_record_offset":14448,"ancestor_record_offset":13880,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":703,"record_offset":14612,"parent_record_offset":14584,"ancestor_record_offset":13880,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":703,"record_offset":15396,"parent_record_offset":15372,"ancestor_record_offset":13880,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":703,"record_offset":14320,"parent_record_offset":14296,"ancestor_record_offset":13880,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":703,"record_offset":14388,"parent_record_offset":14360,"ancestor_record_offset":13880,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":703,"record_offset":14272,"parent_record_offset":14248,"ancestor_record_offset":13880,"inlinee":4460,"source_line":1050,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":15232,"parent_record_offset":15208,"ancestor_record_offset":13880,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":14948,"parent_record_offset":14924,"ancestor_record_offset":13880,"inlinee":4164,"source_line":87,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_mut_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":15208,"parent_record_offset":15184,"ancestor_record_offset":13880,"inlinee":5242,"source_line":112,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":14924,"parent_record_offset":14852,"ancestor_record_offset":13880,"inlinee":4612,"source_line":101,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":15184,"parent_record_offset":15160,"ancestor_record_offset":13880,"inlinee":5241,"source_line":413,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":14448,"parent_record_offset":14420,"ancestor_record_offset":13880,"inlinee":4610,"source_line":431,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":15016,"parent_record_offset":14988,"ancestor_record_offset":13880,"inlinee":5308,"source_line":444,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::get_unchecked_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":15160,"parent_record_offset":15136,"ancestor_record_offset":13880,"inlinee":5240,"source_line":553,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":703,"record_offset":15136,"parent_record_offset":15112,"ancestor_record_offset":13880,"inlinee":5239,"source_line":568,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter.rs
//     evidence={"module":703,"record_offset":14296,"parent_record_offset":14272,"ancestor_record_offset":13880,"inlinee":4461,"source_line":96,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":703,"record_offset":14128,"parent_record_offset":14084,"ancestor_record_offset":13880,"inlinee":11948,"source_line":14,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":703,"record_offset":14180,"parent_record_offset":14084,"ancestor_record_offset":13880,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":703,"record_offset":14156,"parent_record_offset":14084,"ancestor_record_offset":13880,"inlinee":11950,"source_line":32,"archive":"_pdb/B/modules/00703.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::impl$6::update_buffer_bytes::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 440
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":707,"record_offset":7464,"rva":12597760,"va":5381306880}
// 编译实例 RVAs: 0xc03a00
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1405e6620
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6412085","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381306880,"raw_hex":"e8cb289eff","size":5,"soft_fail":false,"target_va":5374895648,"va":5381307728,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140613c20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6226036","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381306880,"raw_hex":"e88cffa0ff","size":5,"soft_fail":false,"target_va":5375081504,"va":5381307535,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::Bump$LT$_$GT$::alloc_layout_slow
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30227562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381306880,"raw_hex":"e86a3ccd01","size":5,"soft_fail":false,"target_va":5411534720,"va":5381307153,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::Bump::new
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t1337595","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381306880,"raw_hex":"e8fb681400","size":5,"soft_fail":false,"target_va":5382644656,"va":5381307056,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] bumpalo::oom
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t30227639","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381306880,"raw_hex":"e8b73ccd01","size":5,"soft_fail":false,"target_va":5411535312,"va":5381307668,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30234142","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381306880,"raw_hex":"e81e56cd01","size":5,"soft_fail":false,"target_va":5411541808,"va":5381307661,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30234262","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381306880,"raw_hex":"e89656cd01","size":5,"soft_fail":false,"target_va":5411541904,"va":5381307637,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t*992(%r15)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381306880,"raw_hex":"41ff97e0030000","size":7,"soft_fail":false,"va":5381307367,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] ash::device::Device::cmd_copy_buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":707,"record_offset":9308,"parent_record_offset":7464,"ancestor_record_offset":7464,"inlinee":39940,"source_line":1466,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] bumpalo::Bump::alloc_slice_fill_iter
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":707,"record_offset":8072,"parent_record_offset":7464,"ancestor_record_offset":7464,"inlinee":39934,"source_line":1747,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":707,"record_offset":9396,"parent_record_offset":7464,"ancestor_record_offset":7464,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::impl$6::update_buffer_bytes::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":707,"record_offset":9244,"parent_record_offset":7464,"ancestor_record_offset":7464,"inlinee":39939,"source_line":445,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::ManagedRef::get_tracked
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":707,"record_offset":7836,"parent_record_offset":7464,"ancestor_record_offset":7464,"inlinee":39610,"source_line":327,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$17::track
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":707,"record_offset":7632,"parent_record_offset":7464,"ancestor_record_offset":7464,"inlinee":8709,"source_line":250,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":707,"record_offset":9572,"parent_record_offset":9444,"ancestor_record_offset":7464,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::cmd_pipeline_barrier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":707,"record_offset":9276,"parent_record_offset":9244,"ancestor_record_offset":7464,"inlinee":39588,"source_line":2370,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_layout
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":707,"record_offset":8260,"parent_record_offset":8108,"ancestor_record_offset":7464,"inlinee":15487,"source_line":1864,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::alloc_slice_fill_with
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":707,"record_offset":8108,"parent_record_offset":8072,"ancestor_record_offset":7464,"inlinee":39643,"source_line":1552,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":707,"record_offset":8288,"parent_record_offset":8260,"ancestor_record_offset":7464,"inlinee":15488,"source_line":1879,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::Bump::try_alloc_layout_fast
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":707,"record_offset":8328,"parent_record_offset":8288,"ancestor_record_offset":7464,"inlinee":15489,"source_line":1888,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::ChunkFooter::is_empty
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":707,"record_offset":9480,"parent_record_offset":9444,"ancestor_record_offset":7464,"inlinee":12170,"source_line":375,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::dealloc_chunk_list
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":707,"record_offset":9444,"parent_record_offset":9420,"ancestor_record_offset":7464,"inlinee":12168,"source_line":395,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$6::drop
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":707,"record_offset":9420,"parent_record_offset":9396,"ancestor_record_offset":7464,"inlinee":16119,"source_line":387,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_layout::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":707,"record_offset":8652,"parent_record_offset":8288,"ancestor_record_offset":7464,"inlinee":15494,"source_line":1865,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_slice_fill_iter::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":707,"record_offset":8784,"parent_record_offset":8108,"ancestor_record_offset":7464,"inlinee":39936,"source_line":1753,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::impl$9::alloc_slice_fill_with::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":707,"record_offset":8228,"parent_record_offset":8160,"ancestor_record_offset":7464,"inlinee":39942,"source_line":1556,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] bumpalo::round_mut_ptr_down_to
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bumpalo","version":"3.18.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bumpalo-3.18.1/src/lib.rs
//     evidence={"module":707,"record_offset":8424,"parent_record_offset":8328,"ancestor_record_offset":7464,"inlinee":15490,"source_line":450,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::array
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":707,"record_offset":8160,"parent_record_offset":8108,"ancestor_record_offset":7464,"inlinee":16143,"source_line":493,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::impl$0::array::inner
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\alloc\layout.rs
//     evidence={"module":707,"record_offset":8200,"parent_record_offset":8160,"ancestor_record_offset":7464,"inlinee":16145,"source_line":498,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":707,"record_offset":8368,"parent_record_offset":8328,"ancestor_record_offset":7464,"inlinee":4798,"source_line":541,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":707,"record_offset":8532,"parent_record_offset":8508,"ancestor_record_offset":7464,"inlinee":4435,"source_line":500,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::set
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cell.rs
//     evidence={"module":707,"record_offset":8508,"parent_record_offset":8328,"ancestor_record_offset":7464,"inlinee":4434,"source_line":428,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$58::lt
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":707,"record_offset":8744,"parent_record_offset":8716,"ancestor_record_offset":7464,"inlinee":4690,"source_line":1824,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::map::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\map.rs
//     evidence={"module":707,"record_offset":8816,"parent_record_offset":8784,"ancestor_record_offset":7464,"inlinee":6061,"source_line":106,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$5::spec_next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":707,"record_offset":8716,"parent_record_offset":8688,"ancestor_record_offset":7464,"inlinee":4688,"source_line":763,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":707,"record_offset":8688,"parent_record_offset":8108,"ancestor_record_offset":7464,"inlinee":4686,"source_line":848,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":707,"record_offset":9124,"parent_record_offset":9100,"ancestor_record_offset":7464,"inlinee":10996,"source_line":853,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":707,"record_offset":9100,"parent_record_offset":9064,"ancestor_record_offset":7464,"inlinee":4933,"source_line":116,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":707,"record_offset":8556,"parent_record_offset":8532,"ancestor_record_offset":7464,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::impls::impl$4::call_once
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ops\function.rs
//     evidence={"module":707,"record_offset":9032,"parent_record_offset":9000,"ancestor_record_offset":7464,"inlinee":5024,"source_line":304,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":707,"record_offset":9172,"parent_record_offset":8784,"ancestor_record_offset":7464,"inlinee":5587,"source_line":930,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":707,"record_offset":9000,"parent_record_offset":8816,"ancestor_record_offset":7464,"inlinee":4149,"source_line":1114,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::ok_or
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":707,"record_offset":8624,"parent_record_offset":8288,"ancestor_record_offset":7464,"inlinee":9461,"source_line":1252,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":707,"record_offset":9508,"parent_record_offset":9480,"ancestor_record_offset":7464,"inlinee":7359,"source_line":2074,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_offset
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":707,"record_offset":8472,"parent_record_offset":8448,"ancestor_record_offset":7464,"inlinee":5968,"source_line":528,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::wrapping_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":707,"record_offset":8448,"parent_record_offset":8424,"ancestor_record_offset":7464,"inlinee":5967,"source_line":1320,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":707,"record_offset":8960,"parent_record_offset":8936,"ancestor_record_offset":7464,"inlinee":4511,"source_line":609,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$14::eq
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":707,"record_offset":8884,"parent_record_offset":8844,"ancestor_record_offset":7464,"inlinee":4414,"source_line":1622,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":707,"record_offset":8580,"parent_record_offset":8556,"ancestor_record_offset":7464,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::post_inc_start
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":707,"record_offset":8936,"parent_record_offset":8912,"ancestor_record_offset":7464,"inlinee":4510,"source_line":99,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":707,"record_offset":8844,"parent_record_offset":8816,"ancestor_record_offset":7464,"inlinee":4507,"source_line":156,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$186::next_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":707,"record_offset":8912,"parent_record_offset":8844,"ancestor_record_offset":7464,"inlinee":4509,"source_line":440,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicU64::fetch_max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":7768,"parent_record_offset":7740,"ancestor_record_offset":7464,"inlinee":39124,"source_line":3344,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_umax
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":707,"record_offset":7792,"parent_record_offset":7768,"ancestor_record_offset":7464,"inlinee":29644,"source_line":3962,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::buffer::impl$6::update_buffer_bytes::closure$0::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":707,"record_offset":9064,"parent_record_offset":9032,"ancestor_record_offset":7464,"inlinee":39938,"source_line":467,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::command_buffer::CommandBufferSeqTracker::track
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/command_buffer.rs
//     evidence={"module":707,"record_offset":7740,"parent_record_offset":7656,"ancestor_record_offset":7464,"inlinee":39123,"source_line":54,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$17::track
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":707,"record_offset":7864,"parent_record_offset":7836,"ancestor_record_offset":7464,"inlinee":8709,"source_line":250,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":707,"record_offset":7712,"parent_record_offset":7656,"ancestor_record_offset":7464,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$4::track
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":707,"record_offset":7656,"parent_record_offset":7632,"ancestor_record_offset":7464,"inlinee":39132,"source_line":96,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::ManagedResourceMeta::checksum
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":707,"record_offset":7684,"parent_record_offset":7656,"ancestor_record_offset":7464,"inlinee":39121,"source_line":43,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::impl$6::update_buffer_bytes::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 445
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C03
// 首条位置: {"module":707,"inlinee":39939}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] ash::device::Device::cmd_pipeline_barrier
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":707,"record_offset":9276,"parent_record_offset":9244,"ancestor_record_offset":9244,"inlinee":39588,"source_line":2370,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::buffer::impl$6::update_buffer_bytes::closure$0::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 467
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9C02
// 首条位置: {"module":707,"inlinee":39938}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\exact_size.rs
//     evidence={"module":707,"record_offset":9100,"parent_record_offset":9064,"ancestor_record_offset":9064,"inlinee":4933,"source_line":116,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::range::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\range.rs
//     evidence={"module":707,"record_offset":9124,"parent_record_offset":9100,"ancestor_record_offset":9064,"inlinee":10996,"source_line":853,"archive":"_pdb/B/modules/00707.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::device::GraphicsDevice::get_vk_buffer
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=8
// 源码行: 489
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9A56
// 首条位置: {"module":699,"inlinee":39510}
// 编译实例 RVAs: 0xbef470
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1418aeba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: buffer.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13367072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381223536,"raw_hex":"e820f7cb00","size":5,"soft_fail":false,"target_va":5394590624,"va":5381223547,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 23
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::raw::error::BackendError> >", "constructor": "core::result::Result", "arguments": ["rhapsody::raw::buffer::Buffer", "enum2$<rhapsody::raw::error::BackendError>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::raw::error::BackendError> > >", "constructor": "enum2$", "arguments": ["core::result::Result<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::raw::error::BackendError> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::raw::error::BackendError> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::raw::error::BackendError> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["rhapsody::raw::buffer::Buffer", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >", "constructor": "tuple$", "arguments": ["rhapsody::raw::buffer::BufferDesc", "alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "anyhow::context::impl$0::with_context<rhapsody::raw::buffer::Buffer,enum2$<rhapsody::raw::error::BackendError>,alloc::string::String,rhapsody::rg::temporal::impl$12::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > >", "constructor": "anyhow::context::impl$0::with_context", "arguments": ["rhapsody::raw::buffer::Buffer", "enum2$<rhapsody::raw::error::BackendError>", "alloc::string::String", "rhapsody::rg::temporal::impl$12::get_or_create_temporal::closure_env$0<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<rhapsody::raw::buffer::Buffer,i32>", "constructor": "tuple$", "arguments": ["rhapsody::raw::buffer::Buffer", "i32"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<tuple$<rhapsody::raw::buffer::Buffer,i32>,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["tuple$<rhapsody::raw::buffer::Buffer,i32>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::poison::mutex::MutexGuard<alloc::vec::Vec<tuple$<rhapsody::raw::buffer::Buffer,i32>,alloc::alloc::Global> >", "constructor": "std::sync::poison::mutex::MutexGuard", "arguments": ["alloc::vec::Vec<tuple$<rhapsody::raw::buffer::Buffer,i32>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::poison::mutex::MutexGuard<alloc::vec::Vec<tuple$<rhapsody::raw::buffer::Buffer,i32>,alloc::alloc::Global> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::poison::mutex::MutexGuard<alloc::vec::Vec<tuple$<rhapsody::raw::buffer::Buffer,i32>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::raw_vec::RawVec::grow_one<tuple$<rhapsody::raw::buffer::Buffer,i32>,alloc::alloc::Global>", "constructor": "alloc::raw_vec::RawVec::grow_one", "arguments": ["tuple$<rhapsody::raw::buffer::Buffer,i32>", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "rhapsody::raw::managed::handle::ManagedRef<rhapsody::raw::buffer::Buffer>", "constructor": "rhapsody::raw::managed::handle::ManagedRef", "arguments": ["rhapsody::raw::buffer::Buffer"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<rhapsody::raw::managed::handle::ManagedRef<rhapsody::raw::buffer::Buffer> >", "constructor": "core::ptr::drop_in_place", "arguments": ["rhapsody::raw::managed::handle::ManagedRef<rhapsody::raw::buffer::Buffer>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::sync::Arc::drop_slow<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>", "constructor": "alloc::sync::Arc::drop_slow", "arguments": ["rhapsody::raw::buffer::Buffer", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<alloc::vec::Vec<tuple$<rhapsody::raw::buffer::Buffer,i32>,alloc::alloc::Global> > >", "constructor": "std::sync::poison::PoisonError", "arguments": ["std::sync::poison::mutex::MutexGuard<alloc::vec::Vec<tuple$<rhapsody::raw::buffer::Buffer,i32>,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<alloc::vec::Vec<tuple$<rhapsody::raw::buffer::Buffer,i32>,alloc::alloc::Global> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<alloc::vec::Vec<tuple$<rhapsody::raw::buffer::Buffer,i32>,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::drain::impl$7::drop::DropGuard<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>", "constructor": "alloc::vec::drain::impl$7::drop::DropGuard", "arguments": ["rhapsody::raw::buffer::Buffer", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::drain::impl$7::drop::DropGuard<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::drain::impl$7::drop::DropGuard<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::drain::Drain<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>", "constructor": "alloc::vec::drain::Drain", "arguments": ["rhapsody::raw::buffer::Buffer", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::drain::Drain<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::drain::Drain<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec::retain<tuple$<rhapsody::raw::buffer::Buffer,i32>,alloc::alloc::Global,system_wall_constructor::plaster_systems::maintain_deferred_plaster_buffer_disposal::closure_env$0>", "constructor": "alloc::vec::Vec::retain", "arguments": ["tuple$<rhapsody::raw::buffer::Buffer,i32>", "alloc::alloc::Global", "system_wall_constructor::plaster_systems::maintain_deferred_plaster_buffer_disposal::closure_env$0"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["rhapsody::raw::buffer::BufferDesc", "alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1", "arguments": ["tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>,std::hash::random::RandomState> >,tuple$<ptr_mut$<u8> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>,std::hash::random::RandomState> >", "tuple$<ptr_mut$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::clear<tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTable::clear", "arguments": ["tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["rhapsody::raw::buffer::BufferDesc", "alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::buffer::BufferDesc
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 7; 过程引用数: 13
// [PDB 泛型关系，非字段关系] {"type": "tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >", "constructor": "tuple$", "arguments": ["rhapsody::raw::buffer::BufferDesc", "alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTableInner::drop_inner_table<tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTableInner::drop_inner_table", "arguments": ["tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>,std::hash::random::RandomState>", "constructor": "hashbrown::map::make_hasher::closure_env$0", "arguments": ["rhapsody::raw::buffer::BufferDesc", "alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>", "std::hash::random::RandomState"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::impl$8::reserve_rehash::closure_env$1", "arguments": ["tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ops::function::FnOnce::call_once<hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>,std::hash::random::RandomState> >,tuple$<ptr_mut$<u8> > >", "constructor": "core::ops::function::FnOnce::call_once", "arguments": ["hashbrown::raw::impl$8::reserve_rehash::closure_env$1<tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>,std::hash::random::RandomState> >", "tuple$<ptr_mut$<u8> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "tuple$<rhapsody::raw::buffer::BufferDesc,rhapsody::rg::resource_registry::ResourceReuseInfo>", "constructor": "tuple$", "arguments": ["rhapsody::raw::buffer::BufferDesc", "rhapsody::rg::resource_registry::ResourceReuseInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::equivalent_key::closure_env$0<rhapsody::raw::buffer::BufferDesc,rhapsody::raw::buffer::BufferDesc,rhapsody::rg::resource_registry::ResourceReuseInfo>", "constructor": "hashbrown::map::equivalent_key::closure_env$0", "arguments": ["rhapsody::raw::buffer::BufferDesc", "rhapsody::raw::buffer::BufferDesc", "rhapsody::rg::resource_registry::ResourceReuseInfo"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::remove_entry<tuple$<rhapsody::raw::buffer::BufferDesc,rhapsody::rg::resource_registry::ResourceReuseInfo>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<rhapsody::raw::buffer::BufferDesc,rhapsody::raw::buffer::BufferDesc,rhapsody::rg::resource_registry::ResourceReuseInfo> >", "constructor": "hashbrown::raw::RawTable::remove_entry", "arguments": ["tuple$<rhapsody::raw::buffer::BufferDesc,rhapsody::rg::resource_registry::ResourceReuseInfo>", "alloc::alloc::Global", "hashbrown::map::equivalent_key::closure_env$0<rhapsody::raw::buffer::BufferDesc,rhapsody::raw::buffer::BufferDesc,rhapsody::rg::resource_registry::ResourceReuseInfo>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::reserve_rehash<tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>,std::hash::random::RandomState> >", "constructor": "hashbrown::raw::RawTable::reserve_rehash", "arguments": ["tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >", "alloc::alloc::Global", "hashbrown::map::make_hasher::closure_env$0<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>,std::hash::random::RandomState>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::raw::RawTable::clear<tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >,alloc::alloc::Global>", "constructor": "hashbrown::raw::RawTable::clear", "arguments": ["tuple$<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global> >", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["rhapsody::raw::buffer::BufferDesc", "alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "ref$<rhapsody::raw::buffer::BufferDesc>", "constructor": "ref$", "arguments": ["rhapsody::raw::buffer::BufferDesc"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<rhapsody::raw::buffer::BufferDesc> >", "constructor": "core::hash::BuildHasher::hash_one", "arguments": ["std::hash::random::RandomState", "ref$<rhapsody::raw::buffer::BufferDesc>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 20; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>,bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved> >,bevy_ecs::event::writer::EventWriter<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>", "bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved> >", "bevy_ecs::event::writer::EventWriter<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::window::WindowSize>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<enum2$<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorMoved> >
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >,bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::mouse::MouseButton> > >,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >,bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>,bevy_ecs::change_detection::ResMut<country_core::input::ActiveInputModeTracker>,bevy_ecs::event::reader::EventReader<bevy_input::mouse::MouseWheel>,bevy_ecs::system::system_param::Local<country_core::systems::backend::input::translate_input::MouseMoved>,bevy_ecs::event::reader::EventReader<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>,bevy_ecs::event::reader::EventReader<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>,bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >", "bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::mouse::MouseButton> > >", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource> > >", "bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>", "bevy_ecs::change_detection::ResMut<country_core::input::ActiveInputModeTracker>", "bevy_ecs::event::reader::EventReader<bevy_input::mouse::MouseWheel>", "bevy_ecs::system::system_param::Local<country_core::systems::backend::input::translate_input::MouseMoved>", "bevy_ecs::event::reader::EventReader<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>", "bevy_ecs::event::reader::EventReader<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>", "bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::keyboard::KeyCode> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_input::button_input::ButtonInput<enum2$<bevy_input::mouse::MouseButton> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::steam::SteamClientResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::resources::gui::EscMenuState> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::ActiveInputModeTracker>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<bevy_input::mouse::MouseWheel>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::backend::input::translate_input::MouseMoved>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::backend::input::translate_input_to_cursor_moved::InputCursorDelta>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::backend::input::calculate_input_delta::UiHistoryButtonPressed>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::game_profile::GameProfile>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>,bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>,bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>,bevy_ecs::change_detection::ResMut<country_core::resources::window::WindowSize>,bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>,bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustClosed>,enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >", "bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>", "bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>", "bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>", "bevy_ecs::change_detection::ResMut<country_core::resources::window::WindowSize>", "bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>", "bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>", "bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustClosed>", "enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::session::NextSession> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::toolbar::Toolbar>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::systems::camera_rig::MainCamera>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::game_config::GameConfig>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::window::WindowSize>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetValues>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::input_stream::InputRecordReplayState>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustClosed>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::NextSession>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >", "bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetValues>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::input::action_set::ActionSetStates>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::ResMut<country_core::systems::mod_key::Modifiers>,bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::system::system_param::Local<country_core::systems::mod_key::ToggleSnapping>,bevy_ecs::event::reader::EventReader<country_core::resources::walls::decorator::DecoratorOpEvent>,bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::ResMut<country_core::systems::mod_key::Modifiers>", "bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::system::system_param::Local<country_core::systems::mod_key::ToggleSnapping>", "bevy_ecs::event::reader::EventReader<country_core::resources::walls::decorator::DecoratorOpEvent>", "bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::systems::mod_key::Modifiers>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::mod_key::ToggleSnapping>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::walls::decorator::DecoratorOpEvent>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>,bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>,bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,bevy_ecs::event::writer::EventWriter<country_core::resources::window::CursorMovedScreenSpace>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>", "bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>", "bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>", "bevy_ecs::event::writer::EventWriter<country_core::resources::window::CursorMovedScreenSpace>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::input::action_set::ActionSetStates>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::mouse_capture::CursorCapturedPosition>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::mouse_terrain_raycast::CursorPositionSS>
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::resources::window::CursorMovedScreenSpace>
// [文件级接口候选，具体函数归属 unknown] bool (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>", "bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState> >
// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::systems::world_raster::RasterHistoryRecordResources,bevy_ecs::event::reader::EventReader<country_core::systems::garden::record_garden_history::RecordGardenEditToHistoryCmd>)
// 候选参数类型: ["country_core::systems::world_raster::RasterHistoryRecordResources", "bevy_ecs::event::reader::EventReader<country_core::systems::garden::record_garden_history::RecordGardenEditToHistoryCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::garden::record_garden_history::RecordGardenEditToHistoryCmd>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::Buffer::device_address
// 信息 ID: 8542b49b96b0e479e21889b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists @ 0x988f99 -> 0xbd4c00 (direct_call); 同地址别名未裁定。
// [静态直接引用] itertools::unziptuple::impl$2::multiunzip<core::iter::adapters::filter_map::FilterMap<enum_iterator::All<country_core::render::nani::gpu_world::GpuWorldSubsetKind>,country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket::closure_env$0>,country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket::PushConstants,alloc::vec::Vec<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::cull_and_bucket::PushConstants,alloc::alloc::Global>,u32,alloc::vec::Vec<u32,alloc::alloc::Global> > @ 0xb2b8da -> 0xbd4c00 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_wall_constructor::plaster_systems::generate_plaster_mesh_and_entities<core::iter::adapters::map::Map<core::iter::adapters::filter::Filter<core::slice::iter::Iter<tuple$<utils::geometry::aabb::Aabb2,country_core::resources::walls::wall_holes::HoleType,country_core::resources::walls::wall_holes::HoleOrigin> >,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$0>,country_core::resources::walls::wall_holes::impl$1::holes_no_duplicates::closure_env$1> > @ 0x1228564 -> 0xbd4c00 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::nani::trimesh::TrimeshBuilder::build @ 0x129ef73 -> 0xbd4c00 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::raw::dynamic_constants::DynamicConstants::new; module=705, inline_record=3792; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::dynamic_constants::DynamicConstants::allocate; module=705, inline_record=4456; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::Buffer::mapped_slice_mut
// 信息 ID: a3e73a5e191ebca715cfd6b5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 25 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::resources::init_populate_ssbo_library @ 0x90a5a9 -> 0xbd4cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::last_known_frame_duration_ns @ 0xbeaaec -> 0xbd4cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::dynamic_constants::DynamicConstants::new @ 0xc025c6 -> 0xbd4cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::dynamic_constants::DynamicConstants::allocate @ 0xc02861 -> 0xbd4cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$4::new_storage_buffer<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo> @ 0x161aea8 -> 0xbd4cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$4::new_storage_buffer<country_core::systems::render_loop::resources::critters::ButterflySsbo> @ 0x161b563 -> 0xbd4cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$4::new_storage_buffer<country_core::resources::terrain_height::TerrainHeightSsbo> @ 0x161ba16 -> 0xbd4cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$4::new_storage_buffer<country_core::systems::render_loop::compute_passes::read_bush_positions_for_cpu::BushCpuSsbo> @ 0x161be7b -> 0xbd4cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$4::new_storage_buffer<country_core::resources::water_exclude_clutter_mask::WaterExcludeClutterMaskSsbo> @ 0x161c226 -> 0xbd4cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$4::new_storage_buffer<country_core::resources::terrain_height::VisualTerrainHeightSsbo> @ 0x161c6e6 -> 0xbd4cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$4::new_storage_buffer<country_core::resources::world_raster_defs::GardenRasterStagingSsbo> @ 0x161cb38 -> 0xbd4cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$4::new_storage_buffer<country_core::systems::render_loop::compute_passes::mouse_read_terrain_stroke_index::MouseTerrainRaycastSsbo> @ 0x161cee0 -> 0xbd4cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$4::new_storage_buffer<country_core::resources::world_raster_defs::Cobblestone3RasterStagingSsbo> @ 0x161d328 -> 0xbd4cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$4::new_storage_buffer<country_core::resources::water_mask::WaterMaskSsbo> @ 0x161d716 -> 0xbd4cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$4::new_storage_buffer<country_core::resources::world_raster_defs::Cobblestone1RasterStagingSsbo> @ 0x161db68 -> 0xbd4cd0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$4::new_storage_buffer<country_core::systems::audio::terrain_stairs_audio::StairsCounterSsbo> @ 0x161df09 -> 0xbd4cd0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::BufferDesc::alignment
// 信息 ID: fabe9874048a2611bac3c0bd
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 106 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::resources::critters::SheepAnimationSsbo> @ 0x160f9ba -> 0xbd4f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo> @ 0x160fc5f -> 0xbd4f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo> @ 0x160fe10 -> 0xbd4f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::render::nani::buffers::NaniListDrawCmdOffsets> @ 0x1610040 -> 0xbd4f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo> @ 0x1610398 -> 0xbd4f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::leaf_particles::ParticleEmitterSsbo> @ 0x16106ef -> 0xbd4f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::indirect_params::DrawIndirectBuffer> @ 0x16108fa -> 0xbd4f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo> @ 0x1610c15 -> 0xbd4f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo> @ 0x1610e63 -> 0xbd4f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::geometry::instanced_wall::BrickPrevVisibilityBitmaskSsbo> @ 0x1611127 -> 0xbd4f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::resources::render::spatial_resolve_offsets::SpatialResolveOffsetsSsbo> @ 0x161149a -> 0xbd4f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo> @ 0x16117e5 -> 0xbd4f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::grass::GrassBladeDataSsbo> @ 0x1611a6f -> 0xbd4f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::resources::rocky_terrain::RockyTerrainVertexSSBO> @ 0x1611c55 -> 0xbd4f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::geometry::instanced_wall::BrickNextVisibilityBitmaskSsbo> @ 0x1611f17 -> 0xbd4f20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::apply_screen_shadows::ShadowMaskHistogramSsbo> @ 0x16123f4 -> 0xbd4f20 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::raw::dynamic_constants::DynamicConstants::allocate_buffer; module=705, inline_record=5636; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::BufferDesc::new
// 信息 ID: 448606ce59f76fad269b391a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 108 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::painterly::painterly_pass @ 0x8a6f6b -> 0xbd4de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::vermeer::vermeer_render_from_target @ 0x8a8f60 -> 0xbd4de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::resources::critters::SheepAnimationSsbo> @ 0x160f9a5 -> 0xbd4de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo> @ 0x160fc4a -> 0xbd4de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo> @ 0x160fdfb -> 0xbd4de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::render::nani::buffers::NaniListDrawCmdOffsets> @ 0x161002b -> 0xbd4de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo> @ 0x1610374 -> 0xbd4de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::leaf_particles::ParticleEmitterSsbo> @ 0x16106da -> 0xbd4de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::indirect_params::DrawIndirectBuffer> @ 0x16108d6 -> 0xbd4de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo> @ 0x1610c00 -> 0xbd4de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo> @ 0x1610e4e -> 0xbd4de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::geometry::instanced_wall::BrickPrevVisibilityBitmaskSsbo> @ 0x1611103 -> 0xbd4de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::resources::render::spatial_resolve_offsets::SpatialResolveOffsetsSsbo> @ 0x1611473 -> 0xbd4de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo> @ 0x16117d0 -> 0xbd4de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::grass::GrassBladeDataSsbo> @ 0x1611a5a -> 0xbd4de0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::resources::rocky_terrain::RockyTerrainVertexSSBO> @ 0x1611c40 -> 0xbd4de0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu
// 信息 ID: de9fb72e06d050289d831900
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0xdedb4 -> 0xbd4e60 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0xdee54 -> 0xbd4e60 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_screenshot::create_graphics_resources @ 0x5898d1 -> 0xbd4e60 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu_low_prio
// 信息 ID: 42a344170652773f33126d0e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 5 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x5b958f -> 0xbd4ea0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<ref$<slice2$<u8> > > @ 0x16a2b3f -> 0xbd4ea0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x16a2f9f -> 0xbd4ea0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x1fdcc3f -> 0xbd4ea0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x288eb6f -> 0xbd4ea0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::update_buffer_bytes; module=703, inline_record=14548; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::update_texture_impl; module=703, inline_record=141240; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > >; module=707, inline_record=6136; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::BufferDesc::new_gpu_only
// 信息 ID: 0ac7543cb6ce26c72c385da6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 35 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true> @ 0x10d02 -> 0xbd4e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false> @ 0x201c2 -> 0xbd4e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::generate_mips::generate_mips<country_core::render::pass_builder::pass_resource_access::WriteTextureUnchecked> @ 0xfb985 -> 0xbd4e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$4::call_once<tuple$<tuple$<ref$<alloc::string::String>,ref$<enum2$<country_core::render::mesh::VertexAttributeValues> > > >,country_core::render::vao::build_vao::closure_env$0> @ 0x8ed169 -> 0xbd4e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::vao::build_vao @ 0x9096d3 -> 0xbd4e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::generate_hzb::generate_hzb @ 0x9287c0 -> 0xbd4e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists @ 0x987e95 -> 0xbd4e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists @ 0x987f0d -> 0xbd4e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists @ 0x98872a -> 0xbd4e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::prefix_scan::inclusive_prefix_scan_u32_1m @ 0x98c190 -> 0xbd4e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass @ 0x9bf84b -> 0xbd4e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::water_surface_pass @ 0x9bf8ed -> 0xbd4e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass @ 0x9c0b05 -> 0xbd4e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass @ 0x9c0b79 -> 0xbd4e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass @ 0x9c0c4d -> 0xbd4e20 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::draw_passes::transparency_pass::ice_surface_pass @ 0x9c0cc6 -> 0xbd4e20 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::BufferDesc::new_gpu_to_cpu
// 信息 ID: 74cd5c27f78e28dadecec9cb
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x31d906 -> 0xbd4ee0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::rg::executor::GraphExecutor::new; module=699, inline_record=8316; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create; module=703, inline_record=59868; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$0::as_managed_handle
// 信息 ID: 5527083d4f4d82c48791d00b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 137943db23b17086ce8b9d61: bool (*)(country_core::systems::collision::world::SubworldColliderHandle,ref$<enum2$<country_core::systems::collision::world::RaycastGeoSource> >)
// [泛型宿主/闭包/直接引用方参数候选] 1e83d0fd90a253224831420b: enum2$<core::ops::control_flow::ControlFlow<rhapsody::raw::physical_device::PhysicalDevice,tuple$<> > > (*)(rhapsody::raw::physical_device::PhysicalDevice)
// 引用方: 原程序直接转移 1177 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::setup::setup_window @ 0x43b79 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::setup_window @ 0x43cba -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::setup_window @ 0x44288 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::find_best_fullscreen_video_mode @ 0x45e63 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::find_best_fullscreen_video_mode @ 0x45c78 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::setup::find_best_fullscreen_video_mode @ 0x457de -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] winit::platform_impl::platform::window::InitData::on_create<tuple$<> > @ 0x7d1dc -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] winit::platform_impl::platform::window::init<tuple$<> > @ 0x7e616 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$8,tuple$<> > @ 0xd797c -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::run_game::run_game_thread::closure_env$8,tuple$<> > @ 0xd7a85 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] std::sys::backtrace::__rust_begin_short_backtrace<tiny_glade::screenshot::write_image::closure_env$0,tuple$<> > @ 0xd8010 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::setup_panic_hook::closure$0 @ 0xdbaba -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::setup_panic_hook::closure$0 @ 0xdba0a -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0xdea3b -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0xdea5c -> 0x67dd80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0xdeef4 -> 0x67dd80 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$10::fmt
// 信息 ID: 230b29dd446ecaec5214209e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$11::hash
// 信息 ID: b010b5f0fb2eeeb7ca4c087f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::hash::BuildHasher::hash_one<std::hash::random::RandomState,ref$<rhapsody::raw::buffer::BufferDesc> >; module=1947, inline_record=11052; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$14::eq
// 信息 ID: 56cd0f189c9e8db8f1048546
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::resource_pool::ResourcePool::take_buffer; module=695, inline_record=5548; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::resource_pool::ResourcePool::insert_buffer; module=695, inline_record=6976; 不是独立调用边。
// [PDB 内联宿主] hashbrown::raw::RawTable::remove_entry<tuple$<rhapsody::raw::buffer::BufferDesc,rhapsody::rg::resource_registry::ResourceReuseInfo>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env$0<rhapsody::raw::buffer::BufferDesc,rhapsody::raw::buffer::BufferDesc,rhapsody::rg::resource_registry::ResourceReuseInfo> >; module=1741, inline_record=15472; 不是独立调用边。
// [PDB 内联宿主] hashbrown::map::HashMap::insert<rhapsody::raw::buffer::BufferDesc,alloc::vec::Vec<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>; module=1917, inline_record=10244; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$3::byte_count
// 信息 ID: f173551813e89bfa69e3d2f1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 75 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] toolbar::mei::config::impl$18::template_with_id<ref$<str$> > @ 0x32e362 -> 0x6868b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::slot<ref$<str$> > @ 0x4fe36a -> 0x6868b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::into_slot<ref$<str$> > @ 0x4fe580 -> 0x6868b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$8,ref$<str$> > @ 0x4fe7c2 -> 0x6868b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::ui_settings_keybindings::closure_env$1,ref$<str$> > @ 0x4fe9fa -> 0x6868b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::do_gui_escape_menu_settings::closure_env$2,ref$<str$> > @ 0x4fec1a -> 0x6868b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::ui_settings_keybindings::closure_env$5,ref$<str$> > @ 0x4fee3a -> 0x6868b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::ui_settings_keybindings::closure_env$7,ref$<str$> > @ 0x4ff06f -> 0x6868b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_link_to_official_game::display_link_to_official_game::closure_env$0,ref$<str$> > @ 0x4ff3ca -> 0x6868b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::ui_settings_keybindings::closure_env$2,ref$<str$> > @ 0x4ff5ea -> 0x6868b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$15,ref$<str$> > @ 0x4ff806 -> 0x6868b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::ui_settings_keybindings::closure_env$3,ref$<str$> > @ 0x4ffa66 -> 0x6868b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_tod::do_time_of_day_screen::closure_env$0,ref$<str$> > @ 0x4ffcba -> 0x6868b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$11,ref$<str$> > @ 0x4ffed6 -> 0x6868b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::settings::ui_settings_keybindings::closure_env$8,ref$<str$> > @ 0x50013a -> 0x6868b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] toolbar::mei::config::TemplateView::with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env$7,ref$<str$> > @ 0x500396 -> 0x6868b0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$3::copy_to_slice
// 信息 ID: 93071283dcb853f5cd2e70f6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<ref$<slice2$<u8> > > @ 0x16a2abe -> 0xbd4f80 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<ref$<slice2$<u8> > > @ 0x16a2c1b -> 0xbd4f80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$4::byte_count
// 信息 ID: e0b43f10d01d29f0caa27650
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 20 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x5b93fe -> 0xbd51a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x5b94c0 -> 0xbd51a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x5b94d1 -> 0xbd51a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x5b94f3 -> 0xbd51a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x5b964d -> 0xbd51a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x16a2e0e -> 0xbd51a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x16a2ed0 -> 0xbd51a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x16a2ee1 -> 0xbd51a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x16a2f03 -> 0xbd51a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x16a305d -> 0xbd51a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x1fdcaae -> 0xbd51a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x1fdcb70 -> 0xbd51a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x1fdcb81 -> 0xbd51a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x1fdcba3 -> 0xbd51a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x1fdccfd -> 0xbd51a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x288e9de -> 0xbd51a0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > >; module=707, inline_record=5724; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$6::create_buffer::closure_env$0<enum2$<core::option::Option<ref$<slice2$<u8> > > > > >; module=707, inline_record=15148; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$6::create_buffer::closure_env$0<enum2$<core::option::Option<ref$<slice2$<u8> > > > > >; module=707, inline_record=16228; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$4::copy_to_slice
// 信息 ID: 36e6f25f4aa4fe56f52f749b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 8 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x5b950e -> 0xbd51b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x5b966b -> 0xbd51b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x16a2f1e -> 0xbd51b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x16a307b -> 0xbd51b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x1fdcbbe -> 0xbd51b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x1fdcd1b -> 0xbd51b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x288eaee -> 0xbd51b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x288ec4b -> 0xbd51b0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > >; module=707, inline_record=5996; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > >; module=707, inline_record=6388; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$6::create_buffer::closure$0
// 信息 ID: 9548c10f8b2001ae1835e000
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::buffer::impl$6::create_buffer", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::buffer::impl$6::create_buffer::closure_env$0 <- rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$6::create_buffer::closure_env$0<enum2$<core::option::Option<ref$<slice2$<u8> > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] rhapsody::raw::buffer::impl$6::create_buffer::closure_env$0 <- rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$6::create_buffer::closure_env$0<ref$<slice2$<u8> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 4 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$6::create_buffer::closure_env$0<enum2$<core::option::Option<ref$<slice2$<u8> > > > > >; module=707, inline_record=14516; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$6::create_buffer::closure_env$0<enum2$<core::option::Option<ref$<slice2$<u8> > > > > >; module=707, inline_record=15596; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$6::create_buffer::closure_env$0<ref$<slice2$<u8> > > >; module=1657, inline_record=8824; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$6::create_buffer::closure_env$0<ref$<slice2$<u8> > > >; module=1657, inline_record=9356; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$6::create_buffer_impl::closure$0
// 信息 ID: af9523045f4814b09d0ee920
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::buffer::impl$6::create_buffer_impl", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_buffer_impl; module=703, inline_record=9420; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$6::create_buffer_impl::closure$1
// 信息 ID: 7a1b22497ba94a87498fc885
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "rhapsody::raw::buffer::impl$6::create_buffer_impl", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::device::GraphicsDevice::create_buffer_impl; module=703, inline_record=10760; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$6::create_buffer_impl::closure$2
// 信息 ID: 1b893cbda833da85a54c058a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "rhapsody::raw::buffer::impl$6::create_buffer_impl", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$6::update_buffer_bytes::closure$0
// 信息 ID: 1a63312e50d81a7cf093a2f6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::buffer::impl$6::update_buffer_bytes", "captures": "unknown"}
// [闭包环境类型引用] rhapsody::raw::buffer::impl$6::update_buffer_bytes::closure_env$0 <- rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$6::update_buffer_bytes::closure_env$0>; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 1 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$6::update_buffer_bytes::closure_env$0> @ 0xc04322 -> 0xc03a00 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::with_setup_cb<rhapsody::raw::buffer::impl$6::update_buffer_bytes::closure_env$0> @ 0xc043de -> 0xc03a00 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$6::update_buffer_bytes::closure$0::closure$0
// 信息 ID: 060414300d5de825a06fe9d7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::buffer::impl$6::update_buffer_bytes", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::buffer::impl$6::update_buffer_bytes::closure$0; module=707, inline_record=9244; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::buffer::impl$6::update_buffer_bytes::closure$0; module=707, inline_record=9336; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$6::update_buffer_bytes::closure$0::closure$1
// 信息 ID: 69e8c8bb774eef22b0f99433
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::buffer::impl$6::update_buffer_bytes", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::buffer::impl$6::update_buffer_bytes::closure$0; module=707, inline_record=9064; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$7::clone
// 信息 ID: f8d66ec95e517523ba935f01
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::import_export::impl$1::import_unowned; module=677, inline_record=2716; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::buffer::impl$8::clone
// 信息 ID: bfb1d28dc1c0c76095bf3add
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 34 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists; module=521, inline_record=58864; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists; module=521, inline_record=58992; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists; module=521, inline_record=62304; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists; module=521, inline_record=64208; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists; module=521, inline_record=68616; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists; module=521, inline_record=68744; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists; module=521, inline_record=69668; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::gpu_generate_draw_lists; module=521, inline_record=69824; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > >
// 信息 ID: f995ff9afb109c2f6dc8f999
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["u8"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<u8>"]; 字段布局 = unknown。
// [类型关系] core::option::Option -> ["ref$<slice2$<u8> >"]; 字段布局 = unknown。
// [类型关系] enum2$ -> ["core::option::Option<ref$<slice2$<u8> > >"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::create_buffer -> ["enum2$<core::option::Option<ref$<slice2$<u8> > > >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 115 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<true> @ 0x10d34 -> 0x5b93b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::stairs_assemble::playermade_stairs_assemble<false> @ 0x201f4 -> 0x5b93b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$12::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0x4f64e5 -> 0x5b93b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::resource_registry::ResourceRegistry::populate_resources @ 0xbe84bc -> 0xc03460 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::executor::GraphExecutor::new @ 0xbea3b1 -> 0xc03460 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$4::call_once<tuple$<tuple$<ref$<alloc::string::String>,ref$<enum2$<country_core::render::mesh::VertexAttributeValues> > > >,country_core::render::vao::build_vao::closure_env$0> @ 0x8ed195 -> 0x16a2dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::vao::build_vao @ 0x9096ff -> 0x16a2dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::temporal::impl$12::get_or_create_temporal<country_core::render::pass_builder::pass_resources::PassResources,ref$<str$> > @ 0x96c525 -> 0x16a2dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::resources::critters::SheepAnimationSsbo> @ 0x160f9e2 -> 0x16a2dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::draw_passes::ui_2d_pass::ShapeStreamSsbo> @ 0x160fc87 -> 0x16a2dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::render::nani::buffers::NaniPrevVisibilityBitmaskSsbo> @ 0x160fe38 -> 0x16a2dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::render::nani::buffers::NaniListDrawCmdOffsets> @ 0x1610068 -> 0x16a2dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::dof_autofocus::DofAutofocusSsbo> @ 0x16103ce -> 0x16a2dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::leaf_particles::ParticleEmitterSsbo> @ 0x1610717 -> 0x16a2dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::indirect_params::DrawIndirectBuffer> @ 0x1610930 -> 0x16a2dc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::ssbo::impl$0::new_storage_buffer<country_core::systems::render_loop::compute_passes::compute_garden::BushSsbo> @ 0x1610c45 -> 0x16a2dc0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_buffer<ref$<slice2$<u8> > >
// 信息 ID: d16d8aaedc0eb3199b2849d6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] slice2$ -> ["u8"]; 字段布局 = unknown。
// [类型关系] ref$ -> ["slice2$<u8>"]; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::create_buffer -> ["ref$<slice2$<u8> >"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::nani::trimesh::TrimeshBuilder::build @ 0x129ef16 -> 0x16a2960 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_buffer<tuple$<> >
// 信息 ID: 807d613c0dd3e8e806fe91c3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] tuple$ -> []; 字段布局 = unknown。
// [类型关系] rhapsody::raw::device::GraphicsDevice::create_buffer -> ["tuple$<>"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::dynamic_constants::DynamicConstants::allocate_buffer @ 0xc02b0f -> 0xc038c0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_buffer_impl
// 信息 ID: f28b89f63198fdf875cc5d70
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 17 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x5b9456 -> 0xbee200 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x5b95da -> 0xbee200 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_mapped_buffer @ 0xbeeb01 -> 0xbee200 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_buffer_bytes @ 0xbef122 -> 0xbee200 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create @ 0xbf5cef -> 0xbee200 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::update_texture_impl @ 0xbfec0a -> 0xbee200 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0xc03519 -> 0xbee200 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0xc0368e -> 0xbee200 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<tuple$<> > @ 0xc03933 -> 0xbee200 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<ref$<slice2$<u8> > > @ 0x16a2a06 -> 0xbee200 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<ref$<slice2$<u8> > > @ 0x16a2b8a -> 0xbee200 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x16a2e66 -> 0xbee200 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x16a2fea -> 0xbee200 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x1fdcb06 -> 0xbee200 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x1fdcc8a -> 0xbee200 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::device::GraphicsDevice::create_buffer<enum2$<core::option::Option<ref$<slice2$<u8> > > > > @ 0x288ea36 -> 0xbee200 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::create_mapped_buffer
// 信息 ID: ee73b8c6396f4cb7e433cbf0
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0xdedd5 -> 0xbeea80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::egui_backend::EguiBackend::draw @ 0xdee7c -> 0xbeea80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::screenshot::handle_screenshot_opportunity @ 0x31d923 -> 0xbeea80 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::panic_reporter::panic_screenshot::create_graphics_resources @ 0x5898ef -> 0xbeea80 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::get_vk_buffer
// 信息 ID: a532fb47421ad9d97f0daa17
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 8 条；内联宿主 4 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::draw_builder::DrawBuilder::bind_external_storage_buffer @ 0x90660d -> 0xbef470 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::bind_external_storage_buffer @ 0x96e3ae -> 0xbef470 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::allocate_draw_lists::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0x15f9005 -> 0xbef470 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::group_draw_cmd_prefix_sum::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0x15f9a8d -> 0xbef470 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::fill_unsorted_instance_indirection::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0x15fa7ae -> 0xbef470 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::generate_draw_lists::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0x15facdd -> 0xbef470 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::group_draw_cmd_prefix_sum::closure_env$1<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0x15fba39 -> 0xbef470 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::FnOnce::call_once<country_core::systems::render_loop::compute_passes::gpu_generate_draw_lists::fill_sorted_instance_list::closure_env$0<country_core::render::nani::gpu_world::RenderSublistsIter>,tuple$<country_core::render::pass_builder::compute_pass_builder::ComputeDispatchIndirectApi> > @ 0x15fbf61 -> 0xbef470 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] rhapsody::rg::executor::GraphExecutionRecorder::end_frame; module=699, inline_record=24616; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_buffer_binding; module=705, inline_record=2568; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_and_storage_buffer_binding; module=705, inline_record=2952; 不是独立调用边。
// [PDB 内联宿主] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_buffer_binding_and_data; module=705, inline_record=3332; 不是独立调用边。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::set_buffer_debug_name
// 信息 ID: 6119e1bd77ce4ae60aee6668
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::resource_registry::ResourceRegistry::populate_resources @ 0xbe8568 -> 0xbeec30 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::device::GraphicsDevice::update_buffer_bytes
// 信息 ID: e2e46da22ae17b7565857c2b
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 36 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::app_systems::walls_on_glade_unload @ 0x2b247e -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::app_systems::walls_on_glade_unload @ 0x2b266d -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::app_systems::walls_on_glade_unload @ 0x2b2852 -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::decorator_visual::lantern::maintain_lantern_lights @ 0x790f05 -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::dormer_holes::upload_dormer_holes_to_gpu @ 0x7b3732 -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_decorator::dormer_holes::upload_dormer_holes_to_gpu @ 0x7b38b8 -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::rt::upload_rt_world @ 0x8bd4b5 -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::rt::upload_rt_world @ 0x8bd5b1 -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::resources::rt::upload_rt_world @ 0x8bd6b7 -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf @ 0x8c1a1b -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf @ 0x8c1c6e -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::asset_libraries::color_library::update_color_library @ 0x93b89c -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::rasterize_buildings::rasterize_buildings @ 0xa32a3f -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::startup_nani::reload_nani @ 0xa39c64 -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::clearing::init_clearing @ 0xa79723 -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::clear_canvas::clear_canvas @ 0xa7b618 -> 0xbeefa0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
