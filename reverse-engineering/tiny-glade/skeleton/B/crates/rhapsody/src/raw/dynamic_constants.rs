// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/rhapsody/src/raw/dynamic_constants.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 16c16b7bef96dbce20b5e1604717456839860439e1e28d490dc3c0199c043c5d
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: dynamic_constants.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 66, 185, 480, 495, 503, 549, 705, 1951
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use ash; // namespace candidate
//   package identities: [{"name":"ash","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.38.0+1.3.281"}]
// use bytemuck; // namespace candidate
//   package identities: [{"name":"bytemuck","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.16.0"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use leekspin; // namespace candidate
//   package identities: [{"name":"leekspin","source":"local_or_path","version":"0.1.0"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use rhapsody; // namespace candidate
//   package identities: [{"name":"rhapsody","source":"local_or_path","version":"0.1.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::dynamic_constants::DynamicConstantsChunk::write
// 记录形式: inline_instance, inlinee_declaration; instances=5
// 源码行: 26
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x271A
// 首条位置: {"module":66,"inlinee":10010}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":66,"record_offset":9848,"parent_record_offset":9816,"ancestor_record_offset":9816,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":66,"record_offset":9884,"parent_record_offset":9848,"ancestor_record_offset":9816,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::dynamic_constants::DynamicConstantsChunk::with_data
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 30
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x67F7
// 首条位置: {"module":495,"inlinee":26615}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/systems/render_loop/compute_passes/rasterize_terrain_stroke.rs
//     evidence={"module":495,"record_offset":75120,"parent_record_offset":75088,"ancestor_record_offset":75088,"inlinee":26599,"source_line":108,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":495,"record_offset":75364,"parent_record_offset":75340,"ancestor_record_offset":75088,"inlinee":4155,"source_line":1561,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":495,"record_offset":75316,"parent_record_offset":75292,"ancestor_record_offset":75088,"inlinee":8707,"source_line":3472,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$8::deref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":495,"record_offset":75340,"parent_record_offset":75316,"ancestor_record_offset":75088,"inlinee":4153,"source_line":3265,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::cast_slice_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/lib.rs
//     evidence={"module":495,"record_offset":75176,"parent_record_offset":75120,"ancestor_record_offset":75088,"inlinee":17634,"source_line":364,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::cast_slice_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":495,"record_offset":75208,"parent_record_offset":75176,"ancestor_record_offset":75088,"inlinee":14011,"source_line":266,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] bytemuck::internal::try_cast_slice_mut
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"bytemuck","version":"1.16.0","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bytemuck-1.16.0/src/internal.rs
//     evidence={"module":495,"record_offset":75244,"parent_record_offset":75208,"ancestor_record_offset":75088,"inlinee":14012,"source_line":379,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::Ord::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":495,"record_offset":75616,"parent_record_offset":75592,"ancestor_record_offset":75088,"inlinee":4530,"source_line":1037,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::min
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":495,"record_offset":75592,"parent_record_offset":75424,"ancestor_record_offset":75088,"inlinee":4528,"source_line":1475,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::TrustedRandomAccessNoCoerce::size
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":495,"record_offset":75452,"parent_record_offset":75424,"ancestor_record_offset":75088,"inlinee":9852,"source_line":595,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":495,"record_offset":75400,"parent_record_offset":75292,"ancestor_record_offset":75088,"inlinee":4524,"source_line":24,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":495,"record_offset":75660,"parent_record_offset":75120,"ancestor_record_offset":75088,"inlinee":4538,"source_line":85,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::new
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":495,"record_offset":75424,"parent_record_offset":75400,"ancestor_record_offset":75088,"inlinee":4526,"source_line":301,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$5::next
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\zip.rs
//     evidence={"module":495,"record_offset":75692,"parent_record_offset":75660,"ancestor_record_offset":75088,"inlinee":4539,"source_line":308,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":495,"record_offset":75292,"parent_record_offset":75120,"ancestor_record_offset":75088,"inlinee":4522,"source_line":586,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":495,"record_offset":75548,"parent_record_offset":75524,"ancestor_record_offset":75088,"inlinee":4937,"source_line":774,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":495,"record_offset":75768,"parent_record_offset":75744,"ancestor_record_offset":75088,"inlinee":4463,"source_line":1023,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mut_ptr.rs
//     evidence={"module":495,"record_offset":75524,"parent_record_offset":75500,"ancestor_record_offset":75088,"inlinee":4936,"source_line":947,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub_ptr
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\non_null.rs
//     evidence={"module":495,"record_offset":75500,"parent_record_offset":75476,"ancestor_record_offset":75088,"inlinee":4935,"source_line":908,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$182::__iterator_get_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":495,"record_offset":75744,"parent_record_offset":75692,"ancestor_record_offset":75088,"inlinee":4540,"source_line":372,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$190::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\iter\macros.rs
//     evidence={"module":495,"record_offset":75476,"parent_record_offset":75452,"ancestor_record_offset":75088,"inlinee":24694,"source_line":171,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::dynamic_constants::DynamicConstantsChunk::device_address
// 记录形式: compiled_procedure; instances=1
// 源码行: 35
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":705,"record_offset":2020,"rva":12591744,"va":5381300864}
// 编译实例 RVAs: 0xc02280
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_gpu_lifetime_tracker
// 记录形式: compiled_procedure; instances=1
// 源码行: 45
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":705,"record_offset":2168,"rva":12591760,"va":5381300880}
// 编译实例 RVAs: 0xc02290
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_buffer_binding
// 记录形式: compiled_procedure; instances=1
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":705,"record_offset":2328,"rva":12591792,"va":5381300912}
// 编译实例 RVAs: 0xc022b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dynamic_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-130730","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381300912,"raw_hex":"e85601feff","size":5,"soft_fail":false,"target_va":5381170352,"va":5381301077,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418aeba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dynamic_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13289647","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381300912,"raw_hex":"e8afc8ca00","size":5,"soft_fail":false,"target_va":5394590624,"va":5381300972,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30240722","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381300912,"raw_hex":"e8d26fcd01","size":5,"soft_fail":false,"target_va":5411541776,"va":5381301049,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::GpuOwned::map_tracker
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":705,"record_offset":2496,"parent_record_offset":2328,"ancestor_record_offset":2328,"inlinee":39911,"source_line":507,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":705,"record_offset":2600,"parent_record_offset":2528,"ancestor_record_offset":2328,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":705,"record_offset":2628,"parent_record_offset":2528,"ancestor_record_offset":2328,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::device::GraphicsDevice::get_vk_buffer
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":705,"record_offset":2568,"parent_record_offset":2528,"ancestor_record_offset":2328,"inlinee":39510,"source_line":489,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::dynamic_constants::impl$0::into_uniform_buffer_binding::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":705,"record_offset":2528,"parent_record_offset":2496,"ancestor_record_offset":2328,"inlinee":39913,"source_line":53,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::dynamic_constants::impl$0::into_uniform_buffer_binding::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 53
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9BE9
// 首条位置: {"module":705,"inlinee":39913}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":705,"record_offset":2600,"parent_record_offset":2528,"ancestor_record_offset":2528,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":705,"record_offset":2628,"parent_record_offset":2528,"ancestor_record_offset":2528,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::get_vk_buffer
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":705,"record_offset":2568,"parent_record_offset":2528,"ancestor_record_offset":2528,"inlinee":39510,"source_line":489,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":705,"record_offset":2652,"parent_record_offset":2628,"ancestor_record_offset":2528,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_and_storage_buffer_binding
// 记录形式: compiled_procedure; instances=1
// 源码行: 66
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":705,"record_offset":2696,"rva":12591984,"va":5381301104}
// 编译实例 RVAs: 0xc02370
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dynamic_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-130954","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381301104,"raw_hex":"e87600feff","size":5,"soft_fail":false,"target_va":5381170352,"va":5381301301,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418aeba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dynamic_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13289455","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381301104,"raw_hex":"e8efc7ca00","size":5,"soft_fail":false,"target_va":5394590624,"va":5381301164,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30240511","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381301104,"raw_hex":"e8ff6ecd01","size":5,"soft_fail":false,"target_va":5411541776,"va":5381301260,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::GpuOwned::map_tracker
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":705,"record_offset":2876,"parent_record_offset":2696,"ancestor_record_offset":2696,"inlinee":39911,"source_line":507,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":705,"record_offset":2984,"parent_record_offset":2908,"ancestor_record_offset":2696,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":705,"record_offset":3012,"parent_record_offset":2908,"ancestor_record_offset":2696,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::device::GraphicsDevice::get_vk_buffer
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":705,"record_offset":2952,"parent_record_offset":2908,"ancestor_record_offset":2696,"inlinee":39510,"source_line":489,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::dynamic_constants::impl$0::into_uniform_and_storage_buffer_binding::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":705,"record_offset":2908,"parent_record_offset":2876,"ancestor_record_offset":2696,"inlinee":39915,"source_line":75,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::dynamic_constants::impl$0::into_uniform_and_storage_buffer_binding::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 75
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9BEB
// 首条位置: {"module":705,"inlinee":39915}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":705,"record_offset":2984,"parent_record_offset":2908,"ancestor_record_offset":2908,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":705,"record_offset":3012,"parent_record_offset":2908,"ancestor_record_offset":2908,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::get_vk_buffer
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":705,"record_offset":2952,"parent_record_offset":2908,"ancestor_record_offset":2908,"inlinee":39510,"source_line":489,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":705,"record_offset":3036,"parent_record_offset":3012,"ancestor_record_offset":2908,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_buffer_binding_and_data
// 记录形式: compiled_procedure; instances=1
// 源码行: 97
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":705,"record_offset":3080,"rva":12592208,"va":5381301328}
// 编译实例 RVAs: 0xc02450
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dynamic_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-131246","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381301328,"raw_hex":"e852fffdff","size":5,"soft_fail":false,"target_va":5381170352,"va":5381301593,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1418aeba0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dynamic_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13289227","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381301328,"raw_hex":"e80bc7ca00","size":5,"soft_fail":false,"target_va":5394590624,"va":5381301392,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30240225","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381301328,"raw_hex":"e8e16dcd01","size":5,"soft_fail":false,"target_va":5411541776,"va":5381301546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::GpuOwned::map_tracker
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":705,"record_offset":3264,"parent_record_offset":3080,"ancestor_record_offset":3080,"inlinee":39911,"source_line":507,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":705,"record_offset":3456,"parent_record_offset":3080,"ancestor_record_offset":3080,"inlinee":38984,"source_line":109,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":705,"record_offset":3364,"parent_record_offset":3292,"ancestor_record_offset":3080,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":705,"record_offset":3392,"parent_record_offset":3292,"ancestor_record_offset":3080,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":705,"record_offset":3480,"parent_record_offset":3456,"ancestor_record_offset":3080,"inlinee":38986,"source_line":2956,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":705,"record_offset":3504,"parent_record_offset":3480,"ancestor_record_offset":3080,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::device::GraphicsDevice::get_vk_buffer
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":705,"record_offset":3332,"parent_record_offset":3292,"ancestor_record_offset":3080,"inlinee":39510,"source_line":489,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::dynamic_constants::impl$0::into_uniform_buffer_binding_and_data::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":705,"record_offset":3292,"parent_record_offset":3264,"ancestor_record_offset":3080,"inlinee":39917,"source_line":101,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::dynamic_constants::impl$0::into_uniform_buffer_binding_and_data::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x9BED
// 首条位置: {"module":705,"inlinee":39917}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":705,"record_offset":3364,"parent_record_offset":3292,"ancestor_record_offset":3292,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":705,"record_offset":3392,"parent_record_offset":3292,"ancestor_record_offset":3292,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::get_vk_buffer
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":705,"record_offset":3332,"parent_record_offset":3292,"ancestor_record_offset":3292,"inlinee":39510,"source_line":489,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":705,"record_offset":3416,"parent_record_offset":3392,"ancestor_record_offset":3292,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::dynamic_constants::DynamicConstants::new
// 记录形式: compiled_procedure; instances=1
// 源码行: 129
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":705,"record_offset":3544,"rva":12592496,"va":5381301616}
// 编译实例 RVAs: 0xc02570
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dynamic_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-131713","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381301616,"raw_hex":"e87ffdfdff","size":5,"soft_fail":false,"target_va":5381170352,"va":5381302060,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30239771","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381301616,"raw_hex":"e81b6ccd01","size":5,"soft_fail":false,"target_va":5411541776,"va":5381302000,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30179675","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381301616,"raw_hex":"e85b81cc01","size":5,"soft_fail":false,"target_va":5411481622,"va":5381301942,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30240161","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381301616,"raw_hex":"e8a16dcd01","size":5,"soft_fail":false,"target_va":5411542064,"va":5381301898,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::Buffer::mapped_slice_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-186619","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381301616,"raw_hex":"e80527fdff","size":5,"soft_fail":false,"target_va":5381115088,"va":5381301702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::dynamic_constants::DynamicConstants::allocate_buffer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1106","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381301616,"raw_hex":"e852040000","size":5,"soft_fail":false,"target_va":5381302768,"va":5381301657,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*1288(%rdi)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381301616,"raw_hex":"ff9708050000","size":6,"soft_fail":false,"va":5381301807,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::num::impl$11::is_power_of_two
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":705,"record_offset":3728,"parent_record_offset":3544,"ancestor_record_offset":3544,"inlinee":39918,"source_line":3310,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":705,"record_offset":3756,"parent_record_offset":3544,"ancestor_record_offset":3544,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":705,"record_offset":4060,"parent_record_offset":3544,"ancestor_record_offset":3544,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::Buffer::device_address
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":705,"record_offset":3792,"parent_record_offset":3544,"ancestor_record_offset":3544,"inlinee":38992,"source_line":39,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::limits
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":705,"record_offset":3700,"parent_record_offset":3544,"ancestor_record_offset":3544,"inlinee":39812,"source_line":686,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::get_buffer_device_address
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":705,"record_offset":4028,"parent_record_offset":3792,"ancestor_record_offset":3544,"inlinee":38973,"source_line":710,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::BufferDeviceAddressInfo::buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":705,"record_offset":4000,"parent_record_offset":3792,"ancestor_record_offset":3544,"inlinee":38972,"source_line":27762,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":705,"record_offset":3964,"parent_record_offset":3792,"ancestor_record_offset":3544,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":705,"record_offset":4084,"parent_record_offset":4060,"ancestor_record_offset":3544,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":705,"record_offset":3876,"parent_record_offset":3832,"ancestor_record_offset":3544,"inlinee":11948,"source_line":14,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":705,"record_offset":3932,"parent_record_offset":3832,"ancestor_record_offset":3544,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":705,"record_offset":3904,"parent_record_offset":3832,"ancestor_record_offset":3544,"inlinee":11950,"source_line":32,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":705,"record_offset":3832,"parent_record_offset":3792,"ancestor_record_offset":3544,"inlinee":11960,"source_line":315,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::dynamic_constants::DynamicConstants::push
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 144
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x2718
// 首条位置: {"module":66,"inlinee":10008}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":66,"record_offset":9920,"parent_record_offset":9768,"ancestor_record_offset":9768,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::write
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/dynamic_constants.rs
//     evidence={"module":66,"record_offset":9816,"parent_record_offset":9768,"ancestor_record_offset":9768,"inlinee":10010,"source_line":26,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":66,"record_offset":9884,"parent_record_offset":9848,"ancestor_record_offset":9768,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":66,"record_offset":9996,"parent_record_offset":9968,"ancestor_record_offset":9768,"inlinee":6218,"source_line":220,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=12
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":66,"record_offset":9944,"parent_record_offset":9920,"ancestor_record_offset":9768,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":66,"record_offset":10020,"parent_record_offset":9996,"ancestor_record_offset":9768,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":66,"record_offset":9848,"parent_record_offset":9816,"ancestor_record_offset":9768,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":66,"record_offset":9968,"parent_record_offset":9944,"ancestor_record_offset":9768,"inlinee":6217,"source_line":550,"archive":"_pdb/B/modules/00066.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::dynamic_constants::DynamicConstants::push_slice
// 记录形式: inline_instance, inlinee_declaration; instances=14
// 源码行: 150
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x67F6
// 首条位置: {"module":495,"inlinee":26614}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":495,"record_offset":79196,"parent_record_offset":79024,"ancestor_record_offset":79024,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::copy_from_slice
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\mod.rs
//     evidence={"module":495,"record_offset":79128,"parent_record_offset":79024,"ancestor_record_offset":79024,"inlinee":5246,"source_line":3720,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::bytes::slice_as_bytes
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/bytes.rs
//     evidence={"module":495,"record_offset":79072,"parent_record_offset":79024,"ancestor_record_offset":79024,"inlinee":8265,"source_line":8,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":495,"record_offset":79164,"parent_record_offset":79128,"ancestor_record_offset":79024,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::manually_drop::ManuallyDrop::take
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\manually_drop.rs
//     evidence={"module":495,"record_offset":79272,"parent_record_offset":79244,"ancestor_record_offset":79024,"inlinee":6218,"source_line":220,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::size_of_val
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":495,"record_offset":79096,"parent_record_offset":79072,"ancestor_record_offset":79024,"inlinee":8266,"source_line":338,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=28
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":495,"record_offset":79220,"parent_record_offset":79196,"ancestor_record_offset":79024,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":495,"record_offset":79296,"parent_record_offset":79272,"ancestor_record_offset":79024,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$25::drop
//     owner=private_source_ownership_candidate; count=7
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":495,"record_offset":79244,"parent_record_offset":79220,"ancestor_record_offset":79024,"inlinee":6217,"source_line":550,"archive":"_pdb/B/modules/00495.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::dynamic_constants::DynamicConstants::allocate
// 记录形式: compiled_procedure; instances=1
// 源码行: 162
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":705,"record_offset":4148,"rva":12592960,"va":5381302080}
// 编译实例 RVAs: 0xc02740
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140be24b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dynamic_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-131878","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381302080,"raw_hex":"e8dafcfdff","size":5,"soft_fail":false,"target_va":5381170352,"va":5381302225,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30239093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381302080,"raw_hex":"e87569cd01","size":5,"soft_fail":false,"target_va":5411541776,"va":5381302678,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::assert_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30178996","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381302080,"raw_hex":"e8b47ecc01","size":5,"soft_fail":false,"target_va":5411481622,"va":5381302621,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30239513","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381302080,"raw_hex":"e8196bcd01","size":5,"soft_fail":false,"target_va":5411542064,"va":5381302546,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_end_index_len_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30240523","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381302080,"raw_hex":"e80b6fcd01","size":5,"soft_fail":false,"target_va":5411543104,"va":5381302576,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::slice::index::slice_index_order_fail
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t30240557","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381302080,"raw_hex":"e82d6fcd01","size":5,"soft_fail":false,"target_va":5411543120,"va":5381302558,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::buffer::Buffer::mapped_slice_mut
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-187286","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381302080,"raw_hex":"e86a24fdff","size":5,"soft_fail":false,"target_va":5381115088,"va":5381302369,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] rhapsody::raw::dynamic_constants::DynamicConstants::allocate_buffer
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t551","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381302080,"raw_hex":"e827020000","size":5,"soft_fail":false,"target_va":5381302768,"va":5381302212,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_indirect_call_unresolved] unresolved_indirect_call
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t*1288(%r14)","direct":false,"event":"call","llvm_opcode":"CALL64m","range_va":5381302080,"raw_hex":"41ff9608050000","size":7,"soft_fail":false,"va":5381302352,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::Ord::max
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":705,"record_offset":4316,"parent_record_offset":4148,"ancestor_record_offset":4148,"inlinee":6234,"source_line":998,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":705,"record_offset":4724,"parent_record_offset":4148,"ancestor_record_offset":4148,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":705,"record_offset":4372,"parent_record_offset":4148,"ancestor_record_offset":4148,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_direct] core::slice::index::impl$1::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":705,"record_offset":4760,"parent_record_offset":4148,"ancestor_record_offset":4148,"inlinee":5238,"source_line":26,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::Buffer::device_address
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":705,"record_offset":4456,"parent_record_offset":4148,"ancestor_record_offset":4148,"inlinee":38992,"source_line":39,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::ManagedHandle::into_gpu_owned
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":705,"record_offset":5056,"parent_record_offset":4148,"ancestor_record_offset":4148,"inlinee":39919,"source_line":237,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::managed::handle::impl$18::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":705,"record_offset":4924,"parent_record_offset":4148,"ancestor_record_offset":4148,"inlinee":9614,"source_line":256,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] ash::device::Device::get_buffer_device_address
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/device.rs
//     evidence={"module":705,"record_offset":4692,"parent_record_offset":4456,"ancestor_record_offset":4148,"inlinee":38973,"source_line":710,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] ash::vk::definitions::BufferDeviceAddressInfo::buffer
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"ash","version":"0.38.0+1.3.281","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ash-0.38.0+1.3.281/src/vk/definitions.rs
//     evidence={"module":705,"record_offset":4664,"parent_record_offset":4456,"ancestor_record_offset":4148,"inlinee":38972,"source_line":27762,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_sub
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":705,"record_offset":4832,"parent_record_offset":4792,"ancestor_record_offset":4148,"inlinee":4611,"source_line":653,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":705,"record_offset":4628,"parent_record_offset":4456,"ancestor_record_offset":4148,"inlinee":4455,"source_line":975,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":705,"record_offset":4396,"parent_record_offset":4372,"ancestor_record_offset":4148,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_mut_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":705,"record_offset":4884,"parent_record_offset":4860,"ancestor_record_offset":4148,"inlinee":5243,"source_line":94,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_mut_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":705,"record_offset":4860,"parent_record_offset":4792,"ancestor_record_offset":4148,"inlinee":5242,"source_line":112,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$4::index_mut
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":705,"record_offset":4792,"parent_record_offset":4760,"ancestor_record_offset":4148,"inlinee":5308,"source_line":444,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicI32::fetch_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":705,"record_offset":4992,"parent_record_offset":4952,"ancestor_record_offset":4148,"inlinee":38986,"source_line":2956,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_add
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":705,"record_offset":5016,"parent_record_offset":4992,"ancestor_record_offset":4148,"inlinee":4128,"source_line":3760,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::GpuOwned::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":705,"record_offset":5080,"parent_record_offset":5056,"ancestor_record_offset":4148,"inlinee":13502,"source_line":469,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$32::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":705,"record_offset":4540,"parent_record_offset":4496,"ancestor_record_offset":4148,"inlinee":11948,"source_line":14,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$40::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":705,"record_offset":4596,"parent_record_offset":4496,"ancestor_record_offset":4148,"inlinee":11952,"source_line":23,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$47::eq
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":705,"record_offset":4568,"parent_record_offset":4496,"ancestor_record_offset":4148,"inlinee":11950,"source_line":32,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::handle::impl$5::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/handle.rs
//     evidence={"module":705,"record_offset":4952,"parent_record_offset":4924,"ancestor_record_offset":4148,"inlinee":38984,"source_line":109,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] rhapsody::raw::managed::resources::impl$4::get
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/managed/resources.rs
//     evidence={"module":705,"record_offset":4496,"parent_record_offset":4456,"ancestor_record_offset":4148,"inlinee":11960,"source_line":315,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: rhapsody::raw::dynamic_constants::DynamicConstants::allocate_buffer
// 记录形式: compiled_procedure; instances=1
// 源码行: 200
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":705,"record_offset":5116,"rva":12593648,"va":5381302768}
// 编译实例 RVAs: 0xc029f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t1341543","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381302768,"raw_hex":"e867781400","size":5,"soft_fail":false,"target_va":5382644528,"va":5381302980,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14067dd80
//     owner=unresolved_namespace; count=1
//     target_aliases=69 candidates; complete list: dynamic_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5786832","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381302768,"raw_hex":"e830b3a7ff","size":5,"soft_fail":false,"target_va":5375516032,"va":5381302859,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1407cea70
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: dynamic_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4407660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381302768,"raw_hex":"e894bebcff","size":5,"soft_fail":false,"target_va":5376895600,"va":5381303255,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140c038c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: dynamic_constants.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t3500","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381302768,"raw_hex":"e8ac0d0000","size":5,"soft_fail":false,"target_va":5381306560,"va":5381303055,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t30239739","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381302768,"raw_hex":"e8fb6bcd01","size":5,"soft_fail":false,"target_va":5411542880,"va":5381303136,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] leekspin::ringbuffer::RingBuffer::append
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"leekspin","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t13363536","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5381302768,"raw_hex":"e850e9cb00","size":5,"soft_fail":false,"target_va":5394666368,"va":5381302827,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\cmp.rs
//     evidence={"module":705,"record_offset":5408,"parent_record_offset":5116,"ancestor_record_offset":5116,"inlinee":5930,"source_line":1389,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":705,"record_offset":5764,"parent_record_offset":5116,"ancestor_record_offset":5116,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":705,"record_offset":5440,"parent_record_offset":5116,"ancestor_record_offset":5116,"inlinee":5932,"source_line":84,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":705,"record_offset":5324,"parent_record_offset":5116,"ancestor_record_offset":5116,"inlinee":5929,"source_line":1321,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::leek::allocate_dynamic_constants_chunk
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/leekspin/src/lib.rs
//     evidence={"module":705,"record_offset":5292,"parent_record_offset":5116,"ancestor_record_offset":5116,"inlinee":39368,"source_line":32,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::buffer::BufferDesc::alignment
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/buffer.rs
//     evidence={"module":705,"record_offset":5636,"parent_record_offset":5116,"ancestor_record_offset":5116,"inlinee":39000,"source_line":130,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_direct] rhapsody::raw::device::GraphicsDevice::limits
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"rhapsody","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/rhapsody/src/raw/device.rs
//     evidence={"module":705,"record_offset":5608,"parent_record_offset":5116,"ancestor_record_offset":5116,"inlinee":39812,"source_line":686,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$6::try_into
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\mod.rs
//     evidence={"module":705,"record_offset":5668,"parent_record_offset":5636,"ancestor_record_offset":5116,"inlinee":9618,"source_line":799,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::num::ptr_try_from_impls::impl$1::try_from
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\convert\num.rs
//     evidence={"module":705,"record_offset":5692,"parent_record_offset":5668,"ancestor_record_offset":5116,"inlinee":38980,"source_line":255,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::file
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":705,"record_offset":5492,"parent_record_offset":5464,"ancestor_record_offset":5116,"inlinee":5935,"source_line":129,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] core::panic::location::Location::line
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\panic\location.rs
//     evidence={"module":705,"record_offset":5520,"parent_record_offset":5464,"ancestor_record_offset":5116,"inlinee":5936,"source_line":154,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::expect
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":705,"record_offset":5724,"parent_record_offset":5636,"ancestor_record_offset":5116,"inlinee":6013,"source_line":1055,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":705,"record_offset":5348,"parent_record_offset":5324,"ancestor_record_offset":5116,"inlinee":5823,"source_line":2698,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\sync\atomic.rs
//     evidence={"module":705,"record_offset":5372,"parent_record_offset":5348,"ancestor_record_offset":5116,"inlinee":4182,"source_line":3727,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":705,"record_offset":5548,"parent_record_offset":5464,"ancestor_record_offset":5116,"inlinee":5938,"source_line":1033,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":705,"record_offset":5464,"parent_record_offset":5440,"ancestor_record_offset":5116,"inlinee":5933,"source_line":56,"archive":"_pdb/B/modules/00705.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$52::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":705,"record_offset":5572,"parent_record_offset":5548,"ancestor_record_offset":5116,"inlinee":5940,"source_line":766,"archive":"_pdb/B/modules/00705.jsonl.gz"}
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
// 完整结构及原始类型记录: dynamic_constants.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::dynamic_constants::DynamicConstants
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 6; 过程引用数: 9
// [PDB 泛型关系，非字段关系] {"type": "lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,rhapsody::raw::dynamic_constants::DynamicConstants>", "constructor": "lock_api::mutex::MutexGuard", "arguments": ["parking_lot::raw_mutex::RawMutex", "rhapsody::raw::dynamic_constants::DynamicConstants"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,rhapsody::raw::dynamic_constants::DynamicConstants> >", "constructor": "core::ptr::drop_in_place", "arguments": ["lock_api::mutex::MutexGuard<parking_lot::raw_mutex::RawMutex,rhapsody::raw::dynamic_constants::DynamicConstants>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] rhapsody::raw::dynamic_constants::DynamicConstantsChunk
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 8; 过程引用数: 5
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: dynamic_constants.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: dynamic_constants.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 3; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >,bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >", "bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::terrain_height::TerrainHeightsData<country_core::resources::terrain_height::TerrainHeightSsbo> >
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::world_raster_defs::PathRaster>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>,bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustClosed>)
// 候选参数类型: ["bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>", "bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustClosed>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::feedback_report_resources::FeedbackReportResources>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::resources::gui::EscapeMenuJustClosed>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::shaderwatch::inner::ShaderWatch>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::DynamicConstants::allocate
// 信息 ID: b9db9c2ded76406a17442349
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 25 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x16210a -> 0xc02740 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0> @ 0x164b2e -> 0xc02740 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<tuple$<>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::execute_presentation_render_graph::closure_env$0> @ 0x1668f8 -> 0xc02740 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke @ 0x8c763e -> 0xc02740 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke @ 0x8c7bfa -> 0xc02740 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::draw_builder::DrawBuilder::new @ 0x905d2f -> 0xc02740 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args @ 0x90aa6d -> 0xc02740 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args @ 0x90ab08 -> 0xc02740 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args @ 0x90ab8b -> 0xc02740 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args @ 0x90ac09 -> 0xc02740 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::rg_buffer_copy_from_slice<u32> @ 0x90db16 -> 0xc02740 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::uniform_binder::UniformBinder::new @ 0x96b467 -> 0xc02740 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::render::nani::cull_shaders::NaniCullAndBucketFlowers> @ 0x15fd1cf -> 0xc02740 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResAllDof> @ 0x15fdadf -> 0xc02740 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::bin_lights::BinLights> @ 0x15fe3ef -> 0xc02740 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::vermeer::GenerateStrokes> @ 0x15fecff -> 0xc02740 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::DynamicConstants::allocate_buffer
// 信息 ID: 79cd7c29cf8ebd051b7ec892
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::raw::dynamic_constants::DynamicConstants::new @ 0xc02599 -> 0xc029f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::dynamic_constants::DynamicConstants::allocate @ 0xc027c4 -> 0xc029f0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::DynamicConstants::new
// 信息 ID: eb1c0fe27af6278342b7e1db
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x59ecb6 -> 0xc02570 (direct_call); 同地址别名未裁定。
// [静态直接引用] tiny_glade::run_game::run_game_thread @ 0x5a50b8 -> 0xc02570 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::DynamicConstants::push
// 信息 ID: e5a2872eb34c7c8aaf27129f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0>; module=66, inline_record=9768; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0>; module=66, inline_record=31200; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::state::RenderGraphState::compile<tuple$<>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::execute_presentation_render_graph::closure_env$0>; module=66, inline_record=45284; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::DynamicConstants::push_slice
// 信息 ID: 130789257274c848102f1a49
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 7 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke; module=495, inline_record=79024; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args; module=503, inline_record=71276; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args; module=503, inline_record=71624; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args; module=503, inline_record=71972; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::reset_indirect_args::reset_indirect_args; module=503, inline_record=72320; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::rg_buffer_copy_from_slice<u32>; module=503, inline_record=89428; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::dof::calc_dof; module=549, inline_record=7512; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::DynamicConstantsChunk::device_address
// 信息 ID: 62588c837ed2e799af3a03ae
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke @ 0x8c7801 -> 0xc02280 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke @ 0x8c7c9b -> 0xc02280 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::raw::buffer::Buffer::mapped_slice_mut @ 0xbd4d26 -> 0xc02280 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_gpu_lifetime_tracker
// 信息 ID: 594931e70ef4870fbd94487c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke @ 0x8c78db -> 0xc02290 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke @ 0x8c7d03 -> 0xc02290 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_and_storage_buffer_binding
// 信息 ID: b2fd87a28049a3e2ce7f471a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0> @ 0x162193 -> 0xc02370 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0> @ 0x164bb7 -> 0xc02370 (direct_call); 同地址别名未裁定。
// [静态直接引用] rhapsody::rg::state::RenderGraphState::compile<tuple$<>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::execute_presentation_render_graph::closure_env$0> @ 0x166981 -> 0xc02370 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_buffer_binding
// 信息 ID: 559d14fd0f141e8ef83979b2
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 0 条；闭包环境类型引用 0 条。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_buffer_binding_and_data
// 信息 ID: 98c4d83ab8906beb2bf345b6
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 15 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::render::pass_builder::draw_builder::DrawBuilder::new @ 0x905d45 -> 0xc02450 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::uniform_binder::UniformBinder::new @ 0x96b47a -> 0xc02450 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::render::nani::cull_shaders::NaniCullAndBucketFlowers> @ 0x15fd1e2 -> 0xc02450 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResAllDof> @ 0x15fdaf2 -> 0xc02450 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::bin_lights::BinLights> @ 0x15fe402 -> 0xc02450 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::vermeer::GenerateStrokes> @ 0x15fed12 -> 0xc02450 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::render::nani::cull_shaders::NaniCullAndBucketMain> @ 0x15ff622 -> 0xc02450 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::dof::HalfResDof> @ 0x15fff32 -> 0xc02450 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<dyn$<country_core::render::pass_builder::shader_uri::AsComputeShaderSource> > > @ 0x1600833 -> 0xc02450 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::taa::TgsrMain> @ 0x160113e -> 0xc02450 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::render::nani::cull_shaders::NaniCullAndBucketBush> @ 0x1601a42 -> 0xc02450 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::render::nani::cull_shaders::NaniCullAndBucketWoodenPlank> @ 0x1602352 -> 0xc02450 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::GaussBlur> @ 0x1602c5e -> 0xc02450 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> > @ 0x1603575 -> 0xc02450 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::pass_builder::compute_pass_builder::ComputePass::new<country_core::render::nani::cull_shaders::NaniCullAndBucketRoofTile> @ 0x1603e82 -> 0xc02450 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::DynamicConstantsChunk::with_data
// 信息 ID: 3e1faeedb9bf0aefb51a315d
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::rasterize_terrain_stroke; module=495, inline_record=75088; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::DynamicConstantsChunk::write
// 信息 ID: fb2933af8ecb58fa39ea2ad5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_main_render_graph::closure_env$0>; module=66, inline_record=9816; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::state::RenderGraphState::compile<rhapsody::rg::resource::ResourceExportHandle<rhapsody::raw::texture::Texture>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::compile_loading_screen_render_graph::closure_env$0>; module=66, inline_record=31248; 不是独立调用边。
// [PDB 内联宿主] rhapsody::rg::state::RenderGraphState::compile<tuple$<>,country_core::render::pass_builder::pass_resources::PassResources,tiny_glade::render_loop::execute_presentation_render_graph::closure_env$0>; module=66, inline_record=45332; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::impl$0::into_uniform_and_storage_buffer_binding::closure$0
// 信息 ID: 4e6f3d0bcf1a41a6546d7718
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::dynamic_constants::impl$0::into_uniform_and_storage_buffer_binding", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_and_storage_buffer_binding; module=705, inline_record=2908; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::impl$0::into_uniform_buffer_binding::closure$0
// 信息 ID: 8eeef279a22f3d5949a8de29
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::dynamic_constants::impl$0::into_uniform_buffer_binding", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_buffer_binding; module=705, inline_record=2528; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: rhapsody::raw::dynamic_constants::impl$0::into_uniform_buffer_binding_and_data::closure$0
// 信息 ID: 4e1aed25e6537e58c40c6730
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "rhapsody::raw::dynamic_constants::impl$0::into_uniform_buffer_binding_and_data", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] rhapsody::raw::dynamic_constants::DynamicConstantsChunk::into_uniform_buffer_binding_and_data; module=705, inline_record=3292; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
