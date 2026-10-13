// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: B
// Original source: crates/country-core/src/utils/audio_utils.rs
// PDB SHA-256: 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911
// Source SHA-256: 04324e4fd55279a98863afbe5d7f950361c390d8eca7c313aaa95dcec3eb3fa5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: audio_utils.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/B/
// 编译单元: 484, 507, 513, 531, 532, 535, 548, 551, 556, 561, 597, 1175, 1596, 1606, 1610, 1611, 1643, 1734, 1738
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.86"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use libfmod; // namespace candidate
//   package identities: [{"name":"libfmod","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.222.6"}]
// use std; // namespace candidate
//   package identities: []
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::utils::audio_utils::AudioLoop::start_if_not_playing
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 0, 134
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7AFB
// 首条位置: {"module":548,"record_offset":77304,"rva":10852016,"va":5379561136}
// 编译实例 RVAs: 0xa596b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a526b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: audio_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28989","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379561136,"raw_hex":"e8c38effff","size":5,"soft_fail":false,"target_va":5379532464,"va":5379561448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a57670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: audio_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-8356","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379561136,"raw_hex":"e85cdfffff","size":5,"soft_fail":false,"target_va":5379552880,"va":5379561231,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31980971","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379561136,"raw_hex":"e8abfde701","size":5,"soft_fail":false,"target_va":5411542296,"va":5379561320,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31981465","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379561136,"raw_hex":"e899ffe701","size":5,"soft_fail":false,"target_va":5411542880,"va":5379561410,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::start
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libfmod","version":"2.222.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10345365","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379561136,"raw_hex":"e895db9d00","size":5,"soft_fail":false,"target_va":5389906640,"va":5379561270,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":548,"record_offset":77460,"parent_record_offset":77304,"ancestor_record_offset":77304,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::start
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":548,"record_offset":77664,"parent_record_offset":77304,"ancestor_record_offset":77304,"inlinee":27234,"source_line":505,"archive":"_pdb/B/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":548,"record_offset":77700,"parent_record_offset":77664,"ancestor_record_offset":77304,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":548,"record_offset":77728,"parent_record_offset":77664,"ancestor_record_offset":77304,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":548,"record_offset":77488,"parent_record_offset":77460,"ancestor_record_offset":77304,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=7
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":548,"record_offset":77516,"parent_record_offset":77488,"ancestor_record_offset":77304,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00548.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::audio_utils::AudioLoop::stop
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":548,"record_offset":76588,"rva":10851376,"va":5379560496}
// 编译实例 RVAs: 0xa59430
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a526b0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: audio_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-28607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379560496,"raw_hex":"e84190ffff","size":5,"soft_fail":false,"target_va":5379532464,"va":5379561066,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a57670
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: audio_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-7788","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379560496,"raw_hex":"e894e1ffff","size":5,"soft_fail":false,"target_va":5379552880,"va":5379560663,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31981352","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379560496,"raw_hex":"e828ffe701","size":5,"soft_fail":false,"target_va":5411542296,"va":5379560939,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t31982081","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379560496,"raw_hex":"e80102e801","size":5,"soft_fail":false,"target_va":5411542880,"va":5379560794,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::stop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"libfmod","version":"2.222.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10346301","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379560496,"raw_hex":"e83ddf9d00","size":5,"soft_fail":false,"target_va":5389907008,"va":5379560702,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::elapsed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3187857","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379560496,"raw_hex":"e891a43000","size":5,"soft_fail":false,"target_va":5382748432,"va":5379560570,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::time::Instant::now
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t3187073","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379560496,"raw_hex":"e881a13000","size":5,"soft_fail":false,"target_va":5382747616,"va":5379560538,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":548,"record_offset":76764,"parent_record_offset":76588,"ancestor_record_offset":76588,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::as_millis
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\time.rs
//     evidence={"module":548,"record_offset":76732,"parent_record_offset":76588,"ancestor_record_offset":76588,"inlinee":18155,"source_line":558,"archive":"_pdb/B/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::stop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":548,"record_offset":76980,"parent_record_offset":76588,"ancestor_record_offset":76588,"inlinee":27233,"source_line":532,"archive":"_pdb/B/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":548,"record_offset":77012,"parent_record_offset":76980,"ancestor_record_offset":76588,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":548,"record_offset":77040,"parent_record_offset":76980,"ancestor_record_offset":76588,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":548,"record_offset":76800,"parent_record_offset":76764,"ancestor_record_offset":76588,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00548.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":548,"record_offset":76836,"parent_record_offset":76800,"ancestor_record_offset":76588,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00548.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::audio_utils::AudioLoopDesc::new
// 记录形式: inline_instance, inlinee_declaration; instances=88
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x6902
// 首条位置: {"module":484,"inlinee":26882}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::audio_resources::AudioCore::instantiate_loop
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=3
// 源码行: 49
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x77D7
// 首条位置: {"module":535,"inlinee":30679}
// 编译实例 RVAs: 0xa59220
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140899740
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: audio_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1833824","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379559968,"raw_hex":"e8a004e4ff","size":5,"soft_fail":false,"target_va":5377726272,"va":5379560091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a526b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: audio_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-27767","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379559968,"raw_hex":"e88993ffff","size":5,"soft_fail":false,"target_va":5379532464,"va":5379560226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1412fafb0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: audio_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t9051430","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379559968,"raw_hex":"e8261d8a00","size":5,"soft_fail":false,"target_va":5388611504,"va":5379560069,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t3094660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379559968,"raw_hex":"e884382f00","size":5,"soft_fail":false,"target_va":5382655024,"va":5379560359,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31981987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379559968,"raw_hex":"e8a301e801","size":5,"soft_fail":false,"target_va":5411542296,"va":5379560304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::get_parameter_by_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libfmod","version":"2.222.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t10348373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379559968,"raw_hex":"e855e79d00","size":5,"soft_fail":false,"target_va":5389908576,"va":5379560198,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":535,"record_offset":27948,"parent_record_offset":23400,"ancestor_record_offset":23400,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":535,"record_offset":23464,"parent_record_offset":23400,"ancestor_record_offset":23400,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":535,"record_offset":27920,"parent_record_offset":23400,"ancestor_record_offset":23400,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":548,"record_offset":76116,"parent_record_offset":75636,"ancestor_record_offset":75636,"inlinee":4231,"source_line":2012,"archive":"_pdb/B/modules/00548.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::impl$10::has_parameter
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":535,"record_offset":28044,"parent_record_offset":23400,"ancestor_record_offset":23400,"inlinee":30680,"source_line":621,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":535,"record_offset":24260,"parent_record_offset":24236,"ancestor_record_offset":23400,"inlinee":4366,"source_line":115,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":535,"record_offset":24236,"parent_record_offset":24180,"ancestor_record_offset":23400,"inlinee":4365,"source_line":265,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$8::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\boxed.rs
//     evidence={"module":535,"record_offset":24640,"parent_record_offset":24616,"ancestor_record_offset":23400,"inlinee":4364,"source_line":1676,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::impl$2::drop
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\ffi\c_str.rs
//     evidence={"module":535,"record_offset":24588,"parent_record_offset":24564,"ancestor_record_offset":23400,"inlinee":7853,"source_line":696,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":535,"record_offset":24208,"parent_record_offset":24180,"ancestor_record_offset":23400,"inlinee":4628,"source_line":522,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":535,"record_offset":24180,"parent_record_offset":24156,"ancestor_record_offset":23400,"inlinee":4627,"source_line":750,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=15
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":535,"record_offset":24156,"parent_record_offset":24132,"ancestor_record_offset":23400,"inlinee":4626,"source_line":400,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$15::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":535,"record_offset":23656,"parent_record_offset":23632,"ancestor_record_offset":23400,"inlinee":4671,"source_line":3423,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::drop
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":535,"record_offset":23760,"parent_record_offset":23720,"ancestor_record_offset":23400,"inlinee":4970,"source_line":3781,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::try_process
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\adapters\mod.rs
//     evidence={"module":535,"record_offset":23552,"parent_record_offset":23508,"ancestor_record_offset":23400,"inlinee":11032,"source_line":152,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\collect.rs
//     evidence={"module":535,"record_offset":23680,"parent_record_offset":23656,"ancestor_record_offset":23400,"inlinee":4756,"source_line":322,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\iter\traits\iterator.rs
//     evidence={"module":535,"record_offset":23632,"parent_record_offset":23608,"ancestor_record_offset":23400,"inlinee":4669,"source_line":1967,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\mem\mod.rs
//     evidence={"module":535,"record_offset":23944,"parent_record_offset":23916,"ancestor_record_offset":23400,"inlinee":4437,"source_line":860,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\num\uint_macros.rs
//     evidence={"module":535,"record_offset":27544,"parent_record_offset":27516,"ancestor_record_offset":23400,"inlinee":4629,"source_line":933,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":535,"record_offset":28072,"parent_record_offset":28044,"ancestor_record_offset":23400,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":535,"record_offset":23916,"parent_record_offset":23872,"ancestor_record_offset":23400,"inlinee":4908,"source_line":1726,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=61
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":535,"record_offset":23720,"parent_record_offset":23552,"ancestor_record_offset":23400,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":535,"record_offset":23972,"parent_record_offset":23944,"ancestor_record_offset":23400,"inlinee":4353,"source_line":1341,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":535,"record_offset":24000,"parent_record_offset":23944,"ancestor_record_offset":23400,"inlinee":4354,"source_line":1559,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":535,"record_offset":28100,"parent_record_offset":28044,"ancestor_record_offset":23400,"inlinee":6884,"source_line":564,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":535,"record_offset":25252,"parent_record_offset":23872,"ancestor_record_offset":23400,"inlinee":5346,"source_line":1103,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$25::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":535,"record_offset":23508,"parent_record_offset":23464,"ancestor_record_offset":23400,"inlinee":30523,"source_line":1984,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$25::from_iter::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":535,"record_offset":23608,"parent_record_offset":23552,"ancestor_record_offset":23400,"inlinee":21653,"source_line":1985,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":535,"record_offset":27976,"parent_record_offset":27948,"ancestor_record_offset":23400,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":535,"record_offset":28004,"parent_record_offset":27976,"ancestor_record_offset":23400,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::impl$13::drop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":535,"record_offset":23872,"parent_record_offset":23840,"ancestor_record_offset":23400,"inlinee":26934,"source_line":668,"archive":"_pdb/B/modules/00535.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::audio_utils::impl$1::instantiate_loop::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x120A7
// 首条位置: {"module":1610,"inlinee":73895}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::instantiate_audio
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":1610,"record_offset":99320,"parent_record_offset":99284,"ancestor_record_offset":99284,"inlinee":27239,"source_line":321,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":99688,"parent_record_offset":99664,"ancestor_record_offset":99284,"inlinee":4269,"source_line":187,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":99712,"parent_record_offset":99688,"ancestor_record_offset":99284,"inlinee":4271,"source_line":90,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\alloc.rs
//     evidence={"module":1610,"record_offset":99664,"parent_record_offset":99636,"ancestor_record_offset":99284,"inlinee":4267,"source_line":253,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":99576,"parent_record_offset":99548,"ancestor_record_offset":99284,"inlinee":4254,"source_line":184,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":99636,"parent_record_offset":99604,"ancestor_record_offset":99284,"inlinee":4256,"source_line":449,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\raw_vec.rs
//     evidence={"module":1610,"record_offset":99604,"parent_record_offset":99576,"ancestor_record_offset":99284,"inlinee":4255,"source_line":417,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1610,"record_offset":99512,"parent_record_offset":99484,"ancestor_record_offset":99284,"inlinee":4252,"source_line":161,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::hack::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1610,"record_offset":99484,"parent_record_offset":99456,"ancestor_record_offset":99284,"inlinee":4250,"source_line":110,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1610,"record_offset":99428,"parent_record_offset":99400,"ancestor_record_offset":99284,"inlinee":4247,"source_line":476,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1610,"record_offset":99456,"parent_record_offset":99428,"ancestor_record_offset":99284,"inlinee":4248,"source_line":500,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\slice.rs
//     evidence={"module":1610,"record_offset":99400,"parent_record_offset":99372,"ancestor_record_offset":99284,"inlinee":4245,"source_line":887,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\str.rs
//     evidence={"module":1610,"record_offset":99372,"parent_record_offset":99320,"ancestor_record_offset":99284,"inlinee":4243,"source_line":210,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1610,"record_offset":99548,"parent_record_offset":99512,"ancestor_record_offset":99284,"inlinee":4253,"source_line":802,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Ok
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.86","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.86/src/lib.rs
//     evidence={"module":1610,"record_offset":99844,"parent_record_offset":99320,"ancestor_record_offset":99284,"inlinee":27241,"source_line":647,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\intrinsics\mod.rs
//     evidence={"module":1610,"record_offset":99788,"parent_record_offset":99764,"ancestor_record_offset":99284,"inlinee":4274,"source_line":4412,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\const_ptr.rs
//     evidence={"module":1610,"record_offset":99764,"parent_record_offset":99512,"ancestor_record_offset":99284,"inlinee":4273,"source_line":1351,"archive":"_pdb/B/modules/01610.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::audio_resources::AudioCore::instantiate_loop_snappy
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7694
// 首条位置: {"module":531,"inlinee":30356}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$26::branch
//     owner=rust_standard_library; count=5
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\result.rs
//     evidence={"module":531,"record_offset":57060,"parent_record_offset":57036,"ancestor_record_offset":57036,"inlinee":4233,"source_line":2000,"archive":"_pdb/B/modules/00531.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::audio_utils::AudioLoop::current
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=116
// 源码行: 95
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x70AB
// 首条位置: {"module":507,"inlinee":28843}
// 编译实例 RVAs: 0xa593f0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t31981815","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379560432,"raw_hex":"e8f700e801","size":5,"soft_fail":false,"target_va":5411542296,"va":5379560476,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=106
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":507,"record_offset":33392,"parent_record_offset":33364,"ancestor_record_offset":33364,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=106
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":507,"record_offset":33424,"parent_record_offset":33392,"ancestor_record_offset":33364,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=106
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":507,"record_offset":33456,"parent_record_offset":33424,"ancestor_record_offset":33364,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/00507.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::audio_utils::AudioLoop::is_playing
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7AFC
// 首条位置: {"module":1606,"inlinee":31484}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$13::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\alloc\src\vec\mod.rs
//     evidence={"module":1606,"record_offset":95036,"parent_record_offset":95008,"ancestor_record_offset":95008,"inlinee":4358,"source_line":3357,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::is_playing
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":1606,"record_offset":95144,"parent_record_offset":95008,"ancestor_record_offset":95008,"inlinee":31448,"source_line":591,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\option.rs
//     evidence={"module":1606,"record_offset":95180,"parent_record_offset":95144,"ancestor_record_offset":95008,"inlinee":4148,"source_line":706,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\ptr\mod.rs
//     evidence={"module":1606,"record_offset":95208,"parent_record_offset":95144,"ancestor_record_offset":95008,"inlinee":4115,"source_line":523,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1606,"record_offset":95068,"parent_record_offset":95036,"ancestor_record_offset":95008,"inlinee":4360,"source_line":15,"archive":"_pdb/B/modules/01606.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\library\core\src\slice\index.rs
//     evidence={"module":1606,"record_offset":95100,"parent_record_offset":95068,"ancestor_record_offset":95008,"inlinee":4640,"source_line":272,"archive":"_pdb/B/modules/01606.jsonl.gz"}
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
// 完整结构及原始类型记录: audio_utils.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::utils::audio_utils::AudioLoop
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 7
// [PDB 泛型关系，非字段关系] {"type": "array$<country_core::utils::audio_utils::AudioLoop,3>", "constructor": "array$", "arguments": ["country_core::utils::audio_utils::AudioLoop", "3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<array$<country_core::utils::audio_utils::AudioLoop,3> >", "constructor": "core::ptr::drop_in_place", "arguments": ["array$<country_core::utils::audio_utils::AudioLoop,3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::utils::audio_utils::AudioLoop,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::utils::audio_utils::AudioLoop", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::utils::audio_utils::AudioLoop,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::utils::audio_utils::AudioLoop,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::utils::audio_utils::AudioLoop>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::utils::audio_utils::AudioLoop"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::utils::audio_utils::AudioLoopDesc
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 4; 过程引用数: 0
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: audio_utils.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// [已有模块证据关联；不自动成为字段事实] evidence/parallel-critters/B-decompile/report.json; explicit B; [{"kind": "exact_PDB_named_entity_mention", "name": "country_core::utils::audio_utils::AudioLoop", "status": "evidence_association; original field ownership unknown"}]
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: audio_utils.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/B/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 236; 完整候选见附档及 _pdb/B/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(country_core::systems::world_raster::RasterHistoryRecordResources,bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone1EditToHistoryCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone2EditToHistoryCmd>,bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone3EditToHistoryCmd>)
// 候选参数类型: ["country_core::systems::world_raster::RasterHistoryRecordResources", "bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>", "bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone1EditToHistoryCmd>", "bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone2EditToHistoryCmd>", "bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone3EditToHistoryCmd>"]
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordPathEditToHistoryCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone1EditToHistoryCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone2EditToHistoryCmd>
// [候选系统接口] event_reader: bevy_ecs::event::reader::EventReader<country_core::systems::paths::record_path_history::RecordCobblestone3EditToHistoryCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::event::writer::EventWriter<country_core::systems::backend::grow_grass::GrowGrass>,bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>,bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >)
// 候选参数类型: ["bevy_ecs::event::writer::EventWriter<country_core::systems::backend::grow_grass::GrowGrass>", "bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>", "bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >"]
// [候选系统接口] event_writer: bevy_ecs::event::writer::EventWriter<country_core::systems::backend::grow_grass::GrowGrass>
// [候选系统接口] system_local: bevy_ecs::system::system_param::Local<country_core::systems::backend::grow_grass::platform::Timer>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >
// [文件级接口候选，具体函数归属 unknown] core::char::EscapeDebug (*)(char)
// 候选参数类型: ["char"]
// [文件级接口候选，具体函数归属 unknown] tuple$<i64,std::time::Instant> (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::color_id::ColorId>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::BalconyDoorInstanceData> > >,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::color_id::ColorId>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::BalconyDoorInstanceData> > >", "bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::BalconyDoorInstanceData> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,ref$<country_core::resources::color_id::ColorId>,enum2$<core::option::Option<ref$<country_core::components::SemiTransparent> > > >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::BalconyDoorInstanceData> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::VelocityOnlyInstanceData> > >,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::VelocityOnlyInstanceData> > >", "bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::VelocityOnlyInstanceData> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<utils::components::transform::Transform>,ref$<country_core::components::drawable::PrevTransform> >,bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::VelocityOnlyInstanceData> > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>
// [文件级接口候选，具体函数归属 unknown] bool (*)(enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress> > >"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::loading_screen::LoadingScreenProgress>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<country_core::resources::stairs::visual::StairsFloorMeshChunk> >,tuple$<bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::StairsFloorInstanceData> >,bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::stairs::visual::StairsFloorMeshChunk>,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniActiveInstance> > > > >,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>)
// 候选参数类型: ["bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<country_core::resources::stairs::visual::StairsFloorMeshChunk> >,tuple$<bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::StairsFloorInstanceData> >,bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::stairs::visual::StairsFloorMeshChunk>,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniActiveInstance> > > > >", "bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>"]
// [候选系统接口] query_with_filter: bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::StairsFloorInstanceData> >
// [候选系统接口] component_query: bevy_ecs::system::query::Query<tuple$<ref$<country_core::components::drawable::NaniActiveInstance>,ref$<country_core::resources::stairs::visual::StairsFloorMeshChunk> >,tuple$<bevy_ecs::query::filter::With<country_core::render::nani::gpu_world::InstanceDataMarker<country_core::render::nani::instance_data::StairsFloorInstanceData> >,bevy_ecs::query::filter::Or<tuple$<bevy_ecs::query::filter::Changed<country_core::resources::stairs::visual::StairsFloorMeshChunk>,bevy_ecs::query::filter::Changed<country_core::components::drawable::NaniActiveInstance> > > > >
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::audio_resources::AudioCore::instantiate_loop
// 信息 ID: 2b659ac840264ed0218d90d9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 91ceb1718f806fece44659eb: void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 74 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::water_audio::water_audio_init_system @ 0x8731ac -> 0xa59220 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::water_audio::water_audio_init_system @ 0x873222 -> 0xa59220 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::water_audio::water_audio_init_system @ 0x873298 -> 0xa59220 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::water_audio::water_audio_init_system @ 0x873308 -> 0xa59220 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system @ 0x94fbe2 -> 0xa59220 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system @ 0x94fc74 -> 0xa59220 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system @ 0x94fd06 -> 0xa59220 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system @ 0x94fd98 -> 0xa59220 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system @ 0x94fe2a -> 0xa59220 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system @ 0x94febc -> 0xa59220 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system @ 0x94ff48 -> 0xa59220 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system @ 0x94ffcb -> 0xa59220 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system @ 0x95004e -> 0xa59220 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system @ 0x9500dd -> 0xa59220 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::path_audio::path_audio_init_system @ 0x9d092f -> 0xa59220 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::path_audio::path_audio_init_system @ 0x9d09c7 -> 0xa59220 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::audio::water_ambience_audio::water_ambience_audio_init_system; module=535, inline_record=23400; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::audio_resources::AudioCore::instantiate_loop_snappy
// 信息 ID: 88f6869eafbd533eec624ba9
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::path_audio::path_audio_init_system; module=531, inline_record=57036; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::path_audio::path_audio_init_system; module=531, inline_record=57196; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::path_audio::path_audio_init_system; module=531, inline_record=57352; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::path_audio::path_audio_init_system; module=531, inline_record=57508; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::path_audio::path_audio_init_system; module=531, inline_record=57664; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::audio_utils::AudioLoop::current
// 信息 ID: 1ef5973623efcd254a40a00c
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 105 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::duck::move_ducks_along_trajectories @ 0x2313554 -> 0xa593f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::duck::move_ducks_along_trajectories @ 0x2313577 -> 0xa593f0 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::duck::move_ducks_along_trajectories @ 0x2313598 -> 0xa593f0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_utils::AudioLoopPlayer::request_play; module=507, inline_record=33364; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod; module=513, inline_record=15424; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod; module=513, inline_record=15656; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod; module=513, inline_record=15888; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod; module=513, inline_record=16312; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod; module=513, inline_record=19756; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod; module=513, inline_record=20924; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::signifier_audio::signifier_audio_system; module=532, inline_record=52592; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::audio_utils::AudioLoop::is_playing
// 信息 ID: d90a1088e517b307e67e3b26
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::terrain_stairs_audio::terrain_stairs_audio_system; module=1606, inline_record=95008; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::audio_utils::AudioLoop::start_if_not_playing
// 信息 ID: a01e0fb9be9dc873dfbe9477
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 47 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod @ 0x951470 -> 0xa596b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod @ 0x951496 -> 0xa596b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod @ 0x9514c0 -> 0xa596b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod @ 0x9514ed -> 0xa596b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod @ 0x95151a -> 0xa596b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod @ 0x951547 -> 0xa596b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod @ 0x951574 -> 0xa596b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod @ 0x9515a1 -> 0xa596b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod @ 0x9515ce -> 0xa596b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod @ 0x9515fb -> 0xa596b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::sheep_audio::sheep_audio_system @ 0x9d61d1 -> 0xa596b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::water_ambience_audio::water_ambience_audio_system @ 0x9ed2da -> 0xa596b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::timberframe_audio::timberframe_audio @ 0xa680d7 -> 0xa596b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::timberframe_audio::timberframe_audio @ 0xa681b6 -> 0xa596b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::timberframe_audio::timberframe_audio @ 0xa6830d -> 0xa596b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::firstperson_steps::firstperson_steps @ 0xa88f48 -> 0xa596b0 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::audio::building_audio::building_audio; module=1734, inline_record=144276; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::building_audio::building_audio; module=1734, inline_record=145468; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::audio_utils::AudioLoop::stop
// 信息 ID: 6b6c44db713fb9a30df1c89a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 61 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::water_audio::water_audio_system @ 0x873bb7 -> 0xa59430 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::water_audio::water_audio_system @ 0x873bcc -> 0xa59430 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::water_audio::water_audio_system @ 0x873c35 -> 0xa59430 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::water_audio::water_audio_system @ 0x873c4d -> 0xa59430 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod @ 0x951606 -> 0xa59430 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod @ 0x9515d9 -> 0xa59430 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod @ 0x9515ac -> 0xa59430 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod @ 0x95157f -> 0xa59430 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod @ 0x951552 -> 0xa59430 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod @ 0x951525 -> 0xa59430 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod @ 0x9514f8 -> 0xa59430 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod @ 0x9514cb -> 0xa59430 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod @ 0x9514a1 -> 0xa59430 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::wall_audio_fmod::wall_audio_fmod @ 0x95147a -> 0xa59430 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::path_audio::path_audio_system @ 0x9d148a -> 0xa59430 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::path_audio::path_audio_system @ 0x9d1497 -> 0xa59430 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::audio_utils::AudioLoopDesc::new
// 信息 ID: 80ae2e25cb2d558c35cf403f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 91ceb1718f806fece44659eb: void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 0 条；内联宿主 74 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::water_audio::water_audio_init_system; module=484, inline_record=40752; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::water_audio::water_audio_init_system; module=484, inline_record=40852; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::water_audio::water_audio_init_system; module=484, inline_record=40952; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::water_audio::water_audio_init_system; module=484, inline_record=41052; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system; module=513, inline_record=9860; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system; module=513, inline_record=9988; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system; module=513, inline_record=10116; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_fmod::wall_audio_init_system; module=513, inline_record=10244; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::audio_utils::impl$1::instantiate_loop::closure$0
// 信息 ID: ba5920a13022f8ad176fe729
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 8021970ebabb368c6d099be2: enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::audio_utils::impl$1::instantiate_loop", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::audio_utils::impl$1::instantiate_loop::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::audio_resources::AudioInstance,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::utils::audio_utils::impl$1::instantiate_loop::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::audio_utils::impl$1::instantiate_loop::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<country_core::resources::audio_resources::AudioInstance,anyhow::Error> >,core::ops::range::Range<usize>,country_core::utils::audio_utils::impl$1::instantiate_loop::closure_env$0,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::utils::audio_utils::impl$1::instantiate_loop::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<country_core::resources::audio_resources::AudioInstance,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::audio_utils::impl$1::instantiate_loop::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<country_core::resources::audio_resources::AudioInstance,anyhow::Error> >,core::ops::range::Range<usize>,country_core::utils::audio_utils::impl$1::instantiate_loop::closure_env$0,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::utils::audio_utils::impl$1::instantiate_loop::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<country_core::resources::audio_resources::AudioInstance,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<country_core::resources::audio_resources::AudioInstance,anyhow::Error> >,core::ops::range::Range<usize>,country_core::utils::audio_utils::impl$1::instantiate_loop::closure_env$0,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::utils::audio_utils::impl$1::instantiate_loop::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<country_core::resources::audio_resources::AudioInstance,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > >,tuple$<> > > >; module=1610, inline_record=99284; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
