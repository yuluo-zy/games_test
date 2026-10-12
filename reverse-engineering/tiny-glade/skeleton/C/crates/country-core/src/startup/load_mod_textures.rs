// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/startup/load_mod_textures.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: 6ac9cfe2f425da889167bb38edc17338015d8892f4f3e4f806cc2b596e5c7a19
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load_mod_textures.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 524, 1658, 1682, 1715
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
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use puffin; // namespace candidate
//   package identities: [{"name":"puffin","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.1"}]
// use std; // namespace candidate
//   package identities: []
// use tg_mods; // namespace candidate
//   package identities: [{"name":"tg-mods","source":"local_or_path","version":"0.1.0"}]
// use turbosloth; // namespace candidate
//   package identities: [{"name":"turbosloth","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mod_textures::is_moddable_texture_name
// 记录形式: compiled_procedure; instances=1
// 源码行: 0
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":524,"record_offset":30412,"rva":10378000,"va":5379087120}
// 编译实例 RVAs: 0x9e5b10
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] core::slice::memchr::memchr_aligned
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5743417","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379087120,"raw_hex":"e839a35700","size":5,"soft_fail":false,"target_va":5384830608,"va":5379087186,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::ne
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":524,"record_offset":31260,"parent_record_offset":30412,"ancestor_record_offset":30412,"inlinee":13036,"source_line":2126,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":30788,"parent_record_offset":30412,"ancestor_record_offset":30412,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":524,"record_offset":30572,"parent_record_offset":30412,"ancestor_record_offset":30412,"inlinee":12765,"source_line":1367,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialEq::ne
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":524,"record_offset":31284,"parent_record_offset":31260,"ancestor_record_offset":30412,"inlinee":4487,"source_line":264,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":524,"record_offset":30956,"parent_record_offset":30928,"ancestor_record_offset":30412,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":524,"record_offset":30740,"parent_record_offset":30648,"ancestor_record_offset":30412,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":524,"record_offset":30980,"parent_record_offset":30956,"ancestor_record_offset":30412,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$18::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":524,"record_offset":30648,"parent_record_offset":30620,"ancestor_record_offset":30412,"inlinee":12768,"source_line":382,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":524,"record_offset":31004,"parent_record_offset":30980,"ancestor_record_offset":30412,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":524,"record_offset":30620,"parent_record_offset":30596,"ancestor_record_offset":30412,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":524,"record_offset":30928,"parent_record_offset":30900,"ancestor_record_offset":30412,"inlinee":8287,"source_line":2619,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::memchr::memchr
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\memchr.rs
//     evidence={"module":524,"record_offset":30672,"parent_record_offset":30648,"ancestor_record_offset":30412,"inlinee":6283,"source_line":25,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::memchr::memchr_naive
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\memchr.rs
//     evidence={"module":524,"record_offset":30700,"parent_record_offset":30672,"ancestor_record_offset":30412,"inlinee":6284,"source_line":35,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::starts_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":524,"record_offset":30844,"parent_record_offset":30816,"ancestor_record_offset":30412,"inlinee":8324,"source_line":1405,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$30::is_prefix_of
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":524,"record_offset":30872,"parent_record_offset":30844,"ancestor_record_offset":30412,"inlinee":30754,"source_line":746,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":524,"record_offset":30900,"parent_record_offset":30872,"ancestor_record_offset":30412,"inlinee":8286,"source_line":981,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$4::is_contained_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":524,"record_offset":30596,"parent_record_offset":30572,"ancestor_record_offset":30412,"inlinee":12766,"source_line":583,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":524,"record_offset":31308,"parent_record_offset":31284,"ancestor_record_offset":30412,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_mod_textures::is_moddable_texture_name::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_mod_textures.rs
//     evidence={"module":524,"record_offset":30816,"parent_record_offset":30788,"ancestor_record_offset":30412,"inlinee":30976,"source_line":160,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mod_textures::reload_mod_textures
// 记录形式: compiled_procedure; instances=1
// 源码行: 40
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":524,"record_offset":6556,"rva":10368464,"va":5379077584}
// 编译实例 RVAs: 0x9e35d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5566592","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e880f05400","size":5,"soft_fail":false,"target_va":5384644912,"va":5379078315,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t5529684","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e854605400","size":5,"soft_fail":false,"target_va":5384609408,"va":5379079719,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-3400761","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e8c71bccff","size":5,"soft_fail":false,"target_va":5375678688,"va":5379079444,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=24
//     evidence={"assembly":"\tcallq\t-3399345","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e84f21ccff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379078044,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=3
//     target_aliases=1699 candidates; complete list: load_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10357244","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e804f661ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5379079431,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14009ffa0
//     owner=unresolved_namespace; count=1
//     target_aliases=25 candidates; complete list: load_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9722686","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e8c2a46bff","size":5,"soft_fail":false,"target_va":5369364384,"va":5379087065,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c05e0
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: load_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9589659","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e865ac6dff","size":5,"soft_fail":false,"target_va":5369497056,"va":5379086710,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1400c4220
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: load_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9573988","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e89ce96dff","size":5,"soft_fail":false,"target_va":5369512480,"va":5379086463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14040fae0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: load_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6110947","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e81dc1a2ff","size":5,"soft_fail":false,"target_va":5372967648,"va":5379078590,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048d130
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-5601798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e8fa85aaff","size":5,"soft_fail":false,"target_va":5373481264,"va":5379083057,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409711d0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-476705","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e8dfb9f8ff","size":5,"soft_fail":false,"target_va":5378609616,"va":5379086316,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140971300
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: load_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-476903","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e819b9f8ff","size":5,"soft_fail":false,"target_va":5378609920,"va":5379086818,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140978b70
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-443098","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e8263df9ff","size":5,"soft_fail":false,"target_va":5378640752,"va":5379083845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098f4c0
//     owner=unresolved_namespace; count=3
//     target_aliases=5 candidates; complete list: load_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-351548","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e8c4a2faff","size":5,"soft_fail":false,"target_va":5378733248,"va":5379084791,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098f9c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-350654","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e842a6faff","size":5,"soft_fail":false,"target_va":5378734528,"va":5379085177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1415f8e90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12668933","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e80550c100","size":5,"soft_fail":false,"target_va":5391748752,"va":5379079814,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1416f4de0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t13696208","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e8d0fcd000","size":5,"soft_fail":false,"target_va":5392780768,"va":5379084555,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141770710
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t14204535","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e877bed800","size":5,"soft_fail":false,"target_va":5393286928,"va":5379082388,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d6f6a0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37271354","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e83ab73802","size":5,"soft_fail":false,"target_va":5416351392,"va":5379080033,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142d77a00
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37300952","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e8d82a3902","size":5,"soft_fail":false,"target_va":5416385024,"va":5379084067,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd56b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37688312","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e8f8133f02","size":5,"soft_fail":false,"target_va":5416769200,"va":5379080883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: load_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37978082","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e8e27f4302","size":5,"soft_fail":false,"target_va":5417057040,"va":5379078953,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=3
//     target_aliases=1 candidates; complete list: load_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37977572","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e8e47d4302","size":5,"soft_fail":false,"target_va":5417057056,"va":5379079479,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t37540389","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e825d23c02","size":5,"soft_fail":false,"target_va":5416620794,"va":5379080400,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t5542558","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e89e925400","size":5,"soft_fail":false,"target_va":5384624080,"va":5379081517,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::cell::panic_already_borrowed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37537282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e802c63c02","size":5,"soft_fail":false,"target_va":5416621408,"va":5379084121,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::ptr::drop_in_place<tuple$<alloc::string::String,enum2$<tg_mods::overlay::ModTextureLayers> > >
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4978","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e872130000","size":5,"soft_fail":false,"target_va":5379090848,"va":5379085865,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::impl$0::rfind<ref$<str$> >
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t13274","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e8da330000","size":5,"soft_fail":false,"target_va":5379091024,"va":5379077745,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::slice_error_fail
//     owner=rust_standard_library; count=3
//     evidence={"assembly":"\tcallq\t37537125","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e865c53c02","size":5,"soft_fail":false,"target_va":5416621376,"va":5379084246,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::add_or_replace
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t194949","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e885f90200","size":5,"soft_fail":false,"target_va":5379276736,"va":5379081782,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::remove_by_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t195203","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e883fa0200","size":5,"soft_fail":false,"target_va":5379278368,"va":5379083160,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::load_texture::create_game_texture
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1754906","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e81ac71a00","size":5,"soft_fail":false,"target_va":5380836016,"va":5379081105,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::bake_mod_textures::bake_and_apply_mod_texture_sources
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t2727145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e8e99c2900","size":5,"soft_fail":false,"target_va":5381809504,"va":5379082354,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1759143","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e8a7d71a00","size":5,"soft_fail":false,"target_va":5380843104,"va":5379083956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_mod_textures::is_moddable_texture_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t8339","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e893200000","size":5,"soft_fail":false,"target_va":5379087120,"va":5379078776,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_textures::reload_packaged_textures_by_name
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t730987","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e86b270b00","size":5,"soft_fail":false,"target_va":5379813424,"va":5379082432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::begin_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4885609","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e8698c4a00","size":5,"soft_fail":false,"target_va":5383963792,"va":5379078178,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::call::THREAD_PROFILER::_$u7b$$u7b$constant$u7d$$u7d$::_$u7b$$u7b$closure$u7d$$u7d$::__RUST_STD_INTERNAL_VAL$u7b$$u7b$tls.shim$u7d$$u7d$
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4894117","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e8a5ad4a00","size":5,"soft_fail":false,"target_va":5383972224,"va":5379078102,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::ThreadProfiler::end_scope
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4880992","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e8607a4a00","size":5,"soft_fail":false,"target_va":5383964912,"va":5379083915,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] puffin::are_scopes_on
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4885107","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e8738a4a00","size":5,"soft_fail":false,"target_va":5383962816,"va":5379077704,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5671086","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e8ae885600","size":5,"soft_fail":false,"target_va":5384751936,"va":5379080845,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5642656","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e8a0195600","size":5,"soft_fail":false,"target_va":5384726688,"va":5379084027,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::thread::local::panic_access_error
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t37535259","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e81bbe3c02","size":5,"soft_fail":false,"target_va":5416619360,"va":5379084096,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::legacy_texture::parse_legacy_texture
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t18093663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e85f161401","size":5,"soft_fail":false,"target_va":5397174608,"va":5379080940,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::ChangedPaths::texture_names
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3807516","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e81c193a00","size":5,"soft_fail":false,"target_va":5382885200,"va":5379077679,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::mod_texture_bindings
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3839901","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379077584,"raw_hex":"e89d973a00","size":5,"soft_fail":false,"target_va":5382918128,"va":5379078222,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] country_core::startup::load_mod_textures::reload_replaced_textures
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_mod_textures.rs
//     evidence={"module":524,"record_offset":6704,"parent_record_offset":6556,"ancestor_record_offset":6556,"inlinee":30967,"source_line":51,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":13684,"parent_record_offset":13660,"ancestor_record_offset":6556,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":13708,"parent_record_offset":13684,"ancestor_record_offset":6556,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":8908,"parent_record_offset":8880,"ancestor_record_offset":6556,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":8936,"parent_record_offset":8908,"ancestor_record_offset":6556,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":13732,"parent_record_offset":13708,"ancestor_record_offset":6556,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":8968,"parent_record_offset":8936,"ancestor_record_offset":6556,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":13660,"parent_record_offset":13512,"ancestor_record_offset":6556,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":8880,"parent_record_offset":8692,"ancestor_record_offset":6556,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::BTreeMap::contains_key
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":524,"record_offset":12496,"parent_record_offset":6704,"ancestor_record_offset":6556,"inlinee":21140,"source_line":980,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::BTreeMap::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":524,"record_offset":11864,"parent_record_offset":6704,"ancestor_record_offset":6556,"inlinee":7203,"source_line":718,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::descend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":524,"record_offset":12376,"parent_record_offset":11928,"ancestor_record_offset":6556,"inlinee":4921,"source_line":1102,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::force
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":524,"record_offset":12320,"parent_record_offset":11928,"ancestor_record_offset":6556,"inlinee":4920,"source_line":1681,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::find_key_index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\search.rs
//     evidence={"module":524,"record_offset":11988,"parent_record_offset":11964,"ancestor_record_offset":6556,"inlinee":5456,"source_line":217,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::force
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":524,"record_offset":12344,"parent_record_offset":12320,"ancestor_record_offset":6556,"inlinee":4916,"source_line":725,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::keys
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":524,"record_offset":12020,"parent_record_offset":11988,"ancestor_record_offset":6556,"inlinee":5457,"source_line":394,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::search_node
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\search.rs
//     evidence={"module":524,"record_offset":11964,"parent_record_offset":11928,"ancestor_record_offset":6556,"inlinee":5455,"source_line":195,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::search_tree
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\search.rs
//     evidence={"module":524,"record_offset":11928,"parent_record_offset":11864,"ancestor_record_offset":6556,"inlinee":5454,"source_line":49,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":15284,"parent_record_offset":15252,"ancestor_record_offset":6556,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":13452,"parent_record_offset":13424,"ancestor_record_offset":6556,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":14248,"parent_record_offset":14220,"ancestor_record_offset":6556,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":14220,"parent_record_offset":14196,"ancestor_record_offset":6556,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":15332,"parent_record_offset":15308,"ancestor_record_offset":6556,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":15308,"parent_record_offset":15284,"ancestor_record_offset":6556,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":13512,"parent_record_offset":13480,"ancestor_record_offset":6556,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":13480,"parent_record_offset":13452,"ancestor_record_offset":6556,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":14196,"parent_record_offset":14172,"ancestor_record_offset":6556,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":13548,"parent_record_offset":13512,"ancestor_record_offset":6556,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":524,"record_offset":17892,"parent_record_offset":17864,"ancestor_record_offset":6556,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":524,"record_offset":17920,"parent_record_offset":17892,"ancestor_record_offset":6556,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":524,"record_offset":13388,"parent_record_offset":13324,"ancestor_record_offset":6556,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":524,"record_offset":17864,"parent_record_offset":17836,"ancestor_record_offset":6556,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":524,"record_offset":17836,"parent_record_offset":17808,"ancestor_record_offset":6556,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":524,"record_offset":17808,"parent_record_offset":17764,"ancestor_record_offset":6556,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":524,"record_offset":23624,"parent_record_offset":23596,"ancestor_record_offset":6556,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$84::cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":524,"record_offset":12164,"parent_record_offset":11988,"ancestor_record_offset":6556,"inlinee":9450,"source_line":350,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":15252,"parent_record_offset":15208,"ancestor_record_offset":6556,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":13360,"parent_record_offset":13324,"ancestor_record_offset":6556,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":9872,"parent_record_offset":6704,"ancestor_record_offset":6556,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":15180,"parent_record_offset":6704,"ancestor_record_offset":6556,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":15208,"parent_record_offset":15180,"ancestor_record_offset":6556,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":13424,"parent_record_offset":13388,"ancestor_record_offset":6556,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":13296,"parent_record_offset":13264,"ancestor_record_offset":6556,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":16764,"parent_record_offset":6704,"ancestor_record_offset":6556,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":12188,"parent_record_offset":12164,"ancestor_record_offset":6556,"inlinee":9440,"source_line":4212,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":13324,"parent_record_offset":13296,"ancestor_record_offset":6556,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":524,"record_offset":21060,"parent_record_offset":21036,"ancestor_record_offset":6556,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":524,"record_offset":21848,"parent_record_offset":21824,"ancestor_record_offset":6556,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":524,"record_offset":16828,"parent_record_offset":6704,"ancestor_record_offset":6556,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":524,"record_offset":23652,"parent_record_offset":23624,"ancestor_record_offset":6556,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":524,"record_offset":13264,"parent_record_offset":13228,"ancestor_record_offset":6556,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":524,"record_offset":13572,"parent_record_offset":13548,"ancestor_record_offset":6556,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":524,"record_offset":9392,"parent_record_offset":9364,"ancestor_record_offset":6556,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":524,"record_offset":10496,"parent_record_offset":10468,"ancestor_record_offset":6556,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":524,"record_offset":9420,"parent_record_offset":9392,"ancestor_record_offset":6556,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":524,"record_offset":9332,"parent_record_offset":9300,"ancestor_record_offset":6556,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":524,"record_offset":9364,"parent_record_offset":9332,"ancestor_record_offset":6556,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":524,"record_offset":9480,"parent_record_offset":9364,"ancestor_record_offset":6556,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":524,"record_offset":9592,"parent_record_offset":9568,"ancestor_record_offset":6556,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":524,"record_offset":13956,"parent_record_offset":6704,"ancestor_record_offset":6556,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$71::cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":524,"record_offset":12268,"parent_record_offset":12236,"ancestor_record_offset":6556,"inlinee":5463,"source_line":2010,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":524,"record_offset":23596,"parent_record_offset":23568,"ancestor_record_offset":6556,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":524,"record_offset":17068,"parent_record_offset":17044,"ancestor_record_offset":6556,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":524,"record_offset":10880,"parent_record_offset":10856,"ancestor_record_offset":6556,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":524,"record_offset":23152,"parent_record_offset":23128,"ancestor_record_offset":6556,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":524,"record_offset":23236,"parent_record_offset":23212,"ancestor_record_offset":6556,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":524,"record_offset":23764,"parent_record_offset":23068,"ancestor_record_offset":6556,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":524,"record_offset":13620,"parent_record_offset":13596,"ancestor_record_offset":6556,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":524,"record_offset":12048,"parent_record_offset":11988,"ancestor_record_offset":6556,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":524,"record_offset":22860,"parent_record_offset":6704,"ancestor_record_offset":6556,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":524,"record_offset":15836,"parent_record_offset":15800,"ancestor_record_offset":6556,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":524,"record_offset":12400,"parent_record_offset":12376,"ancestor_record_offset":6556,"inlinee":4923,"source_line":775,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":524,"record_offset":9444,"parent_record_offset":9420,"ancestor_record_offset":6556,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":524,"record_offset":11136,"parent_record_offset":11048,"ancestor_record_offset":6556,"inlinee":4369,"source_line":761,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":524,"record_offset":8784,"parent_record_offset":8756,"ancestor_record_offset":6556,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":524,"record_offset":8812,"parent_record_offset":8784,"ancestor_record_offset":6556,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":524,"record_offset":21976,"parent_record_offset":21948,"ancestor_record_offset":6556,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":524,"record_offset":10524,"parent_record_offset":10468,"ancestor_record_offset":6556,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":524,"record_offset":11572,"parent_record_offset":11548,"ancestor_record_offset":6556,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":524,"record_offset":11548,"parent_record_offset":11524,"ancestor_record_offset":6556,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":524,"record_offset":9200,"parent_record_offset":9152,"ancestor_record_offset":6556,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":524,"record_offset":11892,"parent_record_offset":11864,"ancestor_record_offset":6556,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":524,"record_offset":13980,"parent_record_offset":13956,"ancestor_record_offset":6556,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":524,"record_offset":10224,"parent_record_offset":10192,"ancestor_record_offset":6556,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":524,"record_offset":7452,"parent_record_offset":7424,"ancestor_record_offset":6556,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":524,"record_offset":9828,"parent_record_offset":9152,"ancestor_record_offset":6556,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":524,"record_offset":13788,"parent_record_offset":13388,"ancestor_record_offset":6556,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":524,"record_offset":12424,"parent_record_offset":12400,"ancestor_record_offset":6556,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":13812,"parent_record_offset":13788,"ancestor_record_offset":6556,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=108
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":8516,"parent_record_offset":6704,"ancestor_record_offset":6556,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":524,"record_offset":16792,"parent_record_offset":16764,"ancestor_record_offset":6556,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":524,"record_offset":16284,"parent_record_offset":16260,"ancestor_record_offset":6556,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":524,"record_offset":10312,"parent_record_offset":10192,"ancestor_record_offset":6556,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":524,"record_offset":8848,"parent_record_offset":8724,"ancestor_record_offset":6556,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":12128,"parent_record_offset":12072,"ancestor_record_offset":6556,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":7924,"parent_record_offset":7900,"ancestor_record_offset":6556,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":16916,"parent_record_offset":16828,"ancestor_record_offset":6556,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":7900,"parent_record_offset":7876,"ancestor_record_offset":6556,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":8108,"parent_record_offset":8080,"ancestor_record_offset":6556,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":12448,"parent_record_offset":12424,"ancestor_record_offset":6556,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":10336,"parent_record_offset":10312,"ancestor_record_offset":6556,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":15372,"parent_record_offset":15208,"ancestor_record_offset":6556,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":524,"record_offset":17100,"parent_record_offset":16976,"ancestor_record_offset":6556,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":524,"record_offset":17136,"parent_record_offset":16976,"ancestor_record_offset":6556,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":524,"record_offset":23680,"parent_record_offset":23652,"ancestor_record_offset":6556,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$15::compare
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":524,"record_offset":12236,"parent_record_offset":12212,"ancestor_record_offset":6556,"inlinee":9435,"source_line":311,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$2::cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":524,"record_offset":12212,"parent_record_offset":12188,"ancestor_record_offset":6556,"inlinee":9442,"source_line":37,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":524,"record_offset":8208,"parent_record_offset":8180,"ancestor_record_offset":6556,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":524,"record_offset":23708,"parent_record_offset":23680,"ancestor_record_offset":6556,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":524,"record_offset":8180,"parent_record_offset":8152,"ancestor_record_offset":6556,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":7852,"parent_record_offset":7828,"ancestor_record_offset":6556,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":7876,"parent_record_offset":7852,"ancestor_record_offset":6556,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":8236,"parent_record_offset":8208,"ancestor_record_offset":6556,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":12072,"parent_record_offset":12048,"ancestor_record_offset":6556,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":7828,"parent_record_offset":7712,"ancestor_record_offset":6556,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":524,"record_offset":7244,"parent_record_offset":7220,"ancestor_record_offset":6556,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":524,"record_offset":7540,"parent_record_offset":7504,"ancestor_record_offset":6556,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":524,"record_offset":7680,"parent_record_offset":7648,"ancestor_record_offset":6556,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":524,"record_offset":7648,"parent_record_offset":7612,"ancestor_record_offset":6556,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":524,"record_offset":7588,"parent_record_offset":7564,"ancestor_record_offset":6556,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":524,"record_offset":7612,"parent_record_offset":7588,"ancestor_record_offset":6556,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":524,"record_offset":7564,"parent_record_offset":7540,"ancestor_record_offset":6556,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":524,"record_offset":8152,"parent_record_offset":7612,"ancestor_record_offset":6556,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":524,"record_offset":7220,"parent_record_offset":7188,"ancestor_record_offset":6556,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":524,"record_offset":7188,"parent_record_offset":7160,"ancestor_record_offset":6556,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":524,"record_offset":7344,"parent_record_offset":7312,"ancestor_record_offset":6556,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":524,"record_offset":7400,"parent_record_offset":7344,"ancestor_record_offset":6556,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":524,"record_offset":7312,"parent_record_offset":7284,"ancestor_record_offset":6556,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":524,"record_offset":7160,"parent_record_offset":7116,"ancestor_record_offset":6556,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":524,"record_offset":7424,"parent_record_offset":7400,"ancestor_record_offset":6556,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":524,"record_offset":7712,"parent_record_offset":7680,"ancestor_record_offset":6556,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":524,"record_offset":7996,"parent_record_offset":7712,"ancestor_record_offset":6556,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":524,"record_offset":7968,"parent_record_offset":7712,"ancestor_record_offset":6556,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":524,"record_offset":7772,"parent_record_offset":7712,"ancestor_record_offset":6556,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":524,"record_offset":13896,"parent_record_offset":13872,"ancestor_record_offset":6556,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":524,"record_offset":13920,"parent_record_offset":13896,"ancestor_record_offset":6556,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::Asset::name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":524,"record_offset":17764,"parent_record_offset":16976,"ancestor_record_offset":6556,"inlinee":21763,"source_line":34,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::Asset::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":524,"record_offset":17736,"parent_record_offset":16976,"ancestor_record_offset":6556,"inlinee":21762,"source_line":30,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_mod_textures::reload_replaced_textures::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_mod_textures.rs
//     evidence={"module":524,"record_offset":16976,"parent_record_offset":6704,"ancestor_record_offset":6556,"inlinee":30971,"source_line":84,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":524,"record_offset":11492,"parent_record_offset":11456,"ancestor_record_offset":6556,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":524,"record_offset":11524,"parent_record_offset":11492,"ancestor_record_offset":6556,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":524,"record_offset":11612,"parent_record_offset":11456,"ancestor_record_offset":6556,"inlinee":4664,"source_line":37,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":524,"record_offset":11456,"parent_record_offset":11408,"ancestor_record_offset":6556,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":524,"record_offset":23128,"parent_record_offset":23068,"ancestor_record_offset":6556,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":524,"record_offset":10856,"parent_record_offset":10820,"ancestor_record_offset":6556,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":524,"record_offset":23792,"parent_record_offset":23068,"ancestor_record_offset":6556,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":524,"record_offset":23212,"parent_record_offset":23068,"ancestor_record_offset":6556,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":524,"record_offset":23916,"parent_record_offset":23068,"ancestor_record_offset":6556,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":524,"record_offset":23568,"parent_record_offset":23540,"ancestor_record_offset":6556,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":524,"record_offset":22976,"parent_record_offset":22948,"ancestor_record_offset":6556,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":524,"record_offset":7080,"parent_record_offset":7056,"ancestor_record_offset":6556,"inlinee":12123,"source_line":845,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":524,"record_offset":22608,"parent_record_offset":22584,"ancestor_record_offset":6556,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::keys
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":524,"record_offset":22584,"parent_record_offset":22560,"ancestor_record_offset":6556,"inlinee":9032,"source_line":650,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":524,"record_offset":23540,"parent_record_offset":23432,"ancestor_record_offset":6556,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$42::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":524,"record_offset":10700,"parent_record_offset":10672,"ancestor_record_offset":6556,"inlinee":11335,"source_line":3199,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":524,"record_offset":24096,"parent_record_offset":24072,"ancestor_record_offset":6556,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$53::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":524,"record_offset":11328,"parent_record_offset":11300,"ancestor_record_offset":6556,"inlinee":11343,"source_line":3323,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$58::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":524,"record_offset":24072,"parent_record_offset":24048,"ancestor_record_offset":6556,"inlinee":9034,"source_line":3365,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":524,"record_offset":23984,"parent_record_offset":22976,"ancestor_record_offset":6556,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":16260,"parent_record_offset":15800,"ancestor_record_offset":6556,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":23480,"parent_record_offset":23456,"ancestor_record_offset":6556,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":11700,"parent_record_offset":11408,"ancestor_record_offset":6556,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":23884,"parent_record_offset":23068,"ancestor_record_offset":6556,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIter::drop_elements
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":15800,"parent_record_offset":15764,"ancestor_record_offset":6556,"inlinee":4991,"source_line":3818,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":10820,"parent_record_offset":10792,"ancestor_record_offset":6556,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":11408,"parent_record_offset":11376,"ancestor_record_offset":6556,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":23456,"parent_record_offset":23432,"ancestor_record_offset":6556,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":23040,"parent_record_offset":23012,"ancestor_record_offset":6556,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":23012,"parent_record_offset":22976,"ancestor_record_offset":6556,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_allocation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":11008,"parent_record_offset":10980,"ancestor_record_offset":6556,"inlinee":11339,"source_line":1487,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_iter_from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":10980,"parent_record_offset":10728,"ancestor_record_offset":6556,"inlinee":11338,"source_line":1473,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":10764,"parent_record_offset":10728,"ancestor_record_offset":6556,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":8724,"parent_record_offset":8692,"ancestor_record_offset":6556,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":22688,"parent_record_offset":22656,"ancestor_record_offset":6556,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":25032,"parent_record_offset":24972,"ancestor_record_offset":6556,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":8636,"parent_record_offset":8612,"ancestor_record_offset":6556,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":23068,"parent_record_offset":23040,"ancestor_record_offset":6556,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":8692,"parent_record_offset":8636,"ancestor_record_offset":6556,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":8664,"parent_record_offset":8636,"ancestor_record_offset":6556,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":10792,"parent_record_offset":10764,"ancestor_record_offset":6556,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":23944,"parent_record_offset":23068,"ancestor_record_offset":6556,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":8756,"parent_record_offset":8724,"ancestor_record_offset":6556,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":8612,"parent_record_offset":8588,"ancestor_record_offset":6556,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$19::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":10728,"parent_record_offset":10700,"ancestor_record_offset":6556,"inlinee":11337,"source_line":3530,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":11376,"parent_record_offset":11352,"ancestor_record_offset":6556,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$40::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":15764,"parent_record_offset":15732,"ancestor_record_offset":6556,"inlinee":4778,"source_line":4042,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$42::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":11352,"parent_record_offset":11328,"ancestor_record_offset":6556,"inlinee":11345,"source_line":4083,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":23432,"parent_record_offset":23068,"ancestor_record_offset":6556,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":524,"record_offset":22948,"parent_record_offset":22920,"ancestor_record_offset":6556,"inlinee":5589,"source_line":862,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":524,"record_offset":11244,"parent_record_offset":11220,"ancestor_record_offset":6556,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::is_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":524,"record_offset":7056,"parent_record_offset":7032,"ancestor_record_offset":6556,"inlinee":26555,"source_line":326,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":524,"record_offset":22560,"parent_record_offset":22536,"ancestor_record_offset":6556,"inlinee":11496,"source_line":290,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$22::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":524,"record_offset":10672,"parent_record_offset":10644,"ancestor_record_offset":6556,"inlinee":12098,"source_line":1771,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$25::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":524,"record_offset":24048,"parent_record_offset":24024,"ancestor_record_offset":6556,"inlinee":11498,"source_line":1798,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$30::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":524,"record_offset":11300,"parent_record_offset":11276,"ancestor_record_offset":6556,"inlinee":12102,"source_line":1840,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":524,"record_offset":14060,"parent_record_offset":14036,"ancestor_record_offset":6556,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":524,"record_offset":14012,"parent_record_offset":6704,"ancestor_record_offset":6556,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":524,"record_offset":14036,"parent_record_offset":14012,"ancestor_record_offset":6556,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":524,"record_offset":14084,"parent_record_offset":14060,"ancestor_record_offset":6556,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":524,"record_offset":13872,"parent_record_offset":6704,"ancestor_record_offset":6556,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":524,"record_offset":9036,"parent_record_offset":6704,"ancestor_record_offset":6556,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":524,"record_offset":9072,"parent_record_offset":9036,"ancestor_record_offset":6556,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":524,"record_offset":7116,"parent_record_offset":6704,"ancestor_record_offset":6556,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":524,"record_offset":27916,"parent_record_offset":27884,"ancestor_record_offset":6556,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":524,"record_offset":28396,"parent_record_offset":28180,"ancestor_record_offset":6556,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":524,"record_offset":9228,"parent_record_offset":9200,"ancestor_record_offset":6556,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":524,"record_offset":9300,"parent_record_offset":9152,"ancestor_record_offset":6556,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":524,"record_offset":9516,"parent_record_offset":9300,"ancestor_record_offset":6556,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":524,"record_offset":7504,"parent_record_offset":6704,"ancestor_record_offset":6556,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":524,"record_offset":22920,"parent_record_offset":22888,"ancestor_record_offset":6556,"inlinee":5587,"source_line":767,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::difference
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":524,"record_offset":22512,"parent_record_offset":6704,"ancestor_record_offset":6556,"inlinee":30972,"source_line":651,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":524,"record_offset":11220,"parent_record_offset":6704,"ancestor_record_offset":6556,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":524,"record_offset":7032,"parent_record_offset":6704,"ancestor_record_offset":6556,"inlinee":26554,"source_line":385,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":524,"record_offset":22536,"parent_record_offset":22512,"ancestor_record_offset":6556,"inlinee":12128,"source_line":349,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":524,"record_offset":9928,"parent_record_offset":6704,"ancestor_record_offset":6556,"inlinee":5639,"source_line":149,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$11::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":524,"record_offset":22148,"parent_record_offset":6704,"ancestor_record_offset":6556,"inlinee":11866,"source_line":1200,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$13::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":524,"record_offset":9956,"parent_record_offset":9928,"ancestor_record_offset":6556,"inlinee":5641,"source_line":1246,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$21::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":524,"record_offset":10644,"parent_record_offset":6704,"ancestor_record_offset":6556,"inlinee":12096,"source_line":1659,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$23::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":524,"record_offset":24024,"parent_record_offset":22888,"ancestor_record_offset":6556,"inlinee":16842,"source_line":1676,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$27::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":524,"record_offset":11276,"parent_record_offset":6704,"ancestor_record_offset":6556,"inlinee":12100,"source_line":1718,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$43::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":524,"record_offset":22888,"parent_record_offset":6704,"ancestor_record_offset":6556,"inlinee":30974,"source_line":1905,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":524,"record_offset":13192,"parent_record_offset":13156,"ancestor_record_offset":6556,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":524,"record_offset":17044,"parent_record_offset":16976,"ancestor_record_offset":6556,"inlinee":7505,"source_line":341,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":524,"record_offset":10012,"parent_record_offset":9984,"ancestor_record_offset":6556,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":524,"record_offset":10272,"parent_record_offset":10248,"ancestor_record_offset":6556,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":524,"record_offset":10132,"parent_record_offset":10104,"ancestor_record_offset":6556,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":524,"record_offset":10468,"parent_record_offset":10068,"ancestor_record_offset":6556,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":524,"record_offset":9984,"parent_record_offset":9956,"ancestor_record_offset":6556,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":524,"record_offset":13156,"parent_record_offset":6704,"ancestor_record_offset":6556,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":524,"record_offset":13228,"parent_record_offset":13192,"ancestor_record_offset":6556,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":524,"record_offset":9256,"parent_record_offset":9228,"ancestor_record_offset":6556,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":524,"record_offset":10192,"parent_record_offset":10160,"ancestor_record_offset":6556,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":524,"record_offset":9152,"parent_record_offset":9108,"ancestor_record_offset":6556,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":524,"record_offset":9108,"parent_record_offset":9072,"ancestor_record_offset":6556,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] tg_mods::overlay::impl$30::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/overlay.rs
//     evidence={"module":524,"record_offset":14412,"parent_record_offset":6704,"ancestor_record_offset":6556,"inlinee":30969,"source_line":787,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mod_textures::reload_replaced_textures
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 51
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78F7
// 首条位置: {"module":524,"inlinee":30967}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::collections::btree::map::BTreeMap::contains_key
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":524,"record_offset":12496,"parent_record_offset":6704,"ancestor_record_offset":6704,"inlinee":21140,"source_line":980,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] alloc::collections::btree::map::BTreeMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":524,"record_offset":11864,"parent_record_offset":6704,"ancestor_record_offset":6704,"inlinee":7203,"source_line":718,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":9872,"parent_record_offset":6704,"ancestor_record_offset":6704,"inlinee":6153,"source_line":463,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::Vec::push
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":15180,"parent_record_offset":6704,"ancestor_record_offset":6704,"inlinee":4492,"source_line":991,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":16764,"parent_record_offset":6704,"ancestor_record_offset":6704,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":524,"record_offset":16828,"parent_record_offset":6704,"ancestor_record_offset":6704,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":524,"record_offset":13956,"parent_record_offset":6704,"ancestor_record_offset":6704,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":524,"record_offset":22860,"parent_record_offset":6704,"ancestor_record_offset":6704,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=18
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":8516,"parent_record_offset":6704,"ancestor_record_offset":6704,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_mod_textures::reload_replaced_textures::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_mod_textures.rs
//     evidence={"module":524,"record_offset":16976,"parent_record_offset":6704,"ancestor_record_offset":6704,"inlinee":30971,"source_line":84,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":524,"record_offset":14012,"parent_record_offset":6704,"ancestor_record_offset":6704,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":524,"record_offset":13872,"parent_record_offset":6704,"ancestor_record_offset":6704,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] puffin::ProfilerScope::new
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":524,"record_offset":9036,"parent_record_offset":6704,"ancestor_record_offset":6704,"inlinee":8276,"source_line":589,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] puffin::clean_function_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":524,"record_offset":7116,"parent_record_offset":6704,"ancestor_record_offset":6704,"inlinee":9267,"source_line":626,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] puffin::short_file_name
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":524,"record_offset":7504,"parent_record_offset":6704,"ancestor_record_offset":6704,"inlinee":8242,"source_line":659,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::difference
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":524,"record_offset":22512,"parent_record_offset":6704,"ancestor_record_offset":6704,"inlinee":30972,"source_line":651,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":524,"record_offset":11220,"parent_record_offset":6704,"ancestor_record_offset":6704,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::is_empty
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":524,"record_offset":7032,"parent_record_offset":6704,"ancestor_record_offset":6704,"inlinee":26554,"source_line":385,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":524,"record_offset":9928,"parent_record_offset":6704,"ancestor_record_offset":6704,"inlinee":5639,"source_line":149,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$11::extend
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":524,"record_offset":22148,"parent_record_offset":6704,"ancestor_record_offset":6704,"inlinee":11866,"source_line":1200,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$21::into_iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":524,"record_offset":10644,"parent_record_offset":6704,"ancestor_record_offset":6704,"inlinee":12096,"source_line":1659,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$27::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":524,"record_offset":11276,"parent_record_offset":6704,"ancestor_record_offset":6704,"inlinee":12100,"source_line":1718,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$43::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":524,"record_offset":22888,"parent_record_offset":6704,"ancestor_record_offset":6704,"inlinee":30974,"source_line":1905,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":524,"record_offset":13156,"parent_record_offset":6704,"ancestor_record_offset":6704,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] tg_mods::overlay::impl$30::clone
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/tg-mods/src/overlay.rs
//     evidence={"module":524,"record_offset":14412,"parent_record_offset":6704,"ancestor_record_offset":6704,"inlinee":30969,"source_line":787,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":13684,"parent_record_offset":13660,"ancestor_record_offset":6704,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":13708,"parent_record_offset":13684,"ancestor_record_offset":6704,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":8908,"parent_record_offset":8880,"ancestor_record_offset":6704,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":8936,"parent_record_offset":8908,"ancestor_record_offset":6704,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":13732,"parent_record_offset":13708,"ancestor_record_offset":6704,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":8968,"parent_record_offset":8936,"ancestor_record_offset":6704,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":13660,"parent_record_offset":13512,"ancestor_record_offset":6704,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=24
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":8880,"parent_record_offset":8692,"ancestor_record_offset":6704,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::map::BTreeMap::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\map.rs
//     evidence={"module":524,"record_offset":12520,"parent_record_offset":12496,"ancestor_record_offset":6704,"inlinee":7203,"source_line":718,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::descend
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":524,"record_offset":12376,"parent_record_offset":11928,"ancestor_record_offset":6704,"inlinee":4921,"source_line":1102,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::Handle::force
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":524,"record_offset":12320,"parent_record_offset":11928,"ancestor_record_offset":6704,"inlinee":4920,"source_line":1681,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::find_key_index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\search.rs
//     evidence={"module":524,"record_offset":11988,"parent_record_offset":11964,"ancestor_record_offset":6704,"inlinee":5456,"source_line":217,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::force
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":524,"record_offset":12344,"parent_record_offset":12320,"ancestor_record_offset":6704,"inlinee":4916,"source_line":725,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::keys
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\node.rs
//     evidence={"module":524,"record_offset":12020,"parent_record_offset":11988,"ancestor_record_offset":6704,"inlinee":5457,"source_line":394,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::search_node
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\search.rs
//     evidence={"module":524,"record_offset":11964,"parent_record_offset":11928,"ancestor_record_offset":6704,"inlinee":5455,"source_line":195,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::collections::btree::node::NodeRef::search_tree
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\collections\btree\search.rs
//     evidence={"module":524,"record_offset":11928,"parent_record_offset":11864,"ancestor_record_offset":6704,"inlinee":5454,"source_line":49,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":15284,"parent_record_offset":15252,"ancestor_record_offset":6704,"inlinee":4180,"source_line":294,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":13452,"parent_record_offset":13424,"ancestor_record_offset":6704,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":14248,"parent_record_offset":14220,"ancestor_record_offset":6704,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":14220,"parent_record_offset":14196,"ancestor_record_offset":6704,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::non_null
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":15332,"parent_record_offset":15308,"ancestor_record_offset":6704,"inlinee":4182,"source_line":611,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":15308,"parent_record_offset":15284,"ancestor_record_offset":6704,"inlinee":4181,"source_line":606,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":13512,"parent_record_offset":13480,"ancestor_record_offset":6704,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":13480,"parent_record_offset":13452,"ancestor_record_offset":6704,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":14196,"parent_record_offset":14172,"ancestor_record_offset":6704,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":13548,"parent_record_offset":13512,"ancestor_record_offset":6704,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":524,"record_offset":17892,"parent_record_offset":17864,"ancestor_record_offset":6704,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":524,"record_offset":17920,"parent_record_offset":17892,"ancestor_record_offset":6704,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":524,"record_offset":13388,"parent_record_offset":13324,"ancestor_record_offset":6704,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":524,"record_offset":17864,"parent_record_offset":17836,"ancestor_record_offset":6704,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":524,"record_offset":17836,"parent_record_offset":17808,"ancestor_record_offset":6704,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":524,"record_offset":17808,"parent_record_offset":17764,"ancestor_record_offset":6704,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":524,"record_offset":23624,"parent_record_offset":23596,"ancestor_record_offset":6704,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$84::cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":524,"record_offset":12164,"parent_record_offset":11988,"ancestor_record_offset":6704,"inlinee":9450,"source_line":350,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_mut_ptr
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":15252,"parent_record_offset":15208,"ancestor_record_offset":6704,"inlinee":4485,"source_line":1997,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::as_slice
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":13360,"parent_record_offset":13324,"ancestor_record_offset":6704,"inlinee":4177,"source_line":1798,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::push_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":15208,"parent_record_offset":15180,"ancestor_record_offset":6704,"inlinee":4493,"source_line":1023,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":13424,"parent_record_offset":13388,"ancestor_record_offset":6704,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$12::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":13296,"parent_record_offset":13264,"ancestor_record_offset":6704,"inlinee":5763,"source_line":3747,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$25::cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":12188,"parent_record_offset":12164,"ancestor_record_offset":6704,"inlinee":9440,"source_line":4212,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$9::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":13324,"parent_record_offset":13296,"ancestor_record_offset":6704,"inlinee":4175,"source_line":3728,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":524,"record_offset":21060,"parent_record_offset":21036,"ancestor_record_offset":6704,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":524,"record_offset":21848,"parent_record_offset":21824,"ancestor_record_offset":6704,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":524,"record_offset":23652,"parent_record_offset":23624,"ancestor_record_offset":6704,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::wtf8::impl$14::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\wtf8\mod.rs
//     evidence={"module":524,"record_offset":13264,"parent_record_offset":13228,"ancestor_record_offset":6704,"inlinee":6712,"source_line":38,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":524,"record_offset":13572,"parent_record_offset":13548,"ancestor_record_offset":6704,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::BorrowRefMut::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":524,"record_offset":9392,"parent_record_offset":9364,"ancestor_record_offset":6704,"inlinee":6507,"source_line":2061,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":524,"record_offset":10496,"parent_record_offset":10468,"ancestor_record_offset":6704,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":524,"record_offset":9420,"parent_record_offset":9392,"ancestor_record_offset":6704,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":524,"record_offset":9332,"parent_record_offset":9300,"ancestor_record_offset":6704,"inlinee":6546,"source_line":1221,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::RefCell::try_borrow_mut
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":524,"record_offset":9364,"parent_record_offset":9332,"ancestor_record_offset":6704,"inlinee":6505,"source_line":1255,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::UnsafeCell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":524,"record_offset":9480,"parent_record_offset":9364,"ancestor_record_offset":6704,"inlinee":5177,"source_line":2443,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::impl$52::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":524,"record_offset":9592,"parent_record_offset":9568,"ancestor_record_offset":6704,"inlinee":4995,"source_line":2052,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$71::cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":524,"record_offset":12268,"parent_record_offset":12236,"ancestor_record_offset":6704,"inlinee":5463,"source_line":2010,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":524,"record_offset":23596,"parent_record_offset":23568,"ancestor_record_offset":6704,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":524,"record_offset":17068,"parent_record_offset":17044,"ancestor_record_offset":6704,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_load_si128
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":524,"record_offset":10880,"parent_record_offset":10856,"ancestor_record_offset":6704,"inlinee":4655,"source_line":1319,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":524,"record_offset":23152,"parent_record_offset":23128,"ancestor_record_offset":6704,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":524,"record_offset":23236,"parent_record_offset":23212,"ancestor_record_offset":6704,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":524,"record_offset":23764,"parent_record_offset":23068,"ancestor_record_offset":6704,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":524,"record_offset":13620,"parent_record_offset":13596,"ancestor_record_offset":6704,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::enumerate::impl$1::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\enumerate.rs
//     evidence={"module":524,"record_offset":12048,"parent_record_offset":11988,"ancestor_record_offset":6704,"inlinee":5459,"source_line":79,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::impl$2::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":524,"record_offset":15836,"parent_record_offset":15800,"ancestor_record_offset":6704,"inlinee":4993,"source_line":4225,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::maybe_uninit::MaybeUninit::assume_init_read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\maybe_uninit.rs
//     evidence={"module":524,"record_offset":12400,"parent_record_offset":12376,"ancestor_record_offset":6704,"inlinee":4923,"source_line":775,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":524,"record_offset":9444,"parent_record_offset":9420,"ancestor_record_offset":6704,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":524,"record_offset":11136,"parent_record_offset":11048,"ancestor_record_offset":6704,"inlinee":4369,"source_line":761,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=7
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":524,"record_offset":8784,"parent_record_offset":8756,"ancestor_record_offset":6704,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::overflowing_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":524,"record_offset":8812,"parent_record_offset":8784,"ancestor_record_offset":6704,"inlinee":4663,"source_line":3050,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":524,"record_offset":21976,"parent_record_offset":21948,"ancestor_record_offset":6704,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":524,"record_offset":10524,"parent_record_offset":10468,"ancestor_record_offset":6704,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":524,"record_offset":11572,"parent_record_offset":11548,"ancestor_record_offset":6704,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":524,"record_offset":11548,"parent_record_offset":11524,"ancestor_record_offset":6704,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":524,"record_offset":9200,"parent_record_offset":9152,"ancestor_record_offset":6704,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":524,"record_offset":11892,"parent_record_offset":11864,"ancestor_record_offset":6704,"inlinee":4170,"source_line":744,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":524,"record_offset":13980,"parent_record_offset":13956,"ancestor_record_offset":6704,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":524,"record_offset":10224,"parent_record_offset":10192,"ancestor_record_offset":6704,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::add
//     owner=rust_standard_library; count=12
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":524,"record_offset":7452,"parent_record_offset":7424,"ancestor_record_offset":6704,"inlinee":4656,"source_line":829,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":524,"record_offset":9828,"parent_record_offset":9152,"ancestor_record_offset":6704,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":524,"record_offset":13788,"parent_record_offset":13388,"ancestor_record_offset":6704,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":524,"record_offset":12424,"parent_record_offset":12400,"ancestor_record_offset":6704,"inlinee":4924,"source_line":1166,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":13812,"parent_record_offset":13788,"ancestor_record_offset":6704,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=90
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":8540,"parent_record_offset":8516,"ancestor_record_offset":6704,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":524,"record_offset":16792,"parent_record_offset":16764,"ancestor_record_offset":6704,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":524,"record_offset":16284,"parent_record_offset":16260,"ancestor_record_offset":6704,"inlinee":4662,"source_line":1395,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":524,"record_offset":10312,"parent_record_offset":10192,"ancestor_record_offset":6704,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=15
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":524,"record_offset":8848,"parent_record_offset":8724,"ancestor_record_offset":6704,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":12128,"parent_record_offset":12072,"ancestor_record_offset":6704,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":7924,"parent_record_offset":7900,"ancestor_record_offset":6704,"inlinee":8268,"source_line":575,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":16916,"parent_record_offset":16828,"ancestor_record_offset":6704,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":7900,"parent_record_offset":7876,"ancestor_record_offset":6704,"inlinee":8267,"source_line":728,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":8108,"parent_record_offset":8080,"ancestor_record_offset":6704,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":12448,"parent_record_offset":12424,"ancestor_record_offset":6704,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":10336,"parent_record_offset":10312,"ancestor_record_offset":6704,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::write
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":15372,"parent_record_offset":15208,"ancestor_record_offset":6704,"inlinee":4494,"source_line":1894,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":524,"record_offset":17100,"parent_record_offset":16976,"ancestor_record_offset":6704,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":524,"record_offset":17136,"parent_record_offset":16976,"ancestor_record_offset":6704,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":524,"record_offset":23680,"parent_record_offset":23652,"ancestor_record_offset":6704,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$15::compare
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":524,"record_offset":12236,"parent_record_offset":12212,"ancestor_record_offset":6704,"inlinee":9435,"source_line":311,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$2::cmp
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":524,"record_offset":12212,"parent_record_offset":12188,"ancestor_record_offset":6704,"inlinee":9442,"source_line":37,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$49::slice_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":524,"record_offset":8208,"parent_record_offset":8180,"ancestor_record_offset":6704,"inlinee":8260,"source_line":405,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":524,"record_offset":23708,"parent_record_offset":23680,"ancestor_record_offset":6704,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":524,"record_offset":8180,"parent_record_offset":8152,"ancestor_record_offset":6704,"inlinee":8258,"source_line":2589,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::next_back_unchecked
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":7852,"parent_record_offset":7828,"ancestor_record_offset":6704,"inlinee":8265,"source_line":81,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::pre_dec_end
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":7876,"parent_record_offset":7852,"ancestor_record_offset":6704,"inlinee":8266,"source_line":119,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":8236,"parent_record_offset":8208,"ancestor_record_offset":6704,"inlinee":8261,"source_line":326,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":12072,"parent_record_offset":12048,"ancestor_record_offset":6704,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$172::next_back
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":7828,"parent_record_offset":7712,"ancestor_record_offset":6704,"inlinee":8264,"source_line":438,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::is_char_boundary
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":524,"record_offset":7244,"parent_record_offset":7220,"ancestor_record_offset":6704,"inlinee":9253,"source_line":377,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::impl$0::rfind
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":524,"record_offset":7540,"parent_record_offset":7504,"ancestor_record_offset":6704,"inlinee":8243,"source_line":1527,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$2::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":524,"record_offset":7680,"parent_record_offset":7648,"ancestor_record_offset":6704,"inlinee":8253,"source_line":125,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::iter::impl$6::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\iter.rs
//     evidence={"module":524,"record_offset":7648,"parent_record_offset":7612,"ancestor_record_offset":6704,"inlinee":8251,"source_line":215,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::ReverseSearcher::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":524,"record_offset":7588,"parent_record_offset":7564,"ancestor_record_offset":6704,"inlinee":8247,"source_line":315,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$11::next_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":524,"record_offset":7612,"parent_record_offset":7588,"ancestor_record_offset":6704,"inlinee":8249,"source_line":709,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$22::next_match_back
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":524,"record_offset":7564,"parent_record_offset":7540,"ancestor_record_offset":6704,"inlinee":8245,"source_line":798,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$8::matches
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":524,"record_offset":8152,"parent_record_offset":7612,"ancestor_record_offset":6704,"inlinee":8257,"source_line":659,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":524,"record_offset":7220,"parent_record_offset":7188,"ancestor_record_offset":6704,"inlinee":9270,"source_line":438,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$10::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":524,"record_offset":7188,"parent_record_offset":7160,"ancestor_record_offset":6704,"inlinee":9269,"source_line":468,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":524,"record_offset":7344,"parent_record_offset":7312,"ancestor_record_offset":6704,"inlinee":9252,"source_line":507,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":524,"record_offset":7400,"parent_record_offset":7344,"ancestor_record_offset":6704,"inlinee":9254,"source_line":527,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$11::index
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":524,"record_offset":7312,"parent_record_offset":7284,"ancestor_record_offset":6704,"inlinee":8274,"source_line":539,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$4::index
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":524,"record_offset":7160,"parent_record_offset":7116,"ancestor_record_offset":6704,"inlinee":8272,"source_line":62,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$7::get_unchecked
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":524,"record_offset":7424,"parent_record_offset":7400,"ancestor_record_offset":6704,"inlinee":8315,"source_line":189,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::next_code_point_reverse
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":524,"record_offset":7712,"parent_record_offset":7680,"ancestor_record_offset":6704,"inlinee":8255,"source_line":78,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_acc_cont_byte
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":524,"record_offset":7996,"parent_record_offset":7712,"ancestor_record_offset":6704,"inlinee":8270,"source_line":16,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_first_byte
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":524,"record_offset":7968,"parent_record_offset":7712,"ancestor_record_offset":6704,"inlinee":8269,"source_line":10,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::validations::utf8_is_cont_byte
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\validations.rs
//     evidence={"module":524,"record_offset":7772,"parent_record_offset":7712,"ancestor_record_offset":6704,"inlinee":8262,"source_line":23,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":524,"record_offset":13896,"parent_record_offset":13872,"ancestor_record_offset":6704,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":524,"record_offset":13920,"parent_record_offset":13896,"ancestor_record_offset":6704,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::Asset::name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":524,"record_offset":17764,"parent_record_offset":16976,"ancestor_record_offset":6704,"inlinee":21763,"source_line":34,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::Asset::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":524,"record_offset":17736,"parent_record_offset":16976,"ancestor_record_offset":6704,"inlinee":21762,"source_line":30,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":524,"record_offset":11492,"parent_record_offset":11456,"ancestor_record_offset":6704,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":524,"record_offset":11524,"parent_record_offset":11492,"ancestor_record_offset":6704,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::remove_lowest_bit
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":524,"record_offset":11612,"parent_record_offset":11456,"ancestor_record_offset":6704,"inlinee":4664,"source_line":37,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=6
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":524,"record_offset":11456,"parent_record_offset":11408,"ancestor_record_offset":6704,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":524,"record_offset":23128,"parent_record_offset":23068,"ancestor_record_offset":6704,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load_aligned
//     owner=package_namespace_candidate; count=9
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":524,"record_offset":10856,"parent_record_offset":10820,"ancestor_record_offset":6704,"inlinee":4654,"source_line":59,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":524,"record_offset":23792,"parent_record_offset":23068,"ancestor_record_offset":6704,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":524,"record_offset":23212,"parent_record_offset":23068,"ancestor_record_offset":6704,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":524,"record_offset":23916,"parent_record_offset":23068,"ancestor_record_offset":6704,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":524,"record_offset":23568,"parent_record_offset":23540,"ancestor_record_offset":6704,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":524,"record_offset":22976,"parent_record_offset":22948,"ancestor_record_offset":6704,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":524,"record_offset":7080,"parent_record_offset":7056,"ancestor_record_offset":6704,"inlinee":12123,"source_line":845,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":524,"record_offset":22608,"parent_record_offset":22584,"ancestor_record_offset":6704,"inlinee":8380,"source_line":754,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::keys
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":524,"record_offset":22584,"parent_record_offset":22560,"ancestor_record_offset":6704,"inlinee":9032,"source_line":650,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":524,"record_offset":23540,"parent_record_offset":23432,"ancestor_record_offset":6704,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$42::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":524,"record_offset":10700,"parent_record_offset":10672,"ancestor_record_offset":6704,"inlinee":11335,"source_line":3199,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$44::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":524,"record_offset":24096,"parent_record_offset":24072,"ancestor_record_offset":6704,"inlinee":8502,"source_line":3219,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$53::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":524,"record_offset":11328,"parent_record_offset":11300,"ancestor_record_offset":6704,"inlinee":11343,"source_line":3323,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::impl$58::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":524,"record_offset":24072,"parent_record_offset":24048,"ancestor_record_offset":6704,"inlinee":9034,"source_line":3365,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":524,"record_offset":23984,"parent_record_offset":22976,"ancestor_record_offset":6704,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::drop
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":16260,"parent_record_offset":15800,"ancestor_record_offset":6704,"inlinee":4661,"source_line":518,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":23480,"parent_record_offset":23456,"ancestor_record_offset":6704,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::next_n
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":11700,"parent_record_offset":11408,"ancestor_record_offset":6704,"inlinee":4660,"source_line":490,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":23884,"parent_record_offset":23068,"ancestor_record_offset":6704,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIter::drop_elements
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":15800,"parent_record_offset":15764,"ancestor_record_offset":6704,"inlinee":4991,"source_line":3818,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::new
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":10820,"parent_record_offset":10792,"ancestor_record_offset":6704,"inlinee":4653,"source_line":3584,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawIterRange::next_impl
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":11408,"parent_record_offset":11376,"ancestor_record_offset":6704,"inlinee":4659,"source_line":3651,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":23456,"parent_record_offset":23432,"ancestor_record_offset":6704,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":23040,"parent_record_offset":23012,"ancestor_record_offset":6704,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":23012,"parent_record_offset":22976,"ancestor_record_offset":6704,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_allocation
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":11008,"parent_record_offset":10980,"ancestor_record_offset":6704,"inlinee":11339,"source_line":1487,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::into_iter_from
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":10980,"parent_record_offset":10728,"ancestor_record_offset":6704,"inlinee":11338,"source_line":1473,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::iter
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":10764,"parent_record_offset":10728,"ancestor_record_offset":6704,"inlinee":8381,"source_line":1395,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::allocation_info
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":8724,"parent_record_offset":8692,"ancestor_record_offset":6704,"inlinee":4366,"source_line":3160,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::data_end
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":22688,"parent_record_offset":22656,"ancestor_record_offset":6704,"inlinee":4651,"source_line":2478,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_elements
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":25032,"parent_record_offset":24972,"ancestor_record_offset":6704,"inlinee":4649,"source_line":2250,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::drop_inner_table
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":8636,"parent_record_offset":8612,"ancestor_record_offset":6704,"inlinee":4363,"source_line":2311,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":23068,"parent_record_offset":23040,"ancestor_record_offset":6704,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::free_buckets
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":8692,"parent_record_offset":8636,"ancestor_record_offset":6704,"inlinee":4365,"source_line":3130,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::is_empty_singleton
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":8664,"parent_record_offset":8636,"ancestor_record_offset":6704,"inlinee":4364,"source_line":2696,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::iter
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":10792,"parent_record_offset":10764,"ancestor_record_offset":6704,"inlinee":4650,"source_line":2178,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::probe_seq
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":23944,"parent_record_offset":23068,"ancestor_record_offset":6704,"inlinee":4305,"source_line":2488,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::TableLayout::calculate_layout_for
//     owner=package_namespace_candidate; count=4
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":8756,"parent_record_offset":8724,"ancestor_record_offset":6704,"inlinee":4368,"source_line":218,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$18::drop
//     owner=package_namespace_candidate; count=3
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":8612,"parent_record_offset":8588,"ancestor_record_offset":6704,"inlinee":4648,"source_line":3493,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$19::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":10728,"parent_record_offset":10700,"ancestor_record_offset":6704,"inlinee":11337,"source_line":3530,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$29::next
//     owner=package_namespace_candidate; count=5
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":11376,"parent_record_offset":11352,"ancestor_record_offset":6704,"inlinee":4658,"source_line":3848,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$40::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":15764,"parent_record_offset":15732,"ancestor_record_offset":6704,"inlinee":4778,"source_line":4042,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$42::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":11352,"parent_record_offset":11328,"ancestor_record_offset":6704,"inlinee":11345,"source_line":4083,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":524,"record_offset":23432,"parent_record_offset":23068,"ancestor_record_offset":6704,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":524,"record_offset":22948,"parent_record_offset":22920,"ancestor_record_offset":6704,"inlinee":5589,"source_line":862,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":524,"record_offset":11244,"parent_record_offset":11220,"ancestor_record_offset":6704,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::is_empty
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":524,"record_offset":7056,"parent_record_offset":7032,"ancestor_record_offset":6704,"inlinee":26555,"source_line":326,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":524,"record_offset":22560,"parent_record_offset":22536,"ancestor_record_offset":6704,"inlinee":11496,"source_line":290,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$22::into_iter
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":524,"record_offset":10672,"parent_record_offset":10644,"ancestor_record_offset":6704,"inlinee":12098,"source_line":1771,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$25::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":524,"record_offset":24048,"parent_record_offset":24024,"ancestor_record_offset":6704,"inlinee":11498,"source_line":1798,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::impl$30::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":524,"record_offset":11300,"parent_record_offset":11276,"ancestor_record_offset":6704,"inlinee":12102,"source_line":1840,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":524,"record_offset":14060,"parent_record_offset":14036,"ancestor_record_offset":6704,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":524,"record_offset":14036,"parent_record_offset":14012,"ancestor_record_offset":6704,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=5
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":524,"record_offset":14084,"parent_record_offset":14060,"ancestor_record_offset":6704,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] puffin::ThreadProfiler::call
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":524,"record_offset":9072,"parent_record_offset":9036,"ancestor_record_offset":6704,"inlinee":8278,"source_line":425,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":524,"record_offset":27916,"parent_record_offset":27884,"ancestor_record_offset":6704,"inlinee":8283,"source_line":601,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$10::drop::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":524,"record_offset":28396,"parent_record_offset":28180,"ancestor_record_offset":6704,"inlinee":6607,"source_line":602,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":524,"record_offset":9228,"parent_record_offset":9200,"ancestor_record_offset":6704,"inlinee":8280,"source_line":92,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$6::call::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":524,"record_offset":9300,"parent_record_offset":9152,"ancestor_record_offset":6704,"inlinee":6545,"source_line":429,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] puffin::impl$9::new::closure$0
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"puffin","version":"0.14.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/puffin-0.14.1/src/lib.rs
//     evidence={"module":524,"record_offset":9516,"parent_record_offset":9300,"ancestor_record_offset":6704,"inlinee":6548,"source_line":591,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":524,"record_offset":22920,"parent_record_offset":22888,"ancestor_record_offset":6704,"inlinee":5587,"source_line":767,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::HashSet::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":524,"record_offset":22536,"parent_record_offset":22512,"ancestor_record_offset":6704,"inlinee":12128,"source_line":349,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$13::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":524,"record_offset":9956,"parent_record_offset":9928,"ancestor_record_offset":6704,"inlinee":5641,"source_line":1246,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$23::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":524,"record_offset":24024,"parent_record_offset":22888,"ancestor_record_offset":6704,"inlinee":16842,"source_line":1676,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::ffi::os_str::impl$10::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\ffi\os_str.rs
//     evidence={"module":524,"record_offset":13192,"parent_record_offset":13156,"ancestor_record_offset":6704,"inlinee":6708,"source_line":699,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":524,"record_offset":17044,"parent_record_offset":16976,"ancestor_record_offset":6704,"inlinee":7505,"source_line":341,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":524,"record_offset":10012,"parent_record_offset":9984,"ancestor_record_offset":6704,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":524,"record_offset":10272,"parent_record_offset":10248,"ancestor_record_offset":6704,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":524,"record_offset":10132,"parent_record_offset":10104,"ancestor_record_offset":6704,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":524,"record_offset":10468,"parent_record_offset":10068,"ancestor_record_offset":6704,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":524,"record_offset":9984,"parent_record_offset":9956,"ancestor_record_offset":6704,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::path::impl$30::clone
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":524,"record_offset":14460,"parent_record_offset":14412,"ancestor_record_offset":6704,"inlinee":6706,"source_line":1874,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::os_str::wtf8::impl$7::clone
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\os_str\wtf8.rs
//     evidence={"module":524,"record_offset":13228,"parent_record_offset":13192,"ancestor_record_offset":6704,"inlinee":6710,"source_line":70,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":524,"record_offset":9256,"parent_record_offset":9228,"ancestor_record_offset":6704,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":524,"record_offset":10192,"parent_record_offset":10160,"ancestor_record_offset":6704,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":524,"record_offset":9152,"parent_record_offset":9108,"ancestor_record_offset":6704,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":524,"record_offset":9108,"parent_record_offset":9072,"ancestor_record_offset":6704,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mod_textures::reload_replaced_textures::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 84
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78FB
// 首条位置: {"module":524,"inlinee":30971}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":17196,"parent_record_offset":16976,"ancestor_record_offset":16976,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":524,"record_offset":17100,"parent_record_offset":16976,"ancestor_record_offset":16976,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":524,"record_offset":17136,"parent_record_offset":16976,"ancestor_record_offset":16976,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::Asset::name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":524,"record_offset":17764,"parent_record_offset":16976,"ancestor_record_offset":16976,"inlinee":21763,"source_line":34,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::Asset::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":524,"record_offset":17736,"parent_record_offset":16976,"ancestor_record_offset":16976,"inlinee":21762,"source_line":30,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":524,"record_offset":17044,"parent_record_offset":16976,"ancestor_record_offset":16976,"inlinee":7505,"source_line":341,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":18244,"parent_record_offset":18220,"ancestor_record_offset":16976,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":18268,"parent_record_offset":18244,"ancestor_record_offset":16976,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":17372,"parent_record_offset":17348,"ancestor_record_offset":16976,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":17396,"parent_record_offset":17372,"ancestor_record_offset":16976,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":18292,"parent_record_offset":18268,"ancestor_record_offset":16976,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":17420,"parent_record_offset":17396,"ancestor_record_offset":16976,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":18220,"parent_record_offset":18072,"ancestor_record_offset":16976,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":17348,"parent_record_offset":17292,"ancestor_record_offset":16976,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":18012,"parent_record_offset":17984,"ancestor_record_offset":16976,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":17320,"parent_record_offset":17292,"ancestor_record_offset":16976,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":17292,"parent_record_offset":17268,"ancestor_record_offset":16976,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":18072,"parent_record_offset":18040,"ancestor_record_offset":16976,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":18040,"parent_record_offset":18012,"ancestor_record_offset":16976,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":17268,"parent_record_offset":17244,"ancestor_record_offset":16976,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":18108,"parent_record_offset":18072,"ancestor_record_offset":16976,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":524,"record_offset":17892,"parent_record_offset":17864,"ancestor_record_offset":16976,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":524,"record_offset":17920,"parent_record_offset":17892,"ancestor_record_offset":16976,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":524,"record_offset":17948,"parent_record_offset":17920,"ancestor_record_offset":16976,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":524,"record_offset":17864,"parent_record_offset":17836,"ancestor_record_offset":16976,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":524,"record_offset":17836,"parent_record_offset":17808,"ancestor_record_offset":16976,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":524,"record_offset":17808,"parent_record_offset":17764,"ancestor_record_offset":16976,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":17984,"parent_record_offset":17948,"ancestor_record_offset":16976,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":524,"record_offset":18132,"parent_record_offset":18108,"ancestor_record_offset":16976,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":524,"record_offset":17068,"parent_record_offset":17044,"ancestor_record_offset":16976,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":524,"record_offset":18180,"parent_record_offset":18156,"ancestor_record_offset":16976,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":524,"record_offset":18156,"parent_record_offset":18132,"ancestor_record_offset":16976,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":524,"record_offset":18348,"parent_record_offset":17948,"ancestor_record_offset":16976,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":18372,"parent_record_offset":18348,"ancestor_record_offset":16976,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":17220,"parent_record_offset":17196,"ancestor_record_offset":16976,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mod_textures::is_moddable_texture_name::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 160
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x7900
// 首条位置: {"module":524,"inlinee":30976}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::impl$0::starts_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":524,"record_offset":30844,"parent_record_offset":30816,"ancestor_record_offset":30816,"inlinee":8324,"source_line":1405,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":524,"record_offset":30956,"parent_record_offset":30928,"ancestor_record_offset":30816,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":524,"record_offset":30980,"parent_record_offset":30956,"ancestor_record_offset":30816,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":524,"record_offset":31004,"parent_record_offset":30980,"ancestor_record_offset":30816,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::starts_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":524,"record_offset":30928,"parent_record_offset":30900,"ancestor_record_offset":30816,"inlinee":8287,"source_line":2619,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$30::is_prefix_of
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":524,"record_offset":30872,"parent_record_offset":30844,"ancestor_record_offset":30816,"inlinee":30754,"source_line":746,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_prefix_of
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":524,"record_offset":30900,"parent_record_offset":30872,"ancestor_record_offset":30816,"inlinee":8286,"source_line":981,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mod_textures::packaged_textures
// 记录形式: compiled_procedure; instances=1
// 源码行: 179
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":524,"record_offset":2848,"rva":10367552,"va":5379076672}
// 编译实例 RVAs: 0x9e3240
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5567598","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379076672,"raw_hex":"e86ef45400","size":5,"soft_fail":false,"target_va":5384644912,"va":5379077309,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t5532099","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379076672,"raw_hex":"e8c3695400","size":5,"soft_fail":false,"target_va":5384609408,"va":5379077304,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-3398257","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379076672,"raw_hex":"e88f25ccff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379076956,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-9737905","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379076672,"raw_hex":"e84f696bff","size":5,"soft_fail":false,"target_va":5369338848,"va":5379076748,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140698b50
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: load_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-3450770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379076672,"raw_hex":"e86e58cbff","size":5,"soft_fail":false,"target_va":5375626064,"va":5379076829,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096d120
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: load_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-484501","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379076672,"raw_hex":"e86b9bf8ff","size":5,"soft_fail":false,"target_va":5378593056,"va":5379077552,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990f00
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: load_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-337525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379076672,"raw_hex":"e88bd9faff","size":5,"soft_fail":false,"target_va":5378739968,"va":5379077488,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1419c2a90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t16643729","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379076672,"raw_hex":"e891f6fd00","size":5,"soft_fail":false,"target_va":5395720848,"va":5379077114,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b0dd90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18000660","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379076672,"raw_hex":"e814ab1201","size":5,"soft_fail":false,"target_va":5397077392,"va":5379076727,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b0df60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t18000798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379076672,"raw_hex":"e89eab1201","size":5,"soft_fail":false,"target_va":5397077856,"va":5379077053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_mod_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37980148","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379076672,"raw_hex":"e8f4874302","size":5,"soft_fail":false,"target_va":5417057040,"va":5379076887,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t5649263","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379076672,"raw_hex":"e86f335600","size":5,"soft_fail":false,"target_va":5384726688,"va":5379077420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":524,"record_offset":5656,"parent_record_offset":2848,"ancestor_record_offset":2848,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":524,"record_offset":4824,"parent_record_offset":2848,"ancestor_record_offset":2848,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] core::option::Option::and_then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":524,"record_offset":3056,"parent_record_offset":2848,"ancestor_record_offset":2848,"inlinee":5449,"source_line":1541,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":4852,"parent_record_offset":2848,"ancestor_record_offset":2848,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":524,"record_offset":3024,"parent_record_offset":2848,"ancestor_record_offset":2848,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":524,"record_offset":4740,"parent_record_offset":2848,"ancestor_record_offset":2848,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":524,"record_offset":5712,"parent_record_offset":2848,"ancestor_record_offset":2848,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":524,"record_offset":5572,"parent_record_offset":2848,"ancestor_record_offset":2848,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::map::HashMap::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":524,"record_offset":5824,"parent_record_offset":2848,"ancestor_record_offset":2848,"inlinee":6553,"source_line":271,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":3964,"parent_record_offset":3940,"ancestor_record_offset":2848,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":3988,"parent_record_offset":3964,"ancestor_record_offset":2848,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":4012,"parent_record_offset":3988,"ancestor_record_offset":2848,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":3940,"parent_record_offset":3876,"ancestor_record_offset":2848,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":3908,"parent_record_offset":3876,"ancestor_record_offset":2848,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":3876,"parent_record_offset":3848,"ancestor_record_offset":2848,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":3848,"parent_record_offset":3820,"ancestor_record_offset":2848,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$93::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":524,"record_offset":3416,"parent_record_offset":3244,"ancestor_record_offset":2848,"inlinee":21307,"source_line":2666,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":3660,"parent_record_offset":3636,"ancestor_record_offset":2848,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":524,"record_offset":6404,"parent_record_offset":6376,"ancestor_record_offset":2848,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":524,"record_offset":6304,"parent_record_offset":6100,"ancestor_record_offset":2848,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":524,"record_offset":3272,"parent_record_offset":3244,"ancestor_record_offset":2848,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":524,"record_offset":6268,"parent_record_offset":6244,"ancestor_record_offset":2848,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":524,"record_offset":4476,"parent_record_offset":4448,"ancestor_record_offset":2848,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":524,"record_offset":6432,"parent_record_offset":6376,"ancestor_record_offset":2848,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":524,"record_offset":6012,"parent_record_offset":5980,"ancestor_record_offset":2848,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":524,"record_offset":5680,"parent_record_offset":5656,"ancestor_record_offset":2848,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":524,"record_offset":3580,"parent_record_offset":3120,"ancestor_record_offset":2848,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":524,"record_offset":6132,"parent_record_offset":6100,"ancestor_record_offset":2848,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":3612,"parent_record_offset":3084,"ancestor_record_offset":2848,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":524,"record_offset":4788,"parent_record_offset":4764,"ancestor_record_offset":2848,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":524,"record_offset":6220,"parent_record_offset":6100,"ancestor_record_offset":2848,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":3208,"parent_record_offset":3180,"ancestor_record_offset":2848,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":6244,"parent_record_offset":6220,"ancestor_record_offset":2848,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":524,"record_offset":4700,"parent_record_offset":4676,"ancestor_record_offset":2848,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":524,"record_offset":3344,"parent_record_offset":3320,"ancestor_record_offset":2848,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":524,"record_offset":3372,"parent_record_offset":3344,"ancestor_record_offset":2848,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":524,"record_offset":4764,"parent_record_offset":4740,"ancestor_record_offset":2848,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":3148,"parent_record_offset":3120,"ancestor_record_offset":2848,"inlinee":6791,"source_line":343,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":3180,"parent_record_offset":3148,"ancestor_record_offset":2848,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":524,"record_offset":3296,"parent_record_offset":3272,"ancestor_record_offset":2848,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":524,"record_offset":5596,"parent_record_offset":5572,"ancestor_record_offset":2848,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":524,"record_offset":5620,"parent_record_offset":5596,"ancestor_record_offset":2848,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_mod_textures::packaged_textures::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_mod_textures.rs
//     evidence={"module":524,"record_offset":3084,"parent_record_offset":3056,"ancestor_record_offset":2848,"inlinee":30960,"source_line":183,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_mod_textures::packaged_textures::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_mod_textures.rs
//     evidence={"module":524,"record_offset":4676,"parent_record_offset":4648,"ancestor_record_offset":2848,"inlinee":30965,"source_line":186,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":524,"record_offset":5760,"parent_record_offset":5736,"ancestor_record_offset":2848,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":524,"record_offset":5736,"parent_record_offset":5712,"ancestor_record_offset":2848,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":524,"record_offset":5784,"parent_record_offset":5760,"ancestor_record_offset":2848,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::map::impl$9::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\map.rs
//     evidence={"module":524,"record_offset":5852,"parent_record_offset":5824,"ancestor_record_offset":2848,"inlinee":5990,"source_line":1454,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":524,"record_offset":5924,"parent_record_offset":5880,"ancestor_record_offset":2848,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":524,"record_offset":6180,"parent_record_offset":6156,"ancestor_record_offset":2848,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":524,"record_offset":6040,"parent_record_offset":6012,"ancestor_record_offset":2848,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":524,"record_offset":6376,"parent_record_offset":5980,"ancestor_record_offset":2848,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":524,"record_offset":5880,"parent_record_offset":5852,"ancestor_record_offset":2848,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":524,"record_offset":6068,"parent_record_offset":6040,"ancestor_record_offset":2848,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":524,"record_offset":6100,"parent_record_offset":6068,"ancestor_record_offset":2848,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":524,"record_offset":5980,"parent_record_offset":5952,"ancestor_record_offset":2848,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":524,"record_offset":5952,"parent_record_offset":5924,"ancestor_record_offset":2848,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::store::PackageManifest::asset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/store.rs
//     evidence={"module":524,"record_offset":3120,"parent_record_offset":3084,"ancestor_record_offset":2848,"inlinee":30962,"source_line":165,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::store::impl$6::asset::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/store.rs
//     evidence={"module":524,"record_offset":3244,"parent_record_offset":3148,"ancestor_record_offset":2848,"inlinee":30964,"source_line":168,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mod_textures::packaged_textures::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 183
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78F0
// 首条位置: {"module":524,"inlinee":30960}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":3612,"parent_record_offset":3084,"ancestor_record_offset":3084,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::store::PackageManifest::asset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/store.rs
//     evidence={"module":524,"record_offset":3120,"parent_record_offset":3084,"ancestor_record_offset":3084,"inlinee":30962,"source_line":165,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":3964,"parent_record_offset":3940,"ancestor_record_offset":3084,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":3988,"parent_record_offset":3964,"ancestor_record_offset":3084,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":4012,"parent_record_offset":3988,"ancestor_record_offset":3084,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":524,"record_offset":3940,"parent_record_offset":3876,"ancestor_record_offset":3084,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":3908,"parent_record_offset":3876,"ancestor_record_offset":3084,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":3876,"parent_record_offset":3848,"ancestor_record_offset":3084,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":524,"record_offset":3848,"parent_record_offset":3820,"ancestor_record_offset":3084,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$93::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":524,"record_offset":3416,"parent_record_offset":3244,"ancestor_record_offset":3084,"inlinee":21307,"source_line":2666,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":524,"record_offset":3660,"parent_record_offset":3636,"ancestor_record_offset":3084,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":524,"record_offset":3272,"parent_record_offset":3244,"ancestor_record_offset":3084,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":524,"record_offset":4476,"parent_record_offset":4448,"ancestor_record_offset":3084,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":524,"record_offset":3580,"parent_record_offset":3120,"ancestor_record_offset":3084,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":524,"record_offset":3636,"parent_record_offset":3612,"ancestor_record_offset":3084,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":524,"record_offset":3208,"parent_record_offset":3180,"ancestor_record_offset":3084,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":524,"record_offset":3344,"parent_record_offset":3320,"ancestor_record_offset":3084,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":524,"record_offset":3372,"parent_record_offset":3344,"ancestor_record_offset":3084,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":3148,"parent_record_offset":3120,"ancestor_record_offset":3084,"inlinee":6791,"source_line":343,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":524,"record_offset":3180,"parent_record_offset":3148,"ancestor_record_offset":3084,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":524,"record_offset":3296,"parent_record_offset":3272,"ancestor_record_offset":3084,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::store::impl$6::asset::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/store.rs
//     evidence={"module":524,"record_offset":3244,"parent_record_offset":3148,"ancestor_record_offset":3084,"inlinee":30964,"source_line":168,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mod_textures::packaged_textures::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 186
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x78F5
// 首条位置: {"module":524,"inlinee":30965}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::Result::ok
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":524,"record_offset":4700,"parent_record_offset":4676,"ancestor_record_offset":4676,"inlinee":5287,"source_line":708,"archive":"_pdb/C/modules/00524.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_mod_textures::packaged_textures::closure$2
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 194
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x140EC
// 首条位置: {"module":1658,"inlinee":82156}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
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
// 完整结构及原始类型记录: load_mod_textures.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::load_mod_textures::PackagedTexture
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 3
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::HashMap<alloc::string::String,country_core::startup::load_mod_textures::PackagedTexture,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "std::collections::hash::map::HashMap", "arguments": ["alloc::string::String", "country_core::startup::load_mod_textures::PackagedTexture", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<std::collections::hash::map::HashMap<alloc::string::String,country_core::startup::load_mod_textures::PackagedTexture,std::hash::random::RandomState,alloc::alloc::Global> >", "constructor": "core::ptr::drop_in_place", "arguments": ["std::collections::hash::map::HashMap<alloc::string::String,country_core::startup::load_mod_textures::PackagedTexture,std::hash::random::RandomState,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "hashbrown::map::HashMap::insert<alloc::string::String,country_core::startup::load_mod_textures::PackagedTexture,std::hash::random::RandomState,alloc::alloc::Global>", "constructor": "hashbrown::map::HashMap::insert", "arguments": ["alloc::string::String", "country_core::startup::load_mod_textures::PackagedTexture", "std::hash::random::RandomState", "alloc::alloc::Global"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::collections::hash::map::impl$78::from_iter<alloc::string::String,country_core::startup::load_mod_textures::PackagedTexture,std::hash::random::RandomState,core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_mod_textures::packaged_textures::closure_env$2> >", "constructor": "std::collections::hash::map::impl$78::from_iter", "arguments": ["alloc::string::String", "country_core::startup::load_mod_textures::PackagedTexture", "std::hash::random::RandomState", "core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_mod_textures::packaged_textures::closure_env$2>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: load_mod_textures.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: load_mod_textures.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 20; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] country_core::resources::clutter_registry::ClutterId (*)(ref$<country_core::resources::clutter_packs::ClutterPackObject>)
// 候选参数类型: ["ref$<country_core::resources::clutter_packs::ClutterPackObject>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<mesh_to_sdf::generate::grid::State>,ref$<mesh_to_sdf::generate::grid::State>)
// 候选参数类型: ["ref$<mesh_to_sdf::generate::grid::State>", "ref$<mesh_to_sdf::generate::grid::State>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >,ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >)
// 候选参数类型: ["ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >", "ref$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId> >"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::walls::WallId>,ref$<country_core::resources::walls::WallId>)
// 候选参数类型: ["ref$<country_core::resources::walls::WallId>", "ref$<country_core::resources::walls::WallId>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<country_core::resources::walls::decorator::DecoratorId>,ref$<country_core::resources::walls::decorator::DecoratorId>)
// 候选参数类型: ["ref$<country_core::resources::walls::decorator::DecoratorId>", "ref$<country_core::resources::walls::decorator::DecoratorId>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<u32>,ref$<u32>)
// 候选参数类型: ["ref$<u32>", "ref$<u32>"]
// [文件级接口候选，具体函数归属 unknown] bool (*)(ref$<u64>,ref$<u64>)
// 候选参数类型: ["ref$<u64>", "ref$<u64>"]
// [文件级接口候选，具体函数归属 unknown] usize (*)(ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >)
// 候选参数类型: ["ref$<hashbrown::map::Keys<bevy_ecs::archetype::ArchetypeId,bevy_ecs::archetype::ArchetypeRecord> >"]
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mod_textures::is_moddable_texture_name
// 信息 ID: 64626014f03143e6fbe9f985
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_mod_textures::reload_mod_textures @ 0x9e3a78 -> 0x9e5b10 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::render::texture_export::export_packaged_textures @ 0xa93339 -> 0x9e5b10 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mod_textures::is_moddable_texture_name::closure$0
// 信息 ID: f257293ca35757b4f4b11255
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::load_mod_textures::is_moddable_texture_name", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_mod_textures::is_moddable_texture_name; module=524, inline_record=30816; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mod_textures::packaged_textures
// 信息 ID: 0007b3eae07cfcf7afc582fe
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::bake_mod_textures::bake_and_apply_mod_texture_sources @ 0xc7e6a5 -> 0x9e3240 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mod_textures::packaged_textures::closure$0
// 信息 ID: a09c954f8ddc9d21eb67b5e4
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::load_mod_textures::packaged_textures", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_mod_textures::packaged_textures; module=524, inline_record=3084; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mod_textures::packaged_textures::closure$1
// 信息 ID: feead0de4b8f148e7ecd2cff
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::load_mod_textures::packaged_textures", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_mod_textures::packaged_textures; module=524, inline_record=4676; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mod_textures::packaged_textures::closure$2
// 信息 ID: ac03d2ddda42af9ad88d6784
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::startup::load_mod_textures::packaged_textures", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::load_mod_textures::packaged_textures::closure_env$2 <- std::collections::hash::map::impl$78::from_iter<alloc::string::String,country_core::startup::load_mod_textures::PackagedTexture,std::hash::random::RandomState,core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_mod_textures::packaged_textures::closure_env$2> >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 1 条。
// [PDB 内联宿主] std::collections::hash::map::impl$78::from_iter<alloc::string::String,country_core::startup::load_mod_textures::PackagedTexture,std::hash::random::RandomState,core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_mod_textures::packaged_textures::closure_env$2> >; module=1715, inline_record=30120; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mod_textures::reload_mod_textures
// 信息 ID: 0dccda05157f4583fa1ee1c7
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 3 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::swap_replacement_mods::ModSwapReloads::apply @ 0x907954 -> 0x9e35d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::swap_replacement_mods::apply_preactivated_mod_textures @ 0x9087c6 -> 0x9e35d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::mod_hot_reload::mod_hot_reload_apply @ 0xb2d16f -> 0x9e35d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mod_textures::reload_replaced_textures
// 信息 ID: 46b63b7e56e357174ee30f6e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_mod_textures::reload_mod_textures; module=524, inline_record=6704; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_mod_textures::reload_replaced_textures::closure$0
// 信息 ID: 719b33f4c929ad60ec379081
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::load_mod_textures::reload_replaced_textures", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_mod_textures::reload_mod_textures; module=524, inline_record=16976; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
