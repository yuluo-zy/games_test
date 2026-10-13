// Generated from PDB metadata by skeleton/_tools/restore_pdb.py.
// Version: C
// Original source: crates/country-core/src/startup/load_textures.rs
// PDB SHA-256: df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902
// Source SHA-256: f866546dd7aba299865478d000cd830c17cadd6eff85f90f708deeaf46d4ca87
// 此文件保存 PDB 元数据，不包含作者原始源码或恢复的函数实现。
// Rust void() 通常为调试占位，不能视为真实无参数、无返回值签名。
// 类型、参数、局部变量未记录时保留 unknown；不会生成假的可调用函数。
// 完整逐条元数据: load_textures.rs.pdb.jsonl.gz
// 完整类型/符号/编译单元及原始 PDB: skeleton/_pdb/C/
// 编译单元: 545, 1636, 1643, 1645, 1682, 1749, 1761, 1803, 1804, 3300
// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 文件引入注解：以下是已观察命名空间的 use 候选；原作者的导入写法、别名及可见性未知。
// 传递内联和物理函数调用可能来自库包装器，不证明源文件显式引入该 crate。
// use alloc; // namespace candidate
//   package identities: []
// use anyhow; // namespace candidate
//   package identities: [{"name":"anyhow","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.100"}]
// use bevy_ecs; // namespace candidate
//   package identities: [{"name":"bevy_ecs","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use bevy_tasks; // namespace candidate
//   package identities: [{"name":"bevy_tasks","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.3"}]
// use core; // namespace candidate
//   package identities: []
// use country_core; // namespace candidate
//   package identities: [{"name":"country-core","source":"local_or_path","version":"0.2.0"}]
// use hashbrown; // namespace candidate
//   package identities: [{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.12.3"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.14.5"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.15.2"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.16.1"},{"name":"hashbrown","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.17.1"}]
// use log; // namespace candidate
//   package identities: [{"name":"log","source":"registry+https://github.com/rust-lang/crates.io-index","version":"0.4.27"}]
// use relative_path; // namespace candidate
//   package identities: [{"name":"relative-path","source":"registry+https://github.com/rust-lang/crates.io-index","version":"2.0.1"}]
// use serde; // namespace candidate
//   package identities: [{"name":"serde","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"},{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"}]
// use serde_core; // namespace candidate
//   package identities: [{"name":"serde_core","source":"registry+https://github.com/rust-lang/crates.io-index","version":"1.0.228"}]
// use serde_json; // namespace candidate
//   package identities: [{"name":"serde_core","source":"PDB_registry_path_only","version":"1.0.228"},{"name":"serde_json","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c","version":"1.0.151"}]
// use std; // namespace candidate
//   package identities: []
// use tg_mods; // namespace candidate
//   package identities: [{"name":"tg-mods","source":"local_or_path","version":"0.1.0"}]
// use turbosloth; // namespace candidate
//   package identities: [{"name":"turbosloth","source":"local_or_path","version":"0.2.0"}]
// END GENERATED DEPENDENCY ANNOTATIONS


// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_textures::impl$0::should_load
// 记录形式: compiled_procedure; instances=1
// 源码行: 21
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":545,"record_offset":99768,"rva":11122144,"va":5379831264}
// 编译实例 RVAs: 0xa9b5e0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1409e6100
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-742655","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379831264,"raw_hex":"e801abf4ff","size":5,"soft_fail":false,"target_va":5379088640,"va":5379831290,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409e6e70
//     owner=unresolved_namespace; count=8
//     target_aliases=2 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-739282","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379831264,"raw_hex":"e82eb8f4ff","size":5,"soft_fail":false,"target_va":5379092080,"va":5379831357,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] alias_set_at_0x1409e6e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tjmp\t-739512","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379831264,"raw_hex":"e948b7f4ff","size":5,"soft_fail":false,"target_va":5379092080,"va":5379831587,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":545,"record_offset":99912,"parent_record_offset":99768,"ancestor_record_offset":99768,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::eq
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":545,"record_offset":100008,"parent_record_offset":99984,"ancestor_record_offset":99768,"inlinee":33894,"source_line":3864,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] relative_path::impl$27::eq
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"relative-path","version":"2.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-2.0.1/src/lib.rs
//     evidence={"module":545,"record_offset":99960,"parent_record_offset":99936,"ancestor_record_offset":99768,"inlinee":33891,"source_line":1757,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] relative_path::impl$5::eq
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"relative-path","version":"2.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-2.0.1/src/lib.rs
//     evidence={"module":545,"record_offset":99984,"parent_record_offset":99960,"ancestor_record_offset":99768,"inlinee":33893,"source_line":554,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] relative_path::impl$99::eq
//     owner=registry_dependency_definition; count=9
//     packages=[{"name":"relative-path","version":"2.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-2.0.1/src/lib.rs
//     evidence={"module":545,"record_offset":99936,"parent_record_offset":99912,"ancestor_record_offset":99768,"inlinee":33889,"source_line":2021,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_textures::impl$1::should_load
// 记录形式: compiled_procedure; instances=1
// 源码行: 43
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":545,"record_offset":97744,"rva":11121184,"va":5379830304}
// 编译实例 RVAs: 0xa9b220
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] alias_set_at_0x1409e6100
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-741770","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379830304,"raw_hex":"e876aef4ff","size":5,"soft_fail":false,"target_va":5379088640,"va":5379830405,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x1409e6e70
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-738358","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379830304,"raw_hex":"e8cabbf4ff","size":5,"soft_fail":false,"target_va":5379092080,"va":5379830433,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::str::pattern::simd_contains::closure$2
//     owner=rust_standard_library; count=6
//     evidence={"assembly":"\tcallq\t36533785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379830304,"raw_hex":"e819762d02","size":5,"soft_fail":false,"target_va":5416364576,"va":5379830786,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":545,"record_offset":97892,"parent_record_offset":97744,"ancestor_record_offset":97744,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] core::str::impl$0::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\mod.rs
//     evidence={"module":545,"record_offset":98032,"parent_record_offset":97744,"ancestor_record_offset":97744,"inlinee":12765,"source_line":1367,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":545,"record_offset":98320,"parent_record_offset":98296,"ancestor_record_offset":97744,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::core_simd::masks::Mask::to_bitmask
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\portable-simd\crates\core_simd\src\masks.rs
//     evidence={"module":545,"record_offset":98824,"parent_record_offset":98536,"ancestor_record_offset":97744,"inlinee":18788,"source_line":299,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::core_simd::masks::impl$2::to_bitmask::to_bitmask_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\portable-simd\crates\core_simd\src\masks.rs
//     evidence={"module":545,"record_offset":98848,"parent_record_offset":98824,"ancestor_record_offset":97744,"inlinee":18790,"source_line":305,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::core_simd::masks::impl$9::bitand
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\portable-simd\crates\core_simd\src\masks.rs
//     evidence={"module":545,"record_offset":98796,"parent_record_offset":98536,"ancestor_record_offset":97744,"inlinee":18786,"source_line":479,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::core_simd::simd::cmp::eq::impl$4::simd_eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\portable-simd\crates\core_simd\src\simd\cmp\eq.rs
//     evidence={"module":545,"record_offset":98656,"parent_record_offset":98536,"ancestor_record_offset":97744,"inlinee":18784,"source_line":28,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":545,"record_offset":98220,"parent_record_offset":98112,"ancestor_record_offset":97744,"inlinee":12047,"source_line":2863,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::any::check::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":545,"record_offset":98272,"parent_record_offset":98244,"ancestor_record_offset":97744,"inlinee":12049,"source_line":2870,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":545,"record_offset":97988,"parent_record_offset":97964,"ancestor_record_offset":97744,"inlinee":33894,"source_line":3864,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::try_fold
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":545,"record_offset":98244,"parent_record_offset":98220,"ancestor_record_offset":97744,"inlinee":5047,"source_line":2461,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::read_unaligned
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":545,"record_offset":98572,"parent_record_offset":98536,"ancestor_record_offset":97744,"inlinee":18782,"source_line":1207,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":545,"record_offset":98620,"parent_record_offset":98596,"ancestor_record_offset":97744,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read_unaligned
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":545,"record_offset":98596,"parent_record_offset":98572,"ancestor_record_offset":97744,"inlinee":11360,"source_line":1796,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":545,"record_offset":98344,"parent_record_offset":98320,"ancestor_record_offset":97744,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":545,"record_offset":98368,"parent_record_offset":98344,"ancestor_record_offset":97744,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::get_offset_len_noubcheck
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":545,"record_offset":98488,"parent_record_offset":98464,"ancestor_record_offset":97744,"inlinee":6161,"source_line":82,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$0::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":545,"record_offset":98440,"parent_record_offset":98412,"ancestor_record_offset":97744,"inlinee":5467,"source_line":18,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::index::impl$7::index
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\index.rs
//     evidence={"module":545,"record_offset":98464,"parent_record_offset":98440,"ancestor_record_offset":97744,"inlinee":6159,"source_line":567,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$62::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":545,"record_offset":98412,"parent_record_offset":98244,"ancestor_record_offset":97744,"inlinee":18778,"source_line":1356,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::impl$31::is_contained_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":545,"record_offset":98068,"parent_record_offset":98032,"ancestor_record_offset":97744,"inlinee":18769,"source_line":987,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::simd_contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":545,"record_offset":98112,"parent_record_offset":98068,"ancestor_record_offset":97744,"inlinee":18771,"source_line":1790,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::simd_contains::closure$1
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":545,"record_offset":98296,"parent_record_offset":98272,"ancestor_record_offset":97744,"inlinee":18776,"source_line":1822,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::str::pattern::simd_contains::closure$3
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\pattern.rs
//     evidence={"module":545,"record_offset":98536,"parent_record_offset":98112,"ancestor_record_offset":97744,"inlinee":18781,"source_line":1857,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":545,"record_offset":99636,"parent_record_offset":99612,"ancestor_record_offset":97744,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] relative_path::impl$27::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"2.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-2.0.1/src/lib.rs
//     evidence={"module":545,"record_offset":97940,"parent_record_offset":97916,"ancestor_record_offset":97744,"inlinee":33891,"source_line":1757,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] relative_path::impl$5::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"2.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-2.0.1/src/lib.rs
//     evidence={"module":545,"record_offset":97964,"parent_record_offset":97940,"ancestor_record_offset":97744,"inlinee":33893,"source_line":554,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] relative_path::impl$99::eq
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"relative-path","version":"2.0.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/relative-path-2.0.1/src/lib.rs
//     evidence={"module":545,"record_offset":97916,"parent_record_offset":97892,"ancestor_record_offset":97744,"inlinee":33889,"source_line":2021,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_textures::load_textures
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 61
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14E77
// 首条位置: {"module":1761,"inlinee":85623}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1761,"record_offset":139772,"parent_record_offset":139620,"ancestor_record_offset":139620,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_direct] core::iter::adapters::zip::impl$1::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1761,"record_offset":140052,"parent_record_offset":139620,"ancestor_record_offset":139620,"inlinee":6656,"source_line":84,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1761,"record_offset":139912,"parent_record_offset":139620,"ancestor_record_offset":139620,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1761,"record_offset":141236,"parent_record_offset":139620,"ancestor_record_offset":139620,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1761,"record_offset":139828,"parent_record_offset":139620,"ancestor_record_offset":139620,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::Asset::name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1761,"record_offset":140540,"parent_record_offset":139620,"ancestor_record_offset":139620,"inlinee":21763,"source_line":34,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::Asset::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1761,"record_offset":140372,"parent_record_offset":139620,"ancestor_record_offset":139620,"inlinee":21762,"source_line":30,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_direct] country_core::render::impl$0::deref
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/render/mod.rs
//     evidence={"module":1761,"record_offset":139716,"parent_record_offset":139620,"ancestor_record_offset":139620,"inlinee":27991,"source_line":42,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::grading_lut::GradingLut::asset_name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":1761,"record_offset":140400,"parent_record_offset":139620,"ancestor_record_offset":139620,"inlinee":30799,"source_line":76,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_textures::load_packaged_textures
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_textures.rs
//     evidence={"module":1761,"record_offset":142496,"parent_record_offset":139620,"ancestor_record_offset":139620,"inlinee":85624,"source_line":91,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1761,"record_offset":141020,"parent_record_offset":140996,"ancestor_record_offset":139620,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1761,"record_offset":141044,"parent_record_offset":141020,"ancestor_record_offset":139620,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1761,"record_offset":141412,"parent_record_offset":141388,"ancestor_record_offset":139620,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1761,"record_offset":141436,"parent_record_offset":141412,"ancestor_record_offset":139620,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1761,"record_offset":141068,"parent_record_offset":141044,"ancestor_record_offset":139620,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1761,"record_offset":141460,"parent_record_offset":141436,"ancestor_record_offset":139620,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1761,"record_offset":140996,"parent_record_offset":140848,"ancestor_record_offset":139620,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1761,"record_offset":141388,"parent_record_offset":141332,"ancestor_record_offset":139620,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1761,"record_offset":140448,"parent_record_offset":140424,"ancestor_record_offset":139620,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format::closure$0
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":1761,"record_offset":140496,"parent_record_offset":140472,"ancestor_record_offset":139620,"inlinee":6853,"source_line":659,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1761,"record_offset":140788,"parent_record_offset":140760,"ancestor_record_offset":139620,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1761,"record_offset":141360,"parent_record_offset":141332,"ancestor_record_offset":139620,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1761,"record_offset":141332,"parent_record_offset":141308,"ancestor_record_offset":139620,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1761,"record_offset":140848,"parent_record_offset":140816,"ancestor_record_offset":139620,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1761,"record_offset":140816,"parent_record_offset":140788,"ancestor_record_offset":139620,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1761,"record_offset":141308,"parent_record_offset":141284,"ancestor_record_offset":139620,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1761,"record_offset":140884,"parent_record_offset":140848,"ancestor_record_offset":139620,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1761,"record_offset":140668,"parent_record_offset":140640,"ancestor_record_offset":139620,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1761,"record_offset":140696,"parent_record_offset":140668,"ancestor_record_offset":139620,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1761,"record_offset":140724,"parent_record_offset":140696,"ancestor_record_offset":139620,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1761,"record_offset":140640,"parent_record_offset":140612,"ancestor_record_offset":139620,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1761,"record_offset":140612,"parent_record_offset":140584,"ancestor_record_offset":139620,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1761,"record_offset":140584,"parent_record_offset":140540,"ancestor_record_offset":139620,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1761,"record_offset":139740,"parent_record_offset":139716,"ancestor_record_offset":139620,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1761,"record_offset":140760,"parent_record_offset":140724,"ancestor_record_offset":139620,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1761,"record_offset":139936,"parent_record_offset":139912,"ancestor_record_offset":139620,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1761,"record_offset":143816,"parent_record_offset":143792,"ancestor_record_offset":139620,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::IntoIter::as_raw_mut_slice
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1761,"record_offset":141624,"parent_record_offset":141584,"ancestor_record_offset":139620,"inlinee":4695,"source_line":117,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1761,"record_offset":141584,"parent_record_offset":141552,"ancestor_record_offset":139620,"inlinee":4693,"source_line":497,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$16::drop::impl$0::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1761,"record_offset":141876,"parent_record_offset":141852,"ancestor_record_offset":139620,"inlinee":4706,"source_line":501,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1761,"record_offset":140216,"parent_record_offset":140080,"ancestor_record_offset":139620,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::size_hint
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1761,"record_offset":141696,"parent_record_offset":141672,"ancestor_record_offset":139620,"inlinee":4701,"source_line":234,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":1761,"record_offset":139796,"parent_record_offset":139772,"ancestor_record_offset":139620,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] bevy_tasks::slice::ParallelSlice::par_chunk_map
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_tasks","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_tasks-0.17.3/src/slice.rs
//     evidence={"module":1761,"record_offset":142572,"parent_record_offset":142496,"ancestor_record_offset":139620,"inlinee":28302,"source_line":37,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] bevy_tasks::task_pool::TaskPool::scope
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_tasks","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_tasks-0.17.3/src/task_pool.rs
//     evidence={"module":1761,"record_offset":142604,"parent_record_offset":142572,"ancestor_record_offset":139620,"inlinee":10713,"source_line":310,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] bevy_tasks::task_pool::impl$2::scope::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_tasks","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_tasks-0.17.3/src/task_pool.rs
//     evidence={"module":1761,"record_offset":142724,"parent_record_offset":142668,"ancestor_record_offset":139620,"inlinee":6586,"source_line":315,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1761,"record_offset":140908,"parent_record_offset":140884,"ancestor_record_offset":139620,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1761,"record_offset":140956,"parent_record_offset":140932,"ancestor_record_offset":139620,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1761,"record_offset":139992,"parent_record_offset":139912,"ancestor_record_offset":139620,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1761,"record_offset":140016,"parent_record_offset":139992,"ancestor_record_offset":139620,"inlinee":12119,"source_line":154,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1761,"record_offset":140080,"parent_record_offset":140052,"ancestor_record_offset":139620,"inlinee":9633,"source_line":164,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::ExactSizeIterator::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1761,"record_offset":141672,"parent_record_offset":141648,"ancestor_record_offset":139620,"inlinee":4699,"source_line":116,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::exact_size::impl$0::len
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\exact_size.rs
//     evidence={"module":1761,"record_offset":141648,"parent_record_offset":141624,"ancestor_record_offset":139620,"inlinee":4697,"source_line":155,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1761,"record_offset":142936,"parent_record_offset":142496,"ancestor_record_offset":139620,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1761,"record_offset":140932,"parent_record_offset":140908,"ancestor_record_offset":139620,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1761,"record_offset":142004,"parent_record_offset":141976,"ancestor_record_offset":139620,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1761,"record_offset":140472,"parent_record_offset":140448,"ancestor_record_offset":139620,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1761,"record_offset":142696,"parent_record_offset":142668,"ancestor_record_offset":139620,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1761,"record_offset":141124,"parent_record_offset":140724,"ancestor_record_offset":139620,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1761,"record_offset":141768,"parent_record_offset":141744,"ancestor_record_offset":139620,"inlinee":4704,"source_line":701,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1761,"record_offset":141148,"parent_record_offset":141124,"ancestor_record_offset":139620,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=33
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1761,"record_offset":141204,"parent_record_offset":140540,"ancestor_record_offset":139620,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1761,"record_offset":139876,"parent_record_offset":139852,"ancestor_record_offset":139620,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1761,"record_offset":141744,"parent_record_offset":141720,"ancestor_record_offset":139620,"inlinee":4703,"source_line":882,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1761,"record_offset":140184,"parent_record_offset":140116,"ancestor_record_offset":139620,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::offset_from_unsigned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1761,"record_offset":141720,"parent_record_offset":141696,"ancestor_record_offset":139620,"inlinee":4702,"source_line":948,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1761,"record_offset":140304,"parent_record_offset":140216,"ancestor_record_offset":139620,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1761,"record_offset":140152,"parent_record_offset":140116,"ancestor_record_offset":139620,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1761,"record_offset":140328,"parent_record_offset":140304,"ancestor_record_offset":139620,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1761,"record_offset":142852,"parent_record_offset":142496,"ancestor_record_offset":139620,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1761,"record_offset":139852,"parent_record_offset":139828,"ancestor_record_offset":139620,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":1761,"record_offset":140116,"parent_record_offset":140080,"ancestor_record_offset":139620,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::Asset::name
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1761,"record_offset":143048,"parent_record_offset":142496,"ancestor_record_offset":139620,"inlinee":21763,"source_line":34,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::Asset::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1761,"record_offset":143020,"parent_record_offset":142496,"ancestor_record_offset":139620,"inlinee":21762,"source_line":30,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::grading_lut::GradingLut::lut_name_to_asset_name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/grading_lut.rs
//     evidence={"module":1761,"record_offset":140424,"parent_record_offset":140400,"ancestor_record_offset":139620,"inlinee":30536,"source_line":80,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1761,"record_offset":142668,"parent_record_offset":142632,"ancestor_record_offset":139620,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1761,"record_offset":142632,"parent_record_offset":142604,"ancestor_record_offset":139620,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_textures::read_texture_list
// 记录形式: compiled_procedure; instances=1
// 源码行: 80
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":545,"record_offset":11236,"rva":11092688,"va":5379801808}
// 编译实例 RVAs: 0xa942d0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=3
//     evidence={"assembly":"\tcallq\t-4123425","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379801808,"raw_hex":"e8df14c1ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379802124,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10463525","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379801808,"raw_hex":"e8db5660ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5379802368,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140698b50
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4175910","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379801808,"raw_hex":"e8da47c0ff","size":5,"soft_fail":false,"target_va":5375626064,"va":5379801969,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990f00
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1062469","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379801808,"raw_hex":"e8bbc9efff","size":5,"soft_fail":false,"target_va":5378739968,"va":5379802432,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b0dd90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17275531","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379801808,"raw_hex":"e88b9a0701","size":5,"soft_fail":false,"target_va":5397077392,"va":5379801856,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b0df60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17275798","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379801808,"raw_hex":"e8969b0701","size":5,"soft_fail":false,"target_va":5397077856,"va":5379802053,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37255014","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379801808,"raw_hex":"e866773802","size":5,"soft_fail":false,"target_va":5417057040,"va":5379802021,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36819206","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379801808,"raw_hex":"e806d13102","size":5,"soft_fail":false,"target_va":5416621536,"va":5379802325,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36819380","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379801808,"raw_hex":"e8b4d13102","size":5,"soft_fail":false,"target_va":5416621632,"va":5379802247,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":545,"record_offset":12984,"parent_record_offset":11236,"ancestor_record_offset":11236,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":545,"record_offset":11956,"parent_record_offset":11236,"ancestor_record_offset":11236,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":545,"record_offset":11388,"parent_record_offset":11236,"ancestor_record_offset":11236,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::store::PackageManifest::asset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/store.rs
//     evidence={"module":545,"record_offset":11432,"parent_record_offset":11236,"ancestor_record_offset":11236,"inlinee":30962,"source_line":165,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":12308,"parent_record_offset":12284,"ancestor_record_offset":11236,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":12332,"parent_record_offset":12308,"ancestor_record_offset":11236,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":12356,"parent_record_offset":12332,"ancestor_record_offset":11236,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":12284,"parent_record_offset":12220,"ancestor_record_offset":11236,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":12252,"parent_record_offset":12220,"ancestor_record_offset":11236,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":12220,"parent_record_offset":12192,"ancestor_record_offset":11236,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":12192,"parent_record_offset":12164,"ancestor_record_offset":11236,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$93::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":545,"record_offset":11716,"parent_record_offset":11548,"ancestor_record_offset":11236,"inlinee":21307,"source_line":2666,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":545,"record_offset":12004,"parent_record_offset":11980,"ancestor_record_offset":11236,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":545,"record_offset":11572,"parent_record_offset":11548,"ancestor_record_offset":11236,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":545,"record_offset":12820,"parent_record_offset":12792,"ancestor_record_offset":11236,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":545,"record_offset":11880,"parent_record_offset":11432,"ancestor_record_offset":11236,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":545,"record_offset":11980,"parent_record_offset":11956,"ancestor_record_offset":11236,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":545,"record_offset":11512,"parent_record_offset":11488,"ancestor_record_offset":11236,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":545,"record_offset":11644,"parent_record_offset":11620,"ancestor_record_offset":11236,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":545,"record_offset":11672,"parent_record_offset":11644,"ancestor_record_offset":11236,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":545,"record_offset":11460,"parent_record_offset":11432,"ancestor_record_offset":11236,"inlinee":6791,"source_line":343,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":545,"record_offset":11488,"parent_record_offset":11460,"ancestor_record_offset":11236,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":545,"record_offset":11596,"parent_record_offset":11572,"ancestor_record_offset":11236,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::store::impl$6::asset::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/store.rs
//     evidence={"module":545,"record_offset":11548,"parent_record_offset":11460,"ancestor_record_offset":11236,"inlinee":30964,"source_line":168,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_textures::load_packaged_textures
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 91
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14E78
// 首条位置: {"module":1761,"inlinee":85624}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::vec::impl$17::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1761,"record_offset":142796,"parent_record_offset":142496,"ancestor_record_offset":142496,"inlinee":6128,"source_line":3891,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_direct] bevy_tasks::slice::ParallelSlice::par_chunk_map
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_tasks","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_tasks-0.17.3/src/slice.rs
//     evidence={"module":1761,"record_offset":142572,"parent_record_offset":142496,"ancestor_record_offset":142496,"inlinee":28302,"source_line":37,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::zip
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1761,"record_offset":142936,"parent_record_offset":142496,"ancestor_record_offset":142496,"inlinee":7226,"source_line":626,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1761,"record_offset":143740,"parent_record_offset":142496,"ancestor_record_offset":142496,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1761,"record_offset":142852,"parent_record_offset":142496,"ancestor_record_offset":142496,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::Asset::name
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1761,"record_offset":143048,"parent_record_offset":142496,"ancestor_record_offset":142496,"inlinee":21763,"source_line":34,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::Asset::new
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":1761,"record_offset":143020,"parent_record_offset":142496,"ancestor_record_offset":142496,"inlinee":21762,"source_line":30,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1761,"record_offset":143524,"parent_record_offset":143500,"ancestor_record_offset":142496,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1761,"record_offset":143548,"parent_record_offset":143524,"ancestor_record_offset":142496,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1761,"record_offset":144092,"parent_record_offset":144068,"ancestor_record_offset":142496,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1761,"record_offset":144116,"parent_record_offset":144092,"ancestor_record_offset":142496,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1761,"record_offset":143572,"parent_record_offset":143548,"ancestor_record_offset":142496,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1761,"record_offset":144140,"parent_record_offset":144116,"ancestor_record_offset":142496,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1761,"record_offset":143500,"parent_record_offset":143352,"ancestor_record_offset":142496,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1761,"record_offset":144068,"parent_record_offset":144004,"ancestor_record_offset":142496,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1761,"record_offset":143292,"parent_record_offset":143264,"ancestor_record_offset":142496,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1761,"record_offset":144036,"parent_record_offset":144004,"ancestor_record_offset":142496,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1761,"record_offset":144004,"parent_record_offset":143976,"ancestor_record_offset":142496,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1761,"record_offset":143352,"parent_record_offset":143320,"ancestor_record_offset":142496,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1761,"record_offset":143320,"parent_record_offset":143292,"ancestor_record_offset":142496,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1761,"record_offset":143976,"parent_record_offset":143948,"ancestor_record_offset":142496,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1761,"record_offset":143388,"parent_record_offset":143352,"ancestor_record_offset":142496,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1761,"record_offset":143172,"parent_record_offset":143144,"ancestor_record_offset":142496,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1761,"record_offset":143200,"parent_record_offset":143172,"ancestor_record_offset":142496,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1761,"record_offset":143228,"parent_record_offset":143200,"ancestor_record_offset":142496,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":1761,"record_offset":143144,"parent_record_offset":143116,"ancestor_record_offset":142496,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":1761,"record_offset":143116,"parent_record_offset":143088,"ancestor_record_offset":142496,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":1761,"record_offset":143088,"parent_record_offset":143048,"ancestor_record_offset":142496,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::sync::impl$34::deref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\sync.rs
//     evidence={"module":1761,"record_offset":142748,"parent_record_offset":142724,"ancestor_record_offset":142496,"inlinee":4944,"source_line":2421,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1761,"record_offset":143264,"parent_record_offset":143228,"ancestor_record_offset":142496,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1761,"record_offset":143816,"parent_record_offset":143792,"ancestor_record_offset":142496,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] bevy_tasks::task_pool::TaskPool::scope
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_tasks","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_tasks-0.17.3/src/task_pool.rs
//     evidence={"module":1761,"record_offset":142604,"parent_record_offset":142572,"ancestor_record_offset":142496,"inlinee":10713,"source_line":310,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] bevy_tasks::task_pool::impl$2::scope::closure$0
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"bevy_tasks","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_tasks-0.17.3/src/task_pool.rs
//     evidence={"module":1761,"record_offset":142724,"parent_record_offset":142668,"ancestor_record_offset":142496,"inlinee":6586,"source_line":315,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":1761,"record_offset":143412,"parent_record_offset":143388,"ancestor_record_offset":142496,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":1761,"record_offset":143460,"parent_record_offset":143436,"ancestor_record_offset":142496,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::Zip::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1761,"record_offset":142960,"parent_record_offset":142936,"ancestor_record_offset":142496,"inlinee":7228,"source_line":23,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::adapters::zip::impl$3::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\adapters\zip.rs
//     evidence={"module":1761,"record_offset":142984,"parent_record_offset":142960,"ancestor_record_offset":142496,"inlinee":12119,"source_line":154,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1761,"record_offset":143436,"parent_record_offset":143412,"ancestor_record_offset":142496,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":1761,"record_offset":144316,"parent_record_offset":144288,"ancestor_record_offset":142496,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::as_ref
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1761,"record_offset":142696,"parent_record_offset":142668,"ancestor_record_offset":142496,"inlinee":6443,"source_line":266,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":1761,"record_offset":143628,"parent_record_offset":143228,"ancestor_record_offset":142496,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1761,"record_offset":143652,"parent_record_offset":143628,"ancestor_record_offset":142496,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=16
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1761,"record_offset":143708,"parent_record_offset":143048,"ancestor_record_offset":142496,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1761,"record_offset":142820,"parent_record_offset":142796,"ancestor_record_offset":142496,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1761,"record_offset":142876,"parent_record_offset":142852,"ancestor_record_offset":142496,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1761,"record_offset":142668,"parent_record_offset":142632,"ancestor_record_offset":142496,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":1761,"record_offset":142632,"parent_record_offset":142604,"ancestor_record_offset":142496,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_textures::load_packaged_textures::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 101
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x14894
// 首条位置: {"module":1749,"inlinee":84116}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::iter::traits::iterator::Iterator::collect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\iterator.rs
//     evidence={"module":1749,"record_offset":113272,"parent_record_offset":113152,"ancestor_record_offset":113152,"inlinee":5069,"source_line":2051,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_direct] core::slice::impl$0::iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\mod.rs
//     evidence={"module":1749,"record_offset":113188,"parent_record_offset":113152,"ancestor_record_offset":113152,"inlinee":6164,"source_line":1040,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$16::from_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1749,"record_offset":113300,"parent_record_offset":113272,"ancestor_record_offset":113152,"inlinee":5071,"source_line":3864,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] core::iter::traits::collect::impl$0::into_iter
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\iter\traits\collect.rs
//     evidence={"module":1749,"record_offset":113328,"parent_record_offset":113300,"ancestor_record_offset":113152,"inlinee":7199,"source_line":322,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":1749,"record_offset":113236,"parent_record_offset":113212,"ancestor_record_offset":113152,"inlinee":4282,"source_line":927,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::Iter::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter.rs
//     evidence={"module":1749,"record_offset":113212,"parent_record_offset":113188,"ancestor_record_offset":113152,"inlinee":6166,"source_line":96,"archive":"_pdb/C/modules/01749.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_textures::load_packaged_textures::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 104
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13DAF
// 首条位置: {"module":1643,"inlinee":81327}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::bool::impl$0::then
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\bool.rs
//     evidence={"module":1643,"record_offset":506112,"parent_record_offset":506080,"ancestor_record_offset":506080,"inlinee":4276,"source_line":65,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_textures::load_packaged_textures::closure$0::closure$0::closure$0
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_textures.rs
//     evidence={"module":1643,"record_offset":506136,"parent_record_offset":506112,"ancestor_record_offset":506080,"inlinee":81329,"source_line":106,"archive":"_pdb/C/modules/01643.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_textures::load_packaged_textures::closure$0::closure$0::closure$0
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 106
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13DB1
// 首条位置: {"module":1643,"inlinee":81329}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_textures::reload_packaged_textures_by_name
// 记录形式: compiled_procedure; instances=1
// 源码行: 195
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":545,"record_offset":43240,"rva":11104304,"va":5379813424}
// 编译实例 RVAs: 0xa97030
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <alloc::string::String as core::clone::Clone>::clone
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t4981607","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379813424,"raw_hex":"e867034c00","size":5,"soft_fail":false,"target_va":5384795984,"va":5379814372,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] <std::hash::random::RandomState>::new::KEYS::{K#0}::{closure#1}::__RUST_STD_INTERNAL_VAL::{shim:tls#0}
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4831145","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379813424,"raw_hex":"e8a9b74900","size":5,"soft_fail":false,"target_va":5384644912,"va":5379813762,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tcallq\t-4135562","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379813424,"raw_hex":"e876e5c0ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5379814245,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=5
//     evidence={"assembly":"\tcallq\t-4135812","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379813424,"raw_hex":"e87ce4c0ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379814511,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=1
//     target_aliases=1699 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11092039","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379813424,"raw_hex":"e8b9bf56ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5379814226,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140099be0
//     owner=unresolved_namespace; count=2
//     target_aliases=3 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-10476635","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379813424,"raw_hex":"e8a52360ff","size":5,"soft_fail":false,"target_va":5369338848,"va":5379815478,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14040fae0
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6846746","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379813424,"raw_hex":"e8e68697ff","size":5,"soft_fail":false,"target_va":5372967648,"va":5379814389,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14048d130
//     owner=unresolved_namespace; count=1
//     target_aliases=3 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-6332663","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379813424,"raw_hex":"e8095f9fff","size":5,"soft_fail":false,"target_va":5373481264,"va":5379813922,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140698b50
//     owner=unresolved_namespace; count=1
//     target_aliases=10 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-4187574","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379813424,"raw_hex":"e84a1ac0ff","size":5,"soft_fail":false,"target_va":5375626064,"va":5379813633,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14096d120
//     owner=unresolved_namespace; count=1
//     target_aliases=6 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1222136","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379813424,"raw_hex":"e8085aedff","size":5,"soft_fail":false,"target_va":5378593056,"va":5379815187,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098f4c0
//     owner=unresolved_namespace; count=1
//     target_aliases=5 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1082040","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379813424,"raw_hex":"e8487defff","size":5,"soft_fail":false,"target_va":5378733248,"va":5379815283,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14098f9c0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1080859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379813424,"raw_hex":"e8e581efff","size":5,"soft_fail":false,"target_va":5378734528,"va":5379815382,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140990f00
//     owner=unresolved_namespace; count=1
//     target_aliases=4 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1075128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379813424,"raw_hex":"e84898efff","size":5,"soft_fail":false,"target_va":5378739968,"va":5379815091,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b0dd90
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17263859","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379813424,"raw_hex":"e8f36c0701","size":5,"soft_fail":false,"target_va":5397077392,"va":5379813528,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141b0df60
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t17264128","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379813424,"raw_hex":"e8006e0701","size":5,"soft_fail":false,"target_va":5397077856,"va":5379813723,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb10
//     owner=unresolved_namespace; count=2
//     target_aliases=1 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37243350","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379813424,"raw_hex":"e8d6493802","size":5,"soft_fail":false,"target_va":5417057040,"va":5379813685,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=1
//     target_aliases=1 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37242780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379813424,"raw_hex":"e89c473802","size":5,"soft_fail":false,"target_va":5417057056,"va":5379814271,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36806195","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379813424,"raw_hex":"e8339e3102","size":5,"soft_fail":false,"target_va":5416620794,"va":5379814594,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::option::expect_failed
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t36806542","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379813424,"raw_hex":"e88e9f3102","size":5,"soft_fail":false,"target_va":5416621536,"va":5379814989,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::result::unwrap_failed
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t36806744","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379813424,"raw_hex":"e858a03102","size":5,"soft_fail":false,"target_va":5416621632,"va":5379814883,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::add_or_replace
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-537625","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379813424,"raw_hex":"e8e7cbf7ff","size":5,"soft_fail":false,"target_va":5379276736,"va":5379814356,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::render::load_texture::load_packaged_texture_image
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t1022330","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379813424,"raw_hex":"e87a990f00","size":5,"soft_fail":false,"target_va":5380836512,"va":5379814177,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::sys::random::hashmap_random_keys
//     owner=rust_standard_library; count=2
//     evidence={"assembly":"\tcallq\t4911735","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379813424,"raw_hex":"e877f24a00","size":5,"soft_fail":false,"target_va":5384726688,"va":5379814948,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::option::Option::expect
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":545,"record_offset":49120,"parent_record_offset":43240,"ancestor_record_offset":43240,"inlinee":5558,"source_line":968,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":545,"record_offset":46692,"parent_record_offset":43240,"ancestor_record_offset":43240,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::expect
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":545,"record_offset":43524,"parent_record_offset":43240,"ancestor_record_offset":43240,"inlinee":6800,"source_line":1179,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":545,"record_offset":44808,"parent_record_offset":43240,"ancestor_record_offset":43240,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::Asset::name
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":545,"record_offset":45940,"parent_record_offset":43240,"ancestor_record_offset":43240,"inlinee":21763,"source_line":34,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::Asset::new
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/mod.rs
//     evidence={"module":545,"record_offset":45912,"parent_record_offset":43240,"ancestor_record_offset":43240,"inlinee":21762,"source_line":30,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::contains
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":545,"record_offset":44872,"parent_record_offset":43240,"ancestor_record_offset":43240,"inlinee":5587,"source_line":767,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::insert
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":545,"record_offset":46636,"parent_record_offset":43240,"ancestor_record_offset":43240,"inlinee":5646,"source_line":995,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::is_empty
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":545,"record_offset":43440,"parent_record_offset":43240,"ancestor_record_offset":43240,"inlinee":26554,"source_line":385,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::HashSet::new
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":545,"record_offset":44092,"parent_record_offset":43240,"ancestor_record_offset":43240,"inlinee":5639,"source_line":149,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] std::collections::hash::set::impl$13::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":545,"record_offset":47412,"parent_record_offset":43240,"ancestor_record_offset":43240,"inlinee":5641,"source_line":1246,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] turbosloth::store::PackageManifest::asset
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/store.rs
//     evidence={"module":545,"record_offset":43568,"parent_record_offset":43240,"ancestor_record_offset":43240,"inlinee":30962,"source_line":165,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":46420,"parent_record_offset":46396,"ancestor_record_offset":43240,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":46444,"parent_record_offset":46420,"ancestor_record_offset":43240,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":47024,"parent_record_offset":47000,"ancestor_record_offset":43240,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":47048,"parent_record_offset":47024,"ancestor_record_offset":43240,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":46468,"parent_record_offset":46444,"ancestor_record_offset":43240,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":47072,"parent_record_offset":47048,"ancestor_record_offset":43240,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":46396,"parent_record_offset":46248,"ancestor_record_offset":43240,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":47000,"parent_record_offset":46936,"ancestor_record_offset":43240,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":46188,"parent_record_offset":46160,"ancestor_record_offset":43240,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":46968,"parent_record_offset":46936,"ancestor_record_offset":43240,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":46936,"parent_record_offset":46908,"ancestor_record_offset":43240,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":46248,"parent_record_offset":46216,"ancestor_record_offset":43240,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":46216,"parent_record_offset":46188,"ancestor_record_offset":43240,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":46908,"parent_record_offset":46880,"ancestor_record_offset":43240,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::layout_array
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":46284,"parent_record_offset":46248,"ancestor_record_offset":43240,"inlinee":4522,"source_line":894,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":545,"record_offset":46068,"parent_record_offset":46040,"ancestor_record_offset":43240,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":545,"record_offset":46096,"parent_record_offset":46068,"ancestor_record_offset":43240,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":545,"record_offset":46124,"parent_record_offset":46096,"ancestor_record_offset":43240,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":545,"record_offset":46040,"parent_record_offset":46012,"ancestor_record_offset":43240,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":545,"record_offset":46012,"parent_record_offset":45984,"ancestor_record_offset":43240,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$48::from
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":545,"record_offset":45984,"parent_record_offset":45940,"ancestor_record_offset":43240,"inlinee":4520,"source_line":3113,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$81::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":545,"record_offset":45596,"parent_record_offset":45568,"ancestor_record_offset":43240,"inlinee":5490,"source_line":350,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::string::impl$93::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\string.rs
//     evidence={"module":545,"record_offset":43852,"parent_record_offset":43684,"ancestor_record_offset":43240,"inlinee":21307,"source_line":2666,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":545,"record_offset":46160,"parent_record_offset":46124,"ancestor_record_offset":43240,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::impl$26::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":545,"record_offset":46748,"parent_record_offset":46720,"ancestor_record_offset":43240,"inlinee":4535,"source_line":4219,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::partial_eq::impl$0::eq
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\partial_eq.rs
//     evidence={"module":545,"record_offset":45624,"parent_record_offset":45596,"ancestor_record_offset":43240,"inlinee":5492,"source_line":15,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::alloc::layout::Layout::repeat_packed
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\alloc\layout.rs
//     evidence={"module":545,"record_offset":46308,"parent_record_offset":46284,"ancestor_record_offset":43240,"inlinee":4524,"source_line":526,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::get
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":545,"record_offset":44660,"parent_record_offset":44632,"ancestor_record_offset":43240,"inlinee":5188,"source_line":552,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::cell::Cell::replace
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cell.rs
//     evidence={"module":545,"record_offset":44560,"parent_record_offset":44356,"ancestor_record_offset":43240,"inlinee":4678,"source_line":510,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":545,"record_offset":43708,"parent_record_offset":43684,"ancestor_record_offset":43240,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_loadu_si128
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":545,"record_offset":45128,"parent_record_offset":45104,"ancestor_record_offset":43240,"inlinee":4309,"source_line":1333,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::core_arch::x86::sse2::_mm_movemask_epi8
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\..\..\stdarch\crates\core_arch\src\x86\sse2.rs
//     evidence={"module":545,"record_offset":45212,"parent_record_offset":45188,"ancestor_record_offset":43240,"inlinee":4312,"source_line":1559,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::likely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":545,"record_offset":45736,"parent_record_offset":45024,"ancestor_record_offset":43240,"inlinee":5448,"source_line":433,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::intrinsics::unlikely
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\intrinsics\mod.rs
//     evidence={"module":545,"record_offset":46356,"parent_record_offset":46332,"ancestor_record_offset":43240,"inlinee":4527,"source_line":456,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":545,"record_offset":44524,"parent_record_offset":44500,"ancestor_record_offset":43240,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::checked_mul
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":545,"record_offset":46332,"parent_record_offset":46308,"ancestor_record_offset":43240,"inlinee":4525,"source_line":1160,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$11::unchecked_mul
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":545,"record_offset":47248,"parent_record_offset":47220,"ancestor_record_offset":43240,"inlinee":4120,"source_line":1220,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::num::impl$9::wrapping_add
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\uint_macros.rs
//     evidence={"module":545,"record_offset":44688,"parent_record_offset":44632,"ancestor_record_offset":43240,"inlinee":5710,"source_line":2456,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::get
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":545,"record_offset":45360,"parent_record_offset":45332,"ancestor_record_offset":43240,"inlinee":4336,"source_line":480,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::num::nonzero::NonZero::trailing_zeros
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\num\nonzero.rs
//     evidence={"module":545,"record_offset":45332,"parent_record_offset":45304,"ancestor_record_offset":43240,"inlinee":4319,"source_line":638,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":545,"record_offset":44268,"parent_record_offset":44232,"ancestor_record_offset":43240,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":545,"record_offset":44016,"parent_record_offset":43568,"ancestor_record_offset":43240,"inlinee":4171,"source_line":1160,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":545,"record_offset":44388,"parent_record_offset":44356,"ancestor_record_offset":43240,"inlinee":4428,"source_line":1061,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":545,"record_offset":46524,"parent_record_offset":46124,"ancestor_record_offset":43240,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":545,"record_offset":45152,"parent_record_offset":45128,"ancestor_record_offset":43240,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=19
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":545,"record_offset":46604,"parent_record_offset":45940,"ancestor_record_offset":43240,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":545,"record_offset":44476,"parent_record_offset":44356,"ancestor_record_offset":43240,"inlinee":5714,"source_line":1490,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::mut_ptr::impl$0::sub
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mut_ptr.rs
//     evidence={"module":545,"record_offset":45476,"parent_record_offset":45452,"ancestor_record_offset":43240,"inlinee":4325,"source_line":1033,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":545,"record_offset":43648,"parent_record_offset":43624,"ancestor_record_offset":43240,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::replace
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":545,"record_offset":44500,"parent_record_offset":44476,"ancestor_record_offset":43240,"inlinee":5715,"source_line":1542,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":545,"record_offset":43780,"parent_record_offset":43756,"ancestor_record_offset":43240,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":545,"record_offset":43808,"parent_record_offset":43780,"ancestor_record_offset":43240,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::find
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":545,"record_offset":43596,"parent_record_offset":43568,"ancestor_record_offset":43240,"inlinee":6791,"source_line":343,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::iter::impl$171::next
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\iter\macros.rs
//     evidence={"module":545,"record_offset":43624,"parent_record_offset":43596,"ancestor_record_offset":43240,"inlinee":5051,"source_line":157,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":545,"record_offset":43732,"parent_record_offset":43708,"ancestor_record_offset":43240,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::lowest_set_bit
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":545,"record_offset":45272,"parent_record_offset":45244,"ancestor_record_offset":43240,"inlinee":4316,"source_line":49,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::BitMask::nonzero_trailing_zeros
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":545,"record_offset":45304,"parent_record_offset":45272,"ancestor_record_offset":43240,"inlinee":4317,"source_line":74,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::bitmask::impl$2::next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\bitmask.rs
//     evidence={"module":545,"record_offset":45244,"parent_record_offset":45024,"ancestor_record_offset":43240,"inlinee":4314,"source_line":112,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::load
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":545,"record_offset":45104,"parent_record_offset":45024,"ancestor_record_offset":43240,"inlinee":4307,"source_line":51,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":545,"record_offset":45764,"parent_record_offset":45024,"ancestor_record_offset":43240,"inlinee":4337,"source_line":94,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::group::sse2::Group::match_tag
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\group\sse2.rs
//     evidence={"module":545,"record_offset":45188,"parent_record_offset":45024,"ancestor_record_offset":43240,"inlinee":4311,"source_line":76,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::control::tag::Tag::full
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\control\tag.rs
//     evidence={"module":545,"record_offset":45076,"parent_record_offset":45024,"ancestor_record_offset":43240,"inlinee":4405,"source_line":36,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::impl$0::equivalent
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\lib.rs
//     evidence={"module":545,"record_offset":45540,"parent_record_offset":45512,"ancestor_record_offset":43240,"inlinee":5439,"source_line":173,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::contains_key
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":545,"record_offset":44920,"parent_record_offset":44896,"ancestor_record_offset":43240,"inlinee":5590,"source_line":1416,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::HashMap::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":545,"record_offset":43488,"parent_record_offset":43464,"ancestor_record_offset":43240,"inlinee":12123,"source_line":845,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::equivalent_key::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":545,"record_offset":45512,"parent_record_offset":45404,"ancestor_record_offset":43240,"inlinee":4327,"source_line":224,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::map::make_hash
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\map.rs
//     evidence={"module":545,"record_offset":44948,"parent_record_offset":44920,"ancestor_record_offset":43240,"inlinee":4403,"source_line":253,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::Bucket::from_base_index
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":545,"record_offset":45452,"parent_record_offset":45428,"ancestor_record_offset":43240,"inlinee":4324,"source_line":312,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::ProbeSeq::move_next
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":545,"record_offset":45856,"parent_record_offset":45024,"ancestor_record_offset":43240,"inlinee":4339,"source_line":83,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::bucket
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":545,"record_offset":45428,"parent_record_offset":45404,"ancestor_record_offset":43240,"inlinee":4322,"source_line":785,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::find
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":545,"record_offset":45000,"parent_record_offset":44976,"ancestor_record_offset":43240,"inlinee":4302,"source_line":1226,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTable::get
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":545,"record_offset":44976,"parent_record_offset":44920,"ancestor_record_offset":43240,"inlinee":4301,"source_line":1249,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::RawTableInner::find_inner
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":545,"record_offset":45024,"parent_record_offset":45000,"ancestor_record_offset":43240,"inlinee":4304,"source_line":2049,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::raw::impl$8::find::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\raw\mod.rs
//     evidence={"module":545,"record_offset":45404,"parent_record_offset":45024,"ancestor_record_offset":43240,"inlinee":4321,"source_line":1236,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::contains
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":545,"record_offset":44896,"parent_record_offset":44872,"ancestor_record_offset":43240,"inlinee":5589,"source_line":862,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::insert
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":545,"record_offset":46660,"parent_record_offset":46636,"ancestor_record_offset":43240,"inlinee":5647,"source_line":1092,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] hashbrown::set::HashSet::is_empty
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"hashbrown","version":"0.12.3","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.14.5","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.15.2","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.16.1","source":"registry+https://github.com/rust-lang/crates.io-index"},{"name":"hashbrown","version":"0.17.1","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/rust/deps\hashbrown-0.16.1\src\set.rs
//     evidence={"module":545,"record_offset":43464,"parent_record_offset":43440,"ancestor_record_offset":43240,"inlinee":26555,"source_line":326,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] std::collections::hash::set::impl$13::default
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\collections\hash\set.rs
//     evidence={"module":545,"record_offset":44120,"parent_record_offset":44092,"ancestor_record_offset":43240,"inlinee":5641,"source_line":1246,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::RandomState::new
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":545,"record_offset":44176,"parent_record_offset":44148,"ancestor_record_offset":43240,"inlinee":5645,"source_line":55,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::__rust_std_internal_init_fn
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":545,"record_offset":44436,"parent_record_offset":44412,"ancestor_record_offset":43240,"inlinee":5713,"source_line":85,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::KEYS::constant$0::closure$1
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\mod.rs
//     evidence={"module":545,"record_offset":44296,"parent_record_offset":44268,"ancestor_record_offset":43240,"inlinee":5705,"source_line":100,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$0::new::closure$0
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":545,"record_offset":44632,"parent_record_offset":44232,"ancestor_record_offset":43240,"inlinee":5709,"source_line":71,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] std::hash::random::impl$5::default
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\hash\random.rs
//     evidence={"module":545,"record_offset":44148,"parent_record_offset":44120,"ancestor_record_offset":43240,"inlinee":5643,"source_line":149,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":545,"record_offset":44324,"parent_record_offset":44296,"ancestor_record_offset":43240,"inlinee":5707,"source_line":58,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] std::sys::thread_local::native::lazy::Storage::get_or_init_slow
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\sys\thread_local\native\lazy.rs
//     evidence={"module":545,"record_offset":44356,"parent_record_offset":44324,"ancestor_record_offset":43240,"inlinee":5711,"source_line":69,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::try_with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":545,"record_offset":44232,"parent_record_offset":44204,"ancestor_record_offset":43240,"inlinee":5703,"source_line":508,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] std::thread::local::LocalKey::with
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\thread\local.rs
//     evidence={"module":545,"record_offset":44204,"parent_record_offset":44176,"ancestor_record_offset":43240,"inlinee":5702,"source_line":473,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] turbosloth::store::impl$6::asset::closure$0
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"turbosloth","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/turbosloth/src/store.rs
//     evidence={"module":545,"record_offset":43684,"parent_record_offset":43596,"ancestor_record_offset":43240,"inlinee":30964,"source_line":168,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_textures::_::impl$0::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=6
// 源码行: 226
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8455
// 首条位置: {"module":545,"inlinee":33877}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_textures::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 记录形式: compiled_procedure; instances=1
// 源码行: 226
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":1761,"record_offset":137016,"rva":28014736,"va":5396723856}
// 编译实例 RVAs: 0x1ab7890
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::str::traits::impl$1::eq
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\str\traits.rs
//     evidence={"module":1761,"record_offset":137208,"parent_record_offset":137016,"ancestor_record_offset":137016,"inlinee":5443,"source_line":29,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::impls::impl$9::eq
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":1761,"record_offset":137236,"parent_record_offset":137208,"ancestor_record_offset":137016,"inlinee":5441,"source_line":2122,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$0::eq
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1761,"record_offset":137264,"parent_record_offset":137236,"ancestor_record_offset":137016,"inlinee":5445,"source_line":18,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   [pdb_inline_transitive] core::slice::cmp::impl$5::equal_same_length
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\slice\cmp.rs
//     evidence={"module":1761,"record_offset":137292,"parent_record_offset":137264,"ancestor_record_offset":137016,"inlinee":5447,"source_line":148,"archive":"_pdb/C/modules/01761.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_textures::_::impl$0::deserialize::impl$1::deserialize
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 226
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13E51
// 首条位置: {"module":1645,"inlinee":81489}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] serde_json::de::impl$19::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1645,"record_offset":265316,"parent_record_offset":265292,"ancestor_record_offset":265292,"inlinee":7696,"source_line":125,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::de::impl$21::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1803,"record_offset":401024,"parent_record_offset":401000,"ancestor_record_offset":401000,"inlinee":37547,"source_line":125,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":401300,"parent_record_offset":401276,"ancestor_record_offset":401000,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":401324,"parent_record_offset":401300,"ancestor_record_offset":401000,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":401348,"parent_record_offset":401324,"ancestor_record_offset":401000,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1803,"record_offset":401276,"parent_record_offset":401220,"ancestor_record_offset":401000,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":401248,"parent_record_offset":401220,"ancestor_record_offset":401000,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":401220,"parent_record_offset":401196,"ancestor_record_offset":401000,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1803,"record_offset":401196,"parent_record_offset":401172,"ancestor_record_offset":401000,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1645,"record_offset":265464,"parent_record_offset":265340,"ancestor_record_offset":265292,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":401124,"parent_record_offset":401096,"ancestor_record_offset":401000,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Visitor::visit_string
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":401096,"parent_record_offset":401072,"ancestor_record_offset":401000,"inlinee":10022,"source_line":1568,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":265380,"parent_record_offset":265340,"ancestor_record_offset":265292,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":265340,"parent_record_offset":265316,"ancestor_record_offset":265292,"inlinee":7593,"source_line":2215,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1645,"record_offset":265404,"parent_record_offset":265380,"ancestor_record_offset":265292,"inlinee":7606,"source_line":693,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1645,"record_offset":265428,"parent_record_offset":265404,"ancestor_record_offset":265292,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$21::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":401048,"parent_record_offset":401024,"ancestor_record_offset":401000,"inlinee":21295,"source_line":1248,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$26::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":401072,"parent_record_offset":401048,"ancestor_record_offset":401000,"inlinee":21297,"source_line":1420,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_textures::_::impl$0::deserialize::impl$2::expecting
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 226
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x847A
// 首条位置: {"module":1761,"inlinee":33914}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   没有已归属的调用/内联证据；不据此认定函数没有依赖。
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_textures::_::impl$0::deserialize::impl$2::visit_map
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 226
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13E4F
// 首条位置: {"module":1645,"inlinee":81487}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::option::Option::is_some
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1645,"record_offset":265516,"parent_record_offset":265068,"ancestor_record_offset":265068,"inlinee":4878,"source_line":635,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=76
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1645,"record_offset":272184,"parent_record_offset":265068,"ancestor_record_offset":265068,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1645,"record_offset":271584,"parent_record_offset":265068,"ancestor_record_offset":265068,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=21
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1804,"record_offset":164916,"parent_record_offset":159708,"ancestor_record_offset":159708,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde::private::de::missing_field
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1645,"record_offset":271904,"parent_record_offset":265068,"ancestor_record_offset":265068,"inlinee":7734,"source_line":24,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::Error::duplicate_field
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":278448,"parent_record_offset":265068,"ancestor_record_offset":265068,"inlinee":7740,"source_line":296,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":265172,"parent_record_offset":265068,"ancestor_record_offset":265068,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":265544,"parent_record_offset":265068,"ancestor_record_offset":265068,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":159776,"parent_record_offset":159708,"ancestor_record_offset":159708,"inlinee":17556,"source_line":1987,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$7::next_value
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":159896,"parent_record_offset":159708,"ancestor_record_offset":159708,"inlinee":11180,"source_line":1995,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=76
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":272396,"parent_record_offset":272372,"ancestor_record_offset":265068,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=76
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":272420,"parent_record_offset":272396,"ancestor_record_offset":265068,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=76
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":272444,"parent_record_offset":272420,"ancestor_record_offset":265068,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=76
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":272372,"parent_record_offset":272300,"ancestor_record_offset":265068,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=56
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1645,"record_offset":272336,"parent_record_offset":272300,"ancestor_record_offset":265068,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=76
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1645,"record_offset":272300,"parent_record_offset":272268,"ancestor_record_offset":265068,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=76
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1645,"record_offset":272268,"parent_record_offset":272240,"ancestor_record_offset":265068,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::clear
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":1645,"record_offset":265464,"parent_record_offset":265340,"ancestor_record_offset":265068,"inlinee":7435,"source_line":2968,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::mem::replace
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\mem\mod.rs
//     evidence={"module":1804,"record_offset":160072,"parent_record_offset":160048,"ancestor_record_offset":159708,"inlinee":4500,"source_line":887,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::take
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":1804,"record_offset":160048,"parent_record_offset":159992,"ancestor_record_offset":159708,"inlinee":4498,"source_line":1884,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=162
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1645,"record_offset":272212,"parent_record_offset":272184,"ancestor_record_offset":265068,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_textures::_::impl$0::deserialize::impl$1::deserialize
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_textures.rs
//     evidence={"module":1645,"record_offset":265292,"parent_record_offset":265268,"ancestor_record_offset":265068,"inlinee":81489,"source_line":226,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_any
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"serde","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde-1.0.228/src/private/de.rs
//     evidence={"module":1645,"record_offset":271976,"parent_record_offset":271952,"ancestor_record_offset":265068,"inlinee":7737,"source_line":38,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde::private::de::missing_field::impl$0::deserialize_string
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1645,"record_offset":271952,"parent_record_offset":271928,"ancestor_record_offset":265068,"inlinee":7748,"source_line":125,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::Error::missing_field
//     owner=registry_dependency_definition; count=20
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":272000,"parent_record_offset":271976,"ancestor_record_offset":265068,"inlinee":7739,"source_line":289,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_key
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":159800,"parent_record_offset":159776,"ancestor_record_offset":159708,"inlinee":7723,"source_line":1897,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::MapAccess::next_value
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1804,"record_offset":159944,"parent_record_offset":159896,"ancestor_record_offset":159708,"inlinee":7732,"source_line":1914,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::ignored_any::impl$1::deserialize
//     owner=registry_dependency_definition; count=2
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/ignored_any.rs
//     evidence={"module":1645,"record_offset":270964,"parent_record_offset":270940,"ancestor_record_offset":265068,"inlinee":7601,"source_line":232,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=23
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":265268,"parent_record_offset":265208,"ancestor_record_offset":265068,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=40
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1645,"record_offset":266028,"parent_record_offset":266004,"ancestor_record_offset":265068,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::eat_char
//     owner=git_dependency_definition; count=23
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":265380,"parent_record_offset":265340,"ancestor_record_offset":265068,"inlinee":7583,"source_line":227,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_object_colon
//     owner=git_dependency_definition; count=11
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":265636,"parent_record_offset":265588,"ancestor_record_offset":265068,"inlinee":7597,"source_line":1061,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::parse_whitespace
//     owner=git_dependency_definition; count=11
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":265684,"parent_record_offset":265636,"ancestor_record_offset":265068,"inlinee":7579,"source_line":255,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::Deserializer::peek
//     owner=git_dependency_definition; count=11
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":265724,"parent_record_offset":265684,"ancestor_record_offset":265068,"inlinee":7580,"source_line":219,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_key_seed
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":265208,"parent_record_offset":265172,"ancestor_record_offset":265068,"inlinee":7724,"source_line":1986,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$11::next_value_seed
//     owner=git_dependency_definition; count=11
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":265588,"parent_record_offset":265544,"ancestor_record_offset":265068,"inlinee":7596,"source_line":2027,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":265340,"parent_record_offset":265316,"ancestor_record_offset":265068,"inlinee":7593,"source_line":2215,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$19::deserialize_identifier
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"serde_core","version":"1.0.228","source":"PDB_registry_path_only"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/macros.rs
//     evidence={"module":1645,"record_offset":265316,"parent_record_offset":265292,"ancestor_record_offset":265068,"inlinee":7696,"source_line":125,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$7::deserialize_ignored_any
//     owner=git_dependency_definition; count=1
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":270988,"parent_record_offset":270964,"ancestor_record_offset":265068,"inlinee":7602,"source_line":1910,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::discard
//     owner=git_dependency_definition; count=23
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1645,"record_offset":265404,"parent_record_offset":265380,"ancestor_record_offset":265068,"inlinee":7606,"source_line":693,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$10::peek
//     owner=git_dependency_definition; count=11
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1645,"record_offset":265752,"parent_record_offset":265724,"ancestor_record_offset":265068,"inlinee":7605,"source_line":688,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::discard
//     owner=git_dependency_definition; count=23
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1645,"record_offset":265428,"parent_record_offset":265404,"ancestor_record_offset":265068,"inlinee":7584,"source_line":569,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::read::impl$7::peek
//     owner=git_dependency_definition; count=11
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/read.rs
//     evidence={"module":1645,"record_offset":265780,"parent_record_offset":265752,"ancestor_record_offset":265068,"inlinee":7582,"source_line":558,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$12::next_value_seed
//     owner=git_dependency_definition; count=11
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":159992,"parent_record_offset":159944,"ancestor_record_offset":159708,"inlinee":21314,"source_line":687,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_string
//     owner=git_dependency_definition; count=10
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1804,"record_offset":160196,"parent_record_offset":160152,"ancestor_record_offset":159708,"inlinee":33611,"source_line":396,"archive":"_pdb/C/modules/01804.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_textures::_::impl$0::deserialize::impl$2::visit_seq
// 记录形式: inline_instance, inlinee_declaration; instances=4
// 源码行: 226
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x13E52
// 首条位置: {"module":1645,"inlinee":81490}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1645,"record_offset":285332,"parent_record_offset":284640,"ancestor_record_offset":284640,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=8
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1645,"record_offset":286540,"parent_record_offset":284640,"ancestor_record_offset":284640,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":284732,"parent_record_offset":284640,"ancestor_record_offset":284640,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_direct] serde_core::de::impl$6::next_element
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":40876,"parent_record_offset":40844,"ancestor_record_offset":40844,"inlinee":15022,"source_line":1798,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":285536,"parent_record_offset":285512,"ancestor_record_offset":284640,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":285560,"parent_record_offset":285536,"ancestor_record_offset":284640,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":285584,"parent_record_offset":285560,"ancestor_record_offset":284640,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":1645,"record_offset":285512,"parent_record_offset":285444,"ancestor_record_offset":284640,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1645,"record_offset":285480,"parent_record_offset":285444,"ancestor_record_offset":284640,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1645,"record_offset":285444,"parent_record_offset":285416,"ancestor_record_offset":284640,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=36
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":1645,"record_offset":285416,"parent_record_offset":285388,"ancestor_record_offset":284640,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::into_iter::impl$6::next
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\into_iter.rs
//     evidence={"module":1803,"record_offset":41008,"parent_record_offset":40964,"ancestor_record_offset":40844,"inlinee":5075,"source_line":213,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=74
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1645,"record_offset":285360,"parent_record_offset":285332,"ancestor_record_offset":284640,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::add
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1803,"record_offset":41064,"parent_record_offset":41008,"ancestor_record_offset":40844,"inlinee":5058,"source_line":651,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::NonNull::read
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1803,"record_offset":41092,"parent_record_offset":41008,"ancestor_record_offset":40844,"inlinee":5076,"source_line":985,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::non_null::impl$16::eq
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\non_null.rs
//     evidence={"module":1803,"record_offset":41036,"parent_record_offset":41008,"ancestor_record_offset":40844,"inlinee":5053,"source_line":1719,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::read
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":1803,"record_offset":41116,"parent_record_offset":41092,"ancestor_record_offset":40844,"inlinee":4644,"source_line":1678,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] core::result::Result::map
//     owner=rust_standard_library; count=6
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":1803,"record_offset":41328,"parent_record_offset":40964,"ancestor_record_offset":40844,"inlinee":5915,"source_line":831,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::SeqAccess::next_element
//     owner=registry_dependency_definition; count=6
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1803,"record_offset":40920,"parent_record_offset":40876,"ancestor_record_offset":40844,"inlinee":7646,"source_line":1769,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impl$5::deserialize
//     owner=registry_dependency_definition; count=12
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/mod.rs
//     evidence={"module":1645,"record_offset":284820,"parent_record_offset":284756,"ancestor_record_offset":284640,"inlinee":7599,"source_line":821,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_core::de::impls::impl$8::deserialize
//     owner=registry_dependency_definition; count=8
//     packages=[{"name":"serde_core","version":"1.0.228","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/src/de/impls.rs
//     evidence={"module":1645,"record_offset":284844,"parent_record_offset":284820,"ancestor_record_offset":284640,"inlinee":7665,"source_line":689,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::de::impl$9::next_element_seed
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/de.rs
//     evidence={"module":1645,"record_offset":284756,"parent_record_offset":284732,"ancestor_record_offset":284640,"inlinee":7609,"source_line":1933,"archive":"_pdb/C/modules/01645.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$10::next_element_seed
//     owner=git_dependency_definition; count=6
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":40964,"parent_record_offset":40920,"ancestor_record_offset":40844,"inlinee":21316,"source_line":636,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::de::impl$4::deserialize_string
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/de.rs
//     evidence={"module":1803,"record_offset":41232,"parent_record_offset":41192,"ancestor_record_offset":40844,"inlinee":33611,"source_line":396,"archive":"_pdb/C/modules/01803.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_textures::GladeTextureMappings::create_mappings
// 记录形式: compiled_procedure; instances=1
// 源码行: 232
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":545,"record_offset":13016,"rva":11093408,"va":5379802528}
// 编译实例 RVAs: 0xa945a0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t-4123892","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379802528,"raw_hex":"e80c13c1ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5379802575,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=19
//     evidence={"assembly":"\tcallq\t-4123985","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379802528,"raw_hex":"e8af12c1ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379802684,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=10
//     target_aliases=1699 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11080373","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379802528,"raw_hex":"e84bed56ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5379802560,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=8
//     evidence={"assembly":"\tcallq\t36816266","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379802528,"raw_hex":"e88ac53102","size":5,"soft_fail":false,"target_va":5416620794,"va":5379804523,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t36818264","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379802528,"raw_hex":"e858cd3102","size":5,"soft_fail":false,"target_va":5416622240,"va":5379803971,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::try_create_name_mapping
//     owner=package_namespace_candidate; count=10
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-522372","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379802528,"raw_hex":"e87c07f8ff","size":5,"soft_fail":false,"target_va":5379280288,"va":5379802655,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_external_jump_candidate] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=1
//     evidence={"assembly":"\tjmp\t-4125218","direct":true,"event":"external_jump","llvm_opcode":"JMP_4","range_va":5379802528,"raw_hex":"e9de0dc1ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379803917,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":545,"record_offset":13184,"parent_record_offset":13016,"ancestor_record_offset":13016,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":545,"record_offset":13728,"parent_record_offset":13016,"ancestor_record_offset":13016,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::texture_library::AssetTextureLibrary::create_name_mapping
//     owner=private_source_ownership_candidate; count=11
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/texture_library.rs
//     evidence={"module":545,"record_offset":13692,"parent_record_offset":13016,"ancestor_record_offset":13016,"inlinee":32035,"source_line":224,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":13500,"parent_record_offset":13476,"ancestor_record_offset":13016,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":13524,"parent_record_offset":13500,"ancestor_record_offset":13016,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":13872,"parent_record_offset":13848,"ancestor_record_offset":13016,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":13896,"parent_record_offset":13872,"ancestor_record_offset":13016,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":13548,"parent_record_offset":13524,"ancestor_record_offset":13016,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":13920,"parent_record_offset":13896,"ancestor_record_offset":13016,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":13476,"parent_record_offset":13448,"ancestor_record_offset":13016,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":13848,"parent_record_offset":13824,"ancestor_record_offset":13016,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":13400,"parent_record_offset":13376,"ancestor_record_offset":13016,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":13824,"parent_record_offset":13800,"ancestor_record_offset":13016,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":13448,"parent_record_offset":13424,"ancestor_record_offset":13016,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":13424,"parent_record_offset":13400,"ancestor_record_offset":13016,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":13800,"parent_record_offset":13776,"ancestor_record_offset":13016,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":545,"record_offset":13300,"parent_record_offset":13276,"ancestor_record_offset":13016,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":545,"record_offset":13324,"parent_record_offset":13300,"ancestor_record_offset":13016,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":545,"record_offset":13348,"parent_record_offset":13324,"ancestor_record_offset":13016,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":545,"record_offset":13276,"parent_record_offset":13252,"ancestor_record_offset":13016,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":545,"record_offset":13252,"parent_record_offset":13232,"ancestor_record_offset":13016,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":545,"record_offset":13376,"parent_record_offset":13348,"ancestor_record_offset":13016,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":545,"record_offset":13232,"parent_record_offset":13208,"ancestor_record_offset":13016,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":545,"record_offset":13208,"parent_record_offset":13184,"ancestor_record_offset":13016,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":545,"record_offset":13604,"parent_record_offset":13348,"ancestor_record_offset":13016,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":545,"record_offset":13628,"parent_record_offset":13604,"ancestor_record_offset":13016,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":545,"record_offset":13752,"parent_record_offset":13728,"ancestor_record_offset":13016,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_textures::GladeTextureMappings::create_mappings_with_fallback
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 244
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8457
// 首条位置: {"module":545,"inlinee":33879}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] alloc::fmt::format
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":545,"record_offset":27796,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":545,"record_offset":28444,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":545,"record_offset":36316,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] country_core::asset_libraries::texture_library::AssetTextureLibrary::create_name_mapping
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/texture_library.rs
//     evidence={"module":545,"record_offset":28612,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":32035,"source_line":224,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":545,"record_offset":28500,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":545,"record_offset":28360,"parent_record_offset":27580,"ancestor_record_offset":27580,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":28168,"parent_record_offset":28144,"ancestor_record_offset":27580,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":28192,"parent_record_offset":28168,"ancestor_record_offset":27580,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":36460,"parent_record_offset":36436,"ancestor_record_offset":27580,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":36484,"parent_record_offset":36460,"ancestor_record_offset":27580,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":28216,"parent_record_offset":28192,"ancestor_record_offset":27580,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":36508,"parent_record_offset":36484,"ancestor_record_offset":27580,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":28144,"parent_record_offset":28116,"ancestor_record_offset":27580,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":36436,"parent_record_offset":36412,"ancestor_record_offset":27580,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":28056,"parent_record_offset":28028,"ancestor_record_offset":27580,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":36412,"parent_record_offset":36388,"ancestor_record_offset":27580,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":28116,"parent_record_offset":28084,"ancestor_record_offset":27580,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":28084,"parent_record_offset":28056,"ancestor_record_offset":27580,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=20
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":36388,"parent_record_offset":36364,"ancestor_record_offset":27580,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":545,"record_offset":27936,"parent_record_offset":27908,"ancestor_record_offset":27580,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":545,"record_offset":27964,"parent_record_offset":27936,"ancestor_record_offset":27580,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":545,"record_offset":27992,"parent_record_offset":27964,"ancestor_record_offset":27580,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":545,"record_offset":27908,"parent_record_offset":27880,"ancestor_record_offset":27580,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":545,"record_offset":27880,"parent_record_offset":27852,"ancestor_record_offset":27580,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":545,"record_offset":28028,"parent_record_offset":27992,"ancestor_record_offset":27580,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":545,"record_offset":27852,"parent_record_offset":27824,"ancestor_record_offset":27580,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":545,"record_offset":28468,"parent_record_offset":28444,"ancestor_record_offset":27580,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":545,"record_offset":27824,"parent_record_offset":27796,"ancestor_record_offset":27580,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":545,"record_offset":28272,"parent_record_offset":27992,"ancestor_record_offset":27580,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":545,"record_offset":28296,"parent_record_offset":28272,"ancestor_record_offset":27580,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=40
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":545,"record_offset":36340,"parent_record_offset":36316,"ancestor_record_offset":27580,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":545,"record_offset":28384,"parent_record_offset":28360,"ancestor_record_offset":27580,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":545,"record_offset":28408,"parent_record_offset":28384,"ancestor_record_offset":27580,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":545,"record_offset":28548,"parent_record_offset":28524,"ancestor_record_offset":27580,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":545,"record_offset":28524,"parent_record_offset":28500,"ancestor_record_offset":27580,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":545,"record_offset":28572,"parent_record_offset":28548,"ancestor_record_offset":27580,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_textures::load_glade_texture_mappings
// 记录形式: compiled_procedure; instances=1
// 源码行: 281
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":545,"record_offset":24332,"rva":11096000,"va":5379805120}
// 编译实例 RVAs: 0xa94fc0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] <std::path::Path>::_join
//     owner=unresolved_namespace; count=2
//     evidence={"assembly":"\tcallq\t4851016","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379805120,"raw_hex":"e848054a00","size":5,"soft_fail":false,"target_va":5384656240,"va":5379805219,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log
//     owner=unresolved_namespace; count=11
//     evidence={"assembly":"\tcallq\t4803052","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379805120,"raw_hex":"e8ec494900","size":5,"soft_fail":false,"target_va":5384609408,"va":5379806351,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_alloc
//     owner=unresolved_namespace; count=10
//     evidence={"assembly":"\tcallq\t-4127780","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379805120,"raw_hex":"e8dc03c1ff","size":5,"soft_fail":false,"target_va":5375678688,"va":5379806463,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] __rustc::__rust_dealloc
//     owner=unresolved_namespace; count=29
//     evidence={"assembly":"\tcallq\t-4126603","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379805120,"raw_hex":"e87508c1ff","size":5,"soft_fail":false,"target_va":5375678704,"va":5379805302,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140003310
//     owner=unresolved_namespace; count=10
//     target_aliases=1699 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-11084261","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379805120,"raw_hex":"e81bde56ff","size":5,"soft_fail":false,"target_va":5368722192,"va":5379806448,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14099f3c0
//     owner=unresolved_namespace; count=6
//     target_aliases=2 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t-1007289","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379805120,"raw_hex":"e847a1f0ff","size":5,"soft_fail":false,"target_va":5378798528,"va":5379805812,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x140b993f0
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t1065547","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379805120,"raw_hex":"e84b421000","size":5,"soft_fail":false,"target_va":5380871152,"va":5379805600,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x14150f870
//     owner=unresolved_namespace; count=2
//     target_aliases=2 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t10987102","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379805120,"raw_hex":"e85ea6a700","size":5,"soft_fail":false,"target_va":5390792816,"va":5379805709,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x141651550
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t12305446","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379805120,"raw_hex":"e826c4bb00","size":5,"soft_fail":false,"target_va":5392110928,"va":5379805477,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd54f0
//     owner=unresolved_namespace; count=3
//     target_aliases=2 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36963235","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379805120,"raw_hex":"e8a3033402","size":5,"soft_fail":false,"target_va":5416768752,"va":5379805512,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142dd56b0
//     owner=unresolved_namespace; count=1
//     target_aliases=2 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t36957414","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379805120,"raw_hex":"e8e6ec3302","size":5,"soft_fail":false,"target_va":5416769200,"va":5379811781,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alias_set_at_0x142e1bb20
//     owner=unresolved_namespace; count=4
//     target_aliases=1 candidates; complete list: load_textures.rs.dependencies.json
//     evidence={"assembly":"\tcallq\t37251290","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379805120,"raw_hex":"e8da683802","size":5,"soft_fail":false,"target_va":5417057056,"va":5379805761,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] alloc::raw_vec::handle_error
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t36808929","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379805120,"raw_hex":"e8e1a83102","size":5,"soft_fail":false,"target_va":5416620794,"va":5379811860,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t4817689","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379805120,"raw_hex":"e819834900","size":5,"soft_fail":false,"target_va":5384624080,"va":5379806386,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=10
//     evidence={"assembly":"\tcallq\t36811134","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379805120,"raw_hex":"e87eb13102","size":5,"soft_fail":false,"target_va":5416622240,"va":5379811101,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::asset_libraries::texture_library::AssetTextureLibrary::try_create_name_mapping
//     owner=package_namespace_candidate; count=20
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-526249","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379805120,"raw_hex":"e857f8f7ff","size":5,"soft_fail":false,"target_va":5379280288,"va":5379806532,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_textures::GladeTextureMappings::create_mappings
//     owner=package_namespace_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t-3276","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379805120,"raw_hex":"e834f3ffff","size":5,"soft_fail":false,"target_va":5379802528,"va":5379805799,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_textures::load_glade_texture_mappings::closure$0
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t36575532","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379805120,"raw_hex":"e82c192e02","size":5,"soft_fail":false,"target_va":5416387376,"va":5379811839,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] country_core::startup::load_textures::load_glade_texture_mappings::closure$2
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t36575443","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379805120,"raw_hex":"e8d3182e02","size":5,"soft_fail":false,"target_va":5416387248,"va":5379811800,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] std::fs::read_to_string::inner
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t4945839","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379805120,"raw_hex":"e8af774b00","size":5,"soft_fail":false,"target_va":5384751264,"va":5379805420,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::is_mod_file
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3079072","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379805120,"raw_hex":"e8a0fb2e00","size":5,"soft_fail":false,"target_va":5382884432,"va":5379805355,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] tg_mods::overlay::resolve
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"tg-mods","version":"0.1.0","source":"local_or_path"}]
//     evidence={"assembly":"\tcallq\t3114315","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5379805120,"raw_hex":"e84b852f00","size":5,"soft_fail":false,"target_va":5382919648,"va":5379805328,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] bevy_ecs::change_detection::impl$38::deref_mut
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":545,"record_offset":26888,"parent_record_offset":24332,"ancestor_record_offset":24332,"inlinee":7222,"source_line":434,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":545,"record_offset":27264,"parent_record_offset":24332,"ancestor_record_offset":24332,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":545,"record_offset":24672,"parent_record_offset":24332,"ancestor_record_offset":24332,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::and_then
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":545,"record_offset":25532,"parent_record_offset":24332,"ancestor_record_offset":24332,"inlinee":7484,"source_line":1488,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::map_err
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":545,"record_offset":25464,"parent_record_offset":24332,"ancestor_record_offset":24332,"inlinee":4574,"source_line":962,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] core::result::Result::unwrap_or_else
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":545,"record_offset":26848,"parent_record_offset":24332,"ancestor_record_offset":24332,"inlinee":6444,"source_line":1616,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] country_core::resources::session::ActiveSession::assets_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":545,"record_offset":24532,"parent_record_offset":24332,"ancestor_record_offset":24332,"inlinee":8070,"source_line":291,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_textures::GladeTextureMappings::create_mappings_with_fallback
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_textures.rs
//     evidence={"module":545,"record_offset":27580,"parent_record_offset":24332,"ancestor_record_offset":24332,"inlinee":33879,"source_line":244,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] country_core::startup::load_textures::read_glade_mappings
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_textures.rs
//     evidence={"module":545,"record_offset":26636,"parent_record_offset":24332,"ancestor_record_offset":24332,"inlinee":33875,"source_line":316,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] log::__private_api::log
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":545,"record_offset":27320,"parent_record_offset":24332,"ancestor_record_offset":24332,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] log::max_level
//     owner=registry_dependency_definition; count=1
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":545,"record_offset":27180,"parent_record_offset":24332,"ancestor_record_offset":24332,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] std::fs::read_to_string
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\fs.rs
//     evidence={"module":545,"record_offset":25408,"parent_record_offset":24332,"ancestor_record_offset":24332,"inlinee":8116,"source_line":383,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":545,"record_offset":24644,"parent_record_offset":24332,"ancestor_record_offset":24332,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":28168,"parent_record_offset":28144,"ancestor_record_offset":24332,"inlinee":4236,"source_line":311,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::alloc_impl_runtime
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":28192,"parent_record_offset":28168,"ancestor_record_offset":24332,"inlinee":4237,"source_line":185,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":24920,"parent_record_offset":24896,"ancestor_record_offset":24332,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":24944,"parent_record_offset":24920,"ancestor_record_offset":24332,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::alloc
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":28216,"parent_record_offset":28192,"ancestor_record_offset":24332,"inlinee":4238,"source_line":89,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":24968,"parent_record_offset":24944,"ancestor_record_offset":24332,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::allocate
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":28144,"parent_record_offset":28116,"ancestor_record_offset":24332,"inlinee":4235,"source_line":428,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":24896,"parent_record_offset":24840,"ancestor_record_offset":24332,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::fmt::format
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\fmt.rs
//     evidence={"module":545,"record_offset":27796,"parent_record_offset":27580,"ancestor_record_offset":24332,"inlinee":6850,"source_line":649,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVec::with_capacity_in
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":28056,"parent_record_offset":28028,"ancestor_record_offset":24332,"inlinee":4454,"source_line":175,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=9
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":24868,"parent_record_offset":24840,"ancestor_record_offset":24332,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":24840,"parent_record_offset":24816,"ancestor_record_offset":24332,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::try_allocate_in
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":28116,"parent_record_offset":28084,"ancestor_record_offset":24332,"inlinee":4456,"source_line":445,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::with_capacity_in
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":28084,"parent_record_offset":28056,"ancestor_record_offset":24332,"inlinee":4455,"source_line":432,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=29
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":24816,"parent_record_offset":24792,"ancestor_record_offset":24332,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":545,"record_offset":27936,"parent_record_offset":27908,"ancestor_record_offset":24332,"inlinee":4449,"source_line":372,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":545,"record_offset":27964,"parent_record_offset":27936,"ancestor_record_offset":24332,"inlinee":4450,"source_line":396,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$0::to_vec_in::impl$1::to_vec
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":545,"record_offset":27992,"parent_record_offset":27964,"ancestor_record_offset":24332,"inlinee":4452,"source_line":446,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::slice::impl$9::to_owned
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\slice.rs
//     evidence={"module":545,"record_offset":27908,"parent_record_offset":27880,"ancestor_record_offset":24332,"inlinee":4447,"source_line":841,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::str::impl$4::to_owned
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\str.rs
//     evidence={"module":545,"record_offset":27880,"parent_record_offset":27852,"ancestor_record_offset":24332,"inlinee":4445,"source_line":210,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::vec::Vec::with_capacity_in
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\vec\mod.rs
//     evidence={"module":545,"record_offset":28028,"parent_record_offset":27992,"ancestor_record_offset":24332,"inlinee":4453,"source_line":964,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] bevy_ecs::change_detection::impl$37::set_changed
//     owner=registry_dependency_definition; count=3
//     packages=[{"name":"bevy_ecs","version":"0.17.3","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.17.3/src/change_detection.rs
//     evidence={"module":545,"record_offset":26916,"parent_record_offset":26888,"ancestor_record_offset":24332,"inlinee":7224,"source_line":397,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::cmp::PartialOrd::le
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\cmp.rs
//     evidence={"module":545,"record_offset":28444,"parent_record_offset":27580,"ancestor_record_offset":24332,"inlinee":5550,"source_line":1428,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::convert::impl$0::as_ref
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\convert\mod.rs
//     evidence={"module":545,"record_offset":25432,"parent_record_offset":25408,"ancestor_record_offset":24332,"inlinee":5107,"source_line":717,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ops::function::FnOnce::call_once
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ops\function.rs
//     evidence={"module":545,"record_offset":25500,"parent_record_offset":25464,"ancestor_record_offset":24332,"inlinee":4169,"source_line":250,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::is_some_and
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":545,"record_offset":27288,"parent_record_offset":27264,"ancestor_record_offset":24332,"inlinee":4223,"source_line":661,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::option::Option::map_or_else
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\option.rs
//     evidence={"module":545,"record_offset":27824,"parent_record_offset":27796,"ancestor_record_offset":24332,"inlinee":6851,"source_line":1271,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::const_ptr::impl$0::copy_to_nonoverlapping
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\const_ptr.rs
//     evidence={"module":545,"record_offset":28272,"parent_record_offset":27992,"ancestor_record_offset":24332,"inlinee":4458,"source_line":1247,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::copy_nonoverlapping
//     owner=rust_standard_library; count=10
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":545,"record_offset":28296,"parent_record_offset":28272,"ancestor_record_offset":24332,"inlinee":4310,"source_line":526,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=101
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":545,"record_offset":24696,"parent_record_offset":24672,"ancestor_record_offset":24332,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$27::branch
//     owner=rust_standard_library; count=5
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":545,"record_offset":25600,"parent_record_offset":25568,"ancestor_record_offset":24332,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=3
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":545,"record_offset":25640,"parent_record_offset":25568,"ancestor_record_offset":24332,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::AtomicUsize::load
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":545,"record_offset":27204,"parent_record_offset":27180,"ancestor_record_offset":24332,"inlinee":4712,"source_line":2842,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::sync::atomic::atomic_load
//     owner=rust_standard_library; count=11
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\sync\atomic.rs
//     evidence={"module":545,"record_offset":27228,"parent_record_offset":27204,"ancestor_record_offset":24332,"inlinee":4203,"source_line":3886,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] country_core::asset_libraries::texture_library::AssetTextureLibrary::create_name_mapping
//     owner=private_source_ownership_candidate; count=10
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/asset_libraries/texture_library.rs
//     evidence={"module":545,"record_offset":28612,"parent_record_offset":27580,"ancestor_record_offset":24332,"inlinee":32035,"source_line":224,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::GladeType::assets_folder
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":545,"record_offset":24580,"parent_record_offset":24556,"ancestor_record_offset":24332,"inlinee":27998,"source_line":52,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] country_core::resources::session::SessionMeta::assets_root
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/resources/session.rs
//     evidence={"module":545,"record_offset":24556,"parent_record_offset":24532,"ancestor_record_offset":24332,"inlinee":27997,"source_line":208,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_textures::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_textures.rs
//     evidence={"module":545,"record_offset":26748,"parent_record_offset":26724,"ancestor_record_offset":24332,"inlinee":33877,"source_line":226,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_textures::load_glade_texture_mappings::closure$1
//     owner=private_source_ownership_candidate; count=1
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_textures.rs
//     evidence={"module":545,"record_offset":25568,"parent_record_offset":25532,"ancestor_record_offset":24332,"inlinee":33873,"source_line":301,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] log::RecordBuilder::build
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":545,"record_offset":27368,"parent_record_offset":27344,"ancestor_record_offset":24332,"inlinee":5216,"source_line":1033,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":545,"record_offset":28500,"parent_record_offset":27580,"ancestor_record_offset":24332,"inlinee":5908,"source_line":84,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] log::__private_api::log_impl
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/__private_api.rs
//     evidence={"module":545,"record_offset":27344,"parent_record_offset":27320,"ancestor_record_offset":24332,"inlinee":5909,"source_line":56,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] log::impl$55::clone
//     owner=registry_dependency_definition; count=11
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":545,"record_offset":27392,"parent_record_offset":27368,"ancestor_record_offset":24332,"inlinee":5218,"source_line":766,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] log::max_level
//     owner=registry_dependency_definition; count=10
//     packages=[{"name":"log","version":"0.4.27","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     definition_source=/home/h3/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/log-0.4.27/src/lib.rs
//     evidence={"module":545,"record_offset":28360,"parent_record_offset":27580,"ancestor_record_offset":24332,"inlinee":5206,"source_line":1321,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] serde_json::value::from_value
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":545,"record_offset":26724,"parent_record_offset":26636,"ancestor_record_offset":24332,"inlinee":21141,"source_line":1037,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] std::path::Path::join
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\std\src\path.rs
//     evidence={"module":545,"record_offset":24608,"parent_record_offset":24556,"ancestor_record_offset":24332,"inlinee":7504,"source_line":3010,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_textures::load_glade_texture_mappings::closure$0
// 记录形式: compiled_procedure; instances=1
// 源码行: 292
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":545,"record_offset":43052,"rva":47678256,"va":5416387376}
// 编译实例 RVAs: 0x2d78330
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-31763415","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416387376,"raw_hex":"e829541bfe","size":5,"soft_fail":false,"target_va":5384624080,"va":5416387490,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t234785","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416387376,"raw_hex":"e821950300","size":5,"soft_fail":false,"target_va":5416622240,"va":5416387450,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":545,"record_offset":43212,"parent_record_offset":43052,"ancestor_record_offset":43052,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_textures::load_glade_texture_mappings::closure$1
// 记录形式: inline_instance, inlinee_declaration; instances=2
// 源码行: 301
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8451
// 首条位置: {"module":545,"inlinee":33873}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":545,"record_offset":25668,"parent_record_offset":25568,"ancestor_record_offset":25568,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":545,"record_offset":25600,"parent_record_offset":25568,"ancestor_record_offset":25568,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":545,"record_offset":25640,"parent_record_offset":25568,"ancestor_record_offset":25568,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":25868,"parent_record_offset":25844,"ancestor_record_offset":25568,"inlinee":4124,"source_line":323,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::Global::deallocate_impl_runtime
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":25892,"parent_record_offset":25868,"ancestor_record_offset":25568,"inlinee":4125,"source_line":199,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::dealloc
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":25916,"parent_record_offset":25892,"ancestor_record_offset":25568,"inlinee":4127,"source_line":114,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::alloc::impl$1::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\alloc.rs
//     evidence={"module":545,"record_offset":25844,"parent_record_offset":25780,"ancestor_record_offset":25568,"inlinee":4122,"source_line":440,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::current_memory
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":25812,"parent_record_offset":25780,"ancestor_record_offset":25568,"inlinee":4118,"source_line":631,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::RawVecInner::deallocate
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":25780,"parent_record_offset":25752,"ancestor_record_offset":25568,"inlinee":4117,"source_line":870,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] alloc::raw_vec::impl$3::drop
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\alloc\src\raw_vec\mod.rs
//     evidence={"module":545,"record_offset":25752,"parent_record_offset":25724,"ancestor_record_offset":25568,"inlinee":4115,"source_line":421,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] core::ptr::drop_in_place
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":545,"record_offset":25696,"parent_record_offset":25668,"ancestor_record_offset":25568,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_textures::load_glade_texture_mappings::closure$2
// 记录形式: compiled_procedure; instances=1
// 源码行: 302
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: none
// 首条位置: {"module":545,"record_offset":42864,"rva":47678128,"va":5416387248}
// 编译实例 RVAs: 0x2d782b0
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [native_direct_call_in_physical_body] anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop
//     owner=package_namespace_candidate; count=1
//     packages=[{"name":"anyhow","version":"1.0.100","source":"registry+https://github.com/rust-lang/crates.io-index"}]
//     evidence={"assembly":"\tcallq\t-31763287","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416387248,"raw_hex":"e8a9541bfe","size":5,"soft_fail":false,"target_va":5384624080,"va":5416387362,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [native_direct_call_in_physical_body] core::panicking::panic_fmt
//     owner=rust_standard_library; count=1
//     evidence={"assembly":"\tcallq\t234913","direct":true,"event":"call","llvm_opcode":"CALL64pcrel32","range_va":5416387248,"raw_hex":"e8a1950300","size":5,"soft_fail":false,"target_va":5416622240,"va":5416387322,"scope":"physical procedure may contain inlined library code; not automatically a direct source-level call"}
//   [pdb_inline_direct] core::ptr::drop_in_place
//     owner=rust_standard_library; count=1
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\ptr\mod.rs
//     evidence={"module":545,"record_offset":43024,"parent_record_offset":42864,"ancestor_record_offset":42864,"inlinee":4113,"source_line":805,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   未解析间接目标、缺失普通调用或导入形式仍保持 unknown。
// }
// END GENERATED DEPENDENCY ANNOTATIONS

// ------------------------------------------------------------------
// 函数符号: country_core::startup::load_textures::read_glade_mappings
// 记录形式: inline_instance, inlinee_declaration; instances=3
// 源码行: 316
// TPI: 0x1001
// 签名状态: void() 调试占位；真实参数、返回类型和 Rust ABI 缺失
// IPI IDs: 0x8453
// 首条位置: {"module":545,"inlinee":33875}
// Rust 参数类型 = unknown; 参数名 = unknown; 返回类型 = unknown; ABI = unknown; body = 未恢复

// BEGIN GENERATED DEPENDENCY ANNOTATIONS
// 函数体依赖注解（实现尚未恢复）：
// {
//   [pdb_inline_direct] core::result::impl$27::branch
//     owner=rust_standard_library; count=4
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":545,"record_offset":26688,"parent_record_offset":26636,"ancestor_record_offset":26636,"inlinee":4228,"source_line":2172,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] core::result::impl$28::from_residual
//     owner=rust_standard_library; count=2
//     definition_source=/home/h3/.rustup/toolchains/1.95-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library\core\src\result.rs
//     evidence={"module":545,"record_offset":26816,"parent_record_offset":26636,"ancestor_record_offset":26636,"inlinee":4230,"source_line":2187,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_direct] serde_json::value::from_value
//     owner=git_dependency_definition; count=2
//     packages=[{"name":"serde_json","version":"1.0.151","source":"git+https://github.com/PounceLight/json.git?rev=9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c#9bf7c40f0c47f6bac5568cb6883c5fb1086aab6c"}]
//     definition_source=/home/h3/.cargo/git/checkouts/json-ebf993c3cc323e18/9bf7c40/src/value/mod.rs
//     evidence={"module":545,"record_offset":26724,"parent_record_offset":26636,"ancestor_record_offset":26636,"inlinee":21141,"source_line":1037,"archive":"_pdb/C/modules/00545.jsonl.gz"}
//   [pdb_inline_transitive] country_core::startup::load_textures::_::impl$0::deserialize
//     owner=private_source_ownership_candidate; count=2
//     packages=[{"name":"country-core","version":"0.2.0","source":"local_or_path"}]
//     definition_source=/home/h3/code/pouncelight/country-slice-private/crates/country-core/src/startup/load_textures.rs
//     evidence={"module":545,"record_offset":26748,"parent_record_offset":26724,"ancestor_record_offset":26636,"inlinee":33877,"source_line":226,"archive":"_pdb/C/modules/00545.jsonl.gz"}
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
// 完整结构及原始类型记录: load_textures.rs.structures.json.gz
// 未记录的字段、布局、枚举值及源码定义归属 = unknown。
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::load_textures::GladeTextureMappings
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 3; 过程引用数: 4
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<country_core::startup::load_textures::GladeTextureMappings>", "constructor": "core::ptr::drop_in_place", "arguments": ["country_core::startup::load_textures::GladeTextureMappings"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "serde_json::de::from_str<country_core::startup::load_textures::GladeTextureMappings>", "constructor": "serde_json::de::from_str", "arguments": ["country_core::startup::load_textures::GladeTextureMappings"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::result::Result<country_core::startup::load_textures::GladeTextureMappings,serde_json::error::Error>", "constructor": "core::result::Result", "arguments": ["country_core::startup::load_textures::GladeTextureMappings", "serde_json::error::Error"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<core::result::Result<country_core::startup::load_textures::GladeTextureMappings,serde_json::error::Error> >", "constructor": "enum2$", "arguments": ["core::result::Result<country_core::startup::load_textures::GladeTextureMappings,serde_json::error::Error>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<core::result::Result<country_core::startup::load_textures::GladeTextureMappings,serde_json::error::Error> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<core::result::Result<country_core::startup::load_textures::GladeTextureMappings,serde_json::error::Error> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::load_textures::LoadAllGameTextures
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 34
// [PDB 泛型关系，非字段关系] {"type": "country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>", "constructor": "country_core::startup::load_textures::load_packaged_textures::closure_env$0", "arguments": ["country_core::startup::load_textures::LoadAllGameTextures"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "constructor": "bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0", "arguments": ["alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>", "asset_pipe::texture::TextureAssetListEntry", "country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>", "alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "constructor": "bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0", "arguments": ["bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block$0::async_block_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "core::panic::unwind_safe::AssertUnwindSafe", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "futures_lite::future::CatchUnwind", "arguments": ["core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "core::panic::unwind_safe::AssertUnwindSafe", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "futures_lite::future::CatchUnwind", "arguments": ["core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "async_executor::impl$5::run::async_fn_env$0", "arguments": ["never$", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "enum2$", "arguments": ["async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::AssertUnwindSafe<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > >", "constructor": "core::panic::unwind_safe::AssertUnwindSafe", "arguments": ["enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > > >", "constructor": "futures_lite::future::CatchUnwind", "arguments": ["core::panic::unwind_safe::AssertUnwindSafe<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "async_executor::impl$5::run::async_fn_env$0", "arguments": ["never$", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "enum2$", "arguments": ["async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::AssertUnwindSafe<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > >", "constructor": "core::panic::unwind_safe::AssertUnwindSafe", "arguments": ["enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > > >", "constructor": "futures_lite::future::CatchUnwind", "arguments": ["core::panic::unwind_safe::AssertUnwindSafe<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "constructor": "bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0", "arguments": ["alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>", "asset_pipe::texture::TextureAssetListEntry", "country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>", "alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "constructor": "enum2$", "arguments": ["bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "core::panic::unwind_safe::AssertUnwindSafe", "arguments": ["enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "futures_lite::future::CatchUnwind", "arguments": ["core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "async_executor::impl$5::spawn_inner::async_block_env$0", "arguments": ["enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "enum2$", "arguments": ["async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_task::raw::impl$3::run::Guard<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >,enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,async_executor::impl$5::schedule::closure_env$0,tuple$<> >", "constructor": "async_task::raw::impl$3::run::Guard", "arguments": ["enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "async_executor::impl$5::schedule::closure_env$0", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<async_task::raw::impl$3::run::Guard<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >,enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,async_executor::impl$5::schedule::closure_env$0,tuple$<> > >", "constructor": "core::ptr::drop_in_place", "arguments": ["async_task::raw::impl$3::run::Guard<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >,enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,async_executor::impl$5::schedule::closure_env$0,tuple$<> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "futures_lite::future::Or", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "futures_lite::future::Or", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "futures_lite::future::Or", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "futures_lite::future::Or", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>", "constructor": "country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0", "arguments": ["country_core::startup::load_textures::LoadAllGameTextures"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>", "country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["enum2$<core::option::Option<rhapsody::raw::texture::Texture> >", "core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "constructor": "bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0", "arguments": ["bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::block_on::closure_env$0<alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "futures_lite::future::block_on::closure_env$0", "arguments": ["alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::thread::local::LocalKey::with<core::cell::RefCell<tuple$<parking::Parker,core::task::wake::Waker> >,futures_lite::future::block_on::closure_env$0<alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >,alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "std::thread::local::LocalKey::with", "arguments": ["core::cell::RefCell<tuple$<parking::Parker,core::task::wake::Waker> >", "futures_lite::future::block_on::closure_env$0<alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::TaskPool::scope_with_executor_inner<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "constructor": "bevy_tasks::task_pool::TaskPool::scope_with_executor_inner", "arguments": ["bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_executor::impl$5::spawn_inner::closure_env$1<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "async_executor::impl$5::spawn_inner::closure_env$1", "arguments": ["enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<async_executor::impl$5::spawn_inner::closure_env$1<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["async_executor::impl$5::spawn_inner::closure_env$1<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_executor::impl$5::run::async_fn$0::async_block_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "async_executor::impl$5::run::async_fn$0::async_block_env$0", "arguments": ["never$", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<async_executor::impl$5::run::async_fn$0::async_block_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "enum2$", "arguments": ["async_executor::impl$5::run::async_fn$0::async_block_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$7::poll<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >,enum2$<async_executor::impl$5::run::async_fn$0::async_block_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > >", "constructor": "futures_lite::future::impl$7::poll", "arguments": ["never$", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "enum2$<async_executor::impl$5::run::async_fn$0::async_block_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$7::poll<alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "futures_lite::future::impl$7::poll", "arguments": ["alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_executor::impl$5::run::async_fn$0::async_block_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "async_executor::impl$5::run::async_fn$0::async_block_env$0", "arguments": ["never$", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<async_executor::impl$5::run::async_fn$0::async_block_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "enum2$", "arguments": ["async_executor::impl$5::run::async_fn$0::async_block_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$7::poll<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >,enum2$<async_executor::impl$5::run::async_fn$0::async_block_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > >", "constructor": "futures_lite::future::impl$7::poll", "arguments": ["never$", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "enum2$<async_executor::impl$5::run::async_fn$0::async_block_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$7::poll<alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "futures_lite::future::impl$7::poll", "arguments": ["alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$7::poll<alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "futures_lite::future::impl$7::poll", "arguments": ["alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$7::poll<alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "futures_lite::future::impl$7::poll", "arguments": ["alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$9::poll<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "futures_lite::future::impl$9::poll", "arguments": ["core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_task::raw::RawTask::clone_waker<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >,enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,async_executor::impl$5::schedule::closure_env$0,tuple$<> >", "constructor": "async_task::raw::RawTask::clone_waker", "arguments": ["enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "async_executor::impl$5::schedule::closure_env$0", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_task::raw::RawTask::drop_future<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >,enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,async_executor::impl$5::schedule::closure_env$0,tuple$<> >", "constructor": "async_task::raw::RawTask::drop_future", "arguments": ["enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "async_executor::impl$5::schedule::closure_env$0", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_task::raw::RawTask::wake_by_ref<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >,enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,async_executor::impl$5::schedule::closure_env$0,tuple$<> >", "constructor": "async_task::raw::RawTask::wake_by_ref", "arguments": ["enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "async_executor::impl$5::schedule::closure_env$0", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_task::raw::RawTask::run<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >,enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,async_executor::impl$5::schedule::closure_env$0,tuple$<> >", "constructor": "async_task::raw::RawTask::run", "arguments": ["enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "async_executor::impl$5::schedule::closure_env$0", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_task::raw::RawTask::schedule<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >,enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,async_executor::impl$5::schedule::closure_env$0,tuple$<> >", "constructor": "async_task::raw::RawTask::schedule", "arguments": ["enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "async_executor::impl$5::schedule::closure_env$0", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::impl$28::poll<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > >", "constructor": "core::panic::unwind_safe::impl$28::poll", "arguments": ["enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::impl$28::poll<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > >", "constructor": "core::panic::unwind_safe::impl$28::poll", "arguments": ["enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// ------------------------------------------------------------------
// [PDB 命名实体；类型/枚举变体身份待确认] country_core::startup::load_textures::LoadOnlyBootupTextures
// fields / size / alignment / enum values = unknown; 不由类型名称猜字段。
// 原始定义文件: namespace_module_candidate; author definition file unknown
// 所有 PDB 名称引用及完整关系见同名 structures.json.gz；IPI 引用数: 0; 过程引用数: 22
// [PDB 泛型关系，非字段关系] {"type": "country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>", "constructor": "country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0", "arguments": ["country_core::startup::load_textures::LoadOnlyBootupTextures"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures> >", "constructor": "core::iter::adapters::map::Map", "arguments": ["core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>", "country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures> > >", "constructor": "alloc::vec::spec_from_iter::impl$0::from_iter", "arguments": ["enum2$<core::option::Option<rhapsody::raw::texture::Texture> >", "core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>", "constructor": "country_core::startup::load_textures::load_packaged_textures::closure_env$0", "arguments": ["country_core::startup::load_textures::LoadOnlyBootupTextures"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "constructor": "bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0", "arguments": ["alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>", "asset_pipe::texture::TextureAssetListEntry", "country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>", "alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "constructor": "bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0", "arguments": ["bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::block_on::closure_env$0<alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "futures_lite::future::block_on::closure_env$0", "arguments": ["alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "std::thread::local::LocalKey::with<core::cell::RefCell<tuple$<parking::Parker,core::task::wake::Waker> >,futures_lite::future::block_on::closure_env$0<alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >,alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global> >", "constructor": "std::thread::local::LocalKey::with", "arguments": ["core::cell::RefCell<tuple$<parking::Parker,core::task::wake::Waker> >", "futures_lite::future::block_on::closure_env$0<alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "constructor": "bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0", "arguments": ["bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::TaskPool::scope_with_executor_inner<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "constructor": "bevy_tasks::task_pool::TaskPool::scope_with_executor_inner", "arguments": ["bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "constructor": "bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0", "arguments": ["alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>", "asset_pipe::texture::TextureAssetListEntry", "country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>", "alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "constructor": "enum2$", "arguments": ["bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "core::panic::unwind_safe::AssertUnwindSafe", "arguments": ["enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "futures_lite::future::CatchUnwind", "arguments": ["core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_executor::impl$5::spawn_inner::closure_env$1<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "async_executor::impl$5::spawn_inner::closure_env$1", "arguments": ["enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<async_executor::impl$5::spawn_inner::closure_env$1<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["async_executor::impl$5::spawn_inner::closure_env$1<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "constructor": "bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block$0", "arguments": ["bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >", "alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$7::poll<alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "futures_lite::future::impl$7::poll", "arguments": ["alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$7::poll<alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "futures_lite::future::impl$7::poll", "arguments": ["alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$7::poll<alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "futures_lite::future::impl$7::poll", "arguments": ["alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$7::poll<alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "futures_lite::future::impl$7::poll", "arguments": ["alloc::vec::Vec<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "futures_lite::future::impl$9::poll<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "futures_lite::future::impl$9::poll", "arguments": ["core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "async_executor::impl$5::spawn_inner::async_block_env$0", "arguments": ["enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "enum2$", "arguments": ["async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_task::raw::RawTask::clone_waker<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >,enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,async_executor::impl$5::schedule::closure_env$0,tuple$<> >", "constructor": "async_task::raw::RawTask::clone_waker", "arguments": ["enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "async_executor::impl$5::schedule::closure_env$0", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_task::raw::RawTask::drop_future<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >,enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,async_executor::impl$5::schedule::closure_env$0,tuple$<> >", "constructor": "async_task::raw::RawTask::drop_future", "arguments": ["enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "async_executor::impl$5::schedule::closure_env$0", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_task::raw::RawTask::run<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >,enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,async_executor::impl$5::schedule::closure_env$0,tuple$<> >", "constructor": "async_task::raw::RawTask::run", "arguments": ["enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >", "async_executor::impl$5::schedule::closure_env$0", "tuple$<>"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::ptr::drop_in_place<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > >", "constructor": "core::ptr::drop_in_place", "arguments": ["enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::impl$28::poll<enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "core::panic::unwind_safe::impl$28::poll", "arguments": ["enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "async_executor::impl$5::run::async_fn_env$0", "arguments": ["never$", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "enum2$", "arguments": ["async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::impl$28::poll<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > >", "constructor": "core::panic::unwind_safe::impl$28::poll", "arguments": ["enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >", "constructor": "bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0", "arguments": ["alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>", "enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >", "constructor": "enum2$", "arguments": ["bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >", "constructor": "async_executor::impl$5::run::async_fn_env$0", "arguments": ["never$", "enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >", "constructor": "enum2$", "arguments": ["async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// [PDB 泛型关系，非字段关系] {"type": "core::panic::unwind_safe::impl$28::poll<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > >", "constructor": "core::panic::unwind_safe::impl$28::poll", "arguments": ["enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >"], "evidence": "pdb_symbol_type_spelling", "layout": "unknown"}
// === PDB STRUCTURES V1 END ===

// === WHOLE PROJECT EVIDENCE V1 BEGIN ===
// 全项目结构/证据核对；本文件已检查，unknown 不代表已恢复实现。
// 完整附档: load_textures.rs.project-evidence.json.gz
// PDB 定义及名称关系见 PDB STRUCTURES V1；业务布局只接受逐项版本证据。
// 此文件本轮没有额外可确认的原指令字段；完整游戏布局 = unknown。
// 只有名称/泛型/PDB/文件覆盖不等于整个游戏实现完成；函数体和未验证业务仍待恢复。
// === WHOLE PROJECT EVIDENCE V1 END ===

// === PDB ENRICHMENT V1 BEGIN ===
// 函数接口、类型关系、闭包与引用方。事实/候选/未知分别标记。
// 完整信息: load_textures.rs.enriched.json.gz
// 引用方全部同地址别名: _pdb/C/enrichment/address-aliases.json.gz（按 caller_alias_group_rva 查找）。
// 文件级接口候选来自同一编译单元，不等于具体函数签名。
// 文件级候选接口: 39; 完整候选见附档及 _pdb/C/enrichment/candidate-interfaces.json.gz

// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::system::commands::Commands", "bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>,bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>,bevy_ecs::system::commands::Commands)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>", "bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>", "bevy_ecs::system::commands::Commands"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::mesh_library::AssetMeshLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::texture_library::AssetTextureLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::meshwatch::MeshWatch>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::asset_libraries::shader_library::AssetShaderLibrary>
// [候选系统接口] deferred_command_interface: bevy_ecs::system::commands::Commands
// [文件级接口候选，具体函数归属 unknown] void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>,bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>,bevy_ecs::message::message_writer::MessageWriter<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>)
// 候选参数类型: ["bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>", "bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>", "bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>", "bevy_ecs::message::message_writer::MessageWriter<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>"]
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::asset_libraries::ssbo_library::AssetSsboLibrary>
// [候选系统接口] resource_read: bevy_ecs::change_detection::Res<country_core::render::nani::gpu_world::GpuWorld>
// [候选系统接口] message_writer: bevy_ecs::message::message_writer::MessageWriter<country_core::resources::mouse_capture::CustomCursorSsboCreatedCmd>
// [文件级接口候选，具体函数归属 unknown] void (*)(enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >,bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>)
// 候选参数类型: ["enum2$<core::option::Option<bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher> > >", "bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>"]
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::startup::load_prefabs::PrefabWatcher>
// [候选系统接口] resource_write_capability: bevy_ecs::change_detection::ResMut<country_core::resources::prefab::PrefabLibrary>
// 文件级全部候选接口及参数类型均在此文件的 enriched.json.gz 附档中。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_textures::GladeTextureMappings::create_mappings
// 信息 ID: 570179f13e1b6d57fc8c9dc1
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_textures::load_glade_texture_mappings @ 0xa954a5 -> 0xa945a0 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_textures::load_glade_texture_mappings @ 0xa95267 -> 0xa945a0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_textures::GladeTextureMappings::create_mappings_with_fallback
// 信息 ID: d159e52b579599325ed39fbc
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_textures::load_glade_texture_mappings; module=545, inline_record=27580; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_textures::_::impl$0::deserialize
// 信息 ID: 36c99a7091fc8c2f9271e4c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 3 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_textures::load_glade_texture_mappings; module=545, inline_record=26748; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_textures::load_glade_texture_mappings; module=545, inline_record=27076; 不是独立调用边。
// [PDB 内联宿主] serde_json::de::from_str<country_core::startup::load_textures::GladeTextureMappings>; module=1645, inline_record=72352; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_textures::_::impl$0::deserialize::impl$0::visit_str<serde_json::error::Error>
// 信息 ID: 81a4ca9334ea447fcc0cf466
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [类型关系] country_core::startup::load_textures::_::impl$0::deserialize::impl$0::visit_str -> ["serde_json::error::Error"]; 字段布局 = unknown。
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::startup::load_textures::_::impl$0::deserialize::__Visitor> @ 0x1660e0e -> 0x1ab7890 (direct_call); 同地址别名未裁定。
// [静态直接引用] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::startup::load_textures::_::impl$0::deserialize::__Field> > @ 0x1ba7d83 -> 0x1ab7890 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_textures::_::impl$0::deserialize::impl$1::deserialize
// 信息 ID: 49c767268ff317e7c270fd04
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::startup::load_textures::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=265292; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$12::next_key_seed<core::marker::PhantomData<country_core::startup::load_textures::_::impl$0::deserialize::__Field> >; module=1803, inline_record=401000; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_textures::_::impl$0::deserialize::impl$2::expecting
// 信息 ID: 956f949454c2e95a39f0c854
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_core::de::impl$1::fmt<country_core::startup::load_textures::_::impl$0::deserialize::__Visitor>; module=1761, inline_record=138976; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_textures::_::impl$0::deserialize::impl$2::visit_map
// 信息 ID: ecfa542a0c6727f972b38938
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::startup::load_textures::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=265068; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::impl$3::deserialize_any<country_core::startup::load_textures::_::impl$0::deserialize::__Visitor>; module=1804, inline_record=159708; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_textures::_::impl$0::deserialize::impl$2::visit_seq
// 信息 ID: 24d4de3fb398d64413cadf02
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] serde_json::de::impl$7::deserialize_struct<serde_json::read::StrRead,country_core::startup::load_textures::_::impl$0::deserialize::__Visitor>; module=1645, inline_record=284640; 不是独立调用边。
// [PDB 内联宿主] serde_json::value::de::visit_array<country_core::startup::load_textures::_::impl$0::deserialize::__Visitor>; module=1803, inline_record=40844; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_textures::impl$0::should_load
// 信息 ID: 40d0092d86885bc2ba37dd53
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures> > > @ 0x160f82a -> 0xa9b5e0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_textures::impl$1::should_load
// 信息 ID: 74e2563b1b76cf6fb10f2933
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> > > @ 0x16079da -> 0xa9b220 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_textures::load_glade_texture_mappings
// 信息 ID: 739b9d450997bcf78a2fb694
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] c40910435d803b262c287786: void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] bevy_ecs::system::system::System::run_without_applying_deferred<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>)> > @ 0x191c7f1 -> 0xa94fc0 (direct_call); 同地址别名未裁定。
// [静态直接引用] bevy_ecs::system::function_system::impl$12::run_unsafe<void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>),tuple$<>,void (*)(bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>)> @ 0x1ab4cb4 -> 0xa94fc0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_textures::load_glade_texture_mappings::closure$0
// 信息 ID: 52f8cbd3234fc5ea072b0fd3
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::load_textures::load_glade_texture_mappings", "captures": "unknown"}
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_textures::load_glade_texture_mappings @ 0xa969ff -> 0x2d78330 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_textures::load_glade_texture_mappings::closure$1
// 信息 ID: ac1939f7fce519f431ad4684
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 1, "lexical_owner": "country_core::startup::load_textures::load_glade_texture_mappings", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 1 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_textures::load_glade_texture_mappings; module=545, inline_record=25568; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_textures::load_glade_texture_mappings::closure$2
// 信息 ID: f0b5239ef960d6d7f155697e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 2, "lexical_owner": "country_core::startup::load_textures::load_glade_texture_mappings", "captures": "unknown"}
// 引用方: 原程序直接转移 1 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_textures::load_glade_texture_mappings @ 0xa969d8 -> 0x2d782b0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_textures::load_packaged_textures
// 信息 ID: cfa18c971d1e207baaef1429
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 738bb63638a82c50e104f256: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)>; module=1761, inline_record=142496; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)>; module=1761, inline_record=145584; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_textures::load_packaged_textures::closure$0
// 信息 ID: d0ae3eb4de977361020b170e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::load_textures::load_packaged_textures", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::load_textures::load_packaged_textures::closure_env$0 <- core::ptr::drop_in_place<futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::load_textures::load_packaged_textures::closure_env$0 <- core::ptr::drop_in_place<futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::task_pool::impl$2::execute_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::load_textures::load_packaged_textures::closure_env$0 <- core::ptr::drop_in_place<futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::load_textures::load_packaged_textures::closure_env$0 <- core::ptr::drop_in_place<futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<async_executor::impl$5::run::async_fn_env$0<never$,enum2$<bevy_tasks::task_pool::impl$2::execute_global_external_scope::async_fn$0::async_block$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::load_textures::load_packaged_textures::closure_env$0 <- core::ptr::drop_in_place<async_task::raw::impl$3::run::Guard<enum2$<async_executor::impl$5::spawn_inner::async_block_env$0<enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,futures_lite::future::CatchUnwind<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >,enum2$<core::result::Result<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,alloc::boxed::Box<dyn$<core::any::Any,core::marker::Send>,alloc::alloc::Global> > >,async_executor::impl$5::schedule::closure_env$0,tuple$<> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::load_textures::load_packaged_textures::closure_env$0 <- core::ptr::drop_in_place<futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::load_textures::load_packaged_textures::closure_env$0 <- core::ptr::drop_in_place<futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::load_textures::load_packaged_textures::closure_env$0 <- core::ptr::drop_in_place<futures_lite::future::Or<enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > >,enum2$<bevy_tasks::task_pool::impl$2::execute_global_scope::async_fn$0::async_block_env$0<alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global>,enum2$<bevy_tasks::task_pool::impl$2::scope_with_executor_inner::async_block$0::async_block_env$0<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> >,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > > > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 68 条。
// [PDB 内联宿主] futures_lite::future::impl$9::poll<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >; module=1749, inline_record=113152; 不是独立调用边。
// [PDB 内联宿主] futures_lite::future::impl$9::poll<core::panic::unwind_safe::AssertUnwindSafe<enum2$<bevy_tasks::slice::ParallelSlice::par_chunk_map::closure$0::async_block_env$0<alloc::vec::Vec<asset_pipe::texture::TextureAssetListEntry,alloc::alloc::Global>,asset_pipe::texture::TextureAssetListEntry,country_core::startup::load_textures::load_packaged_textures::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures>,alloc::vec::Vec<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,alloc::alloc::Global> > > > >; module=1749, inline_record=114360; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_textures::load_packaged_textures::closure$0::closure$0
// 信息 ID: 3d7748c68663fa419d12a5c8
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::load_textures::load_packaged_textures", "captures": "unknown"}
// [闭包环境类型引用] country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> > >; 捕获字段/偏移 = unknown。
// [闭包环境类型引用] country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0 <- alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures> > >; 捕获字段/偏移 = unknown。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 2 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> > >; module=1643, inline_record=506080; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures> > >; module=1643, inline_record=646128; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_textures::load_packaged_textures::closure$0::closure$0::closure$0
// 信息 ID: 79d5e28372594d395e7fec98
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// [闭包] {"kind": "closure", "index": 0, "lexical_owner": "country_core::startup::load_textures::load_packaged_textures", "captures": "unknown"}
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0<country_core::startup::load_textures::LoadAllGameTextures> > >; module=1643, inline_record=506136; 不是独立调用边。
// [PDB 内联宿主] alloc::vec::spec_from_iter::impl$0::from_iter<enum2$<core::option::Option<rhapsody::raw::texture::Texture> >,core::iter::adapters::map::Map<core::slice::iter::Iter<asset_pipe::texture::TextureAssetListEntry>,country_core::startup::load_textures::load_packaged_textures::closure$0::closure_env$0<country_core::startup::load_textures::LoadOnlyBootupTextures> > >; module=1643, inline_record=646184; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_textures::load_textures
// 信息 ID: 10cc7b5a3fe4eef9cb4c0569
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 738bb63638a82c50e104f256: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)>; module=1761, inline_record=139620; 不是独立调用边。
// [PDB 内联宿主] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)>; module=1761, inline_record=145424; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_textures::read_glade_mappings
// 信息 ID: 8a6ac2c80f2daab910d5784e
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 0 条；内联宿主 2 条；闭包环境类型引用 0 条。
// [PDB 内联宿主] country_core::startup::load_textures::load_glade_texture_mappings; module=545, inline_record=26636; 不是独立调用边。
// [PDB 内联宿主] country_core::startup::load_textures::load_glade_texture_mappings; module=545, inline_record=26984; 不是独立调用边。
// 静态直接调用方 = unknown/当前扫描未找到；不能据此认定函数无引用。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_textures::read_texture_list
// 信息 ID: 7657853ae255a0d8a3b9dc34
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// [泛型宿主/闭包/直接引用方参数候选] 738bb63638a82c50e104f256: void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)
// 引用方: 原程序直接转移 2 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)> @ 0x1ab7e2a -> 0xa942d0 (direct_call); 同地址别名未裁定。
// [静态直接引用] core::ops::function::impls::impl$3::call_mut<tuple$<bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands>,void (*)(bevy_ecs::change_detection::Res<country_core::render::GraphicsDeviceResource>,bevy_ecs::change_detection::ResMut<country_core::asset_libraries::texture_library::AssetTextureLibrary>,bevy_ecs::system::commands::Commands)> @ 0x1ab83ed -> 0xa942d0 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// ------------------------------------------------------------------
// 函数: country_core::startup::load_textures::reload_packaged_textures_by_name
// 信息 ID: 7c6cd2466ab5b85d65335555
// 参数名 = unknown; 完整 Rust 签名 = unknown; ABI = unknown; 函数体 = unknown。
// 参数类型候选 = unknown（没有足够函数绑定证据；可另查文件级候选）。
// 引用方: 原程序直接转移 4 条；内联宿主 0 条；闭包环境类型引用 0 条。
// [静态直接引用] country_core::startup::load_mod_textures::reload_mod_textures @ 0x9e48c0 -> 0xa97030 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons @ 0xb92ca1 -> 0xa97030 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons @ 0xb93240 -> 0xa97030 (direct_call); 同地址别名未裁定。
// [静态直接引用] country_core::startup::load_mod_clutter_icons::rebake_modded_clutter_icons @ 0xb936f4 -> 0xa97030 (direct_call); 同地址别名未裁定。
// 间接调用方、运行时实际调用和完整调度关系 = unknown。

// === PDB ENRICHMENT V1 END ===
