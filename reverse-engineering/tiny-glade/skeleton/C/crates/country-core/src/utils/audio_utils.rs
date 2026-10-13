// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/utils/audio_utils.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 04324e4fd55279a98863afbe5d7f950361c390d8eca7c313aaa95dcec3eb3fa5
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: audio_utils.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 512, 519, 525, 538, 541, 546, 549, 571, 580, 588, 596, 597, 598, 605, 1231, 1662, 1665, 1682, 1698, 1722, 1727, 1774, 1818
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use libfmod; // namespace candidate
//   package identities: [{"name":"libfmod","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.222.6"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::utils::audio_utils::AudioLoop::start_if_not_playing
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=4
// 源码行: 0, 134
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x84E6
// 首条位置: {"module":546,"record_offset":2648,"rva":11130448,"va":5379839568}
// 编译实例 RVAs: 0xa9d650
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7bd70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: audio_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-137544","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379839568,"raw_hex":"e8b8e6fdff","size":5,"soft_fail":false,"target_va":5379702128,"va":5379839667,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7cbd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: audio_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-134093","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379839568,"raw_hex":"e833f4fdff","size":5,"soft_fail":false,"target_va":5379705808,"va":5379839896,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36781922","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379839568,"raw_hex":"e8623f3102","size":5,"soft_fail":false,"target_va":5416621762,"va":5379839835,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36781813","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379839568,"raw_hex":"e8f53e3102","size":5,"soft_fail":false,"target_va":5416621632,"va":5379839814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::start
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libfmod","version":"2.222.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16814589","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379839568,"raw_hex":"e8fd910001","size":5,"soft_fail":false,"target_va":5396654304,"va":5379839710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":546,"record_offset":2804,"parent_record_offset":2648,"ancestor_record_offset":2648,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::start
//     owner=private_source_ownership_candidate; count=3
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":546,"record_offset":3008,"parent_record_offset":2648,"ancestor_record_offset":2648,"inlinee":33513,"source_line":505,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":546,"record_offset":3048,"parent_record_offset":3008,"ancestor_record_offset":2648,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":546,"record_offset":3076,"parent_record_offset":3008,"ancestor_record_offset":2648,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":546,"record_offset":2832,"parent_record_offset":2804,"ancestor_record_offset":2648,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":546,"record_offset":2860,"parent_record_offset":2832,"ancestor_record_offset":2648,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00546.jsonl.gz"}
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
// 首条位置: {"module":546,"record_offset":3224,"rva":11130800,"va":5379839920}
// 编译实例 RVAs: 0xa9d7b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::time::Instant>::elapsed
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4808449","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379839920,"raw_hex":"e8015f4900","size":5,"soft_fail":false,"target_va":5384648448,"va":5379839994,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::time::Instant>::now
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4808065","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379839920,"raw_hex":"e8815d4900","size":5,"soft_fail":false,"target_va":5384648032,"va":5379839962,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7bd70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: audio_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-137960","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379839920,"raw_hex":"e818e5fdff","size":5,"soft_fail":false,"target_va":5379702128,"va":5379840083,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7cbd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: audio_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-134559","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379839920,"raw_hex":"e861f2fdff","size":5,"soft_fail":false,"target_va":5379705808,"va":5379840362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36781445","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379839920,"raw_hex":"e8853d3102","size":5,"soft_fail":false,"target_va":5416621762,"va":5379840312,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36781336","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379839920,"raw_hex":"e8183d3102","size":5,"soft_fail":false,"target_va":5416621632,"va":5379840291,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::stop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libfmod","version":"2.222.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16813820","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379839920,"raw_hex":"e8fc8e0001","size":5,"soft_fail":false,"target_va":5396653968,"va":5379840143,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":546,"record_offset":3400,"parent_record_offset":3224,"ancestor_record_offset":3224,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] core::time::Duration::as_millis
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\time.rs
//     evidence={"module":546,"record_offset":3368,"parent_record_offset":3224,"ancestor_record_offset":3224,"inlinee":20292,"source_line":593,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::stop
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":546,"record_offset":3616,"parent_record_offset":3224,"ancestor_record_offset":3224,"inlinee":33511,"source_line":532,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":546,"record_offset":3660,"parent_record_offset":3616,"ancestor_record_offset":3224,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":546,"record_offset":3688,"parent_record_offset":3616,"ancestor_record_offset":3224,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":546,"record_offset":3436,"parent_record_offset":3400,"ancestor_record_offset":3224,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":546,"record_offset":3472,"parent_record_offset":3436,"ancestor_record_offset":3224,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00546.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::audio_utils::AudioLoopDesc::new
// 记录形式: inline_instance, inlinee_declaration; instances=122
// 源码行: 12
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x73CB
// 首条位置: {"module":512,"inlinee":29643}
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
// IPI IDs: 0x82B5
// 首条位置: {"module":541,"record_offset":3296,"rva":10965600,"va":5379674720}
// 编译实例 RVAs: 0xa75260
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1409aa2e0
//     owner=unresolved_namespace; count=2
//     target_aliases=4 candidates; complete list: audio_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-831482","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379674720,"raw_hex":"e80650f3ff","size":5,"soft_fail":false,"target_va":5378843360,"va":5379674837,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140a7cbd0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: audio_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t30829","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379674720,"raw_hex":"e86d780000","size":5,"soft_fail":false,"target_va":5379705808,"va":5379674974,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141612ae0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: audio_utils.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12179485","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379674720,"raw_hex":"e81dd8b900","size":5,"soft_fail":false,"target_va":5391854304,"va":5379674814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4948964","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379674720,"raw_hex":"e8e4834b00","size":5,"soft_fail":false,"target_va":5384624080,"va":5379675111,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36946705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379674720,"raw_hex":"e811c33302","size":5,"soft_fail":false,"target_va":5416621762,"va":5379675052,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] libfmod::EventInstance::get_parameter_by_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"libfmod","version":"2.222.6","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t16977402","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379674720,"raw_hex":"e8fa0d0301","size":5,"soft_fail":false,"target_va":5396652352,"va":5379674945,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":541,"record_offset":3804,"parent_record_offset":3296,"ancestor_record_offset":3296,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":541,"record_offset":3468,"parent_record_offset":3296,"ancestor_record_offset":3296,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":541,"record_offset":3748,"parent_record_offset":3296,"ancestor_record_offset":3296,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":541,"record_offset":3776,"parent_record_offset":3296,"ancestor_record_offset":3296,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::impl$10::has_parameter
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":541,"record_offset":3900,"parent_record_offset":3296,"ancestor_record_offset":3296,"inlinee":33403,"source_line":621,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":605,"record_offset":43156,"parent_record_offset":43132,"ancestor_record_offset":42352,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":605,"record_offset":43180,"parent_record_offset":43156,"ancestor_record_offset":42352,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":605,"record_offset":43204,"parent_record_offset":43180,"ancestor_record_offset":42352,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":605,"record_offset":43132,"parent_record_offset":43076,"ancestor_record_offset":42352,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::boxed::impl$10::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\boxed.rs
//     evidence={"module":605,"record_offset":43648,"parent_record_offset":43624,"ancestor_record_offset":42352,"inlinee":4463,"source_line":1913,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::ffi::c_str::impl$2::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\ffi\c_str.rs
//     evidence={"module":605,"record_offset":43596,"parent_record_offset":43572,"ancestor_record_offset":42352,"inlinee":4646,"source_line":700,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":605,"record_offset":43104,"parent_record_offset":43076,"ancestor_record_offset":42352,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":605,"record_offset":43076,"parent_record_offset":43052,"ancestor_record_offset":42352,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":605,"record_offset":43052,"parent_record_offset":43028,"ancestor_record_offset":42352,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":541,"record_offset":3616,"parent_record_offset":3592,"ancestor_record_offset":3296,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":605,"record_offset":42716,"parent_record_offset":42676,"ancestor_record_offset":42352,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::try_process
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\mod.rs
//     evidence={"module":541,"record_offset":3524,"parent_record_offset":3496,"ancestor_record_offset":3296,"inlinee":11759,"source_line":152,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":541,"record_offset":3640,"parent_record_offset":3616,"ancestor_record_offset":3296,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":541,"record_offset":3592,"parent_record_offset":3572,"ancestor_record_offset":3296,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":605,"record_offset":42896,"parent_record_offset":42872,"ancestor_record_offset":42352,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":605,"record_offset":46880,"parent_record_offset":46852,"ancestor_record_offset":42352,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":541,"record_offset":3928,"parent_record_offset":3900,"ancestor_record_offset":3296,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":605,"record_offset":42872,"parent_record_offset":42828,"ancestor_record_offset":42352,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=61
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":541,"record_offset":3680,"parent_record_offset":3524,"ancestor_record_offset":3296,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::is_ok
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":541,"record_offset":3956,"parent_record_offset":3900,"ancestor_record_offset":3296,"inlinee":5676,"source_line":593,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::unwrap
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":605,"record_offset":44316,"parent_record_offset":42828,"ancestor_record_offset":42352,"inlinee":4983,"source_line":1227,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":541,"record_offset":3496,"parent_record_offset":3468,"ancestor_record_offset":3296,"inlinee":23302,"source_line":2155,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$26::from_iter::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":541,"record_offset":3572,"parent_record_offset":3524,"ancestor_record_offset":3296,"inlinee":23304,"source_line":2156,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":541,"record_offset":3832,"parent_record_offset":3804,"ancestor_record_offset":3296,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":541,"record_offset":3860,"parent_record_offset":3832,"ancestor_record_offset":3296,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00541.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::audio_resources::impl$13::drop
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":605,"record_offset":42828,"parent_record_offset":42796,"ancestor_record_offset":42352,"inlinee":30134,"source_line":668,"archive":"_pdb/C/modules/00605.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::audio_utils::impl$1::instantiate_loop::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x143B2
// 首条位置: {"module":1682,"inlinee":82866}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioCore::instantiate_audio
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":1682,"record_offset":179876,"parent_record_offset":179840,"ancestor_record_offset":179840,"inlinee":29635,"source_line":321,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":180208,"parent_record_offset":180184,"ancestor_record_offset":179840,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":180232,"parent_record_offset":180208,"ancestor_record_offset":179840,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":180256,"parent_record_offset":180232,"ancestor_record_offset":179840,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1682,"record_offset":180184,"parent_record_offset":180156,"ancestor_record_offset":179840,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":180096,"parent_record_offset":180068,"ancestor_record_offset":179840,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":180156,"parent_record_offset":180124,"ancestor_record_offset":179840,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1682,"record_offset":180124,"parent_record_offset":180096,"ancestor_record_offset":179840,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1682,"record_offset":179980,"parent_record_offset":179952,"ancestor_record_offset":179840,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1682,"record_offset":180008,"parent_record_offset":179980,"ancestor_record_offset":179840,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1682,"record_offset":180036,"parent_record_offset":180008,"ancestor_record_offset":179840,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1682,"record_offset":179952,"parent_record_offset":179924,"ancestor_record_offset":179840,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1682,"record_offset":179924,"parent_record_offset":179876,"ancestor_record_offset":179840,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1682,"record_offset":180068,"parent_record_offset":180036,"ancestor_record_offset":179840,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] anyhow::Ok
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/lib.rs
//     evidence={"module":1682,"record_offset":180388,"parent_record_offset":179876,"ancestor_record_offset":179840,"inlinee":29637,"source_line":653,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1682,"record_offset":180312,"parent_record_offset":180036,"ancestor_record_offset":179840,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1682,"record_offset":180336,"parent_record_offset":180312,"ancestor_record_offset":179840,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01682.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::resources::audio_resources::AudioCore::instantiate_loop_snappy
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 65
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x82B6
// 首条位置: {"module":588,"inlinee":33462}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":588,"record_offset":48896,"parent_record_offset":48872,"ancestor_record_offset":48872,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00588.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::audio_utils::AudioLoop::current
// 记录形式: compiled_procedure, inline_instance, inlinee_declaration; instances=140
// 源码行: 95
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x73C9
// 首条位置: {"module":512,"inlinee":29641}
// 编译实例 RVAs: 0xa9d980
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::panicking::panic_bounds_check
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36781329","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379840384,"raw_hex":"e8113d3102","size":5,"soft_fail":false,"target_va":5416621762,"va":5379840428,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=130
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":512,"record_offset":31488,"parent_record_offset":31460,"ancestor_record_offset":31460,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=130
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":512,"record_offset":31516,"parent_record_offset":31488,"ancestor_record_offset":31460,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=130
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":512,"record_offset":31544,"parent_record_offset":31516,"ancestor_record_offset":31460,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/00512.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::utils::audio_utils::AudioLoop::is_playing
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 147
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x84E5
// 首条位置: {"module":1665,"inlinee":34021}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$14::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1665,"record_offset":8224,"parent_record_offset":8192,"ancestor_record_offset":8192,"inlinee":5465,"source_line":3803,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::audio_resources::AudioInstance::is_playing
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/audio_resources.rs
//     evidence={"module":1665,"record_offset":8332,"parent_record_offset":8192,"ancestor_record_offset":8192,"inlinee":33504,"source_line":591,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1665,"record_offset":8372,"parent_record_offset":8332,"ancestor_record_offset":8192,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1665,"record_offset":8400,"parent_record_offset":8332,"ancestor_record_offset":8192,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1665,"record_offset":8256,"parent_record_offset":8224,"ancestor_record_offset":8192,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/01665.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$2::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":1665,"record_offset":8288,"parent_record_offset":8256,"ancestor_record_offset":8192,"inlinee":5468,"source_line":270,"archive":"_pdb/C/modules/01665.jsonl.gz"}
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
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 5; 过程引用数: 11
// [PDB 泛型关系，非字段关系] {"type": "array$<country_core::utils::audio_utils::AudioLoop,3>", "constructor": "array$", "arguments": ["country_core::utils::audio_utils::AudioLoop", "3"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<array$<country_core::utils::audio_utils::AudioLoop,3> >", "constructor": "core::ptr::drop_in_place", "arguments": ["array$<country_core::utils::audio_utils::AudioLoop,3>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "array$<country_core::utils::audio_utils::AudioLoop,5>", "constructor": "array$", "arguments": ["country_core::utils::audio_utils::AudioLoop", "5"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<array$<country_core::utils::audio_utils::AudioLoop,5> >", "constructor": "core::ptr::drop_in_place", "arguments": ["array$<country_core::utils::audio_utils::AudioLoop,5>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::Vec<country_core::utils::audio_utils::AudioLoop,alloc::alloc::Global>", "constructor": "alloc::vec::Vec", "arguments": ["country_core::utils::audio_utils::AudioLoop", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<alloc::vec::Vec<country_core::utils::audio_utils::AudioLoop,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["alloc::vec::Vec<country_core::utils::audio_utils::AudioLoop,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::array::Guard<country_core::utils::audio_utils::AudioLoop>", "constructor": "core::array::Guard", "arguments": ["country_core::utils::audio_utils::AudioLoop"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<core::array::Guard<country_core::utils::audio_utils::AudioLoop> >", "constructor": "core::ptr::drop_in_place", "arguments": ["core::array::Guard<country_core::utils::audio_utils::AudioLoop>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::array::impl$24::map<ref$<str$>,5,country_core::systems::audio::garden_audio::garden_audio_init_system::closure_env$0,country_core::utils::audio_utils::AudioLoop>", "constructor": "core::array::impl$24::map", "arguments": ["ref$<str$>", "5", "country_core::systems::audio::garden_audio::garden_audio_init_system::closure_env$0", "country_core::utils::audio_utils::AudioLoop"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
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
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: audio_utils.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 306; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] core::cell::RefCell<tuple$<parking::Parker,core::task::wake::Waker> > (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] core::cell::Cell<tuple$<u64,u64> > (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] crossbeam_epoch::collector::LocalHandle (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] alloc::sync::Arc<bevy_tasks::thread_executor::ThreadExecutor,alloc::alloc::Global> (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] arc_swap::debt::list::LocalNode (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] core::cell::RefCell<array$<alloc::vec::Vec<bvh::bvh::bvh_node::ShapeIndex,alloc::alloc::Global>,6> > (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] fastrand::Rng (*)()
// 候选参数类型: []
// [文件级接口候选，具体函数归属 unknown] core::cell::Cell<enum2$<core::option::Option<std::sync::mpmc::context::Context> > > (*)()
// 候选参数类型: []
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::resources::audio_resources::AudioCore::instantiate_loop
// 信息 ID: 92d31e4114697789b643f489
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 91ceb1718f806fece44659eb: void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 104 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::signifier_audio::signifier_audio_init_system @ 0x951685 -> 0xa75260 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::water_audio::water_audio_init_system @ 0x9c5dac -> 0xa75260 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::water_audio::water_audio_init_system @ 0x9c5e22 -> 0xa75260 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::water_audio::water_audio_init_system @ 0x9c5e99 -> 0xa75260 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::water_audio::water_audio_init_system @ 0x9c5f0a -> 0xa75260 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)> @ 0x9ecbc9 -> 0xa75260 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::roof_audio::roof_audio_init_system @ 0xab3f2f -> 0xa75260 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::roof_audio::roof_audio_init_system @ 0xab3f9c -> 0xa75260 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::roof_audio::roof_audio_init_system @ 0xab3ffe -> 0xa75260 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::building_audio::building_audio_init_system @ 0xab6cc2 -> 0xa75260 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::building_audio::building_audio_init_system @ 0xab6d4d -> 0xa75260 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::building_audio::building_audio_init_system @ 0xab6dd9 -> 0xa75260 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::building_audio::building_audio_init_system @ 0xab6e65 -> 0xa75260 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::building_audio::building_audio_init_system @ 0xab6ef1 -> 0xa75260 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::building_audio::building_audio_init_system @ 0xab6f7d -> 0xa75260 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::building_audio::building_audio_init_system @ 0xab7009 -> 0xa75260 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::audio::water_ambience_audio::water_ambience_audio_init_system; module=605, inline_record=42352; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::resources::audio_resources::AudioCore::instantiate_loop_snappy
// 信息 ID: f9bb699c135adc7664ef916a
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 5 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::path_audio::path_audio_init_system; module=588, inline_record=48872; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::path_audio::path_audio_init_system; module=588, inline_record=49032; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::path_audio::path_audio_init_system; module=588, inline_record=49188; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::path_audio::path_audio_init_system; module=588, inline_record=49344; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::path_audio::path_audio_init_system; module=588, inline_record=49500; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::audio_utils::AudioLoop::current
// 信息 ID: f414e31378427f5d3444bfb4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 129 条；闭包环境类型引用 0 条。
// [静态直接引用] system_critters::duck::move_ducks_along_trajectories @ 0x292f5de -> 0xa9d980 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::duck::move_ducks_along_trajectories @ 0x292f601 -> 0xa9d980 (direct_call); 同地址别名未裁定。
// [静态直接引用] system_critters::duck::move_ducks_along_trajectories @ 0x292f624 -> 0xa9d980 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::audio::signifier_audio::signifier_audio_system; module=512, inline_record=31460; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::signifier_audio::signifier_audio_system; module=512, inline_record=31588; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::wall_audio_utils::AudioLoopPlayer::request_play; module=538, inline_record=104884; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::roof_audio::roof_audio_system; module=549, inline_record=12168; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::roof_audio::roof_audio_system; module=549, inline_record=12296; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::roof_audio::roof_audio_system; module=549, inline_record=12456; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::roof_audio::roof_audio_system; module=549, inline_record=12640; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::roof_audio::roof_audio_system; module=549, inline_record=12880; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::audio_utils::AudioLoop::is_playing
// 信息 ID: 76964aa7719b3de65bf6fe8e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::terrain_stairs_audio::terrain_stairs_audio_system; module=1665, inline_record=8192; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::audio_utils::AudioLoop::start_if_not_playing
// 信息 ID: 151c6fa714b9a43820522e34
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 63 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::roof_audio::roof_audio_system @ 0xab37b4 -> 0xa9d650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::roof_audio::roof_audio_system @ 0xab3838 -> 0xa9d650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::roof_audio::roof_audio_system @ 0xab36a2 -> 0xa9d650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::sheep_audio::sheep_audio_system @ 0xab4a96 -> 0xa9d650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::building_audio::building_audio @ 0xab5d70 -> 0xa9d650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::building_audio::building_audio @ 0xab5d7c -> 0xa9d650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0xabafc1 -> 0xa9d650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0xabaff9 -> 0xa9d650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0xabb02a -> 0xa9d650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0xabb05e -> 0xa9d650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0xabb088 -> 0xa9d650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0xabb0c3 -> 0xa9d650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0xabb0f7 -> 0xa9d650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0xabb128 -> 0xa9d650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0xabb159 -> 0xa9d650 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0xabb18a -> 0xa9d650 (direct_call); 同地址别名未裁定。
// [PDB 内联宿主] country_core::systems::audio::building_audio::building_audio; module=549, inline_record=28532; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::building_audio::building_audio; module=549, inline_record=28920; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::audio_utils::AudioLoop::stop
// 信息 ID: 651d0d4b7fa6482a764a1016
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 74 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::systems::audio::signifier_audio::signifier_audio_system @ 0x94eac2 -> 0xa9d7b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::water_audio::water_audio_system @ 0x9c5ace -> 0xa9d7b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::water_audio::water_audio_system @ 0x9c5ae7 -> 0xa9d7b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::water_audio::water_audio_system @ 0x9c5b51 -> 0xa9d7b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::water_audio::water_audio_system @ 0x9c5b67 -> 0xa9d7b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::roof_audio::roof_audio_system @ 0xab3673 -> 0xa9d7b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::roof_audio::roof_audio_system @ 0xab388e -> 0xa9d7b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::roof_audio::roof_audio_system @ 0xab3804 -> 0xa9d7b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::sheep_audio::sheep_audio_system @ 0xab4aad -> 0xa9d7b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::building_audio::building_audio @ 0xab5dda -> 0xa9d7b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::building_audio::building_audio @ 0xab5df1 -> 0xa9d7b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0xabb3d3 -> 0xa9d7b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0xabb388 -> 0xa9d7b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0xabb357 -> 0xa9d7b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0xabb326 -> 0xa9d7b0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::systems::audio::decorator_audio::decorator_audio_system @ 0xabb2f5 -> 0xa9d7b0 (direct_call); 同地址别名未裁定。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::audio_utils::AudioLoopDesc::new
// 信息 ID: 57c75f8ba00ede436456eb1f
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 91ceb1718f806fece44659eb: void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 0 条；内联宿主 104 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::systems::audio::signifier_audio::signifier_audio_init_system; module=512, inline_record=37068; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::water_audio::water_audio_init_system; module=519, inline_record=18680; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::water_audio::water_audio_init_system; module=519, inline_record=18780; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::water_audio::water_audio_init_system; module=519, inline_record=18880; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::water_audio::water_audio_init_system; module=519, inline_record=18980; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::audio_resources::AudioCore>,bevy_ecs::system::commands::Commands)>; module=525, inline_record=151664; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::roof_audio::roof_audio_init_system; module=549, inline_record=14388; 不是独立调用边。
// [PDB 内联宿主] country_core::systems::audio::roof_audio::roof_audio_init_system; module=549, inline_record=14488; 不是独立调用边。
// 以上展示前若干条；全部引用方、指令字节及内联记录保存在完整附档，没有丢弃。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::utils::audio_utils::impl$1::instantiate_loop::closure$0
// 信息 ID: 85e396d4567455e7a97247c5
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 8021970ebabb368c6d099be2: enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::utils::audio_utils::impl$1::instantiate_loop", "captures": "unknown"}
// [闭包环境类型引用] country_core::utils::audio_utils::impl$1::instantiate_loop::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<country_core::resources::audio_resources::AudioInstance,core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::utils::audio_utils::impl$1::instantiate_loop::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::audio_utils::impl$1::instantiate_loop::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<country_core::resources::audio_resources::AudioInstance,anyhow::Error> >,core::ops::range::Range<usize>,country_core::utils::audio_utils::impl$1::instantiate_loop::closure_env$0,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::utils::audio_utils::impl$1::instantiate_loop::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<country_core::resources::audio_resources::AudioInstance,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::utils::audio_utils::impl$1::instantiate_loop::closure_env$0 <- core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<country_core::resources::audio_resources::AudioInstance,anyhow::Error> >,core::ops::range::Range<usize>,country_core::utils::audio_utils::impl$1::instantiate_loop::closure_env$0,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::utils::audio_utils::impl$1::instantiate_loop::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<country_core::resources::audio_resources::AudioInstance,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > >,tuple$<> > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 3 条。
// [PDB 内联宿主] core::iter::adapters::map::impl$2::try_fold<enum2$<core::result::Result<country_core::resources::audio_resources::AudioInstance,anyhow::Error> >,core::ops::range::Range<usize>,country_core::utils::audio_utils::impl$1::instantiate_loop::closure_env$0,tuple$<>,core::iter::adapters::impl$0::try_fold::closure_env$0<core::iter::adapters::map::Map<core::ops::range::Range<usize>,country_core::utils::audio_utils::impl$1::instantiate_loop::closure_env$0>,enum2$<core::result::Result<enum2$<core::convert::Infallible>,anyhow::Error> >,tuple$<>,core::iter::traits::iterator::Iterator::try_for_each::call::closure_env$0<country_core::resources::audio_resources::AudioInstance,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > >,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > (*)(country_core::resources::audio_resources::AudioInstance)>,enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > > >,enum2$<core::ops::control_flow::ControlFlow<enum2$<core::ops::control_flow::ControlFlow<country_core::resources::audio_resources::AudioInstance,tuple$<> > >,tuple$<> > > >; module=1682, inline_record=179840; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
